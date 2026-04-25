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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:508"
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

# Behavioral Contract BC-1.03.014: factory-dispatcher::executor (integration)::parallel_timeout_does_not_cascade — hang plugin times out at 120ms while siblings complete in parallel; wall < 2s for 4-plugin tier

## Description

Integration test: EpochTicker started; 4 plugins at priority=100: `ok-a`, `hanger` (timeout_ms=120, infinite loop), `ok-b`, `ok-c`. After `execute_tiers`, total wall < 2_000 ms, `hanger` is `Timeout`, others are `Ok`. The hang doesn't cascade — concurrent execution is real.

## Preconditions

1. EpochTicker is running.
2. 4 plugins at priority 100; one has infinite loop and timeout_ms=120; others return Ok.

## Postconditions

1. `wall_ms < 2_000`.
2. `hanger`'s result is `Timeout`.
3. All other results are `Ok`.

## Invariants

1. Concurrent execution within a tier is real, not theoretical.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| 4-plugin tier with one hanger | Wall < 2s; hanger Timeout; others Ok | happy-path |
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
| **Path** | `crates/factory-dispatcher/tests/executor_integration.rs::parallel_timeout_does_not_cascade` (lines 161–215) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `508` |

#### Evidence Types Used

- assertion (integration test with timing assertion)

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
