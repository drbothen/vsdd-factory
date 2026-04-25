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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:567
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

# Behavioral Contract BC-5.07.017: e2e-tester: tests are idempotent and clean up

## Description

Every E2E test MUST clean up after itself and be runnable multiple times without
state-coupled failures.

## Preconditions

1. e2e-tester authoring a test.

## Postconditions

1. Re-running the e2e suite back-to-back yields identical pass/fail counts.
2. No `setup-once` fixtures that fail on re-run.

## Invariants

1. Tests are idempotent.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | External system has accumulating state | Test cleans up its own fixture data |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Run e2e suite twice | Identical results | happy-path |
| Test that leaks state | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Back-to-back e2e runs yield identical pass/fail counts | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/e2e-tester.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.015 — composes with (no internal mocking)

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
| **Path** | `plugins/vsdd-factory/agents/e2e-tester.md:192, 200-201` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit idempotency rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (test files + cleanup) |
| **Global state access** | none |
| **Deterministic** | yes (idempotent) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
