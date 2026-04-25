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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:134"
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

# Behavioral Contract BC-1.03.010: Per-plugin `plugin_config` spliced into HookPayload before invocation

## Description

Each plugin's per-entry `[hooks.config]` is spliced into the payload before invocation. The payload bytes are deep-cloned per plugin so that multiple registry entries pointing at the same `legacy-bash-adapter.wasm` (with different `[hooks.config]` blocks) each see only their own config. The 45-entry registry exercises this multi-instance pattern.

## Preconditions

1. Multiple registry entries route to the same plugin binary.
2. Each entry has a distinct `[hooks.config]` block.

## Postconditions

1. Each plugin invocation sees only its own entry's `plugin_config`.
2. Payload bytes are deep-cloned per plugin (no cross-contamination between adjacent plugin invocations in the tier).

## Invariants

1. Plugin-level config isolation is preserved across multi-instance dispatch.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Two entries with empty config | Both see empty/null config (no leak) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| 45 registry entries all routing through one .wasm | Each sees own plugin_config | happy-path |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/executor.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `executor.rs::execute_tier` clones `payload_value` per plugin and inserts `plugin_config` |
| **Confidence** | HIGH (tested implicitly by the 45-entry registry routing through one .wasm) |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `134` |

#### Evidence Types Used

- assertion (clone + splice logic)
- inferred (production registry's 45-entry pattern)

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
