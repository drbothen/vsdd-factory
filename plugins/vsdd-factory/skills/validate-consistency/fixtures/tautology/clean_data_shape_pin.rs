// FIXTURE: this file is NOT compiled. It documents a tautology-shaped
// test that opts out via the `data-shape pin` doc-comment.
//
// Expected detector verdict: CLEAN (exception)
// Reason: the test IS structurally a tautology, but its doc-comment
// explicitly declares it a data-shape pin — the test exists to lock
// the struct's field set so additions/removals fail the build, not
// to exercise production behavior. The check honors this opt-out.

#[cfg(test)]
mod tests {
    use crate::entry::LogEntry;

    /// data-shape pin
    ///
    /// This test pins LogEntry's public field set. If a field is added
    /// or removed, this test breaks at compile time and forces a BC
    /// review. It is intentionally tautological.
    #[test]
    fn test_BC_3_02_001_log_entry_field_set_pin() {
        let entry = LogEntry {
            ts: 0,
            level: String::new(),
            message: String::new(),
            trace_id: None,
        };

        // Pure shape assertion — no production fn called. The opt-out
        // comment above is what keeps this off the advisory list.
        assert_eq!(entry.ts, 0);
        assert_eq!(entry.level, "");
        assert_eq!(entry.message, "");
        assert!(entry.trace_id.is_none());
    }
}
