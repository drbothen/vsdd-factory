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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1427
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

# Behavioral Contract BC-5.07.045: test-writer: BC-NNN-traceable test naming required

## Description

Every test name MUST follow `test_BC_S_SS_NNN_[assertion_name]()`. Generic names
(`test_1`, `test_basic`, `test_it_works`) are forbidden.

## Preconditions

1. test-writer authoring tests.

## Postconditions

1. Every test in the suite matches the regex
   `^test_BC_\d+_\d{1,2}_\d{3}_[a-z_]+(\(\))?$` (allowing language-specific differences).

## Invariants

1. Naming is BC-traceable, never generic.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Test exercises multiple BCs | Pick most-load-bearing in name |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `test_BC_5_01_001_lobster_yaml_top_level()` | Accepted | happy-path |
| `test_basic()` | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every test name matches the canonical regex | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/test-writer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.044 — composes with (no implementation)

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
| **Path** | `plugins/vsdd-factory/agents/test-writer.md:88, 134-149` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Test Naming Convention

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (tests) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (file authoring) |

#### Refactoring Notes

No refactoring needed.
