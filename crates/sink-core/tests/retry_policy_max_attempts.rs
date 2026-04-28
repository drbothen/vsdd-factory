//! Tests for RetryPolicy max_retries enforcement (S-4.04).
//!
//! AC: "Shared RetryPolicy struct: max_retries, base_delay_ms, max_delay_ms, jitter_factor"
//! AC: "after max_attempts retries, returns Err with original failure"
//! Traces to: BC-3.NN.NNN-retry-policy-exponential-backoff, VP-011

use sink_core::resilience::{with_retry, CircuitBreaker, RetryError, RetryPolicy};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Exercises VP-011 postcondition: with_retry exhausts max_retries and returns
/// RetryError::Exhausted carrying the last error.
#[tokio::test]
async fn test_BC_3_03_002_max_retries_exhausted_returns_err() {
    let policy = RetryPolicy {
        max_retries: 3,
        base_delay_ms: 1, // tiny delay so test is fast
        max_delay_ms: 10,
        jitter_factor: 0.0,
    };
    let breaker = CircuitBreaker::new(10, Duration::from_secs(60));
    let call_count = Arc::new(AtomicU32::new(0));
    let cc = Arc::clone(&call_count);

    let result: Result<(), RetryError<&str>> = with_retry(&policy, &breaker, || {
        let c = Arc::clone(&cc);
        async move {
            c.fetch_add(1, Ordering::Relaxed);
            Err("always fails")
        }
    })
    .await;

    assert!(
        matches!(result, Err(RetryError::Exhausted { attempts: 4, .. })),
        "expected Exhausted with 4 attempts (1 initial + 3 retries), got {result:?}"
    );
}

/// Exercises postcondition: the number of op invocations equals
/// max_retries + 1 (initial attempt plus each retry).
#[tokio::test]
async fn test_BC_3_03_002_op_called_max_retries_plus_one_times() {
    let policy = RetryPolicy {
        max_retries: 2,
        base_delay_ms: 1,
        max_delay_ms: 10,
        jitter_factor: 0.0,
    };
    let breaker = CircuitBreaker::new(10, Duration::from_secs(60));
    let call_count = Arc::new(AtomicU32::new(0));
    let cc = Arc::clone(&call_count);

    let _: Result<(), RetryError<&str>> = with_retry(&policy, &breaker, || {
        let c = Arc::clone(&cc);
        async move {
            c.fetch_add(1, Ordering::Relaxed);
            Err("fail")
        }
    })
    .await;

    assert_eq!(
        call_count.load(Ordering::Relaxed),
        3,
        "max_retries=2 means 3 total invocations (1 initial + 2 retries)"
    );
}

/// Exercises postcondition: when max_retries is 0, the op is called exactly
/// once and any error is returned immediately (no retries).
#[tokio::test]
async fn test_BC_3_03_002_zero_max_retries_fails_immediately() {
    let policy = RetryPolicy {
        max_retries: 0,
        base_delay_ms: 1,
        max_delay_ms: 10,
        jitter_factor: 0.0,
    };
    let breaker = CircuitBreaker::new(10, Duration::from_secs(60));
    let call_count = Arc::new(AtomicU32::new(0));
    let cc = Arc::clone(&call_count);

    let result: Result<(), RetryError<&str>> = with_retry(&policy, &breaker, || {
        let c = Arc::clone(&cc);
        async move {
            c.fetch_add(1, Ordering::Relaxed);
            Err("immediate fail")
        }
    })
    .await;

    assert!(
        matches!(result, Err(RetryError::Exhausted { attempts: 1, .. })),
        "max_retries=0 must return Exhausted after 1 attempt"
    );
    assert_eq!(call_count.load(Ordering::Relaxed), 1);
}

/// Exercises postcondition: RetryError::Exhausted carries the last error
/// value from the underlying operation (not a generic message).
#[tokio::test]
async fn test_BC_3_03_002_exhausted_carries_last_error() {
    let policy = RetryPolicy {
        max_retries: 2,
        base_delay_ms: 1,
        max_delay_ms: 10,
        jitter_factor: 0.0,
    };
    let breaker = CircuitBreaker::new(10, Duration::from_secs(60));
    let attempt_counter = Arc::new(AtomicU32::new(0));
    let ac = Arc::clone(&attempt_counter);

    let result: Result<(), RetryError<String>> = with_retry(&policy, &breaker, || {
        let c = Arc::clone(&ac);
        async move {
            let n = c.fetch_add(1, Ordering::Relaxed) + 1;
            Err(format!("error on attempt {n}"))
        }
    })
    .await;

    match result {
        Err(RetryError::Exhausted { last_error, .. }) => {
            assert!(
                last_error.contains("attempt 3"),
                "last_error should be from the final attempt, got: {last_error}"
            );
        }
        other => panic!("expected Exhausted, got {other:?}"),
    }
}
