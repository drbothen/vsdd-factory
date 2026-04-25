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
extracted_from: "plugins/vsdd-factory/workflows/planning.lobster"
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

# Behavioral Contract BC-5.35.004: planning: DAG integrity

## Description

`planning.lobster` defines 25 acyclic steps. Two parallel routing tracks fan from `market-intel-review` (L0 collaborative discovery vs L1-L4 spec intake), both converging at `start-pipeline` sub-workflow `greenfield.lobster`. `environment-gate` is blocking.

## Preconditions

1. Workflow file is parsed by lobster loader.

## Postconditions

1. Topological sort succeeds; both routing tracks merge at start-pipeline.

## Invariants

1. environment-gate is on the critical path and blocks downstream on failure.
2. start-pipeline always invokes `greenfield.lobster` regardless of routing track.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | L0 path | brainstorming/research → guided-brief-creation → brief-validation → brief-approval |
| EC-002 | L1-L4 path | validate-existing-* → intake-approval |
| EC-003 | Cycle hypothetical | Loader rejects |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| L0 routing | L0 path taken | happy-path |
| L4 routing | L4 path taken | edge-case |
| Cycle | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | DAG acyclic | manual / lobster-parse |
| VP-002 | Both routing tracks merge at start-pipeline | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.35.001 — identity

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#planning-workflow`

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
| **Path** | `plugins/vsdd-factory/workflows/planning.lobster` |
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
