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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1313
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

# Behavioral Contract BC-5.10.003: state-manager: STATE.md cap of 200 lines (hook blocks at 500)

## Description

STATE.md MUST stay under 200 lines target / 500 lines hard cap (enforced by
validate-state-size hook). Burst narratives, full adversary findings, multiple
session checkpoints, resolved blocking issues, and accumulated lessons go to
cycle files instead — never STATE.md.

## Preconditions

1. state-manager updating STATE.md.

## Postconditions

1. STATE.md line count ≤ 200 typical, ≤ 500 always.
2. validate-state-size.sh hook on PostToolUse:Edit/Write rejects commits pushing
   STATE.md above 500 lines.
3. Burst narratives, adversary findings, multiple checkpoints, resolved blockers,
   and lessons live in cycle files, NOT STATE.md.

## Invariants

1. STATE.md is concise; cycle files hold detail.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | STATE.md naturally grows past 200 | Move details to cycle files |
| EC-002 | STATE.md commit pushes to 501 | Hook rejects |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| STATE.md at 180 lines | Accepted | happy-path |
| STATE.md at 510 lines | Hook FAIL | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | STATE.md ≤ 500 lines after every commit | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/state-manager.md`, validate-state-size.sh |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.10.002 — composes with (no spec/source writes)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#state-manager`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/state-manager.md:102-103, 140-146` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit STATE.md size cap + Anti-Patterns

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (STATE.md) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
