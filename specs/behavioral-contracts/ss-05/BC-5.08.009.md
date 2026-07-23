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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:461
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

# Behavioral Contract BC-5.08.009: devops-engineer: worktree-per-story discipline (.worktrees/STORY-NNN)

## Description

For each story in a wave, the devops-engineer creates `.worktrees/STORY-NNN` on
branch `feature/STORY-NNN`. After PR merges, the worktree is removed via
`git worktree remove`. `.worktrees/` is gitignored.

## Preconditions

1. devops-engineer initializing per-story worktree.

## Postconditions

1. Each active story has a `.worktrees/STORY-NNN` directory on its feature branch.
2. Merged story worktrees are removed within the cleanup step.
3. `.worktrees/` is gitignored.

## Invariants

1. One worktree per active story.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Worktree exists from prior session | Re-use if branch matches; else recreate |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Story STORY-001 worktree present | Accepted | happy-path |
| Worktree leftover post-merge | Rejected (cleanup needed) | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Each active story has worktree; merged stories cleaned up | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/devops-engineer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.08.008 — composes with (.factory worktree)

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
| **Path** | `plugins/vsdd-factory/agents/devops-engineer.md:192-213, 244-249` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Worktree Creation + cleanup sections

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (git worktree) |
| **Global state access** | reads/writes git state |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
