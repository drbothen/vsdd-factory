---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-deep-rust-tests.md]
input-hash: "[pending-recompute]"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:376"
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

# Behavioral Contract BC-1.05.024: factory-dispatcher::host::read_file::rejects_file_exceeding_max_bytes — file size > max_bytes → OUTPUT_TOO_LARGE

## Description

A 2048-byte file under an allowed dir, called with `max_bytes=512`, returns `Err(OUTPUT_TOO_LARGE)`. The bound is enforced via file metadata BEFORE `read_to_end` runs (defensive).

## Preconditions

1. Target file size > `max_bytes`.
2. Path is on the allow-list.

## Postconditions

1. Returns `Err(OUTPUT_TOO_LARGE)`.

## Invariants

1. A plugin cannot accidentally OOM the dispatcher by reading a giant file.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | File size exactly == max_bytes | TBD per source |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| 2048-byte file, max_bytes=512 | `Err(OUTPUT_TOO_LARGE)` | error |
| TBD | TBD | happy-path |
| TBD | TBD | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/host/read_file.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/factory-dispatcher/src/host/read_file.rs::tests::rejects_file_exceeding_max_bytes` (lines 184–194) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `376` |

#### Evidence Types Used

- assertion (unit test)
- guard clause (size pre-check)

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
