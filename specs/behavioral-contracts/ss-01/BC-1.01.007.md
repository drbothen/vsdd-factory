---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-deep-rust-tests.md]
input-hash: "[pending-recompute]"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:101"
subsystem: "SS-01"
capability: "CAP-TBD"
lifecycle_status: active
introduced: v1.0.0-beta.4
modified: [v1.1-async-semantics-F2-2026-05-07]
last_amended: "2026-05-07 (v1.0-feature-plugin-async-semantics-pass-1 cycle F2; see ADR-019)"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-1.01.007: factory-dispatcher::registry::parses_minimal_registry — minimum-viable registry parses with one hook entry, schema_version=2, enabled defaults to true

## Description

A minimum-viable registry TOML (`schema_version = 2`, single `[[hooks]]` entry with `name`, `event`, `tool`, `plugin`) parses through `Registry::parse_str` into `Ok(Registry)` with `hooks.len() == 1` and the entry's `enabled` defaulted to true. The `async` field defaults to false when absent.

## Preconditions

1. TOML input has `schema_version = 2` and a single `[[hooks]]` stanza declaring `name`, `event`, `tool`, `plugin`.

## Postconditions

1. `Registry::parse_str(toml)` returns `Ok(Registry)`.
2. `registry.schema_version == 2`.
3. `registry.hooks.len() == 1`.
4. The entry's `name`, `event`, `tool` fields populate as declared.
5. `enabled` defaults to true (since not specified).
6. `async` defaults to false (since not specified).

## Invariants

1. The minimum-viable registry shape is stable.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Minimum-viable TOML with one hook entry (`schema_version = 2`) | `Ok(Registry)` with one entry, enabled=true, async=false | happy-path |
| Minimum-viable TOML with `schema_version = 1` | `Err(RegistryError::SchemaVersion { got: 1, expected: 2 })` — v1 registries are rejected by v2 dispatcher | error (schema-mismatch) |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/registry.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/factory-dispatcher/src/registry.rs::tests::parses_minimal_registry` (lines 324–333) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `101` |

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

## Amendment 2026-05-07

**Cycle:** v1.0-feature-plugin-async-semantics-pass-1 (F2). **ADR:** ADR-019.

**Delta:** `schema_version` in the minimum-viable registry test fixture must be updated from 1 to 2. This BC previously asserted that `schema_version = 1` is the minimum-viable parse target. Per ADR-019, the dispatcher now requires `schema_version = 2`. Consequently:
- H1 title updated: "schema_version=1" → "schema_version=2".
- Precondition 1 updated: `schema_version = 2`.
- Postcondition 2 updated: `registry.schema_version == 2`.
- Postcondition 6 added: `async` defaults to false when absent (new field in v2).
- Canonical Test Vectors updated: the happy-path vector uses `schema_version = 2`; a new error vector documents that `schema_version = 1` is now rejected.
- Implementation note: the actual Rust test `parses_minimal_registry` must have its fixture TOML updated to use `schema_version = 2`; the test continues to assert `Ok(Registry)` on the updated fixture.
