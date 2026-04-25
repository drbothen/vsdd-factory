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
extracted_from: "plugins/vsdd-factory/workflows/multi-repo.lobster"
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

# Behavioral Contract BC-5.34.005: multi-repo: failure semantics

## Description

`multi-repo.lobster` lines 27-30, 469 declare workflow defaults: on_failure=escalate, retries=2, timeout=4h. `integration-gate.fail_action: block` on 7 criteria. Primary blocking gates plus per-repo classification fan-out.

## Preconditions

1. Workflow invoked under default failure config.

## Postconditions

1. Step failures retry up to 2x then escalate, with 4h step timeout.
2. integration-gate blocks downstream on any of 7 criteria failing.

## Invariants

1. integration-gate is the sole primary-track blocking gate.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Cross-repo e2e fails | integration-gate blocks |
| EC-002 | Per-repo failure | Per-repo escalation, may halt project |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| All gates pass | Proceed | happy-path |
| One gate fails | Block | error |
| Per-repo fail | Escalate | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | integration-gate has 7 criteria | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.34.001 — identity

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#multi-repo-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/multi-repo.lobster` (lines 27-30, 469) |
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
