---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md]
input-hash: "a022087"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: .factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:288
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

# Behavioral Contract BC-5.01.004: Step ordering is by `depends_on:` topological resolution (NOT array position)

## Description

The orchestrator sequences workflow steps via topological sort over the `depends_on:`
DAG, NOT by their position in the YAML array. Steps with empty `depends_on: []` are
roots and run first; steps that share no transitive dependency MAY run in parallel.

## Preconditions

1. Steps array contains entries with `depends_on: [other-step-name, ...]`.

## Postconditions

1. Orchestrator computes a topological sort of the `depends_on` DAG.
2. Steps execute in topological order — array position is ignored.
3. Steps with empty `depends_on: []` run first (roots).
4. Steps may run in parallel if and only if they share no transitive dependency.

## Invariants

1. The `depends_on` graph is acyclic (cycles are rejected at parse).
2. Every dependency name in `depends_on` references a known step name in the same workflow.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Cyclic `depends_on` chain | Parse error |
| EC-002 | `depends_on` references unknown step name | Parse / validation error |
| EC-003 | Two steps with disjoint dependency closures | May run in parallel |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `greenfield.lobster:40-41` (root: `repo-initialization` with `depends_on: []`) | Runs first | happy-path |
| `greenfield.lobster:55` (`factory-worktree-health depends_on: [repo-initialization]`) | Runs after `repo-initialization` | happy-path |
| `greenfield.lobster:233-235` (gate depends on multiple parallel siblings) | Gate runs after all siblings | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Topological sort of every workflow's step graph terminates without cycle detection | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `bin/lobster-parse`, orchestrator dispatch logic |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.01.001 — composes with (workflow envelope)
- BC-5.01.003 — composes with (step taxonomy)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#topological-resolution`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `greenfield.lobster:40-41, 55, 233-235` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit `depends_on:` annotations across multiple steps

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none (sort algorithm) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure |

#### Refactoring Notes

No refactoring needed — topological sort is a pure algorithm.
