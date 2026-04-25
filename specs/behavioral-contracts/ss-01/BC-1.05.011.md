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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:228"
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

# Behavioral Contract BC-1.05.011: log host fn emits `plugin.log` internal event with level mapped to {trace,debug,info,warn,error}

## Description

When a plugin calls `vsdd::log(level, msg)` with `level` in 0..=4, the dispatcher emits an internal event `plugin.log` with the mapped level string and the message. Levels >4 default to `"info"`.

## Preconditions

1. Plugin calls `vsdd::log(level, msg)`.

## Postconditions

1. An internal event `plugin.log` is written.
2. `level` (u32) maps to one of `{"trace","debug","info","warn","error"}`.
3. `level > 4` falls back to `"info"`.
4. Event includes the `message`.

## Invariants

1. Level mapping is total over u32 with defined fallback.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | level = 5 | "info" |
| EC-002 | level = 0 | "trace" |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `log(2, "hello")` | `plugin.log` event with level="info", message="hello" | happy-path |
| `log(99, "x")` | `plugin.log` event with level="info" (fallback) | edge-case |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/invoke.rs` (log arm + level mapping) |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `invoke.rs` log arm + level mapping switch |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `228` |

#### Evidence Types Used

- assertion (level mapping switch)

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
