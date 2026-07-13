---
story_id: S-19.05
title: "Observability gaps: async plugin completion telemetry + VSDD_SINK_FILE release-mode opt-in"
version: "1.22"
recorded: 2026-07-13
branch: feature/S-19.05
head: 405a871f
product_type: Rust binary + library (no UI)
evidence_mode: captured-stdout test transcripts + live dispatcher binary invocations + grep gates
---

# Demo Evidence — S-19.05

**Story:** S-19.05 — Observability gaps: async plugin completion telemetry + VSDD_SINK_FILE release-mode opt-in
**Epic:** E-19 — Post-rc.22 Operator Hardening
**BC gate:** BC-3.08.001 §Event 5 (plugin.abandoned), §Event 6 (plugin.completed async path), §Invariant 6
**Closes:**
- rc.22 smoke finding (c): async plugins emit plugin.invoked but never plugin.completed — async hangs invisible below 5000ms timeout
- rc.22 smoke finding (d): VSDD_SINK_FILE gated #[cfg(debug_assertions)] — release dispatcher ignores sink env var

This story delivers changes to the `factory-dispatcher` Rust binary and its supporting `vsdd_sink` library module. Evidence is provided as:
1. Captured-stdout transcripts from `cargo test` binary-invocation tests (bc_3_08_001_s19_05.rs)
2. Live dispatcher binary invocations with real WAT fixtures + VSDD_SINK_FILE (money shots)
3. Grep gate verification for static code properties (cfg gates, CLAUDE.md documentation)

---

## Coverage Matrix

| AC | Criterion (summary) | Test(s) | Transcript | Result |
|----|---------------------|---------|------------|--------|
| AC-001 | Async plugin exits 0 within drain → `plugin.completed` emitted with all 9 mandatory fields per BC-3.08.001 §Event 6 | T-001, T-001-EC-001 (binary invocation) | transcript-AC001-AC002-test-suite.txt, transcript-AC001-money-shot-completed.txt | PASS |
| AC-002 | Drain timer fires with in-flight plugin → `plugin.abandoned` emitted with all 7 mandatory fields; Invariant 6 terminal; exit code unchanged | T-002, T-002-EC-002 (binary invocation) | transcript-AC001-AC002-test-suite.txt, transcript-AC002-money-shot-abandoned.txt | PASS |
| AC-003 | plugin.completed and plugin.abandoned NOT relayed to dispatcher stderr | T-004 ×2 (binary invocation) | transcript-AC003-stderr-check.txt | PASS |
| AC-004 | VSDD_SINK_FILE honored at runtime in both builds; cfg(debug_assertions) gates around ENV_SINK_FILE/flush_sink_file/sink Mutex removed; any() gate preserved | T-005, T-006 + grep gates | transcript-AC004-grep-gates.txt | PASS |
| AC-005 | SEC-003 path traversal sanitization preserved in all builds; absolute paths accepted | T-007 (integration) + unit tests + concurrent atomicity | transcript-AC005-sec003-tests.txt | PASS |
| AC-006 | CLAUDE.md Factory Hook Diagnostics updated: VSDD_SINK_FILE documented for debug and release builds, usage example, SEC-003 constraint | T-008 + grep gate | transcript-AC006-claude-md-grep.txt | PASS |
| AC-007 | VSDD_ASYNC_DRAIN_WINDOW_MS gate is any() form; test-support feature absent from release.yml; release profile proof 10/10 | Mechanism gate + T-009 (release profile) + shipping gate | transcript-AC007-release-proof.txt | PASS |

---

## AC-001: Async plugin.completed Event — All 9 Mandatory Fields

**Transcripts:** `transcript-AC001-AC002-test-suite.txt` (T-001 test results), `transcript-AC001-money-shot-completed.txt` (live JSONL)

The money shot — real `plugin.completed` JSONL from a live dispatcher binary invocation with a WAT_EXIT_0 async fixture:

```json
{
  "type": "plugin.completed",
  "ts": "2026-07-13T18:30:09-0500",
  "ts_epoch": 1783985409,
  "schema_version": 1,
  "trace_id": "26160c7d-ab71-4f8c-8f63-75f32722901d",
  "session_id": "money-shot-completed",
  "plugin_name": "demo-async-exit0",
  "plugin_version": "0.0.1",
  "elapsed_ms": 0,
  "entry_index": 0,
  "exit_code": 0,
  "fuel_consumed": 1
}
```

All 9 mandatory BC-3.08.001 §Event 6 fields present: `type`, `trace_id`, `session_id`, `plugin_name`, `plugin_version`, `entry_index`, `exit_code`, `elapsed_ms`, `fuel_consumed`. Note `plugin_version` and `entry_index` are async-path-specific (not in Events 1, 4, 5).

```
running 2 tests
test test_BC_3_08_001_s19_05_t001_async_exit0_within_drain_emits_plugin_completed ... ok
test test_BC_3_08_001_s19_05_t001_ec001_async_nonzero_exit_emits_completed_with_actual_exit_code ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 1.30s
```

**BC Trace:** BC-3.08.001 §Event 6 (plugin.completed async path, 9 mandatory fields); VP-100.

---

## AC-002: Async plugin.abandoned Event — All 7 Mandatory Fields + Invariant 6

**Transcripts:** `transcript-AC001-AC002-test-suite.txt` (T-002 test results), `transcript-AC002-money-shot-abandoned.txt` (live JSONL)

The money shot — real `plugin.abandoned` JSONL from a live dispatcher binary invocation with a WAT_INFINITE_LOOP fixture and 50ms drain window:

```json
{
  "type": "plugin.abandoned",
  "ts": "2026-07-13T18:30:24-0500",
  "ts_epoch": 1783985424,
  "schema_version": 1,
  "trace_id": "a3745bd2-54ec-451d-9812-56603b3c8346",
  "session_id": "money-shot-abandoned",
  "plugin_name": "demo-async-slow-plugin",
  "drain_window_ms": 50,
  "entry_index": 0,
  "timestamp": "2026-07-13T18:30:24-0500"
}
```

All 7 mandatory BC-3.08.001 §Event 5 fields present: `type`, `trace_id`, `session_id`, `plugin_name`, `entry_index`, `drain_window_ms`, `timestamp`. Dispatcher exit_code=0 confirms observability-only semantics.

```
running 2 tests
test test_BC_3_08_001_s19_05_t002_drain_timer_fires_with_in_flight_plugin_emits_abandoned ... ok
test test_BC_3_08_001_s19_05_t002_ec002_all_complete_before_drain_no_abandoned_events ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 1.39s
```

**BC Trace:** BC-3.08.001 §Event 5 (plugin.abandoned, 7 mandatory fields, entry_index: u32); BC-3.08.001 §Invariant 6 (terminal key trace_id+plugin_name+entry_index); VP-100.

---

## AC-003: Events NOT Relayed to Stderr

**Transcript:** `transcript-AC003-stderr-check.txt`

```
running 2 tests
test test_BC_3_08_001_s19_05_t004_async_completed_event_not_relayed_to_stderr ... ok
test test_BC_3_08_001_s19_05_t004_async_abandoned_event_not_relayed_to_stderr ... ok
test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 8 filtered out; finished in 1.35s
```

Dispatcher stderr contains only the summary line (trace UUID + sync/async count). No plugin.completed or plugin.abandoned text in stderr. Events route exclusively to VSDD_SINK_FILE and the internal JSONL log.

**BC Trace:** BC-3.08.001 §Postconditions (event routing to FileSink); BC-1.14.001 Invariant 4; VP-079.

---

## AC-004: VSDD_SINK_FILE cfg-Gate Removal

**Transcript:** `transcript-AC004-grep-gates.txt`

Static grep evidence (all gates pass):

```
Gate 1 (ENV_SINK_FILE const not cfg-gated in main.rs):     exit 0 — PASS
Gate 2 (flush_sink_file not cfg-gated in vsdd_sink.rs):    exit 0 — PASS
Gate 3 (zero cfg(debug_assertions) in main.rs):            (no output) — PASS
Gate 4 (any() form present for DRAIN_WINDOW_MS):           3 occurrences at lines 77/476/482 — PASS
Gate 5 (T-006: Mutex import not cfg-gated):                test result: ok. 1 passed — PASS
```

**BC Trace:** AC-004 runtime opt-in gate.

---

## AC-005: SEC-003 Path Traversal Sanitization Preserved

**Transcript:** `transcript-AC005-sec003-tests.txt`

```
test test_BC_3_08_001_s19_05_t007_sec003_traversal_rejection_release_profile ... ok
test vsdd_sink::tests::test_flush_sink_file_rejects_traversal_path ... ok
test vsdd_sink::tests::test_flush_sink_file_concurrent_append_no_line_merging_f_p6_001 ... ok
```

Traversal paths rejected silently (tracing::warn). Absolute paths accepted. Concurrent 8-thread O_APPEND atomicity test passes (no JSON line merging). No cfg(debug_assertions) around the SEC-003 guard — it's unconditional in all builds.

**BC Trace:** CLAUDE.md Conventions SEC-003; AC-005.

---

## AC-006: CLAUDE.md Documentation Updated

**Transcript:** `transcript-AC006-claude-md-grep.txt`

```
$ grep -nE "VSDD_SINK_FILE.{1,60}(debug and release|release builds)" CLAUDE.md
378:### VSDD_SINK_FILE diagnostic capture (debug and release builds)
380:As of S-19.05 AC-004, `VSDD_SINK_FILE` is honored in both debug and release builds...
exit code: 0

test test_BC_3_08_001_s19_05_t008_claude_md_vsdd_sink_file_release_documentation_present ... ok
```

All three documentation requirements covered: (1) honored in debug and release builds, (2) usage example with absolute path, (3) SEC-003 path constraint.

**BC Trace:** AC-006 documentation gate.

---

## AC-007: Release Profile Proof + Shipping Gate

**Transcript:** `transcript-AC007-release-proof.txt`

```
Mechanism gate: grep any(debug_assertions, feature = "test-support") → PASS (3 occurrences)

T-009 release profile (10/10):
$ cargo test -p factory-dispatcher --release --features test-support --test bc_3_08_001_s19_05
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.50s

Shipping gate:
$ ! grep -qE 'test-support|--features factory-dispatcher' .github/workflows/release.yml
SHIPPING_GATE_OK: test-support absent from release.yml
```

All three AC-007 sub-gates PASS. DI-019 100ms shipped-binary invariant preserved.

**BC Trace:** DI-019 (shipped-binary 100ms invariant); AC-007.

---

## Full Test Run Summary

```
$ cargo test -p factory-dispatcher --test bc_3_08_001_s19_05
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test -p factory-dispatcher --release --features test-support --test bc_3_08_001_s19_05
test result: ok. 10 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out

$ cargo test --workspace --all-targets
Total: 2010 passed, 0 failed (all crates, all targets, debug profile)

$ cargo fmt --check --all       → FMT: PASS
$ cargo clippy --workspace --all-targets -- -D warnings  → CLIPPY: PASS
```

**Total: 10/10 S-19.05 integration tests GREEN (debug + release profile). 2010 workspace tests GREEN. 0 failures.**

---

## Notes on Evidence Mode

This story delivers changes to `factory-dispatcher` (Rust binary) and `crates/factory-dispatcher/src/vsdd_sink.rs` (library module). The bc_3_08_001_s19_05.rs test suite uses binary invocation via `CARGO_BIN_EXE_factory-dispatcher` to test the actual async drain loop in `main.rs` — not a library replica.

The money-shot transcripts replay the exact test setup: WAT_EXIT_0 (async exit-0 plugin) and WAT_INFINITE_LOOP (async slow plugin) compiled to WASM, fed to the live dispatcher binary with VSDD_SINK_FILE set, and the resulting JSONL rendered with jq.

On branch `feature/S-19.05` (HEAD `405a871f`), all evidence is reproduced by:
```
cargo test -p factory-dispatcher --test bc_3_08_001_s19_05
cargo test -p factory-dispatcher --release --features test-support --test bc_3_08_001_s19_05
cargo test --workspace --all-targets
cargo fmt --check --all
cargo clippy --workspace --all-targets -- -D warnings
```

---

## Behavioral Discrepancies Found

None. All tests pass against the implementation on `feature/S-19.05`. No behavioral discrepancy between the implementation and the ACs was observed.

---

## Files

| File | Content |
|------|---------|
| `transcript-AC001-AC002-test-suite.txt` | 10/10 full test suite run + T-001/T-002 focused runs |
| `transcript-AC001-money-shot-completed.txt` | Live dispatcher binary run: plugin.completed JSONL (9 fields, jq-rendered) |
| `transcript-AC002-money-shot-abandoned.txt` | Live dispatcher binary run: plugin.abandoned JSONL (7 fields, jq-rendered) |
| `transcript-AC003-stderr-check.txt` | T-004 ×2: async events not in dispatcher stderr |
| `transcript-AC004-grep-gates.txt` | grep gates: ENV_SINK_FILE/flush_sink_file/Mutex not cfg-gated; any() form present |
| `transcript-AC005-sec003-tests.txt` | SEC-003 traversal rejection + concurrent atomicity tests |
| `transcript-AC006-claude-md-grep.txt` | CLAUDE.md documentation grep gate + T-008 |
| `transcript-AC007-release-proof.txt` | Mechanism gate + T-009 release 10/10 + shipping gate |
| `transcript-workspace-summary.txt` | Full workspace test count (2010 pass, 0 fail) + fmt/clippy clean |
| `evidence-report.md` | This file — coverage matrix + per-AC narrative + money shot JSONL |
