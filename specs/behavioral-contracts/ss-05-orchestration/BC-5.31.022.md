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

# Behavioral Contract BC-5.31.022: code-delivery:brownfield-full-regression

## Description

Step `brownfield-full-regression` (line 345). Type: agent. Agent: implementer. Depends: `[pr-review-convergence]`. Condition: `mode == 'brownfield'`. Source 345-353. HALT if any existing test fails.

## Preconditions

1. mode is `brownfield`.
2. pr-review-convergence reached APPROVE.

## Postconditions

1. Full pre-existing test suite executes.
2. Workflow halts on any failure (no merge).

## Invariants

1. Brownfield merges are gated by zero pre-existing-test failures.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | mode=greenfield | Step skipped |
| EC-002 | Single existing test fails | HALT |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Brownfield, all green | Pass | happy-path |
| Greenfield | Skipped | edge-case |
| Brownfield regression | HALT | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | HALT on any pre-existing test failure | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.021 — pr-review-convergence

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
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 345-353) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- guard clause: mode condition + HALT semantic

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (test runner) |
| **Global state access** | reads test results |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
