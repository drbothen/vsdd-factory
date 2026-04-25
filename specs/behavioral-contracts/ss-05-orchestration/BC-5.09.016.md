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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1527
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

# Behavioral Contract BC-5.09.016: validate-extraction: max 3 refinement iterations (AgenticAKM)

## Description

Refinement caps at 3 iterations. Iteration 1 flags issues; Iteration 2 verifies
corrections; Iteration 3 final consistency check. Beyond 3 iterations is forbidden.

## Preconditions

1. validate-extraction running refinement loop.

## Postconditions

1. Validation report's "Refinement Iterations" field shows ≤3.

## Invariants

1. AgenticAKM bound (3 iterations) is canonical.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Iteration 3 still has issues | Stop and report; don't iterate further |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Refinement Iterations: 2 | Accepted | happy-path |
| Refinement Iterations: 4 | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Refinement Iterations field ≤3 | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/validate-extraction.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.09.014 — composes with (phase split)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#validate-extraction`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/validate-extraction.md:75-91` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Refinement Loop rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads/writes (report) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (counter) |

#### Refactoring Notes

No refactoring needed.
