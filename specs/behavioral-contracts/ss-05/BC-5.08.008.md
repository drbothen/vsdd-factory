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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:453
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

# Behavioral Contract BC-5.08.008: devops-engineer: .factory mounted as git worktree on factory-artifacts orphan branch

## Description

Instead of gitignoring `.factory/`, the agent creates an orphan branch
`factory-artifacts` and mounts `.factory/` as a git worktree on it.
Pattern: `git checkout --orphan factory-artifacts && git rm -rf . && git commit
--allow-empty -m "..." && git push origin factory-artifacts && git worktree add
.factory factory-artifacts`. Recovery from disk failure: `git clone ... && git
worktree add .factory factory-artifacts`.

## Preconditions

1. devops-engineer initializing the .factory worktree.

## Postconditions

1. `.factory/.git` exists as a worktree marker.
2. `git -C .factory branch --show-current` returns `factory-artifacts`.

## Invariants

1. .factory is on a git worktree, not gitignored.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | .factory exists as plain dir | Halt with conversion instructions |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| .factory mounted on factory-artifacts | Accepted | happy-path |
| .factory as plain dir | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | git -C .factory branch shows factory-artifacts | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/devops-engineer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.10.004 — composes with (state-manager worktree preconditions)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#devops-engineer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/devops-engineer.md:87-104` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Artifact Backup section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (git ops) |
| **Global state access** | reads/writes git state |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
