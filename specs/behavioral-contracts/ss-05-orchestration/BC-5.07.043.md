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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1161
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

# Behavioral Contract BC-5.07.043: security-reviewer: posts via gh pr review, never gh pr comment (per-story)

## Description

Per-story security findings MUST be posted via
`gh pr review --request-changes --body-file ...` (or `--approve`) using
github-ops with exact agent name. `gh pr comment` is forbidden for security verdicts.

## Preconditions

1. security-reviewer producing per-story PR verdict.

## Postconditions

1. PR review history shows security review posted as a formal review
   (state: APPROVED or CHANGES_REQUESTED).
2. Not a free-form comment.

## Invariants

1. Formal review only; never PR comments.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Need to leave inline note | Use formal review with inline comments |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `gh pr review --request-changes` | Accepted | happy-path |
| `gh pr comment` | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every per-story security verdict is a formal review | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/security-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.08.020 — composes with (pr-reviewer gh pr review rule)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#security-reviewer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/security-reviewer.md:124-130` |
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
