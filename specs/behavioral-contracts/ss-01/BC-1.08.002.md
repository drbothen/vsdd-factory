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
extracted_from: ".factory/phase-0-ingestion/pass-3-behavioral-contracts.md:542"
subsystem: "SS-01"
capability: "CAP-002"
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

# Behavioral Contract BC-1.08.002: dispatcher exit code is 2 iff at least one sync-group plugin recorded a block_intent; async-group verdicts never affect exit code

## Description

`TierExecutionSummary.exit_code = 2` iff any plugin in the **sync group** (with `on_error=block`, `async=false`) returned a Block; otherwise 0. The dispatcher's process exit code is set from the sync group summary. Async group plugin verdicts are logged to events-*.jsonl and explicitly excluded from exit code computation. This BC is scoped to the sync group as of the async-semantics cycle (ADR-019); see Amendment 2026-05-07.

## Preconditions

1. Per-plugin outcomes are available after `execute_tiers` for the sync group.
2. Registry is schema_version = 2 (per BC-7.06.001 and BC-1.14.001 Precondition 1).
3. Async group plugins have been spawned fire-and-forget and are not included in the execution summary.

## Postconditions

1. `summary.exit_code = 2` iff at least one **sync group** plugin recorded a `block_intent`.
2. Otherwise `summary.exit_code = 0`.
3. Async group plugin outcomes do NOT contribute to `summary.exit_code` under any condition. If an async group plugin returns a block verdict (indicating a classifier error — async plugins must not be `on_error=block` per BC-7.06.001 Invariant 1), the verdict is logged as `plugin.async_block_discarded` and the exit code is unaffected.

## Invariants

1. Exit code 2 is the only blocking signal; non-block sync group outcomes always yield 0.
2. Async group plugin results are excluded from the exit code computation. This invariant must hold even if an async group plugin erroneously returns a block verdict.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | All sync group plugins crashed | exit 0 (crash without block_intent does not block) |
| EC-002 | Sync group empty (all matched plugins are async) | exit 0 immediately; no sync outcomes to aggregate |
| EC-003 | Async group plugin returns block verdict | Logged as `plugin.async_block_discarded`; exit code unchanged (0 or per sync group) |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| One sync group plugin Block + on_error=block | exit_code=2 | happy-path |
| All sync group plugins Continue | exit_code=0 | edge-case |
| Sync group empty; async group has one plugin | exit_code=0 immediately; async plugin spawned | async-only |
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

## Amendment 2026-05-07

**Cycle:** v1.0-feature-plugin-async-semantics-pass-1 (F2). **ADR:** ADR-019.

**Delta:** Exit code semantics are now scoped to the sync group only. This BC previously stated the exit code is 2 iff any plugin recorded a block_intent, implicitly assuming all dispatched plugins were sync (the pre-ADR-019 assumption). With the partition model (BC-1.14.001), only sync group plugins contribute to the exit code. Async group plugins are fire-and-forget and their verdicts never affect the exit code. Changes:
- H1 title updated to make sync-group scope explicit.
- Description updated to clarify sync group scoping.
- Preconditions 2 and 3 added.
- Postcondition 3 added: async group explicitly excluded.
- Invariant 2 added: async group exclusion is an invariant, not just an observation.
- New edge cases EC-002 and EC-003 added.
- New test vector: async-only scenario (sync group empty, exit 0).
- H1 title change reflects actual postcondition content (H1 is source of truth per bc_h1_is_title_source_of_truth policy).
