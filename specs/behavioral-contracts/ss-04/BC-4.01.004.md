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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:380"
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

# Behavioral Contract BC-4.01.004: legacy-bash-adapter caps combined output at 1 MiB

## Description

The legacy-bash-adapter declares a `MAX_OUTPUT_BYTES = 1024 * 1024` constant that caps the total stdout+stderr volume returned from any bash hook. The host's `exec_subprocess` enforces the cap by returning `OUTPUT_TOO_LARGE` when exceeded.

## Preconditions

1. A bash hook is invoked through the adapter.
2. The bash hook emits more than 1 MiB combined to stdout + stderr.

## Postconditions

1. `MAX_OUTPUT_BYTES = 1024 * 1024` is enforced.
2. The host returns `OUTPUT_TOO_LARGE` (-3) for the call.

## Invariants

1. The adapter never accepts more than 1 MiB of bash output per invocation.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Output exactly 1 MiB | Allowed (cap is the upper bound) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Bash hook emits 2 MiB to stdout | host returns OUTPUT_TOO_LARGE | error |
| TBD | TBD | edge-case |
| TBD | TBD | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-04 — `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` (MAX_OUTPUT_BYTES); cross-cuts SS-01 host enforcement |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` (MAX_OUTPUT_BYTES constant) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `380` |

#### Evidence Types Used

- type constraint (constant declaration)

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
