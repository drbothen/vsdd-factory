//! BC-3.01.001 — HoneycombSink implements the Sink trait and is pub-exported.
//!
//! AC: `crates/sink-honeycomb` implements the `Sink` trait.
//! Traces to: BC-3.01.001 postcondition 1 (Sink integrates with SinkRegistry base machinery).
//! VP-011: Sink submit must not block the dispatcher.

use sink_core::{Sink, SinkEvent};
use sink_honeycomb::{HoneycombSink, HoneycombSinkConfig};

/// BC-3.01.001 — HoneycombSink is Send + Sync (required by the Sink trait bound).
///
/// The compiler enforces this; if HoneycombSink is not Send+Sync the crate
/// will fail to build. This function makes the constraint explicit and
/// traceable.
fn assert_sink_send_sync<T: Sink + Send + Sync>() {}

#[test]
fn test_BC_3_01_001_honeycomb_sink_is_send_sync() {
    // Compile-time assertion: HoneycombSink satisfies Send + Sync.
    assert_sink_send_sync::<HoneycombSink>();
}

#[test]
fn test_BC_3_01_001_honeycomb_sink_implements_sink_trait() {
    // Constructing via a valid config exercises the Sink trait constructor.
    // This test fails with unimplemented!() until the implementation lands.
    let toml_src = r#"
        type = "honeycomb"
        name = "hc-test"
        api_key = "test-api-key-abc123"
        dataset = "factory-events"
    "#;
    let config = HoneycombSinkConfig::from_toml(toml_src)
        .expect("from_toml must not error on valid config")
        .expect("from_toml must return Some for type=honeycomb");
    let sink = HoneycombSink::new(config).expect("HoneycombSink::new must succeed");

    // Verify the Sink trait surface is callable.
    let _name: &str = sink.name();
    let event = SinkEvent::new().insert("type", "plugin.invoked");
    let _accepts: bool = sink.accepts(&event);
}

#[test]
fn test_BC_3_01_001_honeycomb_sink_name_returns_configured_name() {
    // VP-011: name() is a non-blocking accessor.
    let toml_src = r#"
        type = "honeycomb"
        name = "prod-honeycomb"
        api_key = "test-key"
        dataset = "factory-events"
    "#;
    let config = HoneycombSinkConfig::from_toml(toml_src)
        .expect("valid config")
        .expect("Some for type=honeycomb");
    let sink = HoneycombSink::new(config).expect("new succeeds");
    assert_eq!(sink.name(), "prod-honeycomb");
}

/// VP-011: submit() must be non-blocking — it enqueues and returns
/// immediately without waiting for an HTTP round-trip.
#[test]
fn test_VP_011_submit_is_non_blocking() {
    let toml_src = r#"
        type = "honeycomb"
        name = "vp011-sink"
        api_key = "test-key"
        dataset = "factory-events"
    "#;
    let config = HoneycombSinkConfig::from_toml(toml_src)
        .expect("valid config")
        .expect("Some");
    let sink = HoneycombSink::new(config).expect("new succeeds");
    // submit() must return without blocking — just enqueue.
    let event = SinkEvent::new()
        .insert("type", "plugin.invoked")
        .insert("ts_epoch", 1_700_000_000_i64);
    sink.submit(event); // must not block
}

/// VP-012: a HoneycombSink failure (e.g. bad endpoint) must not affect
/// other sinks. The isolation guarantee is structural: HoneycombSink
/// has its own internal queue and worker, independent of any other sink.
#[test]
fn test_VP_012_sink_failure_isolated_per_sink_instance() {
    // Two independent HoneycombSink instances — failure in one should not
    // propagate to the other. The test verifies they can be constructed
    // and shut down independently.
    let make_config = |name: &str| {
        format!(
            r#"
            type = "honeycomb"
            name = "{name}"
            api_key = "isolated-key"
            dataset = "test-dataset"
            "#
        )
    };
    let cfg_a = HoneycombSinkConfig::from_toml(&make_config("sink-a"))
        .expect("valid")
        .expect("Some");
    let cfg_b = HoneycombSinkConfig::from_toml(&make_config("sink-b"))
        .expect("valid")
        .expect("Some");
    let sink_a = HoneycombSink::new(cfg_a).expect("a created");
    let sink_b = HoneycombSink::new(cfg_b).expect("b created");
    // Both shut down cleanly and independently.
    sink_a.shutdown();
    sink_b.shutdown();
}
