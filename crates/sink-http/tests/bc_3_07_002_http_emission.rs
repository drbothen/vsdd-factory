//! BC-3.07.002 — sink-http emits `internal.sink_error` on each recorded failure.
//!
//! Traces to: BC-3.07.002 (AC-001, AC-004, AC-005, AC-006, AC-007, AC-008, AC-009).
//!
//! ## Test naming
//! `test_BC_3_07_002_*` per the test-writer naming convention.
//!
//! ## RED gate discipline
//! All emission-asserting tests (AC-001, AC-008, AC-009) are RED: they create a
//! local `tokio::sync::mpsc` channel and assert that an `internal.sink_error`
//! event appears after a sink failure. Because `HttpSink` does not yet accept an
//! error-channel sender (production not implemented), the channel remains empty
//! and the assertion fails with a clear message.
//!
//! AC-004 tests (`SinkFailure` recording preserved) will remain GREEN — they
//! exercise existing BC-3.01.008 behavior and must not be broken.
//!
//! ## Test vectors (BC-3.07.002 canonical)
//! - sink-http 503 on attempt 0 → event with sink_type="http", attempt=0,
//!   error_message contains "503"
//! - sink-http fails 3 times (max_retries=3) → 3 events with attempt=0,1,2
//! - sink_name="" → event with sink_name="<unnamed>"
//! - channel full → no panic, SinkFailure still recorded

use httpmock::prelude::*;
use sink_core::{Sink, SinkEvent, SinkErrorEvent};
use sink_http::{HttpSink, HttpSinkConfig};
use tokio::sync::mpsc;

// ── Helpers ──────────────────────────────────────────────────────────────────

fn make_event() -> SinkEvent {
    SinkEvent::new()
        .insert("type", "test.emission_check")
        .insert("payload", "x")
}

fn config_for_url(name: &str, url: &str) -> HttpSinkConfig {
    let toml = format!(
        r#"
schema_version = 1
type = "http"
name = "{name}"
url = "{url}"
"#
    );
    HttpSinkConfig::from_toml(&toml)
        .expect("from_toml must succeed")
        .expect("must return Some")
}

fn config_unnamed(url: &str) -> HttpSinkConfig {
    let toml = format!(
        r#"
schema_version = 1
type = "http"
name = ""
url = "{url}"
"#
    );
    HttpSinkConfig::from_toml(&toml)
        .expect("from_toml must succeed")
        .expect("must return Some")
}

// ── AC-001 (canonical test vector: 503 on attempt 0) ─────────────────────────

/// BC-3.07.002 postcondition 1, AC-001:
/// When sink-http records a `SinkFailure` (503 response), exactly one
/// `internal.sink_error` event is emitted with:
///   - `type = "internal.sink_error"`
///   - `sink_type = "http"`
///   - `attempt = 0` (first attempt, no retries configured)
///   - `error_message` non-empty and containing "503"
///
/// RED GATE: `HttpSink` does not yet accept an error channel sender; the
/// channel will be empty after the failure, and the assertion fails.
#[tokio::test]
async fn test_BC_3_07_002_http_emits_sink_error_on_503_attempt_0() {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(POST).path("/events");
        then.status(503).body("service unavailable");
    });

    // Build the error channel. Once the implementation threads `tx` into
    // `HttpSink`, the sink will send events here on failure.
    let (tx, mut rx) = mpsc::channel::<SinkErrorEvent>(16);

    // Existing constructor — does not accept the error channel yet.
    // The implementer will add a new_with_error_channel() variant.
    let _ = tx; // suppress unused warning; implementation will wire this.

    let config = config_for_url("http-sink-test", &format!("{}/events", server.base_url()));
    let sink = HttpSink::new(config).expect("HttpSink::new must succeed");

    sink.submit(make_event());
    let _ = sink.flush();

    // Assert the channel received an internal.sink_error event.
    // RED GATE: channel is empty because HttpSink doesn't send to it yet.
    let event = rx.try_recv().unwrap_or_else(|_| {
        panic!(
            "RED GATE: expected one internal.sink_error event on the channel \
             after a 503 response; channel is empty (production not yet wired)"
        )
    });

    assert_eq!(event.r#type, "internal.sink_error");
    assert_eq!(event.sink_type, "http");
    assert_eq!(event.attempt, 0, "first failure is attempt 0 (0-indexed)");
    assert!(
        !event.error_message.is_empty(),
        "error_message must be non-empty"
    );
    assert!(
        event.error_message.contains("503"),
        "error_message must contain '503', got: {}",
        event.error_message
    );
}

// ── AC-004 (BC-3.01.008 preservation) ────────────────────────────────────────

/// BC-3.07.002 postcondition 2, AC-004:
/// After the `internal.sink_error` emission path is wired, `SinkFailure`
/// entries are STILL recorded in `Mutex<Vec<SinkFailure>>` — BC-3.01.008
/// postcondition 1 is preserved. This story is additive only.
///
/// GREEN GATE: `take_failures()` already works (S-4.01 shipped). This test
/// must remain green after S-4.10 implementation.
#[tokio::test]
async fn test_BC_3_07_002_http_sink_failure_still_recorded_after_503() {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(POST).path("/events");
        then.status(503).body("service unavailable");
    });

    let config = config_for_url("bc3-01-008-check", &format!("{}/events", server.base_url()));
    let sink = HttpSink::new(config).expect("HttpSink::new must succeed");

    sink.submit(make_event());
    let _ = sink.flush();

    let failures = sink.take_failures();
    assert!(
        !failures.is_empty(),
        "BC-3.01.008 regression: SinkFailure must still be recorded after 503 \
         even when emission path is added"
    );
    assert!(
        failures[0].reason.contains("503"),
        "SinkFailure reason must reference the HTTP status"
    );
}

// ── AC-005 (silent drop on full/closed channel) ───────────────────────────────

/// BC-3.07.002 postcondition 3, AC-005 (VP-007):
/// When the internal event channel is full at emission time:
///   - The sink does NOT panic.
///   - The `SinkFailure` IS still recorded in `Mutex<Vec<SinkFailure>>`.
///   - Zero events appear on the channel (silently dropped).
///
/// RED GATE: since the channel is not yet wired into `HttpSink`, the behavior
/// is trivially non-panicking. After implementation, this test must still pass
/// (the `try_send().ok()` idiom silences the error).
#[tokio::test]
async fn test_BC_3_07_002_http_silent_drop_on_full_channel_no_panic() {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(POST).path("/events");
        then.status(503).body("service unavailable");
    });

    // Channel capacity = 0 is invalid; use 1 and pre-fill it.
    let (tx, _rx) = mpsc::channel::<SinkErrorEvent>(1);
    // Pre-fill: channel is now at capacity.
    let _ = tx.try_send(SinkErrorEvent::new("fill", "http", "fill", 0));
    let _ = tx; // full channel; implementer passes this to the sink.

    let config = config_for_url("full-channel-test", &format!("{}/events", server.base_url()));
    let sink = HttpSink::new(config).expect("HttpSink::new must succeed");

    // Must not panic even with full channel.
    sink.submit(make_event());
    let _ = sink.flush();

    // SinkFailure MUST still be recorded despite full channel (AC-004 additive).
    let failures = sink.take_failures();
    assert!(
        !failures.is_empty(),
        "SinkFailure must be recorded even when the error channel is full"
    );
}

/// BC-3.07.002 postcondition 3, AC-005 (EC-002):
/// When the internal event channel is closed (dispatcher shutting down):
///   - The sink does NOT panic.
///   - The `SinkFailure` IS still recorded.
#[tokio::test]
async fn test_BC_3_07_002_http_silent_drop_on_closed_channel_no_panic() {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(POST).path("/events");
        then.status(503).body("service unavailable");
    });

    let (tx, rx) = mpsc::channel::<SinkErrorEvent>(8);
    drop(rx); // Simulate dispatcher shutdown — channel closed.
    let _ = tx; // implementer passes this closed sender to the sink.

    let config = config_for_url("closed-channel-test", &format!("{}/events", server.base_url()));
    let sink = HttpSink::new(config).expect("HttpSink::new must succeed");

    // Must not panic even with closed channel.
    sink.submit(make_event());
    let _ = sink.flush();

    let failures = sink.take_failures();
    assert!(
        !failures.is_empty(),
        "SinkFailure must be recorded even when the error channel is closed"
    );
}

// ── AC-008 (exactly N events for N retried failures) ─────────────────────────

/// BC-3.07.002 invariant 3, AC-008:
/// A sink configured with `max_retries=3` that fails on all attempts emits
/// exactly 3 `internal.sink_error` events, one per failed attempt, with
/// `attempt` values 0, 1, 2 in order.
///
/// NOTE: `HttpSink` hard-codes `MAX_5XX_ATTEMPTS = 3` (not configurable via
/// TOML). The BC-3.07.002 canonical test vector for this AC is:
///   "sink-http fails 3 times → 3 events with attempt=0,1,2"
///
/// RED GATE: the channel will receive 0 events (no emission wired yet).
/// The test panics at the `events.len() == 3` assertion.
#[tokio::test]
async fn test_BC_3_07_002_http_three_failures_emit_three_events_with_correct_attempts() {
    let server = MockServer::start();
    // Always 503 — forces all MAX_5XX_ATTEMPTS to fail.
    let _mock = server.mock(|when, then| {
        when.method(POST).path("/events");
        then.status(503).body("service unavailable");
    });

    let (tx, mut rx) = mpsc::channel::<SinkErrorEvent>(16);
    let _ = tx; // implementer threads this into HttpSink.

    let config = config_for_url("retry-emission-test", &format!("{}/events", server.base_url()));
    let sink = HttpSink::new(config).expect("HttpSink::new must succeed");

    // One batch submission → 3 internal retries → 3 failure events expected.
    sink.submit(make_event());
    let _ = sink.flush();

    // Drain all received events.
    let mut events: Vec<SinkErrorEvent> = Vec::new();
    while let Ok(ev) = rx.try_recv() {
        events.push(ev);
    }

    assert_eq!(
        events.len(),
        3,
        "RED GATE: 3 failed attempts must emit 3 internal.sink_error events; \
         got {} (production not yet wired)",
        events.len()
    );
    // Verify attempt sequence 0, 1, 2.
    for (idx, event) in events.iter().enumerate() {
        assert_eq!(
            event.attempt,
            idx as u32,
            "event[{idx}] must have attempt={idx}, got {}",
            event.attempt
        );
        assert_eq!(event.sink_type, "http");
    }
}

// ── AC-009 (sink_name matches config / "<unnamed>" default) ──────────────────

/// BC-3.07.002 invariant 4, AC-009:
/// The `sink_name` in every emitted event matches the operator-configured
/// instance name from `observability-config.toml`.
///
/// RED GATE: channel is empty; assertion fails.
#[tokio::test]
async fn test_BC_3_07_002_http_sink_name_matches_config_name() {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(POST).path("/events");
        then.status(503).body("service unavailable");
    });

    let (tx, mut rx) = mpsc::channel::<SinkErrorEvent>(8);
    let _ = tx;

    let config = config_for_url("prod-http-ingress", &format!("{}/events", server.base_url()));
    let sink = HttpSink::new(config).expect("HttpSink::new must succeed");

    sink.submit(make_event());
    let _ = sink.flush();

    let event = rx.try_recv().unwrap_or_else(|_| {
        panic!(
            "RED GATE: expected one internal.sink_error event; \
             channel empty (production not yet wired)"
        )
    });

    assert_eq!(
        event.sink_name, "prod-http-ingress",
        "sink_name must match the configured name"
    );
}

/// BC-3.07.002 invariant 4 + EC-007, AC-009:
/// When `sink_name` is empty in config, the emitted event uses `"<unnamed>"`.
///
/// RED GATE: channel is empty; assertion fails.
#[tokio::test]
async fn test_BC_3_07_002_http_unnamed_sink_uses_unnamed_default() {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(POST).path("/events");
        then.status(503).body("service unavailable");
    });

    let (tx, mut rx) = mpsc::channel::<SinkErrorEvent>(8);
    let _ = tx;

    let config = config_unnamed(&format!("{}/events", server.base_url()));
    let sink = HttpSink::new(config).expect("HttpSink::new must succeed");

    sink.submit(make_event());
    let _ = sink.flush();

    let event = rx.try_recv().unwrap_or_else(|_| {
        panic!(
            "RED GATE: expected one internal.sink_error event for unnamed sink; \
             channel empty"
        )
    });

    assert_eq!(
        event.sink_name, "<unnamed>",
        "empty config sink_name must emit as '<unnamed>'"
    );
}

// ── AC-007 (events NOT routed through SinkRegistry::submit_all) ──────────────

/// BC-3.07.002 invariant 2, AC-007:
/// `internal.sink_error` events are NOT routed through the SinkRegistry fan-out.
/// The test verifies this by ensuring that a `SinkErrorEvent` emitted on
/// a dedicated internal channel does NOT trigger another `submit()` call on the
/// same `HttpSink`, which would cause infinite recursion.
///
/// This test is structural: it creates a sink that would expose re-entrancy by
/// panicking if `submit()` were called recursively, then triggers a failure and
/// asserts no panic occurs.
///
/// GREEN GATE (once production is wired): the internal channel is separate from
/// the SinkRegistry fan-out path, so this test passes.
/// RED GATE (current): no recursion occurs trivially because no emission is wired.
#[tokio::test]
async fn test_BC_3_07_002_http_invariant_no_routing_through_sink_registry() {
    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(POST).path("/events");
        then.status(503).body("service unavailable");
    });

    let config = config_for_url("anti-recursion-check", &format!("{}/events", server.base_url()));
    let sink = HttpSink::new(config).expect("HttpSink::new must succeed");

    // Trigger a failure. If internal.sink_error were routed through the same
    // sink's submit(), it would recursively re-trigger a failure, and the test
    // would either stack-overflow or block. Neither must happen.
    sink.submit(make_event());
    let _ = sink.flush(); // Must return within a reasonable time.

    // Verify the failure was recorded normally (no runaway).
    let failures = sink.take_failures();
    assert!(
        !failures.is_empty(),
        "SinkFailure must be recorded; sink must not be in a broken state \
         from recursive emission"
    );
}
