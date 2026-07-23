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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:983
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

# Behavioral Contract BC-5.08.021: pr-reviewer: spawns `github-ops` (exact name) for posting

## Description

When dispatching for `gh pr review`, the agent MUST use
`subagent_type="vsdd-factory:github-ops"` exactly. Variants like `github`,
`gh-ops` are forbidden.

## Preconditions

1. pr-reviewer dispatching for posting.

## Postconditions

1. Tool-call audit shows the exact string `vsdd-factory:github-ops` in Agent dispatches.

## Invariants

1. Exact agent name; no aliases.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Typo `github_ops` | Rejected |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `subagent_type: vsdd-factory:github-ops` | Accepted | happy-path |
| `subagent_type: vsdd-factory:gh-ops` | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | All dispatches use exact agent name | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/pr-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.08.020 — composes with (gh pr review)
- BC-5.08.010 — composes with (github-ops execute-only)

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
| **Path** | `plugins/vsdd-factory/agents/pr-reviewer.md:43, 142` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit exact-name rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | dispatches |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (string match) |

#### Refactoring Notes

No refactoring needed.
