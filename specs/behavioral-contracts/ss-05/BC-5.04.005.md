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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:109
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

# Behavioral Contract BC-5.04.005: adversary: max 3 self-validation iterations per pass (AgenticAKM)

## Description

Within a single pass, the adversary self-validates findings (evidence check,
actionability check, duplication check) at most 3 times before shipping the
report. Diminishing returns beyond 3 iterations validated by AgenticAKM study
(29 repositories).

## Preconditions

1. adversary self-validating findings within a pass.

## Postconditions

1. Pass report includes a self-validation iteration counter ≤ 3.

## Invariants

1. The 3-iteration cap is empirically grounded.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | 3 iterations still finding issues | Ship report; do not exceed 3 |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Pass with 2 self-validation iterations | Accepted | happy-path |
| Pass with 4 iterations | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | self-validation iteration counter ≤ 3 in every pass report | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/adversary.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.04.004 — composes with (3-clean / 10-max bounds)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#adversary-bounds`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/adversary.md:78-86` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit AgenticAKM iteration bound

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | none (counter logic) |
| **Global state access** | reads iteration counter |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure |

#### Refactoring Notes

No refactoring needed.
