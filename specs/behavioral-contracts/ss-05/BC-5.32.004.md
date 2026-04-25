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
extracted_from: "plugins/vsdd-factory/workflows/discovery.lobster"
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

# Behavioral Contract BC-5.32.004: discovery: DAG integrity

## Description

`discovery.lobster` defines 29 acyclic steps. Multiple parallel ingestion streams (feature-research, customer-feedback-ingestion, competitive-monitoring, usage-analytics) all rooted at `state-init`, fanning into `intelligence-synthesis`. Three Delphi scoring agents (value, feasibility, novelty) run in parallel after synthesis, fan into `feature-debate`. Then dedup → report → notifications + review → routing → conditional sub-workflow execution.

## Preconditions

1. Workflow file is parsed by lobster loader.

## Postconditions

1. Topological sort succeeds.
2. Parallel ingestion structure is preserved.

## Invariants

1. No cycle exists.
2. Three Delphi scorers run in parallel (no in-edge between them).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | One ingestion stream disabled | DAG still acyclic |
| EC-002 | All ingestion streams disabled | Synthesis input set is empty; downstream behavior per spec |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Full config | All 29 steps participate | happy-path |
| Some streams off | Skipped without breaking DAG | edge-case |
| Hypothetical cycle | Loader rejects | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | DAG acyclic | manual / lobster-parse |
| VP-002 | Delphi scorers parallel | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.32.001 — identity

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#discovery-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-001
- VP-002

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/discovery.lobster` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: declarative workflow

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none (structural) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | N/A |
| **Overall classification** | pure |

#### Refactoring Notes

No refactoring needed.
