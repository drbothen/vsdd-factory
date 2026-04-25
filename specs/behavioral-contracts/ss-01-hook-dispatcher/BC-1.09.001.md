---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts-deep-r1.md]
input-hash: "[pending-recompute]"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:350"
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

# Behavioral Contract BC-1.09.001: PluginCache key is `path` only; invalidation is mtime-driven

## Description

`PluginCache::get_or_compile(path)` looks up the cache by `PathBuf` only. A hit returns the cached `Module` IF the stored mtime matches the current file mtime (`probe()` re-stat'd on every call). On mtime mismatch OR first sight, the cache reads + compiles via `Module::from_binary(&engine, &bytes)` and replaces the entry in place — cache size stays constant for the same path.

## Preconditions

1. Plugin path passed to `PluginCache::get_or_compile`.

## Postconditions

1. Cache key is `PathBuf` only.
2. On mtime match → return cached module.
3. On mtime mismatch OR first sight → read + compile, replace entry in place.

## Invariants

1. Cache size is bounded above by distinct paths seen.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Same path called twice with unchanged mtime | Same module returned (cache hit) |
| EC-002 | Same path with mtime changed | New compile; cache size unchanged |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Two consecutive calls, same path, same mtime | Cached module returned on second call | happy-path |
| Same path with bumped mtime | Recompiled; cache size stays at 1 | edge-case |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/plugin_loader.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugin_loader.rs:56-84` (probe() reads metadata + modified()`; entries.lock() check) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md` line `350` |

#### Evidence Types Used

- assertion (cache lookup logic)

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
