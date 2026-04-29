//! AC-1: DatadogSink implements the Sink trait; struct is pub-exported.
//!
//! BC-3.01.001 postcondition 1: Sink trait interface shared across all sink types.
//! BC-3.01.001 invariant 1: same Sink trait interface shared across all sink types.

use sink_core::{Sink, SinkEvent};
use sink_datadog::{DatadogSink, DatadogSinkConfig};

/// Minimal valid config TOML with api_key for tests that need a working DatadogSink.
fn valid_config_toml() -> String {
    r#"
schema_version = 1
type = "datadog"
name = "test-datadog-sink"
api_key = "test-api-key-abc123"
"#
    .to_string()
}

/// BC-3.01.001 — DatadogSink satisfies the Sink trait at the call site.
///
/// Exercises: DatadogSinkConfig::from_toml, DatadogSink::new, Sink::name,
/// Sink::accepts, Sink::submit, Sink::flush, Sink::shutdown.
/// All will panic (unimplemented!) until the implementer fills them in.
#[test]
fn test_BC_3_01_001_datadog_sink_implements_sink_trait() {
    let toml = valid_config_toml();
    let config = DatadogSinkConfig::from_toml(&toml)
        .expect("from_toml must succeed")
        .expect("must return Some for valid datadog type");

    let sink = DatadogSink::new(config).expect("DatadogSink::new must succeed");

    // Verify name is set from config.
    assert_eq!(sink.name(), "test-datadog-sink");

    // Verify the trait bound: the sink must accept a generic event.
    let ev = SinkEvent::new().insert("type", "test.event");
    assert!(
        sink.accepts(&ev),
        "enabled sink with no routing filter must accept all events"
    );

    // submit is non-blocking; call it and verify it returns without panic.
    sink.submit(ev);

    // flush must complete without error on an empty queue.
    sink.flush().expect("flush must not error on empty queue");

    // Coerce to trait object to confirm Send + Sync bounds are satisfied.
    let _: &dyn Sink = &sink;

    sink.shutdown();
}

/// BC-3.01.001 invariant 1 — DatadogSink is pub and usable as a trait object.
///
/// Exercises: DatadogSink visibility, trait object coercion.
/// Verifies DatadogSink struct can be embedded or boxed by downstream consumers.
#[test]
fn test_BC_3_01_001_datadog_sink_exported_as_trait_object() {
    let toml = valid_config_toml();
    let config = DatadogSinkConfig::from_toml(&toml)
        .expect("from_toml must succeed")
        .expect("must return Some");

    let sink: Box<dyn Sink> =
        Box::new(DatadogSink::new(config).expect("DatadogSink::new must succeed"));

    // The trait object must be usable — name() call exercises the vtable.
    let name = sink.name();
    assert!(!name.is_empty(), "sink name must not be empty");
}
