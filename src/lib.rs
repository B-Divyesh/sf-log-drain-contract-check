//! Local log-drain contract analysis.
//!
//! ```
//! use drain_check::analyse_events;
//!
//! let events = vec![serde_json::json!({"level": "info", "ok": true})];
//! let report = analyse_events(&events, 60, &[7, 30]);
//! assert_eq!(report.events, 1);
//! assert_eq!(report.fields.len(), 3);
//! ```

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
    /// Number of events containing this path, not the number of array occurrences.
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

#[derive(Debug, Clone)]
pub struct DetectorConfig {
    pub sensitive_field_patterns: Vec<String>,
    pub ignored_paths: Vec<String>,
}

impl Default for DetectorConfig {
    fn default() -> Self {
        Self {
            sensitive_field_patterns: [
                "password",
                "passwd",
                "secret",
                "token",
                "api_key",
                "apikey",
                "authorization",
                "cookie",
                "ssn",
                "credit_card",
                "card_number",
            ]
            .into_iter()
            .map(str::to_owned)
            .collect(),
            ignored_paths: Vec::new(),
        }
    }
}

impl DetectorConfig {
    pub fn with_overrides(custom_patterns: &[String], ignored_paths: &[String]) -> Self {
        let mut config = Self::default();
        config.sensitive_field_patterns.extend(
            custom_patterns
                .iter()
                .map(|pattern| pattern.to_ascii_lowercase()),
        );
        config.ignored_paths = ignored_paths.to_vec();
        config
    }

    fn ignores(&self, path: &str) -> bool {
        self.ignored_paths
            .iter()
            .any(|pattern| path_matches(pattern, path))
    }
}

/// A streaming summary that never retains parsed events or their values.
#[derive(Debug, Default)]
pub struct Aggregate {
    events: usize,
    bytes: u64,
    fields: BTreeMap<String, (BTreeSet<String>, usize)>,
    findings: BTreeMap<(String, String), Finding>,
}

impl Aggregate {
    pub fn record_event(&mut self, event: &Value, serialized_bytes: u64, config: &DetectorConfig) {
        let mut event_fields: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        self.events += 1;
        self.bytes = self.bytes.saturating_add(serialized_bytes);
        inspect_value(event, "$", &mut event_fields, &mut self.findings, config);

        for (path, types) in event_fields {
            let entry = self.fields.entry(path).or_default();
            entry.0.extend(types);
            entry.1 += 1;
        }
    }

    pub fn report(self, sample_seconds: u64, retention_days: &[u32], bodies_saved: bool) -> Report {
        let average_event_bytes = if self.events == 0 {
            0
        } else {
            self.bytes / self.events as u64
        };
        let events_per_second = if sample_seconds == 0 {
            0.0
        } else {
            self.events as f64 / sample_seconds as f64
        };
        let per_day = (events_per_second * 86_400.0 * average_event_bytes as f64).round() as u64;

        Report {
            events: self.events,
            sample_seconds,
            events_per_second,
            average_event_bytes,
            fields: self
                .fields
                .into_iter()
                .map(|(path, (types, present_in))| FieldSummary {
                    path,
                    types: types.into_iter().collect(),
                    present_in,
                })
                .collect(),
            findings: self.findings.into_values().collect(),
            retention: retention_days
                .iter()
                .map(|days| {
                    let estimated_bytes = per_day.saturating_mul(*days as u64);
                    RetentionEstimate {
                        days: *days,
                        estimated_bytes,
                        display: bytes_display(estimated_bytes),
                    }
                })
                .collect(),
            bodies_saved,
            forwarding:
                "Forward only after review. Keep the receiving URL in your platform's secret store."
                    .to_string(),
        }
    }
}

pub fn analyse_events(events: &[Value], sample_seconds: u64, retention_days: &[u32]) -> Report {
    analyse_events_with_config(
        events,
        sample_seconds,
        retention_days,
        &DetectorConfig::default(),
    )
}

pub fn analyse_events_with_config(
    events: &[Value],
    sample_seconds: u64,
    retention_days: &[u32],
    config: &DetectorConfig,
) -> Report {
    let mut aggregate = Aggregate::default();
    for event in events {
        let bytes = serde_json::to_vec(event).map_or(0, |value| value.len() as u64);
        aggregate.record_event(event, bytes, config);
    }
    aggregate.report(sample_seconds, retention_days, false)
}

fn inspect_value(
    value: &Value,
    path: &str,
    event_fields: &mut BTreeMap<String, BTreeSet<String>>,
    findings: &mut BTreeMap<(String, String), Finding>,
    config: &DetectorConfig,
) {
    let type_name = match value {
        Value::Null => "null",
        Value::Bool(_) => "boolean",
        Value::Number(number) if number.is_i64() || number.is_u64() => "integer",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    };
    event_fields
        .entry(path.to_string())
        .or_default()
        .insert(type_name.to_string());

    let ignored = config.ignores(path);
    let key = path.rsplit('.').next().unwrap_or("").to_ascii_lowercase();
    if !ignored
        && config
            .sensitive_field_patterns
            .iter()
            .any(|word| key.contains(word))
    {
        add_finding(
            findings,
            path,
            "sensitive field name",
            "high",
            "Review this field before forwarding.",
        );
    }

    match value {
        Value::Object(map) => {
            for (key, child) in map {
                inspect_value(
                    child,
                    &format!("{path}.{key}"),
                    event_fields,
                    findings,
                    config,
                );
            }
        }
        Value::Array(items) => {
            for child in items {
                inspect_value(child, &format!("{path}[]"), event_fields, findings, config);
            }
        }
        Value::String(text) if !ignored => {
            if looks_like_secret(text) {
                add_finding(
                    findings,
                    path,
                    "secret-shaped value",
                    "high",
                    "Mask this value or exclude the field.",
                );
            } else if looks_like_email(text) {
                add_finding(
                    findings,
                    path,
                    "email-shaped value",
                    "medium",
                    "Confirm that this identifier belongs in the drain.",
                );
            }
        }
        _ => {}
    }
}

fn path_matches(pattern: &str, path: &str) -> bool {
    if pattern == path {
        return true;
    }
    pattern
        .strip_suffix('*')
        .is_some_and(|prefix| path.starts_with(prefix))
}

fn add_finding(
    findings: &mut BTreeMap<(String, String), Finding>,
    path: &str,
    detector: &str,
    confidence: &str,
    action: &str,
) {
    let finding = Finding {
        path: path.to_string(),
        detector: detector.to_string(),
        confidence: confidence.to_string(),
        action: action.to_string(),
    };
    findings.insert((path.to_string(), detector.to_string()), finding);
}

fn looks_like_secret(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    lower.starts_with("bearer ")
        || lower.starts_with("sk_")
        || lower.starts_with("ghp_")
        || lower.starts_with("xoxb-")
        || (text.len() >= 32
            && text.chars().all(|character| {
                character.is_ascii_alphanumeric() || character == '_' || character == '-'
            }))
}

fn looks_like_email(text: &str) -> bool {
    let parts: Vec<_> = text.split('@').collect();
    parts.len() == 2 && !parts[0].is_empty() && parts[1].contains('.') && !text.contains(' ')
}

pub fn bytes_display(bytes: u64) -> String {
    if bytes >= 1_073_741_824 {
        format!("{:.1} GiB", bytes as f64 / 1_073_741_824.0)
    } else if bytes >= 1_048_576 {
        format!("{:.1} MiB", bytes as f64 / 1_048_576.0)
    } else if bytes >= 1024 {
        format!("{:.1} KiB", bytes as f64 / 1024.0)
    } else {
        format!("{bytes} B")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn documented_sample_has_exact_metrics() {
        let sample: Vec<Value> = include_str!("../examples/drain.ndjson")
            .lines()
            .map(|line| serde_json::from_str(line).unwrap())
            .collect();
        let report = analyse_events(&sample, 600, &[7, 30]);
        assert_eq!(report.events, 3);
        assert_eq!(report.average_event_bytes, 189);
        assert_eq!(report.fields.len(), 17);
        assert_eq!(report.retention[0].display, "558.1 KiB");
        assert_eq!(report.retention[1].display, "2.3 MiB");
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.path == "$.request.authorization"));
    }

    #[test]
    fn detects_without_storing_values() {
        let events = vec![serde_json::json!({
            "request":{"authorization":"Bearer this_is_a_fake_token_12345678901234567890"},
            "user":"sam@example.test",
            "latency_ms":12
        })];
        let report = analyse_events(&events, 60, &[7, 30]);
        let rendered = serde_json::to_string(&report).unwrap();
        assert!(!rendered.contains("fake_token"));
        assert!(!rendered.contains("sam@example"));
    }

    #[test]
    fn counts_array_path_once_per_event() {
        let events = vec![serde_json::json!({"items":[{"id":1},{"id":2}]})];
        let report = analyse_events(&events, 60, &[7]);
        let field = report
            .fields
            .iter()
            .find(|field| field.path == "$.items[].id")
            .unwrap();
        assert_eq!(field.present_in, 1);
    }

    #[test]
    fn supports_custom_patterns_and_explicit_suppression() {
        let events = vec![serde_json::json!({
            "session_key":"ordinary-value",
            "request_id":"12345678901234567890123456789012"
        })];
        let config =
            DetectorConfig::with_overrides(&["session_key".into()], &["$.request_id".into()]);
        let report = analyse_events_with_config(&events, 60, &[7], &config);
        assert!(report
            .findings
            .iter()
            .any(|finding| finding.path == "$.session_key"));
        assert!(!report
            .findings
            .iter()
            .any(|finding| finding.path == "$.request_id"));
    }
}
