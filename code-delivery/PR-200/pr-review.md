# PR #200 Review — fix/develop-ci-robustness → develop

**Verdict: APPROVE**

Fresh-eyes review of the diff, the two changed files in full context, and live verification against the production STATE.md input. CI gate (fmt + clippy + `cargo test -p validate-dispatch-advance`) reproduced GREEN locally; 42/42 tests pass on the changed crate including the live-STATE.md regression test.

This is a clean, well-targeted maintenance PR. Both root causes are real, the fixes are grounded in the actual production input format (not speculative), and the new tests provide genuine RED→GREEN coverage. No blocking or major findings.

---

## What I verified

1. **Live production input matches the fix's assumption.** The current `.factory/STATE.md` frontmatter carries `current_cycle: v1.0-brownfield-backfill` and a bare dispatch-step `current_step: "D-689-S18.14-3CLEAN-CONVERGED-PROMOTION-2026-06-22"`. So in production `extract_current_cycle` → `Some("v1.0-brownfield-backfill")` → `is_f5_cycle` → `false` → the 4-index-cite and trajectory-tail checks are correctly skipped. The fix is anchored to the real input, not a hypothetical one.

2. **Single chokepoint, no missed sibling callsites (focus #5).** `validate_state_md` is the only non-test caller of both `check_index_version_cites` and `check_trajectory_tail_length` (grep-confirmed across `crates/`). Placing the gate inside `validate_state_md` covers every production path. There is no `extract_current_step` callsite that needed a parallel `extract_current_cycle` and was missed — the plugin entry point (~line 955) only uses `extract_current_step` for the fail-open presence check, which is orthogonal to cycle identity.

3. **`is_f5_cycle` predicate is correct (focus #1).** Exact-string equality against the sole F5 id `v1.0-feature-engine-discipline-pass-1`. No prefix/substring fuzz that could misclassify a future `...-pass-2` cycle as F5 — which is the conservative-correct behavior (a new cycle name would fall through to the `None`-style F5-apply path only if `current_cycle` were absent; a *present* non-matching value is treated as non-F5, which is the documented intent).

4. **Conservative fallback is correct and tested (focus #2).** `None → true` (apply F5 checks) never silently disables a guard on a genuine-but-malformed F5 state file. Directly covered by `test_is_f5_cycle_none_is_conservative`, plus `test_extract_current_cycle_absent` proving the `None` arm is reachable. In production this fallback is a backstop only — the real path is `Some(...)`.

5. **`check_d_chain_currency` still runs unconditionally and the brownfield ZERO-violation assertion is sound.** The brownfield test input cites `D-689` in current_step and `D-689` in the body, so `max_cited (689) >= max_in_file (689)` → no violation. The test genuinely exercises all unconditional checks (`check_forbidden_meta_commentary`, `check_d_chain_currency`), not just the gated ones — so "zero violations" is a real assertion, not an artifact of the gate.

6. **MSRV safety.** `is_none_or` (stabilized Rust 1.82) is safe under the pinned channel 1.95.0 / `rust-version = "1.95"`. Clippy `-D warnings` and fmt both clean on the changed crates locally.

7. **Timing-bound justification (focus #4).** The 450ms bound is well documented: the deterministic no-backoff proof is `attempt_count == 1` (asserted above the timing check); the timing assertion is explicitly belt-and-suspenders and references the +300ms CI-overhead allowance used by the 2-sleep sibling test. A genuine retry would add a 500ms base sleep, which still clears 450ms by a wide margin — so the relaxation does not weaken the test's discriminating power.

---

## Findings

| # | Severity | Category | File | Finding | Suggestion |
|---|----------|----------|------|---------|------------|
| 1 | MINOR | coverage | `crates/hook-plugins/validate-dispatch-advance/src/lib.rs` (`test_f5_cycle_still_enforces_index_and_trajectory_checks`) | The F5-enforcement test's final assertion uses an OR: `descs.any(\|d\| d.contains("BC-INDEX v") \|\| d.contains("trajectory-tail"))`. The synthetic F5 input is missing *both* the 4-index cites and the `trajectory-tail ` prefix, so it should yield *two* violations — but the OR only proves *at least one* of the two F5 checks fired. If a future refactor accidentally dropped one of the two gated checks (e.g. removed `check_index_version_cites` from the `apply_f5_checks` block), this test could still pass on the surviving check. It does not strictly prove *both* F5 checks remain wired. | Strengthen to assert both independently, e.g. `assert!(descs.iter().any(\|d\| d.contains("BC-INDEX v")))` AND `assert!(descs.iter().any(\|d\| d.contains("trajectory-tail")))`. The dedicated per-check unit tests already cover each in isolation, so this is a robustness nicety, not a coverage gap that blocks merge. |
| 2 | COSMETIC | coherence | `crates/hook-plugins/validate-dispatch-advance/src/lib.rs` (`extract_current_cycle`) | `extract_current_cycle` is a near-verbatim clone of `extract_current_step` — the frontmatter-delimiter scan, BOM strip, char-boundary guard, and quote-strip logic are duplicated line-for-line, differing only in the key string (`current_cycle:` vs `current_step:`). | Optional: extract a shared `extract_frontmatter_value(content, key) -> Option<&str>` helper and have both call it. Low priority — the duplication is small, self-contained, and both copies are individually unit-tested, so the maintenance risk is minimal. Not worth blocking a CI-fix PR. |

---

## Checklist

- [x] **Diff coherence** — both changes map to exactly the two stated root causes; no unrelated edits.
- [x] **Description accuracy** — PR body matches the diff precisely (helper + gate + tests for FIX 1; bound relaxation + comment for FIX 2).
- [x] **Test coverage** — every new branch covered: brownfield no-false-positive (RED→GREEN), F5-still-enforces, extract on brownfield/F5/absent, conservative-None, plus the live-STATE.md production regression test.
- [x] **Demo evidence** — N/A for a CI-fix maintenance PR (fix-pr-delivery flow); test evidence is the appropriate artifact and is present.
- [x] **Commit quality** — conventional, scoped.
- [x] **Diff size** — 222 insertions / 10 deletions across 2 files. Well within bounds.
- [x] **Missing changes** — none; both root causes fully addressed.
- [x] **Dependency status** — no upstream PR dependencies.

**Recommendation: APPROVE.** Finding #1 (test-rigor strengthening) is worth a follow-up but does not gate this merge — the underlying behavior is correct and separately unit-tested.
