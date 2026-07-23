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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:805
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

# Behavioral Contract BC-5.02.006: orchestrator: never sets runTimeoutSeconds below 300

## Description

Every Agent dispatch MUST have `runTimeoutSeconds` ≥ 300 (5 min); the default is
7200 (2 hours). Aggressive timeouts cause agents to die mid-work and are forbidden.

## Preconditions

1. The orchestrator dispatches a sub-agent via the Agent tool.

## Postconditions

1. The dispatch sets `runTimeoutSeconds` ≥ 300.
2. Default `runTimeoutSeconds` is 7200 (2 hours) unless explicitly overridden.

## Invariants

1. No Agent dispatch has `runTimeoutSeconds` < 300.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Dispatch sets `runTimeoutSeconds: 60` | Audit failure |
| EC-002 | Dispatch omits `runTimeoutSeconds` | Default 7200 applied |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Audit of orchestrator dispatches | Zero dispatches with `runTimeoutSeconds < 300` | happy-path |
| Dispatch with `runTimeoutSeconds: 200` | Audit failure | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Tool-call audit shows minimum runTimeoutSeconds = 300 across all dispatches | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.02.001 — composes with (orchestrator dispatch rules)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#dispatch-timeouts`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/orchestrator/orchestrator.md:128` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit minimum-timeout rule in orchestrator agent body

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none |
| **Global state access** | reads dispatch parameters |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (numerical guard) |

#### Refactoring Notes

No refactoring needed.
