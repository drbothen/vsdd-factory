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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:845
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

# Behavioral Contract BC-5.08.013: orchestrator: dispatches state-manager directly for .factory/ commits — never devops-engineer

## Description

All `.factory/` git operations are delegated to state-manager directly. The
orchestrator MUST NOT dispatch devops-engineer for factory artifact commits —
devops-engineer's git scope is source-code branches only.

## Preconditions

1. Orchestrator needs `.factory/` git operation.

## Postconditions

1. All `git commit` events on `factory-artifacts` branch are authored via state-manager dispatches.
2. devops-engineer commits only on `develop`/`feature/*` branches.

## Invariants

1. Git scope is split: state-manager → factory-artifacts; devops-engineer → source branches.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Need both source + factory commit in same burst | Two dispatches: devops-engineer + state-manager |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| .factory commit via state-manager | Accepted | happy-path |
| .factory commit via devops-engineer | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | All factory-artifacts commits authored by state-manager | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.10.001 — composes with (state-manager git scope)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#git-scope-split`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md:196-204` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit State Manager Delegation section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none (dispatch routing) |
| **Global state access** | reads dispatch decision |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (routing rule) |

#### Refactoring Notes

No refactoring needed.
