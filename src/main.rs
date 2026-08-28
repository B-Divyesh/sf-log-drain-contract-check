use clap::{Parser, Subcommand};
use drain_check::analyse_events;
use serde_json::Value;
use std::{fs, io::{Read, Write}, net::{IpAddr, Ipv4Addr, TcpListener, TcpStream}, path::PathBuf, time::{Duration, Instant}};

#[derive(Parser)]
#[command(name="drain-check", version, about="Sample a local log drain and review its data contract.")]
struct Cli { #[command(subcommand)] command: Command }
#[derive(Subcommand)]
enum Command {
    /// Receive a bounded local sample. Bodies are discarded after aggregation.
    Listen { #[arg(long, default_value_t=600)] duration: u64, #[arg(long, default_value_t=8787)] port: u16, #[arg(long, default_value="report.json")] output: PathBuf, #[arg(long, value_delimiter=',', default_value="7,30")] retention_days: Vec<u32>, #[arg(long)] save_sample: Option<PathBuf>, #[arg(long)] json: bool },
    /// Analyse newline-delimited JSON from a file without opening a listener.
    Inspect { input: PathBuf, #[arg(long, default_value_t=600)] sample_seconds: u64, #[arg(long, default_value="report.json")] output: PathBuf, #[arg(long, value_delimiter=',', default_value="7,30")] retention_days: Vec<u32>, #[arg(long)] json: bool },
    /// Run the bundled sample in a temporary report directory.
    Demo { #[arg(long)] json: bool },
    /// Print a forwarding configuration template.
    Forwarding { #[arg(long, default_value="https://receiver.example/logs")] url: String, #[arg(long, default_value="generic-http")] platform: String },
}

fn main() -> Result<(), String> {
    let cli = Cli::parse();
    match cli.command {
        Command::Inspect { input, sample_seconds, output, retention_days, json } => { let events = read_ndjson(&input)?; write_report(events, sample_seconds, retention_days, output, json, false) }
        Command::Demo { json } => { let output = std::env::temp_dir().join("drain-check-demo-report.json"); let events = read_ndjson(&PathBuf::from("examples/drain.ndjson"))?; write_report(events, 600, vec![7,30], output, json, false) }
        Command::Forwarding { url, platform } => { println!("# {platform}\n# Send POST requests to this endpoint after contract review\nurl = \"{url}\"\nmethod = \"POST\"\ncontent_type = \"application/json\""); Ok(()) }
        Command::Listen { duration, port, output, retention_days, save_sample, json } => listen(duration, port, output, retention_days, save_sample, json),
    }
}

fn write_report(events: Vec<Value>, seconds: u64, days: Vec<u32>, output: PathBuf, json: bool, bodies_saved: bool) -> Result<(), String> {
    let mut report = analyse_events(&events, seconds, &days);
    report.bodies_saved = bodies_saved;
    let text = serde_json::to_string_pretty(&report).map_err(|e| e.to_string())?;
    fs::write(&output, &text).map_err(|e| format!("Could not write {}: {e}", output.display()))?;
    if json { println!("{text}"); } else { println!("Reviewed {} events in {}s. {} fields. {} possible risks.\nReport: {}", report.events, seconds, report.fields.len(), report.findings.len(), output.display()); }
    Ok(())
}
fn read_ndjson(path: &PathBuf) -> Result<Vec<Value>, String> {
    let data = fs::read_to_string(path).map_err(|e| format!("Could not read {}: {e}", path.display()))?;
    data.lines().filter(|line| !line.trim().is_empty()).enumerate().map(|(i,line)| serde_json::from_str(line).map_err(|e| format!("Invalid JSON on line {}: {e}",i+1))).collect()
}
fn listen(duration: u64, port: u16, output: PathBuf, days: Vec<u32>, save_sample: Option<PathBuf>, json: bool) -> Result<(), String> {
    let listener = TcpListener::bind((IpAddr::V4(Ipv4Addr::LOCALHOST), port)).map_err(|e| format!("Could not bind 127.0.0.1:{port}: {e}"))?;
    listener.set_nonblocking(true).map_err(|e| e.to_string())?;
    eprintln!("Drain Check listens only on http://127.0.0.1:{port}/ for {duration}s. Press Ctrl-C to stop early.");
    let end = Instant::now() + Duration::from_secs(duration); let mut events = vec![]; let mut saved = vec![];
    while Instant::now() < end {
        match listener.accept() { Ok((stream,_)) => { let accepted = read_request(stream)?; if let Some(raw) = accepted.1 { saved.push(raw); } events.extend(accepted.0); }, Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => std::thread::sleep(Duration::from_millis(50)), Err(e) => return Err(e.to_string()) }
    }
    let bodies_saved = save_sample.is_some();
    if let Some(path) = save_sample { fs::write(path, saved.join("\n")).map_err(|e| e.to_string())?; }
    write_report(events, duration, days, output, json, bodies_saved)
}
fn read_request(mut stream: TcpStream) -> Result<(Vec<Value>, Option<String>), String> {
    stream.set_read_timeout(Some(Duration::from_secs(2))).map_err(|e| e.to_string())?;
    let mut raw = Vec::new(); let mut chunk = [0_u8; 1024]; let header_end;
    loop {
        let count = stream.read(&mut chunk).map_err(|e| format!("Could not read request: {e}"))?;
        if count == 0 { return Err("Request ended before headers arrived.".to_string()); }
        raw.extend_from_slice(&chunk[..count]);
        if let Some(end) = raw.windows(4).position(|part| part == b"\r\n\r\n") { header_end = end + 4; break; }
        if raw.len() > 32 * 1024 { return Err("Request headers exceed 32 KiB.".to_string()); }
    }
    let headers = String::from_utf8_lossy(&raw[..header_end]);
    let length = headers.lines().find_map(|line| line.strip_prefix("Content-Length:").or_else(|| line.strip_prefix("content-length:")).and_then(|value| value.trim().parse::<usize>().ok())).unwrap_or(0);
    if length > 2 * 1024 * 1024 { return Err("Request body exceeds 2 MiB sample limit.".to_string()); }
    while raw.len() - header_end < length {
        let count = stream.read(&mut chunk).map_err(|e| format!("Could not read body: {e}"))?;
        if count == 0 { return Err("Request ended before its declared body length.".to_string()); }
        raw.extend_from_slice(&chunk[..count]);
    }
    let body = String::from_utf8_lossy(&raw[header_end..header_end + length]).to_string();
    let values = body.lines().filter_map(|line| serde_json::from_str(line).ok()).collect();
    stream.write_all(b"HTTP/1.1 202 Accepted\r\nContent-Length: 0\r\nConnection: close\r\n\r\n").map_err(|e| e.to_string())?;
    Ok((values, Some(body)))
}
