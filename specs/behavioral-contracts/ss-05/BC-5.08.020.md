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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:975
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

# Behavioral Contract BC-5.08.020: pr-reviewer: posts via `gh pr review`, never `gh pr comment`

## Description

Every review MUST be posted via `gh pr review --request-changes <body>` or
`gh pr review --approve <body>`, not `gh pr comment`. Every review needs an
explicit verdict.

## Preconditions

1. pr-reviewer producing a verdict.

## Postconditions

1. PR review history on GitHub shows the review created via `gh pr review` events
   with `state: APPROVED` or `state: CHANGES_REQUESTED`.

## Invariants

1. Formal review only; never PR comments.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Need to post non-blocking suggestion | Use formal review with SUGGESTION tag |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `gh pr review --approve` | Accepted | happy-path |
| `gh pr comment` | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every review is a formal `gh pr review` event | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/pr-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.08.021 — composes with (github-ops exact agent name)

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
| **Path** | `plugins/vsdd-factory/agents/pr-reviewer.md:42, 142-145` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit gh pr review rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | dispatches github-ops |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
