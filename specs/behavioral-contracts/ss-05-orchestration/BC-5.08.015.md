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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:929
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

# Behavioral Contract BC-5.08.015: pr-manager: delegates all gh/git commands to github-ops

## Description

pr-manager has no shell access. Every `gh` and `git` invocation MUST go through
github-ops via Agent dispatch with `subagent_type="vsdd-factory:github-ops"`.

## Preconditions

1. pr-manager needs to execute a gh or git command.

## Postconditions

1. pr-manager's effective tool profile excludes exec/process.
2. All gh/git commands appear in github-ops dispatch prompts.

## Invariants

1. pr-manager has zero shell access.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Need quick gh status check | Still goes through github-ops |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| All gh commands via github-ops dispatch | Accepted | happy-path |
| pr-manager calling gh directly | Tool denied | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | pr-manager tool profile excludes exec/process | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/pr-manager.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.08.014 — composes with (9-step coordinator)
- BC-5.08.010 — composes with (github-ops execution-only)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#pr-manager`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/pr-manager.md:41, 313-318` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Tool Access denial of exec/process

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | dispatches |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
