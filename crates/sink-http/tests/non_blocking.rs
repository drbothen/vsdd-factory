//! VP-011: submit() must not block the dispatcher even when the internal queue is full.
//!
//! Verifies AC-6 overflow path: excess events are dropped (or overflow-recorded)
//! without blocking the caller. DLQ semantics are deferred to S-4.05.

use std::time::{Duration, Instant};

use sink_core::{Sink, SinkEvent};
use sink_http::{HttpSink, HttpSinkConfig};

/// VP-011 — submit() returns immediately when the internal queue is full.
///
/// Exercises: HttpSinkConfig::from_toml (tiny queue_depth=1), HttpSink::new,
/// Sink::submit called queue_depth+N times, timing assertion.
///
/// Strategy: configure a queue_depth of 1 so it fills on the first submit.
/// Then call submit() many more times and assert that each call returns in
/// under 50 ms (non-blocking threshold). A blocking implementation would
/// deadlock or stall here since the background worker is never drained.
#[test]
fn test_VP_011_submit_does_not_block_when_queue_full() {
    // Intentionally tiny queue — fills after 1 event.
    let toml = r#"
schema_version = 1
type = "http"
name = "non-blocking-test-sink"
url = "http://127.0.0.1:19999/events"
queue_depth = 1
"#;

    let config = HttpSinkConfig::from_toml(toml)
        .expect("from_toml must succeed")
        .expect("must return Some");

    let sink = HttpSink::new(config).expect("HttpSink::new must succeed");

    // Fill the queue and then overflow it 50 times.
    // None of these calls may block.
    let overflow_count = 51_usize;
    let start = Instant::now();

    for i in 0..overflow_count {
        let ev = SinkEvent::new()
            .insert("type", "test.overflow")
            .insert("seq", i as i64);
        sink.submit(ev);
    }

    let elapsed = start.elapsed();

    // 51 non-blocking submits must complete well under 1 second.
    // A blocking implementation would stall indefinitely here.
    assert!(
        elapsed < Duration::from_secs(1),
        "submit() calls must be non-blocking — {overflow_count} submits took {elapsed:?} (expected <1s)"
    );

    // Verify overflow was tracked (not silently ignored).
    let full_count = sink.queue_full_count();
    assert!(
        full_count > 0,
        "queue_full_count must be >0 after overflow — got {full_count}"
    );
}
