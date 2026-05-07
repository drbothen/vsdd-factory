---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: codebase-analyzer
timestamp: 2026-04-25T00:00:00
phase: 1.4b
inputs: [bc-id-mapping.md, pass-3-behavioral-contracts.md]
input-hash: "[pending-recompute]"
traces_to: bc-id-mapping.md
origin: brownfield
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:30"
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

# Behavioral Contract BC-1.01.001: Registry rejects unknown schema version

## Description

`Registry::parse_str` rejects any TOML registry whose `schema_version` is not the canonical value. Unknown versions surface a typed error variant carrying the offending and expected versions.

## Preconditions

1. TOML input has `schema_version != REGISTRY_SCHEMA_VERSION` (currently 2 per BC-7.06.001 and ADR-019).

## Postconditions

1. `Registry::parse_str` returns `Err(RegistryError::SchemaVersion { got, expected })`.

## Invariants

1. `REGISTRY_SCHEMA_VERSION = 2` is the only accepted version (bumped from 1 per async-semantics cycle F2; see Amendment 2026-05-07 below).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | (No edge cases captured in Phase 0 extraction; to be added in Phase 1.5/test-writer pass) | TBD |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| TOML with `schema_version = 99` | `Err(RegistryError::SchemaVersion { got: 99, expected: 2 })` | error |
| TOML with `schema_version = 1` | `Err(RegistryError::SchemaVersion { got: 1, expected: 2 })` — v1 registries are rejected by v2 dispatcher | error (schema-mismatch) |
| TOML with `schema_version = 2` | `Ok(Registry)` | happy-path |
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
| **Path** | `crates/factory-dispatcher/src/registry.rs::tests::rejects_unknown_schema_version` (lines 387–404) |
| **Confidence** | HIGH (test asserts exact error variant) |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `30` |

#### Evidence Types Used

- assertion (unit test asserts exact error variant)

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

**Delta:** `REGISTRY_SCHEMA_VERSION` bumped from 1 to 2. This BC previously asserted that `schema_version = 1` is the only accepted version. Per ADR-019, the dispatcher now requires `schema_version = 2` to support the per-plugin `async: bool` field (see BC-7.06.001). Consequently:
- Precondition 1 updated: rejects `schema_version != 2` (not `!= 1`).
- Invariant 1 updated: `REGISTRY_SCHEMA_VERSION = 2` is the canonical value.
- Canonical Test Vectors updated: the happy-path vector is now `schema_version = 2`; `schema_version = 1` is now an error test vector (not a pass).
- The H1 title "Registry rejects unknown schema version" remains accurate — v1 is now an "unknown" version from the v2 dispatcher's perspective.

**No backward compatibility:** v1 registries are hard-rejected. No compat shim. No downgrade attempt.
