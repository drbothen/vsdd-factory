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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:361
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

# Behavioral Contract BC-5.08.003: data-engineer: pure validation logic never touches DB I/O

## Description

Pure-core functions (validation, transformation, business rules) MUST be
side-effect-free. DB connections, query execution, migration runners live in
the Effectful Shell. Functions mixing both are rejected.

## Preconditions

1. data-engineer authoring validation/transformation logic.

## Postconditions

1. Static analysis or peer review confirms no function annotated `pure` performs I/O.

## Invariants

1. Pure validation is structurally separated from DB I/O.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Validation needs lookup against DB | Move lookup to caller (effectful shell); pass result as parameter |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Pure validate_email(s) function | Accepted | happy-path |
| validate_user_exists() with DB query | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | No `pure`-annotated function performs I/O | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/data-engineer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.05.001 — composes with (architect purity boundary)
- BC-5.07.031 — composes with (implementer respects purity)

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
| **Path** | `plugins/vsdd-factory/agents/data-engineer.md:30, 54-69, 95` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Purity Boundary Discipline section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (source) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (rule check) |

#### Refactoring Notes

No refactoring needed.
