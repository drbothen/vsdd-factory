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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1191
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

# Behavioral Contract BC-5.09.011: session-reviewer: actionable proposals, not vague observations

## Description

Output includes (a) Session Review Report and (b) Improvement Proposals —
structured with category, priority, evidence, recommendation, affected files,
risk. Vague observations are forbidden.

## Preconditions

1. session-reviewer producing review output.

## Postconditions

1. Improvement Proposals document has structured rows.
2. Every proposal has all 6 fields populated (category, priority, evidence,
   recommendation, affected files, risk).

## Invariants

1. Proposals are structured and actionable.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Vague observation | Force into structured form or drop |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Proposal with all 6 fields | Accepted | happy-path |
| Proposal missing recommendation | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every proposal has all 6 fields populated | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/session-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.09.010 — composes with (8-dimensional analysis)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#session-reviewer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/session-reviewer.md:22, 158-162` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Improvement Proposals structure

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (proposal text) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure |

#### Refactoring Notes

No refactoring needed.
