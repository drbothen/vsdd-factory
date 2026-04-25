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

# Behavioral Contract BC-5.32.030: discovery:execute-product-ideas

## Description

Step `execute-product-ideas` (line 384). Type: sub-workflow. Sub-workflow: `planning.lobster`. Depends: `[route-approved-ideas]`. Condition: `discovery.approved_products.count > 0`. Source 384-388.

## Preconditions

1. route-approved-ideas completed.
2. At least one product idea was approved.

## Postconditions

1. `planning.lobster` invoked per approved product idea.

## Invariants

1. Step skipped when no approved products.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | No approved products | Skipped |
| EC-002 | planning.lobster fails | Sub-workflow escalation |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| 1 product approved | planning.lobster invoked | happy-path |
| 0 products | Skipped | edge-case |
| Sub-workflow error | Escalate | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Sub-workflow target = planning.lobster | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.32.028 — route-approved-ideas
- BC-5.35.001 — planning identity (sub-workflow target)

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#discovery-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/discovery.lobster` (lines 384-388) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- guard clause + documentation

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (sub-workflow execution) |
| **Global state access** | filesystem + state |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
