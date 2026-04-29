// FIXTURE: this file is NOT compiled. It documents a NON-tautological
// test shape that Check 8 must NOT flag.
//
// Expected detector verdict: CLEAN
// Reason: the test still constructs a struct and asserts on results, but
// it routes the struct through `emit_log_entry`, exercising production
// code. The `emit_log_entry(...)` call satisfies condition 4 (production
// fn invoked) and breaks the tautology.

#[cfg(test)]
mod tests {
    use crate::entry::LogEntry;
    use crate::emitter::emit_log_entry;

    #[test]
    fn test_BC_3_02_001_log_entry_emits_canonical_payload() {
        let entry = LogEntry {
            ts: 1_700_000_000,
            level: "info".to_string(),
            message: "ok".to_string(),
            trace_id: None,
        };

        // emit_log_entry matches the production-fn regex (`emit_*`),
        // so condition 4 is satisfied — this is a real behavior test.
        let json = emit_log_entry(&entry);

        assert!(json.contains("\"ts\":1700000000"));
        assert!(json.contains("\"level\":\"info\""));
        assert!(!json.contains("trace_id"), "trace_id is absent when None");
    }

    #[test]
    fn test_TV_dlq_round_trip_through_serialize() {
        let envelope = DlqEnvelope {
            sink: "datadog".into(),
            reason: "5xx".into(),
            payload: vec![1, 2, 3],
        };

        // serialize_dlq_envelope matches `serialize_*` — non-tautological.
        let bytes = serialize_dlq_envelope(&envelope);
        let decoded = decode_dlq_envelope(&bytes);

        assert_eq!(decoded.sink, "datadog");
        assert_eq!(decoded.payload, vec![1, 2, 3]);
    }
}

// Stand-ins for production fns and types.
struct DlqEnvelope {
    sink: String,
    reason: String,
    payload: Vec<u8>,
}

fn serialize_dlq_envelope(_e: &DlqEnvelope) -> Vec<u8> {
    unimplemented!("fixture only")
}

fn decode_dlq_envelope(_b: &[u8]) -> DlqEnvelope {
    unimplemented!("fixture only")
}
