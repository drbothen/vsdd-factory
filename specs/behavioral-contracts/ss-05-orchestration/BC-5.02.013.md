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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:869
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

# Behavioral Contract BC-5.02.013: orchestrator: pipeline resume requires factory-worktree-health BEFORE STATE.md read

## Description

On resume, the orchestrator MUST first spawn devops-engineer to run the
factory-worktree-health skill. STATE.md and `.factory/` are not read until the
health check passes.

## Preconditions

1. A pipeline resume action is requested.

## Postconditions

1. devops-engineer is dispatched with the factory-worktree-health skill BEFORE
   any Read on STATE.md or `.factory/` artifacts.
2. The dispatch is BLOCKING — the orchestrator waits for completion.
3. Only on PASS does the orchestrator proceed to read STATE.md.

## Invariants

1. The health check is a precondition for resume — never skipped.
2. Reading STATE.md before health check passes is forbidden.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | health check FAIL | Halt; do not read STATE.md; report recovery action |
| EC-002 | devops-engineer unavailable | Halt; report blocker |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Resume session log | devops-engineer dispatch precedes STATE.md Read | happy-path |
| Resume with broken worktree | Halt before STATE.md Read | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Resume session log shows devops-engineer dispatch before any Read on STATE.md | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md`, factory-worktree-health skill |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.02.009 — composes with (workspace resolution)
- BC-5.10.004 — composes with (state-manager worktree preconditions)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#pipeline-resume`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md:354-360` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit "Pipeline Resume" section in orchestrator agent body

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (worktree state) |
| **Global state access** | reads `.factory/.git`, branch state |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
