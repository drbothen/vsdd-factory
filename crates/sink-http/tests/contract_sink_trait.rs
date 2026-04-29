//! AC-1 + AC-9: HttpSink implements Sink trait; HttpSink struct exported for reuse.
//!
//! BC-3.01.001 postcondition 1: Sink trait interface shared across all sink types.
//! BC-3.01.001 invariant 1: same Sink trait interface shared across all sink types.

use sink_core::{Sink, SinkEvent};
use sink_http::{HttpSink, HttpSinkConfig};

/// Minimal valid config TOML for tests that need a working HttpSink.
fn valid_config_toml(url: &str) -> String {
    format!(
        r#"
schema_version = 1
type = "http"
name = "test-http-sink"
url = "{url}"
"#
    )
}

/// BC-3.01.001 — HttpSink satisfies the Sink trait at the call site.
///
/// Exercises: HttpSinkConfig::from_toml, HttpSink::new, Sink::name,
/// Sink::accepts, Sink::submit, Sink::flush, Sink::shutdown.
/// All will panic (unimplemented!) until the implementer fills them in.
#[test]
fn test_BC_3_01_001_http_sink_implements_sink_trait() {
    let toml = valid_config_toml("http://localhost:9999/events");
    let config = HttpSinkConfig::from_toml(&toml)
        .expect("from_toml must succeed")
        .expect("must return Some for valid http type");

    let sink = HttpSink::new(config).expect("HttpSink::new must succeed");

    // Verify name is set from config.
    assert_eq!(sink.name(), "test-http-sink");

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

/// BC-3.01.001 invariant 1 — HttpSink is pub and usable as a trait object.
///
/// Exercises: HttpSink visibility, trait object coercion.
/// Verifies AC-9: HttpSink struct exposed for reuse by Datadog and Honeycomb.
#[test]
fn test_HttpSink_struct_exported_for_reuse() {
    // This test verifies that HttpSink is a public type that downstream crates
    // (S-4.02, S-4.03) can import and embed. We construct it with a valid config
    // and coerce it into a Box<dyn Sink> — the same pattern Datadog/Honeycomb use.
    let toml = valid_config_toml("http://localhost:9999/events");
    let config = HttpSinkConfig::from_toml(&toml)
        .expect("from_toml must succeed")
        .expect("must return Some");

    let sink: Box<dyn Sink> = Box::new(HttpSink::new(config).expect("HttpSink::new must succeed"));

    // The trait object must be usable — name() call exercises the vtable.
    let name = sink.name();
    assert!(!name.is_empty(), "sink name must not be empty");
}
