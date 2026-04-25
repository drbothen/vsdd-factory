---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: phase-1-4b-agent-5
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [.factory/phase-0-ingestion/pass-3-deep-workflows.md]
input-hash: "99bbe9c"
traces_to: domain-spec/L2-INDEX.md
origin: brownfield
extracted_from: "plugins/vsdd-factory/workflows/discovery.lobster"
subsystem: "SS-05"
capability: "CAP-TBD"
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

# Behavioral Contract BC-5.32.005: discovery: failure semantics

## Description

`discovery.lobster` lines 19-22 declare workflow defaults: on_failure=escalate, retries=2, timeout=2h. No gates with `fail_action: block` — discovery findings are advisory, not blocking.

## Preconditions

1. Workflow invoked under default failure config.

## Postconditions

1. Step failures retry up to 2x then escalate.
2. No gate halts the entire workflow on failure.

## Invariants

1. Discovery is advisory: no blocking gate exists.
2. Default timeout per step is 2h.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Ingestion stream fails | Step escalates after retries; downstream proceeds with partial inputs per design |
| EC-002 | All ingestion streams fail | Synthesis runs with empty inputs |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Normal | Run completes | happy-path |
| Single ingestion failure | Escalate, others run | edge-case |
| Total failure | Escalation cascade | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | No `fail_action: block` in workflow | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.32.001 — identity

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#discovery-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-001

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/discovery.lobster` (lines 19-22) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none (structural) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | N/A |
| **Overall classification** | pure |

#### Refactoring Notes

No refactoring needed.
