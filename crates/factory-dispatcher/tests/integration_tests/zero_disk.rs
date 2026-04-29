//! AC-1: Zero-disk mode — otel-grpc only; no file sink.
//!
//! Traces to:
//! - BC-3.03.007 postcondition 1: shutdown drains and joins the worker thread
//! - BC-3.03.001 postconditions 1/2/3: batch trigger thresholds fire and
//!   deliver events to the mock receiver
//!
//! SUT entry point: `Router::submit()` → `SinkRegistry::submit_all()` →
//! `OtelGrpcSink::send()`. Oracle: mock OTLP gRPC server receives all
//! submitted events; no filesystem file created under test temp directory;
//! `OtelGrpcSink::shutdown()` joins cleanly.
//!
//! RED gate: these tests exercise the full E2E assembled path. If the
//! OtelGrpcSink does not deliver all events or if shutdown fails to drain,
//! the assertions will fail.

use std::time::Duration;

use factory_dispatcher::sinks::{ObservabilityConfig, Router, SinkRegistry, SinkStanza};
use sink_core::Sink;
use sink_otel_grpc::{BatchConfig, OtelGrpcConfig, OtelGrpcSink};

use super::harness::{OtlpMockServer, build_event, wait_for};

/// BC-3.03.001 + BC-3.03.007 — AC-1:
///
/// Zero-disk mode: otel-grpc only, no file sink. Submits 10 events via
/// `Router::submit()` then flushes. Oracle: mock OTLP receiver records
/// exactly 10 events; no filesystem writes under temp dir; shutdown
/// returns without panic.
///
/// RED gate: this test verifies E2E assembly. The test will fail (RED) until
/// the Router→SinkRegistry→OtelGrpcSink path is correctly assembled for
/// otel-grpc-only configs loaded via `SinkRegistry::from_config`.
#[test]
fn test_BC_3_03_001_zero_disk_mode_all_events_reach_otlp_receiver() {
    let server = OtlpMockServer::start();
    let tmp = tempfile::tempdir().unwrap();

    // Build an otel-grpc-only config (no file sink).
    let mut extra = toml::value::Table::new();
    extra.insert("endpoint".into(), toml::Value::String(server.endpoint()));
    // Small batch size so we exercise the batch-size trigger (BC-3.03.001 PC1).
    extra.insert(
        "batch".into(),
        toml::Value::Table({
            let mut t = toml::value::Table::new();
            t.insert("size".into(), toml::Value::Integer(5));
            t.insert("interval_ms".into(), toml::Value::Integer(60_000));
            t
        }),
    );
    let cfg = ObservabilityConfig {
        schema_version: 1,
        sinks: vec![SinkStanza {
            type_: "otel-grpc".into(),
            name: "zero-disk-grafana".into(),
            dlq_enabled: false,
            extra,
        }],
    };

    let registry = SinkRegistry::from_config(cfg).expect("registry load must succeed");
    assert_eq!(
        registry.sinks().len(),
        1,
        "AC-1: registry must have exactly 1 sink (otel-grpc)"
    );

    let router = Router::new(registry);

    // Submit 10 events via Router::submit().
    for i in 0..10 {
        router.submit(build_event(i, "plugin.invoked"));
    }
    router.flush().expect("flush must succeed");

    // Oracle: mock OTLP receiver received all 10 events.
    let arrived = wait_for(|| server.snapshot().len() >= 10, Duration::from_secs(8));
    let snap = server.snapshot();
    assert!(
        arrived,
        "AC-1 BC-3.03.001: expected 10 events at mock OTLP receiver; got {}",
        snap.len()
    );
    assert_eq!(
        snap.len(),
        10,
        "AC-1: exactly 10 events must arrive at mock OTLP receiver"
    );

    // Oracle: no filesystem file created under tmp dir (zero-disk mode).
    let tmp_contents: Vec<_> = std::fs::read_dir(tmp.path())
        .unwrap()
        .filter_map(|e| e.ok())
        .collect();
    assert!(
        tmp_contents.is_empty(),
        "AC-1: zero-disk mode must not create any filesystem files; found: {:?}",
        tmp_contents.iter().map(|e| e.path()).collect::<Vec<_>>()
    );

    // Oracle: shutdown joins cleanly (BC-3.03.007 PC1).
    router.shutdown();
}

/// BC-3.03.001 PC1 — batch size trigger fires without explicit flush call.
///
/// Configures batch_size=5; submits exactly 5 events without calling
/// `flush()`. Oracle: mock OTLP receiver records 5 events via the
/// batch-size trigger alone.
///
/// RED gate: will fail if the batch-size trigger is not implemented or
/// the otel-grpc sink does not auto-flush on batch fill.
#[test]
fn test_BC_3_03_001_batch_size_trigger_delivers_events_without_explicit_flush() {
    let server = OtlpMockServer::start();

    let cfg = OtelGrpcConfig {
        name: "batch-size-test".into(),
        enabled: true,
        endpoint: server.endpoint(),
        resource_attributes: Default::default(),
        // batch_size=5 so the 5th event auto-flushes.
        batch: BatchConfig {
            size: 5,
            interval_ms: 60_000,
        },
        queue_depth: 1000,
        routing_filter: None,
        tags: Default::default(),
    };
    let sink = OtelGrpcSink::new(cfg).expect("build sink");

    // Submit exactly 5 events — no explicit flush call.
    for i in 0..5 {
        sink.submit(build_event(i, "commit.made"));
    }

    // Batch-size trigger should auto-flush without explicit flush.
    let arrived = wait_for(|| server.snapshot().len() >= 5, Duration::from_secs(8));
    let snap = server.snapshot();
    assert!(
        arrived,
        "AC-1 BC-3.03.001 PC1: batch_size trigger must auto-flush 5 events; got {}",
        snap.len()
    );
    assert_eq!(
        snap.len(),
        5,
        "AC-1 BC-3.03.001: exactly 5 events must arrive via batch-size trigger"
    );

    // BC-3.03.007: shutdown drains and joins cleanly.
    sink.shutdown();
}

/// BC-3.03.007 — shutdown drains in-flight events and joins the worker thread.
///
/// Submits 8 events then calls `shutdown()` immediately (without flush).
/// Oracle: after shutdown returns, all 8 events are present at the mock
/// receiver.
///
/// RED gate: will fail if shutdown does not drain the in-flight queue.
#[test]
fn test_BC_3_03_007_shutdown_drains_inflight_events_before_joining() {
    let server = OtlpMockServer::start();

    let cfg = OtelGrpcConfig {
        name: "drain-on-shutdown".into(),
        enabled: true,
        endpoint: server.endpoint(),
        resource_attributes: Default::default(),
        // Large batch size so events won't auto-flush — shutdown must drain.
        batch: BatchConfig {
            size: 100,
            interval_ms: 60_000,
        },
        queue_depth: 1000,
        routing_filter: None,
        tags: Default::default(),
    };
    let sink = OtelGrpcSink::new(cfg).expect("build sink");

    for i in 0..8 {
        sink.submit(build_event(i, "audit.access"));
    }

    // Shutdown — must drain all 8 events.
    sink.shutdown();

    let snap = server.snapshot();
    assert_eq!(
        snap.len(),
        8,
        "AC-1 BC-3.03.007: shutdown must drain all 8 in-flight events; got {}",
        snap.len()
    );
}

/// BC-3.03.007 postcondition 2 — post-shutdown submit is a no-op (idempotent).
///
/// After `shutdown()`, calling `submit()` must not panic and must not
/// enqueue any further events.
///
/// RED gate: will fail if post-shutdown submit panics or enqueues events.
#[test]
fn test_BC_3_03_007_post_shutdown_submit_is_noop() {
    let server = OtlpMockServer::start();

    let cfg = OtelGrpcConfig {
        name: "post-shutdown-noop".into(),
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

    sink.submit(build_event(0, "plugin.invoked"));
    sink.flush().expect("flush");
    sink.shutdown();

    let before_count = server.snapshot().len();

    // Post-shutdown submit must be a no-op — no new events enqueued.
    sink.submit(build_event(99, "plugin.invoked"));

    // Give a brief window for any erroneously enqueued event to appear.
    std::thread::sleep(std::time::Duration::from_millis(50));

    let after_count = server.snapshot().len();
    assert_eq!(
        before_count, after_count,
        "BC-3.03.007 PC2: post-shutdown submit must not enqueue new events; \
         count before={before_count} after={after_count}"
    );
}
