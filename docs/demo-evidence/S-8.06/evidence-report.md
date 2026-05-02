# Evidence Report — S-8.06: Native port: session-learning (Stop)

**Story ID:** S-8.06
**Branch:** feature/S-8.06-native-port-session-learning
**Latest commit at evidence capture:** 5d5733a
**Date:** 2026-05-02
**Bats result:** 22/22 tests pass (4 in canonical suite + 18 in extended integration suite)

---

## Coverage Summary

| AC | BC Trace | Evidence File | Verdict |
|----|----------|--------------|---------|
| AC-001 | BC-7.03.076 postcondition 1 | AC-1.md | PASS |
| AC-002 | BC-7.03.077 postcondition 1 | AC-2.md | PASS |
| AC-003 | BC-7.03.078 postcondition 1 | AC-3.md | PASS |
| AC-004 | BC-7.03.077 postcondition 1 (append-only) | AC-4.md | PASS |
| Bonus | WASI preopen fix (dispatcher) | bonus-preopened-dir-fix.md | N/A |

---

## Bats Test Output (full run)

```
1..22
ok 1  T5-case1 (AC-002): first invocation creates file with header and one marker; exit 0
ok 2  T5-case2 (AC-004): second invocation appends one more marker, no duplicate header; exit 0
ok 3  T5-case3 (AC-003): invocation without .factory/ exits 0, no file created
ok 4  T5-case4 (EC-005): large Stop envelope (>64KB) drained without write error; exit 0
ok 5  AC-001: WASM artifact exists at hook-plugins/session-learning.wasm
ok 6  AC-001: WASM artifact has valid WASM magic header
ok 7  AC-001: hooks-registry.toml references hook-plugins/session-learning.wasm
ok 8  AC-001: session-learning registry entry does not reference legacy-bash-adapter
ok 9  AC-001: session-learning registry entry preserves event=Stop priority=910 on_error=continue
ok 10 AC-001: session-learning registry entry has no exec_subprocess or binary_allow
ok 11 AC-002: .factory/ present, sidecar-learning.md absent — creates file with header and marker
ok 12 AC-002: created sidecar-learning.md starts with exact header (byte-identical to bash output)
ok 13 AC-002: created sidecar-learning.md contains marker line with ISO-8601 UTC timestamp
ok 14 AC-002: exactly one marker line on first invocation
ok 15 AC-003: .factory/ absent — plugin exits 0 (direct WASM invocation)
ok 16 AC-003: .factory/ absent — no sidecar-learning.md created (direct WASM invocation)
ok 17 AC-003: .factory/ absent — no .factory directory created (direct WASM invocation)
ok 18 AC-004: second invocation appends one marker line, no duplicate header
ok 19 AC-004: two invocations produce exactly two marker lines
ok 20 AC-004: three invocations produce exactly three marker lines
ok 21 EC-005: plugin drains large Stop envelope (65536+ bytes) without SIGPIPE-equivalent failure
ok 22 EC-005 (direct): plugin drains 128KB stdin without error when invoked directly
```

Test files:
- `tests/integration/hooks/session-learning.bats` — canonical T-5 suite (4 cases)
- `tests/integration/E-8-hook-plugins/session-learning.bats` — extended integration suite (18 cases)

---

## AC-001: WASM crate + registry migration

**File:** `crates/hook-plugins/session-learning/Cargo.toml`

- Target: `wasm32-wasip1` (NOT deprecated `wasm32-wasi`)
- Path dep: `vsdd-hook-sdk = { path = "../../hook-sdk" }` (empirically verified per D-172 finding #8)
- No `serde_json` dependency (not needed — plugin discards stdin)
- No `legacy-bash-adapter` dependency (forbidden per architecture compliance)
- `chrono` workspace pin for UTC timestamp

**Registry entry** (`plugins/vsdd-factory/hooks-registry.toml`, lines 845-851):

```toml
[[hooks]]
name = "session-learning"
event = "Stop"
plugin = "hook-plugins/session-learning.wasm"
priority = 910
timeout_ms = 5000
on_error = "continue"
```

All removed fields confirmed absent: `script_path`, `shell_bypass_acknowledged`,
`[hooks.config]`, `[hooks.capabilities.exec_subprocess]`, `[hooks.capabilities]`.
`session-learning.sh` deleted (T-7 complete). No `hooks.json` entries to modify
(DRIFT-004 already in AFTER state).

---

## AC-002: Header creation on first invocation

**File:** `crates/hook-plugins/session-learning/src/lib.rs`

`SIDECAR_HEADER` constant (line 32):
```
"# Sidecar Learning\n\nSession-end markers for the VSDD factory. Run /session-review to synthesize.\n\n"
```

The trailing `\n\n` produces the required blank line — bats case 1 verifies
byte-identical content against the bash source output.

File creation guard: `if !sidecar_path.exists()` (line 90) — header written exactly
once on first invocation via `std::fs::write`. File I/O uses `std::fs` directly
(`host::write_file` is absent per D-172 empirical verification).

Marker format: `- Session ended at {} (awaiting /session-review)\n` with `{}` replaced
by `chrono::Utc::now().format("%Y-%m-%dT%H:%M:%SZ")`.

---

## AC-003: .factory/ absent guard

First check in hook body (lines 75-83), before any file I/O:

```rust
if !factory_dir.is_dir() {
    return HookResult::Continue;  // exit 0, no side effects
}
```

`HookResult::Continue` maps to exit code 0. No files created, no stderr emitted.
Matches bash source `exit 0` in the absence check.

---

## AC-004: Append-only invariant

`OpenOptions::new().append(true).open(...)` (line 107) positions cursor at EOF without
reading or buffering the full file — EC-004 large-file safe.

Header guard `if !sidecar_path.exists()` (line 90) is the only condition that writes
the header. On second and subsequent invocations the file exists, so the header block
is skipped entirely and only the marker line is appended.

Bats cases 18-20 verify: 2 invocations → 2 markers, 3 invocations → 3 markers,
exactly 1 header occurrence throughout.

---

## EC-005: stdin drain

`main.rs` (lines 27-31):

```rust
let mut buf = Vec::with_capacity(4096);
let _ = io::stdin().read_to_end(&mut buf);
// buf intentionally dropped here — content is discarded.
```

Drain happens before hook logic. Bats cases 4 and 21-22 verify the plugin exits 0
when fed a >64KB Stop envelope, which exceeds the OS pipe buffer and would cause a
dispatcher write error if stdin were not drained.

---

## Architecture compliance checks

| Rule | Status |
|------|--------|
| No `emit_event` calls in session-learning crate | PASS — grep returns empty |
| No `exec_subprocess` calls | PASS — no subprocess needed; block removed from registry |
| `std::fs` used for file I/O (not `host::write_file`) | PASS — `host::write_file` absent per D-172 |
| `wasm32-wasip1` target (not deprecated `wasm32-wasi`) | PASS — Cargo.toml uses wasip1 |
| `vsdd-hook-sdk` as path dep `../../hook-sdk` | PASS — matches empirically verified form |
| No `serde_json` dependency | PASS — plugin discards stdin without parsing |
| `session-learning.sh` deleted | PASS — T-7 complete |
| No `hooks.json` modifications | PASS — DRIFT-004 AFTER state; no entries to modify |
| `bin/emit-event` binary NOT touched | PASS — session-learning.sh never called it |
| `[hooks.capabilities]` top-level block removed | PASS — no env vars needed |

---

## Bonus: WASI preopened_dir fix (workspace-shared)

**File:** `crates/factory-dispatcher/src/invoke.rs` (lines 130-157)

The dispatcher now preopens `host_ctx.cwd` as `"."` in the WASI guest namespace before
calling `_start`. This unblocks any future hook plugin that uses `std::fs` for host
filesystem access — including session-learning (S-8.06) which is the first native
plugin to need it.

Without this fix, `std::fs::write(".factory/sidecar-learning.md", ...)` returns
`Err(EBADF)` because the guest filesystem namespace is empty by default in WASI.

See `bonus-preopened-dir-fix.md` for full analysis.

---

## Files in this evidence package

| File | Purpose |
|------|---------|
| `evidence-report.md` | This file — master coverage mapping |
| `AC-1.md` | AC-001: crate existence, Cargo.toml, registry migration evidence |
| `AC-2.md` | AC-002: header creation on first invocation evidence |
| `AC-3.md` | AC-003: .factory/ absent guard evidence |
| `AC-4.md` | AC-004: append-only parity, full bats run output |
| `bonus-preopened-dir-fix.md` | Dispatcher WASI preopen fix analysis |
