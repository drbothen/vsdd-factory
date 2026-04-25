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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:681
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

# Behavioral Contract BC-5.07.025: holdout-evaluator: gate criteria — mean ≥0.85, every critical scenario ≥0.60

## Description

The Phase 4 gate passes only when mean satisfaction across all evaluated scenarios
is ≥ 0.85 AND every critical scenario scores ≥ 0.60.

## Preconditions

1. holdout-evaluator concluding Phase 4 gate evaluation.

## Postconditions

1. Holdout report's Gate row reads PASS only if both numerical conditions hold.
2. Otherwise FAIL with gap report.

## Invariants

1. Both conditions are required for PASS.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Mean 0.86 but one critical scenario at 0.55 | FAIL |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Mean 0.90, all critical ≥0.65 | PASS | happy-path |
| Mean 0.80 | FAIL | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Gate PASS only when mean ≥0.85 AND every critical ≥0.60 | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/holdout-evaluator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.024 — composes with (info wall)
- BC-5.07.026 — composes with (scoring scale)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#holdout-evaluator`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/holdout-evaluator.md:86-88` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Gate criteria

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (numerical thresholds) |

#### Refactoring Notes

No refactoring needed.
