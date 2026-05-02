# Evidence Report: S-8.07 — warn-pending-wave-gate Native WASM Port

**Story:** S-8.07 — Native port: warn-pending-wave-gate (Stop)
**Branch:** feature/S-8.07-native-port-warn-pending-wave-gate
**Commit:** 216f05e
**Date:** 2026-05-02
**Status:** CONVERGENCE REACHED — 5/5 bats pass, all 7 ACs verified

---

## Coverage Summary

| AC | Title | BC Trace | Evidence File | Result |
|----|-------|----------|---------------|--------|
| AC-001 | WASM crate exists, builds wasm32-wasip1, registry updated | BC-7.03.091 postcondition 1 | [AC-1.md](AC-1.md) | PASS |
| AC-002 | hooks.json entry absent; .sh file deleted | BC-7.03.091 invariant 1 | [AC-2.md](AC-2.md) | PASS |
| AC-003 | Pending wave: emit hook.block + WAVE GATE REMINDER | BC-7.03.092 postcondition 1 | [AC-3.md](AC-3.md) | PASS |
| AC-004 | Early-exit paths silent (3 paths + EC-009) | BC-7.03.091 postcondition 2 | [AC-4.md](AC-4.md) | PASS |
| AC-005 | Bats parity tests pass 5/5 (dispatcher path) | BC-7.03.091 PC-1 + BC-7.03.092 PC-1 | [AC-5.md](AC-5.md) | PASS |
| AC-006 | python3 replaced with serde_yaml 0.9.34; exec_subprocess removed | BC-7.03.091 invariant 2 | [AC-6.md](AC-6.md) | PASS |
| AC-007 | host::emit_event replaces bin/emit-event; binary preserved | BC-7.03.092 postcondition 1 | [AC-7.md](AC-7.md) | PASS |

**Bonus:** [bonus-dispatcher-fixes.md](bonus-dispatcher-fixes.md) — 3 workspace-shared
dispatcher fixes that unblock all future native WASM hook ports using `host::read_file`.

---

## Behavioral Contract Status

| BC ID | Title | Status |
|-------|-------|--------|
| BC-7.03.091 | warn-pending-wave-gate: identity & registry binding (Stop, priority=920, on_error=continue) | SATISFIED |
| BC-7.03.092 | warn-pending-wave-gate: stderr warning when any wave has gate_status: pending, hook.block severity=warn | SATISFIED |

---

## Key Implementation Facts

### Architecture

The hook splits into two Rust files:
- `src/lib.rs` — pure `warn_pending_wave_gate_logic()` function accepting closures
  for file reading, event emission, and stderr writing. Testable without a WASM runtime.
- `src/main.rs` — WASI command entry point that wires the closures to actual host
  functions (`host::read_file`, `host::emit_event`) and hands off to
  `vsdd_hook_sdk::__internal::run(on_hook)`.

This lib/bin split enables unit testing of the YAML parsing and warning logic in
`cargo test` without requiring wasmtime or a dispatcher fixture.

### Behavior Delta vs Bash Source

The bash source had a `python3`-absent early-exit guard: if python3 was not installed,
the hook silently exited 0. The WASM port replaces python3 with serde_yaml 0.9.34 and
always proceeds to the YAML check — the warning is no longer suppressed by a transitive
python3 dependency. This is a deliberate behavior change documented in the story Goal
section (E-8 D-2 "behavior-parity-only" is satisfied modulo this exception).

### serde_yaml Usage

`serde_yaml::Value` (dynamic value API) is used rather than strict typed
deserialization. This ensures graceful handling of EC-008 (non-string `gate_status`
values) via `.and_then(Value::as_str)` which returns `None` for non-string values
instead of panicking.

### Registry Migration

| Field | Before (legacy bash adapter) | After (native WASM) |
|-------|------------------------------|---------------------|
| `plugin` | `hook-plugins/legacy-bash-adapter.wasm` | `hook-plugins/warn-pending-wave-gate.wasm` |
| `script_path` | `hooks/warn-pending-wave-gate.sh` | removed |
| `shell_bypass_acknowledged` | `"legacy-bash-adapter runs unported hooks"` | removed |
| `[hooks.capabilities.exec_subprocess]` | `binary_allow = ["bash"]` | removed entirely |
| `[hooks.capabilities.read_file]` | absent | `path_allow = [".factory/wave-state.yaml"]` |

---

## Bonus: Dispatcher Fixes

Three workspace-shared fixes landed in commit 216f05e:

1. **Real read_file implementation** (`invoke.rs`): Previously a stub returning
   CAPABILITY_DENIED. Now grows WASM memory and writes file bytes via out-param
   protocol. warn-pending-wave-gate is the first in-tree consumer.

2. **cwd path resolution** (`host/read_file.rs`): Relative paths (e.g.
   `.factory/wave-state.yaml`) now resolve under `ctx.cwd` (`$CLAUDE_PROJECT_DIR`)
   instead of `ctx.plugin_root`. Without this fix, all project-relative file reads
   would fail silently.

3. **stderr relay** (`main.rs`): Plugin stderr (captured into `MemoryOutputPipe` by
   the WASI sandbox) is now relayed to the dispatcher's process stderr after
   `execute_tiers`. Without this relay, the WAVE GATE REMINDER would be invisible
   to the user.

See [bonus-dispatcher-fixes.md](bonus-dispatcher-fixes.md) for full details.

---

## File Inventory

| File | Lines | Purpose |
|------|-------|---------|
| `docs/demo-evidence/S-8.07/evidence-report.md` | 100+ | This report |
| `docs/demo-evidence/S-8.07/AC-1.md` | 62 | WASM crate + registry verification |
| `docs/demo-evidence/S-8.07/AC-2.md` | 43 | hooks.json absent + .sh deleted |
| `docs/demo-evidence/S-8.07/AC-3.md` | 92 | emit_event + WAVE GATE REMINDER format |
| `docs/demo-evidence/S-8.07/AC-4.md` | 101 | Early-exit paths (4 paths) |
| `docs/demo-evidence/S-8.07/AC-5.md` | 95 | Bats 5/5 pass via dispatcher path |
| `docs/demo-evidence/S-8.07/AC-6.md` | 82 | serde_yaml replaces python3 |
| `docs/demo-evidence/S-8.07/AC-7.md` | 83 | host::emit_event replaces bin/emit-event |
| `docs/demo-evidence/S-8.07/bonus-dispatcher-fixes.md` | 170 | 3 dispatcher runtime fixes |

---

## Commit

All evidence files committed to feature branch `feature/S-8.07-native-port-warn-pending-wave-gate`
at commit `docs(s-8.07): per-AC demo evidence for warn-pending-wave-gate native WASM port`.
