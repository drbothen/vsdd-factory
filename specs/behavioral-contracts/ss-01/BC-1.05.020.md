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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:332"
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

# Behavioral Contract BC-1.05.020: factory-dispatcher::host::log::level_mapping_matches_sdk — level u32 0..=4 maps to {trace,debug,info,warn,error}

## Description

`level_to_str(level)` for `level` u32 in 0..=4 returns "trace", "debug", "info", "warn", "error" respectively. The mapping is paired with `vsdd_hook_sdk::LogLevel` (BC-2.02.010 covers SDK side). Cross-crate enum stays in lock-step.

## Preconditions

1. `level_to_str(level)` invoked with `level` in 0..=4.

## Postconditions

1. Returns "trace" for 0, "debug" for 1, "info" for 2, "warn" for 3, "error" for 4.

## Invariants

1. Cross-crate level mapping is byte-stable.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | level > 4 | Falls through to "info" per BC-1.05.011 |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `level_to_str(2)` | "info" | happy-path |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/host/log.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/factory-dispatcher/src/host/log.rs::tests::level_mapping_matches_sdk` (lines 58–65) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `332` |

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
