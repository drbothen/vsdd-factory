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
extracted_from: ".factory/phase-0-ingestion/pass-3-deep-rust-tests.md:134"
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

# Behavioral Contract BC-1.01.010: factory-dispatcher::registry::defaults_applied_when_missing — omitted entry timeouts/fuel/priority/on_error fall through to Registry.defaults

## Description

When an entry omits `timeout_ms`, `fuel_cap`, `priority`, or `on_error`, the helper functions `priority(&defaults)` and `timeout_ms(&defaults)` return spec-pinned defaults: `timeout_ms=5_000`, `fuel_cap=10_000_000`, `priority=500`, `on_error=Continue`.

## Preconditions

1. Registry TOML has no `[defaults]` block.
2. A hook entry omits `timeout_ms`, `fuel_cap`, `priority`, and `on_error`.

## Postconditions

1. `entry.timeout_ms(&defaults) == 5_000`.
2. `entry.fuel_cap(&defaults) == 10_000_000`.
3. `entry.priority(&defaults) == 500`.
4. `entry.on_error == Continue`.

## Invariants

1. Operators can omit fields and rely on documented defaults.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `[defaults]` block partially overrides | Entry-level omissions inherit from `[defaults]`, then from spec sentinels |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Hook entry with no per-entry timeout/fuel/priority/on_error | timeout=5000, fuel=10M, priority=500, on_error=Continue | happy-path |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/registry.rs` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `crates/factory-dispatcher/src/registry.rs::tests::defaults_applied_when_missing` (lines 375–384) |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-deep-rust-tests.md` line `134` |

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
