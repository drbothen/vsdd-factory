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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:575
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

# Behavioral Contract BC-5.07.018: e2e-tester: writes tests, not implementation code

## Description

Despite full profile, the agent MUST limit writes to `tests/e2e/` (or
framework-specific directory) and evidence directories. It MUST NOT modify `src/`.

## Preconditions

1. e2e-tester dispatched.

## Postconditions

1. Git diff shows no agent commits to `src/`.
2. Writes confined to `tests/e2e/` and evidence directories.

## Invariants

1. e2e-tester is a test author, not an implementer.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Test reveals implementation gap | Report; do not patch |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Test authoring | Diff confined to tests/e2e/ | happy-path |
| Attempt to edit src/ | Self-blocked | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Git diff shows zero src/ entries from e2e-tester | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/e2e-tester.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.044 — composes with (test-writer no implementation)

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
| **Path** | `plugins/vsdd-factory/agents/e2e-tester.md:92` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit no-source-modification rule

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
