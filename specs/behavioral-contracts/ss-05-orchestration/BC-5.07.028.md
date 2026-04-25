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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:711
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

# Behavioral Contract BC-5.07.028: implementer: never writes code without a failing test (Red Gate)

## Description

The implementer MUST consume failing tests from test-writer. Writing implementation
without a corresponding failing test is forbidden. Tests are produced by
test-writer; implementer never writes tests.

## Preconditions

1. implementer dispatched against a story.

## Postconditions

1. Every implementer commit advances at least one previously-failing test to passing.
2. No commit adds source code without a related test having been previously red.

## Invariants

1. Red-Gate-first TDD discipline.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Implementer needs additional test | Spawn test-writer; do not author tests |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Commit advancing test_BC_X red→green | Accepted | happy-path |
| Commit adding code with no failing test | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every implementer commit advances at least one previously-failing test | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/implementer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.029 — composes with (minimum code per test)
- BC-5.07.046 — composes with (test-writer Red Gate)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#implementer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/implementer.md:95-96, 356` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Red Gate rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (tests) + writes (source) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
