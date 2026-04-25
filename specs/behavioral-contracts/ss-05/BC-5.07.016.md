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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:559
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

# Behavioral Contract BC-5.07.016: e2e-tester: BC-NNN traceable test naming

## Description

Every E2E test name MUST follow the pattern `test_e2e_BC_S_SS_NNN_xxx()` for
full traceability.

## Preconditions

1. e2e-tester authoring a test.

## Postconditions

1. Every test under `tests/e2e/` matches the naming regex.
2. No `test_e2e_1()` or vague names.

## Invariants

1. Every test traces back to a BC.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Test exercises multiple BCs | Pick the most-load-bearing in name; document others in body |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `test_e2e_BC_5_01_001_lobster_parses()` | Accepted | happy-path |
| `test_e2e_basic_flow()` | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every e2e test name matches the BC-traceable regex | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/e2e-tester.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.044 — composes with (test-writer BC-NNN naming)

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
| **Path** | `plugins/vsdd-factory/agents/e2e-tester.md:110, 207` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit BC-NNN tracing rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (test files) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (file authoring) |

#### Refactoring Notes

No refactoring needed.
