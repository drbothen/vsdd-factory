---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts-deep-r1.md]
input-hash: "[pending-recompute]"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:373"
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

# Behavioral Contract BC-1.09.004: Missing plugin path returns NotFound; corrupt bytes return Compile; IO errors carry path context

## Description

`PluginLoadError` has three distinct variants: `NotFound(PathBuf)`, `Io { path, source }`, and `Compile { path, source }`. Each carries the offending path so dispatcher diagnostics can name the bad plugin.

## Preconditions

1. Plugin path doesn't exist OR file is unreadable OR bytes are not valid wasm.

## Postconditions

1. Missing path → `PluginLoadError::NotFound(PathBuf)`.
2. Unreadable / I/O error → `PluginLoadError::Io { path, source }`.
3. Bad wasm bytes → `PluginLoadError::Compile { path, source }`.

## Invariants

1. Every error variant carries the offending path.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Path doesn't exist | `Err(NotFound(...))` | error |
| Bad wasm bytes | `Err(Compile { path, source })` | error |
| TBD | TBD | happy-path |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD (anchor in Phase 1.5) |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/plugin_loader.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/factory-dispatcher/src/plugin_loader.rs::PluginLoadError` (variants); `crates/factory-dispatcher/src/plugin_loader.rs::PluginCache::get_or_compile` (Io/Compile arms); `crates/factory-dispatcher/src/plugin_loader.rs::probe` (NotFound); `plugin_loader.rs::tests::not_found_error_for_missing_path`; `plugin_loader.rs::tests::corrupt_bytes_produce_compile_error` (tests) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md` line `373` |

#### Evidence Types Used

- type constraint (enum variants)
- assertion (tests for NotFound and Compile errors)

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


## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| v1.0 | 2026-04-25 | codebase-analyzer | Initial authoring. |
| v1.1 | 2026-05-08 | implementer | TD-VSDD-091 Chunk 6 — migrated 1 body cite: `plugin_loader.rs:18-34` / `:71-79` / `:93-106` + tests `:128-135/:174-181` → `plugin_loader.rs::PluginLoadError`, `::PluginCache::get_or_compile`, `::probe`, `::tests::not_found_error_for_missing_path`, `::tests::corrupt_bytes_produce_compile_error`. |
