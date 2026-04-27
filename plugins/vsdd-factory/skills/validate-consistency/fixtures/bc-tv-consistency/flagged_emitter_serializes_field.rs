// FIXTURE: this file is NOT compiled. It pairs with
// `flagged_bc_excludes_field.md` to exercise Check 9's positive path.
//
// Expected detector verdict: FLAG (severity HIGH)
// Reason: BC-FIXTURE-FLAGGED.001's Canonical Test Vectors table marks
// `trace_id in Entry? = No` for retry-exhaustion and queue-overflow
// DLQ entries, but the struct below declares `pub trace_id: String,` —
// non-Option, no `skip_serializing_if`, no feature gate. Default serde
// derivation will emit `"trace_id": ""` for every entry, contradicting
// the BC.

use serde::Serialize;

#[derive(Serialize)]
pub struct LogEntry {
    pub ts: u64,
    pub level: String,
    pub message: String,

    // VIOLATION: BC says this is excluded for retry-exhaustion / queue-overflow,
    // but the field has no skip attribute and is non-Option, so serde will
    // always emit it. Check 9 should flag this.
    pub trace_id: String,
}

// Hand-rolled emitter that also serializes unconditionally — Check 9 should
// also catch this branch by inspecting the struct definition above.
pub fn emit_log_entry(entry: &LogEntry) -> String {
    serde_json::to_string(entry).expect("LogEntry is always serializable")
}
