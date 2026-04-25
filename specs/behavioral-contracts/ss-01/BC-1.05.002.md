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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:174"
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

# Behavioral Contract BC-1.05.002: exec_subprocess denies binaries not on allow-list

## Description

When `exec_subprocess` is called with a command whose basename is not in the entry's `binary_allow`, it returns CAPABILITY_DENIED and emits a denial event with `reason = "binary_not_on_allow_list"` and the offending command.

## Preconditions

1. Plugin's exec_subprocess capability is declared.
2. The cmd's basename is not in `binary_allow`.

## Postconditions

1. Returns CAPABILITY_DENIED.
2. Emits denial event with `reason = "binary_not_on_allow_list"`, `command = <cmd>`.

## Invariants

1. Allow-list is enforced by basename, never by full path.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Allow-list contains "git" and cmd is "/usr/bin/git" | Allowed (basename match) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| binary_allow=["git"], cmd="curl" | DENIED, reason="binary_not_on_allow_list" | error |
| binary_allow=["git"], cmd="/usr/bin/git" | Allowed | happy-path |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/host/exec_subprocess.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `host/exec_subprocess.rs::binary_allowed` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `174` |

#### Evidence Types Used

- guard clause (binary_allowed function)

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
