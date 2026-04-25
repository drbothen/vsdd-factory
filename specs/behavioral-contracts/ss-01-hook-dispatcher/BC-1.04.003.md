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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:160"
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

# Behavioral Contract BC-1.04.003: timeout_ms_to_epochs rounds up

## Description

`timeout_ms_to_epochs(timeout_ms)` returns `ceil(timeout_ms / 10)` so that any sub-tick remainder gets at least 1 tick of grace. This avoids spurious zero-epoch deadlines.

## Preconditions

1. Caller supplies a non-negative `timeout_ms`.

## Postconditions

1. Returns `ceil(timeout_ms / 10)`.

## Invariants

1. Sub-tick timeouts always get at least 1 tick of grace.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | timeout_ms = 0 | 0 |
| EC-002 | timeout_ms = 1 | 1 |
| EC-003 | timeout_ms = 10 | 1 |
| EC-004 | timeout_ms = 11 | 2 |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `timeout_ms = 11` | 2 | happy-path |
| `timeout_ms = 1` | 1 | edge-case |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/engine.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `engine.rs::tests::timeout_ms_to_epochs_rounds_up` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `160` |

#### Evidence Types Used

- assertion (unit test)

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
