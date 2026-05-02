# Demo Evidence Report — S-8.04: Native port: update-wave-state-on-merge

**Story:** S-8.04 — Native port: update-wave-state-on-merge (SubagentStop)
**Branch:** feature/S-8.04-native-port-update-wave-state-on-merge
**Latest commit:** 39f0de3
**Date recorded:** 2026-05-02
**BC anchors:** BC-7.03.083, BC-7.03.084, BC-7.03.085, BC-7.03.086
**Toolchain:** rustc 1.95.0 (59807616e 2026-04-14)

---

## Coverage Summary

| AC | Description | Evidence File | Recording | Status |
|----|-------------|---------------|-----------|--------|
| AC-001 | WASM crate built; registry entry: native .wasm, priority=940, on_error=continue, timeout_ms=10000 | [AC-1.md](AC-1.md) | [gif](AC-001-wasm-crate-exists.gif) | PASS |
| AC-002 | hooks.json entry deleted; update-wave-state-on-merge.sh deleted | [AC-2.md](AC-2.md) | [gif](AC-002-sh-deleted-hooksjson-removed.gif) | PASS |
| AC-003 | Non-pm agent or no merge signal → exit 0, no YAML mutation; 7 concrete test vectors | [AC-3.md](AC-3.md) | [gif](AC-003-agent-scope-merge-signal.gif) | PASS |
| AC-004 | Merge detected → extract story_id, append to stories_merged, write YAML, emit event | [AC-4.md](AC-4.md) | [gif](AC-004-yaml-append-emit-event.gif) | PASS |
| AC-005 | All stories merged → gate_status=pending (4-case truth table), next_gate_required, stderr reminder | [AC-5.md](AC-5.md) | [gif](AC-005-gate-status-flip.gif) | PASS |
| AC-006 | 20 bats parity tests: grep-gate + fixtures + dispatcher parity (all 6 AC-006 cases) | [AC-6.md](AC-6.md) | [gif](AC-006-bats-parity-tests.gif) | PASS |
| AC-007 | vsdd_hook_sdk::host::emit_event replaces bin/emit-event; bin/emit-event not removed; perf gate excluded | [AC-7.md](AC-7.md) | [gif](AC-007-emit-event-no-bin.gif) | PASS |

All 7 acceptance criteria: **PASS**. Zero regressions.

---

## Test Counts

| Suite | Count | Result |
|-------|-------|--------|
| Rust unit tests (`cargo test -p update-wave-state-on-merge`) | 39 | 39 PASS |
| Bats integration tests (grep-gate + fixture + dispatcher parity) | 20 | 20 PASS |
| **Total** | **59** | **59 PASS / 0 FAIL** |

---

## AC-001: WASM crate built; registry entry preserved

Crate at `crates/hook-plugins/update-wave-state-on-merge/` with `Cargo.toml` targeting
`wasm32-wasip1`. Key dependencies: `vsdd-hook-sdk = { path = "../../hook-sdk" }`,
`serde_yaml = { workspace = true }` (pinned 0.9.34 per OQ-002), `regex`.

Registry entry (hooks-registry.toml lines 943–956):
- `event = "SubagentStop"`, `priority = 940`, `on_error = "continue"`, `timeout_ms = 10000`
- `plugin = "hook-plugins/update-wave-state-on-merge.wasm"` (no `script_path`)
- Structured `read_file` + `write_file` capability blocks with `path_allow = [".factory/wave-state.yaml"]`

See [AC-1.md](AC-1.md).

---

## AC-002: .sh deleted; hooks.json entries removed

`plugins/vsdd-factory/hooks/update-wave-state-on-merge.sh` deleted.
No `update-wave-state-on-merge` entry in any `hooks.json.*` platform file or template.
Native WASM plugins route via `hooks-registry.toml` only (E-8 D-7 / DRIFT-004).

See [AC-2.md](AC-2.md).

---

## AC-003: Agent scoping + merge signal detection

`is_pr_manager_agent` matches `pr-manager` (hyphen) and `pr_manager` (underscore).
`has_merge_signal` uses compiled regex `(?i)STEP_COMPLETE: step=8.*status=ok|merge|squash`
(port-as-is per OQ-001; ERE precedence quirk preserved as TD for v1.2 fix).

7 concrete test vectors from story AC-003 all pass. Bats parity-7 and parity-8 confirm
no YAML mutation for non-pm agent and no-signal cases.

See [AC-3.md](AC-3.md).

---

## AC-004: YAML append + emit_event

`process_wave_state` extracts story_id (`S-N.NN` preferred, `STORY-NNN` fallback),
finds the containing wave, appends to `stories_merged` (if not duplicate), serializes
via `serde_yaml::to_string` (key-order preserved via IndexMap per OQ-002), writes back
via `host::write_file`.

`emit_event` fires with fields: `hook`, `matcher`, `reason`, `story_id`, `wave`,
`total`, `merged`, `gate_transitioned`. Emits `hook.error` on write failure (EC-005).

See [AC-4.md](AC-4.md).

---

## AC-005: gate_status flip (4-case truth table)

`gate_status` field uses `Option<String>` + `#[serde(default)]`:
- Cases 1+2 (absent, YAML null/`~`) → `None` → triggers flip
- Case 3 (`"not_started"`) → `Some("not_started")` → triggers flip
- Case 4 (any other value) → `Some("...")` → no flip

Stderr reminder emitted on gate transition. `wave-state-gate-null.yaml` fixture
(verbatim from AC-006 spec) exercises case 2.

See [AC-5.md](AC-5.md).

---

## AC-006: Bats parity tests (20/20)

8 grep-gate tests (source-level), 4 fixture existence tests, 8 dispatcher parity tests.
All 6 AC-006 cases covered: (a) append, (b) gate flip, (c) duplicate guard, (d) absent YAML,
(e) story not in wave, (f) YAML-null gate_status.

Parity tests route through `factory-dispatcher` (CLAUDE_PROJECT_DIR isolation via
`BATS_TEST_TMPDIR`). Checksum comparison confirms no YAML mutation for no-op paths.

See [AC-6.md](AC-6.md).

---

## AC-007: host::emit_event; bin/emit-event preserved

3 `emit_event` call sites in `main.rs` (success path, error path EC-005, type annotation).
No `bin/emit-event` reference in the WASM crate. `bin/emit-event` binary exists (not
removed per E-8 D-10; removal deferred to S-8.29).

Perf gate explicitly excluded: YAML-I/O classification criterion (read+parse+write
expected to dominate at >50% wall time; comparison with S-8.00 baseline would be
misleading). Informational timing via hyperfine deferred to T-2 E-8 measurement story.

See [AC-7.md](AC-7.md).

---

## Bonus Sections

### First Consumer of S-8.10 host::write_file

S-8.04 is the first production consumer of the `host::write_file` capability introduced
by S-8.10. The call chain runs: SDK wrapper → dispatcher `invoke.rs` capability check
→ `path_allow` enforcement → `std::fs::write`. Deny-by-default: absent capability
block returns `CAPABILITY_DENIED`. On denial, hook emits `hook.error` and exits 0.

See [bonus-write-file-consumer.md](bonus-write-file-consumer.md) for full call chain
diagram and comparison with `read_file` protocol differences.

### Independent read_file impl in invoke.rs

`crates/factory-dispatcher/src/invoke.rs` contains an inline `read_file` host function
binding (lines ~459–550) that is a **parallel implementation** to the one in
`crates/factory-dispatcher/src/host/write_file.rs`. Both use the same
canonicalize-before-compare path allowlist check. Comment at line 467 notes:
"First consumer: update-wave-state-on-merge (S-8.04 BC-7.03.085)".

This parallelism (inline `invoke.rs` vs dedicated `host/*.rs` module) will need merge
coordination when S-8.07 lands its own `read_file` implementation. Both branches should
be reconciled toward the `host/*.rs` module pattern.

See [bonus-write-file-consumer.md](bonus-write-file-consumer.md) for protocol comparison.

### Standalone feature flag (bats parity testing)

The `standalone` feature (default for debug builds) substitutes WASI `std::fs` for
`vsdd` host functions via `#[cfg(feature = "standalone")]`. File path resolution uses:
1. `VSDD_WAVE_STATE_PATH` env var
2. WASI preopened directory enumeration via raw `fd_prestat_get`/`fd_prestat_dir_name`
   syscalls
3. `.factory/wave-state.yaml` production fallback

This is the first E-8 Tier 1 hook to establish the dual-mode feature flag pattern for
file-I/O hooks. Previous ports (S-8.01, S-8.03) had no file I/O and no analogous pattern.

---

## Recordings Index

| File | AC | Format |
|------|----|--------|
| `AC-001-wasm-crate-exists.gif` | AC-001 | GIF |
| `AC-001-wasm-crate-exists.webm` | AC-001 | WebM |
| `AC-001-wasm-crate-exists.tape` | AC-001 | VHS source |
| `AC-002-sh-deleted-hooksjson-removed.gif` | AC-002 | GIF |
| `AC-002-sh-deleted-hooksjson-removed.webm` | AC-002 | WebM |
| `AC-002-sh-deleted-hooksjson-removed.tape` | AC-002 | VHS source |
| `AC-003-agent-scope-merge-signal.gif` | AC-003 | GIF |
| `AC-003-agent-scope-merge-signal.webm` | AC-003 | WebM |
| `AC-003-agent-scope-merge-signal.tape` | AC-003 | VHS source |
| `AC-004-yaml-append-emit-event.gif` | AC-004 | GIF |
| `AC-004-yaml-append-emit-event.webm` | AC-004 | WebM |
| `AC-004-yaml-append-emit-event.tape` | AC-004 | VHS source |
| `AC-005-gate-status-flip.gif` | AC-005 | GIF |
| `AC-005-gate-status-flip.webm` | AC-005 | WebM |
| `AC-005-gate-status-flip.tape` | AC-005 | VHS source |
| `AC-006-bats-parity-tests.gif` | AC-006 | GIF |
| `AC-006-bats-parity-tests.webm` | AC-006 | WebM |
| `AC-006-bats-parity-tests.tape` | AC-006 | VHS source |
| `AC-007-emit-event-no-bin.gif` | AC-007 | GIF |
| `AC-007-emit-event-no-bin.webm` | AC-007 | WebM |
| `AC-007-emit-event-no-bin.tape` | AC-007 | VHS source |

---

## Commits on Branch

| Hash | Description |
|------|-------------|
| `39f0de3` | Latest commit on feature/S-8.04-native-port-update-wave-state-on-merge |

Rustc 1.95.0 used throughout. All `cargo build/test/clippy` clean.
All bats tests routed through `factory-dispatcher` release build (pre-built at
`target/release/factory-dispatcher`).
