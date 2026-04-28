//! AC-11: OTLP LogRecord field mapping in integration context.
//!
//! Traces to:
//! - BC-3.05.003 postcondition 1: 10 records arrive; `type` → body,
//!   `ts_epoch` → `time_unix_nano`; type and ts_epoch are NOT present
//!   in attributes; all extra fields appear as attributes
//!
//! SUT entry point (exception from Router::submit() — see story SUT boundary):
//! `OtelGrpcSink::send()` directly. Rationale: deep OTLP wire-level
//! verification is independent of routing decisions.
//!
//! RED gate: will fail until OtelGrpcSink correctly maps type→body,
//! ts_epoch→time_unix_nano, and does not leak reserved fields as attributes.

use std::time::Duration;

use opentelemetry_proto::tonic::common::v1::any_value::Value as AnyValueInner;
use opentelemetry_proto::tonic::logs::v1::LogRecord;
use sink_core::{Sink, SinkEvent};
use sink_otel_grpc::{BatchConfig, OtelGrpcConfig, OtelGrpcSink};

use super::harness::{OtlpMockServer, wait_for};

// ── Helper accessors ──────────────────────────────────────────────────────────

fn body_str(record: &LogRecord) -> Option<&str> {
    match record.body.as_ref()?.value.as_ref()? {
        AnyValueInner::StringValue(s) => Some(s.as_str()),
        _ => None,
    }
}

fn attr_str<'a>(record: &'a LogRecord, key: &str) -> Option<&'a str> {
    record
        .attributes
        .iter()
        .find(|kv| kv.key == key)
        .and_then(|kv| kv.value.as_ref())
        .and_then(|v| v.value.as_ref())
        .and_then(|v| match v {
            AnyValueInner::StringValue(s) => Some(s.as_str()),
            _ => None,
        })
}

fn attr_int(record: &LogRecord, key: &str) -> Option<i64> {
    record
        .attributes
        .iter()
        .find(|kv| kv.key == key)
        .and_then(|kv| kv.value.as_ref())
        .and_then(|v| v.value.as_ref())
        .and_then(|v| match v {
            AnyValueInner::IntValue(i) => Some(*i),
            _ => None,
        })
}

// ── Tests ─────────────────────────────────────────────────────────────────────

/// BC-3.05.003 PC1 — AC-11:
///
/// 10 events sent to OtelGrpcSink directly. Oracle:
/// - Mock OTLP receiver records exactly 10 LogRecords
/// - `body = event.type` (string)
/// - `time_unix_nano = event.ts_epoch * 1_000_000`
/// - `type` NOT present in attributes (lifted to body)
/// - `ts_epoch` NOT present in attributes (lifted to time_unix_nano)
/// - All extra fields (dispatcher_trace_id, session_id, etc.) present as attributes
///
/// RED gate: will fail if any field mapping is incorrect.
#[test]
fn test_BC_3_05_003_ten_events_arrive_with_correct_otlp_field_mapping() {
    let server = OtlpMockServer::start();

    let cfg = OtelGrpcConfig {
        name: "otlp-mapping-test".into(),
        enabled: true,
        endpoint: server.endpoint(),
        resource_attributes: Default::default(),
        batch: BatchConfig {
            size: 100,      // Large batch — flush() is the trigger.
            interval_ms: 60_000,
        },
        queue_depth: 1000,
        routing_filter: None,
        tags: Default::default(),
    };
    let sink = OtelGrpcSink::new(cfg).expect("build sink");

    // Submit 10 events with canonical fields.
    let ts_base = 1_777_003_425_000_u64;
    for i in 0..10 {
        let event = SinkEvent::new()
            .insert("type", "plugin.invoked")
            .insert("ts_epoch", serde_json::json!(ts_base + i))
            .insert("dispatcher_trace_id", format!("trace-otlp-{i}"))
            .insert("session_id", "sess-otlp")
            .insert("plugin_name", "capture-commit-activity")
            .insert("seq", serde_json::json!(i as i64));
        sink.submit(event);
    }
    sink.flush().expect("flush must succeed");

    let arrived = wait_for(|| server.snapshot().len() >= 10, Duration::from_secs(8));
    let snap = server.snapshot();
    assert!(
        arrived,
        "AC-11 BC-3.05.003: expected 10 records; got {}",
        snap.len()
    );
    assert_eq!(snap.len(), 10, "BC-3.05.003 PC1: exactly 10 records");

    // Spot-check the first record in detail.
    let r0 = &snap[0];

    // Oracle: body = event.type
    assert_eq!(
        body_str(r0),
        Some("plugin.invoked"),
        "BC-3.05.003: body must equal event.type ('plugin.invoked')"
    );

    // Oracle: time_unix_nano = ts_epoch * 1_000_000
    assert_eq!(
        r0.time_unix_nano,
        ts_base * 1_000_000,
        "BC-3.05.003: time_unix_nano must be ts_epoch * 1_000_000; \
         expected={} got={}",
        ts_base * 1_000_000,
        r0.time_unix_nano
    );

    // Oracle: dispatcher_trace_id in attributes.
    assert_eq!(
        attr_str(r0, "dispatcher_trace_id"),
        Some("trace-otlp-0"),
        "BC-3.05.003: dispatcher_trace_id must be an attribute"
    );
    assert_eq!(
        attr_str(r0, "session_id"),
        Some("sess-otlp"),
        "BC-3.05.003: session_id must be an attribute"
    );
    assert_eq!(
        attr_str(r0, "plugin_name"),
        Some("capture-commit-activity"),
        "BC-3.05.003: plugin_name must be an attribute"
    );
    assert_eq!(
        attr_int(r0, "seq"),
        Some(0),
        "BC-3.05.003: seq must be an integer attribute"
    );

    // Oracle: NO reserved fields leaked as attributes.
    assert!(
        !r0.attributes.iter().any(|kv| kv.key == "type"),
        "BC-3.05.003: 'type' must NOT appear in attributes (lifted to body)"
    );
    assert!(
        !r0.attributes.iter().any(|kv| kv.key == "ts_epoch"),
        "BC-3.05.003: 'ts_epoch' must NOT appear in attributes (lifted to time_unix_nano)"
    );

    sink.shutdown();
}

/// BC-3.05.003 — each of the 10 records has distinct time_unix_nano.
///
/// Verifies that per-event ts_epoch values are correctly mapped to distinct
/// time_unix_nano values (not all the same).
///
/// RED gate: will fail if ts_epoch mapping is batched incorrectly.
#[test]
fn test_BC_3_05_003_each_record_has_distinct_time_unix_nano() {
    let server = OtlpMockServer::start();

    let cfg = OtelGrpcConfig {
        name: "distinct-ts".into(),
        enabled: true,
        endpoint: server.endpoint(),
        resource_attributes: Default::default(),
        batch: BatchConfig {
            size: 100,
            interval_ms: 60_000,
        },
        queue_depth: 1000,
        routing_filter: None,
        tags: Default::default(),
    };
    let sink = OtelGrpcSink::new(cfg).expect("build sink");

    let ts_base = 1_777_003_425_000_u64;
    for i in 0..10 {
        let event = SinkEvent::new()
            .insert("type", "commit.made")
            .insert("ts_epoch", serde_json::json!(ts_base + i));
        sink.submit(event);
    }
    sink.flush().expect("flush");

    let arrived = wait_for(|| server.snapshot().len() >= 10, Duration::from_secs(8));
    assert!(arrived, "10 records must arrive");

    let snap = server.snapshot();
    let timestamps: Vec<u64> = snap.iter().map(|r| r.time_unix_nano).collect();

    // All 10 timestamps must be distinct.
    let unique: std::collections::HashSet<u64> = timestamps.iter().copied().collect();
    assert_eq!(
        unique.len(),
        10,
        "BC-3.05.003: each of the 10 records must have a distinct time_unix_nano; \
         got {} unique values: {:?}",
        unique.len(),
        timestamps
    );

    sink.shutdown();
}
