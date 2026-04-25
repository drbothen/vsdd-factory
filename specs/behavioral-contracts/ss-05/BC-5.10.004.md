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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1321
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

# Behavioral Contract BC-5.10.004: state-manager: worktree preconditions verified before any .factory/ creation

## Description

Before initializing or writing to `.factory/` or `.factory-project/`, the
state-manager MUST verify all three worktree preconditions: (1) `.factory/.git`
exists, (2) `git -C .factory rev-parse --git-dir` succeeds, (3) `git -C .factory
branch --show-current` shows `factory-artifacts`. Any failure halts with the
exact recovery command. Creating `.factory/` as a regular directory is forbidden.

## Preconditions

1. state-manager about to write to `.factory/` or `.factory-project/`.

## Postconditions

1. State-manager invocation log includes precondition checks before any Write/Edit.
2. Failure paths emit `ERROR: .factory/ is not mounted as a git worktree on
   factory-artifacts branch.` with recovery command.

## Invariants

1. Worktree preconditions are mandatory.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | .factory/ exists but not as worktree | Halt with recovery instructions |
| EC-002 | .factory/ on wrong branch | Halt |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| All 3 checks pass | Proceed with writes | happy-path |
| `.factory/.git` missing | Halt with error message | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Precondition checks logged before any .factory/ write | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/state-manager.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.08.008 — composes with (devops-engineer .factory worktree)
- BC-5.02.013 — composes with (orchestrator factory-worktree-health on resume)

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
| **Path** | `plugins/vsdd-factory/agents/state-manager.md:41-72` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Preconditions section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (git state) |
| **Global state access** | reads worktree state |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
