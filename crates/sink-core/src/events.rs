//! `internal.sink_error` event schema and fire-and-forget emission helper (S-4.10).
//!
//! This module is **pure-core**: it defines `SinkErrorEvent` (the structured event
//! emitted by every sink driver when a `SinkFailure` is recorded) and the
//! `emit_sink_error` helper that wraps the `try_send` call.
//!
//! ## Pure/effectful boundary
//!
//! `SinkErrorEvent` construction is pure — no I/O. The single effectful
//! operation (`Sender::try_send`) is isolated to `emit_sink_error`. Driver
//! crates call `emit_sink_error` at their failure-recording site; they never
//! construct the `try_send` themselves.
//!
//! ## Fire-and-forget contract (BC-3.07.002 postcondition 3)
//!
//! `emit_sink_error` silently drops send errors (`try_send(...).ok()`).
//! A full or closed channel is not an error from the sink's perspective.

use tokio::sync::mpsc;

/// Structured `internal.sink_error` event emitted by every sink driver that
/// records a [`crate::SinkFailure`] (BC-3.07.002 postcondition 1).
///
/// The `type` field is always `"internal.sink_error"`. Receivers can
/// distinguish which driver emitted the event via `sink_type`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SinkErrorEvent {
    /// Event type discriminant. Always `"internal.sink_error"`.
    pub r#type: &'static str,
    /// Operator-assigned sink instance name from `observability-config.toml`.
    /// Defaults to `"<unnamed>"` when the operator did not supply a name
    /// (BC-3.07.002 invariant 4, EC-007).
    pub sink_name: String,
    /// Driver kind. One of `"http"`, `"otel-grpc"`, `"file"`.
    /// (BC-3.07.002 postcondition 1 `sink_type` field).
    pub sink_type: String,
    /// Human-readable description of the failure. Always non-empty.
    /// OS error messages are sanitized to valid UTF-8 via lossy conversion
    /// (BC-3.07.002 EC-006).
    pub error_message: String,
    /// Zero-based retry attempt number at which the failure occurred
    /// (BC-3.07.002 postcondition 1 `attempt` field).
    pub attempt: u32,
}

impl SinkErrorEvent {
    /// Construct a new `SinkErrorEvent` with the mandatory fields set.
    ///
    /// `sink_name_raw`: if empty, defaults to `"<unnamed>"` per EC-007.
    pub fn new(
        sink_name_raw: impl Into<String>,
        sink_type: impl Into<String>,
        error_message: impl Into<String>,
        attempt: u32,
    ) -> Self {
        let sink_name = {
            let raw = sink_name_raw.into();
            if raw.is_empty() {
                "<unnamed>".to_owned()
            } else {
                raw
            }
        };
        Self {
            r#type: "internal.sink_error",
            sink_name,
            sink_type: sink_type.into(),
            error_message: error_message.into(),
            attempt,
        }
    }
}

/// Fire-and-forget emission of a `SinkErrorEvent` to the internal event
/// channel (BC-3.07.002 postcondition 4 + invariant 1).
///
/// Calls `Sender::try_send` — non-blocking. If the channel is full or
/// closed the error is silently dropped (`.ok()`); the sink never panics
/// and never fails to record the `SinkFailure` because of an emission
/// error (BC-3.07.002 postcondition 3).
///
/// The caller MUST release any mutex guard over the `SinkFailure` buffer
/// **before** calling this function to avoid holding the lock across the
/// channel operation (S-4.10 previous-story intelligence note).
pub fn emit_sink_error(tx: &mpsc::Sender<SinkErrorEvent>, event: SinkErrorEvent) {
    // Fire-and-forget: try_send is non-blocking. If the channel is full or
    // closed, the error is silently discarded (.ok()). The sink never panics
    // and never fails to record SinkFailure because of an emission error
    // (BC-3.07.002 postcondition 3, VP-007).
    let _ = tx.try_send(event);
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::mpsc;

    // ── BC-3.07.002 struct shape tests ──────────────────────────────────────

    /// BC-3.07.002 postcondition 1: `type` field is always `"internal.sink_error"`.
    ///
    /// Calls `SinkErrorEvent::new(...)` and asserts the type literal.
    /// This is a pure-core test — no I/O, no effectful helpers.
    #[test]
    fn test_BC_3_07_002_type_field_is_always_internal_sink_error() {
        let ev = SinkErrorEvent::new("my-http-sink", "http", "503 error", 0);
        assert_eq!(
            ev.r#type, "internal.sink_error",
            "type must always be 'internal.sink_error'"
        );
    }

    /// BC-3.07.002 postcondition 1: `sink_type` field carries the driver kind.
    #[test]
    fn test_BC_3_07_002_sink_type_carries_driver_kind() {
        for (driver, expected) in [
            ("http", "http"),
            ("otel-grpc", "otel-grpc"),
            ("file", "file"),
        ] {
            let ev = SinkErrorEvent::new("s", driver, "err", 0);
            assert_eq!(
                ev.sink_type, expected,
                "sink_type must match driver kind for '{driver}'"
            );
        }
    }

    /// BC-3.07.002 invariant 4 + EC-007: empty `sink_name` defaults to `"<unnamed>"`.
    #[test]
    fn test_BC_3_07_002_invariant_sink_name_defaults_to_unnamed_when_empty() {
        let ev = SinkErrorEvent::new("", "http", "err", 0);
        assert_eq!(
            ev.sink_name, "<unnamed>",
            "empty sink name must default to '<unnamed>'"
        );
    }

    /// BC-3.07.002 invariant 4: non-empty `sink_name` is preserved as-is.
    #[test]
    fn test_BC_3_07_002_invariant_sink_name_preserved_when_non_empty() {
        let ev = SinkErrorEvent::new("production-http", "http", "err", 0);
        assert_eq!(ev.sink_name, "production-http");
    }

    /// BC-3.07.002 postcondition 1: `attempt` field is 0-indexed.
    ///
    /// Tests boundary values: attempt=0 (first attempt) and attempt=2
    /// (third attempt in a max_retries=3 sequence).
    #[test]
    fn test_BC_3_07_002_attempt_field_is_zero_indexed() {
        let first = SinkErrorEvent::new("s", "http", "err", 0);
        assert_eq!(first.attempt, 0, "first failure is attempt 0");

        let third = SinkErrorEvent::new("s", "http", "err", 2);
        assert_eq!(third.attempt, 2, "third failure is attempt 2");
    }

    /// BC-3.07.002 postcondition 1: `error_message` is non-empty.
    ///
    /// Production code must always pass a non-empty reason from the driver
    /// failure path. This test verifies the struct preserves the message.
    #[test]
    fn test_BC_3_07_002_error_message_preserved_from_constructor() {
        let ev = SinkErrorEvent::new("s", "http", "HTTP 503 after 3 attempts", 0);
        assert!(
            !ev.error_message.is_empty(),
            "error_message must be non-empty"
        );
        assert!(
            ev.error_message.contains("503"),
            "error_message must contain the status code"
        );
    }

    // ── BC-3.07.002 emit_sink_error helper tests ─────────────────────────────

    /// BC-3.07.002 postcondition 1 + postcondition 4:
    /// `emit_sink_error` sends exactly one event to the channel when called
    /// with a valid sender and correctly-constructed event.
    ///
    /// RED GATE: fails until the stub body is replaced with the real try_send.
    #[tokio::test]
    async fn test_BC_3_07_002_emit_sink_error_sends_event_to_channel() {
        let (tx, mut rx) = mpsc::channel::<SinkErrorEvent>(8);
        let event = SinkErrorEvent::new("test-sink", "http", "503 Service Unavailable", 0);

        emit_sink_error(&tx, event.clone());

        let received = rx
            .try_recv()
            .expect("RED GATE: emit_sink_error must send to the channel; currently a no-op stub");
        assert_eq!(received.r#type, "internal.sink_error");
        assert_eq!(received.sink_name, "test-sink");
        assert_eq!(received.sink_type, "http");
        assert_eq!(received.attempt, 0);
    }

    /// BC-3.07.002 postcondition 3 (VP-007): full channel causes silent drop,
    /// no panic.
    ///
    /// RED GATE: once emit_sink_error is real, this must pass (no panic).
    /// While stub is a no-op, it also doesn't panic — test passes vacuously.
    /// Post-implementation this remains green (try_send().ok() discards error).
    #[tokio::test]
    async fn test_BC_3_07_002_emit_sink_error_silent_drop_on_full_channel() {
        // Capacity-0 channels require a bound of at least 1, so use 1 and fill it.
        let (tx, _rx) = mpsc::channel::<SinkErrorEvent>(1);
        // Fill the channel to capacity.
        let _ = tx.try_send(SinkErrorEvent::new("fill", "http", "fill", 0));

        // This must not panic regardless of channel state.
        emit_sink_error(&tx, SinkErrorEvent::new("overflow", "http", "err", 0));
        // No assertion needed — the test passes if no panic occurs.
    }

    /// BC-3.07.002 postcondition 3: closed channel causes silent drop, no panic.
    #[tokio::test]
    async fn test_BC_3_07_002_emit_sink_error_silent_drop_on_closed_channel() {
        let (tx, rx) = mpsc::channel::<SinkErrorEvent>(8);
        drop(rx); // Close the receiving end.

        // Must not panic.
        emit_sink_error(&tx, SinkErrorEvent::new("closed", "http", "err", 0));
    }

    /// BC-3.07.002 invariant 3: one event per failure, not one per batch.
    ///
    /// RED GATE: verifies exactly one event is emitted per call to emit_sink_error.
    #[tokio::test]
    async fn test_BC_3_07_002_invariant_one_event_emitted_per_failure_call() {
        let (tx, mut rx) = mpsc::channel::<SinkErrorEvent>(16);

        // Simulate 3 consecutive failures (attempt 0, 1, 2).
        for attempt in 0u32..3 {
            let event = SinkErrorEvent::new("sink", "http", format!("error {attempt}"), attempt);
            emit_sink_error(&tx, event);
        }

        let mut events = Vec::new();
        while let Ok(ev) = rx.try_recv() {
            events.push(ev);
        }

        assert_eq!(
            events.len(),
            3,
            "RED GATE: 3 failures must produce exactly 3 events; got {}",
            events.len()
        );
        assert_eq!(events[0].attempt, 0);
        assert_eq!(events[1].attempt, 1);
        assert_eq!(events[2].attempt, 2);
    }
}
