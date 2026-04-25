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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:239
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

# Behavioral Contract BC-5.07.003: code-reviewer: pass 2+ never re-reports prior findings

## Description

Pass 1: all findings in Part B (no Part A). Pass 2+: Part A is fix-verification
of prior findings (RESOLVED / PARTIALLY_RESOLVED / UNRESOLVED), Part B is only
NEW findings not in any previous pass.

## Preconditions

1. code-reviewer dispatched on pass 2 or later.

## Postconditions

1. Pass-N report frontmatter has `pass: N` and `previous_review: <path>`.
2. Part A row count equals prior pass's CR finding count.
3. Part B contains only IDs > max prior CR-NNN.

## Invariants

1. Re-report of unresolved findings goes in Part A (not Part B).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Pass 1 includes Part A | Rejected (no prior pass) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Pass 2 with Part A status updates + Part B new IDs | Accepted | happy-path |
| Pass 2 with old findings re-listed in Part B | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Part B IDs > max prior CR-NNN | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/code-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.002 — composes with (6-category classification)
- BC-5.07.004 — composes with (convergence verdict)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#code-reviewer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/code-reviewer.md:30, 87-104` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Multi-Pass Review Protocol

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (prior pass) + writes (current pass) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
