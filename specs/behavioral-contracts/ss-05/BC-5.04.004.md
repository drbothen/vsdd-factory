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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:101
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

# Behavioral Contract BC-5.04.004: adversary: minimum 3 clean passes, max 10 before human escalation

## Description

The adversary may declare convergence only after at least 3 consecutive
NITPICK-novelty passes; orchestration must escalate to human after 10 total
passes regardless of state.

## Preconditions

1. An adversarial review loop is active.

## Postconditions

1. `convergence_reached: true` is set only when N ≥ 3 AND prior 3 passes were all NITPICK.
2. The 10th pass without convergence triggers human escalation.

## Invariants

1. The 3-clean / 10-max bounds are universal.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Pass 4 finds SUBSTANTIVE finding | Counter resets; need new 3 clean passes |
| EC-002 | Pass 10 still SUBSTANTIVE | Escalate to human |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| 3 consecutive NITPICK passes | convergence_reached: true | happy-path |
| 10 passes without 3-clean run | Human escalation | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | No pass-N report sets convergence_reached: true with fewer than 3 prior NITPICK | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/adversary.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.02.010 — composes with (orchestrator 3-clean convergence rule)
- BC-5.04.005 — composes with (max 3 self-validation iterations)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#adversary-bounds`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/adversary.md:96` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit minimum/maximum rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (prior pass histories) |
| **Global state access** | reads pass-novelty history |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (counter logic) |

#### Refactoring Notes

No refactoring needed.
