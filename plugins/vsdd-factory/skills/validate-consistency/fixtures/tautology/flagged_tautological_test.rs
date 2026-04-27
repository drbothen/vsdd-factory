// FIXTURE: this file is NOT compiled. It documents a tautological test
// shape that Check 8 of the validate-consistency skill must flag.
//
// Expected detector verdict: FLAG (severity MEDIUM)
// Reason: constructs `LogEntry` and asserts on its own fields without
// ever invoking the production emitter (`emit_log_entry`, `serialize_log_entry`,
// `build_log_entry`, etc.). The test would pass even if the production
// crate were emptied.

#[cfg(test)]
mod tests {
    // Imagine LogEntry lives in `crate::entry` and is serialized by
    // `crate::emitter::emit_log_entry(&entry) -> String`.
    use crate::entry::LogEntry;

    #[test]
    fn test_BC_3_02_001_log_entry_has_canonical_fields() {
        // Body matches the tautology shape:
        //   1. struct literal binding
        //   2. assertions on its own fields
        //   3. zero calls to `emit_*` / `serialize_*` / `build_*` / etc.
        let entry = LogEntry {
            ts: 1_700_000_000,
            level: "info".to_string(),
            message: "ok".to_string(),
            trace_id: None,
        };

        assert_eq!(entry.ts, 1_700_000_000);
        assert_eq!(entry.level, "info");
        assert_eq!(entry.message, "ok");
        assert!(entry.trace_id.is_none());
    }

    #[test]
    fn test_TV_dlq_envelope_includes_sink_name() {
        // Same shape, different domain — also a tautology.
        let envelope = DlqEnvelope {
            sink: "datadog".into(),
            reason: "5xx".into(),
            payload: vec![],
        };

        assert_eq!(envelope.sink, "datadog");
        assert_eq!(envelope.reason, "5xx");
        assert!(envelope.payload.is_empty());
    }
}

// Dummy types to keep this fixture readable in isolation.
struct DlqEnvelope {
    sink: String,
    reason: String,
    payload: Vec<u8>,
}
