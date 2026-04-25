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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:180"
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

# Behavioral Contract BC-1.05.003: exec_subprocess denies shell interpreters without shell_bypass_acknowledged

## Description

When the cmd basename is one of the SHELL_NAMES set (`bash`, `sh`, `zsh`, `pwsh`, `fish`, `csh`, `tcsh`, `ksh`) AND `shell_bypass_acknowledged` is None, the call returns CAPABILITY_DENIED with `reason = "shell_bypass_not_acknowledged"`. The legacy-bash-adapter sets the ack to the verbatim string `"legacy-bash-adapter runs unported hooks"`.

## Preconditions

1. cmd basename is in SHELL_NAMES.
2. `shell_bypass_acknowledged` is None.

## Postconditions

1. Returns CAPABILITY_DENIED.
2. Denial event has `reason = "shell_bypass_not_acknowledged"`.

## Invariants

1. Shell execution is gated by an explicit acknowledgement string.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | cmd is `bash` and ack is set | Allowed |
| EC-002 | cmd is `git` (non-shell) | Allowed without ack |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| cmd=`bash`, no ack | DENIED with `shell_bypass_not_acknowledged` | error |
| cmd=`bash`, ack="legacy-bash-adapter runs unported hooks" | Allowed | happy-path |
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
| **Path** | `host/exec_subprocess.rs::run` shell check; SHELL_NAMES constant; `legacy-bash-adapter/src/lib.rs:20` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `180` |

#### Evidence Types Used

- guard clause (shell check)
- type constraint (SHELL_NAMES constant)

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
