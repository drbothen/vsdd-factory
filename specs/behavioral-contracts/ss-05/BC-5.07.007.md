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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:277
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

# Behavioral Contract BC-5.07.007: codebase-analyzer: convergence requires binary novelty (SUBSTANTIVE / NITPICK)

## Description

Each deepening round MUST end with `Novelty: SUBSTANTIVE` or `Novelty: NITPICK`
(literal binary). Soft phrases (borderline, effectively, mostly) are forbidden.
The test: "Would removing this round's findings change how you'd spec the system?
If yes → SUBSTANTIVE. If no → NITPICK."

## Preconditions

1. codebase-analyzer concluding a deepening round.

## Postconditions

1. Every deepening output ends with `Novelty: SUBSTANTIVE` or `Novelty: NITPICK`.
2. No other tokens.

## Invariants

1. Novelty is binary.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | "Borderline" novelty | Force binary classification |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Output ends with `Novelty: SUBSTANTIVE` | Accepted | happy-path |
| Output ends with `Novelty: borderline` | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | validate-novelty-assessment.sh hook PASS on every deepening output | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/codebase-analyzer.md`, validate-novelty-assessment.sh |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.008 — composes with (convergence bounds)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#codebase-analyzer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/codebase-analyzer.md:322-332` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Novelty Decay Assessment rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (assessment line) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure |

#### Refactoring Notes

No refactoring needed.
