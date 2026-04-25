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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:821
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

# Behavioral Contract BC-5.02.008: orchestrator: prepends `cd <project-path> &&` and uses absolute paths in every dispatch

## Description

Every Agent dispatch prompt MUST begin with `cd <resolved-project-path> &&` and
reference all file paths as absolute paths. Relative paths cause writes to land
in the engine directory rather than the project workspace.

## Preconditions

1. Orchestrator dispatches a sub-agent via the Agent tool.
2. Workspace path has been resolved at session start (per BC-5.02.009).

## Postconditions

1. The dispatch prompt starts with `cd <resolved-project-path> &&`.
2. All file paths in the prompt are absolute paths (start with `/`).
3. No relative `.factory/` paths appear.

## Invariants

1. The cd preamble matches the resolved workspace, not engine root.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Dispatch missing cd preamble | Audit failure |
| EC-002 | Dispatch references `./factory/...` (relative) | Audit failure |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Audit of orchestrator prompts | Every prompt starts with `cd /` followed by `&&` | happy-path |
| Prompt with relative `./...` path | Audit failure | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every dispatch prompt matches regex `^cd /.*&&` | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.02.009 — composes with (workspace resolution at session start)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#dispatch-preamble`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md:142-156` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit "Task Preamble (CRITICAL)" section in orchestrator agent body

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none |
| **Global state access** | reads resolved workspace path |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (string formatter) |

#### Refactoring Notes

No refactoring needed.
