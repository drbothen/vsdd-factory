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

# Behavioral Contract BC-5.31.023: code-delivery:brownfield-codeowners-check

## Description

Step `brownfield-codeowners-check` (line 354). Type: agent. Agent: consistency-validator. Depends: `[create-pr]`. Condition: `mode == 'brownfield'`. Source 354-362.

## Preconditions

1. mode is `brownfield`.
2. PR is open and codeowners file present in repo.

## Postconditions

1. Codeowners coverage verified for all changed paths.

## Invariants

1. Skipped when mode is not brownfield.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Missing codeowner mapping | Step fails |
| EC-002 | Greenfield mode | Skipped |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Full coverage | Pass | happy-path |
| Gap | Fail | error |
| Greenfield | Skipped | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | All changed paths matched to ≥1 codeowner | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.018 — create-pr

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#code-delivery-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-001

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 354-362) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- guard clause: mode condition
- documentation

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (codeowners file + diff) |
| **Global state access** | reads filesystem |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (decision) |

#### Refactoring Notes

No refactoring needed.
