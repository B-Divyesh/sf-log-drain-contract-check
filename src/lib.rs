use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Finding {
    pub path: String,
    pub detector: String,
    pub confidence: String,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldSummary {
    pub path: String,
    pub types: Vec<String>,
    pub present_in: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetentionEstimate {
    pub days: u32,
    pub estimated_bytes: u64,
    pub display: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Report {
    pub events: usize,
    pub sample_seconds: u64,
    pub events_per_second: f64,
    pub average_event_bytes: u64,
    pub fields: Vec<FieldSummary>,
    pub findings: Vec<Finding>,
    pub retention: Vec<RetentionEstimate>,
    pub bodies_saved: bool,
    pub forwarding: String,
}

#[derive(Default)]
struct Aggregate {
    events: usize,
    bytes: u64,
    fields: BTreeMap<String, (BTreeSet<String>, usize)>,
    findings: BTreeMap<(String, String), Finding>,
}

pub fn analyse_events(events: &[Value], sample_seconds: u64, retention_days: &[u32]) -> Report {
    let mut aggregate = Aggregate::default();
    for event in events {
        aggregate.events += 1;
        aggregate.bytes += serde_json::to_vec(event).map_or(0, |v| v.len() as u64);
        inspect_value(event, "$", &mut aggregate);
    }
    let average_event_bytes = if aggregate.events == 0 { 0 } else { aggregate.bytes / aggregate.events as u64 };
    let events_per_second = if sample_seconds == 0 { 0.0 } else { aggregate.events as f64 / sample_seconds as f64 };
    let per_day = (events_per_second * 86_400.0 * average_event_bytes as f64).round() as u64;
    Report {
        events: aggregate.events,
        sample_seconds,
        events_per_second,
        average_event_bytes,
        fields: aggregate.fields.into_iter().map(|(path, (types, present_in))| FieldSummary { path, types: types.into_iter().collect(), present_in }).collect(),
        findings: aggregate.findings.into_values().collect(),
        retention: retention_days.iter().map(|days| RetentionEstimate { days: *days, estimated_bytes: per_day.saturating_mul(*days as u64), display: bytes_display(per_day.saturating_mul(*days as u64)) }).collect(),
        bodies_saved: false,
        forwarding: "Forward only after review. Keep the receiving URL in your platform's secret store.".to_string(),
    }
}

fn inspect_value(value: &Value, path: &str, aggregate: &mut Aggregate) {
    let type_name = match value { Value::Null => "null", Value::Bool(_) => "boolean", Value::Number(n) if n.is_i64() || n.is_u64() => "integer", Value::Number(_) => "number", Value::String(_) => "string", Value::Array(_) => "array", Value::Object(_) => "object" };
    let entry = aggregate.fields.entry(path.to_string()).or_insert_with(|| (BTreeSet::new(), 0));
    entry.0.insert(type_name.to_string()); entry.1 += 1;
    let key = path.rsplit('.').next().unwrap_or("").to_ascii_lowercase();
    if is_sensitive_key(&key) {
        add_finding(aggregate, path, "sensitive field name", "high", "Review this field before forwarding.");
    }
    match value {
        Value::Object(map) => for (key, child) in map { inspect_value(child, &format!("{path}.{key}"), aggregate); },
        Value::Array(items) => for child in items { inspect_value(child, &format!("{path}[]"), aggregate); },
        Value::String(text) => {
            if looks_like_secret(text) { add_finding(aggregate, path, "secret-shaped value", "high", "Mask this value or exclude the field."); }
            else if looks_like_email(text) { add_finding(aggregate, path, "email-shaped value", "medium", "Confirm that this identifier belongs in the drain."); }
        }
        _ => {}
    }
}

fn add_finding(a: &mut Aggregate, path: &str, detector: &str, confidence: &str, action: &str) {
    let finding = Finding { path: path.to_string(), detector: detector.to_string(), confidence: confidence.to_string(), action: action.to_string() };
    a.findings.insert((path.to_string(), detector.to_string()), finding);
}

fn is_sensitive_key(key: &str) -> bool {
    ["password", "passwd", "secret", "token", "api_key", "apikey", "authorization", "cookie", "ssn", "credit_card", "card_number"].iter().any(|word| key.contains(word))
}
fn looks_like_secret(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.starts_with("bearer ") || lower.starts_with("sk_") || lower.starts_with("ghp_") || lower.starts_with("xoxb-") || (text.len() >= 32 && text.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-'))
}
fn looks_like_email(text: &str) -> bool { let parts: Vec<_> = text.split('@').collect(); parts.len() == 2 && parts[0].len() > 0 && parts[1].contains('.') && !text.contains(' ') }
pub fn bytes_display(bytes: u64) -> String { if bytes >= 1_073_741_824 { format!("{:.1} GiB", bytes as f64 / 1_073_741_824.0) } else if bytes >= 1_048_576 { format!("{:.1} MiB", bytes as f64 / 1_048_576.0) } else if bytes >= 1024 { format!("{:.1} KiB", bytes as f64 / 1024.0) } else { format!("{bytes} B") } }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn documented_sample_finds_contract_risks() {
        let sample: Vec<Value> = include_str!("../examples/drain.ndjson").lines().map(|line| serde_json::from_str(line).unwrap()).collect();
        let report = analyse_events(&sample, 600, &[7]);
        assert_eq!(report.events, 3);
        assert!(report.findings.iter().any(|f| f.path == "$.request.authorization"));
    }
    #[test]
    fn detects_key_and_value_without_storing_values() {
        let events = vec![serde_json::json!({"request":{"authorization":"Bearer this_is_a_fake_token_12345678901234567890"},"user":"sam@example.test","latency_ms":12})];
        let report = analyse_events(&events, 60, &[7, 30]);
        assert!(report.findings.iter().any(|f| f.path == "$.request.authorization"));
        assert!(report.findings.iter().any(|f| f.detector == "email-shaped value"));
        let rendered = serde_json::to_string(&report).unwrap();
        assert!(!rendered.contains("fake_token"));
    }
    #[test]
    fn computes_retention() {
        let report = analyse_events(&vec![serde_json::json!({"ok":true})], 1, &[1]);
        assert!(report.retention[0].estimated_bytes > 0);
    }
}
