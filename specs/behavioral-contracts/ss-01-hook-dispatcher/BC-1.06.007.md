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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:233"
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

# Behavioral Contract BC-1.06.007: factory-dispatcher::internal_log::writes_jsonl_events_with_expected_shape — 10 events with trace_id and iteration field write 10 JSONL lines into a single rotated file

## Description

With `InternalLog::new(tempdir)` and a fixed timestamp `2026-04-24T12:00:00`, 10 events written via `log.write(...)` produce a single rotated file `dispatcher-internal-2026-04-24.jsonl` with 10 valid JSON lines. Each line has `type="dispatcher.started"`, `schema_version=INTERNAL_EVENT_SCHEMA_VERSION (=1)`, `dispatcher_trace_id="trace-{i}"`, `iteration=i`, `ts` starting with `"2026-04-24"`, and an i64 `ts_epoch`.

## Preconditions

1. `InternalLog::new(tempdir)` constructs a log instance.
2. 10 events are written with fixed ts.

## Postconditions

1. Exactly one file `dispatcher-internal-2026-04-24.jsonl` exists.
2. The file has 10 lines.
3. Each line is valid JSON with the documented envelope shape.

## Invariants

1. JSONL shape is wire-stable.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| 10 events at fixed ts | 1 file, 10 valid JSONL lines, expected envelope | happy-path |
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
| **Path** | `crates/factory-dispatcher/src/internal_log.rs::tests::writes_jsonl_events_with_expected_shape` (lines 317–347) |
| **Confidence** | HIGH (filesystem-touching unit test) |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `233` |

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
