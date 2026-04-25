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
extracted_from: .factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:313
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

# Behavioral Contract BC-5.01.007: `loop:` blocks are bounded; require `max_iterations` and `exit_condition`

## Description

Steps with `type: loop` declare a `loop:` map containing `max_iterations: N`
(typical 10), `exit_condition: "<expr>"`, and inner `steps[]`. The orchestrator
iterates the inner steps until `exit_condition` fires OR `max_iterations` is
reached. Infinite loops are forbidden.

## Preconditions

1. Step has `type: loop`.

## Postconditions

1. `loop.max_iterations` is set to a positive integer (typical: `10`).
2. `loop.exit_condition` is a string expression (typical:
   `adversary.verdict == 'CONVERGENCE_REACHED'`,
   `spec_reviewer.verdict == 'APPROVED'`).
3. Orchestrator iterates inner `steps[]` until `exit_condition` evaluates true OR
   `max_iterations` is reached.
4. No infinite loops are permitted.

## Invariants

1. `max_iterations` is required and > 0.
2. `exit_condition` is required and non-empty.
3. Loop terminates within `max_iterations` regardless of `exit_condition` truthiness.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `exit_condition` never fires | Loop terminates at `max_iterations` |
| EC-002 | `max_iterations` omitted | Validation error |
| EC-003 | `exit_condition` omitted | Validation error |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `greenfield.lobster:262-266` (`max_iterations: 10, exit_condition: "adversary.verdict == 'CONVERGENCE_REACHED'"`) | Bounded adversary loop | happy-path |
| `greenfield.lobster:301-303` (spec_reviewer convergence) | Bounded review loop | happy-path |
| `greenfield.lobster:738-741` (pr_reviewer) | Bounded PR review loop | happy-path |
| Loop with no exit and `max_iterations: 10` | Terminates at iteration 10 | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every `loop:` block declares both `max_iterations` and `exit_condition` | manual |
| VP-TBD | No loop runs more than `max_iterations` iterations | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | orchestrator loop dispatcher |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.01.003 — composes with (step taxonomy: loop)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#loop-blocks`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `greenfield.lobster:262-266, 301-303, 738-741, 815-817, 895-897, 1063-1066`; `code-delivery.lobster:104-106` |
| **Confidence** | high (consistent shape across 7 distinct loop sites) |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit loop blocks in workflow YAML

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none (data shape) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (data) |

#### Refactoring Notes

No refactoring needed — schema-level invariant.
