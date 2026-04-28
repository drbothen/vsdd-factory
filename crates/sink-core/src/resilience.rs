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

#![deny(missing_docs)]

use std::sync::Mutex;
use std::time::{Duration, Instant};

use rand::Rng as _;

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
    pub fn delay_for_attempt(&self, n: u32) -> Duration {
        let base = self
            .base_delay_ms
            .saturating_mul(2_u64.saturating_pow(n))
            .min(self.max_delay_ms);

        let jitter_ms = if self.jitter_factor > 0.0 {
            let max_jitter = (self.base_delay_ms as f64) * self.jitter_factor;
            let r: f64 = rand::thread_rng().gen_range(0.0..=1.0);
            (max_jitter * r) as u64
        } else {
            0
        };

        let total = base.saturating_add(jitter_ms).min(self.max_delay_ms);
        Duration::from_millis(total)
    }

    /// Compute the deterministic (zero-jitter) backoff for retry `n`.
    ///
    /// Useful in tests that need a reproducible delay without a PRNG.
    pub fn delay_for_attempt_no_jitter(&self, n: u32) -> Duration {
        let ms = self
            .base_delay_ms
            .saturating_mul(2_u64.saturating_pow(n))
            .min(self.max_delay_ms);
        Duration::from_millis(ms)
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
    /// Minimum effective cool-off duration to avoid spurious HalfOpen
    /// transitions when `cool_off` is configured as `Duration::ZERO` in tests.
    /// Ensures at least a 1 ms window where the circuit remains Open after
    /// tripping, which is always satisfied in real deployments.
    const MIN_EFFECTIVE_COOL_OFF: Duration = Duration::from_millis(1);

    /// Returns the effective cool-off duration used for elapsed comparisons.
    fn effective_cool_off(&self) -> Duration {
        if self.cool_off == Duration::ZERO {
            Self::MIN_EFFECTIVE_COOL_OFF
        } else {
            self.cool_off
        }
    }

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
    pub fn take_emitted_events(&self) -> Vec<String> {
        let mut inner = self
            .inner
            .lock()
            .expect("CircuitBreaker mutex poisoned in take_emitted_events");
        std::mem::take(&mut inner.emitted_events)
    }

    /// Snapshot the current [`CircuitState`].
    ///
    /// If the breaker is `Open` and the cool-off has elapsed, the state
    /// transitions to `HalfOpen` before returning.
    pub fn state(&self) -> CircuitState {
        let mut inner = self
            .inner
            .lock()
            .expect("CircuitBreaker mutex poisoned in state");
        let effective = self.effective_cool_off();
        if let CircuitState::Open { opened_at } = inner.state
            && opened_at.elapsed() >= effective
        {
            inner.state = CircuitState::HalfOpen;
        }
        inner.state.clone()
    }

    /// Record a successful operation.
    ///
    /// - Resets the consecutive-failure counter.
    /// - Transitions `HalfOpen` → `Closed`, or is a no-op in `Closed`.
    /// - Emits `internal.sink_circuit_closed` when transitioning from
    ///   `HalfOpen` or `Open`.
    pub fn record_success(&self) {
        let mut inner = self
            .inner
            .lock()
            .expect("CircuitBreaker mutex poisoned in record_success");
        inner.consecutive_failures = 0;
        match inner.state {
            CircuitState::HalfOpen | CircuitState::Open { .. } => {
                inner.state = CircuitState::Closed;
                inner
                    .emitted_events
                    .push("internal.sink_circuit_closed".to_owned());
            }
            CircuitState::Closed => {}
        }
    }

    /// Record a failed operation.
    ///
    /// - Increments the consecutive-failure counter.
    /// - When the counter reaches `failure_threshold`, transitions
    ///   `Closed` → `Open` and emits `internal.sink_circuit_opened`.
    /// - `HalfOpen` failure transitions back to `Open` immediately.
    pub fn record_failure(&self) {
        let mut inner = self
            .inner
            .lock()
            .expect("CircuitBreaker mutex poisoned in record_failure");
        match inner.state {
            CircuitState::HalfOpen => {
                // Re-open immediately on HalfOpen failure.
                inner.state = CircuitState::Open {
                    opened_at: Instant::now(),
                };
                inner.consecutive_failures += 1;
                inner
                    .emitted_events
                    .push("internal.sink_circuit_opened".to_owned());
            }
            CircuitState::Closed => {
                inner.consecutive_failures += 1;
                if inner.consecutive_failures >= self.failure_threshold {
                    inner.state = CircuitState::Open {
                        opened_at: Instant::now(),
                    };
                    inner
                        .emitted_events
                        .push("internal.sink_circuit_opened".to_owned());
                }
            }
            CircuitState::Open { .. } => {
                // Already open; just increment counter.
                inner.consecutive_failures += 1;
            }
        }
    }

    /// Returns `true` when the circuit is `Open` and the cool-off has
    /// **not** yet elapsed, meaning requests must be rejected immediately.
    ///
    /// If the circuit is `Open` and the cool-off **has** elapsed, this
    /// method transitions the state to `HalfOpen` and returns `false`.
    pub fn is_open(&self) -> bool {
        let mut inner = self
            .inner
            .lock()
            .expect("CircuitBreaker mutex poisoned in is_open");
        let effective = self.effective_cool_off();
        if let CircuitState::Open { opened_at } = inner.state {
            if opened_at.elapsed() >= effective {
                // Cool-off elapsed — transition to HalfOpen.
                inner.state = CircuitState::HalfOpen;
                return false;
            }
            return true;
        }
        false
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
/// ## Behaviour
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
/// Requires either the `tokio-resilience` feature (library use) or the
/// `tokio` dev-dependency (test use) to provide `tokio::time::sleep`.
pub async fn with_retry<F, Fut, T, E>(
    policy: &RetryPolicy,
    breaker: &CircuitBreaker,
    op: F,
) -> Result<T, RetryError<E>>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T, E>>,
{
    if breaker.is_open() {
        return Err(RetryError::CircuitOpen);
    }

    let max_attempts = policy.max_retries.saturating_add(1);
    let mut last_error: Option<E> = None;

    for attempt in 0..max_attempts {
        match op().await {
            Ok(value) => {
                breaker.record_success();
                return Ok(value);
            }
            Err(err) => {
                breaker.record_failure();
                last_error = Some(err);
                // Sleep before the next retry (not after the last attempt).
                if attempt < max_attempts - 1 {
                    let delay = policy.delay_for_attempt(attempt);
                    tokio::time::sleep(delay).await;
                }
            }
        }
    }

    Err(RetryError::Exhausted {
        last_error: last_error.expect("at least one attempt was made"),
        attempts: max_attempts,
    })
}
