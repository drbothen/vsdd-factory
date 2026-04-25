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
extracted_from: .factory/phase-0-ingestion/pass-3-deep-agents.md:323
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

# Behavioral Contract BC-5.05.009: consistency-validator: gate fails when blocking findings exist

## Description

The validation gate result MUST be FAIL if any CRITICAL-severity criterion has
unresolved violations, regardless of other passing criteria. Reports also produce
a 0-100% consistency score.

## Preconditions

1. consistency-validator runs the gate evaluation.

## Postconditions

1. Gate row shows FAIL when any CRITICAL-severity finding lacks remediation.
2. PASS only when all CRITICAL criteria have status PASS.
3. Reports produce a 0-100% consistency score.

## Invariants

1. CRITICAL severity is a binary gate.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | All CRITICAL fixed; some MEDIUM remain | Gate may PASS (per criteria definitions) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Report with one CRITICAL violation | Gate FAIL | error |
| Report with all CRITICALs resolved | Gate PASS | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-TBD | Gate FAIL when any CRITICAL is unresolved | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | DI-TBD |
| Architecture Module | `plugins/vsdd-factory/agents/consistency-validator.md` |
| Stories | TBD |

## Related BCs (Recommended)

- BC-5.05.007 — composes with (80-criteria coverage)

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
| **Path** | `plugins/vsdd-factory/agents/consistency-validator.md:99, 113-118` |
| **Confidence** | high |
| **Extraction Date** | 2026-04-25 |

#### Evidence Types Used

- documentation: explicit Success Criteria section

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | reads + writes (report) |
| **Global state access** | none |
| **Deterministic** | yes |
| **Thread safety** | unknown |
| **Overall classification** | effectful shell |

#### Refactoring Notes

No refactoring needed.
