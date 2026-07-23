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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1551
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

# Behavioral Contract BC-5.09.019: validate-extraction: >50% hallucination rate triggers Level 3 escalation

## Description

When more than 50% of items are hallucinated, the validator escalates. The
implication: codebase-analyzer pass needs re-running with better prioritization —
not iterative refinement.

## Preconditions

1. validate-extraction computing hallucination rate.

## Postconditions

1. Reports with hallucination rate >50% include a Level 3 escalation flag.
2. Recommendation: re-run codebase-analyzer (not iterate refinement).

## Invariants

1. The 50% threshold is canonical.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Rate exactly 50% | TBD — likely no escalation (use `>` not `>=`) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Hallucination rate 30% | No escalation | happy-path |
| Hallucination rate 60% | Level 3 escalation | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Reports with rate > 50% have Level 3 escalation flag | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/validate-extraction.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.09.017 — composes with (4-tier disposition)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#validate-extraction`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/validate-extraction.md:153-156` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Level 3 escalation rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (threshold check) |

#### Refactoring Notes

No refactoring needed.
