---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-deep-rust-tests.md]
input-hash: "[pending-recompute]"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:497"
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

# Behavioral Contract BC-1.03.013: factory-dispatcher::executor (integration)::crash_does_not_affect_siblings — one Crashed plugin doesn't break sibling Ok plugins; final exit_code stays 0

## Description

Integration test: 3 plugins at same priority — `ok-a` (normal), `crash` (unreachable instruction), `ok-b` (normal). All 3 results are present after `execute_tiers`; `ok-a` and `ok-b` are `Ok`, `crash` is `Crashed`; final `exit_code == 0` (with default `on_error=continue`, a crash does not block).

## Preconditions

1. 3 plugins at same priority — two normal, one with `unreachable`.
2. Default `on_error = continue`.

## Postconditions

1. All 3 results present.
2. `ok-a`, `ok-b` → `Ok`.
3. `crash` → `Crashed`.
4. `summary.exit_code == 0`.

## Invariants

1. Plugin crashes are isolated; siblings continue (BC-AUDIT-016 isolation invariant pinned with a real test).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| 3 plugins, middle one crashes | All 3 reported; siblings Ok; exit_code=0 | happy-path |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/tests/executor_integration.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/factory-dispatcher/tests/executor_integration.rs::crash_does_not_affect_siblings` (lines 120–159) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `497` |

#### Evidence Types Used

- assertion (integration test)

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
