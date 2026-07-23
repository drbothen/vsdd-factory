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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:1221
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

# Behavioral Contract BC-5.04.007: spec-reviewer: never re-reports adversary findings

## Description

The spec-reviewer runs AFTER the adversary pass and AFTER remediation. Findings
MUST be net-new — not restatements of adversary findings already addressed.
Focus is constructive improvement.

## Preconditions

1. spec-reviewer dispatched after adversary pass + remediation.

## Postconditions

1. SR-NNN findings have no overlap with prior ADV-NNN findings from the same phase.
2. Findings focus on constructive improvement (post-remediation).

## Invariants

1. Re-attack of adversary findings is prohibited.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Adversary finding was incompletely remediated | spec-reviewer may flag the remaining gap as a NEW finding (with distinct framing) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| spec-reviewer pass after adversary remediation | Net-new findings only | happy-path |
| spec-reviewer restating ADV-001 | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Zero SR-NNN findings overlap with ADV-NNN findings from the same phase | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/spec-reviewer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.05.012 — composes with (SR-NNN ID space)
- BC-5.05.013 — composes with (information wall)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#spec-reviewer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/spec-reviewer.md:41, 156` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit "review POST-remediation" rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads (specs + remediated artifacts) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
