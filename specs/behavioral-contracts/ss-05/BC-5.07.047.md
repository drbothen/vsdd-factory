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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1443
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

# Behavioral Contract BC-5.07.047: test-writer: never writes vacuously true tests

## Description

Tests MUST exercise actual behavior — `assert!(true)`, no-op assertions, or
tautological tests are forbidden. Boundary tests are mandatory.

## Preconditions

1. test-writer authoring tests.

## Postconditions

1. No test body matches simple tautology patterns.
2. Mutation testing kill rate (formal-verifier) is high.
3. Boundary tests (empty, too-long, whitespace, case sensitivity, invalid formats) are present.

## Invariants

1. Tests are non-vacuous.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Sanity check that always passes | Use proptest with non-trivial input space |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Test exercising boundary | Accepted | happy-path |
| `assert!(true)` test | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Mutation kill rate is high (proves tests are non-vacuous) | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/test-writer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.021 — composes with (mutation kill rate threshold)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#test-writer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/test-writer.md:321, 322` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit boundary-tests rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (tests) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (test authoring) |

#### Refactoring Notes

No refactoring needed.
