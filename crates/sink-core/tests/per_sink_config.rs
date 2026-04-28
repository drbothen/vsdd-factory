//! Tests for per-sink RetryPolicy + CircuitBreaker configuration (S-4.04).
//!
//! AC: "Per-sink configurable"
//! Traces to: BC-3.NN.NNN-retry-isolation-per-sink (v1.1 BC candidate), VP-012

use sink_core::resilience::{CircuitBreaker, RetryPolicy};
use std::time::Duration;

/// VP-012: Sink Failure Affects Only That Sink.
/// Each sink can have different RetryPolicy values; their configs are independent.
#[test]
fn test_BC_3_03_002_vp012_different_sinks_have_different_retry_configs() {
    let policy_http = RetryPolicy {
        max_retries: 3,
        base_delay_ms: 100,
        max_delay_ms: 5_000,
        jitter_factor: 0.25,
    };
    let policy_dd = RetryPolicy {
        max_retries: 5,
        base_delay_ms: 200,
        max_delay_ms: 10_000,
        jitter_factor: 0.10,
    };

    // Different configs → different delay formulas.
    let delay_http_0 = policy_http.delay_for_attempt_no_jitter(0);
    let delay_dd_0 = policy_dd.delay_for_attempt_no_jitter(0);

    assert_ne!(
        delay_http_0, delay_dd_0,
        "http and datadog sinks have different base_delay_ms — delays at n=0 must differ"
    );
}

/// Exercises postcondition: different sinks have independent CircuitBreaker
/// state machines (VP-012 per-sink isolation).
#[test]
fn test_BC_3_03_002_vp012_different_sinks_have_independent_circuit_breakers() {
    let breaker_http = CircuitBreaker::new(3, Duration::from_secs(30));
    let breaker_dd = CircuitBreaker::new(5, Duration::from_secs(60));

    // Trip the http breaker.
    for _ in 0..3 {
        breaker_http.record_failure();
    }

    assert!(
        breaker_http.is_open(),
        "http circuit must be open after 3 failures"
    );
    assert!(
        !breaker_dd.is_open(),
        "datadog circuit must stay closed — VP-012"
    );
}

/// Exercises postcondition: per-sink cool-off durations are respected
/// independently (a short cool-off on one sink does not affect another).
#[test]
fn test_BC_3_03_002_per_sink_cooloff_independent() {
    // Sink A: very short cool-off (0ms).
    let breaker_a = CircuitBreaker::new(1, Duration::from_millis(0));
    // Sink B: long cool-off.
    let breaker_b = CircuitBreaker::new(1, Duration::from_secs(9999));

    breaker_a.record_failure(); // open A
    breaker_b.record_failure(); // open B

    std::thread::sleep(Duration::from_millis(5)); // A's cool-off elapses

    // A has elapsed → HalfOpen (not fully open).
    assert!(
        !breaker_a.is_open(),
        "breaker_a cool-off elapsed — must not be Open"
    );
    // B has not elapsed → still Open.
    assert!(
        breaker_b.is_open(),
        "breaker_b cool-off has NOT elapsed — must still be Open"
    );
}

/// Exercises postcondition: max_retries from each sink's RetryPolicy is used
/// correctly (different sinks stop retrying at different attempt counts).
#[tokio::test]
async fn test_BC_3_03_002_per_sink_max_retries_honoured_independently() {
    use sink_core::resilience::{RetryError, with_retry};
    use std::sync::Arc;
    use std::sync::atomic::{AtomicU32, Ordering};

    let policy_few = RetryPolicy {
        max_retries: 1,
        base_delay_ms: 1,
        max_delay_ms: 10,
        jitter_factor: 0.0,
    };
    let policy_many = RetryPolicy {
        max_retries: 4,
        base_delay_ms: 1,
        max_delay_ms: 10,
        jitter_factor: 0.0,
    };

    let breaker = CircuitBreaker::new(100, Duration::from_secs(60));

    let calls_few = Arc::new(AtomicU32::new(0));
    let cf = Arc::clone(&calls_few);
    let _: Result<(), RetryError<&str>> = with_retry(&policy_few, &breaker, || {
        let c = Arc::clone(&cf);
        async move {
            c.fetch_add(1, Ordering::Relaxed);
            Err("fail")
        }
    })
    .await;

    let calls_many = Arc::new(AtomicU32::new(0));
    let cm = Arc::clone(&calls_many);
    let _: Result<(), RetryError<&str>> = with_retry(&policy_many, &breaker, || {
        let c = Arc::clone(&cm);
        async move {
            c.fetch_add(1, Ordering::Relaxed);
            Err("fail")
        }
    })
    .await;

    assert_eq!(
        calls_few.load(Ordering::Relaxed),
        2,
        "policy_few: max_retries=1 → 2 total calls"
    );
    assert_eq!(
        calls_many.load(Ordering::Relaxed),
        5,
        "policy_many: max_retries=4 → 5 total calls"
    );
}
