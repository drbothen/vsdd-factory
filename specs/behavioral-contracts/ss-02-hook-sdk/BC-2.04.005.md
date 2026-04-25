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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:776"
subsystem: "SS-02"
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

# Behavioral Contract BC-2.04.005: hook-sdk::payload::plugin_config_passes_through_when_present — plugin_config field arrives populated when the registry sets it

## Description

When the envelope `{ ..., "plugin_config": {"script_path": "hooks/foo.sh"} }` is parsed, `payload.plugin_config.get("script_path").as_str() == Some("hooks/foo.sh")`. Plugin authors (especially the legacy-bash-adapter) can rely on `plugin_config` arriving populated when the registry declares `[hooks.config]`.

## Preconditions

1. Envelope contains a non-null `plugin_config` populated by the registry.

## Postconditions

1. After parsing, `payload.plugin_config.get("script_path").as_str() == Some("hooks/foo.sh")`.

## Invariants

1. Registry-supplied `plugin_config` flows through to the SDK-side payload unchanged.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `plugin_config` missing | Defaults to `Value::Null` per BC-2.01.004 |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `{ ..., "plugin_config": {"script_path": "hooks/foo.sh"} }` | `plugin_config.script_path == "hooks/foo.sh"` | happy-path |
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
| Architecture Module | SS-02 — `crates/hook-sdk/src/payload.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/hook-sdk/src/payload.rs::tests::plugin_config_passes_through_when_present` (lines 146–160) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `776` |

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
