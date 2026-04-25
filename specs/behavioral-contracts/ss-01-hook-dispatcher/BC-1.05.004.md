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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:186"
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

# Behavioral Contract BC-1.05.004: exec_subprocess refuses setuid/setgid binaries categorically (Unix)

## Description

If the resolved binary path has a setuid or setgid bit set (Unix), `exec_subprocess` returns CAPABILITY_DENIED regardless of the allow-list contents. This is design Q4 resolution.

## Preconditions

1. Resolved binary path has setuid OR setgid bit (Unix).

## Postconditions

1. Returns CAPABILITY_DENIED.
2. Even if the binary is on `binary_allow`, the call fails.

## Invariants

1. Setuid/setgid binaries are never executed via exec_subprocess on Unix.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Windows host | Refuse-setuid check is Unix-only; behavior on Windows is TBD per source |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Setuid binary on allow-list (Unix) | DENIED | error |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/host/exec_subprocess.rs` (`refuse_setuid`, Unix-only) |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `host/exec_subprocess.rs::refuse_setuid` (Unix-only); design Q4 resolution |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `186` |

#### Evidence Types Used

- guard clause (refuse_setuid)

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
