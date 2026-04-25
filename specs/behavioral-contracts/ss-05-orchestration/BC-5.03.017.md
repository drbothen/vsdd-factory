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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1581
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

# Behavioral Contract BC-5.03.017: visual-reviewer: blank/missing demos report BLOCKED with satisfaction 0.0

## Description

Missing or blank recordings MUST be reported with status BLOCKED and satisfaction
0.0 — never given partial credit.

## Preconditions

1. visual-reviewer evaluating a recording.

## Postconditions

1. Any demo with file missing or zero-length → row has `Visual Satisfaction: 0.0`
   and `Status: BLOCKED`.
2. No partial-credit treatment of broken demos.

## Invariants

1. BLOCKED with 0.0 is a structural verdict, not optional.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Recording file present but corrupt | Reported BLOCKED with 0.0 |
| EC-002 | Recording is 1 frame long | TBD — likely BLOCKED |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Missing demo file | `Status: BLOCKED, Visual Satisfaction: 0.0` | happy-path |
| Zero-length demo | `Status: BLOCKED, Visual Satisfaction: 0.0` | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | All blank/missing demos reported BLOCKED with 0.0 | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/visual-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.03.016 — composes with (4-dimensional scoring)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#visual-reviewer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/visual-reviewer.md:84, 102-104, 162` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit BLOCKED rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (recording presence) + writes (review report) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
