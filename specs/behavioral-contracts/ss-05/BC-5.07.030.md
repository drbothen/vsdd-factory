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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:727
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

# Behavioral Contract BC-5.07.030: implementer: micro-commit per passing test, squash before PR

## Description

During the TDD loop, the implementer commits after each test goes green with
`wip(STORY-NNN): test_X passes`. Before pushing the PR, it squashes wip commits
into a clean conventional commit. Limits crash loss to ~5 min.

## Preconditions

1. implementer in TDD loop.

## Postconditions

1. Pre-rebase reflog shows multiple `wip(STORY-NNN):` commits.
2. Final pushed history shows one clean `feat(STORY-NNN):` commit per story.

## Invariants

1. Wip commits are local; squashed before push.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Crash mid-loop | Resume from last wip commit (≤5 min loss) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| TDD loop with wip commits + final squash | Accepted | happy-path |
| Single bulk commit | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Pushed history has one feat commit per story | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/implementer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.029 — composes with (minimum code per test)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#implementer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/implementer.md:134-156` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Micro-Commit Protocol

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (commits) |
| **Global state access** | reads/writes git state |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
