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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1535
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

# Behavioral Contract BC-5.09.017: validate-extraction: 4-tier per-item disposition (VERIFIED / INACCURATE / HALLUCINATED / UNVERIFIABLE)

## Description

Every extracted item is dispositioned into one of 4 buckets: VERIFIED (matches
code), INACCURATE (corrections provided), HALLUCINATED (removed), UNVERIFIABLE
(cannot be checked, marked as such).

## Preconditions

1. validate-extraction dispositioning items.

## Postconditions

1. Validation report includes "Inaccurate Items (Corrected)", "Hallucinated Items
   (Removed)", and "Unverifiable Items" tables, plus a verified count.

## Invariants

1. Disposition enum is closed; every item is dispositioned.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Item partially correct | INACCURATE with correction |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Item with disposition VERIFIED | Accepted | happy-path |
| Item without disposition | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every item has disposition in 4-tier enum | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/validate-extraction.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.09.019 — composes with (>50% hallucination escalation)

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
| **Path** | `plugins/vsdd-factory/agents/validate-extraction.md:78-85` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit 4-tier disposition

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (classification) |

#### Refactoring Notes

No refactoring needed.
