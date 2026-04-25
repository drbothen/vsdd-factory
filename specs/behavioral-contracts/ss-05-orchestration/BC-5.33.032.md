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

# Behavioral Contract BC-5.33.032: maintenance:fix-pr-delivery

## Description

Step `fix-pr-delivery` (line 311). Type: loop. Depends: `[maintenance-report]`. Condition: `maintenance.has_auto_fixable_findings == true`. Source 311-331. `for_each: finding in auto_fixable_findings` — generate-fix → deliver-fix sub-workflow `code-delivery.lobster`.

## Preconditions

1. maintenance-report identified at least one auto-fixable finding.

## Postconditions

1. For each auto-fixable finding, a fix is generated and delivered via `code-delivery.lobster`.

## Invariants

1. Sub-workflow target is `code-delivery.lobster`.
2. Step skipped when no auto-fixable findings.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | No findings | Skipped |
| EC-002 | Sub-workflow fails | Per fix-step semantics |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| 3 auto-fixable findings | 3 PR deliveries | happy-path |
| 0 findings | Skipped | edge-case |
| Sub-workflow error | Per code-delivery contract | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Sub-workflow target = code-delivery.lobster | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.33.031 — maintenance-report
- BC-5.31.001 — code-delivery identity (sub-workflow target)

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#maintenance-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-001

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/maintenance.lobster` (lines 311-331) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- type constraint: loop with for_each
- guard clause: condition

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (sub-workflow execution) |
| **Global state access** | filesystem + git host |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
