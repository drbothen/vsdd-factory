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

# Behavioral Contract BC-3.04.004: Router::submit applies RoutingFilter before delegating to each sink (wired dispatch)

> Section: Router dispatch layer (S-4.06)
> Traces to: CAP-003 per capabilities.md §CAP-003
> Deprecates: BC-3.04.001 (thin pass-through state)

## Description

`Router::submit` is the single dispatch gate. For each configured sink, it reads the sink's `RoutingFilter` via `sink.routing_filter()`, applies it to the event, enriches the event with the sink's static tags (non-overwrite), and only then calls `sink.submit()`. This replaces the previous thin pass-through behaviour (BC-3.04.001).

## Preconditions

1. Router wraps a SinkRegistry with one or more configured sinks.
2. Each sink optionally has a RoutingFilter in its SinkConfigCommon.

## Postconditions

1. For each sink: if the sink's RoutingFilter is non-empty and the event fails it, that sink's `submit()` is NOT called.
2. For each sink: if the sink's RoutingFilter is empty or the event passes it, that sink's `submit()` IS called with the (optionally tag-enriched) event.
3. Tag enrichment from sink `tags` config is applied to the event before `submit()` is called.
4. Tag enrichment must NOT overwrite producer-set fields. Producer fields (e.g., `type`, `ts`, `plugin_id`) are immutable through the Router enrichment pass. If a config-tag key collides with a producer field key, the producer field wins.

## Invariants

1. Router is the single dispatch gate; no call sites bypass RoutingFilter evaluation.
2. effectful-shell: Router orchestrates; pure filter matching is delegated to `crates/sink-core/src/router_filter.rs`.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Sink has no RoutingFilter (None) | All events pass through to that sink |
| EC-002 | Event passes filter but sink has empty tags | Event submitted unenriched |
| EC-003 | Config tag key collides with producer field | Producer field wins; config tag is not written |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Sink-A(allow=['commit.made']); submit 'plugin.invoked' | submit() NOT called | happy-path |
| Sink-A(allow=['commit.made']); submit 'commit.made' | submit() called with event | happy-path |
| Sink(tags={env:prod, type:INJECTED}); submit event(type=commit.made) | event arrives with env=prod AND type=commit.made (producer wins) | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-028 | Fan-out: every accepted sink receives the event | integration |
| VP-031 | Tag enrichment does not overwrite producer fields | unit-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-003: "Stream observability events to multiple configurable sinks" per capabilities.md §CAP-003 |
| Architecture Module | SS-03 — crates/factory-dispatcher/src/sinks/router.rs |
| Stories | S-4.06 |

## Source

Greenfield BC created in S-4.06 per story's "BCs to Create" section. Deprecates BC-3.04.001.

**Pinned test:** `crates/factory-dispatcher/src/sinks/router.rs::tests::test_BC_3_04_004_routing_filter_applied_in_dispatch_path`
**Test type:** unit
