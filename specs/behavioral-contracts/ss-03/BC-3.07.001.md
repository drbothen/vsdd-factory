---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
origin: greenfield
producer: product-owner
timestamp: 2026-04-27T00:00:00
phase: "1.8"
inputs:
  - .factory/stories/S-4.01-sink-http-driver.md
  - .factory/specs/behavioral-contracts/ss-03/BC-3.03.002.md
input-hash: "41088b0"
traces_to: .factory/specs/prd.md#FR-044
subsystem: "SS-03"
capability: "CAP-024"
lifecycle_status: active
introduced: v1.0.0-rc.1
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
extracted_from: null
---

# BC-3.07.001: sink-http exponential backoff with jitter between 5xx retries

> Section: SS-03 Observability Sinks — sink-http resilience
> Promoted from v1.1 BC candidate `BC-3.NN.NNN-retry-policy-exponential-backoff` in S-4.04 backlog.
> Narrowly scoped to sink-http only; full cross-sink RetryPolicy struct is S-4.04 scope.

## Description

When sink-http receives a 5xx response, it waits a computed backoff delay before issuing the next
retry attempt. The delay follows exponential backoff with uniform jitter: `delay = min(base_delay_ms
* 2^attempt + random_jitter_ms, max_delay_ms)`. This prevents thundering-herd behaviour when a
backend recovers after a transient outage.

## Preconditions

1. A `sink-http` sink instance has been constructed with a `RetryConfig` that includes
   `base_delay_ms`, `max_delay_ms`, and `jitter_factor` fields.
2. The sink's send method has already received a 5xx HTTP response on attempt N (N < max_retries).
3. A next retry attempt (N+1) is about to be issued.

## Postconditions

1. The sink sleeps for `delay_ms = min(base_delay_ms * 2^N + jitter_ms, max_delay_ms)` before
   issuing retry N+1, where `jitter_ms` is drawn uniformly from `[0, base_delay_ms * jitter_factor]`.
2. `delay_ms` is strictly positive: when `base_delay_ms > 0`, even attempt 0 introduces a
   nonzero floor.
3. `delay_ms` never exceeds `max_delay_ms` regardless of N.
4. The backoff sleep occurs on the sink's worker/batch thread — the `submit()` call path remains
   non-blocking (BC-3.03.005 invariant preserved).
5. After the sleep, the HTTP request is retried with the same payload as attempt N.
6. If the response is a 4xx client error other than 408 (Request Timeout) or 429 (Too Many
   Requests), the failure is recorded immediately with no retry and no backoff sleep.

## Invariants

1. `max_delay_ms >= base_delay_ms > 0` is a construction-time invariant; violation is a
   configuration error, not a runtime panic.
2. Jitter source must be seeded from a thread-local or per-instance PRNG — not a global static
   (prevents correlated thundering herd across concurrent sink instances).
3. The sleep does NOT hold any lock over `Mutex<Vec<SinkFailure>>` (i.e., failure recording and
   backoff sleeping are non-overlapping operations).
4. A single invocation of send() produces exactly (max_retries - 1) sleeps on full-failure
   sequences. After the final retry fails, there is no trailing sleep before recording failure.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `base_delay_ms = 0` (misconfigured) | Construction rejects config with `ConfigError::InvalidBackoff`; sink not started |
| EC-002 | `max_delay_ms < base_delay_ms` (misconfigured) | Construction rejects config with `ConfigError::InvalidBackoff`; sink not started |
| EC-003 | Single retry configured (`max_retries = 1`) | No backoff sleep: first attempt fails, failure recorded immediately, no retry |
| EC-004 | 4xx response received (client error) | No retry, no backoff sleep; failure recorded as non-retryable immediately |
| EC-005 | Network timeout (connection refused, not 5xx) | Treated as retriable; backoff applies identically to 5xx path |
| EC-006 | Jitter would make delay exceed max_delay_ms | Clamped to max_delay_ms exactly |
| EC-007 | Concurrent sink instances under same backend | Each instance has independent PRNG seed → jitter values are uncorrelated |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `base=100ms, max=5000ms, jitter_factor=0.5, attempt=0` | `delay in [100, 150]ms` | happy-path |
| `base=100ms, max=5000ms, jitter_factor=0.5, attempt=1` | `delay in [200, 250]ms` | happy-path |
| `base=100ms, max=5000ms, jitter_factor=0.5, attempt=6` | `delay == 5000ms` (clamped) | boundary |
| `base=100ms, max=50ms` | `ConfigError::InvalidBackoff` at construction | error |
| `base=0` | `ConfigError::InvalidBackoff` at construction | error |
| `max_retries=1, 5xx on attempt 0` | failure recorded, no sleep, no attempt 1 | edge-case |
| `max_retries=3, 5xx on attempts 0 and 1, 200 on attempt 2` | 2 sleeps before success; event delivered | happy-path |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-011 | Sink submit Must Not Block the Dispatcher | unit-test: submit returns immediately; backoff sleep in worker thread not on caller |
| VP-012 | Sink Failure Affects Only That Sink | unit-test: two sink instances with different backends do not share backoff state |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-024 ("Per-sink retry, circuit breaker, and dead-letter queue") per capabilities.md §CAP-024 |
| Capability Anchor Justification | CAP-024 ("Per-sink retry, circuit breaker, and dead-letter queue") per capabilities.md §CAP-024 — this BC implements the "retries failed sends with exponential backoff" clause of CAP-024 for the sink-http driver |
| L2 Domain Invariants | TBD — domain invariant file pending; no DI-NNN assigned yet |
| Architecture Module | SS-03 (crates/sink-http/src/retry.rs — backoff formula + jitter computation [pure-core]; crates/sink-http/src/lib.rs — retry loop wiring + tokio::time::sleep [effectful-shell]) |
| Functional Requirement | FR-044 ("Per-sink resilience: retry, circuit breaker, dead-letter queue") per prd.md §FR-044 |
| Stories | S-4.09 (Wave 11, implementing story) |

## Related BCs

- BC-3.03.002: "Send failure protocol — drop gRPC client on error; rebuild on next batch" — the
  reconnect-on-error pattern that this BC's backoff wraps in the HTTP context.
- BC-3.03.005: "Producer-side submit is fully non-blocking via try_send" — invariant 1 of this BC
  depends on: backoff sleep must not be on the submit() call path.
- BC-3.01.008: "file sink failures recorded into Mutex<Vec<SinkFailure>>" — parallel pattern for
  failure recording that sink-http follows; invariant 3 of this BC references this pattern.

## Architecture Anchors

- `architecture/SS-03-observability-sinks.md` — section: sink-http resilience layer
- `crates/sink-http/src/lib.rs` — existing retry loop (added in S-4.01); backoff delay added in S-4.09

## Story Anchor

S-4.09 — sink-http retry backoff with jitter (Wave 11)

## VP Anchors

- VP-011 (Sink submit Must Not Block the Dispatcher)
- VP-012 (Sink Failure Affects Only That Sink)
