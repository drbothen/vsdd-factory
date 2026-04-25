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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1435
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

# Behavioral Contract BC-5.07.046: test-writer: Red Gate must be verified — all tests fail before implementation

## Description

After writing tests, the test-writer MUST run the suite and verify every test
fails (Red Gate). A test that passes without implementation is suspect — flag
for human review. Red gate results written to `red-gate-log.md`.

## Preconditions

1. test-writer authoring tests for a story.

## Postconditions

1. `.factory/cycles/**/implementation/red-gate-log.md` exists.
2. All tests in failing state with timestamps.
3. No test passing pre-implementation (suspect tests flagged).

## Invariants

1. Red Gate is a structural prerequisite for handoff to implementer.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Test passes pre-impl | Flag for human review (suspect or spec wrong) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| All tests red, log captured | Accepted | happy-path |
| One test green pre-impl | Flagged for human | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | red-gate-log.md shows all tests failing pre-impl | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/test-writer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.028 — composes with (implementer Red Gate)

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
| **Path** | `plugins/vsdd-factory/agents/test-writer.md:89, 235-243` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Verify Red Gate step

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (test runner) + writes (log) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
