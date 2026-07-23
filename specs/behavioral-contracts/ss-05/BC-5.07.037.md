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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:907
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

# Behavioral Contract BC-5.07.037: performance-engineer: every NFR-NNN gets a compliance row

## Description

For each NFR-NNN in `prd-supplements/nfr-catalog.md`, the agent MUST execute the
stated Validation Method and produce a compliance matrix row showing measured
value vs target with PASS/FAIL.

## Preconditions

1. performance-engineer evaluating NFRs.

## Postconditions

1. performance-report.md NFR compliance matrix has one row per NFR-NNN.
2. No NFRs missing.

## Invariants

1. Every NFR is exercised.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | NFR validation method N/A (e.g., requires production environment) | Document with explanation |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Report with row per NFR-NNN | Accepted | happy-path |
| Report missing an NFR row | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | NFR compliance matrix has every NFR-NNN | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/performance-engineer.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.07.036 — composes with (numerical thresholds)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#performance-engineer`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/performance-engineer.md:138-144` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit NFR Validation Method Execution Obligation

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
