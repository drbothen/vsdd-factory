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
input-hash: "d714872"
traces_to: .factory/specs/behavioral-contracts/bc-id-mapping.md
origin: brownfield
extracted_from: "pass-3-deep-rust-tests.md:475"
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

# Behavioral Contract BC-3.05.001: factory-dispatcher::sinks::mod::load_builds_file_sink_from_parsed_config: ObservabilityConfig wit

> Section: Sink integration via dispatcher
> Source BC (audit ID): BC-AUDIT-2334

## Description

Given ObservabilityConfig with one `type="file"` stanza named "local-events", path_template, enabled=true. When `SinkRegistry::from_config(cfg)`.. Then Returns Ok; `reg.sinks().len() == 1`; `reg.sinks()[0].name() == "local-events"`. The sink-type dispatch correctly routes "file" to FileSink::new.

## Preconditions

1. ObservabilityConfig with one `type="file"` stanza named "local-events", path_template, enabled=true.

## Postconditions

1. Returns Ok; `reg.sinks().len() == 1`; `reg.sinks()[0].name() == "local-events"`. The sink-type dispatch correctly routes "file" to FileSink::new.

## Invariants

1. Operators write `type = "file"` and get a file sink in the registry.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | TBD — Phase 1.6b will enumerate edge cases from source review | TBD |

## Canonical Test Vectors

> Test vectors will be pulled from the pinned test case in Phase 1.6b.

| Input | Expected Output | Category |
|-------|----------------|----------|
| ObservabilityConfig with one `type="file"` stanza named "local-events", path_template, enabled=true. | Returns Ok; `reg.sinks().len() == 1`; `reg.sinks()[0].name() == "local-events"`. The sink-type dispatch correctly routes | happy-path |
| TBD | TBD | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6c) | Given ObservabilityConfig with one `type="file"` stanza named "local-events", path_template, enabled=true. When `SinkReg | manual (existing test: `crates/factory-dispatcher/src/sinks/mod.rs::tests::load_builds_file_sink_from_parsed_config`) |

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
| **Path** | `crates/factory-dispatcher/src/sinks/mod.rs` |
| **Source line(s)** | 262–282 |
| **Confidence** | high |
| **Original audit ID** | BC-AUDIT-2334 |
| **Pass-3 source** | `pass-3-deep-rust-tests.md:475` |
| **Extraction Date** | 2026-04-25 |
| **Pinned test** | ``crates/factory-dispatcher/src/sinks/mod.rs::tests::load_builds_file_sink_from_parsed_config`` |
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

