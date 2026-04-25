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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md:459"
subsystem: "SS-04"
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

# Behavioral Contract BC-4.02.003: Adapter's plugin_config.script_path validation is checked BEFORE any subprocess invocation

## Description

The first action of `adapter_logic` is to inspect `payload.plugin_config.get("script_path")` and validate it. Missing → `HookResult::Error` with verbose hint mentioning `[hooks.config] script_path`. Not-a-string → Error mentioning "non-empty string". Empty string → Error mentioning "non-empty". `run_bash` is never called when validation fails (tests assert this with `panic!("must not run")` runner closures).

## Preconditions

1. Registry entry routes through legacy-bash-adapter.

## Postconditions

1. If `script_path` is missing → Error with verbose hint mentioning `[hooks.config] script_path`.
2. If `script_path` is not a string → Error mentioning "non-empty string".
3. If `script_path` is empty → Error mentioning "non-empty".
4. `run_bash` is NEVER invoked when any validation case fails.

## Invariants

1. Adapter never spawns a subprocess on invalid `script_path`.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | script_path is a JSON number | Error mentioning "non-empty string" |
| EC-002 | script_path is "" | Error mentioning "non-empty" |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| script_path missing in plugin_config | Error with `[hooks.config] script_path` hint; bash never invoked | error |
| TBD | TBD | edge-case |
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
| Architecture Module | SS-04 — `crates/hook-plugins/legacy-bash-adapter/src/lib.rs` (validation block) |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-plugins/legacy-bash-adapter/src/lib.rs:62-77` (validation), :218-247 (3 tests with `panic!("must not run")`) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts-deep-r1.md` line `459` |

#### Evidence Types Used

- guard clause (validation block)
- assertion (panic-on-run tests)

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
