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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:66"
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

# Behavioral Contract BC-1.02.001: HookPayload requires non-empty event_name and session_id

## Description

`HookPayload::from_bytes` rejects JSON envelopes with empty or missing `event_name` or `session_id`. Errors are returned as `PayloadError::MissingField(<which>)` or `PayloadError::Json(_)` for malformed input.

## Preconditions

1. JSON envelope is supplied to `HookPayload::from_bytes`.
2. Either `event_name` is empty/missing, OR `session_id` is empty/missing, OR JSON is malformed.

## Postconditions

1. Missing field → `Err(PayloadError::MissingField(<field>))`.
2. Empty field → Same error variant per field.
3. Malformed JSON → `Err(PayloadError::Json(_))`.

## Invariants

1. A successfully-parsed payload always has non-empty `event_name` and `session_id`.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | event_name is empty string | Err |
| EC-002 | session_id is empty string | Err |
| EC-003 | malformed JSON | Err with Json variant |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Envelope without event_name | `Err(MissingField("event_name"))` | error |
| Envelope with empty session_id | `Err(MissingField("session_id"))` | error |
| Valid envelope | Ok(HookPayload) | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/payload.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `payload.rs::tests::{rejects_missing_event_name, rejects_empty_event_name, rejects_empty_session_id, rejects_malformed_json}` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `66` |

#### Evidence Types Used

- assertion (4 unit tests)
- guard clause (validation in from_bytes)

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
