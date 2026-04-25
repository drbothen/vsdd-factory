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
extracted_from: "plugins/vsdd-factory/workflows/code-delivery.lobster"
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

# Behavioral Contract BC-5.31.012: code-delivery:e2e-tests

## Description

Step `e2e-tests` (line 151). Type: agent. Agent: e2e-tester. Depends: `[per-story-adversarial-review]`. Condition: `feature_type in ['ui', 'full-stack']`. Source 151-158.

## Preconditions

1. per-story-adversarial-review reached convergence.
2. feature_type is `ui` or `full-stack` (otherwise step is skipped).

## Postconditions

1. End-to-end tests have been authored and executed against the assembled system.
2. Result is recorded; failures escalate per default semantics.

## Invariants

1. Step does not run for non-UI / non-full-stack stories.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | feature_type=backend | Step skipped |
| EC-002 | E2E env unavailable | Step fails |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| ui story | E2E run | happy-path |
| backend story | Skipped | edge-case |
| Env error | Step fails | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Step gated by feature_type condition | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.011 — per-story-adversarial-review

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#code-delivery-workflow`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- (TBD — to be assigned in Phase 1.6c)

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 151-158) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- guard clause: conditional expression
- documentation

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | network calls (e2e harness) |
| **Global state access** | reads/writes test env |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
