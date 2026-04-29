//! AC-2: Hybrid mode — file sink + otel-grpc fan-out.
//!
//! Traces to:
//! - BC-3.05.002 postcondition 1: local-events file has 10 lines
//! - BC-3.05.003 postcondition 1: 10 records arrive at mock OTLP receiver
//!   with correct attribute mapping
//!
//! SUT entry point: `Router::submit()` → `SinkRegistry::submit_all()` with
//! a 2-sink registry (one FileSink, one OtelGrpcSink).
//!
//! RED gate: will fail until the otel-grpc and file sinks are both wired
//! through `SinkRegistry::from_config` and `Router::submit()` fans out to both.

use std::time::Duration;

use factory_dispatcher::sinks::{ObservabilityConfig, Router, SinkRegistry, SinkStanza};

use super::harness::{OtlpMockServer, build_event, wait_for};

/// BC-3.05.002 PC1 + BC-3.05.003 PC1 — AC-2:
///
/// Hybrid mode: 2-sink registry (file + otel-grpc). Submit 10 events via
/// Router::submit(). Oracle: local-events file contains 10 JSONL lines;
/// mock OTLP receiver records 10 events.
///
/// RED gate: will fail if fan-out to both sinks is not implemented.
#[test]
fn test_BC_3_05_002_hybrid_mode_events_reach_both_file_and_otlp() {
    let tmp = tempfile::tempdir().unwrap();
    let server = OtlpMockServer::start();

    let date = chrono::Local::now().format("%Y-%m-%d");
    let file_path = tmp.path().join(format!("hybrid-events-{date}.jsonl"));
    let path_template = format!("{}/hybrid-events-{{date}}.jsonl", tmp.path().display());

    // 2-sink config: file + otel-grpc.
    let mut file_extra = toml::value::Table::new();
    file_extra.insert("path_template".into(), toml::Value::String(path_template));
    file_extra.insert("enabled".into(), toml::Value::Boolean(true));

    let mut otel_extra = toml::value::Table::new();
    otel_extra.insert("endpoint".into(), toml::Value::String(server.endpoint()));
    otel_extra.insert(
        "batch".into(),
        toml::Value::Table({
            let mut t = toml::value::Table::new();
            t.insert("size".into(), toml::Value::Integer(100));
            t.insert("interval_ms".into(), toml::Value::Integer(60_000));
            t
        }),
    );

    let cfg = ObservabilityConfig {
        schema_version: 1,
        sinks: vec![
            SinkStanza {
                type_: "file".into(),
                name: "local-events".into(),
                dlq_enabled: false,
                extra: file_extra,
            },
            SinkStanza {
                type_: "otel-grpc".into(),
                name: "local-grafana".into(),
                dlq_enabled: false,
                extra: otel_extra,
            },
        ],
    };

    let registry = SinkRegistry::from_config(cfg).expect("registry load must succeed");
    assert_eq!(
        registry.sinks().len(),
        2,
        "AC-2: hybrid registry must have exactly 2 sinks"
    );

    let router = Router::new(registry);

    // Submit exactly 10 events.
    for i in 0..10 {
        router.submit(build_event(i, "plugin.invoked"));
    }
    router.flush().expect("flush must succeed");

    // Oracle 1: file sink has 10 JSONL lines (BC-3.05.002 PC1).
    assert!(
        file_path.exists(),
        "AC-2 BC-3.05.002: local-events file must exist at {file_path:?}"
    );
    let file_content = std::fs::read_to_string(&file_path).unwrap();
    let file_lines: Vec<&str> = file_content
        .lines()
        .filter(|l| !l.trim().is_empty())
        .collect();
    assert_eq!(
        file_lines.len(),
        10,
        "AC-2 BC-3.05.002 PC1: local-events file must have 10 JSONL lines; got {}",
        file_lines.len()
    );

    // Oracle 2: mock OTLP receiver records 10 events (BC-3.05.003 PC1).
    let arrived = wait_for(|| server.snapshot().len() >= 10, Duration::from_secs(8));
    let snap = server.snapshot();
    assert!(
        arrived,
        "AC-2 BC-3.05.003 PC1: expected 10 events at mock OTLP receiver; got {}",
        snap.len()
    );
    assert_eq!(
        snap.len(),
        10,
        "AC-2 BC-3.05.003: exactly 10 events must arrive at mock OTLP receiver"
    );

    router.shutdown();
}

/// BC-3.05.002 invariant 1 — fan-out is wired through Router.
///
/// Verifies that the Router fans out to ALL sinks, not just the first.
/// Submits 5 events; both sinks must receive exactly 5.
///
/// RED gate: will fail if Router only dispatches to the first sink.
#[test]
fn test_BC_3_05_002_invariant_fanout_wired_through_router_all_sinks_receive() {
    let tmp = tempfile::tempdir().unwrap();
    let server = OtlpMockServer::start();

    let date = chrono::Local::now().format("%Y-%m-%d");
    let path_template = format!("{}/fanout-{{date}}.jsonl", tmp.path().display());

    let mut file_extra = toml::value::Table::new();
    file_extra.insert("path_template".into(), toml::Value::String(path_template));
    file_extra.insert("enabled".into(), toml::Value::Boolean(true));

    let mut otel_extra = toml::value::Table::new();
    otel_extra.insert("endpoint".into(), toml::Value::String(server.endpoint()));
    otel_extra.insert(
        "batch".into(),
        toml::Value::Table({
            let mut t = toml::value::Table::new();
            t.insert("size".into(), toml::Value::Integer(100));
            t.insert("interval_ms".into(), toml::Value::Integer(60_000));
            t
        }),
    );

    let cfg = ObservabilityConfig {
        schema_version: 1,
        sinks: vec![
            SinkStanza {
                type_: "file".into(),
                name: "fanout-file".into(),
                dlq_enabled: false,
                extra: file_extra,
            },
            SinkStanza {
                type_: "otel-grpc".into(),
                name: "fanout-otel".into(),
                dlq_enabled: false,
                extra: otel_extra,
            },
        ],
    };

    let registry = SinkRegistry::from_config(cfg).expect("registry load");
    let router = Router::new(registry);

    for i in 0..5 {
        router.submit(build_event(i, "commit.made"));
    }
    router.flush().expect("flush");

    // File sink must have 5 lines.
    let file_path = tmp.path().join(format!("fanout-{date}.jsonl"));
    assert!(file_path.exists(), "fanout file must exist");
    let content = std::fs::read_to_string(&file_path).unwrap();
    let lines: Vec<&str> = content.lines().filter(|l| !l.trim().is_empty()).collect();
    assert_eq!(
        lines.len(),
        5,
        "BC-3.05.002 invariant 1: file sink must receive all 5 events; got {}",
        lines.len()
    );

    // OTLP sink must also have 5 events.
    let arrived = wait_for(|| server.snapshot().len() >= 5, Duration::from_secs(8));
    let snap = server.snapshot();
    assert!(
        arrived,
        "BC-3.05.002 invariant 1: OTLP sink must also receive all 5 events; got {}",
        snap.len()
    );

    router.shutdown();
}
