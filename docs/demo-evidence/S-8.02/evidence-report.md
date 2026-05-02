# S-8.02 Demo Evidence — pr-manager-completion-guard Native WASM Port

Story spec: `.factory/stories/S-8.02-native-port-pr-manager-completion-guard.md`

Branch: `feature/S-8.02-native-port-pr-manager-completion-guard`
HEAD SHA: `23eaf06`
Date: 2026-05-02

**Summary:** 19/19 bats GREEN; 32/32 Rust unit tests GREEN. Native WASM crate ports
`pr-manager-completion-guard.sh` to `crates/hook-plugins/pr-manager-completion-guard/`,
implementing the FM4 guard (prevents pr-manager SubagentStop before all 9 PR lifecycle
steps complete) with full BC-2.02.012 typed-projection fallback chains.

This is a Rust WASM product. Evidence is comprised of:
- Per-AC markdown files (`AC-001.md` through `AC-008.md`) with grep/diff/test output
- `bonus.md` covering deny_unknown_fields fix + workspace clippy nits
- This evidence report (index + coverage map)

VHS is installed but not applicable — pr-manager-completion-guard is a WASM plugin
invoked by the factory-dispatcher, not an interactive CLI binary. The bats integration
tests (dispatching JSON payloads through the live dispatcher) are the authoritative
runtime evidence.

---

## Evidence Files

| File | Description |
|------|-------------|
| `AC-001.md` | WASM crate structure; Cargo.toml; WASM artifact; registry migration diff |
| `AC-002.md` | .sh file deletion; hooks.json cleanup (all 6 platform files confirmed clean) |
| `AC-003.md` | STEP_COMPLETE line counting >= 8; BC-2.02.012 Postcondition 6 typed chain |
| `AC-004.md` | BLOCKED detection; ERE pattern; success and error paths |
| `AC-005.md` | FM4 block path; hook.block event fields; 9-step hint table; stderr injection |
| `AC-006.md` | Full bats run (19/19); coverage map; wildcard NEXT_STEP=10/99 |
| `AC-007.md` | host::emit_event; no bin/emit-event subprocess; perf measurement (advisory) |
| `AC-008.md` | Malformed JSON graceful exit; T-11 amendment obligation |
| `bonus.md` | deny_unknown_fields fix; factory-dispatcher + track-agent-start clippy nits |
| `evidence-report.md` | This file — top-level index and AC coverage table |

---

## AC Coverage Table

| AC | Statement | BC Trace | Status | Evidence File | Bats |
|----|-----------|----------|--------|---------------|------|
| AC-001 | WASM crate built; registry migrated to native plugin path; event/priority/on_error preserved | BC-7.03.045 postcondition 1 | PASS | AC-001.md | N/A (build + file check) |
| AC-002 | .sh file deleted; hooks.json command entry removed from all 6 platform files | BC-7.03.045 invariant 1 | PASS | AC-002.md | N/A (file absence check) |
| AC-003 | >= 8 lines containing STEP_COMPLETE → exit 0; BC-2.02.012 P6 typed chain for result | BC-7.03.046 P1; BC-2.02.012 P6 | PASS | AC-003.md | ok 10, ok 11 |
| AC-004 | BLOCKED result at line start → exit 0 (legitimate early exit) | BC-7.03.047 P1 | PASS | AC-004.md | ok 12, ok 13 |
| AC-005 | < 8 steps, not BLOCKED → hook.block emit + hint injection + exit 2; BC-2.02.012 P5 agent chain | BC-7.03.048 P1; BC-2.02.012 P5+P6 | PASS | AC-005.md | ok 1-9 |
| AC-006 | Bats: all 9 step positions + non-pm + BLOCKED + 0-steps + NEXT_STEP=10 + NEXT_STEP=99 | BC-7.03.048 P1 | PASS | AC-006.md | ok 1-19 (all 19) |
| AC-007 | host::emit_event replaces bin/emit-event; bin/emit-event binary preserved; perf logged | BC-7.03.045 P1 | PASS | AC-007.md | N/A (code + perf check) |
| AC-008 | Malformed JSON → exit 0, no panic | BC-7.03.045 invariant 2 | PASS | AC-008.md | ok 14 |

All 8 ACs: PASS.

---

## Behavioral Contract Coverage

| BC ID | Title | Verified by |
|---|---|---|
| BC-7.03.045 | pr-manager-completion-guard: identity & registry binding | AC-001, AC-002, AC-007; bats ok 15, ok 18, ok 19 |
| BC-7.03.046 | counts STEP_COMPLETE emissions; passes if >= 8 | AC-003; bats ok 10, ok 11; unit tests |
| BC-7.03.047 | BLOCKED status is legitimate early exit | AC-004; bats ok 12, ok 13; unit tests |
| BC-7.03.048 | blocks with step-specific continuation hint | AC-005, AC-006; bats ok 1-9, ok 16, ok 17 |
| BC-2.02.012 | HookPayload SubagentStop typed projection: Postconditions 5 and 6 | AC-003, AC-005; bats ok 19; unit tests |

---

## Test Counts

| Suite | Count | Result |
|---|---|---|
| Bats integration (factory-dispatcher + WASM) | 19 | 19/19 GREEN |
| Rust unit tests (pr-manager-completion-guard) | 32 | 32/32 GREEN |
| **Total** | **51** | **51/51 GREEN** |

---

## Key Implementation Decisions

1. **BC-2.02.012 Postcondition 5 (agent identity):** `payload.agent_type.as_deref().or(payload.subagent_name.as_deref()).unwrap_or("unknown")` — canonical 3-arm chain; `subagent="unknown"` always emitted in hook.block event, never omitted.

2. **BC-2.02.012 Postcondition 6 (result text):** `payload.last_assistant_message.as_deref().or(payload.result.as_deref()).unwrap_or("")` — canonical 2-stage chain; `""` when both absent.

3. **STEP_COMPLETE line counting:** Uses `str::lines().filter(contains).count()` — matches `grep -c` behavior: one line with multiple tokens = 1 occurrence.

4. **BLOCKED regex:** `(?m)^(Status:|##?\s*)?\s*BLOCKED` — multiline flag anchors `^` to line start; Rust regex crate ERE syntax (unescaped `|`).

5. **9-step hint table:** `match next_step { 1 => ..., 9 => ..., _ => "continue the 9-step lifecycle" }` — wildcard arm fires for NEXT_STEP >= 10 or unknown.

6. **emit_event capability:** `host::emit_event` is always wired unconditionally; NOT a declared capability in the `Capabilities` struct. The spurious `emit_event = true` field removed from legacy-bash-adapter scaffolding (`deny_unknown_fields` correctness fix).

---

## Registry State After Migration

```toml
[[hooks]]
name = "pr-manager-completion-guard"
event = "SubagentStop"
plugin = "hook-plugins/pr-manager-completion-guard.wasm"
priority = 920
timeout_ms = 5000
on_error = "block"
```

Legacy-bash-adapter removed. No `[hooks.config]`, no `[hooks.capabilities]` block.
`pr-manager-completion-guard.sh` deleted. All six `hooks.json` platform files clean.

---

## Bonus Changes (commit 23eaf06)

1. **deny_unknown_fields fix:** `emit_event = true` removed from registry capabilities block (was invalid — `Capabilities` struct has no such field; `deny_unknown_fields` would reject it at registry load)
2. **factory-dispatcher/src/main.rs clippy nit:** Collapsed nested `if let + if !is_empty` into let-chain
3. **track-agent-start/src/lib.rs clippy nits:** Removed `let_unit_value` warning on `host::emit_event`; fixed `doc_overindented_list_items` in doc comment

All three are pre-existing or incidental nits; no behavior change.

---

## Commit Reference

Implementation commit: `23eaf06` — `feat(s-8.02): green — pr-manager-completion-guard native WASM port + advisory block-mode + bats dispatcher`

7 files changed in green commit:
- `crates/factory-dispatcher/src/main.rs` (clippy nit)
- `crates/hook-plugins/pr-manager-completion-guard/src/lib.rs` (full implementation)
- `crates/hook-plugins/track-agent-start/src/lib.rs` (clippy nits)
- `plugins/vsdd-factory/hooks-registry.toml` (registry migration)
- `plugins/vsdd-factory/hooks/hooks.json` (entry removed)
- `plugins/vsdd-factory/hooks/pr-manager-completion-guard.sh` (deleted)
- `tests/integration/E-8-hook-plugins/pr-manager-completion-guard.bats` (19 tests)
