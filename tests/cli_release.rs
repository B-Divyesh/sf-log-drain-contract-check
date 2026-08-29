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
