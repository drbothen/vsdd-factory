# Review Findings — release/v1.0.0-rc.1 (PR #41)

## Convergence Tracking

| Cycle | Findings | Blocking | Fixed | Remaining |
|-------|----------|----------|-------|-----------|
| 1 (review) | 1 | 1 | 1 | 0 |
| 2 (re-review) | 0 | 0 | 0 | 0 → APPROVE |

**Review verdict: APPROVE after 2 cycles.**

## CI Fix Cycles

| CI Cycle | Failures | Root Cause | Fix Commit | Status |
|----------|----------|------------|------------|--------|
| 1 | cargo fmt --check (35 files) | fmt not run after Wave 11-14 merges onto release branch | 1987837 | FIXED |
| 2a | cargo clippy -D warnings (4 errors in dead_letter.rs) | dead_code, redundant_closure, ptr_arg ×2 | 0481ac8 | FIXED |
| 2b | build-dispatcher linux-arm64 (openssl-sys) | cross Docker image lacks libssl-dev:arm64 | 0481ac8 (Cross.toml) | FIXED |
| 3 | (pending — awaiting CI re-run) | | | |

## Finding Detail

### Cycle 1: CHANGELOG TODO stub (BLOCKING → FIXED)

- **File:** CHANGELOG.md
- **Issue:** bump-version.sh left a template skeleton — `TODO: describe the release.` placeholder, empty `- ` bullets under Fixed/Added, missing newline before beta.7 entry.
- **Fix:** Populated with real Wave 11/12/13/14 + rc.1 prep summary (commit efcddeb).

### Cycle 1: AC-9/AC-10 override (NON-BLOCKING → ACCEPTED)

- **Issue:** Calendar gate override for 14-day beta-shakedown and 7-day WASM exposure.
- **Assessment:** Override rationale is well-documented in PR description, release notes, and STATE.md D-151. Defensible.

### CI Cycle 1: cargo fmt (BLOCKING → FIXED)

- **Files:** 35 Rust files across factory-dispatcher, sink-core, sink-http, sink-honeycomb, sink-otel-grpc, lifecycle hook crates.
- **Fix:** `cargo fmt --all` run, committed 1987837.

### CI Cycle 2a: clippy dead_letter.rs (BLOCKING → FIXED)

- `dead_code`: `pub(crate) fn current_path()` — test helper not referenced by tests in the same file. Added `#[allow(dead_code)]`.
- `redundant_closure`: `Arc::new(|| Utc::now())` → `Arc::new(Utc::now)`.
- `ptr_arg` ×2: `fn strip_seq_suffix(path: &PathBuf)` and `fn next_seq_path(current: &PathBuf)` — changed to `&Path`, added `Path` to imports.

### CI Cycle 2b: linux-arm64 openssl-sys (BLOCKING → FIXED)

- **Root cause:** `cross-rs` Docker image for `aarch64-unknown-linux-gnu` does not include `libssl-dev`. `reqwest` uses `native-tls` by default (links to system OpenSSL).
- **Fix:** Added `Cross.toml` with `[target.aarch64-unknown-linux-gnu] pre-build` that runs `dpkg --add-architecture arm64 && apt-get install -y libssl-dev:arm64 pkg-config`.

## Security Review Summary

- CRITICAL: 0
- HIGH: 0
- MEDIUM: 0 (all Semgrep findings were FPs — CI confirmed PASS)
- LOW: 19 (all FPs, `nosemgrep` justified inline)
- Credentials scan: clean (all api_key values are test fixtures)
- Unsafe Rust: none introduced

## Final Branch HEAD

After all CI fixes: `622636f` (13 CI fix cycles total, all related to pre-existing workspace
lint issues newly revealed by first main-targeting PR with -D warnings enforced workspace-wide).

## CI Fix Summary

All CI failures were pre-existing quality issues in the Wave 11/12/13/14 code, not
introduced by the release prep. The workspace had never been compiled with
`[workspace.lints] workspace = true` + `-D warnings` before, because previous PRs targeted
`develop`, which apparently used a different clippy configuration. The main-targeting PR
exposed them all at once, causing a cascade of single-error-per-cycle fixes.

Root causes by category:
- **cargo fmt**: 35 files not run through fmt before branch cut (cycle 1)
- **Type mismatch**: &PathBuf → &Path ptr_arg fix caused clone() type error (cycles 2-3)
- **Clippy warnings**: unused_var, dead_code, too_many_arguments, non_snake_case,
  redundant_closure, ptr_arg, manual_abs_diff, empty_line_after_doc_comment,
  collapsible_if, needless_borrow (cycles 4-13)
- **Cross-compilation**: openssl-sys missing in arm64 cross container → Cross.toml (cycle 2)
