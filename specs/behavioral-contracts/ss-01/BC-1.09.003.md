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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:364"
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

# Behavioral Contract BC-1.09.003: PluginCache has no eviction policy — entries live for the dispatcher's process lifetime

## Description

`PluginCache` has no LRU, TTL, or memory-pressure eviction. The cache grows monotonically with distinct plugin paths seen. Acceptable today because (1) the dispatcher process is per-event (cold start; the cache never builds up beyond one event's plugin set) and (2) the plugin set is bounded (45 entries, all currently the same `legacy-bash-adapter.wasm`).

## Preconditions

1. Long-running dispatcher process (today's deployment is per-event, short-lived).

## Postconditions

1. No eviction occurs.
2. Cache grows monotonically with distinct paths seen.

## Invariants

1. Cache lifetime equals dispatcher process lifetime.
2. The only mutation is `get_or_compile` (no `evict`, `clear`, `prune`).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Same path twice | Cache size remains 1 (no growth) |
| EC-002 | mtime change for same path | Cache size remains 1 (replace) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Same path twice | Cache size stays 1 | happy-path |
| Bumped mtime | Cache size stays 1 (replace) | edge-case |
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
| **Path** | `plugin_loader.rs:43-91` (only `get_or_compile`); tests `compiles_on_first_use_and_caches`, `invalidates_on_mtime_change` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md` line `364` |

#### Evidence Types Used

- assertion (no eviction methods present)

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
