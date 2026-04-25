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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:719
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

# Behavioral Contract BC-5.07.029: implementer: minimum code per test (TDD discipline)

## Description

The implementer writes the minimum code to make exactly ONE failing test pass,
then moves to the next failing test. After all tests pass, refactor with tests
as safety net. Bulk implementation is forbidden.

## Preconditions

1. implementer in TDD loop.

## Postconditions

1. Micro-commit history shows one test at a time progressing red→green.
2. No single commit makes >2 tests pass simultaneously (allowing for shared dependencies).
3. Refactoring step happens after all tests pass.

## Invariants

1. Bulk implementation is forbidden.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Two tests share infrastructure | One commit may make both pass |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Commits progressing one test at a time | Accepted | happy-path |
| Single commit making 5 tests pass | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | No commit makes >2 tests pass simultaneously | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/implementer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.028 — composes with (Red Gate)
- BC-5.07.030 — composes with (micro-commit per passing test)

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
| **Path** | `plugins/vsdd-factory/agents/implementer.md:97, 89-93` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit TDD Protocol

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (source) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
