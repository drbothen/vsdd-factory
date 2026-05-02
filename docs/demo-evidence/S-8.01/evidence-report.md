# S-8.01 Demo Evidence Report

**Story:** S-8.01 — Native port: handoff-validator (SubagentStop)
**Branch:** `feature/S-8.01-native-port-handoff-validator`
**Implementation commit:** `6f2e0e7`
**Evidence produced by:** Demo Recorder
**Date:** 2026-05-02

---

## Summary

S-8.01 ports `plugins/vsdd-factory/hooks/handoff-validator.sh` to a native
Rust WASM crate at `crates/hook-plugins/handoff-validator/`. The crate
implements the `vsdd-hook-sdk` hook interface, reads SubagentStop JSON from
stdin using BC-2.02.012 typed projection fields, and emits advisory warnings
for empty or suspiciously short subagent results.

All 7 acceptance criteria are met. 23 cargo unit tests pass. 7 bats parity
tests pass. The registry entry is migrated to the native WASM plugin. The
legacy bash hook is deleted.

---

## AC Coverage Map

| AC | Title | Status | Evidence File |
|----|-------|--------|--------------|
| AC-001 | Native WASM crate + registry migration | PASS | [AC-001.md](AC-001.md) |
| AC-002 | hooks.json.* zero-entry verification + .sh deletion | PASS | [AC-002.md](AC-002.md) |
| AC-003 | Empty result → hook.block warn + stderr + exit 0 | PASS | [AC-003.md](AC-003.md) |
| AC-004 | Short result (<40) → hook.block warn + result_len + exit 0 | PASS | [AC-004.md](AC-004.md) |
| AC-005 | All 7 bats parity tests pass (cases a-g) | PASS | [AC-005.md](AC-005.md) |
| AC-006 | Malformed JSON graceful exit 0; no jq dependency | PASS | [AC-006.md](AC-006.md) |
| AC-007 | host::emit_event replaces bin/emit-event; bin/emit-event not removed | PASS | [AC-007.md](AC-007.md) |

---

## BC Coverage Map

| BC | Title | Verified By |
|----|-------|-------------|
| BC-7.03.042 | handoff-validator: identity & registry binding | AC-001 (registry), AC-002 (hooks.json), AC-006 (invariant 2), AC-007 (D-10) |
| BC-7.03.043 | handoff-validator: warns on empty subagent result | AC-003 (implementation), AC-005 (cases a, f) |
| BC-7.03.044 | handoff-validator: warns on suspiciously short result (<40 chars) | AC-004 (implementation), AC-005 (cases b, d, e) |
| BC-2.02.012 | HookPayload SubagentStop typed-projection fields | AC-001 (field usage), AC-003 (PC-5+6+EC-004), AC-005 (integration) |

---

## Test Results

### Cargo unit tests

```
cargo test -p handoff-validator
test result: ok. 23 passed; 0 failed; 0 ignored
```

All 23 tests pass. Test names follow the `test_BC_<id>_<description>` naming
convention, directly tracing each test to its governing behavioral contract.

### Bats integration tests

```
bats tests/integration/E-8-hook-plugins/handoff-validator.bats
7 tests, 0 failures
```

| Case | Description | Pass |
|------|-------------|------|
| (a) | empty `last_assistant_message` → exit 0, stderr "empty result" | yes |
| (b) | 5-char result → exit 0, stderr "non-whitespace characters" | yes |
| (c) | 50-char result → exit 0, no stderr warning | yes |
| (d) | LEN=39 → exit 0, warning emitted (below threshold) | yes |
| (e) | LEN=40 → exit 0, NO warning (at-or-above threshold) | yes |
| (f) | missing `last_assistant_message` → exit 0, stderr warns | yes |
| (g) | malformed JSON → exit 0, no panic | yes |

---

## Bonus Evidence

### Critical Fix: HookPayload `#[serde(flatten)]` in factory-dispatcher

`crates/factory-dispatcher/src/payload.rs` was extended with:

```rust
#[serde(flatten)]
pub extra: std::collections::HashMap<String, serde_json::Value>,
```

This ensures all event-specific fields (SubagentStop's `agent_type`,
`subagent_name`, `last_assistant_message`, `result` per BC-2.02.012) are
forwarded from the dispatcher to plugins unchanged. Without this fix, the
four BC-2.02.012 typed projection fields added by S-8.30 were silently
dropped in the dispatcher's parse→serialize round-trip, making them always
`None` at the plugin level.

Full rationale: [bonus-payload-flatten.md](bonus-payload-flatten.md)

### Architecture Advisory: Block-Mode Interpretation

handoff-validator emits `hook.block` events (via `host::emit_event`) but
always returns `HookResult::Continue`. The registry `on_error = "block"`
is a dispatcher crash-handler setting, not a plugin decision. There is no
`HookResult::Block` SDK variant. This advisory documents the naming tension
and flags the question of whether a real block variant is needed for a future
SDK extension story.

Full rationale: [advisory-block-mode-rationale.md](advisory-block-mode-rationale.md)

---

## File Inventory

| File | Lines | Purpose |
|------|-------|---------|
| `evidence-report.md` | 120+ | This report — coverage map, test results, bonus sections |
| `AC-001.md` | 80+ | Crate existence, registry migration, typed-field usage |
| `AC-002.md` | 55+ | hooks.json zero-entry verification, .sh deletion |
| `AC-003.md` | 90+ | Empty result warning path (BC-7.03.043 + BC-2.02.012 PC-5/6/EC-004) |
| `AC-004.md` | 85+ | Short result warning path + off-by-one boundary (BC-7.03.044) |
| `AC-005.md` | 80+ | 7/7 bats parity tests + 23/23 cargo tests + perf measurement |
| `AC-006.md` | 70+ | Malformed JSON graceful exit; no jq; invariant 2 divergence |
| `AC-007.md` | 65+ | host::emit_event substitution; bin/emit-event preserved |
| `bonus-payload-flatten.md` | 80+ | Critical dispatcher #[serde(flatten)] fix |
| `advisory-block-mode-rationale.md` | 90+ | Block-mode naming tension + wave-gate flag |

---

## Architecture Compliance Verification

| Rule | Status |
|------|--------|
| HOST_ABI_VERSION = 1 unchanged | PASS — no new host fns; typed projection is additive per D-6 |
| BC anchor strategy Option C — reuse existing BCs | PASS — BC-7.03.042/043/044 + BC-2.02.012 used; no new BC family |
| bin/emit-event NOT removed (deferred to S-8.29) | PASS — binary untouched |
| hooks.json routing: native plugins not in hooks.json | PASS — zero hooks.json entries for handoff-validator |
| No legacy-bash-adapter dependency | PASS — Cargo.toml has no such dep |
| Typed-projection uses canonical BC-2.02.012 fallback chains | PASS — PC-5 and PC-6 chains verbatim; EC-004 documented |
| on_error = "block" preserved in registry | PASS — registry entry confirmed |
| Capabilities block removed (no exec_subprocess) | PASS — no [hooks.capabilities] block in handoff-validator entry |

---

## Convergence Status

S-8.01 meets all 7 acceptance criteria. 23 cargo tests + 7 bats tests pass.
Two bonus items documented: the critical flatten fix (cross-subsystem, needed
for all E-8 Tier 1 WASM plugins that read SubagentStop fields) and the
block-mode advisory (flagged for W-15 wave-gate review).
