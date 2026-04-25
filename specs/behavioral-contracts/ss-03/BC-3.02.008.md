---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: "1.4b"
inputs:
  - .factory/specs/behavioral-contracts/bc-id-mapping.md
  - .factory/phase-0-ingestion/pass-3-deep-rust-tests.md
input-hash: "fc47c40"
traces_to: .factory/specs/behavioral-contracts/bc-id-mapping.md
origin: brownfield
extracted_from: "pass-3-deep-rust-tests.md:934"
subsystem: "SS-03"
capability: "TBD"
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

# Behavioral Contract BC-3.02.008: sink-file::jsonl_append_preserves_three_events: 3 sequential events produce 3 lines in submission

> Section: File sink behavior
> Source BC (audit ID): BC-AUDIT-2375

## Description

Given 3 events submitted: plugin.invoked (a), plugin.completed (a), commit.made (sha=deadbeef). When flush().. Then File has 3 lines; first line's `type == "plugin.invoked"`; last line's `sha == "deadbeef"`. Append-only ordering is preserved.

## Preconditions

1. 3 events submitted: plugin.invoked (a), plugin.completed (a), commit.made (sha=deadbeef).

## Postconditions

1. File has 3 lines; first line's `type == "plugin.invoked"`; last line's `sha == "deadbeef"`. Append-only ordering is preserved.

## Invariants

1. Downstream consumers (jq, logsearch tooling) get reliable JSONL.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD — Phase 1.6b will enumerate edge cases from source review | TBD |

## Canonical Test Vectors

> Test vectors will be pulled from the pinned test case in Phase 1.6b.

| Input | Expected Output | Category |
|-------|----------------|----------|
| 3 events submitted: plugin.invoked (a), plugin.completed (a), commit.made (sha=deadbeef). | File has 3 lines; first line's `type == "plugin.invoked"`; last line's `sha == "deadbeef"`. Append-only ordering is pres | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Given 3 events submitted: plugin.invoked (a), plugin.completed (a), commit.made (sha=deadbeef). When flush().. Then File | manual (existing test: `crates/sink-file/src/lib.rs::tests::jsonl_append_preserves_three_events`) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (subsystem L2 spec pending) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-03 (File sink behavior) |
| Stories | TBD (Phase 2 story-writer pass) |

## Related BCs (Recommended)

- TBD — cross-references will be filled in Phase 1.6b after all per-BC files exist.

## Architecture Anchors (Recommended)

- `architecture/SS-03-observability-sinks.md` — section: File sink behavior

---

### Brownfield-Specific Sections

> This BC was extracted during Phase 0d brownfield ingestion (BC-AUDIT pass) and migrated to canonical one-per-file BC-S.SS.NNN format in Phase 1.4b.

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/sink-file/src/lib.rs` |
| **Source line(s)** | 651–680 |
| **Confidence** | high |
| **Original audit ID** | BC-AUDIT-2375 |
| **Pass-3 source** | `pass-3-deep-rust-tests.md:934` |
| **Extraction Date** | 2026-04-25 |
| **Pinned test** | ``crates/sink-file/src/lib.rs::tests::jsonl_append_preserves_three_events`` |
| **Test type** | unit |

#### Evidence Types Used

- **assertion**: pinned by Rust unit/integration test

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | TBD — Phase 1.6b will classify |
| **Global state access** | TBD |
| **Deterministic** | TBD |
| **Thread safety** | TBD |
| **Overall classification** | TBD |

#### Refactoring Notes

TBD — Phase 1.6b will produce refactoring guidance.

