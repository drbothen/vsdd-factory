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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:260"
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

# Behavioral Contract BC-1.06.004: prune_old removes only `dispatcher-internal-*.jsonl` files older than threshold

## Description

`prune_old` only unlinks files whose names match the `dispatcher-internal-*.jsonl` prefix+suffix and whose mtime is older than `max_age_days`. Unrelated files (e.g., `unrelated-2020-01-01.jsonl`) are preserved. Note the 30-day boundary race fix in commit `ba78e5f`.

## Preconditions

1. Log dir contains a mix of internal-log files (varied mtime) and unrelated files.

## Postconditions

1. Files matching `dispatcher-internal-*.jsonl` AND older than `max_age_days` are unlinked.
2. Unrelated files are preserved.

## Invariants

1. Prune is name- and age-bounded; never affects unrelated files.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | File age exactly at boundary | Avoid 30-day boundary race per commit `ba78e5f` fix |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `prune_old` against mixed dir, max_age_days=30 | Only matching+old files removed | happy-path |
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
| **Path** | `internal_log.rs::tests::prune_removes_files_older_than_max_age`; commit `ba78e5f` (boundary race fix) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `260` |

#### Evidence Types Used

- assertion (unit test)
- documentation (boundary race fix commit)

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
