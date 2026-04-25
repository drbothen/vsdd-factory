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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:247
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

# Behavioral Contract BC-5.07.004: code-reviewer: convergence verdict line is exact format

## Description

Each pass concludes with a verdict line that is one of two exact strings:
`CONVERGENCE_REACHED` or `findings remain -- iterate`. Free-form alternatives
are forbidden.

## Preconditions

1. code-reviewer concluding a pass.

## Postconditions

1. Pass report ends with one of two literal strings as a single line.

## Invariants

1. The verdict format is binary and exact.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Mixed verdict text | Rejected |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Final line `CONVERGENCE_REACHED` | Accepted | happy-path |
| Final line `Looks good!` | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Last line of every pass report is one of two exact strings | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/code-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.003 — composes with (multi-pass protocol)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#code-reviewer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/code-reviewer.md:105-109` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Convergence Verdict format

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | writes (verdict line) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (string emission) |

#### Refactoring Notes

No refactoring needed.
