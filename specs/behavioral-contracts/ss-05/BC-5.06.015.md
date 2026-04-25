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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1375
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

# Behavioral Contract BC-5.06.015: story-writer: dependency graph must be acyclic

## Description

Story `depends_on` and `blocks` fields MUST form an acyclic directed graph.
Topological sort MUST succeed on `dependency-graph.md`.

## Preconditions

1. story-writer setting story dependencies.

## Postconditions

1. Topological sort of stories from STORY-INDEX produces a valid linear ordering.
2. No cycles detected.

## Invariants

1. Dependency graph is a DAG.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Cycle introduced via `blocks` | Reject; show cycle |
| EC-002 | Disconnected components | Acceptable (each component is its own DAG) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Linear story dependencies | Topological sort succeeds | happy-path |
| Cyclic dependencies | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Topological sort of dependency-graph.md succeeds | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/story-writer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.06.011 — composes with (one-file-per-story)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#story-writer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/story-writer.md:54, 224` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit acyclic dependency rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads/writes |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (DAG check) |

#### Refactoring Notes

No refactoring needed.
