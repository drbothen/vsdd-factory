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

# Behavioral Contract BC-5.31.024: code-delivery:wait-for-ci

## Description

Step `wait-for-ci` (line 367). Type: loop. max_iterations: 3. exit_condition: `ci.status == 'all_passed'`. Depends: `[pr-review-convergence, brownfield-full-regression]`. Source 367-389.

## Preconditions

1. PR has been pushed and CI has been triggered.
2. CI status API is reachable.

## Postconditions

1. Loop exits when CI reports all_passed or after 3 polls.
2. Failure path escalates per defaults.

## Invariants

1. Loop bounded ≤ 3 polls.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | CI flaky failure | Cap may be hit |
| EC-002 | CI offline | Step fails |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| CI passes immediately | Exit at iter 1 | happy-path |
| CI failure | Cap, escalate | error |
| Slow CI | Cap | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Bounded ≤ 3 iterations | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.021 — pr-review-convergence
- BC-5.31.025 — dependency-merge-check (downstream)

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
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 367-389) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- type constraint: loop with max_iterations + exit_condition

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | network calls (CI API) |
| **Global state access** | reads CI host |
| **Deterministic** | yes (given CI state) |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
