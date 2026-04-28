---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: implementer
timestamp: 2026-04-27T00:00:00
phase: "4.0"
inputs:
  - .factory/stories/S-4.06-routing-tag-enrichment.md
input-hash: "4b9819c"
traces_to: .factory/specs/behavioral-contracts/bc-id-mapping.md
origin: greenfield
extracted_from: "S-4.06-routing-tag-enrichment.md:BCs-to-Create"
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
---

# Behavioral Contract BC-3.04.003: Router::submit silently drops events that fail RoutingFilter; no SinkFailure recorded; debug-level log emitted

> Section: Router dispatch layer (S-4.06)
> Traces to: CAP-003 per capabilities.md §CAP-003

## Description

When `Router::submit` processes an event that fails a sink's `RoutingFilter`, the Router silently drops the event for that sink without recording a `SinkFailure` or raising an error. A debug-level log entry with message category `internal.event_filtered` is emitted per (event, filtering-sink) pair.

## Preconditions

1. Router has a configured sink with a non-empty RoutingFilter.
2. An event is submitted that does not pass the RoutingFilter.

## Postconditions

1. The event is not forwarded to that sink's `submit()` call.
2. No `SinkFailure` or error event is recorded.
3. A `debug`-level log entry with message category `internal.event_filtered` is emitted for each (event, filtering-sink) pair. If N sinks filter the same event, N log entries are emitted (NOT deduplicated). Each entry includes the sink_name and event type.

## Invariants

1. Silent drop applies per-sink; other sinks without filter restrictions still receive the event.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Event matches no sink filters at all | Silently dropped for all sinks; a `debug`-level `internal.event_filtered` log entry is observable per sink that filtered it |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Router with Sink-A(allow=['commit.made']); submit type='plugin.invoked' | submit() NOT called on Sink-A; no SinkFailure recorded | happy-path |
| Router with 2 sinks both filtering same event | 2 log entries emitted (one per sink) | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD) | Silent drop: no submit() call + no SinkFailure | unit-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-003: "Stream observability events to multiple configurable sinks" per capabilities.md §CAP-003 |
| Architecture Module | SS-03 — crates/factory-dispatcher/src/sinks/router.rs |
| Stories | S-4.06 |

## Source

Greenfield BC created in S-4.06 per story's "BCs to Create" section.

**Pinned test:** `crates/factory-dispatcher/src/sinks/router.rs::tests::test_BC_3_04_003_silent_drop_no_sink_failure_on_filtered_event`
**Test type:** unit
