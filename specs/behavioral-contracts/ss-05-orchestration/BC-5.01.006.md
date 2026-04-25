---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md]
input-hash: "a022087"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: .factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:306
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

# Behavioral Contract BC-5.01.006: Failure handling — `on_failure: escalate` is the workflow default; per-step override via `on_failure: <action>`; `gate.fail_action: block` is the explicit blocking shape

## Description

Workflow-level `defaults.on_failure: escalate` is the default action when a step
fails (bubble to operator). Steps may override per-step. Gate-type steps use
`fail_action: block` to halt the workflow on criteria miss. Steps may declare
`optional: true` to make failure non-blocking.

## Preconditions

1. A step declares failure handling, OR the step has `type: gate`.

## Postconditions

1. `on_failure: escalate` (workflow default) bubbles failure to the operator.
2. Per-step override via `on_failure: <action>` is permitted.
3. Gate steps use `fail_action: block` to halt the workflow on criteria miss.
4. Steps may declare `optional: true` to make their failure non-blocking.

## Invariants

1. Observed `on_failure` values: `escalate`. Observed gate `fail_action` values: `block`.
2. Sample is limited to greenfield + brownfield + first 150 LOC of code-delivery; the
   broader retry/timeout-action universe MAY exist in the 13 unread workflows.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Step declares `optional: true` and fails | Workflow continues |
| EC-002 | Gate step's `fail_action: block` triggers | Workflow halts |
| EC-003 | `on_failure` value other than `escalate` | TBD — expected universe unverified |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `greenfield.lobster:27` (`on_failure: escalate`) | Workflow default | happy-path |
| `greenfield.lobster:75` (`optional: true`) | Failure non-blocking | happy-path |
| `greenfield.lobster:257, 495, 614` (gate: `fail_action: block`) | Workflow halts on criteria miss | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every step's `on_failure` value is within the documented enum | manual |
| VP-TBD | Every gate step's `fail_action` is set | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | orchestrator failure-handling logic |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.01.002 — composes with (defaults block)
- BC-5.01.003 — composes with (step taxonomy: gate)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#failure-handling`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `greenfield.lobster:27, 75, 257, 495, 614` |
| **Confidence** | HIGH for `escalate` + `block` + `optional: true`; MEDIUM for the universe of values |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: failure-handling fields explicit in workflow YAML

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none (data shape) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (data) |

#### Refactoring Notes

Sample broadening recommended — read the 13 unread workflows to confirm the failure-action universe.
