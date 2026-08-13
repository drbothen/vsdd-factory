# PR #775 Review — S-21.09 validate-factory-path-staging.wasm Artifact Restore + Registry Parity Gate

## Verdict: APPROVE — merge-ready

`covered_sha: c20cf2fe` (final fresh-eyes review at PR head; supersedes the prior `1c93f499` verdict — the Windows TOML-path portability fix `c20cf2fe` landed on top and is covered here)

Base `develop`, head `feature/S-21.09` @ `c20cf2fe`. No blockers. Two documented low-severity observations, both already recorded as accepted residuals / scoped deferrals; neither blocks merge.

---

## Verification performed (evidence, not assertion)

Ran the actual PR-head code in an isolated detached worktree at `c20cf2fe` (the session working tree is on a different branch, so an in-place build would have tested the wrong code):

- `cargo test -p factory-dispatcher --test bundle_orphan_check --release` → **51 passed; 0 failed**
- new `registry.rs` unit test `on_error_falls_back_to_registry_defaults_when_entry_omits_it` → **1 passed**
- `cargo fmt --check --all` → **exit 0**
- `cargo clippy -p factory-dispatcher --tests --release -- -D warnings` → **clean, no warnings**
- `git ls-tree c20cf2fe` confirms `validate-factory-path-staging.wasm` is committed in the HEAD tree at **193,427 bytes** (matches PR body); the `hook-plugins/` dir is gitignored, so the `git add -f` approach was necessary and correct.

## Checklist findings

1. **Diff coherence** — Clean. All 21 files map to S-21.09. No leftover debug code; the only two `todo!()` grep hits are inside comments documenting TDD RED-state history, not live code. No `dbg!`/`println!`/`FIXME`/commented-out blocks; no `#[ignore]`.

2. **Tests exercise the gate (not tautological)** — Confirmed. `T-012` runs the real `run_t012_gate(&workspace_root())` against the live registries + git-tracked set (genuine end-to-end, not a replica). Fixture controls call the real gate functions per the no-replica discipline. Spot-checked the assertions the earlier PR-review flagged as vacuous: `T-026(a)/(b)` now assert load-bearing `refs.is_empty()`, and the removed bare-basename checks are explicitly documented as vacuous-and-why. The new `registry.rs` test is a real mutation-kill (entry `on_error=None` + `defaults.on_error=Block` distinguishes live code `Block` from the `Default::default()` mutant `Continue`, with a premise sub-assertion pinning `Continue` as default).

3. **Windows portability fix (T-026) — correct and complete.** `PathBuf::push` emits the OS-native separator (`\` on Windows); interpolated into a TOML `plugin = "..."` string literal, `\` is an invalid escape and panics `Registry::parse_str`. The fix builds the path with explicit `/` in a `String`. Minimal and correct; it is the only place in the file that constructed a registry-path literal via `PathBuf`, and the forward-slash form correctly mirrors production semantics (registry `plugin` values are always forward-slash regardless of host OS).

4. **PR description accuracy** — Consistent. 51 tests in `bundle_orphan_check` + 1 in `registry.rs` = the 52/52 badge; WASM byte count matches; declared=tracked=36 claim is validated by the passing real-gate test.

5. **Demo evidence** — Present and adequate. GIF+WebM per AC plus a captured-log for the zero-skip claim; both success (AC-006 happy path) and failure (AC-006 orphan-fires) paths covered. `evidence-report.md` present.

## Observations (LOW / non-blocking, already documented)

- **LOW — `enabled = false` false-positive class:** `parse_plugin_refs` extracts every `plugin` value regardless of `enabled` state, so a disabled-but-untracked entry would fire a false-positive `MISSING`. Currently latent (0 disabled entries), fail-loud, documented in-code per POLICY 13. Acceptable.
- **LOW — non-recursive `fs::read_dir` in `collect_orphans_dual` (F8):** affects only staging-simulation tests (T-010/T-011), not the git-based gates (T-009/T-012 handle nested paths correctly). Documented pre-existing limitation scoped to S-21.14 with a concrete future-story anchor. Legitimate deferral.
- **NIT — file size:** `bundle_orphan_check.rs` is 5,555 lines (+5,135), over the 500-line flag. Intentional mutation-isolation density per the production-grade default; well-sectioned and navigable. Noted, not objecting.

## Bottom line

The gate is genuinely reachable and load-bearing, the artifact gap it guards is actually closed (WASM committed + real end-to-end gate green), the previously-flagged vacuous assertions are demonstrably fixed, the Windows fix is correct, and CI gates (test/fmt/clippy) pass at the exact PR head. **APPROVE — merge-ready.**
