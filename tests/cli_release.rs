use serde_json::Value;
use std::{fs, net::TcpListener, path::Path, process::Command, thread, time::Duration};

fn drain_check() -> Command {
    Command::new(env!("CARGO_BIN_EXE_drain-check"))
}

#[test]
fn installed_demo_runs_outside_repository() {
    let consumer = tempfile::tempdir().unwrap();
    let result = drain_check()
        .args(["demo", "--json"])
        .current_dir(consumer.path())
        .output()
        .unwrap();

    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["events"], 3);
    assert_eq!(report["retention"][0]["display"], "558.1 KiB");

    let location = String::from_utf8(result.stderr).unwrap();
    let path = location.trim().strip_prefix("Demo report: ").unwrap();
    assert!(path.contains("drain-check-demo-"));
    assert!(fs::metadata(path).unwrap().is_file());
    assert!(!path.starts_with(env!("CARGO_MANIFEST_DIR")));
}

#[test]
fn inspect_requires_a_positive_sample_duration() {
    let result = drain_check()
        .args(["inspect", "examples/drain.ndjson", "--sample-seconds", "0"])
        .output()
        .unwrap();

    assert!(!result.status.success());
    assert!(String::from_utf8_lossy(&result.stderr).contains("1.."));
}

#[test]
fn inspect_json_writes_the_report_to_standard_output() {
    let directory = tempfile::tempdir().unwrap();
    let output_path = directory.path().join("report.json");
    let input = Path::new(env!("CARGO_MANIFEST_DIR")).join("examples/drain.ndjson");
    let result = drain_check()
        .args([
            "inspect",
            input.to_str().unwrap(),
            "--sample-seconds",
            "600",
            "--output",
            output_path.to_str().unwrap(),
            "--json",
        ])
        .output()
        .unwrap();

    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let stdout: Value = serde_json::from_slice(&result.stdout).unwrap();
    let written: Value = serde_json::from_str(&fs::read_to_string(output_path).unwrap()).unwrap();
    assert_eq!(stdout["events"], 3);
    assert_eq!(stdout, written);
}

#[test]
fn inspect_preserves_received_bytes_and_punctuated_source_keys() {
    let directory = tempfile::tempdir().unwrap();
    let input = directory.path().join("punctuated.ndjson");
    let output_path = directory.path().join("report.json");
    let spaced_value = " ".repeat(1024);
    fs::write(
        &input,
        format!(
            "{{\"a\":{spaced_value}1,\"http.method\":\"GET\",\"http\":{{\"method\":42}},\"items[]\":\"flat\",\"items\":[\"array\"],\"password.hash\":\"ordinary\",\"customer.id\":\"12345\"}}\n"
        ),
    )
    .unwrap();
    let received_event_bytes = fs::metadata(&input).unwrap().len() - 1;
    let result = drain_check()
        .args([
            "inspect",
            input.to_str().unwrap(),
            "--sample-seconds",
            "1",
            "--sensitive-field",
            "customer",
            "--output",
            output_path.to_str().unwrap(),
            "--json",
        ])
        .output()
        .unwrap();

    assert!(
        result.status.success(),
        "{}",
        String::from_utf8_lossy(&result.stderr)
    );
    let report: Value = serde_json::from_slice(&result.stdout).unwrap();
    assert_eq!(report["average_event_bytes"], received_event_bytes);
    assert_eq!(
        report["retention"][0]["estimated_bytes"],
        received_event_bytes * 86_400 * 7
    );
    let fields = report["fields"].as_array().unwrap();
    for (path, expected_type) in [
        ("$['http.method']", "string"),
        ("$.http.method", "integer"),
        ("$['items[]']", "string"),
        ("$.items[]", "string"),
    ] {
        let field = fields
            .iter()
            .find(|field| field["path"] == path)
            .unwrap_or_else(|| panic!("missing field path {path}"));
        assert_eq!(field["types"], serde_json::json!([expected_type]));
    }
    let findings = report["findings"].as_array().unwrap();
    for path in ["$['password.hash']", "$['customer.id']"] {
        assert!(findings.iter().any(|finding| {
            finding["path"] == path && finding["detector"] == "sensitive field name"
        }));
    }
}

#[test]
fn help_lists_each_command_and_its_options() {
    let result = drain_check().arg("--help").output().unwrap();

    assert!(result.status.success());
    let help = String::from_utf8(result.stdout).unwrap();
    for expected in [
        "listen",
        "inspect",
        "demo",
        "forwarding",
        "--help",
        "--version",
    ] {
        assert!(
            help.contains(expected),
            "missing {expected} from help: {help}"
        );
    }
}

#[test]
fn version_matches_the_released_changelog_section() {
    let result = drain_check().arg("--version").output().unwrap();

    assert!(result.status.success());
    let version = env!("CARGO_PKG_VERSION");
    assert_eq!(
        String::from_utf8(result.stdout).unwrap().trim(),
        format!("drain-check {version}")
    );
    assert!(include_str!("../CHANGELOG.md").contains(&format!("## {version}\n")));
}

#[cfg(unix)]
#[test]
fn interrupt_writes_the_partial_report() {
    let directory = tempfile::tempdir().unwrap();
    let report_path = directory.path().join("interrupted.json");
    let port_probe = TcpListener::bind("127.0.0.1:0").unwrap();
    let port = port_probe.local_addr().unwrap().port().to_string();
    drop(port_probe);
    let mut child = drain_check()
        .args([
            "listen",
            "--duration",
            "60",
            "--port",
            port.as_str(),
            "--output",
            report_path.to_str().unwrap(),
        ])
        .spawn()
        .unwrap();
    thread::sleep(Duration::from_millis(200));
    let signal = Command::new("kill")
        .args(["-INT", &child.id().to_string()])
        .status()
        .unwrap();
    assert!(signal.success());
    assert!(child.wait().unwrap().success());
    let report: Value = serde_json::from_str(&fs::read_to_string(report_path).unwrap()).unwrap();
    assert_eq!(report["events"], 0);
    assert_eq!(report["sample_seconds"], 1);
}
