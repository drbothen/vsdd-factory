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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:398"
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

# Behavioral Contract BC-1.05.026: factory-dispatcher::host::exec_subprocess::allows_shell_with_acknowledgment — shell_bypass_acknowledged set → bash allowed (gate passes; spawn may still fail with INTERNAL_ERROR on bashless hosts)

## Description

When the capability allows `bash` AND `shell_bypass_acknowledged` is set to a non-empty acknowledgement (e.g., `"needed for git status parsing"`), the policy gate passes for command `bash -c 'exit 0'`. On bash-less hosts, the spawn may still fail with INTERNAL_ERROR — both outcomes prove the policy gate did not deny.

## Preconditions

1. Capability allows `bash`.
2. `shell_bypass_acknowledged` is set to a non-empty string.

## Postconditions

1. `result.is_ok() OR result == Err(INTERNAL_ERROR)`. INTERNAL_ERROR confirms the policy gate passed and only the spawn failed.

## Invariants

1. Acknowledged shell capability unblocks shell execution unambiguously.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | bash-less host | INTERNAL_ERROR (policy passed; spawn failed) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `bash -c 'exit 0'` with ack set | `Ok(_)` or `Err(INTERNAL_ERROR)` | happy-path |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/host/exec_subprocess.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/factory-dispatcher/src/host/exec_subprocess.rs::tests::allows_shell_with_acknowledgment` (lines 346–366) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `398` |

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
