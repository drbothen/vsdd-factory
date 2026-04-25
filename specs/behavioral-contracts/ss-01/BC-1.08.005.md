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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:560"
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

# Behavioral Contract BC-1.08.005: dispatcher injects CLAUDE_PLUGIN_ROOT into base_host_ctx.plugin_root

## Description

At dispatcher startup, `base_host_ctx.plugin_root` is set to `${CLAUDE_PLUGIN_ROOT}` if defined; otherwise an empty `PathBuf`.

## Preconditions

1. Dispatcher startup.

## Postconditions

1. `base_host_ctx.plugin_root = ${CLAUDE_PLUGIN_ROOT}` if defined.
2. Else empty `PathBuf`.

## Invariants

1. `plugin_root` always has a defined value (never panics).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `CLAUDE_PLUGIN_ROOT` unset | empty `PathBuf` |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `CLAUDE_PLUGIN_ROOT=/p` set | plugin_root=/p | happy-path |
| `CLAUDE_PLUGIN_ROOT` unset | plugin_root=PathBuf::new() | edge-case |
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
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/main.rs:143-145` |
| Stories | TBD (re-anchor in Phase 1.8 from S-N.MM stories) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `main.rs:143–145` |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `560` |

#### Evidence Types Used

- assertion (plugin_root assignment)

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
