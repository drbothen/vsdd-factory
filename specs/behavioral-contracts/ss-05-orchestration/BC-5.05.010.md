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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:331
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

# Behavioral Contract BC-5.05.010: consistency-validator: mis-anchoring is never an "Observation"

## Description

Criteria 70-73 (Semantic Anchoring Integrity) findings MUST be classified at
MEDIUM severity or higher and block convergence. They cannot be filed as
Observations or deferred.

## Preconditions

1. consistency-validator detects a mis-anchoring (criteria 70-73) finding.

## Postconditions

1. Finding has severity ≥ MEDIUM.
2. Finding is not classified as Observation, LOW, or status "deferred."
3. Finding blocks convergence.

## Invariants

1. Semantic anchoring integrity is non-negotiable.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Trivially small mis-anchor | Still ≥ MEDIUM (per rule) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Mis-anchor finding tagged MEDIUM | Accepted; blocks convergence | happy-path |
| Mis-anchor classified as Observation | Rejected | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | No criterion 70-73 finding has severity Observation/LOW or status deferred | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/consistency-validator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.04.003 — composes with (adversary mis-anchor rule)

## Architecture Anchors (Recommended)

- `architecture/SS-05-orchestration.md#consistency-validator`

## Story Anchor (Recommended)

TBD

## VP Anchors (Recommended)

- VP-TBD

---

### Brownfield-Specific Sections

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugins/vsdd-factory/agents/consistency-validator.md:318` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit anti-deferral rule

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (report) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | pure (classification rule) |

#### Refactoring Notes

No refactoring needed.
