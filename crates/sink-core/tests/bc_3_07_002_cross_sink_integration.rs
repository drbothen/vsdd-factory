//! BC-3.07.002 — AC-010: cross-sink integration test for consistent schema.
//!
//! Verifies that all three sink drivers (sink-http, sink-otel-grpc, sink-file)
//! emit `internal.sink_error` events with a consistent schema when their
//! respective failure conditions are triggered.
//!
//! Captures events from the internal channel and asserts `type`, `sink_type`,
//! `sink_name`, `attempt`, and `error_message` are present and correctly typed.
//!
//! ## RED gate
//! All three sinks will fail to populate the channel because the production
//! emission wiring is not yet implemented. The `collect_events` helper returns
//! an empty Vec, and the cross-sink assertion fails.
//!
//! Traces to: BC-3.07.002 AC-010 (canonical test vectors: all three sink types).

// NOTE: This integration test imports from three driver crates. It lives in
// sink-core/tests/ because AC-010 is a cross-cutting concern that spans all
// three drivers — placing it here keeps the assertion centralised.

use httpmock::prelude::*;
use sink_core::{Sink, SinkErrorEvent, SinkEvent};
use sink_file::{DEFAULT_QUEUE_DEPTH as FILE_QUEUE_DEPTH, FileSink, FileSinkConfig};
use sink_http::{HttpSink, HttpSinkConfig};
use sink_otel_grpc::{BatchConfig, OtelGrpcConfig, OtelGrpcSink};
use std::collections::BTreeMap;
use std::time::Duration;
use tokio::sync::mpsc;

// ── Helpers ──────────────────────────────────────────────────────────────────

fn make_event(label: &str) -> SinkEvent {
    SinkEvent::new()
        .insert("type", "test.cross_sink_integration")
        .insert("label", label)
}

/// Drain all currently available events from the channel.
fn collect_events(rx: &mut mpsc::Receiver<SinkErrorEvent>) -> Vec<SinkErrorEvent> {
    let mut events = Vec::new();
    while let Ok(ev) = rx.try_recv() {
        events.push(ev);
    }
    events
}

// ── AC-010 cross-sink integration ────────────────────────────────────────────

/// BC-3.07.002 AC-010:
/// All three sink types emit `internal.sink_error` events with consistent schema
/// when their respective failure conditions are triggered.
///
/// Test sequence:
///   1. Trigger a failure in sink-http (503 response).
///   2. Trigger a failure in sink-otel-grpc (connection refused).
///   3. Trigger a failure in sink-file (read-only dir on Unix).
///   4. Assert each driver produced exactly one event with correct field values.
///
/// RED GATE: channels receive 0 events; all three assertions fail.
#[test]
#[cfg(unix)]
fn test_BC_3_07_002_cross_sink_consistent_schema_all_three_drivers() {
    // ── Shared internal error channel ────────────────────────────────────────
    // In production, all sinks share the dispatcher's internal event channel.
    // In this test, we use a single channel and a separate sender clone per sink.
    let (tx, mut rx) = mpsc::channel::<SinkErrorEvent>(64);

    // ── sink-http: trigger 503 ───────────────────────────────────────────────
    let server = MockServer::start();
    let _http_mock = server.mock(|when, then| {
        when.method(POST).path("/events");
        then.status(503).body("service unavailable");
    });

    let http_toml = format!(
        r#"
schema_version = 1
type = "http"
name = "cross-sink-http"
url = "{}/events"
"#,
        server.base_url()
    );
    let http_config = HttpSinkConfig::from_toml(&http_toml)
        .expect("from_toml")
        .expect("Some");
    let http_sink = HttpSink::new_with_error_channel(http_config, tx.clone())
        .expect("HttpSink::new_with_error_channel");

    http_sink.submit(make_event("http-trigger"));
    let _ = http_sink.flush();

    // ── sink-otel-grpc: trigger connection refused ───────────────────────────
    let otel_cfg = OtelGrpcConfig {
        name: "cross-sink-otel".to_string(),
        enabled: true,
        endpoint: "http://127.0.0.1:1".to_string(), // RFC 5736 reserved
        resource_attributes: BTreeMap::new(),
        batch: BatchConfig {
            size: 1,
            interval_ms: 50,
        },
        queue_depth: sink_otel_grpc::DEFAULT_QUEUE_DEPTH,
        routing_filter: None,
        tags: BTreeMap::new(),
    };
    let otel_sink = OtelGrpcSink::new_with_error_channel(otel_cfg, tx.clone())
        .expect("OtelGrpcSink::new_with_error_channel");
    otel_sink.submit(make_event("otel-trigger"));

    // Wait for otel connection failure.
    let otel_deadline = std::time::Instant::now() + Duration::from_secs(8);
    while std::time::Instant::now() < otel_deadline {
        if !otel_sink.take_failures().is_empty() {
            break;
        }
        std::thread::sleep(Duration::from_millis(50));
    }

    // ── sink-file: trigger read-only dir failure ─────────────────────────────
    use std::os::unix::fs::PermissionsExt;
    let tmp = tempfile::tempdir().expect("tempdir");
    let ro = tmp.path().join("ro");
    std::fs::create_dir_all(&ro).unwrap();
    let mut perms = std::fs::metadata(&ro).unwrap().permissions();
    perms.set_mode(0o555);
    std::fs::set_permissions(&ro, perms.clone()).unwrap();

    let file_cfg = FileSinkConfig {
        name: "cross-sink-file".to_string(),
        enabled: true,
        path_template: format!("{}/{{name}}-{{date}}.jsonl", ro.display()),
        queue_depth: FILE_QUEUE_DEPTH,
        routing_filter: None,
        tags: Default::default(),
    };
    let file_sink = FileSink::new_with_error_channel(file_cfg, None, tx.clone())
        .expect("FileSink::new_with_error_channel");
    file_sink.submit(make_event("file-trigger"));
    file_sink.flush().unwrap_or_default();

    // ── Collect and assert ───────────────────────────────────────────────────
    // Small sleep to allow any async emission to settle.
    std::thread::sleep(Duration::from_millis(200));

    let events = collect_events(&mut rx);

    // Expect at least 3 events — one per driver. HTTP may emit more than one
    // because MAX_5XX_ATTEMPTS=3 fires a separate event per retry attempt.
    assert!(
        events.len() >= 3,
        "expected at least 3 internal.sink_error events (one per driver); \
         got {}",
        events.len()
    );

    // Verify each event has all mandatory fields and correct sink_type.
    let mut found_http = false;
    let mut found_otel = false;
    let mut found_file = false;

    for event in &events {
        // All events must have the canonical type literal.
        assert_eq!(
            event.r#type, "internal.sink_error",
            "all events must have type='internal.sink_error'"
        );
        // error_message must be non-empty.
        assert!(
            !event.error_message.is_empty(),
            "error_message must be non-empty for sink_type='{}'",
            event.sink_type
        );

        match event.sink_type.as_str() {
            "http" => {
                assert_eq!(event.sink_name, "cross-sink-http");
                found_http = true;
            }
            "otel-grpc" => {
                assert_eq!(event.sink_name, "cross-sink-otel");
                found_otel = true;
            }
            "file" => {
                assert_eq!(event.sink_name, "cross-sink-file");
                found_file = true;
            }
            other => panic!("unexpected sink_type: {other}"),
        }
    }

    assert!(found_http, "expected one event with sink_type='http'");
    assert!(found_otel, "expected one event with sink_type='otel-grpc'");
    assert!(found_file, "expected one event with sink_type='file'");

    // Cleanup.
    perms.set_mode(0o755);
    std::fs::set_permissions(&ro, perms).unwrap();
    drop(tmp);
}

/// BC-3.07.002 AC-010 (schema field types):
/// Verifies that all mandatory fields have the correct Rust types:
///   - `type`: `&'static str`
///   - `sink_name`: `String`
///   - `sink_type`: `String`
///   - `error_message`: `String`
///   - `attempt`: `u32`
///
/// This is a pure-core shape test — no I/O, no drivers. It validates the
/// `SinkErrorEvent` struct itself has the correct field types for serialization.
///
/// GREEN GATE: `SinkErrorEvent` struct exists (created in this RED gate commit).
/// Will remain green throughout implementation.
#[test]
fn test_BC_3_07_002_cross_sink_schema_field_types_correct() {
    // Build events representing each driver's schema.
    let http_ev = SinkErrorEvent::new("http-sink", "http", "HTTP 503 after 3 attempts", 0u32);
    let otel_ev = SinkErrorEvent::new(
        "otel-sink",
        "otel-grpc",
        "connect: connection refused",
        0u32,
    );
    let file_ev = SinkErrorEvent::new("file-sink", "file", "permission denied", 0u32);

    for event in [&http_ev, &otel_ev, &file_ev] {
        // type is always "internal.sink_error".
        let _: &'static str = event.r#type;
        assert_eq!(event.r#type, "internal.sink_error");

        // sink_name is a String.
        let _: &String = &event.sink_name;
        assert!(!event.sink_name.is_empty());

        // sink_type is a String with a valid value.
        let _: &String = &event.sink_type;
        assert!(
            ["http", "otel-grpc", "file"].contains(&event.sink_type.as_str()),
            "sink_type must be one of http/otel-grpc/file, got '{}'",
            event.sink_type
        );

        // error_message is a String.
        let _: &String = &event.error_message;
        assert!(!event.error_message.is_empty());

        // attempt is u32.
        let _: u32 = event.attempt;
    }
}

/// BC-3.07.002 AC-006 — emission is synchronous with failure recording.
///
/// This test verifies the expected synchrony: after `flush()` returns, the
/// event should already be in the channel (no async delay). This is the
/// property that distinguishes `try_send` from a spawned task.
///
/// RED GATE: the channel is empty because emission is not yet wired.
#[test]
fn test_BC_3_07_002_ac006_emission_synchronous_with_failure_recording() {
    use httpmock::prelude::*;

    let server = MockServer::start();
    let _mock = server.mock(|when, then| {
        when.method(POST).path("/events");
        then.status(503).body("svc unavail");
    });

    let (tx, mut rx) = mpsc::channel::<SinkErrorEvent>(8);

    let config = {
        let toml = format!(
            r#"
schema_version = 1
type = "http"
name = "sync-emission-test"
url = "{}/events"
"#,
            server.base_url()
        );
        HttpSinkConfig::from_toml(&toml).unwrap().unwrap()
    };
    let sink =
        HttpSink::new_with_error_channel(config, tx).expect("HttpSink::new_with_error_channel");
    sink.submit(make_event("sync"));
    let _ = sink.flush(); // After flush(), SinkFailure is recorded AND event must be in channel.

    // No sleep between flush() and try_recv() — emission must be synchronous.
    let event = rx.try_recv().unwrap_or_else(|_| {
        panic!(
            "RED GATE: after flush() returns, the internal.sink_error event must \
             already be in the channel (synchronous emission via try_send); \
             channel is empty (production not yet wired)"
        )
    });
    assert_eq!(event.sink_type, "http");
}
