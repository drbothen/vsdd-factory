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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1419
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

# Behavioral Contract BC-5.07.044: test-writer: never writes implementation code

## Description

test-writer writes tests, stubs (compilable empty implementations), and
red-gate-log only. Production source code is the implementer's exclusive scope.

## Preconditions

1. test-writer dispatched.

## Postconditions

1. Git diff shows changes only in `tests/`, stub files, or
   `.factory/cycles/**/implementation/red-gate-log.md`.
2. Zero production source code changes.

## Invariants

1. test-writer is a tests-only agent.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Need to add stub | Allowed (compilable empty) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Test authoring | Diff confined to tests/ + stubs | happy-path |
| Attempt to add real logic to src/ | Self-blocked | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Git diff has zero production source changes | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/test-writer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.045 — composes with (BC-NNN test naming)

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
| **Path** | `plugins/vsdd-factory/agents/test-writer.md:87, 320, 351` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit tests-only rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (tests/) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
