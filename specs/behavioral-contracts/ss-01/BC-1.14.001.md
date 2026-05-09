---
document_type: behavioral-contract
level: L3
version: "1.10"
status: draft
producer: product-owner
timestamp: 2026-05-07T00:00:00Z
last_amended: 2026-05-08
phase: F2
inputs:
  - .factory/cycles/v1.0-feature-plugin-async-semantics-pass-1/F1-delta-analysis.md
  - plugins/vsdd-factory/hooks-registry.toml
  - crates/factory-dispatcher/src/registry.rs
  - crates/factory-dispatcher/src/partition.rs
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

# BC-1.14.001: factory-dispatcher::partition::sync_async_dispatch — matched plugins partitioned into sync_group (await-all, verdict gates Claude Code) and async_group (fire-and-forget with bounded ASYNC_DRAIN_WINDOW_MS, no verdict gate)

## Description

When the dispatcher is invoked by Claude Code with a synchronous hook envelope, matched plugins are partitioned into a sync group and an async group based on the per-plugin `async` field in the registry (`schema_version = 2`). Sync group plugins run in parallel within each priority tier and the dispatcher awaits all completions; any block verdict in the sync group causes the dispatcher to exit 2. Async group plugins are spawned fire-and-forget; their verdicts are logged to events-*.jsonl and never affect the dispatcher exit code. This is the canonical partition model introduced by ADR-019.

## Preconditions

1. Registry loaded from `hooks-registry.toml` with `schema_version = 2`. A registry with `schema_version != 2` causes a hard error before partitioning begins (see Error Paths: schema_version mismatch).
2. Dispatcher is invoked by Claude Code with a tool event on a synchronous hook envelope. All hook event types (PreToolUse, PostToolUse, Stop, SubagentStop, SessionStart, SessionEnd, WorktreeCreate, WorktreeRemove, PostToolUseFailure, PermissionRequest) are synchronous at the envelope layer per ADR-019. There are no per-event carve-outs.
3. At least one plugin entry in the registry matches the event (by event name and optional tool regex filter per BC-1.01.005).
4. Registry validation has already confirmed that no entry has both `on_error = "block"` and `async = true` (enforced at registry-load time; violation causes hard error before dispatch) (per BC-7.06.001 Invariant 1).

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
   - **Each async group plugin is spawned via `tokio::spawn` (or equivalent independent task)**. The dispatcher does NOT call `execute_tiers` or any tier-ordered execution path for async group plugins. Each plugin becomes an independent task; partial completion is therefore observable.
   - **Plugin results MAY arrive in any order during the drain window.** Specifically: after `sync_group` completes and `async_group` plugins are spawned, their terminal events MAY arrive in any order produced by the tokio scheduler before the drain timer fires. (Concurrent reception during `sync_group` execution is structurally impossible — see PC6.) The dispatcher collects completed results via a shared channel or equivalent collection mechanism.
   - The dispatcher does NOT await async group plugin completions beyond the drain window defined below.
   - Async group plugin verdicts (including any exit codes) are logged to `events-*.jsonl` via the standard FileSink path.
   - Async group results never reach Claude Code as a blocking signal.
   - **Async-task drain window**: After `sync_group` completes, the dispatcher waits up to `ASYNC_DRAIN_WINDOW_MS` (per DI-019) for spawned async tasks to emit terminal events to FileSink. Tasks that complete within the drain window emit cleanly. Tasks that do not complete within the drain window are forcibly terminated. The drain is implemented via `tokio::select!` (or equivalent) over the per-task result channels and a drain timer — NOT via `tokio::time::timeout(execute_tiers(...))`, which is an all-or-nothing await that discards completed-task results on truncation. The drain window is a bounded constant per DI-019 — async tasks DO NOT extend dispatcher latency beyond `max(sync_plugin_durations) + ASYNC_DRAIN_WINDOW_MS`. The drain is purely for terminal-event emission visibility (e.g., `plugin.timeout`, `plugin.async_block_discarded`), not for completing arbitrary async work.
   - **Async plugin lifetime is best-effort**: Tasks not done within `ASYNC_DRAIN_WINDOW_MS` (per DI-019) are terminated (truncated telemetry remains an explicit acceptable cost for tasks slower than the drain). Telemetry plugins classified `async = true` (e.g., `capture-commit-activity`) accept this truncation risk.
   - **Async-group plugins MUST NOT be subject to tier ordering**: Implementations MUST NOT call `group_by_priority` on async-group plugins (see also Invariant 3).

5. The partition function `partition_plugins(matched_plugins, registry)` is pure and deterministic: given identical inputs it always produces identical `(sync_group, async_group)` splits. No side effects occur during partitioning.

6. Async group spawn ordering:
   - Async group plugins are spawned **only after** `sync_group` execution completes (per ADR-019 §Decision 3 pseudocode). The dispatcher does not interleave async spawns with sync execution. Async spawning is a post-sync-group step.

## Invariants

1. **Partition purity**: No plugin appears in both `sync_group` and `async_group`. The union of the two groups equals the full set of matched plugins.
2. **Sync group execution is parallel within tier, not sequential**: Within a given priority tier, all sync group plugins run concurrently. Sequential execution within a tier is explicitly forbidden to prevent future performance regressions. The tier ordering (sequential between tiers) is preserved per ADR-008.
3. **Async group plugins are excluded from the tier ordering model entirely**: The ADR-008 tier model (sequential-between-tier, parallel-within-tier) applies exclusively to the sync group. Async group plugins have no guaranteed ordering relative to each other or to sync group execution. **Code-level note**: Implementations MUST NOT call `group_by_priority` on async-group plugins. Passing async-group plugins through tier-sorted execution paths (e.g., `execute_tiers`) is a spec violation regardless of whether the result is wrapped in a timeout. The required spawn pattern is `tokio::spawn` per plugin with result collection via a channel and `tokio::select!` drain timer (see PC4).
4. **`on_error = "block"` implies `async = false`**: Any registry entry with `on_error = "block"` must have `async` absent or `async = false`. This is a load-time invariant enforced by `registry.rs::validate()`. If this invariant is violated in the registry file, the dispatcher hard-errors at startup with `E-REG-002` before dispatching any plugins. This invariant is also enforced by CI lint (VP-078; see BC-7.06.001).
5. **No downgrade attempt on schema_version mismatch**: A v1 registry loaded into a v2 dispatcher produces a deterministic error (see Error Paths). The dispatcher never silently accepts a v1 registry, never provides a compatibility shim, and never downgrades to v1 behavior.

## Constant Reference

The async-task drain window is `ASYNC_DRAIN_WINDOW_MS` per **DI-019** (`invariants.md` §Dispatcher Timing Invariants). This BC references the DI; the canonical value and its rationale live in the domain spec. Do not inline the constant value here — consult DI-019 for the authoritative value and any future env-var override policy.

The total dispatcher wall-clock latency upper bound is therefore:
`max(sync_plugin_durations_within_slowest_tier) + ASYNC_DRAIN_WINDOW_MS + bounded_overhead`

(See DI-019 for the `ASYNC_DRAIN_WINDOW_MS` value and configurability notes. VP-079 fixture timing assertions must also anchor to DI-019.)

## Error Paths

| Condition | Dispatcher Behavior |
|-----------|-------------------|
| `schema_version != 2` in registry (v1 or unknown version) | Dispatcher exits with `E-REG-001` (`dispatcher.schema_mismatch` event logged); **exit code 2 (fail-closed)**. Explicit stderr emission of the schema-mismatch diagnostic. No partition attempted. No downgrade. No compat shim. **Exception to BC-1.08.001 fail-open**: schema-version mismatch is the one startup error that is fail-closed, because silent acceptance of a mismatched schema is the root-cause bug ADR-019 was created to fix. |
| Sync plugin times out (exceeds `timeout_ms`) | Treated as block verdict: dispatcher exits 2. `plugin.timeout` event logged. Timeout is surfaced to Claude Code as a blocking signal (fail-closed for sync plugins). |
| Async plugin times out | Plugin process terminated. `plugin.timeout` event logged to `events-*.jsonl`. No impact on dispatcher exit code. |
| Sync plugin crashes (trap / panic) | Treated as non-block (exit 0) per existing crash semantics (BC-1.08.001: dispatcher exits 0 on engine errors unless block_intent recorded). `plugin.crashed` event logged. |
| Async plugin crashes | Crash logged to `events-*.jsonl` as `plugin.crashed`. No impact on dispatcher exit code. |
| Registry entry has `on_error = "block"` AND `async = true` | Hard error at registry-load time (`E-REG-002`). Dispatcher refuses to start. No plugins executed. |
| `partition_plugins` receives empty matched list | Both groups are empty; dispatcher exits 0 immediately. No plugins executed. |
| Async plugin returns exit code 2 | `plugin.async_block_discarded` event logged with `reason: "async_plugin_block_verdict_discarded"`; dispatcher exit code unaffected (async group verdicts never reach Claude Code); result discarded per BC-7.06.001 Invariant 1 (block + async coexistence forbidden by load-time validation, so no async plugin can have a legitimate block intent). See also EC-005. |

## Related BCs

- BC-1.01.001 — depends on: registry schema version enforcement (schema_version = 2 required by this BC's Precondition 1; BC-1.01.001 now governs v2 rejection of v1)
- BC-1.01.007 — depends on: minimal registry parse (test vector references schema_version=1; amended per F2 cycle to reflect v2)
- BC-1.08.001 — composes with: dispatcher fail-open for non-blocking errors; this BC adds the clarification that sync-plugin timeouts are fail-closed (exit 2)
- BC-1.08.002 — extends: exit code 2 semantics now scoped to sync group only; async group blocks are explicitly excluded
- BC-7.06.001 — depends on: per-plugin `async` field schema in `hooks-registry.toml` v2; lint invariant on `on_error=block` ⇒ `async=false` enforced there

## Architecture Anchors

- `crates/factory-dispatcher/src/registry.rs` — `RegistryEntry.async` field; `validate()` enforcing Invariant 4; `REGISTRY_SCHEMA_VERSION = 2`
- `crates/factory-dispatcher/src/partition.rs` — `partition_plugins()` pure function (sync/async split); Kani proof harnesses in `partition.rs::kani_proofs` module (`#[cfg(kani)] mod kani_proofs`)
- `crates/factory-dispatcher/src/engine.rs` (or equivalent dispatch loop) — sync group `run_tiers()` + async group `spawn_detached()` calls

## Story Anchor

TBD — single story per ADR-019 §6 (no phased rollout, user decision 2026-05-07)

## VP Anchors

- VP-077 — Kani proof of `partition_plugins` purity and correctness: given same input always produces same `(sync_group, async_group)` split; partition is disjoint and exhaustive

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | All matched plugins have `async = false` (default) | `async_group` is empty; all plugins run in sync path; behavior identical to pre-partition model |
| EC-002 | All matched plugins have `async = true` | `sync_group` is empty; all plugins fire-and-forget; dispatcher exits 0 immediately after spawning async tasks |
| EC-003 | Mixed: some plugins async=true, some async=false | Partition splits correctly; sync group runs and determines exit code; async group fires-and-forgets |
| EC-004 | Sync plugin times out | Treated as block (exit 2); `plugin.timeout` logged; other sync plugins in the same tier already completed (parallel); async group unaffected |
| EC-005 | Async plugin returns exit code 2 | **`block_intent` definition**: `block_intent := (plugin.exit_code == 2 && plugin.config.on_error == "block")`. Since Invariant 4 guarantees no async plugin has `on_error = "block"`, `block_intent` is **FALSE by definition** for any async plugin that exits 2. The result is logged to events-*.jsonl as `plugin.async_block_discarded` with `reason: "async_plugin_block_verdict_discarded"` for diagnostic visibility; dispatcher exit code unchanged (determined solely by sync group). This edge case documents the theoretical path where an async plugin's exit code 2 is structurally incapable of being a block intent. |
| EC-006 | Schema_version = 1 registry (v1 dispatcher upgrade scenario) | **Fail-closed**: Hard error at load time; `dispatcher.schema_mismatch` logged; **exit code 2** (non-zero, not exit 0 — this is the explicit exception to BC-1.08.001 fail-open); explicit stderr diagnostic emitted; no plugins executed; no downgrade attempted. |
| EC-007 | `partition_plugins` called with empty matched list | Returns `([], [])`; dispatcher proceeds to exit 0 without executing any plugins |
| EC-008 | Registry entry has `on_error = "block"` AND `async = true` | Hard error at registry-load time (`E-REG-002`); dispatcher refuses to start; `dispatcher.registry_invalid` logged |
| EC-009 | Two plugins at the same priority tier, one sync, one async | Sync plugin enters sync_group and runs in parallel with other same-tier sync plugins; async plugin enters async_group and is spawned separately without tier ordering |
| EC-010 | Async task emits terminal event during drain window | Event reaches FileSink before dispatcher exit; consumer sees the terminal event (e.g., `plugin.timeout`, `plugin.async_block_discarded`) in `events-*.jsonl` |
| EC-011 | Async task exceeds `ASYNC_DRAIN_WINDOW_MS` (per DI-019) drain window | Task forcibly terminated; no terminal event emitted; consumer sees absence of the terminal event, interpretable as "task did not complete within the drain window"; truncated telemetry is an accepted cost |
| EC-012 | Drain truncation with partial completions: drain timer fires while ≥1 async plugins are still executing AND ≥1 async plugins have already completed within the window (e.g., 2 of 3 plugins finished in 50ms; 3rd takes 200ms; drain fires at `ASYNC_DRAIN_WINDOW_MS`) | **Completed plugins' terminal events MUST emit before dispatcher exit**: results already received via the result channel before the drain timer fires MUST be flushed to FileSink. In-flight plugins' events MAY be lost (no guarantee; truncated telemetry accepted). This is the key difference from the all-or-nothing `tokio::time::timeout(execute_tiers(...))` anti-pattern — which discards ALL results on truncation regardless of completion state. Cite: F-P1-010 |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| Registry v2; 2 plugins both async=false; 1 returns Block | `sync_group=[p1,p2]`; `async_group=[]`; dispatcher exits 2 | happy-path (block) |
| Registry v2; 2 plugins both async=false; 0 returns Block | `sync_group=[p1,p2]`; `async_group=[]`; dispatcher exits 0 | happy-path (pass) |
| Registry v2; 1 async=false + 1 async=true; sync returns Block | `sync_group=[p1]`; `async_group=[p2]`; p2 spawned; dispatcher exits 2 | partition-with-block |
| Registry v2; 1 async=false + 1 async=true; sync returns Continue | `sync_group=[p1]`; `async_group=[p2]`; p2 spawned; dispatcher exits 0 | partition-pass |
| Registry v2; all async=true | `sync_group=[]`; `async_group=[p1,p2]`; both spawned; dispatcher exits 0 immediately | async-only |
| Registry v1 (schema_version=1) | Hard error `E-REG-001`; `dispatcher.schema_mismatch` logged; **exit 2** (fail-closed exception to BC-1.08.001); explicit stderr diagnostic; no plugins run | schema-mismatch (fail-closed) |
| Registry v2; entry has on_error=block AND async=true | Hard error `E-REG-002` at load time; `dispatcher.registry_invalid` logged; **exit 2** (fail-closed); explicit stderr diagnostic naming offending plugin; no plugins run | invariant-violation (fail-closed) |
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
| L2 Domain Invariants | DI-014 — Schema version mismatch is a hard load error (the fail-closed schema_version=2 enforcement in this BC is the BC-1 enforcement arm of DI-014; the fail-closed behavior was amended per ADR-019 to extend to registry schema_version); DI-017 — Wire-format field exclusivity: `trace_id` is the exclusive wire-format key for trace correlation; no alias fields may appear alongside it (BC-1.14.001 is the primary dispatch-path enforcer of this wire-format exclusivity invariant; DI-017 v1.1 extended BC range to include BC-1.14.001 in its Stage 1 amendment); DI-019 — `ASYNC_DRAIN_WINDOW_MS` (PC4 async-task drain window is bounded by DI-019; the canonical constant value lives in invariants.md §DI-019, not in this BC) |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/partition.rs` (`partition_plugins`), `crates/factory-dispatcher/src/engine.rs` (dispatch loop) |
| ADR | ADR-019 — Async Semantics at Registry Layer, Not Envelope Layer |
| Stories | TBD — single story per ADR-019 §6 (no phased rollout, user decision 2026-05-07) |
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

## Amendment 2026-05-08 (v1.9 → v1.10 — F-P20-001: §Architecture Anchors Kani line-range migrated to module anchor)

**Driver:** F-P20-001 pass-20 extended prose-form sweep — §Architecture Anchors cited `Kani proof harnesses at lines 148-224`, a line-range reference subject to refactor drift. Stable anchor identified: `partition.rs::kani_proofs` module (`#[cfg(kani)] mod kani_proofs` at partition.rs line 112 in HEAD 34a9f5c2). The module name is stable across future additions or reordering of individual proof functions within the module.

**Change made:**
- §Architecture Anchors second bullet: `Kani proof harnesses at lines 148-224` replaced with `Kani proof harnesses in \`partition.rs::kani_proofs\` module (\`#[cfg(kani)] mod kani_proofs\`)`.
- Frontmatter `version:` bumped `"1.9"` → `"1.10"`; `last_amended:` unchanged (2026-05-08, same date).

**Source-of-truth verification (POLICY 4/5):** `grep -n "^#\[cfg(kani)\]" crates/factory-dispatcher/src/partition.rs` → line 111; `mod kani_proofs` → line 112. Module contains VP-077 proof harnesses: `proof_vp077_totality`, `proof_vp077_disjointness`, `proof_vp077_async_field_respected`. Module anchor is stable regardless of internal proof function count or reordering.

## Amendment 2026-05-08 (v1.8 → v1.9 — F5 fix-burst-2 F-P2-004: PC4 concurrent-reception clause corrected)

**Driver:** F5 pass-2 finding F-P2-004 — PC4 contained an internal contradiction with PC6. PC4 previously stated async plugin results "may be received while `sync_group` is still running OR during the drain window," but PC6 establishes that async group plugins are spawned **only after** `sync_group` execution completes. Reception during `sync_group` execution is therefore structurally impossible; the "during sync_group running" clause was unreachable and misleading.

**Change made:**
- PC4 second bullet amended: the "MAY be received concurrently ... while `sync_group` is still running OR during the drain window" phrasing replaced with "MAY arrive in any order during the drain window" with an explicit parenthetical cross-referencing PC6 as the authority establishing that concurrent reception during `sync_group` is structurally impossible.

No postcondition semantics were changed. The spawn-after-sync ordering (PC6) is unmodified. This is a spec-consistency correction only.

## Amendment 2026-05-08 (v1.7 → v1.8 — F5 pass-1 path-A: deferred DI-017 reciprocal citation)

**Driver:** F5 pass-1 Stage-1 deferred fix — bidirectional traceability pairing with `invariants.md` DI-017 v1.1. DI-017 v1.1 (Stage 1 amendment) extended its BC range to include BC-1.14.001 as the primary dispatch-path enforcer of wire-format field exclusivity. This amendment adds the reciprocal citation in BC-1.14.001's Traceability §L2 Domain Invariants row so the pairing is complete in both directions.

**Change made:**
- Traceability §L2 Domain Invariants: DI-017 added between DI-014 and DI-019 entries, with full justification text explaining BC-1.14.001's role as the dispatch-path enforcer of wire-format `trace_id` exclusivity per DI-017 v1.1.
- Frontmatter `version:` bumped `"1.7"` → `"1.8"`; `last_amended:` unchanged (2026-05-08, same date).

No substantive behavioral changes. This is a traceability-completeness fix only.

## Amendment 2026-05-08 (v1.6 → v1.7 — F5 pass-1 fix-burst)

Addresses adversary F5-pass-1 findings F-P1-004, F-P1-006, and F-P1-010.

**F-P1-004 (anchor module fix)**: BC-1.14.001 cited `routing.rs` as the home of `partition_plugins` at three sites (frontmatter `inputs:`, Architecture Anchors, Traceability Architecture Module). The implementation placed `partition_plugins` in `partition.rs` (confirmed at `crates/factory-dispatcher/src/partition.rs:90`). All three cite sites updated to `partition.rs`. Zero `routing.rs` references remain in this file.

**F-P1-006 (drain semantics strengthening — spawn-based pattern)**: PC4 tightened to explicitly require:
- Each async-group plugin is spawned via `tokio::spawn` (or equivalent independent task).
- Plugin results MAY be received concurrently while `sync_group` runs OR during the drain window.
- The drain is implemented via `tokio::select!` over per-task result channels and a drain timer — NOT via `tokio::time::timeout(execute_tiers(...))`.
- Async-group plugins MUST NOT be subject to tier ordering (restated near PC4 for clarity; see Invariant 3 code-level note).

**F-P1-010 (drain truncation with partial completions — EC-012 added)**: When the drain timer fires while ≥1 async plugins are still executing AND ≥1 async plugins have already completed, completed plugins' terminal events MUST emit before dispatcher exit; in-flight plugins' events MAY be lost. EC-012 documents this partial-completion behavior and distinguishes it from the all-or-nothing `tokio::time::timeout(execute_tiers(...))` anti-pattern.

**Invariant 3 code-level note**: Added explicit prohibition on calling `group_by_priority` on async-group plugins. The code-level note names the required spawn pattern (`tokio::spawn` per plugin, channel collection, `tokio::select!` drain timer) to prevent regression.

**Changes made:**
- Frontmatter `inputs:` field: `routing.rs` → `partition.rs`
- Frontmatter `version:` bumped to `"1.7"`; `last_amended:` updated to `2026-05-08`
- PC4: spawn-based semantics added; three new bullets; anti-pattern sentence naming `tokio::time::timeout(execute_tiers(...))` explicitly called out
- Invariant 3: code-level note added prohibiting `group_by_priority` on async plugins
- Architecture Anchors: `routing.rs` bullet → `partition.rs` with Kani harness line reference
- Traceability Architecture Module: `routing.rs` → `partition.rs`
- EC-012 added: drain truncation with partial completions (F-P1-010)

## Amendment 2026-05-07 (v1.5 → v1.6 — F2 pass-7 F-P7-004: redundant parenthetical removed)

Removed `(per DI-019;` from the L2 Domain Invariants DI-019 cell. The phrase was redundant with the surrounding `DI-019 —` prefix. No substantive content changed.

## Amendment 2026-05-07 (v1.4 → v1.5 — F2 pass-4 F-P4-005)

Removed residual inline `100 ms` literal values from three live body locations to enforce the §Constant Reference rule and preserve DI-019's canonical ownership of the drain-window value.

**Changes made:**
- PC4 drain-window bullet: `(per DI-019, default 100 ms)` → `(per DI-019)`.
- §Constant Reference narrative: `the canonical value (100 ms) and its rationale` → `the canonical value and its rationale` (the parenthetical was redundant given the surrounding DI-019 citation).
- Traceability §L2 Domain Invariants: `ASYNC_DRAIN_WINDOW_MS = 100 ms` → `ASYNC_DRAIN_WINDOW_MS (per DI-019)`.

**No substantive content changed.** Postconditions, invariants, error paths, test vectors, and EC-011 expected behavior are identical to v1.4. This is a consistency-only correction driven by adversary finding F-P4-005.

## Amendment 2026-05-07 (v1.3 → v1.4 — F2 pass-3 user-correction: ASYNC_DRAIN_WINDOW_MS lifted to DI-019)

**Structural correction requested by user after reviewing v1.3.**

v1.3 added a "Constant Definitions" section inlining `ASYNC_DRAIN_WINDOW_MS = 100 ms`. The user identified this as architecturally incorrect: constants that bound dispatcher behavior across multiple BCs and VPs belong in domain invariants, not inside a single BC's body.

**Changes made:**
- "Constant Definitions" section replaced by "Constant Reference" section. The inline `= 100` value and unit columns are removed. The section now states: "The async-task drain window is `ASYNC_DRAIN_WINDOW_MS` per DI-019." The total latency formula is preserved as a reading aid.
- PC4 prose amended: both occurrences of `ASYNC_DRAIN_WINDOW_MS` now cite "per DI-019" to make the DI reference explicit in the prose.
- EC-011: "(100 ms)" parenthetical replaced with "(per DI-019)" to avoid re-inlining the value.
- Traceability L2 Domain Invariants updated: DI-019 added alongside DI-014.

**Semantic invariant:** The constant value (100 ms), the drain-window mechanism, all postconditions, and all test vectors are unchanged. This is a placement-only structural correction.

**Cross-burst dependencies (architect, unchanged from v1.3 obligation):**
- VP-079 must anchor fixture timing to DI-019 (not the now-removed BC constant table).
- ADR-019 §Consequences should cite DI-019.

## Amendment 2026-05-07 (v1.2 → v1.3 — F2 pass-3 fix burst)

Addresses adversary pass-3 findings F-P3-002 and F-P3-007.

**F-P3-002 / F-P3-007 (Design conflict: PC4 vs VP-079 Scenarios 1+4)**: The previous PC4 stated the dispatcher "does NOT await async tasks" and "the dispatcher process exits as soon as `sync_group` completes." VP-079 Scenarios 1 and 4 test for terminal events (`plugin.timeout`, `plugin.async_block_discarded`) emitted by async tasks before dispatcher exit — which was impossible under the strict fire-and-forget model. This is a design conflict, not a test configuration issue.

**Resolution: bounded async-task drain window.** After `sync_group` completes, the dispatcher now waits up to `ASYNC_DRAIN_WINDOW_MS` (100 ms) for spawned async tasks to emit terminal events to FileSink. This preserves the core guarantee: user-facing latency is gated exclusively by `sync_group` with only a deterministic, bounded constant added. Async tasks that complete within the drain window emit cleanly. Tasks that exceed the window are forcibly terminated (truncated telemetry accepted).

**Changes made:**
- H1 updated to mention `ASYNC_DRAIN_WINDOW_MS` drain semantics.
- Postcondition 4 amended: drain window specified, best-effort wording updated to reference `ASYNC_DRAIN_WINDOW_MS`.
- New section "Constant Definitions" added: `ASYNC_DRAIN_WINDOW_MS = 100 ms` with VP-079 referenceable anchor.
- EC-010 added: async task emits terminal event during drain window.
- EC-011 added: async task exceeds drain window (forcible termination).
- Total latency upper bound formula updated to include `ASYNC_DRAIN_WINDOW_MS`.

**Architect obligations (not in scope for this burst):**
- VP-079 must be updated to reference `ASYNC_DRAIN_WINDOW_MS` and anchor its Property to amended PC4.
- VP-079 fixture timing must be adjusted so async events can emit within the 100 ms drain.
- ADR-019 §Consequences should note the drain window as part of the latency-budget discussion.

## Amendment 2026-05-07 (v1.2 — F2 pass-2 fix burst)

Addresses adversary pass-2 findings F-P2-005, F-P2-009, F-P2-015.

**F-P2-005 (Postcondition renumbering)**: Postconditions were out of monotonic order (1, 2, 3, 4, 6, 5) due to insertion artifact during pass-1 burst. Renumbered to strict 1-2-3-4-5-6. New order: 1=sync_group/async_group partition, 2=sync execution, 3=sync verdict, 4=async execution, 5=partition purity/determinism, 6=async spawn ordering. The apologetic note about "PC5 vs PC6" gap removed (now obsolete). Ripple-check: no sibling BCs were found to cite specific BC-1.14.001 postcondition numbers; VP citations are by property description not number.

**F-P2-009 (Error Paths — async-plugin-exit-2)**: Added Error Paths row for "Async plugin returns exit code 2". EC-005 already documented this case fully; the Error Paths table now cross-references it explicitly. The new row notes the `plugin.async_block_discarded` emission and cross-links to BC-7.06.001 Invariant 1.

**F-P2-015 (Precondition 4 BC pin)**: Appended "(per BC-7.06.001 Invariant 1)" to Precondition 4. The load-time invariant is enforced externally by BC-7.06.001; the pin makes the dependency explicit.

## Amendment 2026-05-07 (v1.1 — F2 pass-1 fix burst)

Addresses adversary pass-1 findings F-P1-003, F-P1-004, F-P1-009, F-P1-010, F-P1-011, F-P1-013, F-P1-015.

**F-P1-004 / F-P1-011 (FAIL-CLOSED)**: Error Paths row 1 (`schema_version != 2`) and EC-006 amended. The schema-version mismatch exit code is now **exit 2 (fail-closed)** with explicit stderr diagnostic. The previous "exit 0 per BC-1.08.001 fail-open" text was a contradiction: "hard error" + "exit 0" is observationally identical to a clean run (silent failure). This is the explicit exception to BC-1.08.001's fail-open policy — schema-version mismatch must be fail-closed because silent acceptance of a mismatched schema is the exact bug ADR-019 was created to fix. BC-1.08.001 has been amended to note this exception.

**F-P1-009 (`block_intent` definition)**: EC-005 reframed. `block_intent := (plugin.exit_code == 2 && plugin.config.on_error == "block")`. Since Invariant 4 prohibits async plugins from having `on_error = "block"`, `block_intent` is structurally FALSE for any async plugin exit code 2. EC-005 now explains the logical consequence.

**F-P1-010 (Story Anchor)**: Story Anchor updated from "Story A + Story B + Story D (F1 phased rollout sketch)" to "TBD — single story per ADR-019 §6 (no phased rollout, user decision 2026-05-07)". Same change in Traceability Stories field.

**F-P1-013 (Async plugin lifetime)**: Postcondition 4 extended with explicit best-effort lifetime caveat: dispatcher does NOT await async tasks; process exits after sync_group; async plugin output may be truncated at process exit; this is an accepted trade-off for sync_group latency.

**F-P1-015 (Spawn ordering)**: Postcondition 6 added: async group plugins are spawned only after sync_group execution completes. Interleaving is forbidden.

**F-P1-003 (DI-014 citation)**: L2 Domain Invariants field updated from "TBD" to DI-014 with explanation.
