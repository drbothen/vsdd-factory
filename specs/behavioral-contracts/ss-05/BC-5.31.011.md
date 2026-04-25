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

# Behavioral Contract BC-5.31.011: code-delivery:per-story-adversarial-review

## Description

Step `per-story-adversarial-review` (line 101). Type: loop. max_iterations: 10. Depends: `[implement]`. Source 101-145. Behavior: spawns adversary on changed files only with extensive context exclusions (no implementer notes, red-gate-log, prior adversary history, semport history, holdout scenarios).

## Preconditions

1. implement step has completed.
2. Adversary agent is available with the configured information-asymmetry wall.

## Postconditions

1. Loop exits when adversary verdict is `CONVERGENCE_REACHED` or after 10 iterations.
2. Adversary is fed only diffed files plus permitted context.

## Invariants

1. Information-asymmetry wall is always applied (excludes implementer notes, red-gate-log, prior adversary history, semport history, holdout scenarios).
2. Loop is bounded (≤10 iterations).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Convergence reached at iter 1 | Loop exits early |
| EC-002 | No convergence by iter 10 | Loop terminates at cap; escalates |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Clean implementation | CONVERGENCE_REACHED quickly | happy-path |
| Persistent flaws | Iteration cap hit | edge-case |
| Wall leak (excluded path read) | Validation fails | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-001 | Loop terminates in ≤10 iterations | manual |
| VP-002 | Excluded paths never appear in adversary context | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| Architecture Module | SS-05 |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.31.010 — implement (depends on)
- BC-5.31.005 — failure semantics

## Architecture Anchors (Recommended)

- `architecture/ss-05-orchestration.md#code-delivery-workflow`
- `architecture/ss-05-orchestration.md#information-asymmetry-walls`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-001 — bounded loop
- VP-002 — wall enforcement

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/workflows/code-delivery.lobster` (lines 101-145) |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- type constraint: loop with max_iterations + exit_condition
- documentation: context.exclude entries

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (filtered context) |
| **Global state access** | reads filtered filesystem |
| **Deterministic** | no |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
