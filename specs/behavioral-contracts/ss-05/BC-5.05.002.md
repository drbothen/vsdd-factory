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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:139
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

# Behavioral Contract BC-5.05.002: architect: every VP has a viable proof strategy and feasibility note

## Description

Each VP-NNN file MUST contain (a) a proof harness skeleton and (b) a feasibility
assessment. Initial status = `draft`.

## Preconditions

1. architect authoring a VP-NNN file.

## Postconditions

1. Every `verification-properties/VP-NNN.md` has a non-empty `proof_harness` block.
2. Every VP file has a `feasibility:` field.
3. Initial status is `draft`.
4. `VP-INDEX.md` lists all VPs.

## Invariants

1. No VP without a viable proof strategy.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Property believed unprovable | TBD — VP withdrawn before authoring or marked LOW feasibility |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| VP file with proof harness + feasibility | Accepted | happy-path |
| VP without proof_harness block | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Every VP-NNN.md has proof_harness and feasibility populated | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/architect.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.05.005 — composes with (VP-INDEX propagation)
- BC-5.07.019 — composes with (formal-verifier proof completion)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#architect`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/architect.md:35, 42` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit proof harness + feasibility rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (VP files) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
