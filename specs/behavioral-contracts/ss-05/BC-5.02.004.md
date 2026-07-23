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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:789
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

# Behavioral Contract BC-5.02.004: orchestrator: never composes PR bodies or gh commands

## Description

PR descriptions, `gh` commands, and shell scripts MUST be authored by pr-manager
(or its delegates), not by the orchestrator. The orchestrator MUST dispatch
pr-manager for the PR lifecycle and never spawn github-ops directly for PR operations.

## Preconditions

1. A pipeline step requires PR creation, review, or merge.

## Postconditions

1. Orchestrator dispatch prompts contain no `gh pr create`/`gh pr merge` strings.
2. PR-related dispatches go to `vsdd-factory:pr-manager`.
3. github-ops is never spawned directly by the orchestrator for PR operations.

## Invariants

1. The orchestrator separates coordination from PR-body authorship.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Orchestrator drafts PR body in dispatch prompt | Rejected by audit |
| EC-002 | Orchestrator spawns github-ops for `gh pr create` | Rejected — must go through pr-manager |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Orchestrator dispatch log | All PR work routed through `vsdd-factory:pr-manager` | happy-path |
| Orchestrator prompt with `gh pr create` | Audit failure | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Tool-call audit shows zero `gh pr` strings in orchestrator dispatch prompts | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.08.014 — composes with (pr-manager 9-step coordinator)
- BC-5.08.015 — composes with (pr-manager delegates gh/git to github-ops)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#orchestrator-pr-rules`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md:122` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit prohibitions in orchestrator agent body

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none |
| **Global state access** | reads dispatch decision |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (logical guard) |

#### Refactoring Notes

No refactoring needed.
