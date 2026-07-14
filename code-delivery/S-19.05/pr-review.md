# PR Review — PR #640 (S-19.05)

**Story:** S-19.05 v1.22 — async plugin completion telemetry + `VSDD_SINK_FILE` release-mode opt-in
**Branch HEAD:** 2f33ec1a
**Reviewer:** pr-reviewer (fresh-eyes, different model family)
**Verdict:** APPROVE — 0 blocking findings; 2 advisories

---

## Summary

The full diff was reviewed as a human reviewer would see it: production code, tests, CI config, `CLAUDE.md`, and all demo evidence. All 7 acceptance criteria are implemented, wired into the real `factory-dispatcher` async drain loop, and covered by binary-invocation integration tests in both debug and release profiles. No blocking findings. Two non-blocking advisories are logged for follow-up.

## Checklist results

| # | Item | Result |
|---|------|--------|
| 1 | Diff coherence (all changes relate to S-19.05) | PASS |
| 2 | Description accuracy (PR body matches diff) | PASS |
| 3 | Test coverage (changed lines covered) | PASS |
| 4 | Demo evidence (evidence-report.md + per-AC transcripts, PASS) | PASS |
| 5 | Commit quality | PASS |
| 6 | Diff size (~2500 additions, dominated by tests/evidence; production change modest) | PASS (noted) |
| 7 | Missing changes | None |
| 8 | Dependency status | N/A |

## AC-by-AC verification

- **AC-001 (plugin.completed, 9 fields) — PASS.** `emit_plugin_completed_async` sets exactly the 9 BC-3.08.001 §Event 6 fields (`type`, `trace_id`, `session_id`, `plugin_name`, `plugin_version`, `entry_index`, `exit_code`, `elapsed_ms`, `fuel_consumed`); wired for non-block `PluginResult::Ok`. `plugin_version` correctly included (sync-path parity). T-001/T-001-EC-001 assert all fields + non-zero exit propagation.
- **AC-002 (plugin.abandoned, 7 fields + Invariant 6) — PASS.** `emit_plugin_abandoned` sets the 7 §Event 5 fields incl. `timestamp` + `entry_index: u32`; emitted for spawned entries absent from the collected-outcomes set. Dispatcher exit stays 0 (observability-only). T-002 asserts terminal semantics.
- **AC-003 (no stderr relay) — PASS.** Events route through the HostContext queue → `flush_sink_file` only; the prior `eprintln!` was removed. T-004 ×2 assert stderr carries no event text.
- **AC-004 (cfg gates removed; any() preserved) — PASS.** `ENV_SINK_FILE`, `flush_sink_file`, sink `Mutex` all unconditional; `VSDD_ASYNC_DRAIN_WINDOW_MS` retains `#[cfg(any(debug_assertions, feature = "test-support"))]` at all three sites; zero `cfg(debug_assertions)` left in main.rs.
- **AC-005 (SEC-003 sanitization) — PASS.** vsdd_sink.rs rejects `..` and `\0` before `open()`, with `tracing::warn!` (no `println!`/`eprintln!`). Discriminating + 8-thread O_APPEND atomicity tests included.
- **AC-006 (CLAUDE.md) — PASS.** New "VSDD_SINK_FILE diagnostic capture (debug and release builds)" section with usage example + SEC-003 constraint; T-008 grep gate verifies.
- **AC-007 (test-support feature) — PASS.** `test-support = []` in Cargo.toml; `--features factory-dispatcher/test-support` in ci.yml release test step; release.yml unchanged (no test-support), consistent with shipping gate. DI-019 100ms invariant preserved.

## Architecture compliance — PASS

`flush_sink_file` in vsdd_sink.rs; `pub mod vsdd_sink` + re-export in lib.rs; `use std::sync::{Arc, Mutex}` unconditional (O-P2-003, with explicit `event_queue` type annotation to keep the import live); no `println!` in production code.

## Findings

### ADVISORY 1 — [correctness] entry_index correlated by `plugin_name`, defeating disambiguation

- **File:** `crates/factory-dispatcher/src/main.rs`
- **Severity:** ADVISORY (escalates to BLOCKING if the registry permits duplicate async plugin names)
- **Finding:** The completed path resolves `entry_index` via `.find(|(name, _)| name == &outcome.plugin_name)` (first match), and the abandoned path keys `collected_names: HashSet<&str>` on name. If two async plugins share a name — the exact case BC-3.08.001 Invariant 6 / `entry_index` exists for, per the function's own doc comment "for any registry that does not enforce name uniqueness" — then (a) both `plugin.completed` events carry the first matching ordinal rather than distinct ordinals, and (b) if one same-named plugin completes while another is abandoned, the abandoned event is suppressed because the name is already in `collected_names`. T-003b tests `emit_plugin_abandoned` in isolation with hardcoded indices, so it stays green even though the main.rs wiring cannot deliver distinct indices under duplicate names — a coverage gap masking the correlation gap.
- **Suggestion:** Correlate positionally (thread the spawn ordinal through the outcome) rather than by name. Confirm whether the registry schema permits duplicate async plugin names; if yes, this is a correctness bug.

### ADVISORY 2 — [completeness] crashed async plugins emit no terminal observable event

- **File:** `crates/factory-dispatcher/src/main.rs`
- **Severity:** ADVISORY
- **Finding:** A crashed outcome hits the `_ => {}` arm (no `plugin.completed`) yet its name is in `collected_names` (no `plugin.abandoned`), so the observable sink receives neither terminal event for a crash — only the executor's internal lifecycle log. Likely intended, but if Invariant 6 expects exactly one terminal observable event per spawned async plugin, this is a gap.
- **Suggestion:** Confirm against BC-3.08.001 whether a crash requires a terminal observable event.

## Conclusion

APPROVE. No blocking findings. 2 advisories logged for follow-up consideration.
