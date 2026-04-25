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
extracted_from: "plugins/vsdd-factory/workflows/multi-repo.lobster"
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

# Behavioral Contract BC-5.34.001: multi-repo-vsdd: identity

## Description

`multi-repo.lobster` v3.0.0 — orchestrates VSDD across multiple repositories per `project.yaml` with cross-repo dependencies and wave ordering. Each repo runs its own greenfield/brownfield/feature pipeline coordinated by cross-repo gates. DF-032: per-repo mode classification, multi-repo brownfield Phase 0 with project-level synthesis, cross-repo information asymmetry walls, cross-repo cost aggregation.

## Preconditions

1. `project.yaml` exists at the project root and is well-formed.
2. Per-repo manifests are reachable via project.yaml.

## Postconditions

1. Multi-repo workflow orchestrates VSDD with wave ordering and cross-repo dependency awareness.

## Invariants

1. Cross-repo information walls are honored consistently (DF-032).
2. Workflow version `v3.0.0`.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Single-repo project.yaml | Workflow still runs but degenerates to single-track |
| EC-002 | Cross-repo dependency cycle | Workflow halts |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Multi-repo project | Wave-ordered execution | happy-path |
| Single repo | Linearized | edge-case |
| Cyclic deps | Halt | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Cross-repo walls applied across all walled steps | manual |
| VP-002 | Wave ordering respects all declared dependencies | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 Pipeline Orchestration |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.34.002 — entry-point
- BC-5.34.003 — terminal-step
- BC-5.34.004 — DAG integrity
- BC-5.34.006 — cross-repo information asymmetry walls

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#multi-repo-workflow`

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
| **Path** | `plugins/vsdd-factory/workflows/multi-repo.lobster` (lines 1-37) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: workflow header

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (per repo) |
| **Global state access** | filesystem |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
