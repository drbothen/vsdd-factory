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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1565
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

# Behavioral Contract BC-5.03.015: visual-reviewer: analyzes recordings, never source

## Description

The visual-reviewer watches demo recordings and produces visual-review.md. It
MUST NOT modify source, tests, or specs. Its judgment is on user-visible output,
not code.

## Preconditions

1. visual-reviewer dispatched against a story with demo recordings.

## Postconditions

1. Git diff shows changes only in `.factory/demo-evidence/visual-review.md`.
2. Zero modifications to source, tests, or specs.

## Invariants

1. visual-reviewer is read-only with respect to source/tests/specs.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Recording file is missing | Reported BLOCKED (per BC-5.03.017) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Demo review session | visual-review.md is the only diff | happy-path |
| Attempt to edit source | Self-blocked | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Git diff after visual-reviewer runs has zero source/test/spec entries | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/visual-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.03.016 — composes with (4-dimensional satisfaction scoring)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#visual-reviewer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/visual-reviewer.md:81, 159, 161` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit read-only rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (recordings) + writes (visual-review.md only) |
| **Global state access** | none |
| **Deterministic** | yes (given fixed recordings) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
