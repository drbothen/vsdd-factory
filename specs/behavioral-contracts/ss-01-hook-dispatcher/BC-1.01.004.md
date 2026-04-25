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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:48"
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

# Behavioral Contract BC-1.01.004: Relative plugin paths resolve against registry file's parent directory

## Description

A registry entry with `plugin = "rel.wasm"` (no leading slash) loaded from a registry at `/foo/bar/hooks-registry.toml` resolves to `/foo/bar/rel.wasm`. Absolute paths are idempotent (no resolution applied).

## Preconditions

1. A registry entry has a relative `plugin` value (e.g., `"rel.wasm"`).
2. The registry was loaded from a known absolute path.

## Postconditions

1. The entry's resolved plugin path is `<registry-parent-dir>/<relative-plugin>`.
2. Absolute paths are idempotent (re-resolution leaves them unchanged).

## Invariants

1. Resolution is purely textual / path-join; no filesystem read is required.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Plugin path already absolute | Returned as-is |
| EC-002 | Resolution called twice | Same result both times (idempotent) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Registry at `/foo/bar/hooks-registry.toml`, entry `plugin = "rel.wasm"` | Resolved path `/foo/bar/rel.wasm` | happy-path |
| Registry at `/foo/bar/hooks-registry.toml`, entry `plugin = "/abs/p.wasm"` | `/abs/p.wasm` (idempotent) | edge-case |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/registry.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `registry.rs::tests::load_resolves_relative_plugin_paths_against_registry_dir`, `resolve_plugin_paths_is_idempotent_for_absolute_paths` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `48` |

#### Evidence Types Used

- assertion (two unit tests)

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
