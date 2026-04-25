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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:743
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

# Behavioral Contract BC-5.07.032: implementer: HALT only on blocker, impossibility, or 3 consecutive failures

## Description

The implementer continues working without pause until story complete or one of
three explicit HALT conditions is met. Pausing for "review checkpoints" or
proposing to stop after one test passes is forbidden.

## Preconditions

1. implementer in TDD loop.

## Postconditions

1. Agent transcripts show no mid-story "should I continue?" prompts.
2. HALT only with explicit blocker / impossibility / 3-failures justification.

## Invariants

1. Continuous Execution is the default state.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Hit 3 consecutive failures on same test | HALT with explanation |
| EC-002 | Need clarifying input | HALT with blocker tag |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Story completed without mid-pauses | Accepted | happy-path |
| "Should I proceed?" mid-story | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | No mid-story "should I continue" prompts | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/implementer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.033 — composes with (status reporting tokens)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#implementer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/implementer.md:160-176` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Continuous Execution rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (source) |
| **Global state access** | reads pipeline state |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
