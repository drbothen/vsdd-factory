---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-05-07T00:00:00Z
last_amended: null
phase: F2
inputs:
  - .factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/F1-delta-analysis.md
  - plugins/vsdd-factory/hooks-registry.toml
  - crates/factory-dispatcher/src/registry.rs
  - crates/factory-dispatcher/src/routing.rs
input-hash: "[to-be-computed-by-state-manager]"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-002"
lifecycle_status: active
introduced: v1.0-feature-plugin-async-semantics-pass-1
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-1.14.001: factory-dispatcher::partition::sync_async_dispatch — matched plugins partitioned into sync_group (await-all, verdict gates Claude Code) and async_group (fire-and-forget, no verdict gate)

## Description

When the dispatcher is invoked by Claude Code with a synchronous hook envelope, matched plugins are partitioned into a sync group and an async group based on the per-plugin `async` field in the registry (`schema_version = 2`). Sync group plugins run in parallel within each priority tier and the dispatcher awaits all completions; any block verdict in the sync group causes the dispatcher to exit 2. Async group plugins are spawned fire-and-forget; their verdicts are logged to events-*.jsonl and never affect the dispatcher exit code. This is the canonical partition model introduced by ADR-019.

## Preconditions

1. Registry loaded from `hooks-registry.toml` with `schema_version = 2`. A registry with `schema_version != 2` causes a hard error before partitioning begins (see Error Paths: schema_version mismatch).
2. Dispatcher is invoked by Claude Code with a tool event on a synchronous hook envelope. All hook event types (PreToolUse, PostToolUse, Stop, SubagentStop, SessionStart, SessionEnd, WorktreeCreate, WorktreeRemove, PostToolUseFailure, PermissionRequest) are synchronous at the envelope layer per ADR-019. There are no per-event carve-outs.
3. At least one plugin entry in the registry matches the event (by event name and optional tool regex filter per BC-1.01.005).
4. Registry validation has already confirmed that no entry has both `on_error = "block"` and `async = true` (enforced at registry-load time; violation causes hard error before dispatch).

## Postconditions

1. All matched plugins are partitioned into exactly two disjoint groups:
   - `sync_group`: plugins where `async` field is absent or `async = false`.
   - `async_group`: plugins where `async = true`.
   No plugin appears in both groups (partition is a strict split).

2. Sync group execution:
   - Plugins in `sync_group` run in parallel within each priority tier (per ADR-008: sequential between tiers, parallel within a tier).
   - The dispatcher awaits completion of all sync group plugins before computing a verdict.
   - Total dispatcher latency for the sync path is bounded by `max(sync_plugin_durations_within_slowest_tier) + bounded overhead`, NOT by the sum of all sync plugin durations.

3. Sync group verdict:
   - If any sync group plugin returns `block_intent: true` (i.e., exit code 2 with `on_error = "block"`), the dispatcher exits 2.
   - If no sync group plugin returns a block verdict, the dispatcher exits 0.
   - Async group verdict never influences the dispatcher exit code under any condition.

4. Async group execution:
   - Each async group plugin is spawned as a fire-and-forget task (tokio task or equivalent).
   - The dispatcher does NOT await async group plugin completions.
   - Async group plugin verdicts (including any exit codes) are logged to `events-*.jsonl` via the standard FileSink path.
   - Async group results never reach Claude Code as a blocking signal.

5. The partition function `partition_plugins(matched_plugins, registry)` is pure and deterministic: given identical inputs it always produces identical `(sync_group, async_group)` splits. No side effects occur during partitioning.

## Invariants

1. **Partition purity**: No plugin appears in both `sync_group` and `async_group`. The union of the two groups equals the full set of matched plugins.
2. **Sync group execution is parallel within tier, not sequential**: Within a given priority tier, all sync group plugins run concurrently. Sequential execution within a tier is explicitly forbidden to prevent future performance regressions. The tier ordering (sequential between tiers) is preserved per ADR-008.
3. **Async group plugins are excluded from the tier ordering model entirely**: The ADR-008 tier model (sequential-between-tier, parallel-within-tier) applies exclusively to the sync group. Async group plugins have no guaranteed ordering relative to each other or to sync group execution.
4. **`on_error = "block"` implies `async = false`**: Any registry entry with `on_error = "block"` must have `async` absent or `async = false`. This is a load-time invariant enforced by `registry.rs::validate()`. If this invariant is violated in the registry file, the dispatcher hard-errors at startup with `E-REG-002` before dispatching any plugins. This invariant is also enforced by CI lint (VP-078; see BC-7.06.001).
5. **No downgrade attempt on schema_version mismatch**: A v1 registry loaded into a v2 dispatcher produces a deterministic error (see Error Paths). The dispatcher never silently accepts a v1 registry, never provides a compatibility shim, and never downgrades to v1 behavior.

## Error Paths

| Condition | Dispatcher Behavior |
|-----------|-------------------|
| `schema_version != 2` in registry (v1 or unknown version) | Dispatcher exits with `E-REG-001` (`dispatcher.schema_mismatch` event logged); exit code per existing dispatcher error convention (non-2, typically 0 per BC-1.08.001 fail-open). No partition attempted. No downgrade. No compat shim. |
| Sync plugin times out (exceeds `timeout_ms`) | Treated as block verdict: dispatcher exits 2. `plugin.timeout` event logged. Timeout is surfaced to Claude Code as a blocking signal (fail-closed for sync plugins). |
| Async plugin times out | Plugin process terminated. `plugin.timeout` event logged to `events-*.jsonl`. No impact on dispatcher exit code. |
| Sync plugin crashes (trap / panic) | Treated as non-block (exit 0) per existing crash semantics (BC-1.08.001: dispatcher exits 0 on engine errors unless block_intent recorded). `plugin.crashed` event logged. |
| Async plugin crashes | Crash logged to `events-*.jsonl` as `plugin.crashed`. No impact on dispatcher exit code. |
| Registry entry has `on_error = "block"` AND `async = true` | Hard error at registry-load time (`E-REG-002`). Dispatcher refuses to start. No plugins executed. |
| `partition_plugins` receives empty matched list | Both groups are empty; dispatcher exits 0 immediately. No plugins executed. |

## Related BCs

- BC-1.01.001 — depends on: registry schema version enforcement (schema_version = 2 required by this BC's Precondition 1; BC-1.01.001 now governs v2 rejection of v1)
- BC-1.01.007 — depends on: minimal registry parse (test vector references schema_version=1; amended per F2 cycle to reflect v2)
- BC-1.08.001 — composes with: dispatcher fail-open for non-blocking errors; this BC adds the clarification that sync-plugin timeouts are fail-closed (exit 2)
- BC-1.08.002 — extends: exit code 2 semantics now scoped to sync group only; async group blocks are explicitly excluded
- BC-7.06.001 — depends on: per-plugin `async` field schema in `hooks-registry.toml` v2; lint invariant on `on_error=block` ⇒ `async=false` enforced there

## Architecture Anchors

- `crates/factory-dispatcher/src/registry.rs` — `RegistryEntry.async` field; `validate()` enforcing Invariant 4; `REGISTRY_SCHEMA_VERSION = 2`
- `crates/factory-dispatcher/src/routing.rs` — `partition_plugins()` pure function (sync/async split)
- `crates/factory-dispatcher/src/engine.rs` (or equivalent dispatch loop) — sync group `run_tiers()` + async group `spawn_detached()` calls

## Story Anchor

TBD — Story A (Dispatcher schema v2 + partition runtime) per F1 Section 12 sketch

## VP Anchors

- VP-077 — Kani proof of `partition_plugins` purity and correctness: given same input always produces same `(sync_group, async_group)` split; partition is disjoint and exhaustive

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | All matched plugins have `async = false` (default) | `async_group` is empty; all plugins run in sync path; behavior identical to pre-partition model |
| EC-002 | All matched plugins have `async = true` | `sync_group` is empty; all plugins fire-and-forget; dispatcher exits 0 immediately after spawning async tasks |
| EC-003 | Mixed: some plugins async=true, some async=false | Partition splits correctly; sync group runs and determines exit code; async group fires-and-forgets |
| EC-004 | Sync plugin times out | Treated as block (exit 2); `plugin.timeout` logged; other sync plugins in the same tier already completed (parallel); async group unaffected |
| EC-005 | Async plugin emits a `block_intent = true` result | Result logged to events-*.jsonl as `plugin.async_block_discarded`; dispatcher exit code unchanged (0 or determined by sync group). Async plugins cannot produce `on_error = "block"` per Invariant 4; any block verdict from an async plugin indicates a classifier error. |
| EC-006 | Schema_version = 1 registry (v1 dispatcher upgrade scenario) | Hard error at load time; `dispatcher.schema_mismatch` logged; exit 0 (fail-open per BC-1.08.001); no plugins executed; no downgrade attempted |
| EC-007 | `partition_plugins` called with empty matched list | Returns `([], [])`; dispatcher proceeds to exit 0 without executing any plugins |
| EC-008 | Registry entry has `on_error = "block"` AND `async = true` | Hard error at registry-load time (`E-REG-002`); dispatcher refuses to start; `dispatcher.registry_invalid` logged |
| EC-009 | Two plugins at the same priority tier, one sync, one async | Sync plugin enters sync_group and runs in parallel with other same-tier sync plugins; async plugin enters async_group and is spawned separately without tier ordering |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Registry v2; 2 plugins both async=false; 1 returns Block | `sync_group=[p1,p2]`; `async_group=[]`; dispatcher exits 2 | happy-path (block) |
| Registry v2; 2 plugins both async=false; 0 returns Block | `sync_group=[p1,p2]`; `async_group=[]`; dispatcher exits 0 | happy-path (pass) |
| Registry v2; 1 async=false + 1 async=true; sync returns Block | `sync_group=[p1]`; `async_group=[p2]`; p2 spawned; dispatcher exits 2 | partition-with-block |
| Registry v2; 1 async=false + 1 async=true; sync returns Continue | `sync_group=[p1]`; `async_group=[p2]`; p2 spawned; dispatcher exits 0 | partition-pass |
| Registry v2; all async=true | `sync_group=[]`; `async_group=[p1,p2]`; both spawned; dispatcher exits 0 immediately | async-only |
| Registry v1 (schema_version=1) | Hard error `E-REG-001`; `dispatcher.schema_mismatch` logged; exit 0; no plugins run | schema-mismatch |
| Registry v2; entry has on_error=block AND async=true | Hard error `E-REG-002` at load time; `dispatcher.registry_invalid` logged; exit 0; no plugins run | invariant-violation |
| `partition_plugins([], registry)` | Returns `([], [])`; no plugins run; dispatcher exits 0 | empty-partition |
| Sync plugin times out | Plugin killed; `plugin.timeout` logged; dispatcher exits 2 (fail-closed) | sync-timeout-block |
| Async plugin times out | Plugin terminated; `plugin.timeout` logged to events-*.jsonl; dispatcher exit code unaffected | async-timeout-no-impact |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-077 | `partition_plugins(matched, registry)` is a pure fn — given same input, always produces same `(sync_group, async_group)` split; result is disjoint and exhaustive | kani-proof |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 |
| Capability Anchor Justification | CAP-002 ("Hook Claude Code tool calls with sandboxed WASM plugins") per capabilities.md §CAP-002 — this BC contracts the dispatcher's partitioned invocation model (sync-group gates Claude Code; async-group fires-and-forgets), which is the core mechanism by which sandboxed WASM plugins enforce `on_error = "block"` governance |
| L2 Domain Invariants | TBD — to be linked to DI entries for hook governance invariants |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/routing.rs` (`partition_plugins`), `crates/factory-dispatcher/src/engine.rs` (dispatch loop) |
| ADR | ADR-019 — Async Semantics at Registry Layer, Not Envelope Layer |
| Stories | TBD — Story A (Dispatcher schema v2 + partition runtime) |
| Cycle | v1.0-feature-plugin-async-semantics-pass-1 (F2) |

### Source Evidence

| Property | Value |
|----------|-------|
| **Path** | `.factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/F1-delta-analysis.md` Section 3 (Proposed State: Change 3) and Section 5 (New BCs Needed) |
| **Confidence** | HIGH — F1 explicitly called for this BC; motivated by 55 silently-discarded block verdicts in prism production logs |
| **Extraction Date** | 2026-05-07 |

#### Purity Classification

| Property | Assessment |
|----------|-----------|
| **I/O operations** | `partition_plugins` itself: NO (pure fn). Sync dispatch: YES (plugin invocation + file sink writes). Async spawn: YES (task spawn + eventual file sink writes). |
| **Global state access** | No global state in `partition_plugins`. Dispatch loop accesses shared plugin cache (Mutex-protected). |
| **Deterministic** | `partition_plugins` is fully deterministic. Dispatch outcomes depend on plugin runtime behavior. |
| **Thread safety** | `partition_plugins` is thread-safe (pure fn, no shared state). Async group spawn uses tokio task model. |
| **Overall classification** | `partition_plugins`: pure deterministic fn suitable for Kani proof. Dispatch loop: effectful with bounded I/O. |
