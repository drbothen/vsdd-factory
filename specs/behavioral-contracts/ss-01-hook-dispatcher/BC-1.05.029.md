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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:431"
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

# Behavioral Contract BC-1.05.029: factory-dispatcher::host::exec_subprocess::is_shell_detects_interpreters — SHELL_NAMES set is bash, sh, zsh, pwsh + path variants

## Description

`is_shell(cmd)` returns true for `"bash"`, `"/bin/bash"`, `"sh"`, `"zsh"`, `"pwsh"` and false for `"git"`, `"curl"`. Detection is by basename and matches the SHELL_NAMES set. Shell-bypass-acknowledged enforcement (BC-1.05.003) catches every documented interpreter.

## Preconditions

1. `is_shell(cmd)` invoked with various cmd shapes.

## Postconditions

1. Returns true iff cmd's basename is in SHELL_NAMES.

## Invariants

1. Shell detection is by basename.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | cmd `"/bin/bash"` (path variant) | true |
| EC-002 | cmd `"git"` | false |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `is_shell("/bin/bash")` | true | happy-path |
| `is_shell("git")` | false | edge-case |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/host/exec_subprocess.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/factory-dispatcher/src/host/exec_subprocess.rs::tests::is_shell_detects_interpreters` (lines 408–417) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `431` |

#### Evidence Types Used

- assertion (unit test)
- type constraint (SHELL_NAMES set)

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
