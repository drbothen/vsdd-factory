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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:394"
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

# Behavioral Contract BC-1.07.001: All 30+ existing bash hooks fire via legacy-bash-adapter on Linux/macOS

## Description

For v1.0.0-beta.1 onward, the auto-generated registry routes every bash hook through `legacy-bash-adapter.wasm`. The full bats suite (1245+ tests) passes. CHANGELOG-asserted; regression-pinned by `plugins/vsdd-factory/tests/regression-v1.0.bats` (11 dispatcher-pipeline regression tests).

## Preconditions

1. v1.0.0-beta.1 (or later) release.
2. Auto-generated registry routes every bash hook through the adapter.
3. Linux or macOS host.

## Postconditions

1. Full bats suite (1245+ tests) passes.
2. Each bash hook fires via legacy-bash-adapter.

## Invariants

1. No bash hook is left un-routed; the adapter is the universal current router (per ADR-012).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | A new bash hook added without registry regeneration | check-platforms-drift / generate-registry script catches via CI |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Run full bats suite on Linux/macOS | All 1245+ tests pass | happy-path |
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
| Architecture Module | SS-01 + SS-04 — dispatcher + legacy-bash-adapter; bats suite at `plugins/vsdd-factory/tests/regression-v1.0.bats` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | CHANGELOG v1.0.0-beta.1; `plugins/vsdd-factory/tests/regression-v1.0.bats` |
| **Confidence** | HIGH (CHANGELOG-asserted, regression suite pinned) |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `394` |

#### Evidence Types Used

- documentation (CHANGELOG)
- assertion (regression bats)

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
