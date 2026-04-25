---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: phase-1-4b-agent-5
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-deep-workflows.md]
input-hash: "99bbe9c"
traces_to: domain-spec/L2-INDEX.md
origin: brownfield
extracted_from: "plugins/vsdd-factory/workflows/code-delivery.lobster"
subsystem: "SS-05"
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

# Behavioral Contract BC-5.31.002: code-delivery: entry-point

## Description

The single entry-point step of `code-delivery.lobster` is `create-worktree` (line 38) with `depends_on: []`, type `agent`, agent `devops-engineer`, condition `worktree.not_exists == true`.

## Preconditions

1. The sub-workflow has been invoked with valid inputs.
2. No prior step has run for this story.

## Postconditions

1. `create-worktree` is the only step with empty `depends_on` (root of DAG).
2. Step is skipped only when the worktree already exists (idempotent re-entry).

## Invariants

1. Exactly one step has `depends_on: []` and that step is `create-worktree`.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Worktree already exists | Step skips, downstream proceeds |
| EC-002 | Worktree creation fails | Workflow escalates per default failure semantics |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Fresh story, no worktree | create-worktree runs | happy-path |
| Re-entry, worktree exists | create-worktree skipped | edge-case |
| devops-engineer agent unavailable | escalate per defaults | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | DAG has exactly one node with no in-edges | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 Pipeline Orchestration |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.001 — identity (parent)
- BC-5.31.006 — create-worktree per-step BC

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#code-delivery-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-001 — DAG single-root invariant

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (line 38) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- guard clause: `condition: worktree.not_exists == true`
- documentation: declarative workflow definition

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (creates worktree) |
| **Global state access** | reads filesystem |
| **Deterministic** | yes (given filesystem state) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
