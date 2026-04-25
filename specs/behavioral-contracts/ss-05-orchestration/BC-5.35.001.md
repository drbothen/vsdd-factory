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

# Behavioral Contract BC-5.35.001: adaptive-planning: identity

## Description

`planning.lobster` v2.0.0 — adaptive front-end for the VSDD pipeline. Detects existing artifacts, validates quality, identifies gaps, and routes to the correct entry point. Supports Collaborative Discovery (from ideas, L0) and Spec Intake (from existing artifacts, L1-L4). DF-029: 4-level hierarchy detection (BC-S.SS.NNN), sharded architecture handling, environment setup as first step.

## Preconditions

1. Workflow invoked from a project root that may or may not have existing artifacts.

## Postconditions

1. Artifacts are detected and routing decision made (L0 brainstorm/research vs L1-L4 spec intake).
2. Pipeline is started via greenfield.lobster sub-workflow.

## Invariants

1. Workflow version is `v2.0.0`.
2. 4 routing levels supported: L0, L1, L2, L3, L4.
3. environment-setup is the very first step.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Greenfield (no artifacts) | L0 path taken |
| EC-002 | Existing brief only | L1 intake |
| EC-003 | Existing PRD | L2/L3 intake |
| EC-004 | Implementation-ready spec | L4 path |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Empty project | L0 brainstorm | happy-path |
| L4 spec | Implementation-readiness | edge-case |
| Inconsistent artifacts | Validation errors | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Routing always selects exactly one path | manual |
| VP-002 | environment-setup runs before any artifact detection | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 Pipeline Orchestration |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.35.002 — entry-point
- BC-5.35.003 — terminal-step
- BC-5.35.004 — DAG integrity
- BC-5.35.005 — failure semantics

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
| **Path** | `plugins/vsdd-factory/workflows/planning.lobster` (lines 1-25) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: workflow header

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (artifact detection) |
| **Global state access** | reads filesystem |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
