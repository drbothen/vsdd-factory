//! AC-12: Cross-sink `internal.sink_error` emission test.
//!
//! Traces to:
//! - BC-3.07.002 postcondition 1: mandatory event fields present
//! - BC-3.07.002 postcondition 3: emission failure silently dropped, no panic
//!
//! SUT entry point (exception from Router::submit() — see story SUT boundary):
//! inject `SinkFailure` via each driver's test harness; observe internal event
//! channel. Rationale: AC-12 verifies the contracted internal-channel emission
//! path on failure, invoked from the sink driver's failure-recording site.
//!
//! Per BC-3.07.002 + S-4.07 BCs to Update:
//! - sink-datadog emits `sink_type='datadog'` (NOT 'http')
//! - sink-honeycomb emits `sink_type='honeycomb'` (NOT 'http')
//! - `sink_type` ∈ {'file', 'otel-grpc', 'http', 'datadog', 'honeycomb'}
//!
//! RED gate: will fail until each sink driver correctly wires the error channel
//! and emits `internal.sink_error` with the correct sink_type.

use std::time::Duration;

use httpmock::prelude::*;
use sink_core::{Sink, SinkErrorEvent, SinkEvent};
use sink_http::{HttpSink, HttpSinkConfig};
use tokio::sync::mpsc;

// ── Helper ────────────────────────────────────────────────────────────────────

fn drain_events(rx: &mut mpsc::Receiver<SinkErrorEvent>, timeout: Duration) -> Vec<SinkErrorEvent> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    rt.block_on(async {
        let mut events = Vec::new();
        let deadline = tokio::time::Instant::now() + timeout;
        loop {
            tokio::select! {
                ev = rx.recv() => {
                    match ev {
                        Some(e) => events.push(e),
                        None => break,
                    }
                }
                _ = tokio::time::sleep_until(deadline) => break,
            }
        }
        events
    })
}

// ── AC-12: sink-http emits internal.sink_error with sink_type='http' ─────────

/// BC-3.07.002 PC1 — AC-12 (sink-http):
///
/// HttpSink injected with a 5xx mock server emits `internal.sink_error` with
/// `sink_type='http'`. Oracle: at least 1 event on the channel with
/// `type='internal.sink_error'`, `sink_type='http'`, non-empty `error_message`.
///
/// RED gate: will fail until HttpSink's new_with_error_channel path
/// correctly emits SinkErrorEvent on failure.
#[test]
fn test_BC_3_07_002_http_sink_emits_internal_sink_error_with_type_http() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.any_request();
        then.status(500).body("Internal Server Error");
    });

    let (error_tx, mut error_rx) = mpsc::channel::<SinkErrorEvent>(64);

    let cfg = HttpSinkConfig::builder()
        .name("http-error-test")
        .url(server.url("/events"))
        .queue_depth(64)
        .build();

    let sink =
        HttpSink::new_with_error_channel(cfg, error_tx).expect("build HttpSink with error channel");

    sink.submit(SinkEvent::new().insert("type", "plugin.invoked"));
    sink.flush().expect("flush");
    sink.shutdown();

    let events = drain_events(&mut error_rx, Duration::from_secs(5));

    assert!(
        !events.is_empty(),
        "BC-3.07.002 PC1: HttpSink must emit at least 1 internal.sink_error event; got 0"
    );

    for ev in &events {
        assert_eq!(
            ev.r#type, "internal.sink_error",
            "BC-3.07.002: type must be 'internal.sink_error'"
        );
        assert_eq!(
            ev.sink_type, "http",
            "BC-3.07.002: HttpSink must emit sink_type='http' (NOT 'datadog' or 'honeycomb')"
        );
        assert_eq!(
            ev.sink_name, "http-error-test",
            "BC-3.07.002: sink_name must match"
        );
        assert!(
            !ev.error_message.is_empty(),
            "BC-3.07.002: error_message must be non-empty"
        );
    }
}

// ── AC-12: sink-datadog emits internal.sink_error with sink_type='datadog' ───

/// BC-3.07.002 PC1 + BCs to Update — AC-12 (sink-datadog):
///
/// DatadogSink wraps HttpSink but must emit `sink_type='datadog'` NOT 'http'.
/// This distinguishes the wrapping driver in `internal.sink_error` events.
///
/// Oracle: at least 1 `internal.sink_error` event with `sink_type='datadog'`.
///
/// RED gate: will fail until DatadogSink passes sink_type='datadog' to its
/// internal error emission path (NOT 'http').
#[test]
fn test_BC_3_07_002_datadog_sink_emits_internal_sink_error_with_type_datadog() {
    use sink_core::SinkConfigCommon;
    use sink_datadog::{DatadogSink, DatadogSinkConfig};

    let server = MockServer::start();
    server.mock(|when, then| {
        when.any_request();
        then.status(500).body("Internal Server Error");
    });

    let (error_tx, mut error_rx) = mpsc::channel::<SinkErrorEvent>(64);

    let cfg = DatadogSinkConfig {
        common: SinkConfigCommon {
            name: "dd-error-test".to_string(),
            enabled: true,
            routing_filter: None,
            tags: Default::default(),
        },
        api_key: "test-dd-api-key".to_string(),
        endpoint: Some(server.url("/v2/logs")),
    };

    // DatadogSink must have a constructor that accepts an error channel.
    // RED gate: `DatadogSink::new_with_error_channel` does not exist yet.
    let sink = DatadogSink::new_with_error_channel(cfg, error_tx)
        .expect("build DatadogSink with error channel");

    sink.submit(SinkEvent::new().insert("type", "audit.access"));
    sink.flush().expect("flush");
    sink.shutdown();

    let events = drain_events(&mut error_rx, Duration::from_secs(5));

    assert!(
        !events.is_empty(),
        "BC-3.07.002 BCs-to-Update: DatadogSink must emit internal.sink_error events; got 0"
    );

    for ev in &events {
        assert_eq!(
            ev.sink_type, "datadog",
            "BC-3.07.002 BCs-to-Update: DatadogSink must emit sink_type='datadog' NOT 'http'; \
             got '{}'",
            ev.sink_type
        );
        assert_eq!(ev.sink_name, "dd-error-test");
    }
}

// ── AC-12: sink-honeycomb emits internal.sink_error with sink_type='honeycomb' ─

/// BC-3.07.002 PC1 + BCs to Update — AC-12 (sink-honeycomb):
///
/// HoneycombSink wraps HttpSink but must emit `sink_type='honeycomb'` NOT 'http'.
///
/// Oracle: at least 1 `internal.sink_error` event with `sink_type='honeycomb'`.
///
/// RED gate: will fail until HoneycombSink passes sink_type='honeycomb' to
/// its internal error emission path.
#[test]
fn test_BC_3_07_002_honeycomb_sink_emits_internal_sink_error_with_type_honeycomb() {
    use sink_core::SinkConfigCommon;
    use sink_honeycomb::{HoneycombSink, HoneycombSinkConfig};

    let server = MockServer::start();
    server.mock(|when, then| {
        when.any_request();
        then.status(500).body("Internal Server Error");
    });

    let (error_tx, mut error_rx) = mpsc::channel::<SinkErrorEvent>(64);

    let cfg = HoneycombSinkConfig {
        sink_type: "honeycomb".to_string(),
        common: SinkConfigCommon {
            name: "hc-error-test".to_string(),
            enabled: true,
            routing_filter: None,
            tags: Default::default(),
        },
        api_key: Some("test-hc-api-key".to_string()),
        dataset: Some("test-dataset".to_string()),
        base_url: Some(server.url("/1/events")),
        queue_depth: 64,
    };

    // HoneycombSink must have a constructor that accepts an error channel.
    // RED gate: `HoneycombSink::new_with_error_channel` does not exist yet.
    let sink = HoneycombSink::new_with_error_channel(cfg, error_tx)
        .expect("build HoneycombSink with error channel");

    sink.submit(SinkEvent::new().insert("type", "plugin.invoked"));
    sink.flush().expect("flush");
    sink.shutdown();

    let events = drain_events(&mut error_rx, Duration::from_secs(5));

    assert!(
        !events.is_empty(),
        "BC-3.07.002 BCs-to-Update: HoneycombSink must emit internal.sink_error events; got 0"
    );

    for ev in &events {
        assert_eq!(
            ev.sink_type, "honeycomb",
            "BC-3.07.002 BCs-to-Update: HoneycombSink must emit sink_type='honeycomb' NOT 'http'; \
             got '{}'",
            ev.sink_type
        );
        assert_eq!(ev.sink_name, "hc-error-test");
    }
}

// ── AC-12: BC-3.07.002 PC3 — emission failure is silent (no panic) ───────────

/// BC-3.07.002 PC3 — emission silently dropped when channel full, no panic.
///
/// Fills the error channel to capacity before the sink starts emitting,
/// then verifies no panic occurs when the channel is full.
///
/// RED gate: will fail if HttpSink panics on a full error channel.
#[test]
fn test_BC_3_07_002_sink_error_emission_silent_on_full_channel() {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.any_request();
        then.status(500).body("Internal Server Error");
    });

    // Capacity-1 channel, immediately filled.
    let (error_tx, _rx) = mpsc::channel::<SinkErrorEvent>(1);
    // Pre-fill the channel to capacity.
    let fill_ev = SinkErrorEvent::new("fill", "http", "fill", 0);
    let _ = error_tx.try_send(fill_ev);

    let cfg = HttpSinkConfig::builder()
        .name("silent-overflow")
        .url(server.url("/events"))
        .queue_depth(64)
        .build();

    let sink = HttpSink::new_with_error_channel(cfg, error_tx).expect("build sink");

    // Must not panic when error channel is full.
    sink.submit(SinkEvent::new().insert("type", "commit.made"));
    sink.flush()
        .expect("flush — must not panic even with full error channel");
    sink.shutdown();
    // If we reach here, no panic occurred — test passes.
}
