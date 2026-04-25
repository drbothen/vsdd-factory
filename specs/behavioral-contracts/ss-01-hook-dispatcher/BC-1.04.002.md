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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:154"
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

# Behavioral Contract BC-1.04.002: Epoch ticker advances epoch every 10ms; cooperative shutdown

## Description

A background OS thread named `vsdd-epoch-ticker` increments the engine's epoch on every `EPOCH_TICK_MS = 10` interval. `Drop` joins the thread cleanly. `shutdown()` is idempotent.

## Preconditions

1. Engine + ticker built.

## Postconditions

1. Engine epoch advances at ~10 ms intervals.
2. `Drop` joins the ticker thread cleanly.
3. `shutdown()` may be called any number of times safely (idempotent).

## Invariants

1. Ticker shutdown is total and idempotent.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | shutdown called twice | No panic; second call is no-op |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Run engine with ticker for 30 ms | Epoch >= 3 | happy-path |
| Call shutdown twice | Both calls succeed; no panic | edge-case |
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
| **Path** | `engine.rs::tests::{ticker_advances_epoch_over_time, ticker_shutdown_is_idempotent}` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `154` |

#### Evidence Types Used

- assertion (2 unit tests)

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
