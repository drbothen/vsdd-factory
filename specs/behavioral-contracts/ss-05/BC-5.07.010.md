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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:491
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

# Behavioral Contract BC-5.07.010: dtu-validator: never modifies clone source — spawns implementer for fixes

## Description

The dtu-validator is read-only with respect to clone implementation. When fidelity
is below threshold, it produces a fidelity-report.md with concrete deltas; the
orchestrator dispatches implementer to fix.

## Preconditions

1. dtu-validator dispatched against a DTU clone.

## Postconditions

1. Git diff shows no validator commits to `.factory/dtu-clones/[service]/` source files.
2. Only `fidelity-report.md` and `adversarial-config.yaml` are written.

## Invariants

1. Clone source is read-only for the validator.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Quick fix would unblock validation | Spawn implementer; do not patch directly |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Validator run | Diff confined to fidelity-report.md / adversarial-config.yaml | happy-path |
| Attempt to edit clone source | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Git diff after validator runs has zero entries in dtu-clones/source/ | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/dtu-validator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.09.002 — composes with (fidelity thresholds)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#dtu-validator`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/dtu-validator.md:38, 135` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit no-clone-modification rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (report only) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
