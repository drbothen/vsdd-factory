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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:697
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

# Behavioral Contract BC-5.07.027: holdout-evaluator: read-only — no Write tool

## Description

The agent MUST execute the SUT (Bash) and Read holdout scenarios. It MUST NOT
write evaluation reports directly — those are persisted by orchestrator's call
back to state-manager (the report is composed in chat).

## Preconditions

1. holdout-evaluator dispatched.

## Postconditions

1. Tool profile = restricted; effective allowed tools = {Bash, Read}.

## Invariants

1. holdout-evaluator never writes files directly.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Evaluator needs to write report | Compose in chat; state-manager persists |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Evaluator session | Tools = {Bash, Read} | happy-path |
| Attempt to Write | Denied | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Tool profile excludes Write/Edit/Glob/Grep | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/holdout-evaluator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.024 — composes with (info wall)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#holdout-evaluator`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/holdout-evaluator.md:91-94` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit restricted tool profile

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + Bash (SUT) |
| **Global state access** | none |
| **Deterministic** | depends on SUT |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
