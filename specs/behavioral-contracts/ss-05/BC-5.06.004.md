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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:209
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

# Behavioral Contract BC-5.06.004: business-analyst: every ASM has a validation method; every R-NNN has a mitigation

## Description

Every ASM-NNN MUST include `Status: unvalidated` (initial state) and a concrete
Validation Method. Every R-NNN MUST have `Status: open`, a `Category` tag
(security|performance|reliability|business), and a Mitigation. HIGH-impact
R-NNNs with quantifiable mitigations get `NFR candidate: yes/no`. Security
R-NNNs get `Security focus: yes`.

## Preconditions

1. business-analyst authoring assumptions or risks.

## Postconditions

1. assumptions.md has zero rows missing `Status` or `Validation Method`.
2. risks.md has zero rows missing `Status`, `Category`, or `Mitigation`.

## Invariants

1. ASM/R metadata is mandatory and complete.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Risk has no known mitigation | Mark mitigation `Pending — track via R-NNN review` |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| ASM-001 with Status + Validation Method | Accepted | happy-path |
| R-001 with Status + Category + Mitigation | Accepted | happy-path |
| ASM without Validation Method | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | All ASM-NNN/R-NNN entries have required fields | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/business-analyst.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.06.001 — composes with (grounding in brief)
- BC-5.06.003 — composes with (all template sections)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#business-analyst`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/business-analyst.md:49, 104-110` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit ASM/R Production Rules

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
