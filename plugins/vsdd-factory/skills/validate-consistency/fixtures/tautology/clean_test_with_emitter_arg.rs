// FIXTURE: this file is NOT compiled. It documents a CLEAN shape where
// the constructed struct is passed directly to a production fn as an
// argument — the most common and idiomatic non-tautological form.
//
// Expected detector verdict: CLEAN
// Reason: `emit_event(&event)` and `validate_event(&event)` both match
// the production-fn regex; the `assert_*` calls observe the result.

#[cfg(test)]
mod tests {
    use crate::events::Event;
    use crate::emitter::emit_event;
    use crate::validator::validate_event;

    #[test]
    fn test_BC_4_03_001_commit_made_event_round_trip() {
        let event = Event::CommitMade {
            sha: "abc123".into(),
            branch: "main".into(),
            message: "fix: typo".into(),
        };

        // Production fn invoked with the constructed value — non-tautological.
        let bytes = emit_event(&event);
        let valid = validate_event(&bytes);

        assert!(valid.is_ok(), "validator rejected canonical commit.made");
        assert!(bytes.starts_with(b"{"), "emitted as JSON");
    }
}
