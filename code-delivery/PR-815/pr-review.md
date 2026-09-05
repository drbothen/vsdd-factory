## Fresh-eyes PR review — #815

**Verdict: APPROVE (with nits)**

`fix(validate-dispatch-advance): close D-2026 self-referential narrative-literal false positive — structured Decisions-Log scan (BC-5.39.006 v1.9)`

Single-file change to `crates/hook-plugins/validate-dispatch-advance/src/lib.rs` (+627/−16, bulk is tests). New `scan_max_decision_log_id` replaces the whole-body `scan_max_d_nnn` for `max_in_file`, scanning only the `## Decisions Log` h2 section's first pipe-cell (whole-cell `D-\d+`) with GFM separator-row detection and emphasis-decoration normalization. No blockers, no majors.

### What I verified
- **Self-validating claim holds.** Locally: `cargo test -p validate-dispatch-advance` → 56 passed / 0 failed; the previously-red `validate_production_state_md_no_false_positive` is now GREEN with zero STATE.md edits. clippy clean.
- **The change is a strict narrowing** of the `max_in_file` scan, so it cannot introduce new false *positives* — only reduce them (its purpose) or introduce false *negatives* (staleness escapes). The two known false-negative vectors (emphasis-decorated ID cell, `---`-in-prose-cell) are exactly what the v1.9 EC-025/EC-026 tests lock down.
- **Panic safety.** `&trimmed[1..]` is safe (leading `|` is 1-byte ASCII). The emphasis-strip slice only ever cuts on ASCII `* _` backtick runs; multibyte lead/continuation bytes never match `is_emphasis`, so no mid-codepoint slice. The `leading_trim + trailing_trim >= cell.len()` guard short-circuits all-emphasis cells to `""` rather than panicking. No `unwrap`/`expect` in the production path; `parse::<u64>()` is `Ok`-guarded.
- **Separator detection is correct** — whole-row composition test with a `contains('-')` guard retains a data row whose ID cell is `D-NNN` even when other cells are all dashes (the ID cell breaks the `all()`). Cross-checked against the live STATE.md archive row `| D-413..D-1088 (exhaustive) | ARCHIVED | ...` (correctly NOT counted) and the newest bare rows (correctly counted). Live heading is exactly `## Decisions Log`, so the exact-match holds today.

### Findings

| Severity | Location | Finding |
|----------|----------|---------|
| MINOR | lib.rs ~L86 | Exact-heading match (`trimmed == "## Decisions Log"`) silently disables the whole currency check if the heading ever gains an annotation suffix (e.g. `## Decisions Log (D-379..D-1163)`): scan → 0, `max_cited < 0` never true, guard goes dark with green CI. Consistent with invariant-7 fail-open philosophy and not a regression, but converts a heading typo into a silent guard-disable. Consider `starts_with("## Decisions Log")` for section entry. Not merge-blocking; live heading matches exactly. |
| NIT | lib.rs ~L133-146 | Doc says "SYMMETRIC ... runs" but leading/trailing emphasis runs are counted independently, so `**D-1164*` (unbalanced) still normalizes to `D-1164`. Harmless (more lenient); tighten the comment or note the tolerance. Asymmetric case is untested. |
| NIT | lib.rs ~L70-76 | Two-variant `State` enum where a `bool` would do — but justified by mirroring `validate_index_md`'s existing scan technique (consistency). Leave as-is. |
| NIT | — | ~90 lines of doc for ~90 lines of code; errs toward over-documentation. Acceptable; the BC/EC traceability is valuable. |

### Test adequacy — strong
Narrative-literal ignore, fail-open on missing section, false-positive repro, stale-via-Decisions-Log positive control, all three emphasis variants (bold/backtick/underscore), triple-dash data-row retention, genuine GFM colon-alignment separator skip, and a verbatim-STATE.md fixture reproducing the exact 3-quoted + 1-bare `D-2026` production defect — plus wiring through `check_d_chain_currency` asserting the violation cites `D-1164` and not `D-2026`. Only untested gaps are the two NITs above (asymmetric emphasis; annotated-heading fail-open) — neither blocking.

### Merge notes
- Spec (BC-5.39.006 v1.9, EC-024/025/026) lives on `factory-artifacts`, outside this diff — not verifiable from the PR alone; the body's version trail is internally consistent.
- CI at review time: `validate`, SAST, bats-wave-handoff, policy-15 pass; `cargo-host` + `build-dispatcher` still pending (`mergeStateStatus: UNSTABLE`). Wait for `cargo-host` green before merge — local `cargo test --all-targets` + clippy are clean, so it is expected to pass.

Reviewed purely on the diff, description, and test evidence.

https://claude.ai/code/session_01Y45N2GutCTbmvLPG8cv9xu
