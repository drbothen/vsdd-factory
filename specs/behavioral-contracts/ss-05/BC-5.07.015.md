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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:551
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

# Behavioral Contract BC-5.07.015: e2e-tester: never mocks internal components

## Description

End-to-end tests MUST exercise the real system through its public interface
(CLI, HTTP, library API). Internal modules MUST NOT be mocked or stubbed.
Test-writer's unit tests handle isolated mocking; e2e is integration-only.

## Preconditions

1. e2e-tester authoring tests.

## Postconditions

1. No file in `tests/e2e/` imports a mocking framework against an internal module.

## Invariants

1. e2e is integration-only; mocks are unit-test territory.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | External dependency truly unreachable in CI | Document; do not mock |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| e2e test using real binary | Accepted | happy-path |
| e2e test mocking internal module | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | tests/e2e/ contains no internal-module mocks | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/e2e-tester.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.016 — composes with (BC-NNN traceable test naming)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#e2e-tester`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/e2e-tester.md:115, 191, 199` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit no-internal-mocking rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (test files) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
