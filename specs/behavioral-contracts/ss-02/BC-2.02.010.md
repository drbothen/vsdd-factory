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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:721"
subsystem: "SS-02"
capability: "CAP-009"
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

# Behavioral Contract BC-2.02.010: hook-sdk::host::log_levels_are_stable — LogLevel discriminants 0..=4 are pinned (Trace=0, Debug=1, Info=2, Warn=3, Error=4)

## Description

The `LogLevel` enum's discriminants are exactly 0..=4 in the order Trace / Debug / Info / Warn / Error. This is a cross-crate stability invariant; the dispatcher's `level_to_str` (BC-1.05.020) maps the same range. A future renumbering is a major-version event.

## Preconditions

1. The `LogLevel` enum is in scope.

## Postconditions

1. `Trace as u32 == 0`.
2. `Debug as u32 == 1`.
3. `Info as u32 == 2`.
4. `Warn as u32 == 3`.
5. `Error as u32 == 4`.

## Invariants

1. Discriminants are wire-stable; the dispatcher must agree on the same numeric mapping.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `[Trace, Debug, Info, Warn, Error] as u32` | `[0,1,2,3,4]` | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-02 — `crates/hook-sdk/src/host.rs` (LogLevel enum) |
| Stories | S-1.03 |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-sdk/src/host.rs::tests::log_levels_are_stable` (lines 367–374) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `721` |

#### Evidence Types Used

- assertion (unit test)
- type constraint (enum discriminants)

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
