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
extracted_from: "plugins/vsdd-factory/workflows/planning.lobster"
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

# Behavioral Contract BC-5.35.005: planning: failure semantics

## Description

`planning.lobster` lines 16-19, 53 declare workflow defaults: on_failure=escalate, retries=2, timeout=2h. `environment-gate.fail_action: block`, `routing-gate.fail_action: block`.

## Preconditions

1. Workflow invoked under default failure config.

## Postconditions

1. Step failures retry up to 2x then escalate, with 2h step timeout.
2. Both `environment-gate` and `routing-gate` block downstream on failure.

## Invariants

1. Two blocking gates exist: environment-gate and routing-gate.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Tooling missing | environment-gate blocks |
| EC-002 | Routing decision absent | routing-gate blocks |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Healthy env + clear routing | Pass | happy-path |
| Missing tooling | Block | error |
| Ambiguous routing | Block | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | environment-gate fail_action = block | manual |
| VP-002 | routing-gate fail_action = block | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.35.001 — identity

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#planning-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)
- VP-002

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/planning.lobster` (lines 16-19, 53) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: declarative gate fail_action

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
