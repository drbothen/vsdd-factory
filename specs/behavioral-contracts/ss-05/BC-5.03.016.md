---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-deep-agents.md]
input-hash: "595f07d"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1573
subsystem: SS-05
capability: CAP-TBD
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

# Behavioral Contract BC-5.03.016: visual-reviewer: 4-dimensional satisfaction scoring (functional / visual / timing / completeness)

## Description

Each demo gets a satisfaction score weighted across 4 dimensions: functional
correctness (40%), visual quality (20%), timing (20%), completeness (20%).

## Preconditions

1. visual-reviewer evaluating a demo recording.

## Postconditions

1. visual-review.md per-demo rows show the 4 sub-scores.
2. Weights are: Functional 0.4, Visual 0.2, Timing 0.2, Completeness 0.2.
3. Sub-scores combine to a 0.0–1.0 satisfaction number.

## Invariants

1. The 4-dimensional weight scheme is canonical.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Demo with no visual content (audio-only) | TBD — visual quality N/A or zero |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Demo with all 4 sub-scores | Combined satisfaction reported | happy-path |
| Single-dimension scoring | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every per-demo row has all 4 sub-scores with documented weights | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/visual-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.026 — composes with (holdout-evaluator scoring scale)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#visual-reviewer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/visual-reviewer.md:132-138` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit 4-dimensional scoring scheme

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (recordings) + writes (review report) |
| **Global state access** | none |
| **Deterministic** | yes (given fixed recordings) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
