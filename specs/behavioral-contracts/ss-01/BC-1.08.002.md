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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:542"
subsystem: "SS-01"
capability: "CAP-002"
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

# Behavioral Contract BC-1.08.002: dispatcher exit code is 2 iff at least one block_intent recorded

## Description

`TierExecutionSummary.exit_code = 2` iff any plugin (with `on_error=block`) returned a Block; otherwise 0. The dispatcher's process exit code is set from the summary.

## Preconditions

1. Per-plugin outcomes are available after `execute_tiers`.

## Postconditions

1. `summary.exit_code = 2` iff at least one plugin recorded a `block_intent`.
2. Otherwise `summary.exit_code = 0`.

## Invariants

1. Exit code 2 is the only blocking signal; non-block outcomes always yield 0.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | All plugins crashed | exit 0 (crash without block_intent does not block) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| One plugin Block + on_error=block | exit_code=2 | happy-path |
| All plugins Continue | exit_code=0 | edge-case |
| TBD | TBD | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — to be assigned in Phase 1.6b) | | |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 — this BC contracts the dispatcher's exit code protocol (0=allow, 2=block), which is the fundamental signaling contract for Claude Code WASM hook gating |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/executor.rs` |
| Stories | S-2.07 (Wave 9 SS-01 straggler re-anchor) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `executor.rs::execute_tiers` final branch + tests |
| **Confidence** | HIGH |
| **Extraction Date** | 2026-04-25 |
| **Extracted from** | `.factory/phase-0-ingestion/pass-3-behavioral-contracts.md` line `542` |

#### Evidence Types Used

- assertion (executor logic + tests)

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
