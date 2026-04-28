//! Tests for RetryPolicy success path and circuit breaker reset on success (S-4.04).
//!
//! AC: "successful op resets internal counter"
//! Traces to: BC-3.NN.NNN-retry-policy-exponential-backoff, EC-001

use sink_core::resilience::{with_retry, CircuitBreaker, RetryPolicy};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// EC-001: First request fails; second succeeds.
/// Exercises postcondition: with_retry returns Ok on the first successful attempt.
#[tokio::test]
async fn test_BC_3_03_002_ec001_first_fail_second_success_returns_ok() {
    let policy = RetryPolicy {
        max_retries: 3,
        base_delay_ms: 1,
        max_delay_ms: 10,
        jitter_factor: 0.0,
    };
    let breaker = CircuitBreaker::new(10, Duration::from_secs(60));
    let call_count = Arc::new(AtomicU32::new(0));
    let cc = Arc::clone(&call_count);

    let result: Result<&str, RetryError<&str>> = with_retry(&policy, &breaker, || {
        let c = Arc::clone(&cc);
        async move {
            let n = c.fetch_add(1, Ordering::Relaxed);
            if n == 0 {
                Err("first attempt fails")
            } else {
                Ok("second attempt succeeds")
            }
        }
    })
    .await;

    assert_eq!(result.unwrap(), "second attempt succeeds");
    assert_eq!(
        call_count.load(Ordering::Relaxed),
        2,
        "should have called op exactly twice"
    );
}

/// Exercises postcondition: after a success, circuit breaker consecutive
/// failure counter is reset so the circuit does not trip on isolated failures.
#[tokio::test]
async fn test_BC_3_03_002_success_resets_consecutive_failure_counter() {
    let breaker = CircuitBreaker::new(3, Duration::from_secs(60));

    // Record 2 failures (one below the threshold).
    breaker.record_failure();
    breaker.record_failure();

    // A success should reset the counter.
    breaker.record_success();

    // Now record 2 more failures — if reset worked, the circuit stays closed
    // because we're back at 0 + 2, still below the threshold of 3.
    breaker.record_failure();
    breaker.record_failure();

    assert!(
        !breaker.is_open(),
        "circuit must remain closed: success reset the counter before the 2 new failures"
    );
}

/// Exercises postcondition: with_retry returns the Ok value unchanged.
#[tokio::test]
async fn test_BC_3_03_002_success_on_first_attempt_no_retry() {
    let policy = RetryPolicy {
        max_retries: 5,
        base_delay_ms: 1,
        max_delay_ms: 100,
        jitter_factor: 0.0,
    };
    let breaker = CircuitBreaker::new(10, Duration::from_secs(60));
    let call_count = Arc::new(AtomicU32::new(0));
    let cc = Arc::clone(&call_count);

    let result: Result<u32, RetryError<()>> = with_retry(&policy, &breaker, || {
        let c = Arc::clone(&cc);
        async move {
            c.fetch_add(1, Ordering::Relaxed);
            Ok(42u32)
        }
    })
    .await;

    assert_eq!(result.unwrap(), 42);
    assert_eq!(
        call_count.load(Ordering::Relaxed),
        1,
        "op called once (no retry on success)"
    );
}

// Bring RetryError into scope for the test above that uses it.
use sink_core::resilience::RetryError;
