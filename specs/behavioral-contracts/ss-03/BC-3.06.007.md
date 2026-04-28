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

# Behavioral Contract BC-3.06.007: sink-core::routing_filter_plugin_ids_allow — only events from listed plugins pass; empty list = pass-through

> Section: sink-core routing filter (S-4.06)
> Traces to: CAP-003 per capabilities.md §CAP-003

## Description

`RoutingFilter.plugin_ids_allow` is a per-sink filter field. When non-empty, only events whose `plugin_id` field matches one of the listed IDs pass. An empty list means all events pass regardless of `plugin_id`. Matching is case-sensitive. An event missing the `plugin_id` field is treated as no-match when `plugin_ids_allow` is non-empty.

## Preconditions

1. A RoutingFilter is configured with a non-empty `plugin_ids_allow` list.
2. An event is submitted with or without a `plugin_id` field.

## Postconditions

1. If `plugin_ids_allow` is non-empty and the event's `plugin_id` is in the list, the event passes.
2. If `plugin_ids_allow` is non-empty and the event's `plugin_id` is NOT in the list (or `plugin_id` is absent), the event is filtered out.
3. If `plugin_ids_allow` is empty, all events pass regardless of `plugin_id`.

## Invariants

1. `plugin_ids_allow` filtering is independent of `event_types_allow`/`event_types_deny`; all configured filter fields must pass simultaneously for the event to be forwarded (AND semantics).
2. Plugin ID matching is case-sensitive (parallels BC-3.06.006).
3. No wildcard semantics; empty list is the only pass-through escape hatch.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Event has no `plugin_id` field and `plugin_ids_allow` is non-empty | Event filtered out (treated as no match) |
| EC-002 | `plugin_ids_allow` is empty | All events pass regardless of plugin_id |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| plugin_ids_allow=['my-plugin']; event(plugin_id='my-plugin') | passes | happy-path |
| plugin_ids_allow=['my-plugin']; event(plugin_id='other-plugin') | filtered | happy-path |
| plugin_ids_allow=[]; event(plugin_id='any') | passes | edge-case (empty list = passthrough) |
| plugin_ids_allow=['my-plugin']; event has no plugin_id | filtered | edge-case (EC-001) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-032 | RoutingFilter correctness (includes plugin_ids_allow) | unit-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-003: "Stream observability events to multiple configurable sinks" per capabilities.md §CAP-003 |
| Architecture Module | SS-03 — crates/sink-core/src/router_filter.rs |
| Stories | S-4.06 |

## Source

Greenfield BC created in S-4.06 per story's "BCs to Create" section. v1.0 scope confirmed.

**Pinned test:** `crates/sink-core/src/router_filter.rs::tests::routing_filter_plugin_ids_allow_passes_listed_plugin`
**Test type:** unit
