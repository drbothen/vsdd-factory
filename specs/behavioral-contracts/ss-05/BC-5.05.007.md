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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:307
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

# Behavioral Contract BC-5.05.007: consistency-validator: 80 criteria, none skipped

## Description

Every consistency report MUST cover all 80 criteria. Criteria that cannot be
checked must be reported as such with explanation — never silently skipped.

## Preconditions

1. consistency-validator dispatched.

## Postconditions

1. Consistency report's summary table has 80 rows (or 80 entries by ID).
2. No criterion has status "skipped without reason."
3. Skipped criteria include explanation.

## Invariants

1. The 80-criterion list is closed and exhaustive.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Criterion not applicable to project | Reported with N/A + explanation |
| EC-002 | Cannot check due to missing artifact | Reported with explanation |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Consistency report with 80 rows | Accepted | happy-path |
| Report with only 75 rows | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every consistency report covers all 80 criteria | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/consistency-validator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.05.008 — composes with (index-first discipline)
- BC-5.05.009 — composes with (gate fails on blocking findings)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#consistency-validator`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/consistency-validator.md:96-99, 113-116, 129-345` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit 80-criteria tables

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (specs) + writes (report) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
