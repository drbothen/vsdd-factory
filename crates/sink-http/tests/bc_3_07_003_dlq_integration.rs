//! AC-006b: Integration test — sink-http retry exhaustion → DLQ.
//!
//! Traces to: BC-3.07.003 (event schema correctness) + BC-3.07.004 (failure path).
//!
//! ## Contract exercised
//!
//! A mock HTTP server always returns 5xx. After all retry attempts exhaust,
//! the dropped events must appear in a valid JSONL DLQ file at
//! `.factory/logs/dlq/dead-letter-<sink-name>-{date}.jsonl`.
//!
//! ## RED gate
//!
//! `HttpSink::new_with_observability` is an `unimplemented!()` stub.
//! Every test here will panic with "stub" and thus FAIL.
//!
//! The implementer must wire the `dlq_writer` parameter through the
//! worker loop's retry-exhaustion path (Tasks 2b + 3 of S-4.05).

use httpmock::prelude::*;
use sink_core::{DlqWriter, DlqWriterConfig, Sink, SinkEvent};
use sink_http::{HttpSink, HttpSinkConfig};
use std::sync::Arc;
use tokio::sync::mpsc;

fn make_always_5xx_server() -> MockServer {
    let server = MockServer::start();
    server.mock(|when, then| {
        when.method(POST).path("/events");
        then.status(503).body("Service Unavailable");
    });
    server
}

fn config_for_url_max_attempts(url: &str, max_attempts: u32) -> HttpSinkConfig {
    let toml = format!(
        r#"
schema_version = 1
type = "http"
name = "dlq-test-sink"
url = "{url}"
max_5xx_attempts = {max_attempts}
"#
    );
    HttpSinkConfig::from_toml(&toml)
        .expect("from_toml must succeed")
        .expect("must return Some")
}

fn make_event(event_type: &str) -> SinkEvent {
    SinkEvent::new()
        .insert("type", event_type)
        .insert("ts", "2026-04-28T12:00:00+0000")
}

/// AC-006b — Traces to: BC-3.07.003 (full event schema in DLQ file).
///
/// Mock 5xx forces all retries to exhaust (max_attempts=2 for fast test).
/// After flush(), the DLQ file must exist and contain the dropped event as
/// valid JSONL with at least the `type` field preserved.
///
/// RED gate: HttpSink::new_with_observability is `unimplemented!()`.
#[tokio::test]
async fn test_BC_3_07_003_http_retry_exhaustion_writes_event_to_dlq() {
    let server = make_always_5xx_server();
    let tmp = tempfile::tempdir().unwrap();
    let dlq_root = tmp.path().join("dlq");

    let (dlq_tx, _dlq_rx) = mpsc::channel(16);

    let dlq_cfg = DlqWriterConfig {
        template: "dead-letter-{name}-{date}.jsonl".to_owned(),
        size_cap_bytes: 100 * 1024 * 1024,
        project: None,
        dlq_root: dlq_root.clone(),
    };
    let dlq_writer = Arc::new(DlqWriter::new(dlq_cfg, dlq_tx));

    let http_cfg = config_for_url_max_attempts(&format!("{}/events", server.base_url()), 2);

    // new_with_observability is the stub — this will panic (RED gate).
    let sink = HttpSink::new_with_observability(http_cfg, None, Some(dlq_writer))
        .expect("new_with_observability must succeed");

    // Submit one event that will be dropped after retry exhaustion.
    sink.submit(make_event("plugin.invoked"));

    // Flush waits for the batch to be attempted (and retries to exhaust).
    sink.flush().expect("flush must not error");
    sink.shutdown();

    // The DLQ file must contain the dropped event.
    // Use glob since we don't know today's exact UTC date in the test.
    let entries: Vec<_> = std::fs::read_dir(&dlq_root)
        .expect("DLQ directory must exist")
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_name()
                .to_string_lossy()
                .starts_with("dead-letter-dlq-test-sink-")
        })
        .collect();

    assert_eq!(
        entries.len(),
        1,
        "AC-006b: exactly one DLQ file must exist for 'dlq-test-sink'; found {:?}",
        entries.iter().map(|e| e.file_name()).collect::<Vec<_>>()
    );

    let dlq_content =
        std::fs::read_to_string(entries[0].path()).expect("DLQ file must be readable");

    let line = dlq_content.trim_end();
    assert!(!line.is_empty(), "AC-006b: DLQ file must not be empty");

    let parsed: serde_json::Value =
        serde_json::from_str(line).expect("AC-006b: DLQ content must be valid JSON");

    assert_eq!(
        parsed["type"], "plugin.invoked",
        "AC-006b: BC-3.07.003 TV — dropped event type must be preserved in DLQ"
    );
}

/// AC-006b — Traces to: BC-3.07.003 (multiple events all written to DLQ).
///
/// Submit 3 events; all must land in the DLQ file as 3 separate JSONL lines.
///
/// RED gate: HttpSink::new_with_observability is `unimplemented!()`.
#[tokio::test]
async fn test_BC_3_07_003_multiple_events_all_written_to_dlq_on_retry_exhaustion() {
    let server = make_always_5xx_server();
    let tmp = tempfile::tempdir().unwrap();
    let dlq_root = tmp.path().join("dlq");

    let (dlq_tx, _dlq_rx) = mpsc::channel(16);

    let dlq_cfg = DlqWriterConfig {
        template: "dead-letter-{name}-{date}.jsonl".to_owned(),
        size_cap_bytes: 100 * 1024 * 1024,
        project: None,
        dlq_root: dlq_root.clone(),
    };
    let dlq_writer = Arc::new(DlqWriter::new(dlq_cfg, dlq_tx));

    let http_cfg = config_for_url_max_attempts(&format!("{}/events", server.base_url()), 2);

    let sink = HttpSink::new_with_observability(http_cfg, None, Some(dlq_writer))
        .expect("new_with_observability must succeed");

    sink.submit(make_event("commit.made"));
    sink.submit(make_event("pr.merged"));
    sink.submit(make_event("plugin.invoked"));

    sink.flush().expect("flush must not error");
    sink.shutdown();

    let entries: Vec<_> = std::fs::read_dir(&dlq_root)
        .expect("DLQ directory must exist")
        .filter_map(|e| e.ok())
        .collect();

    assert!(
        !entries.is_empty(),
        "AC-006b: DLQ directory must contain at least one file"
    );

    let total_lines: usize = entries
        .iter()
        .map(|e| {
            let content = std::fs::read_to_string(e.path()).unwrap_or_default();
            content.lines().filter(|l| !l.trim().is_empty()).count()
        })
        .sum();

    assert_eq!(
        total_lines, 3,
        "AC-006b: DLQ must contain exactly 3 JSONL lines for 3 dropped events; got {total_lines}"
    );
}
