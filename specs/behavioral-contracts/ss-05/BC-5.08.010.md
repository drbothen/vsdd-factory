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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:643
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

# Behavioral Contract BC-5.08.010: github-ops: executes only — never makes decisions

## Description

The github-ops agent is a thin wrapper around the `gh` CLI. It MUST NOT modify
the requested command, MUST NOT interpret or filter results, and MUST NOT decide
what to merge / review / triage.

## Preconditions

1. github-ops dispatched with a gh command.

## Postconditions

1. Agent's tool-call log shows the exact command from the dispatch prompt being executed.
2. Output is returned verbatim.

## Invariants

1. github-ops is a tool, not a decision-maker.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Command would modify behavior | Execute as given; do not adjust |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Dispatch with `gh pr view 42` | Execute exactly | happy-path |
| Modify command before execution | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Tool log shows verbatim execution | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/github-ops.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.08.011 — composes with (full output unmodified)
- BC-5.08.012 — composes with (retry policy)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#github-ops`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/github-ops.md:36, 86-90, 121-122` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit "tool, not decision-maker" rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | shell (gh) |
| **Global state access** | none |
| **Deterministic** | yes (verbatim execution) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
