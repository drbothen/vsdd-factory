# PR #798 — Fresh-Eyes Pre-Merge Review

**PR:** feat(S-17.05): stamp-state-timestamp PostToolUse WASM hook (ADR-046, BC-4.17.001)
**Head:** feature/S-17.05 @ `bdb659472228e3efa8ab01cfacb46a11d2016fb4`
**Base:** develop
**Reviewer:** pr-reviewer (fresh-eyes diff review)
**Date:** 2026-08-28

## VERDICT: REQUEST_CHANGES

The diff itself has **0 blocking findings** and is production-grade. However, the required CI check `cargo-host (ubuntu-latest)` is currently **RED**, so the PR is **not safe to merge yet**. The failing tests are NOT caused by this diff (see Blocking section).

covered_sha: `bdb659472228e3efa8ab01cfacb46a11d2016fb4`

---

## Diff Review — 0 blocking findings

Verified across spec-fidelity (BC-4.17.001 / BC-5.40.001), code quality, test completeness, and security:

- **`guard_logic`** (`crates/hook-plugins/stamp-state-timestamp/src/lib.rs`): PC1 unconditional `timestamp:` re-stamp; PC2 identity-gated renewal delegated to `factory_lock::renew_lock_if_holder`; PC3 fail-open on every read / parse / non-UTF-8 / write error; GAP-3 `tool_response` success gate; GAP-1 no-timestamp-anchor path; CRLF-aware opening-fence offset and empty-frontmatter guard. Correct.
- **Anti-resurrection (SAFETY-CRITICAL, Invariant 2):** expired self-held locks (Case 2), foreign holders (Case 3), and resolution failures (Case 4) are never renewed. Logic delegated, not reimplemented.
- **CRLF preservation** (`crates/factory-lock/src/lib.rs` `rewrite_expires_at`): switched from whole-file `\r\n`→`\n` normalization to `split_inclusive('\n')` with per-line terminator mirroring. Byte-for-byte preserving for all lines; only the `expires_at` value line changes. Confirmed by `renew.rs` mutation-kill tests (terminator preservation + whole-file-normalization regressions).
- **Canonical constants** moved to `factory-lock-parse` (`TTL_SECONDS=2700`, `STATE_MD_MAX_BYTES=262144`); `renew_lock_with_now` migrated off the bare `2700` literal. Single-source respected (Architecture Compliance Rule 7).
- **`hooks-registry.toml`:** `stamp-state-timestamp` added (PostToolUse, `^(Edit|Write|MultiEdit)$`, `on_error=continue`, `async=false`, `read_file`/`write_file` path_allow scoped to `.factory/STATE.md`, `exec_subprocess` binary_allow=`[git]`, env scoped). `verify-state-timestamp-refresh` entry removed. Docs (`state-manager.md`, `state-burst/SKILL.md`, `factory-lock-write.sh`) updated consistently.
- **Tests:** this PR's crates all pass in CI (stamp-state-timestamp unit suites, factory-lock `renew.rs` CRLF tests, factory-lock-parse constant tests).
- **Security:** fail-open PostToolUse (no block capability), no user-controlled input to subprocess (`git config user.email` only), WASM path sandbox, identity compared by exact string equality. Sound.

### Non-blocking nit (optional)
The GAP-4 soft-warn band in `lib.rs` hardcodes `262_144` / `"262144"` as literals instead of reusing `flp::STATE_MD_MAX_BYTES`. Cosmetic only; does not affect correctness.

---

## BLOCKING for merge — NOT caused by this diff

CI check **`cargo-host (ubuntu-latest)` = FAIL**. The `cargo test (workspace, all targets)` step failed on 2 tests in `crates/hook-plugins/validate-state-structure/src/lib.rs` — a crate this PR does not touch:

- `test_BC_5_39_005_f_p1_001_real_state_md_banner_wc_passes` (lib.rs:2548)
- `test_BC_5_39_005_full_validation_against_real_state_md` (lib.rs:2291)

Both read the live mounted `.factory/STATE.md` and assert its SIZE BUDGET banner contains an `N lines (wc-l)` claim (per D-421(c)+D-422(c)+D-424(b)+D-428(d)+D-438(a)+D-440(d)+D-442(d)). The current STATE.md banner lacks that pattern, so the assertions panic. This is a STATE.md-content issue on `factory-artifacts` — environmental to any PR's CI right now — not a defect in the S-17.05 diff.

**Recommended route:** `state-manager` restores the STATE.md SIZE BUDGET banner's `N lines (wc-l)` claim (or confirms these real-STATE.md-coupled tests are a tracked pre-existing failure), then re-run CI. Once `cargo-host` is green (and the pending `build-dispatcher` / `bats-full-suite` jobs pass), the diff is approvable as-is at `bdb659472228e3efa8ab01cfacb46a11d2016fb4` — no code changes to S-17.05 required.
