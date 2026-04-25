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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:357"
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

# Behavioral Contract BC-1.09.002: PluginCache.get_or_compile is thread-safe via Mutex<HashMap>

## Description

`PluginCache` storage is `Mutex<HashMap<PathBuf, (SystemTime, Module)>>`. Two readers serialize through the mutex; the lock is held briefly (single get + early return) on cache hits, and released before the file read + compile on cache miss (lock taken again to insert). `expect("plugin cache poisoned")` would panic if a prior holder panicked, but that's a never-in-practice case.

## Preconditions

1. Concurrent calls to `get_or_compile`.

## Postconditions

1. Internal storage is `Mutex<HashMap<PathBuf, (SystemTime, Module)>>`.
2. Lock is held briefly on cache hits.
3. Lock is released before the file read + compile.
4. Lock is taken again to insert.

## Invariants

1. Mutex prevents data races; cache is thread-safe.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Prior lock holder panicked | `expect` panics on poisoned lock (never-in-practice) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Two concurrent get_or_compile calls (same path) | Both see same cached entry; no data race | happy-path |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/plugin_loader.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `plugin_loader.rs:43-46` (Mutex<HashMap>); :62-69 (lock-on-hit); :81-83 (insert) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md` line `357` |

#### Evidence Types Used

- type constraint (Mutex<HashMap>)
- assertion (lock acquisition pattern)

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
