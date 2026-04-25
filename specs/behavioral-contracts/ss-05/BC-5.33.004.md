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
extracted_from: "plugins/vsdd-factory/workflows/maintenance.lobster"
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

# Behavioral Contract BC-5.33.004: maintenance: DAG integrity

## Description

`maintenance.lobster` defines 33 acyclic steps. 11 parallel sweeps all root at `state-init`, each with its own state-backup. `maintenance-report` fan-in from all 11 sweep outputs. `fix-pr-delivery` loop conditional on `auto_fixable_findings`. Final: notifications + state-final + maintenance-gate + session-review chain.

## Preconditions

1. Workflow file is parsed by lobster loader.

## Postconditions

1. Topological sort succeeds.
2. 11 parallel sweep streams structurally preserved.

## Invariants

1. No cycles.
2. maintenance-report depends on 11 sweep outputs.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | One sweep skipped | DAG remains acyclic; report runs on remaining 10 |
| EC-002 | All sweeps skipped | Report still runs (with empty inputs) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Full | All 33 steps participate | happy-path |
| Subset enabled | DAG acyclic with skips | edge-case |
| Cycle hypothetical | Loader rejects | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | DAG acyclic | manual / lobster-parse |
| VP-002 | maintenance-report has 11 in-edges | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.33.001 — identity

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#maintenance-workflow`

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
| **Path** | `plugins/vsdd-factory/workflows/maintenance.lobster` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation

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
