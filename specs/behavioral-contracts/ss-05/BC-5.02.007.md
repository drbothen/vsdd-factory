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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:813
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

# Behavioral Contract BC-5.02.007: orchestrator: input-hash drift check before Phase 1/2/3/7 human approval

## Description

Before requesting human approval at phase gates 1, 2, 3, and 7, the orchestrator
MUST invoke `/vsdd-factory:check-input-drift`. Any DRIFT findings block approval
until resolved.

## Preconditions

1. Phase gate 1, 2, 3, or 7 is reached.
2. Human approval is about to be requested.

## Postconditions

1. `/vsdd-factory:check-input-drift` is invoked before the approval prompt.
2. If any DRIFT findings exist, approval is blocked until resolved.
3. Phase gate logs include the `check-input-drift` skill invocation entry preceding human approval.

## Invariants

1. Drift check is mandatory at the named gates.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Drift check returns DRIFT findings | Approval blocked; user must resolve |
| EC-002 | Drift check passes (no drift) | Approval prompt proceeds |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Phase 1 gate | Drift check invoked before approval | happy-path |
| Drift check returns FOUND | Approval blocked | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Phase 1/2/3/7 gate logs contain a check-input-drift entry preceding approval | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md`, `check-input-drift` skill |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.01.008 — composes with (human-approval shape)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#input-hash-drift`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md:121` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit gate-precondition rule in orchestrator agent body

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (drift skill invocation) |
| **Global state access** | reads spec frontmatter input-hashes |
| **Deterministic** | yes (given fixed inputs) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
