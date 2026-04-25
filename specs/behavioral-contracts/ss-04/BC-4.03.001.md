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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:631"
subsystem: "SS-04"
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

# Behavioral Contract BC-4.03.001: hook-plugins::capture-commit-activity::on_hook_returns_zero_in_stub — stub on_hook returns 0 (pre-S-3.1 placeholder)

## Description

The pre-S-3.1 `capture-commit-activity` crate is a stub. Its `on_hook()` returns 0 to keep the workspace member layout and `wasm32-wasip1` build path alive until S-3.1 supplies the real binding. Test `on_hook_returns_zero_in_stub` documents this stub behavior. S-3.1 will replace this BC.

## Preconditions

1. Pre-S-3.1 stub crate is built.

## Postconditions

1. `on_hook()` returns 0.
2. The crate compiles and the workspace's wasm32-wasip1 build path remains operational.

## Invariants

1. Stub's behavior is exactly "return 0"; no side effects.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `on_hook()` invoked | 0 | happy-path |
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
| Architecture Module | SS-04 — `crates/hook-plugins/capture-commit-activity/src/lib.rs` (stub) |
| Stories | S-3.1 (will replace this BC); re-anchor in Phase 1.8 |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-plugins/capture-commit-activity/src/lib.rs::tests::on_hook_returns_zero_in_stub` (lines 16–19) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `631` |

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
