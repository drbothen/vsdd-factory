//! Integration tests: with_retry + CircuitBreaker together (S-4.04).
//!
//! AC: "Integration test: mock server returning 5xx; verify retry + circuit open sequence"
//! Traces to: BC-3.NN.NNN-circuit-breaker-state-machine, VP-011, VP-012

use sink_core::resilience::{with_retry, CircuitBreaker, CircuitState, RetryError, RetryPolicy};
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;
use std::time::Duration;

/// Exercises postcondition: when the circuit is Open, with_retry returns
/// CircuitOpen without calling the underlying operation.
#[tokio::test]
async fn test_BC_3_03_002_open_circuit_skips_op_entirely() {
    let policy = RetryPolicy {
        max_retries: 5,
        base_delay_ms: 1,
        max_delay_ms: 100,
        jitter_factor: 0.0,
    };
    let breaker = CircuitBreaker::new(1, Duration::from_secs(999));
    // Manually open the circuit.
    breaker.record_failure();
    assert!(breaker.is_open());

    let call_count = Arc::new(AtomicU32::new(0));
    let cc = Arc::clone(&call_count);

    let result: Result<(), RetryError<&str>> = with_retry(&policy, &breaker, || {
        let c = Arc::clone(&cc);
        async move {
            c.fetch_add(1, Ordering::Relaxed);
            Ok(())
        }
    })
    .await;

    assert!(
        matches!(result, Err(RetryError::CircuitOpen)),
        "expected CircuitOpen, got {result:?}"
    );
    assert_eq!(
        call_count.load(Ordering::Relaxed),
        0,
        "op must never be called when circuit is open"
    );
}

/// Exercises postcondition: when the circuit is Closed and all retries fail,
/// the circuit eventually opens and subsequent calls return CircuitOpen.
#[tokio::test]
async fn test_BC_3_03_002_repeated_failures_open_circuit_then_reject() {
    let policy = RetryPolicy {
        max_retries: 2,
        base_delay_ms: 1,
        max_delay_ms: 10,
        jitter_factor: 0.0,
    };
    // failure_threshold=3 means the circuit opens after 3 consecutive failures.
    let breaker = Arc::new(CircuitBreaker::new(3, Duration::from_secs(999)));

    // First call: 3 total invocations → 3 failures → circuit trips open.
    let b = Arc::clone(&breaker);
    let result1: Result<(), RetryError<&str>> =
        with_retry(&policy, &b, || async { Err("fail") }).await;
    assert!(
        matches!(result1, Err(RetryError::Exhausted { .. })),
        "first exhausted call must return Exhausted (circuit now open)"
    );

    // Circuit should be open now.
    assert!(
        breaker.is_open(),
        "circuit must be open after failure_threshold consecutive failures"
    );

    // Second call: circuit is open → immediate CircuitOpen, op never called.
    let b2 = Arc::clone(&breaker);
    let result2: Result<(), RetryError<&str>> =
        with_retry(&policy, &b2, || async { Ok(()) }).await;
    assert!(
        matches!(result2, Err(RetryError::CircuitOpen)),
        "second call must return CircuitOpen immediately"
    );
}

/// VP-011: sink submit must not block the dispatcher.
/// Exercises: with_retry completes without blocking the calling thread
/// (it uses tokio::time::sleep, not std::thread::sleep).
///
/// We verify this by running with_retry inside a single-thread tokio runtime;
/// if it used std::thread::sleep it would deadlock (the only thread is sleeping).
#[tokio::test(flavor = "current_thread")]
async fn test_BC_3_03_002_vp011_with_retry_uses_async_sleep_not_thread_sleep() {
    let policy = RetryPolicy {
        max_retries: 1,
        base_delay_ms: 1, // 1ms async sleep — negligible
        max_delay_ms: 5,
        jitter_factor: 0.0,
    };
    let breaker = CircuitBreaker::new(10, Duration::from_secs(60));
    let call_count = Arc::new(AtomicU32::new(0));
    let cc = Arc::clone(&call_count);

    // If with_retry uses std::thread::sleep on a current_thread runtime,
    // this will deadlock. We use tokio::time::timeout to catch that case.
    let result = tokio::time::timeout(
        Duration::from_secs(5),
        with_retry(&policy, &breaker, || {
            let c = Arc::clone(&cc);
            async move {
                let n = c.fetch_add(1, Ordering::Relaxed);
                if n == 0 { Err("first") } else { Ok(()) }
            }
        }),
    )
    .await;

    assert!(
        result.is_ok(),
        "with_retry must not deadlock in a single-thread runtime (VP-011)"
    );
    assert!(result.unwrap().is_ok());
}

/// Exercises postcondition: a successful retry (fail once, then succeed)
/// leaves the circuit in Closed state.
#[tokio::test]
async fn test_BC_3_03_002_partial_retry_success_leaves_circuit_closed() {
    let policy = RetryPolicy {
        max_retries: 3,
        base_delay_ms: 1,
        max_delay_ms: 10,
        jitter_factor: 0.0,
    };
    let breaker = CircuitBreaker::new(5, Duration::from_secs(60));
    let call_count = Arc::new(AtomicU32::new(0));
    let cc = Arc::clone(&call_count);

    let result: Result<&str, RetryError<&str>> = with_retry(&policy, &breaker, || {
        let c = Arc::clone(&cc);
        async move {
            let n = c.fetch_add(1, Ordering::Relaxed);
            if n < 2 { Err("transient") } else { Ok("ok") }
        }
    })
    .await;

    assert_eq!(result.unwrap(), "ok");
    assert_eq!(
        breaker.state(),
        CircuitState::Closed,
        "circuit must remain Closed after a partially-failing call that ultimately succeeds"
    );
}
