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
input-hash: "09d5d0c"
traces_to: .factory/specs/behavioral-contracts/bc-id-mapping.md
origin: brownfield
extracted_from: "pass-3-deep-rust-tests.md:607"
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

# Behavioral Contract BC-3.05.002: factory-dispatcher::sinks_file_integration::registry_fans_events_to_file_sinks_with_filter_and_ta

> Section: Sink integration via dispatcher
> Source BC (audit ID): BC-AUDIT-2346

## Description

Given Real observability-config.toml with two `type="file"` stanzas (one tagged, one filtered) and one unknown `type="datadog"` stanza. 10 events submitted via Router. When Router.flush() then files inspected.. Then local-events file has 10 lines (accepts everything) with `env=dev,host=ci` on every line; audit-filtered file has 8 lines (denies plugin.timeout + internal.sink_error); datadog stanza was skipped (per BC-AUDIT-042); post-shutdown submit is a no-op (no new lines).

## Preconditions

1. Real observability-config.toml with two `type="file"` stanzas (one tagged, one filtered) and one unknown `type="datadog"` stanza. 10 events submitted via Router.

## Postconditions

1. local-events file has 10 lines (accepts everything) with `env=dev,host=ci` on every line; audit-filtered file has 8 lines (denies plugin.timeout + internal.sink_error); datadog stanza was skipped (per BC-AUDIT-042); post-shutdown submit is a no-op (no new lines).

## Invariants

1. End-to-end sink fan-out + filter + tag enrichment is wired through Router (not just per-driver behavior).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD — Phase 1.6b will enumerate edge cases from source review | TBD |

## Canonical Test Vectors

> Test vectors will be pulled from the pinned test case in Phase 1.6b.

| Input | Expected Output | Category |
|-------|----------------|----------|
| Real observability-config.toml with two `type="file"` stanzas (one tagged, one filtered) and one unknown `type="datadog" | local-events file has 10 lines (accepts everything) with `env=dev,host=ci` on every line; audit-filtered file has 8 line | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Given Real observability-config.toml with two `type="file"` stanzas (one tagged, one filtered) and one unknown `type="da | manual (existing test: `crates/factory-dispatcher/tests/sinks_file_integration.rs::registry_fans_events_to_file_sinks_with_filter_and_tags`) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (subsystem L2 spec pending) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-03 (Sink integration via dispatcher) |
| Stories | TBD (Phase 2 story-writer pass) |

## Related BCs (Recommended)

- TBD — cross-references will be filled in Phase 1.6b after all per-BC files exist.

## Architecture Anchors (Recommended)

- `architecture/SS-03-observability-sinks.md` — section: Sink integration via dispatcher

---

### Brownfield-Specific Sections

> This BC was extracted during Phase 0d brownfield ingestion (BC-AUDIT pass) and migrated to canonical one-per-file BC-S.SS.NNN format in Phase 1.4b.

#### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/factory-dispatcher/tests/sinks_file_integration.rs` |
| **Source line(s)** | 68–166 |
| **Confidence** | high |
| **Original audit ID** | BC-AUDIT-2346 |
| **Pass-3 source** | `pass-3-deep-rust-tests.md:607` |
| **Extraction Date** | 2026-04-25 |
| **Pinned test** | ``crates/factory-dispatcher/tests/sinks_file_integration.rs::registry_fans_events_to_file_sinks_with_filter_and_tags`` |
| **Test type** | integration (end-to-end) |

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

