---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-deep-agents.md]
input-hash: "595f07d"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1297
subsystem: SS-05
capability: CAP-TBD
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

# Behavioral Contract BC-5.10.001: state-manager: git access scoped to .factory/ only

## Description

State-manager's exec scope is git operations inside `.factory/` only. Git on
other paths is forbidden; non-git shell commands (cargo, npm, curl) are
forbidden entirely.

## Preconditions

1. state-manager dispatched.

## Postconditions

1. Tool-call audit shows exec calls limited to `cd .factory && git ...` patterns.
2. Zero exec calls in source-code branches or non-git invocations.

## Invariants

1. Shell scope is git + .factory only.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Need to run `npm install` in .factory | Forbidden (non-git) |
| EC-002 | Need to commit on develop branch | Forbidden (out of scope) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `cd .factory && git commit -m "..."` | Accepted | happy-path |
| `cargo build` | Rejected | error |
| `cd src && git commit` | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Tool-call audit confirms scope limit | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/state-manager.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.10.002 — composes with (no spec/source writes)
- BC-5.10.004 — composes with (worktree preconditions)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#state-manager`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/state-manager.md:351-355` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit shell scope rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | shell (git in .factory) |
| **Global state access** | git state |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
