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
  - .factory/specs/behavioral-contracts/ss-03/BC-3.01.008.md
input-hash: "41088b0"
traces_to: .factory/specs/prd.md#FR-045
subsystem: "SS-03"
capability: "CAP-003"
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

# BC-3.07.002: sink driver emits `internal.sink_error` event on each recorded failure

> Section: SS-03 Observability Sinks — cross-sink failure event emission
> Promoted from implicit gap noted in S-4.04 backlog (`BC-3.NN.NNN-cross-sink-failure-recording`).
> Scoped to all sink drivers that record into `Mutex<Vec<SinkFailure>>`: sink-http, sink-otel-grpc,
> sink-file.

## Description

Each sink driver that records a `SinkFailure` into its internal buffer must also emit an
`internal.sink_error` structured event via the dispatcher's event bus. This makes sink health
observable to operators without requiring them to poll the internal `SinkFailure` buffer directly.
The event is emitted synchronously with the failure recording — if recording succeeds, the event
is emitted; if emission fails, it is silently dropped (emission failure is never fatal).
This BC is additive only: BC-3.01.008's recording postcondition is preserved; emission occurs
alongside (not in place of) recording. Event conforms to `INTERNAL_EVENT_SCHEMA_VERSION = 1`
(PRD §3.1); the `type` literal `internal.sink_error` is reserved in the SS-01 internal-event-type
registry.

## Preconditions

1. A sink driver instance (sink-http, sink-otel-grpc, or sink-file) has attempted a send
   operation and the attempt has resulted in a failure (5xx response, connection error, I/O error,
   or retry exhaustion).
2. The failure is about to be recorded into the sink's `Mutex<Vec<SinkFailure>>` buffer.
3. The dispatcher event bus is available for emission (the sink holds a reference to it or to
   the internal-log writer).

## Postconditions

1. An `internal.sink_error` event is emitted to the dispatcher's event bus with the following
   mandatory fields:
   - `type`: `"internal.sink_error"` (string, exact)
   - `sink_name`: the operator-assigned name for this sink instance (string)
   - `sink_type`: one of `"file"`, `"otel-grpc"`, `"http"` (string)
   - `error_message`: human-readable description of the failure (string, non-empty)
   - `attempt`: the retry attempt number at which the failure occurred (u32, 0-indexed)
2. The `SinkFailure` is also recorded into `Mutex<Vec<SinkFailure>>` as before (BC-3.01.008
   postcondition 1 is preserved — this BC is additive only).
3. If `internal.sink_error` event emission itself fails (e.g., event bus channel is full or
   closed), the failure is silently ignored — the sink does NOT panic and does NOT fail to record
   the `SinkFailure`.
4. The event is emitted on the same thread/task that records the failure — no additional
   concurrency is introduced for emission.

## Invariants

1. Emission is fire-and-forget: the sink never awaits acknowledgement that the event reached a
   downstream sink (no deadlock risk from recursive sink-calls).
2. `internal.sink_error` events are NOT routed back through the SinkRegistry (no recursive loop):
   the emission target is the internal-log writer or a dedicated internal event channel, not the
   full fan-out registry.
3. A single send failure produces exactly one `internal.sink_error` event. Retry attempts that
   individually fail each produce one event per failure — not one event per batch.
4. The `sink_name` field matches the operator-configured name used in `observability-config.toml`
   for that sink instance.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Event bus channel is full at time of emission | Emission silently dropped; `SinkFailure` still recorded; sink continues |
| EC-002 | Event bus is shut down (dispatcher shutting down) | Emission silently dropped; `SinkFailure` still recorded; sink continues |
| EC-003 | Failure occurs on final retry attempt (retry exhaustion) | One `internal.sink_error` emitted for the final attempt; total N events for N failed attempts |
| EC-004 | Sink is disabled (`enabled=false`) | No send attempted, no failure, no `internal.sink_error` event emitted |
| EC-005 | First attempt succeeds after previous failure | No `internal.sink_error` for the successful attempt; prior failure events already emitted |
| EC-006 | Failure message contains non-UTF-8 bytes (OS error) | `error_message` is sanitized to valid UTF-8 (replacement char or lossy conversion); event still emitted |
| EC-007 | `sink_name` not configured (unnamed sink) | `sink_name` defaults to `"<unnamed>"` (string literal); event still emitted |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| sink-http receives 503 on attempt 0 | one `internal.sink_error` event with `sink_type="http"`, `attempt=0`, `error_message` non-empty | happy-path |
| sink-file write fails (read-only dir) | one `internal.sink_error` event with `sink_type="file"`, `attempt=0` | happy-path |
| sink-otel-grpc send fails | one `internal.sink_error` event with `sink_type="otel-grpc"`, `attempt=0` | happy-path |
| sink-http fails 3 times (max_retries=3) | 3 `internal.sink_error` events, `attempt` = 0, 1, 2 in order | boundary |
| event bus channel at capacity during emission | `SinkFailure` recorded; no panic; zero events in bus (dropped) | edge-case |
| sink disabled (`enabled=false`) | no `internal.sink_error` events emitted | edge-case |

## Verification Properties

| VP ID | Property | Proof Method |
|-------|----------|-------------|
| VP-012 | Sink Failure Affects Only That Sink | unit-test: `internal.sink_error` from sink-A does not affect sink-B operation |
| VP-007 | Dispatcher Self-Telemetry Is Always-On and Never Panics | unit-test: emission failure (full channel) does not panic the sink |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-003 ("Stream observability events to multiple configurable sinks") per capabilities.md §CAP-003 |
| Capability Anchor Justification | CAP-003 ("Stream observability events to multiple configurable sinks") per capabilities.md §CAP-003 — `internal.sink_error` is an observability event that operators consume to monitor sink health; emitting it is part of the "fans out every internal event" promise in CAP-003 |
| L2 Domain Invariants | TBD — domain invariant file pending; no DI-NNN assigned yet |
| Architecture Module | SS-03 (crates/sink-core/src/events.rs — SinkErrorEvent schema [pure-core]; crates/sink-http/src/lib.rs, crates/sink-otel-grpc/src/lib.rs, crates/sink-file/src/lib.rs — emission sites [effectful-shell]) |
| Functional Requirement | FR-045 ("Emit `internal.sink_error` structured event on each sink failure") per prd.md §FR-045 |
| Stories | S-4.10 (Wave 11, implementing story) |

## Related BCs

- BC-3.01.008: "file sink failures recorded into Mutex<Vec<SinkFailure>>" — the recording
  mechanism this BC extends with emission. Postcondition 1 of BC-3.01.008 is preserved; this BC
  adds emission on top.
- BC-3.03.002: "Send failure protocol — drop gRPC client on error; rebuild on next batch" — the
  gRPC sink failure path where `internal.sink_error` emission will be added.
- BC-3.07.001: "sink-http exponential backoff with jitter between 5xx retries" — companion BC
  for the sink-http retry path; failure events are emitted at each retry failure before backoff.

## Architecture Anchors

- `architecture/SS-03-observability-sinks.md` — section: cross-sink failure recording and event emission
- `crates/sink-http/src/lib.rs` — failure recording site; emission added in S-4.10
- `crates/sink-otel-grpc/src/lib.rs` — failure recording site; emission added in S-4.10
- `crates/sink-file/src/lib.rs` — failure recording site; emission added in S-4.10

## Story Anchor

S-4.10 — internal.sink_error event emission, cross-sink (Wave 11)

## VP Anchors

- VP-007 (Dispatcher Self-Telemetry Is Always-On and Never Panics)
- VP-012 (Sink Failure Affects Only That Sink)
