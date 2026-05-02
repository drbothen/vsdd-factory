# Evidence Report: S-8.08 — Native port: track-agent-start (PreToolUse:Agent)

**Story:** S-8.08
**Branch:** feature/S-8.08-native-port-track-agent-start
**Implementation commit:** 67fb911
**Evidence commit:** see git log
**Date:** 2026-05-02
**Bats result:** 13/13 PASS

## Summary

S-8.08 ports `plugins/vsdd-factory/hooks/track-agent-start.sh` to a native Rust WASM
crate at `crates/hook-plugins/track-agent-start/`. The hook fires on PreToolUse events
filtered to the Agent tool. It emits an `agent.start` telemetry event with strict
E-8 D-2 bash parity: exactly five fields (`type`, `hook`, `matcher`, `subagent`,
optional `story_id`) — no additive fields, no `agent_id`, no `tool_name`.

All 6 acceptance criteria pass. 13 bats tests pass. The `VSDD_SINK_FILE` dispatcher
improvement enables bats integration testing for all future hook stories.

## Acceptance Criteria Coverage

| AC | Description | BC Trace | Evidence File | Status |
|----|-------------|----------|---------------|--------|
| AC-001 | WASM crate exists, builds wasm32-wasip1, registry updated | BC-7.03.079 postcondition 1 | AC-001.md | PASS |
| AC-002a | Parity audit: exact field set, no agent_id, no tool_name | BC-7.03.079 invariant 1 | AC-002a.md | PASS |
| AC-002b | hooks.json zero entries + track-agent-start.sh deleted | Architecture Compliance | AC-002b.md | PASS |
| AC-003 | Valid Agent dispatch emits agent.start with subagent; exit 0 | BC-7.03.080 postcondition 1 | AC-003.md | PASS |
| AC-004 | story_id two-pattern cascade: S-N.NN first, STORY-NNN fallback, omit if absent | BC-7.03.080 postcondition 1 | AC-004.md | PASS |
| AC-005 | 13 bats parity tests pass; WASM perf measured (INFORMATIONAL) | BC-7.03.079 pc-1 + BC-7.03.080 pc-1 | AC-005.md | PASS |
| AC-006 | Best-effort: all error paths exit 0 silently; bin/emit-event not removed | BC-7.03.079 invariant 2 | AC-006.md | PASS |

## Bats Test Results

```
1..13
ok 1  AC-001: hooks-registry.toml track-agent-start entry references native WASM (not legacy-bash-adapter)
ok 2  AC-001: hooks-registry.toml track-agent-start has no script_path (legacy-bash-adapter artifact removed)
ok 3  AC-001: hooks-registry.toml track-agent-start has no exec_subprocess block
ok 4  AC-002b: plugins/vsdd-factory/hooks/track-agent-start.sh is deleted
ok 5  AC-001 invariant: track-agent-start WASM artifact exists at wasm32-wasip1 target
ok 6  AC-005(a): Agent dispatch subagent=pr-manager S-6.07 => agent.start with subagent + story_id
ok 7  AC-005(b): Agent dispatch subagent=implementer STORY-042 => agent.start story_id=STORY-042
ok 8  AC-005(c): Agent dispatch subagent=reviewer no story pattern => agent.start no story_id
ok 9  AC-005(d): non-Agent tool_name => exit 0 and no agent.start event emitted
ok 10 AC-005(e): malformed JSON stdin => exit 0 and no event emitted (best-effort AC-006)
ok 11 AC-005(e) variant: empty stdin => exit 0 and no event emitted (EC-005)
ok 12 AC-002a parity audit: agent.start event contains exactly the bash-parity field set
ok 13 EC-001: missing subagent_type in Agent envelope defaults to subagent=unknown
```

**Total: 13 passed, 0 failed**

## Behavioral Contract Coverage

| BC ID | Title | Covered By | Status |
|-------|-------|-----------|--------|
| BC-7.03.079 | track-agent-start: identity & registry binding | AC-001, AC-002a, AC-005 (ok 1–5, 12), AC-006 (ok 9–11) | PASS |
| BC-7.03.080 | track-agent-start: emits agent.start with subagent + best-effort story_id | AC-003, AC-004, AC-005 (ok 6–8, 13) | PASS |

## Edge Case Coverage

| EC ID | Scenario | Covered By | Status |
|-------|----------|-----------|--------|
| EC-001 | subagent_type missing → default "unknown" | ok 13 + unit test | PASS |
| EC-002 | prompt missing/null → empty string; no story_id | ok 8 + unit test | PASS |
| EC-003 | Both S-N.NN and STORY-NNN in prompt → S-N.NN wins | unit test (pattern1_beats_pattern2) | PASS |
| EC-004 | tool_name != "Agent" → exit 0, no event | ok 9 + unit test | PASS |
| EC-005 | Empty stdin → exit 0, no event, no panic | ok 11 | PASS |
| EC-006 | emit_event returns Err → silently swallowed; exit 0 | unit test (invariant_exit_0_on_any_path) + `let _` pattern | PASS |

## Implementation Artifacts

| File | Action | Purpose |
|------|--------|---------|
| `crates/hook-plugins/track-agent-start/Cargo.toml` | created | Crate manifest; wasm32-wasip1; vsdd-hook-sdk, serde_json, regex deps |
| `crates/hook-plugins/track-agent-start/src/lib.rs` | created | Hook logic: stdin parse, Agent guard, subagent extract, story_id regex, emit_event |
| `crates/hook-plugins/track-agent-start/src/main.rs` | created | WASI command entry point; wires lib.rs to host fns |
| `tests/integration/E-8-hook-plugins/track-agent-start.bats` | created | 13 bats parity tests covering all ACs and edge cases |
| `plugins/vsdd-factory/hooks-registry.toml` | modified | track-agent-start entry updated: native .wasm, script_path removed, exec_subprocess removed |
| `plugins/vsdd-factory/hooks/track-agent-start.sh` | deleted | Replaced by native WASM crate |
| `crates/factory-dispatcher/src/main.rs` | modified | VSDD_SINK_FILE mechanism added (see bonus evidence) |

## Bonus: VSDD_SINK_FILE Infrastructure

See `bonus-vsdd-sink-file.md` for full documentation.

The `VSDD_SINK_FILE` env var, added to `crates/factory-dispatcher/src/main.rs` as part
of S-8.08, is a workspace-shared improvement that enables bats integration testing for
ALL future hook stories. When set, the dispatcher appends all plugin-domain events
(filtered to exclude `dispatcher.*`, `internal.*`, `plugin.*` lifecycle noise) as JSONL
to the specified path after execution completes.

Architecture properties:
- Best-effort: sink write errors silently dropped; dispatcher exit code unaffected
- Domain-only filter: only `agent.start`-style telemetry events reach the sink
- Thread-safe: drains the `Arc<Mutex<Vec<InternalEvent>>>` event queue via `mem::take`
- No production overhead: triggered only when `VSDD_SINK_FILE` is set (test harnesses only)

Mutation testing (CC-W15-002): filter correctness mutations (include/exclude lifecycle
events) are caught by AC-002a parity audit. Happy-path mutations (events not written)
caught by AC-005(a–c) `[ -f "$SINK_FILE" ]` + `jq` assertions.

## Architecture Compliance Checklist

| Rule | Status |
|------|--------|
| HOST_ABI_VERSION = 1 unchanged | PASS — only stdin read + emit_event used |
| BC-anchor strategy Option C: reuse BC-7.03.079/080 | PASS — no new BC-7.xx migration family |
| bin/emit-event NOT removed (deferred to S-8.29) | PASS — binary untouched |
| No hooks.json entries for native plugins (DRIFT-004) | PASS — zero entries confirmed |
| exec_subprocess block removed from registry | PASS — removed in hooks-registry.toml |
| Compilation target wasm32-wasip1 (not deprecated wasm32-wasi) | PASS — Cargo.toml target correct |
| Forbidden dependency on legacy-bash-adapter | PASS — no dependency in Cargo.toml |
