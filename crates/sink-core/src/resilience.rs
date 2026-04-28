//! Shared resilience primitives for all sink drivers (S-4.04).
//!
//! Provides [`RetryPolicy`], [`CircuitBreaker`], and [`CircuitState`] so
//! every HTTP-based sink (`sink-http`, `sink-datadog`, `sink-honeycomb`,
//! `sink-otel-grpc`) can share a single, configurable retry + circuit-
//! breaker implementation rather than each duplicating the logic.
//!
//! ## Design
//!
//! - **RetryPolicy** owns backoff math (exponential + jitter).  It is
//!   pure-data: no internal mutable state, no timers.
//! - **CircuitBreaker** owns the three-state machine
//!   (Closed → Open → HalfOpen → Closed).  It carries mutable state
//!   behind a `Mutex` so it can be shared across threads.
//! - **with_retry** is the effectful shell: it drives the retry loop,
//!   consults the circuit breaker, sleeps via `tokio::time::sleep`, and
//!   returns either the successful value or the last error.
//!
//! All three items are `STUB` — methods call `unimplemented!()` until the
//! implementer fills them in (RED gate).

#![deny(missing_docs)]

use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Circuit breaker state.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CircuitState {
    /// Normal operation: requests flow through.
    Closed,
    /// Sustained failures detected: requests are rejected immediately
    /// without calling the underlying operation.
    Open {
        /// When the circuit may transition to [`CircuitState::HalfOpen`].
        opened_at: Instant,
    },
    /// One test request is allowed through; success closes the circuit,
    /// failure re-opens it.
    HalfOpen,
}

/// Per-sink retry configuration.
///
/// Field names mirror the story AC wording for traceability.
///
/// ```text
/// delay_n = min(base_delay_ms * 2^n + jitter, max_delay_ms)
/// ```
/// where `n` is the zero-based retry index and `jitter` is a random value
/// in `[0, base_delay_ms * jitter_factor]`.
#[derive(Debug, Clone)]
pub struct RetryPolicy {
    /// Maximum number of retry attempts (not counting the initial attempt).
    /// 0 means no retries — fail immediately.
    pub max_retries: u32,
    /// Initial backoff delay in milliseconds (base for the exponential).
    pub base_delay_ms: u64,
    /// Hard cap on the computed backoff delay, in milliseconds.
    pub max_delay_ms: u64,
    /// Fraction of `base_delay_ms` added as random jitter (`[0, 1]`).
    /// e.g. `0.25` adds up to 25 % of `base_delay_ms` as jitter.
    pub jitter_factor: f64,
}

impl RetryPolicy {
    /// Compute the backoff delay for retry number `n` (0-based).
    ///
    /// Formula: `min(base_delay_ms * 2^n + jitter, max_delay_ms)`
    /// where `jitter ∈ [0, base_delay_ms * jitter_factor]`.
    ///
    /// # Panics
    ///
    /// Panics in the stub (not yet implemented).
    pub fn delay_for_attempt(&self, _n: u32) -> Duration {
        unimplemented!("RetryPolicy::delay_for_attempt — awaiting implementation (S-4.04)")
    }

    /// Compute the deterministic (zero-jitter) backoff for retry `n`.
    ///
    /// Useful in tests that need a reproducible delay without a PRNG.
    pub fn delay_for_attempt_no_jitter(&self, _n: u32) -> Duration {
        unimplemented!(
            "RetryPolicy::delay_for_attempt_no_jitter — awaiting implementation (S-4.04)"
        )
    }
}

impl Default for RetryPolicy {
    fn default() -> Self {
        Self {
            max_retries: 3,
            base_delay_ms: 100,
            max_delay_ms: 30_000,
            jitter_factor: 0.25,
        }
    }
}

/// Internal mutable state for [`CircuitBreaker`].
struct BreakerInner {
    state: CircuitState,
    consecutive_failures: u32,
    /// Buffered event type strings (e.g. `"internal.sink_circuit_opened"`)
    /// accumulated since the last call to
    /// [`CircuitBreaker::take_emitted_events`].
    emitted_events: Vec<String>,
}

/// Per-sink circuit breaker.
///
/// Thread-safe: inner state is guarded by a `Mutex`.
pub struct CircuitBreaker {
    /// How many consecutive failures trip the circuit (Closed → Open).
    pub failure_threshold: u32,
    /// How long the circuit stays Open before allowing one test request.
    pub cool_off: Duration,
    inner: Mutex<BreakerInner>,
}

impl CircuitBreaker {
    /// Create a new circuit breaker, initially `Closed`.
    pub fn new(failure_threshold: u32, cool_off: Duration) -> Self {
        Self {
            failure_threshold,
            cool_off,
            inner: Mutex::new(BreakerInner {
                state: CircuitState::Closed,
                consecutive_failures: 0,
                emitted_events: Vec::new(),
            }),
        }
    }

    /// Drain and return the list of internal event type strings emitted by
    /// circuit state transitions since the last call.
    ///
    /// Expected values: `"internal.sink_circuit_opened"`,
    /// `"internal.sink_circuit_closed"`.
    ///
    /// Useful in tests and for the dispatcher integration that routes
    /// circuit events into the sink pipeline.
    ///
    /// # Panics
    ///
    /// Panics in the stub (not yet implemented).
    pub fn take_emitted_events(&self) -> Vec<String> {
        unimplemented!(
            "CircuitBreaker::take_emitted_events — awaiting implementation (S-4.04)"
        )
    }

    /// Snapshot the current [`CircuitState`].
    ///
    /// If the breaker is `Open` and the cool-off has elapsed, the state
    /// transitions to `HalfOpen` before returning.
    pub fn state(&self) -> CircuitState {
        unimplemented!("CircuitBreaker::state — awaiting implementation (S-4.04)")
    }

    /// Record a successful operation.
    ///
    /// - Resets the consecutive-failure counter.
    /// - Transitions `HalfOpen` → `Closed`, or is a no-op in `Closed`.
    /// - Emits `internal.sink_circuit_closed` when transitioning from
    ///   `HalfOpen` or `Open`.
    pub fn record_success(&self) {
        unimplemented!("CircuitBreaker::record_success — awaiting implementation (S-4.04)")
    }

    /// Record a failed operation.
    ///
    /// - Increments the consecutive-failure counter.
    /// - When the counter reaches `failure_threshold`, transitions
    ///   `Closed` → `Open` and emits `internal.sink_circuit_opened`.
    /// - `HalfOpen` failure transitions back to `Open` immediately.
    pub fn record_failure(&self) {
        unimplemented!("CircuitBreaker::record_failure — awaiting implementation (S-4.04)")
    }

    /// Returns `true` when the circuit is `Open` and the cool-off has
    /// **not** yet elapsed, meaning requests must be rejected immediately.
    pub fn is_open(&self) -> bool {
        unimplemented!("CircuitBreaker::is_open — awaiting implementation (S-4.04)")
    }
}

impl std::fmt::Debug for CircuitBreaker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CircuitBreaker")
            .field("failure_threshold", &self.failure_threshold)
            .field("cool_off", &self.cool_off)
            .finish_non_exhaustive()
    }
}

/// Error returned by [`with_retry`] when all attempts are exhausted or
/// the circuit is open.
#[derive(Debug)]
pub enum RetryError<E> {
    /// The operation returned an error on every attempt and retries are
    /// exhausted.
    Exhausted {
        /// The last error returned by the underlying operation.
        last_error: E,
        /// Total number of attempts made (initial + retries).
        attempts: u32,
    },
    /// The circuit breaker was `Open`; the operation was never called.
    CircuitOpen,
}

impl<E: std::fmt::Display> std::fmt::Display for RetryError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            RetryError::Exhausted {
                last_error,
                attempts,
            } => write!(f, "retry exhausted after {attempts} attempts: {last_error}"),
            RetryError::CircuitOpen => write!(f, "circuit breaker is open — request rejected"),
        }
    }
}

/// Execute an async operation `op` under the given [`RetryPolicy`] and
/// [`CircuitBreaker`].
///
/// ## Behaviour (stub — not yet implemented)
///
/// 1. If the circuit is `Open`, return `Err(RetryError::CircuitOpen)`
///    immediately without calling `op`.
/// 2. Otherwise call `op`.  On success, call
///    [`CircuitBreaker::record_success`] and return `Ok(value)`.
/// 3. On failure, call [`CircuitBreaker::record_failure`], sleep for
///    `policy.delay_for_attempt(attempt)`, then retry — up to
///    `policy.max_retries` additional attempts.
/// 4. If all attempts fail, return `Err(RetryError::Exhausted { last_error, attempts })`.
///
/// The function is `async` because it calls `tokio::time::sleep` between
/// retries (VP-011: sink submit must not block the dispatcher).
///
/// # Panics
///
/// Panics in the stub (not yet implemented).
pub async fn with_retry<F, Fut, T, E>(
    _policy: &RetryPolicy,
    _breaker: &CircuitBreaker,
    _op: F,
) -> Result<T, RetryError<E>>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    unimplemented!("with_retry — awaiting implementation (S-4.04)")
}
