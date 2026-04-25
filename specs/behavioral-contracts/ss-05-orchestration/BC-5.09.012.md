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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1199
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

# Behavioral Contract BC-5.09.012: session-reviewer: no information walls — sees everything

## Description

Unlike other reviewers, session-reviewer MUST be able to see the complete picture
(source, specs, adversary findings, TDD logs, cost data, convergence history,
holdout results) — needed for run analysis. No exclusion list.

## Preconditions

1. session-reviewer dispatched.

## Postconditions

1. Tool-call audit shows reads across all `.factory/` paths.
2. No exclusions enforced.

## Invariants

1. session-reviewer is the only reviewer with no information walls.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Adversary findings include sensitive analysis | session-reviewer can read; treat with care |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| session-reviewer reading adversary findings | Allowed | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | session-reviewer has no `context: { exclude }` block | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/session-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.09.009 — composes with (read-only)
- BC-5.09.010 — composes with (8-dimensional analysis)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#session-reviewer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/session-reviewer.md:82-87` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit "No Information Asymmetry Wall"

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (any path) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (read-only) |

#### Refactoring Notes

No refactoring needed.
