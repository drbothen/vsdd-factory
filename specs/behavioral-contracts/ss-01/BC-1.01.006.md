---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts.md]
input-hash: "[pending-recompute]"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:60"
subsystem: "SS-01"
capability: "CAP-TBD"
lifecycle_status: active
introduced: v1.0.0-beta.4
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-1.01.006: Tiers ordered ascending by priority, registry order preserved within tier

## Description

`group_by_priority` returns a `Vec<Vec<...>>` whose outer order is ascending priority and whose inner order preserves the original registry entry index. Same-priority entries pack into a single tier.

## Preconditions

1. Matched entries with mixed priorities are provided as input.

## Postconditions

1. Outer vector is sorted ascending by priority.
2. Inner vector ordering equals the original registry entry order.
3. Multiple entries at the same priority share one inner tier.

## Invariants

1. Tier ordering is stable, deterministic, and pure.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Empty input | Empty `Vec<Vec<>>` |
| EC-002 | All same priority | Single tier with all entries in registry order |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Entries at priority [100, 10, 100] (registry order) | `[[priority=10], [priority=100 (idx 0), priority=100 (idx 2)]]` | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/routing.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `routing.rs::tests::{group_orders_tiers_ascending, group_keeps_registry_order_within_tier, group_packs_multiple_entries_at_same_priority}` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `60` |

#### Evidence Types Used

- assertion (3 unit tests)

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | TBD (Phase 1.6b will refine) |
| **Global state access** | TBD |
| **Deterministic** | TBD |
| **Thread safety** | TBD |
| **Overall classification** | TBD |

#### Refactoring Notes

(TBD — to be assessed in Phase 1.6b verification properties pass)
