// FIXTURE: this file is NOT compiled. It pairs with
// `clean_bc_excludes_field.md` to exercise Check 9's negative path.
//
// Expected detector verdict: CLEAN
// Reason: same BC exclusion as the flagged fixture, but the struct below
// declares `pub trace_id: Option<String>` with
// `#[serde(skip_serializing_if = "Option::is_none")]`. When the DLQ
// builder sets `trace_id: None`, serde drops the field from the emitted
// JSON, satisfying the BC.

use serde::Serialize;

#[derive(Serialize)]
pub struct LogEntry {
    pub ts: u64,
    pub level: String,
    pub message: String,

    // CLEAN: Option<T> + skip_serializing_if. DLQ path sets this to None,
    // serde omits the key entirely.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
}

pub fn emit_log_entry(entry: &LogEntry) -> String {
    serde_json::to_string(entry).expect("LogEntry is always serializable")
}

// Example call sites that satisfy the BC:
pub fn build_normal_entry(ts: u64, msg: &str, trace: &str) -> LogEntry {
    LogEntry {
        ts,
        level: "info".into(),
        message: msg.into(),
        trace_id: Some(trace.into()),
    }
}

pub fn build_dlq_entry(ts: u64, msg: &str) -> LogEntry {
    LogEntry {
        ts,
        level: "error".into(),
        message: msg.into(),
        trace_id: None, // BC mandates omission; skip_serializing_if drops the key.
    }
}
