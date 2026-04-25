---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts.md]
input-hash: "[pending-recompute]"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:248"
subsystem: "SS-01"
capability: "CAP-TBD"
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

# Behavioral Contract BC-1.06.002: Daily rotation by event timestamp produces separate files per UTC date

## Description

When two events have timestamps on different UTC dates, each event lands in its own `dispatcher-internal-YYYY-MM-DD.jsonl` file. Daily rotation is by event ts (not write time).

## Preconditions

1. Two events with timestamps on different UTC dates are written.

## Postconditions

1. Each event lands in `dispatcher-internal-YYYY-MM-DD.jsonl` per its own date.

## Invariants

1. Rotation is by event ts in UTC; never by write time.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Events at midnight UTC boundary | Each lands in its respective UTC date file |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Events on 2026-04-25 and 2026-04-26 | Two files: `dispatcher-internal-2026-04-25.jsonl`, `dispatcher-internal-2026-04-26.jsonl` | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/internal_log.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `internal_log.rs::tests::daily_rotation_writes_separate_files_per_date` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `248` |

#### Evidence Types Used

- assertion (unit test)

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | TBD (Phase 1.6b will refine) |
| **Global state access** | TBD |
| **Deterministic** | TBD |
| **Thread safety** | TBD |
| **Overall classification** | TBD |

#### Refactoring Notes

(TBD — to be assessed in Phase 1.6b verification properties pass)
