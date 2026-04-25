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

# Behavioral Contract BC-5.33.001: maintenance-sweep: identity

## Description

`maintenance.lobster` v2.0.0 — background quality sweeps on schedule. 11 sweep types: dependency vulnerabilities, doc drift, pattern inconsistencies, stale holdouts, performance regressions, DTU clone fidelity drift, spec coherence, overdue tech debt, accessibility, design drift, risk/assumption monitoring. Opens fix PRs through `code-delivery.lobster`.

## Preconditions

1. Maintenance schedule trigger fires.
2. Maintenance config loadable.

## Postconditions

1. All 11 sweep types executed (subject to per-sweep conditions).
2. Auto-fixable findings routed to code-delivery for PR generation.

## Invariants

1. Exactly 11 sweep types defined.
2. Maintenance findings are advisory; fix PRs go through standard delivery.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | One sweep disabled | Skipped without breaking workflow |
| EC-002 | Auto-fixable finding | Routed to code-delivery |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Standard run | All sweeps run | happy-path |
| Sweep disabled | Skipped | edge-case |
| Auto-fixable found | Fix PR opened | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | 11 sweep types defined | manual |
| VP-002 | Fix PRs use code-delivery sub-workflow | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 Pipeline Orchestration |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.33.002 — entry-point
- BC-5.33.003 — terminal-step
- BC-5.33.004 — DAG integrity

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#maintenance-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)
- VP-002

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/maintenance.lobster` (lines 1-21) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: workflow header

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes |
| **Global state access** | filesystem |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
