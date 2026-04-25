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

# Behavioral Contract BC-5.31.004: code-delivery: DAG integrity

## Description

`code-delivery.lobster` defines 23 top-level steps that form an acyclic dependency graph. Linear backbone: create-worktree → generate-stubs → write-tests → red-gate → implement → per-story-adversarial-review → e2e-tests + storybook chain (UI) → per-story-ui-quality-gate → demo-recording → squash-and-push → create-pr → ai-pr-review + security-review (parallel) → pr-review-convergence → brownfield-full-regression + brownfield-codeowners-check (mode=brownfield) → wait-for-ci → dependency-merge-check → merge-pr → delivery-human-approval (conditional) → cleanup-worktree.

## Preconditions

1. Workflow file is loaded by lobster parser.
2. All step IDs are unique within the workflow.

## Postconditions

1. Topological sort succeeds (no cycles).
2. `create-worktree` is the unique root, `cleanup-worktree` is the unique terminal.

## Invariants

1. No step depends on itself transitively.
2. Every dependency name resolves to a defined step.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | UI feature_type | storybook chain + ui-quality-gate run |
| EC-002 | Non-UI feature_type | UI conditional steps skipped without breaking DAG |
| EC-003 | Brownfield mode | brownfield-full-regression + codeowners-check inserted |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| All 23 steps | Topological order computed | happy-path |
| Conditional UI off | DAG still acyclic with skipped nodes | edge-case |
| Hypothetical cycle introduced | Loader rejects workflow | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | DAG is acyclic | manual / lobster-parse |
| VP-002 | All `depends_on` references resolve | manual / lobster-parse |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 Pipeline Orchestration |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.001 — identity
- BC-5.31.002 — entry-point
- BC-5.31.003 — terminal-step
- BC-5.31.005 — failure semantics

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#code-delivery-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-001 — DAG acyclicity
- VP-002 — dependency resolution

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: declarative workflow definition lines 38-437

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none (structural property) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | N/A |
| **Overall classification** | pure |

#### Refactoring Notes

No refactoring needed; declarative.
