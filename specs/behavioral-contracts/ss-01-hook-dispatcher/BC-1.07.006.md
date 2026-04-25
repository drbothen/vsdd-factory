---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-deep-rust-tests.md]
input-hash: "[pending-recompute]"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:596"
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

# Behavioral Contract BC-1.07.006: factory-dispatcher::loads_legacy_registry::every_entry_carries_a_script_path — every entry has plugin_config.script_path matching `hooks/<name>.sh`

## Description

Integration test: For each entry in the production registry, `entry.config_as_json().get("script_path")` returns a non-empty string matching `hooks/*.sh`. Catches generator drift — if `generate-registry-from-hooks-json.sh` ever drops `script_path`, the dispatcher refuses the payload at runtime (per BC-4.01.001).

## Preconditions

1. Production registry is loaded.

## Postconditions

1. Each entry's `config_as_json().get("script_path")` returns a non-empty string.
2. The string matches `hooks/*.sh` shape.

## Invariants

1. Schema-time invariant for legacy-bash-adapter routing is CI-enforced.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Production registry | Every entry has `plugin_config.script_path = hooks/<name>.sh` | happy-path |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/tests/loads_legacy_registry.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/factory-dispatcher/tests/loads_legacy_registry.rs::every_entry_carries_a_script_path` (lines 78–105) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `596` |

#### Evidence Types Used

- assertion (integration test)

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
