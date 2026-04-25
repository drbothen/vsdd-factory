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

# Behavioral Contract BC-5.31.001: per-story-delivery: identity

## Description

`code-delivery.lobster` v2.0.0 — reusable per-story delivery sub-workflow invoked by greenfield, feature, maintenance, and multi-repo. Encapsulates worktree → stubs → Red Gate → implement → micro-commits → demo → squash → PR → AI review → security review → converge → merge → cleanup. DF-037 adds Storybook component-test self-healing loop (max 10 iterations) and per-story UI quality gate (D16). Defines 5 typed inputs: story_id, worktree_path, feature_type, module_criticality, implementation_strategy.

## Preconditions

1. Caller workflow (greenfield/feature/maintenance/multi-repo) has populated all 5 typed inputs.
2. Story has been decomposed and is ready for implementation.

## Postconditions

1. Sub-workflow is callable and obeys the documented input contract.
2. Workflow version `v2.0.0` is recorded in invocation lineage.

## Invariants

1. The 5 typed inputs (story_id, worktree_path, feature_type, module_criticality, implementation_strategy) are mandatory and not defaulted by the sub-workflow.
2. DF-037 storybook self-healing iteration cap is 10 across all invocations.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Caller omits required input | Sub-workflow fails at type-validation before any step runs |
| EC-002 | feature_type not in {ui, full-stack, backend, ...} | Conditional steps (storybook, e2e, ui-gate) skip cleanly |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| All 5 inputs valid, feature_type=ui | Full pipeline including storybook chain executes | happy-path |
| feature_type=backend | Storybook + ui-gate steps skipped | edge-case |
| Missing story_id | Type-validation error before create-worktree | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Sub-workflow rejects invocation when any of the 5 typed inputs is missing | manual |
| VP-002 | DF-037 storybook self-healing loop terminates at ≤10 iterations | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-05 Pipeline Orchestration |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.002 — entry-point composes with this identity contract
- BC-5.31.003 — terminal-step composes with this identity contract
- BC-5.31.004 — DAG integrity composes with this identity contract

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#code-delivery-workflow` — sub-workflow class

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)
- VP-002 — bounded loop termination

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

Source line(s): 1-26.

#### Evidence Types Used

- documentation: workflow header declares version, invokers, and 5 typed inputs.

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads only (workflow header) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed; declarative workflow header.
