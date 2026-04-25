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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:216"
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

# Behavioral Contract BC-1.05.009: read_file at the StoreData-typed linker layer is currently a CAPABILITY_DENIED stub

## Description

When a plugin calls `vsdd::read_file` through the per-invocation StoreData-typed linker, the call returns CAPABILITY_DENIED unconditionally. The full `read_file` implementation exists in `host/read_file.rs` but is not yet wired to the StoreData-typed linker. (DRIFT — flagged in Pass 6.) No in-tree plugin currently exercises the path.

## Preconditions

1. Plugin calls `vsdd::read_file`.
2. The call is dispatched through the StoreData-typed linker.

## Postconditions

1. Returns CAPABILITY_DENIED unconditionally.

## Invariants

1. Until S-1.x integration wires the prepare fn, read_file at the StoreData layer is hard-denied.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Plugin invokes read_file via StoreData linker | CAPABILITY_DENIED | error |
| TBD | TBD | happy-path |
| TBD | TBD | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/invoke.rs:447–474` (stub); `host/read_file.rs` (full impl, not yet wired) |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `invoke.rs:447–474` (registers stub returning CAPABILITY_DENIED) |
| **Confidence** | HIGH (drift flag — see Pass 6) |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `216` |

#### Evidence Types Used

- assertion (stub registration)

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
