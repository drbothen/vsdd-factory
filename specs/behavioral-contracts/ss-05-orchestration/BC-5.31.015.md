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

# Behavioral Contract BC-5.31.015: code-delivery:per-story-ui-quality-gate

## Description

Step `per-story-ui-quality-gate` (line 206). Type: agent. Agent: consistency-validator. Depends: `[per-story-adversarial-review, storybook-component-tests]`. UI condition. Source 206-219. Behavior: token compliance + a11y zero violations + component contract + async states; blocks merge on failure.

## Preconditions

1. Both upstream UI gates have completed.
2. UI feature_type.

## Postconditions

1. Token compliance verified.
2. A11y violations = 0.
3. Component contract verified.
4. Async-state coverage verified.
5. Merge is blocked on any failure.

## Invariants

1. UI quality gate is non-bypassable for UI stories (DF-037).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | A11y violation present | Gate blocks merge |
| EC-002 | Token drift | Gate blocks merge |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Compliant UI | Gate passes | happy-path |
| A11y violation | Gate blocks | error |
| Token drift | Gate blocks | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | All four sub-checks must pass | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.011 — per-story-adversarial-review
- BC-5.31.014 — storybook-component-tests

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
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 206-219) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: gate behavior comment

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (artifacts, tokens, a11y reports) |
| **Global state access** | reads filesystem |
| **Deterministic** | yes (given inputs) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
