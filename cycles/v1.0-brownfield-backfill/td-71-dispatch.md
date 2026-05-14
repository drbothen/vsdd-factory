# TD #71 — Dispatcher Stderr blocking_plugins + block_reason Surfacing
## Self-Contained Dispatch Package (2026-05-14)

> **Post-CLEAR entry point.** Read this file top-to-bottom to dispatch the implementer agent with zero prior context.
> Verified against develop@21d444d8.

---

## Goal

When the factory-dispatcher binary emits a PreToolUse blocking signal (`block_intent=true exit_code=2`), extend the stderr summary line to include `blocking_plugins=<names>` and `block_reason=<text>` so operators don't have to grep `.factory/logs/dispatcher-internal-YYYY-MM-DD.jsonl` to find the actual reason.

## Current Pain (CLAUDE.md "Factory Hook Diagnostics" Step 2)

Operators today see stderr like:
```
factory-dispatcher trace=<UUID> event=PreToolUse tool=Agent host_abi=1 sync_plugins=N async_plugins=N
  plugins_run=N total_ms=N block_intent=true exit_code=2
```

The trace UUID is the ONLY way to find the actual block reason (must grep the internal log for that UUID's `plugin.log level=warn` records). TD #71 surfaces the reason inline so the workaround in CLAUDE.md "Factory Hook Diagnostics" Step 2 becomes obsolete for the common case.

## Target Stderr Format After TD #71

```
factory-dispatcher trace=<UUID> event=PreToolUse tool=Agent host_abi=1 sync_plugins=N async_plugins=N
  plugins_run=N total_ms=N block_intent=true exit_code=2 blocking_plugins=plugin-a,plugin-b block_reason="FAIL: MULTI_COMMIT_CHAIN_NOT_ALLOWED — HEAD and HEAD^ both contain 'backfill'..."
```

When `block_intent=false`, the new fields are omitted (no behavior change for non-blocking dispatches).

## File Surface (verified 2026-05-14 against develop@21d444d8)

| File | Lines | Role |
|------|-------|------|
| `crates/factory-dispatcher/src/main.rs` | 214-222 (line 1 of stderr summary); 535-541 (line 2 — extend this) | Stderr emission sites |
| `crates/factory-dispatcher/src/executor.rs` | 42-50 (`PluginOutcome { plugin_name: String, result, on_error, ... }`); 52-58 (`ExecutionSummary { per_plugin_results, total_elapsed_ms, exit_code, block_intent }`) | Per-plugin result types; iterate `summary.per_plugin_results` to extract names |
| `crates/factory-dispatcher/src/aggregator.rs` | 96-106 (block-intent classification: `exit_code=2 AND on_error=Block`) | Reuse the same filter to identify blocking plugins |
| `crates/factory-dispatcher/src/internal_log.rs` | Plugin log emission with `level: warn` carries the human-readable block reason | Source of `block_reason` text — see decision tree below |

## Decision Tree for block_reason Extraction

The human-readable reason is currently emitted by plugins as `plugin.log` records with `level: warn` and a multi-line `message` field. Three viable extraction approaches, in order of architectural cleanliness:

**(a) Add `block_reason: Option<String>` to `PluginOutcome`** (executor.rs:42). Plugins that block populate it; aggregator surfaces the first non-None as the dispatcher's `block_reason`. Cleanest separation; requires touching host-side log capture path so the warn-level message gets buffered into the outcome struct. Recommended cleanest.

**(b) Read internal_log writes for this trace at end of dispatch.** Lower-impact but couples stderr formatter to internal log structure. Working set: `internal_log` is already in scope at main.rs near the eprintln; iterate captured records, filter by `trace_id == current && level == warn && plugin_name in blocking_plugins`, concatenate. **Recommended for fastest landing (lowest blast radius).**

**(c) Have plugins emit `block_reason` via a separate WASI host call** (e.g., `host_set_block_reason(reason: &str)`). Cleanest plugin-side API but requires SDK changes + new hook-sdk crate version. Defer to S-15.03 PRIORITY-A wave scope.

**Recommendation:** Start with (b) for fastest landing, then refactor to (a) once the eprintln format is stable. Skip (c) unless plugin-side SDK changes are already in flight.

## Filter Logic for blocking_plugins

Reuse the pattern at main.rs:512-526 (the `sync_agg_results: Vec<AggregatorPluginResult>` map). The blocking subset is:
```rust
let blocking_plugins: Vec<&str> = summary.per_plugin_results.iter()
    .filter(|o| matches!(&o.result, PluginResult::Ok { exit_code: 2, .. })
        && o.on_error == OnError::Block)
    .map(|o| o.plugin_name.as_str())
    .collect();
```
(Adapt exact match to the actual PluginResult enum shape; reference executor.rs lines 42-50 for canonical type definitions.)

## Test Surface

Bats test location: `plugins/vsdd-factory/tests/`. Candidate test files (verify with `grep -l "block_intent\|exit_code=2" plugins/vsdd-factory/tests/*.bats`):

- `hook-robustness.bats` — likely existing test asserting on exit_code=2; extend with new stderr field assertions
- `agent-guards-emission.bats` — covers Agent-tool blocking; extend
- NEW test file `tests/td-71-stderr-block-reason.bats` — if existing files don't have clean extension points

**5 test cases required (BC-style):**
1. PreToolUse + 1 plugin block → stderr contains `blocking_plugins=<that-plugin>` (single)
2. PreToolUse + 2 plugins block → stderr contains `blocking_plugins=plugin-a,plugin-b` (comma-joined, deterministic order)
3. PreToolUse + 0 blocks → stderr OMITS `blocking_plugins=` and `block_reason=` (negative test)
4. PostToolUse blocking → same blocking_plugins format applies (PostToolUse can also block per existing dispatcher semantics)
5. block_reason newline handling — multi-line plugin messages must be escaped/normalized in single-line stderr output

Run: `cd plugins/vsdd-factory/tests && ./run-all.sh` for the canonical end-to-end suite + `cargo test --workspace --all-targets` for unit-level.

## Pre-Flight Gate (CI Mirror)

Per CLAUDE.md "Build / Test / Lint":
```bash
cargo fmt --check --all
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace --all-targets
cd plugins/vsdd-factory/tests && ./run-all.sh
```

All four must pass before opening PR.

## PR Strategy

- Branch: `feature/td-71-stderr-block-reason` off `origin/develop` (currently `21d444d8`).
- Target: develop (NOT main; main is release-branch-only).
- Conventional Commits: `feat(dispatcher): surface blocking_plugins + block_reason in stderr (TD #71)` or similar.
- Standard 9-step pr-manager lifecycle. No demo recording required (CLI-only behavior change).
- AI-review + security-review apply (modifies dispatcher binary — production-code change, not docs-only).

## CLAUDE.md Follow-Up (in Same PR or Follow-Up)

The "Factory Hook Diagnostics" section in CLAUDE.md currently documents the grep-internal-log workaround as the canonical diagnostic path. After TD #71 lands, that section should be updated to note: "Step 2 grep is now needed only for advisory-level plugin.log records; block_reason itself is in stderr." Update inline.

## Dispatch Sequence (Executable by Orchestrator, Fresh Context)

1. Spawn implementer agent (NOT architect — this is a coding task, not a spec/architecture change). Hand off this file as the task description.
2. Implementer drafts the executor.rs / aggregator.rs / main.rs changes per decision tree option (b) (lowest blast radius). Stages but does not commit.
3. Implementer writes bats test coverage per the 5 test cases.
4. Implementer runs the 4 pre-flight commands; iterates until green.
5. Implementer pushes `feature/td-71-stderr-block-reason`; dispatches pr-manager.
6. pr-manager opens PR to develop; runs standard 9-step lifecycle (AI-review, security-review, CI wait, merge).
7. After merge: state-manager updates STATE.md (mark TD #71 RESOLVED in Drift Items; pivot Tier-A to next priority).

## Effort Estimate

Small (1-2 hours implementer + standard PR cycle). No spec/architecture changes; pure dispatcher binary enhancement.

## Resumption Gate

None. TD #71 is independent of E-10 (sealed at asymptotic-acceptance) and F5 (paused). Can ship immediately.
