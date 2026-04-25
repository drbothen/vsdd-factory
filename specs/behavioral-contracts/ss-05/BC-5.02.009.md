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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:829
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

# Behavioral Contract BC-5.02.009: orchestrator: workspace resolution at session start (not from env var)

## Description

At session start, the orchestrator MUST resolve the target project path via a
4-step order: (1) Resume — read STATE.md; (2) User provides path; (3) Greenfield
— devops-engineer creates repo; (4) Explicit path. The resolved path MUST NOT
contain `dark-factory` (engine guard). The path is stored and used in every
dispatch's `cd` preamble.

## Preconditions

1. New session begins.

## Postconditions

1. WORKSPACE_PATH is resolved via the 4-step order (Resume → User → Greenfield → Explicit).
2. The resolved path does NOT contain `dark-factory`.
3. The resolved path is recorded in the session log.
4. All subsequent dispatches use this path in their `cd` preamble.

## Invariants

1. Workspace resolution is mandatory at session start — no env-var shortcut.
2. The dark-factory guard prevents writes to the engine source tree.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Resolved path contains `dark-factory` | Halt with engine-guard error |
| EC-002 | STATE.md missing on Resume | Fall through to step 2 (ask user) |
| EC-003 | Greenfield with no devops-engineer | TBD — error |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Resume with valid STATE.md | Path read from STATE.md | happy-path |
| Greenfield mode | devops-engineer creates repo, path returned | happy-path |
| Path contains `dark-factory` | Halt with engine-guard error | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Session start log records resolved WORKSPACE_PATH | manual |
| VP-TBD | Resolved path passes the no-`dark-factory` check | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.02.008 — composes with (cd-preamble dispatch rule)
- BC-5.02.013 — composes with (pipeline resume worktree health check)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#workspace-resolution`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md:62-80` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: "Workspace Resolution (CRITICAL — do this FIRST)" section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (STATE.md, user input) |
| **Global state access** | reads/writes session WORKSPACE_PATH |
| **Deterministic** | no (depends on user input / repo state) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
