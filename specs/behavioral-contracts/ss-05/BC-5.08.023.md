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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:999
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

# Behavioral Contract BC-5.08.023: pr-reviewer: no rubber-stamping — explain what was verified

## Description

When no findings are produced, the review MUST include an explanation of which
checklist items were verified (8-item checklist: diff coherence, description
accuracy, test coverage, demo evidence, commit quality, diff size, missing
changes, dependency status).

## Preconditions

1. pr-reviewer producing an APPROVE verdict with no findings.

## Postconditions

1. Approving review body has a non-empty "verified" section enumerating the 8 checklist items.

## Invariants

1. No silent approvals.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | One checklist item N/A (e.g., no demo for non-UI) | Note as N/A in verified section |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| APPROVE with 8-item verification | Accepted | happy-path |
| APPROVE with empty body | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Approving reviews have a non-empty "verified" section | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/pr-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.08.022 — composes with (3-tier severity)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#pr-reviewer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/pr-reviewer.md:40, 63` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit no-rubber-stamping rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (review) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
