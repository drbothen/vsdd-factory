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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:116"
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

# Behavioral Contract BC-1.03.007: Tier execution preserves between-tier order

## Description

`executor::execute_tiers` awaits each `execute_tier` call sequentially in priority order: tier N completes (all spawn_blocking joined) before tier N+1 begins. `total_elapsed_ms` is measured across all tiers.

## Preconditions

1. The dispatch produced multiple priority tiers.

## Postconditions

1. Tier N's plugins all complete before tier N+1's begin executing.
2. `total_elapsed_ms` covers all tiers.

## Invariants

1. Tiers execute strictly in ascending-priority order.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | A single tier fails | Subsequent tiers still execute (per `on_error` semantics) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Tiers at priority 10 and 100 | Tier 10 plugins complete before tier 100 plugins start | happy-path |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/executor.rs` (`execute_tiers`) |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `executor.rs::execute_tiers` for-loop awaits each `execute_tier`; routing tests |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `116` |

#### Evidence Types Used

- assertion (sequential await loop)

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
