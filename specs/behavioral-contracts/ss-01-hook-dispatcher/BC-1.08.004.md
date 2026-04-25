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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:554"
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

# Behavioral Contract BC-1.08.004: dispatcher uses CLAUDE_PROJECT_DIR for cwd, falling back to current_dir

## Description

`base_host_ctx.cwd` is set to `${CLAUDE_PROJECT_DIR}` if set, else `current_dir()`, else `"."`. This fixes log writes landing in surprising places when the dispatcher is invoked from arbitrary cwds.

## Preconditions

1. Dispatcher invoked.

## Postconditions

1. If `CLAUDE_PROJECT_DIR` env is set → `base_host_ctx.cwd` = `${CLAUDE_PROJECT_DIR}`.
2. Else if `current_dir()` succeeds → that value.
3. Else `"."`.

## Invariants

1. `cwd` always has a defined value (never panics).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | All sources fail | Falls back to "." |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `CLAUDE_PROJECT_DIR=/p` set | cwd=/p | happy-path |
| `CLAUDE_PROJECT_DIR` unset, valid current_dir | cwd=current_dir | edge-case |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/main.rs:137-142` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `main.rs:137–142` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `554` |

#### Evidence Types Used

- assertion (cwd resolution chain)

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
