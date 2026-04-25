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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:773
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

# Behavioral Contract BC-5.02.002: orchestrator: never delegates to itself

## Description

The orchestrator MUST NOT include `subagent_type: "vsdd-factory:orchestrator"`
in any Agent tool dispatch. Self-delegation would create infinite loops and
violates the coordinator pattern.

## Preconditions

1. Orchestrator dispatches a sub-agent via the Agent tool.

## Postconditions

1. The dispatch's `subagent_type` is NOT `vsdd-factory:orchestrator`.
2. Tool-call audit shows zero Agent dispatches with self-targeting `subagent_type`.

## Invariants

1. The orchestrator is a coordinator, not a worker — it never assigns work to a copy of itself.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Self-targeting dispatch attempted | Halt before dispatch |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Audit of orchestrator session dispatches | Zero `subagent_type=vsdd-factory:orchestrator` | happy-path |
| Self-targeting dispatch | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Tool-call audit shows zero self-targeting dispatches | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.02.001 — composes with (orchestrator never writes)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#orchestrator-dispatch-rules`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md:126` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit prohibition in orchestrator agent body

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none |
| **Global state access** | reads dispatch decision |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (logical guard) |

#### Refactoring Notes

No refactoring needed.
