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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1175
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

# Behavioral Contract BC-5.09.009: session-reviewer: T1 read-only, NEVER writes files

## Description

The session-reviewer is T1 read-only. It MUST NOT use Write/Edit/Bash/Agent
tools. State-manager persists its output to `.factory/session-reviews/`.

## Preconditions

1. session-reviewer dispatched.

## Postconditions

1. Tool profile = minimal.
2. Effective allowed tools include read but exclude write/edit/apply_patch/exec/process/Agent.

## Invariants

1. session-reviewer is structurally read-only.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Need to write output | Compose in chat; state-manager persists |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| session-reviewer session | Tools include read; exclude Write/Edit/Bash/Agent | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Tool profile = minimal | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/session-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.09.010 — composes with (8-dimensional analysis)
- BC-5.09.012 — composes with (no info walls)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#session-reviewer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/session-reviewer.md:21, 64-74, 199-204` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit T1 Read-Only rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads only |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure |

#### Refactoring Notes

No refactoring needed.
