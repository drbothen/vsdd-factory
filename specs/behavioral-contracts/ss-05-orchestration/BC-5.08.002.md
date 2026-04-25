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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:353
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

# Behavioral Contract BC-5.08.002: data-engineer: every field has a privacy classification before schema finalization

## Description

Every column/field in any schema artifact MUST be tagged PII, sensitive, or public.
No schema is finalized with unclassified fields.

## Preconditions

1. data-engineer authoring or finalizing a schema.

## Postconditions

1. Every schema field has a `privacy:` annotation set to one of {PII, sensitive, public}.

## Invariants

1. Schema finalization requires complete classification.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Field privacy unclear | Mark sensitive (most restrictive sane default) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Schema with privacy tag per field | Accepted | happy-path |
| Schema missing privacy on a column | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every schema field has privacy ∈ {PII, sensitive, public} | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/data-engineer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.08.001 — composes with (reversible migrations)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#data-engineer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/data-engineer.md:49, 96` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit privacy classification rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (schema files) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (annotation rule) |

#### Refactoring Notes

No refactoring needed.
