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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:293
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

# Behavioral Contract BC-5.07.009: codebase-analyzer: state checkpoint at end of every pass

## Description

Every pass output MUST include a YAML state checkpoint block at the end specifying
`pass`, `status`, `files_scanned`, `timestamp`, `next_pass`, and `resume_from`
(if partial).

## Preconditions

1. codebase-analyzer concluding a pass.

## Postconditions

1. Every pass file contains a `## State Checkpoint` heading with a YAML block matching the schema.

## Invariants

1. Checkpoint is mandatory at end of every pass.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Pass interrupted | `status: partial`, `resume_from:` populated |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Pass file with `## State Checkpoint` block | Accepted | happy-path |
| Pass file without checkpoint | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every pass file has a State Checkpoint YAML block | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/codebase-analyzer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.005 — composes with (6-pass protocol)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#codebase-analyzer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/codebase-analyzer.md:156-162, 385-398` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit State Checkpointing section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (checkpoint block) |
| **Global state access** | reads pass progress |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
