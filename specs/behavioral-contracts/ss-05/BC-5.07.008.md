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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:285
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

# Behavioral Contract BC-5.07.008: codebase-analyzer: convergence bounds — min 2 rounds, max 5 before escalation

## Description

A pass cannot declare NITPICK convergence before at least 2 deepening rounds.
After 5 rounds without convergence, the agent escalates to human.

## Preconditions

1. A pass is undergoing convergence deepening.

## Postconditions

1. No pass declared NITPICK at round 1.
2. Round 5 with SUBSTANTIVE triggers escalation.

## Invariants

1. The 2/5 bounds are universal.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Round 2 still SUBSTANTIVE | Continue to round 3 |
| EC-002 | Round 5 SUBSTANTIVE | Escalate to human |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Round 1 NITPICK | Rejected (need round 2 minimum) | error |
| Round 2 NITPICK after 1 SUBSTANTIVE | Convergence declared | happy-path |
| Round 5 SUBSTANTIVE | Human escalation | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Min 2 rounds before NITPICK; max 5 rounds before escalation | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/codebase-analyzer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.007 — composes with (binary novelty)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#codebase-analyzer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/codebase-analyzer.md:341-345` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Convergence Bounds

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none (counter logic) |
| **Global state access** | reads round counter |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure |

#### Refactoring Notes

No refactoring needed.
