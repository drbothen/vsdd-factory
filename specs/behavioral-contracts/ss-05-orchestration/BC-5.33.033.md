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

# Behavioral Contract BC-5.33.033: maintenance:maintenance-demo-recording

## Description

Step `maintenance-demo-recording` (line 337). Type: skill. Skill: `skills/demo-recording/SKILL.md`. Depends: `[maintenance-report]`. Condition: `maintenance.request_demo == true`. Timeout: 30m. Source 337-342.

## Preconditions

1. maintenance-report completed.
2. Maintenance run requested a demo.

## Postconditions

1. Demo recording artifact produced summarizing maintenance findings.

## Invariants

1. Bounded by 30m timeout.
2. Skipped when demo not requested.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | request_demo=false | Skipped |
| EC-002 | Timeout | Skip per defaults |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Demo requested | Recording produced | happy-path |
| Not requested | Skipped | edge-case |
| Timeout | Skipped | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Bounded by 30m | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.33.031 — maintenance-report

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
| **Path** | `plugins/vsdd-factory/workflows/maintenance.lobster` (lines 337-342) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- guard clause + type constraint (timeout)

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (recording) |
| **Global state access** | filesystem |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
