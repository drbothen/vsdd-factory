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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1511
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

# Behavioral Contract BC-5.09.014: validate-extraction: behavioral and metric phases must be split

## Description

Validation runs in two strictly separated phases: Phase 1 — behavioral verification
(judgment, sample-and-confirm); Phase 2 — metric verification (independent recount
with `find`, `wc -l`, `grep -c`, `ls | wc -l` — no estimation). Each phase reports
in its own table; phases MUST NOT be interleaved.

## Preconditions

1. validate-extraction dispatched.

## Postconditions

1. Output report has two distinct top-level tables labeled
   "Phase 1 — Behavioral Verification" and "Phase 2 — Metric Verification."

## Invariants

1. Phase split prevents metric inflation slipping through behavioral sampling.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Single-claim validation | Both phases still apply |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Two distinct tables | Accepted | happy-path |
| Interleaved phases | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Output has Phase 1 and Phase 2 tables, distinct | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/validate-extraction.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.09.015 — composes with (numeric triple)
- BC-5.09.016 — composes with (refinement bound)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#validate-extraction`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/validate-extraction.md:19-34` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Behavioral vs Metric Split rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
