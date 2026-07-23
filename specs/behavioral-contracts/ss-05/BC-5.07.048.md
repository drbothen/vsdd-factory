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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1451
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

# Behavioral Contract BC-5.07.048: test-writer: property-based tests generate ≥1000 random cases

## Description

proptest / fast-check / Hypothesis property tests MUST be configured to generate
at least 1000 random cases. Lower bounds are forbidden for property tests.

## Preconditions

1. test-writer authoring a property test.

## Postconditions

1. Every property test has a config with `cases >= 1000` (or framework equivalent).

## Invariants

1. The 1000-case minimum is canonical.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | proptest default is 256 | Override to ≥1000 |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `proptest! { #![proptest_config(ProptestConfig::with_cases(1000))] }` | Accepted | happy-path |
| Property test with 256 cases | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every property test has cases >= 1000 | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/test-writer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.047 — composes with (no vacuous tests)

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
| **Path** | `plugins/vsdd-factory/agents/test-writer.md:325` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit property-test minimum

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (tests) |
| **Global state access** | none |
| **Deterministic** | yes (given fixed seed) |
| **Thread safety** | unknown |
| **Overall classification** | pure (test authoring) |

#### Refactoring Notes

No refactoring needed.
