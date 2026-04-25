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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1519
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

# Behavioral Contract BC-5.09.015: validate-extraction: every numeric claim has a (claimed, recounted, delta) triple

## Description

Every numeric claim in the analysis MUST appear in the Phase 2 table with
claimed / recounted / delta / command columns. A row with `Delta: 0` is a pass;
any non-zero delta is an error regardless of magnitude.

## Preconditions

1. validate-extraction running Phase 2.

## Postconditions

1. Phase 2 table covers every numeric claim from the analysis.
2. No claim missing its row.

## Invariants

1. Every number is independently re-counted.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Claim 50 LOC, recount 50 | Delta 0; pass |
| EC-002 | Claim 500 files, recount 499 | Delta -1; error |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Triple for each claim | Accepted | happy-path |
| Claim without recount | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every numeric claim has a Phase 2 row | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/validate-extraction.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.09.014 — composes with (phase split)

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
| **Path** | `plugins/vsdd-factory/agents/validate-extraction.md:27-31` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit numeric triple rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (recount via shell) |

#### Refactoring Notes

No refactoring needed.
