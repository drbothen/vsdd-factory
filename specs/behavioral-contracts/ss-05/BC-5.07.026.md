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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:689
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

# Behavioral Contract BC-5.07.026: holdout-evaluator: 0.0–1.0 satisfaction scoring per scenario

## Description

Each scenario gets a satisfaction score on a 0.0–1.0 continuous scale interpreted
via the documented anchors (1.0 = fully satisfied, 0.8 = minor deviation,
0.5 = partial, 0.2 = mostly failing, 0.0 = complete failure).

## Preconditions

1. holdout-evaluator scoring a scenario.

## Postconditions

1. Every Per-Scenario Result row has a `Score` value in [0.0, 1.0].

## Invariants

1. Scoring scale is bounded and continuous.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Scenario partially blocked | Score around 0.2-0.5 with anchor explanation |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Scenario fully satisfied | Score: 1.0 | happy-path |
| Scenario complete failure | Score: 0.0 | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every scenario score is in [0.0, 1.0] | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/holdout-evaluator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.025 — composes with (gate criteria)

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
| **Path** | `plugins/vsdd-factory/agents/holdout-evaluator.md:50-60` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit scoring anchors

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (scenario) + writes (score) |
| **Global state access** | none |
| **Deterministic** | depends on SUT |
| **Thread safety** | unknown |
| **Overall classification** | pure (scoring rule) |

#### Refactoring Notes

No refactoring needed.
