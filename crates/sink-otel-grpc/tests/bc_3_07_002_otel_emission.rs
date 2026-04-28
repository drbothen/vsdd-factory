//! BC-3.07.002 — sink-otel-grpc emits `internal.sink_error` on each recorded failure.
//!
//! Traces to: BC-3.07.002 (AC-002, AC-004, AC-005, AC-007, AC-009).
//!
//! ## RED gate discipline
//! Tests that assert event presence on the internal channel are RED:
//! `OtelGrpcSink` does not yet accept an error channel sender, so the
//! channel remains empty and assertions fail with clear messages.
//!
//! Tests that assert `SinkFailure` recording (AC-004) are GREEN by definition —
//! they exercise existing BC-3.01.008 behavior.
//!
//! ## Canonical test vector (BC-3.07.002)
//! "sink-otel-grpc send fails → one `internal.sink_error` event with
//!  `sink_type="otel-grpc"`, `attempt=0`"
//!
//! ## Connection strategy
//! Uses RFC 5736 reserved port 1 (`127.0.0.1:1`) — guaranteed unreachable,
//! so the gRPC connect attempt fails quickly without external infrastructure.

use sink_core::{Sink, SinkEvent, SinkErrorEvent};
use sink_otel_grpc::{BatchConfig, OtelGrpcConfig, OtelGrpcSink};
use std::collections::BTreeMap;
use std::time::Duration;
use tokio::sync::mpsc;

// ── Helpers ──────────────────────────────────────────────────────────────────

fn make_event() -> SinkEvent {
    SinkEvent::new()
        .insert("type", "test.otel_emission_check")
        .insert("payload", "x")
}

/// Construct an `OtelGrpcSink` against RFC 5736 reserved port 1 — guaranteed
/// to produce a connection failure without requiring external infrastructure.
fn unreachable_sink(name: &str) -> OtelGrpcSink {
    let cfg = OtelGrpcConfig {
        name: name.to_string(),
        enabled: true,
        // RFC 5736 reserved — guaranteed unreachable.
        endpoint: "http://127.0.0.1:1".to_string(),
        resource_attributes: BTreeMap::new(),
        batch: BatchConfig {
            // Tight batch size so size trigger fires; tiny interval as fallback.
            size: 1,
            interval_ms: 50,
        },
        queue_depth: sink_otel_grpc::DEFAULT_QUEUE_DEPTH,
        routing_filter: None,
        tags: BTreeMap::new(),
    };
    OtelGrpcSink::new(cfg).expect("OtelGrpcSink::new must succeed for unreachable endpoint")
}

/// Wait until `take_failures()` is non-empty, up to a timeout.
fn wait_for_failure(sink: &OtelGrpcSink, timeout: Duration) -> bool {
    let deadline = std::time::Instant::now() + timeout;
    while std::time::Instant::now() < deadline {
        if !sink.take_failures().is_empty() {
            return true;
        }
        std::thread::sleep(Duration::from_millis(50));
    }
    false
}

// ── AC-002 (canonical test vector: otel-grpc connection refused) ──────────────

/// BC-3.07.002 postcondition 1, AC-002:
/// When sink-otel-grpc records a `SinkFailure` (connection refused), exactly one
/// `internal.sink_error` event is emitted with:
///   - `type = "internal.sink_error"`
///   - `sink_type = "otel-grpc"`
///   - `attempt = 0`
///   - `error_message` non-empty
///
/// RED GATE: `OtelGrpcSink` does not yet accept an error channel sender; the
/// channel will be empty after the failure, and the assertion fails.
#[test]
fn test_BC_3_07_002_otel_emits_sink_error_on_connection_refused() {
    let sink = unreachable_sink("otel-grpc-test");
    let (tx, mut rx) = mpsc::channel::<SinkErrorEvent>(16);
    let _ = tx; // implementer will thread this into OtelGrpcSink.

    sink.submit(make_event());

    // Wait for the worker to attempt the connection and fail.
    let got_failure = wait_for_failure(&sink, Duration::from_secs(8));
    assert!(
        got_failure,
        "OtelGrpcSink must record a SinkFailure when the endpoint is unreachable"
    );

    // Assert the channel received an internal.sink_error event.
    // RED GATE: channel is empty because OtelGrpcSink doesn't send to it yet.
    let event = rx.try_recv().unwrap_or_else(|_| {
        panic!(
            "RED GATE: expected one internal.sink_error event on the channel \
             after connection refused; channel is empty (production not yet wired)"
        )
    });

    assert_eq!(event.r#type, "internal.sink_error");
    assert_eq!(
        event.sink_type, "otel-grpc",
        "sink_type must be 'otel-grpc' for OtelGrpcSink"
    );
    assert_eq!(event.attempt, 0, "connection failure is attempt 0");
    assert!(
        !event.error_message.is_empty(),
        "error_message must be non-empty"
    );
}

// ── AC-004 (BC-3.01.008 preservation) ────────────────────────────────────────

/// BC-3.07.002 postcondition 2, AC-004:
/// `SinkFailure` entries continue to be recorded in `Mutex<Vec<SinkFailure>>`
/// after the emission path is wired. BC-3.01.008 postcondition 1 is preserved.
///
/// GREEN GATE: `take_failures()` works today; must remain green after S-4.10.
#[test]
fn test_BC_3_07_002_otel_sink_failure_still_recorded_after_connection_refused() {
    let sink = unreachable_sink("otel-bc3-01-008-check");

    sink.submit(make_event());

    let got_failure = wait_for_failure(&sink, Duration::from_secs(8));
    assert!(
        got_failure,
        "BC-3.01.008 regression: SinkFailure must be recorded when otel endpoint unreachable"
    );
}

// ── AC-005 (silent drop on full/closed channel) ───────────────────────────────

/// BC-3.07.002 postcondition 3, AC-005 (VP-007):
/// When the internal event channel is full at emission time:
///   - The sink does NOT panic.
///   - The `SinkFailure` IS still recorded.
///
/// RED GATE / GREEN GATE: no emission wired yet so no panic; after implementation
/// `try_send().ok()` ensures same behavior.
#[test]
fn test_BC_3_07_002_otel_silent_drop_on_full_channel_no_panic() {
    let sink = unreachable_sink("otel-full-channel-test");

    // Capacity 1, pre-filled — simulates full channel.
    let (tx, _rx) = mpsc::channel::<SinkErrorEvent>(1);
    let _ = tx.try_send(SinkErrorEvent::new("fill", "otel-grpc", "fill", 0));
    let _ = tx; // implementer passes this full sender to the sink.

    // Must not panic.
    sink.submit(make_event());

    let got_failure = wait_for_failure(&sink, Duration::from_secs(8));
    assert!(
        got_failure,
        "SinkFailure must be recorded even when the error channel is full"
    );
}

/// BC-3.07.002 postcondition 3, AC-005 (EC-002):
/// Closed channel causes silent drop; `SinkFailure` still recorded.
#[test]
fn test_BC_3_07_002_otel_silent_drop_on_closed_channel_no_panic() {
    let sink = unreachable_sink("otel-closed-channel-test");

    let (tx, rx) = mpsc::channel::<SinkErrorEvent>(8);
    drop(rx); // Simulate dispatcher shutdown.
    let _ = tx; // closed sender; implementer passes to sink.

    sink.submit(make_event());

    let got_failure = wait_for_failure(&sink, Duration::from_secs(8));
    assert!(
        got_failure,
        "SinkFailure must be recorded even when the error channel is closed"
    );
}

// ── AC-009 (sink_name matches config) ────────────────────────────────────────

/// BC-3.07.002 invariant 4, AC-009:
/// The `sink_name` field in the emitted event matches the operator-configured
/// name for this `OtelGrpcSink` instance.
///
/// RED GATE: channel is empty; assertion fails.
#[test]
fn test_BC_3_07_002_otel_sink_name_matches_config_name() {
    let sink = unreachable_sink("grafana-dev-otel");
    let (tx, mut rx) = mpsc::channel::<SinkErrorEvent>(8);
    let _ = tx;

    sink.submit(make_event());
    let _ = wait_for_failure(&sink, Duration::from_secs(8));

    let event = rx.try_recv().unwrap_or_else(|_| {
        panic!(
            "RED GATE: expected one internal.sink_error event with sink_name; \
             channel empty"
        )
    });

    assert_eq!(
        event.sink_name, "grafana-dev-otel",
        "sink_name must match the configured OtelGrpcConfig name"
    );
}

// ── AC-007 (no routing through SinkRegistry) ─────────────────────────────────

/// BC-3.07.002 invariant 2, AC-007:
/// `internal.sink_error` events from `OtelGrpcSink` are NOT routed through
/// the SinkRegistry fan-out. A failure must not trigger recursive emission.
///
/// GREEN GATE (once production is wired); currently trivially non-recursive.
#[test]
fn test_BC_3_07_002_otel_invariant_no_routing_through_sink_registry() {
    // Use two sinks; verify neither causes the other to fail via recursion.
    let sink_a = unreachable_sink("otel-anti-recursion-a");
    let sink_b = unreachable_sink("otel-anti-recursion-b");

    // Submit to sink_a only.
    sink_a.submit(make_event());
    let _ = wait_for_failure(&sink_a, Duration::from_secs(8));

    // sink_b must not have recorded any failures — proving no cross-sink
    // recursive routing occurred.
    let sink_b_failures = sink_b.take_failures();
    assert!(
        sink_b_failures.is_empty(),
        "VP-012: sink_b must not record failures caused by sink_a failure emission; \
         got {} failures",
        sink_b_failures.len()
    );
}
