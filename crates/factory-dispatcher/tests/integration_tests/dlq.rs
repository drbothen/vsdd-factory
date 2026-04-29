//! AC-4: Sink failure → DLQ — mock 5xx server; events appear in DLQ file.
//!
//! Traces to:
//! - BC-3.07.003: DLQ write event emission on retry exhaustion
//! - BC-3.07.004: when DLQ write itself fails, `internal.sink_dlq_failure`
//!   emitted to internal channel, no panic
//!
//! SUT entry point: `Router::submit()` → `SinkRegistry::submit_all()` →
//! `HttpSink::send()` (mock returns 500 for every request). Oracle: after
//! RetryPolicy exhaustion, DLQ file exists and contains submitted events
//! in valid JSONL; `internal.sink_dlq_write` event emitted.
//!
//! RED gate: will fail until HttpSink's DLQ write path is exercised through
//! the full Router→SinkRegistry→HttpSink→DlqWriter pipeline.

use std::sync::Arc;
use std::time::Duration;

use httpmock::prelude::*;
use sink_core::{DlqReason, DlqWriter, DlqWriterConfig, Sink, SinkDlqEvent, SinkEvent};
use sink_http::{HttpSink, HttpSinkConfig};

/// Helper: build an HttpSink pointing at `url` with a DlqWriter backed by
/// the given channel sender. Returns (sink, dlq_writer).
fn make_http_sink_with_dlq(
    url: &str,
    dlq_root: std::path::PathBuf,
    dlq_tx: tokio::sync::mpsc::Sender<SinkDlqEvent>,
) -> (HttpSink, Arc<DlqWriter>) {
    let dlq_cfg = DlqWriterConfig {
        template: "dead-letter-{name}-{date}.jsonl".to_owned(),
        size_cap_bytes: 100 * 1024 * 1024,
        project: None,
        dlq_root,
    };
    let dlq_writer = Arc::new(DlqWriter::new(dlq_cfg, dlq_tx));

    let http_cfg = HttpSinkConfig::builder()
        .name("http-5xx-test")
        .url(url)
        .queue_depth(64)
        .build();

    let sink = HttpSink::new_with_observability(http_cfg, None, Some(Arc::clone(&dlq_writer)))
        .expect("build HttpSink with DLQ");

    (sink, dlq_writer)
}

/// BC-3.07.003 — AC-4:
///
/// After mock 5xx forces retry exhaustion, events appear in the DLQ file.
/// A `internal.sink_dlq_write` event is emitted to the internal channel.
///
/// RED gate: will fail until HttpSink's DLQ write path correctly writes
/// events to the DLQ file after retry exhaustion.
#[test]
fn test_BC_3_07_003_dlq_written_after_5xx_retry_exhaustion() {
    let tmp = tempfile::tempdir().unwrap();
    let dlq_root = tmp.path().join("dlq");

    // Mock HTTP server that always returns 500.
    let server = MockServer::start();
    server.mock(|when, then| {
        when.any_request();
        then.status(500).body("Internal Server Error");
    });

    // Build a tokio runtime for the DLQ channel (the DLQ writer uses tokio::sync::mpsc).
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let (dlq_tx, mut dlq_rx) = tokio::sync::mpsc::channel::<SinkDlqEvent>(64);
        let url = server.url("/events");
        let (sink, dlq_writer) = make_http_sink_with_dlq(&url, dlq_root.clone(), dlq_tx);

        // Submit 2 events.
        let ev1 = SinkEvent::new()
            .insert("type", "plugin.invoked")
            .insert("dispatcher_trace_id", "trace-dlq-001");
        let ev2 = SinkEvent::new()
            .insert("type", "commit.made")
            .insert("dispatcher_trace_id", "trace-dlq-002");
        sink.submit(ev1);
        sink.submit(ev2);

        // Flush — triggers send attempts → 3 × 500 → retry exhaustion → DLQ write.
        sink.flush().expect("flush");
        sink.shutdown();

        // Oracle 1: DLQ file exists and contains the 2 events.
        let date = chrono::Utc::now().format("%Y-%m-%d");
        let expected_dlq_path = dlq_root.join(format!("dead-letter-http-5xx-test-{date}.jsonl"));
        assert!(
            expected_dlq_path.exists(),
            "AC-4 BC-3.07.003: DLQ file must exist after retry exhaustion; expected: {expected_dlq_path:?}"
        );

        let dlq_content = std::fs::read_to_string(&expected_dlq_path).unwrap();
        let dlq_lines: Vec<&str> = dlq_content
            .lines()
            .filter(|l| !l.trim().is_empty())
            .collect();
        assert_eq!(
            dlq_lines.len(),
            2,
            "AC-4 BC-3.07.003: DLQ file must contain 2 events; got {}",
            dlq_lines.len()
        );

        // Each line must be valid JSONL.
        for line in &dlq_lines {
            let parsed: serde_json::Value =
                serde_json::from_str(line).expect("DLQ line must be valid JSON");
            assert!(
                parsed.get("type").is_some(),
                "BC-3.07.003: each DLQ line must have a 'type' field; got: {parsed}"
            );
        }

        // Oracle 2: internal.sink_dlq_write events emitted (BC-3.07.003).
        // Drain the channel briefly.
        let mut dlq_events: Vec<SinkDlqEvent> = Vec::new();
        // We sent 2 events; each retry exhaustion writes one DLQ record + emits one event.
        // Give a short timeout for channel to drain.
        let deadline = std::time::Instant::now() + Duration::from_millis(500);
        while std::time::Instant::now() < deadline {
            if let Ok(ev) = dlq_rx.try_recv() {
                dlq_events.push(ev);
            }
        }

        let write_events: Vec<_> = dlq_events
            .iter()
            .filter(|e| matches!(e, SinkDlqEvent::Write(_)))
            .collect();

        assert_eq!(
            write_events.len(),
            2,
            "AC-4 BC-3.07.003: 2 internal.sink_dlq_write events must be emitted; got {}",
            write_events.len()
        );

        // Verify field schema on the first write event.
        if let SinkDlqEvent::Write(w) = &write_events[0] {
            assert_eq!(
                w.sink_name, "http-5xx-test",
                "BC-3.07.003: sink_name must match"
            );
            assert_eq!(
                w.reason,
                DlqReason::RetryExhausted,
                "BC-3.07.003: reason must be retry_exhausted"
            );
        }
    });
}

/// BC-3.07.004 — DLQ write failure emits `internal.sink_dlq_failure`, no panic.
///
/// This test verifies that when the DLQ write itself fails (e.g. unwritable
/// directory), a `internal.sink_dlq_failure` event is emitted and the sink
/// does not panic.
///
/// RED gate: will fail until DlqWriter failure path emits the correct event.
#[test]
fn test_BC_3_07_004_dlq_write_failure_emits_failure_event_no_panic() {
    // Use a path that will fail: a file (not a dir) as the dlq_root.
    let tmp = tempfile::tempdir().unwrap();
    let not_a_dir = tmp.path().join("this-is-a-file.txt");
    std::fs::write(&not_a_dir, b"not a directory").unwrap();

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let (dlq_tx, mut dlq_rx) = tokio::sync::mpsc::channel::<SinkDlqEvent>(64);
        let dlq_cfg = DlqWriterConfig {
            template: "dead-letter-{name}-{date}.jsonl".to_owned(),
            size_cap_bytes: 100 * 1024 * 1024,
            project: None,
            dlq_root: not_a_dir.clone(), // a file, not a dir — mkdir_p will fail
        };
        let dlq_writer = DlqWriter::new(dlq_cfg, dlq_tx);

        let event = SinkEvent::new()
            .insert("type", "plugin.invoked")
            .insert("dispatcher_trace_id", "trace-dlq-fail");

        // DLQ write must fail (dlq_root is a file, not a dir).
        // Must NOT panic.
        let result = dlq_writer.write_event("test-sink", "http", &event, DlqReason::RetryExhausted);
        assert!(
            result.is_err(),
            "BC-3.07.004: DLQ write to unwritable path must return Err"
        );

        // Oracle: internal.sink_dlq_failure event emitted, not Write.
        let deadline = std::time::Instant::now() + Duration::from_millis(200);
        let mut failure_events: Vec<SinkDlqEvent> = Vec::new();
        while std::time::Instant::now() < deadline {
            if let Ok(ev) = dlq_rx.try_recv() {
                failure_events.push(ev);
            }
        }

        let failure_count = failure_events
            .iter()
            .filter(|e| matches!(e, SinkDlqEvent::Failure(_)))
            .count();
        assert_eq!(
            failure_count, 1,
            "AC-4 BC-3.07.004: exactly 1 internal.sink_dlq_failure event must be emitted; got {}",
            failure_count
        );

        if let Some(SinkDlqEvent::Failure(f)) = failure_events.first() {
            assert_eq!(
                f.sink_name, "test-sink",
                "BC-3.07.004: failure event sink_name must match"
            );
            assert!(
                !f.error.is_empty(),
                "BC-3.07.004: failure event error must be non-empty"
            );
        }
    });
}

/// BC-3.07.003 canonical TV — DLQ event schema validation.
///
/// Verifies the `internal.sink_dlq_write` event fields match the
/// BC-3.07.003 canonical test vector schema:
/// type, sink_name, event_type, reason, ts, dispatcher_trace_id.
///
/// RED gate: will fail if DlqWriteEvent fields do not match the contracted schema.
#[test]
fn test_BC_3_07_003_dlq_write_event_schema_matches_canonical_tv() {
    let tmp = tempfile::tempdir().unwrap();
    let dlq_root = tmp.path().join("dlq-schema-test");
    std::fs::create_dir_all(&dlq_root).unwrap();

    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    rt.block_on(async {
        let (dlq_tx, mut dlq_rx) = tokio::sync::mpsc::channel::<SinkDlqEvent>(64);
        let dlq_cfg = DlqWriterConfig {
            template: "dead-letter-{name}-{date}.jsonl".to_owned(),
            size_cap_bytes: 100 * 1024 * 1024,
            project: None,
            dlq_root,
        };
        let dlq_writer = DlqWriter::new(dlq_cfg, dlq_tx);

        let event = SinkEvent::new()
            .insert("type", "plugin.timeout")
            .insert("dispatcher_trace_id", "trace-schema-check")
            .insert("ts_epoch", serde_json::json!(1_777_003_425_000_u64));

        let result = dlq_writer.write_event("prod-http", "http", &event, DlqReason::RetryExhausted);
        assert!(result.is_ok(), "DLQ write must succeed to test schema");

        // Wait briefly for the channel event.
        let deadline = std::time::Instant::now() + Duration::from_millis(200);
        let mut received: Option<SinkDlqEvent> = None;
        while std::time::Instant::now() < deadline {
            if let Ok(ev) = dlq_rx.try_recv() {
                received = Some(ev);
                break;
            }
        }

        let ev =
            received.expect("BC-3.07.003: DLQ write must emit an event to the internal channel");
        match ev {
            SinkDlqEvent::Write(w) => {
                assert_eq!(w.sink_name, "prod-http", "sink_name must match");
                assert_eq!(w.sink_type, "http", "sink_type must match");
                assert_eq!(w.event_type, "plugin.timeout", "event_type must match");
                assert_eq!(
                    w.reason,
                    DlqReason::RetryExhausted,
                    "reason must be retry_exhausted"
                );
                // ts must be a recent timestamp (within last minute).
                let now = chrono::Utc::now();
                let delta = now.signed_duration_since(w.ts);
                assert!(
                    delta.num_seconds() < 60,
                    "BC-3.07.003: ts must be a recent UTC timestamp; delta={}s",
                    delta.num_seconds()
                );
            }
            SinkDlqEvent::Failure(f) => {
                panic!("BC-3.07.003: expected Write event, got Failure: {f:?}");
            }
        }
    });
}
