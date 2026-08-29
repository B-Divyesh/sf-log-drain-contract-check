use clap::{Parser, Subcommand};
use drain_check::{Aggregate, DetectorConfig, Report};
use serde_json::Value;
use std::{
    collections::{HashSet, VecDeque},
    fs::{self, File},
    io::{BufRead, BufReader, Read, Write},
    net::{IpAddr, Ipv4Addr, TcpListener, TcpStream},
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};
use url::Url;

const DEMO_SAMPLE: &str = include_str!("../examples/drain.ndjson");
const MAX_HEADER_BYTES: usize = 32 * 1024;
const MAX_BODY_BYTES: usize = 2 * 1024 * 1024;
const READ_TIMEOUT: Duration = Duration::from_millis(750);
static RUNNING: AtomicBool = AtomicBool::new(true);

#[derive(Debug, Parser)]
#[command(
    name = "drain-check",
    version,
    about = "Sample a local log drain and review its fields, size, and sensitive data."
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Receive a bounded local sample. Bodies are aggregated and then discarded.
    Listen {
        /// Sampling window in seconds.
        #[arg(long, default_value_t = 600, value_parser = clap::value_parser!(u64).range(1..))]
        duration: u64,
        /// Local TCP port. The receiver always binds to 127.0.0.1.
        #[arg(long, default_value_t = 8787, value_parser = clap::value_parser!(u16).range(1..))]
        port: u16,
        #[arg(long, default_value = "report.json")]
        output: PathBuf,
        #[arg(long, value_delimiter = ',', default_value = "7,30")]
        retention_days: Vec<u32>,
        /// Save accepted bodies as NDJSON. Without this flag, bodies are not retained.
        #[arg(long)]
        save_sample: Option<PathBuf>,
        /// Add a case-insensitive field-name fragment to the sensitive-field detector.
        #[arg(long, value_delimiter = ',')]
        sensitive_field: Vec<String>,
        /// Suppress findings for an exact JSON path, or a path prefix ending in `*`.
        #[arg(long, value_delimiter = ',')]
        ignore_field: Vec<String>,
        /// Maximum accepted requests per rolling second.
        #[arg(long, default_value_t = 20, value_parser = clap::value_parser!(u32).range(1..))]
        rate_limit: u32,
        #[arg(long)]
        json: bool,
    },
    /// Analyse newline-delimited JSON from a file without opening a listener.
    Inspect {
        input: PathBuf,
        /// Duration of the file sample in seconds. Must be at least one second.
        #[arg(long, default_value_t = 600, value_parser = clap::value_parser!(u64).range(1..))]
        sample_seconds: u64,
        #[arg(long, default_value = "report.json")]
        output: PathBuf,
        #[arg(long, value_delimiter = ',', default_value = "7,30")]
        retention_days: Vec<u32>,
        /// Add a case-insensitive field-name fragment to the sensitive-field detector.
        #[arg(long, value_delimiter = ',')]
        sensitive_field: Vec<String>,
        /// Suppress findings for an exact JSON path, or a path prefix ending in `*`.
        #[arg(long, value_delimiter = ',')]
        ignore_field: Vec<String>,
        #[arg(long)]
        json: bool,
    },
    /// Run the sample embedded in this binary in a new temporary directory.
    Demo {
        #[arg(long)]
        json: bool,
    },
    /// Print a forwarding configuration template.
    Forwarding {
        #[arg(
            long,
            default_value = "https://receiver.example/logs",
            value_parser = validate_http_url
        )]
        url: Url,
        #[arg(long, default_value = "generic-http")]
        platform: String,
    },
}

fn main() -> Result<(), String> {
    match Cli::parse().command {
        Command::Inspect {
            input,
            sample_seconds,
            output,
            retention_days,
            sensitive_field,
            ignore_field,
            json,
        } => {
            let config = DetectorConfig::with_overrides(&sensitive_field, &ignore_field);
            let aggregate = inspect_path(&input, &config)?;
            write_report(
                aggregate.report(sample_seconds, &retention_days, false),
                &output,
                json,
            )
        }
        Command::Demo { json } => run_demo(json),
        Command::Forwarding { url, platform } => {
            print!("{}", forwarding_config(&url, &platform));
            Ok(())
        }
        Command::Listen {
            duration,
            port,
            output,
            retention_days,
            save_sample,
            sensitive_field,
            ignore_field,
            rate_limit,
            json,
        } => listen(ListenOptions {
            duration,
            port,
            output,
            retention_days,
            save_sample,
            detector: DetectorConfig::with_overrides(&sensitive_field, &ignore_field),
            rate_limit,
            json,
        }),
    }
}

fn validate_http_url(value: &str) -> Result<Url, String> {
    let url = Url::parse(value).map_err(|error| format!("URL must be valid: {error}"))?;
    if !matches!(url.scheme(), "http" | "https") {
        return Err("URL must use http:// or https://".to_string());
    }
    if url.host_str().is_none() {
        return Err("URL must include a valid host".to_string());
    }
    Ok(url)
}

fn forwarding_config(url: &Url, platform: &str) -> String {
    // JSON string encoding keeps the generated quoted configuration valid even
    // when a URL contains characters that need escaping.
    let encoded_url = serde_json::to_string(url.as_str()).expect("URL strings serialize");
    format!(
        "# {platform}\n# Send POST requests to this endpoint after report review\nurl = {encoded_url}\nmethod = \"POST\"\ncontent_type = \"application/json\"\n"
    )
}

fn run_demo(json: bool) -> Result<(), String> {
    let directory = tempfile::Builder::new()
        .prefix("drain-check-demo-")
        .tempdir()
        .map_err(|error| format!("Could not create demo directory: {error}"))?
        .keep();
    let output = directory.join("report.json");
    eprintln!("Demo report: {}", output.display());
    let aggregate = inspect_reader(
        BufReader::new(DEMO_SAMPLE.as_bytes()),
        &DetectorConfig::default(),
    )?;
    write_report(aggregate.report(600, &[7, 30], false), &output, json)
}

fn inspect_path(path: &Path, config: &DetectorConfig) -> Result<Aggregate, String> {
    let file =
        File::open(path).map_err(|error| format!("Could not read {}: {error}", path.display()))?;
    inspect_reader(BufReader::new(file), config)
}

fn inspect_reader<R: BufRead>(reader: R, config: &DetectorConfig) -> Result<Aggregate, String> {
    let mut aggregate = Aggregate::default();
    for (index, line) in reader.lines().enumerate() {
        let line = line.map_err(|error| format!("Could not read line {}: {error}", index + 1))?;
        if line.trim().is_empty() {
            continue;
        }
        let event: Value = serde_json::from_str(&line)
            .map_err(|error| format!("Invalid JSON on line {}: {error}", index + 1))?;
        let bytes = serde_json::to_vec(&event).map_or(0, |value| value.len() as u64);
        aggregate.record_event(&event, bytes, config);
    }
    Ok(aggregate)
}

fn write_report(report: Report, output: &Path, json: bool) -> Result<(), String> {
    let text = serde_json::to_string_pretty(&report).map_err(|error| error.to_string())?;
    fs::write(output, &text)
        .map_err(|error| format!("Could not write {}: {error}", output.display()))?;
    if json {
        println!("{text}");
    } else {
        let review_fields = report
            .findings
            .iter()
            .map(|finding| finding.path.as_str())
            .collect::<HashSet<_>>()
            .len();
        println!(
            "Reviewed {} events in {}s. {} fields. {} findings across {} fields.\nReport: {}",
            report.events,
            report.sample_seconds,
            report.fields.len(),
            report.findings.len(),
            review_fields,
            output.display()
        );
    }
    Ok(())
}

struct ListenOptions {
    duration: u64,
    port: u16,
    output: PathBuf,
    retention_days: Vec<u32>,
    save_sample: Option<PathBuf>,
    detector: DetectorConfig,
    rate_limit: u32,
    json: bool,
}

fn listen(options: ListenOptions) -> Result<(), String> {
    validate_distinct_output_paths(&options.output, options.save_sample.as_deref())?;
    let listener = bind_local(options.port)?;
    listener
        .set_nonblocking(true)
        .map_err(|error| error.to_string())?;
    RUNNING.store(true, Ordering::SeqCst);
    ctrlc::set_handler(|| RUNNING.store(false, Ordering::SeqCst))
        .map_err(|error| format!("Could not install Ctrl-C handler: {error}"))?;
    eprintln!(
        "Drain Check listens only on http://127.0.0.1:{}/ for {}s. Press Ctrl-C to finish early and write the report.",
        options.port, options.duration
    );
    run_listener(
        listener,
        Duration::from_secs(options.duration),
        options.duration,
        &options,
        &RUNNING,
    )
}

fn validate_distinct_output_paths(output: &Path, save_sample: Option<&Path>) -> Result<(), String> {
    let Some(sample) = save_sample else {
        return Ok(());
    };
    if comparable_path(output)? == comparable_path(sample)? {
        return Err(
            "--output and --save-sample must name different files so the report cannot overwrite accepted bodies."
                .to_string(),
        );
    }
    Ok(())
}

fn comparable_path(path: &Path) -> Result<PathBuf, String> {
    if path.exists() {
        return fs::canonicalize(path)
            .map_err(|error| format!("Could not resolve {}: {error}", path.display()));
    }

    let parent = path.parent().unwrap_or_else(|| Path::new("."));
    let filename = path
        .file_name()
        .ok_or_else(|| format!("{} must name a file", path.display()))?;
    let parent = fs::canonicalize(parent)
        .map_err(|error| format!("Could not resolve {}: {error}", parent.display()))?;
    Ok(parent.join(filename))
}

fn bind_local(port: u16) -> Result<TcpListener, String> {
    TcpListener::bind((IpAddr::V4(Ipv4Addr::LOCALHOST), port))
        .map_err(|error| format!("Could not bind 127.0.0.1:{port}: {error}"))
}

fn run_listener(
    listener: TcpListener,
    window: Duration,
    report_seconds: u64,
    options: &ListenOptions,
    running: &AtomicBool,
) -> Result<(), String> {
    let started = Instant::now();
    let end = started + window;
    let mut aggregate = Aggregate::default();
    let mut limiter = RateLimiter::new(options.rate_limit as usize);
    let mut sample_file = options
        .save_sample
        .as_ref()
        .map(|path| {
            File::create(path)
                .map_err(|error| format!("Could not create {}: {error}", path.display()))
        })
        .transpose()?;

    while Instant::now() < end && running.load(Ordering::SeqCst) {
        match listener.accept() {
            Ok((mut stream, _address)) => match read_request(&mut stream) {
                Ok(request) => {
                    if !limiter.accepts(Instant::now()) {
                        let _ = write_response(
                            &mut stream,
                            429,
                            "Too Many Requests",
                            "Rate limit reached. Retry in one second.\n",
                            Some(("Retry-After", "1")),
                        );
                        continue;
                    }
                    aggregate_ndjson_body(&request.body, &mut aggregate, &options.detector);
                    if let Some(file) = sample_file.as_mut() {
                        file.write_all(request.body.as_bytes())
                            .and_then(|()| {
                                if request.body.ends_with('\n') {
                                    Ok(())
                                } else {
                                    file.write_all(b"\n")
                                }
                            })
                            .map_err(|error| format!("Could not save accepted sample: {error}"))?;
                    }
                    let _ = write_response(&mut stream, 202, "Accepted", "", None);
                }
                Err(error) => {
                    let body = format!("{}\n", error.message);
                    let _ = write_response(&mut stream, error.status, error.reason, &body, None);
                    eprintln!("Rejected request: {}", error.message);
                }
            },
            Err(error) if error.kind() == std::io::ErrorKind::WouldBlock => {
                std::thread::sleep(Duration::from_millis(10));
            }
            Err(error) => return Err(format!("Could not accept request: {error}")),
        }
    }

    if let Some(file) = sample_file.as_mut() {
        file.flush()
            .map_err(|error| format!("Could not finish sample file: {error}"))?;
    }
    let elapsed_seconds = started.elapsed().as_secs().clamp(1, report_seconds);
    let report = aggregate.report(
        elapsed_seconds,
        &options.retention_days,
        options.save_sample.is_some(),
    );
    write_report(report, &options.output, options.json)
}

struct AcceptedRequest {
    body: String,
}

#[derive(Debug)]
struct RequestError {
    status: u16,
    reason: &'static str,
    message: String,
}

impl RequestError {
    fn bad_request(message: impl Into<String>) -> Self {
        Self {
            status: 400,
            reason: "Bad Request",
            message: message.into(),
        }
    }
}

fn read_request(stream: &mut TcpStream) -> Result<AcceptedRequest, RequestError> {
    stream
        .set_read_timeout(Some(READ_TIMEOUT))
        .map_err(|error| {
            RequestError::bad_request(format!("Could not set read timeout: {error}"))
        })?;
    let mut raw = Vec::new();
    let mut chunk = [0_u8; 4096];
    let header_end;
    loop {
        let count = stream.read(&mut chunk).map_err(|error| {
            RequestError::bad_request(format!("Could not read headers: {error}"))
        })?;
        if count == 0 {
            return Err(RequestError::bad_request(
                "Request ended before headers arrived.",
            ));
        }
        raw.extend_from_slice(&chunk[..count]);
        if let Some(end) = raw.windows(4).position(|part| part == b"\r\n\r\n") {
            header_end = end + 4;
            break;
        }
        if raw.len() > MAX_HEADER_BYTES {
            return Err(RequestError {
                status: 431,
                reason: "Request Header Fields Too Large",
                message: "Request headers exceed 32 KiB.".to_string(),
            });
        }
    }

    let headers = std::str::from_utf8(&raw[..header_end])
        .map_err(|_| RequestError::bad_request("Request headers must be UTF-8."))?;
    let request_line = headers.lines().next().unwrap_or_default();
    if !request_line.starts_with("POST ") {
        return Err(RequestError {
            status: 405,
            reason: "Method Not Allowed",
            message: "Send the drain sample with POST.".to_string(),
        });
    }
    let length_header = headers.lines().skip(1).find_map(|line| {
        let (name, value) = line.split_once(':')?;
        name.eq_ignore_ascii_case("content-length")
            .then_some(value.trim())
    });
    let length = length_header
        .ok_or_else(|| RequestError::bad_request("Content-Length is required."))?
        .parse::<usize>()
        .map_err(|_| RequestError::bad_request("Content-Length must be a whole number."))?;
    if length > MAX_BODY_BYTES {
        return Err(RequestError {
            status: 413,
            reason: "Content Too Large",
            message: "Request body exceeds the 2 MiB sample limit.".to_string(),
        });
    }

    while raw.len() - header_end < length {
        let count = stream.read(&mut chunk).map_err(|error| {
            RequestError::bad_request(format!(
                "Request ended before its declared body length: {error}"
            ))
        })?;
        if count == 0 {
            return Err(RequestError::bad_request(
                "Request ended before its declared body length.",
            ));
        }
        raw.extend_from_slice(&chunk[..count]);
    }
    let body = std::str::from_utf8(&raw[header_end..header_end + length])
        .map_err(|_| RequestError::bad_request("Request body must be UTF-8."))?
        .to_string();
    validate_ndjson_body(&body)?;
    Ok(AcceptedRequest { body })
}

fn validate_ndjson_body(body: &str) -> Result<(), RequestError> {
    let mut event_count = 0;
    for (index, line) in body.lines().enumerate() {
        if line.trim().is_empty() {
            continue;
        }
        serde_json::from_str::<Value>(line).map_err(|error| {
            RequestError::bad_request(format!("Invalid JSON on line {}: {error}", index + 1))
        })?;
        event_count += 1;
    }
    if event_count == 0 {
        return Err(RequestError::bad_request(
            "Request body contains no JSON events.",
        ));
    }
    Ok(())
}

fn aggregate_ndjson_body(body: &str, aggregate: &mut Aggregate, config: &DetectorConfig) {
    for line in body.lines().filter(|line| !line.trim().is_empty()) {
        // Validation is transactional and completes before this function runs.
        let event: Value = serde_json::from_str(line).expect("validated NDJSON changed");
        let bytes = serde_json::to_vec(&event).map_or(0, |value| value.len() as u64);
        aggregate.record_event(&event, bytes, config);
    }
}

fn write_response(
    stream: &mut TcpStream,
    status: u16,
    reason: &str,
    body: &str,
    extra_header: Option<(&str, &str)>,
) -> std::io::Result<()> {
    let extra = extra_header
        .map(|(name, value)| format!("{name}: {value}\r\n"))
        .unwrap_or_default();
    write!(
        stream,
        "HTTP/1.1 {status} {reason}\r\nContent-Type: text/plain; charset=utf-8\r\nContent-Length: {}\r\n{extra}Connection: close\r\n\r\n{body}",
        body.len()
    )
}

struct RateLimiter {
    limit: usize,
    accepted: VecDeque<Instant>,
}

impl RateLimiter {
    fn new(limit: usize) -> Self {
        Self {
            limit,
            accepted: VecDeque::with_capacity(limit),
        }
    }

    fn accepts(&mut self, now: Instant) -> bool {
        while self
            .accepted
            .front()
            .is_some_and(|earliest| now.duration_since(*earliest) >= Duration::from_secs(1))
        {
            self.accepted.pop_front();
        }
        if self.accepted.len() >= self.limit {
            return false;
        }
        self.accepted.push_back(now);
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{net::Shutdown, sync::Arc, thread};

    #[test]
    fn rejects_malformed_payload_transactionally() {
        assert!(validate_ndjson_body("{\"ok\":true}\n{not-json").is_err());
        assert!(validate_ndjson_body("{\"ok\":true}\n").is_ok());
    }

    #[test]
    fn rate_limit_has_a_rolling_threshold() {
        let start = Instant::now();
        let mut limiter = RateLimiter::new(2);
        assert!(limiter.accepts(start));
        assert!(limiter.accepts(start));
        assert!(!limiter.accepts(start));
        assert!(limiter.accepts(start + Duration::from_secs(1)));
    }

    #[test]
    fn rate_limit_returns_429_with_retry_after() {
        let cli = Cli::try_parse_from(["drain-check", "listen"]).unwrap();
        let Command::Listen { rate_limit, .. } = cli.command else {
            unreachable!();
        };
        assert_eq!(rate_limit, 20);

        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        listener.set_nonblocking(true).unwrap();
        let address = listener.local_addr().unwrap();
        let directory = tempfile::tempdir().unwrap();
        let options = ListenOptions {
            duration: 1,
            port: address.port(),
            output: directory.path().join("report.json"),
            retention_days: vec![7],
            save_sample: None,
            detector: DetectorConfig::default(),
            rate_limit,
            json: false,
        };
        let running = Arc::new(AtomicBool::new(true));
        let server_running = Arc::clone(&running);
        let server = thread::spawn(move || {
            run_listener(
                listener,
                Duration::from_secs(2),
                1,
                &options,
                &server_running,
            )
            .unwrap();
        });
        thread::sleep(Duration::from_millis(30));
        for event in 1..=rate_limit {
            let accepted = send(address, &format!("{{\"event\":{event}}}"), None);
            assert!(
                accepted.starts_with("HTTP/1.1 202"),
                "request {event} should be accepted: {accepted}"
            );
        }
        let limited = send(address, "{\"event\":21}", None);
        assert!(limited.starts_with("HTTP/1.1 429"));
        assert!(limited.contains("Retry-After: 1"));
        running.store(false, Ordering::SeqCst);
        server.join().unwrap();
        let report: Report = serde_json::from_str(
            &fs::read_to_string(directory.path().join("report.json")).unwrap(),
        )
        .unwrap();
        assert_eq!(report.events, rate_limit as usize);
    }

    #[test]
    fn cli_rejects_invalid_listener_and_forwarding_values() {
        assert!(Cli::try_parse_from(["drain-check", "listen", "--duration", "0"]).is_err());
        assert!(Cli::try_parse_from(["drain-check", "listen", "--port", "0"]).is_err());
        assert!(Cli::try_parse_from([
            "drain-check",
            "inspect",
            "sample.ndjson",
            "--sample-seconds",
            "0"
        ])
        .is_err());
        assert!(Cli::try_parse_from(["drain-check", "forwarding", "--url", "not a url"]).is_err());
    }

    #[test]
    fn forwarding_requires_a_real_http_url_and_encodes_it_safely() {
        for value in [
            "http://:",
            "https://?query",
            "not a url",
            "ftp://example.com",
        ] {
            assert!(
                validate_http_url(value).is_err(),
                "{value} should be rejected"
            );
        }
        let url = validate_http_url("https://example.com/\"").unwrap();
        let config = forwarding_config(&url, "generic-http");
        assert!(config.contains("url = \"https://example.com/%22\""));
        assert!(!config.contains("%22\"\""));
    }

    #[test]
    fn colliding_output_and_sample_paths_are_rejected_before_binding() {
        let directory = tempfile::tempdir().unwrap();
        let shared = directory.path().join("accepted.ndjson");
        fs::write(&shared, "{\"event\":\"saved\"}\n").unwrap();
        let occupied = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        let options = ListenOptions {
            duration: 1,
            port: occupied.local_addr().unwrap().port(),
            output: shared.clone(),
            retention_days: vec![7],
            save_sample: Some(shared.clone()),
            detector: DetectorConfig::default(),
            rate_limit: 20,
            json: false,
        };

        let error = listen(options).unwrap_err();
        assert!(error.contains("--output and --save-sample must name different files"));
        assert_eq!(
            fs::read_to_string(shared).unwrap(),
            "{\"event\":\"saved\"}\n"
        );
    }

    #[test]
    fn listener_binds_to_loopback() {
        let listener = bind_local(0).unwrap();
        assert!(listener.local_addr().unwrap().ip().is_loopback());
    }

    #[test]
    fn receiver_rejects_bad_requests_and_keeps_prior_events() {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        listener.set_nonblocking(true).unwrap();
        let address = listener.local_addr().unwrap();
        let directory = tempfile::tempdir().unwrap();
        let output = directory.path().join("report.json");
        let options = ListenOptions {
            duration: 1,
            port: address.port(),
            output: output.clone(),
            retention_days: vec![7],
            save_sample: None,
            detector: DetectorConfig::default(),
            rate_limit: 20,
            json: false,
        };
        let running = Arc::new(AtomicBool::new(true));
        let server_running = Arc::clone(&running);
        let server = thread::spawn(move || {
            run_listener(
                listener,
                Duration::from_millis(600),
                1,
                &options,
                &server_running,
            )
            .unwrap();
        });

        thread::sleep(Duration::from_millis(30));
        let good = send(address, "{\"ok\":true}\n", None);
        assert!(good.starts_with("HTTP/1.1 202"));
        let bad = send(address, "{not-json\n", None);
        assert!(bad.starts_with("HTTP/1.1 400"));
        let short = send(address, "{\"late\":true}", Some(100));
        assert!(
            short.starts_with("HTTP/1.1 400"),
            "incomplete request must receive HTTP 400, got: {short:?}"
        );
        server.join().unwrap();

        let report: Report = serde_json::from_str(&fs::read_to_string(output).unwrap()).unwrap();
        assert_eq!(report.events, 1);
        assert!(!report.bodies_saved);
    }

    #[test]
    fn save_sample_writes_only_accepted_bodies_when_requested() {
        let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 0)).unwrap();
        listener.set_nonblocking(true).unwrap();
        let address = listener.local_addr().unwrap();
        let directory = tempfile::tempdir().unwrap();
        let output = directory.path().join("report.json");
        let sample = directory.path().join("accepted.ndjson");
        let options = ListenOptions {
            duration: 1,
            port: address.port(),
            output: output.clone(),
            retention_days: vec![7],
            save_sample: Some(sample.clone()),
            detector: DetectorConfig::default(),
            rate_limit: 20,
            json: false,
        };
        let running = Arc::new(AtomicBool::new(true));
        let server_running = Arc::clone(&running);
        let server = thread::spawn(move || {
            run_listener(
                listener,
                Duration::from_secs(2),
                1,
                &options,
                &server_running,
            )
            .unwrap();
        });
        thread::sleep(Duration::from_millis(30));
        assert!(send(address, "{not-json}", None).starts_with("HTTP/1.1 400"));
        assert!(send(address, "{\"saved\":true}", None).starts_with("HTTP/1.1 202"));
        running.store(false, Ordering::SeqCst);
        server.join().unwrap();
        assert_eq!(fs::read_to_string(sample).unwrap(), "{\"saved\":true}\n");
        let report: Report = serde_json::from_str(&fs::read_to_string(output).unwrap()).unwrap();
        assert!(report.bodies_saved);
    }

    fn send(address: std::net::SocketAddr, body: &str, declared: Option<usize>) -> String {
        let mut stream = TcpStream::connect(address).unwrap();
        let length = declared.unwrap_or(body.len());
        write!(
            stream,
            "POST / HTTP/1.1\r\nHost: localhost\r\nContent-Length: {length}\r\n\r\n{body}"
        )
        .unwrap();
        if declared.is_some() {
            stream.shutdown(Shutdown::Write).unwrap();
        }
        let mut response = String::new();
        stream.read_to_string(&mut response).unwrap();
        response
    }
}
