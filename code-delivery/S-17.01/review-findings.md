# Review Findings — S-17.01

**PR:** #181
**Branch:** feature/S-17.01-factory-lock-schema-cas-push
**Story:** S-17.01 v1.3 — factory_lock STATE.md schema + state-burst CAS push

## Convergence Table

| Cycle | Findings | Blocking | Non-blocking | Fixed | Remaining | Verdict |
|-------|----------|----------|--------------|-------|-----------|---------|
| 1 | 0 | 0 | 0 | 0 | 0 | APPROVE |

## Cycle 1 — Initial Review

**Reviewer:** pr-reviewer (pr-manager cognitive-diversity review)
**Date:** 2026-06-10
**Verdict:** APPROVE

### Findings Summary

No blocking findings. No non-blocking findings.

### Review Notes

**Correctness — factory-lock-write.sh:**
- `set -euo pipefail`: present. PASS.
- TTL_SECONDS=2700 constant: non-configurable, named, correct. PASS.
- Single-epoch capture in `_capture_now_epoch`: both locked_at and expires_at derived from one `date -u +%s` call. PASS (Invariant 3).
- CRLF normalization `_normalize_crlf` called before awk in all three modes (acquire/renew/clear). PASS (F-P1-010 + F-R1-001).
- File mode preservation via `_get_file_mode` (BSD/GNU branch): `stat -f '%Lp'` on macOS, `stat -c '%a'` on Linux. Replaces GNU-only `chmod --reference`. PASS (F-P1-011).
- EXIT trap cleanup for `_TMPFILES`: all mktemp calls go through `_make_tmpfile`; trap fires on any exit. PASS.
- Post-write assertion in acquire (factory_lock block present after write). PASS.
- Post-clear assertion in clear (factory_lock key absent after remove — StaleNullBlock guard). PASS.
- Post-renew assertion in renew (expires_at matches new value). PASS.
- `_remove_factory_lock` awk: frontmatter-boundary-aware (only operates in fence==1 region). PASS.
- `_write_factory_lock_block` awk: inserts before closing --- (not appending to body). PASS.
- `_update_expires_at` awk: frontmatter-boundary-aware; only modifies expires_at sub-key under factory_lock. PASS.
- validate_frontmatter: checks for ≥2 fences; emits SchemaViolation on failure. PASS.
- git config user.email validated non-empty; SchemaViolation emitted if absent. PASS.
- No shellcheck violations (verified via CI path).

**Correctness — factory-cas-push.sh:**
- `set -euo pipefail`: present. PASS.
- Step 1 fetch guard: on non-zero exit, emits exact AC-010 message and exits 1; push does NOT proceed. PASS.
- Step 2 rev-parse guard (`if ! EXPECTED_SHA="$(... 2>&1)")`): when rev-parse fails, `2>&1` redirects git's error output to stdout which is captured in EXPECTED_SHA — BUT the `!` condition catches the non-zero exit before EXPECTED_SHA is used. The variable is set to git's error text, but the conditional exits before any use of that value. NOT a bug; the guard works correctly.
- Step 3 cat-file -e guard: object-existence check before push; handles EC-008b (partial fetch/GC'd object race). PASS (F-R1-003).
- Exact CASPushRejected message strings match AC-005 spec. PASS.
- Exact fetch-failure message matches AC-010 spec. PASS.
- No user-controlled inputs; hardcoded branch name and remote. PASS.

**Test coverage:**
- factory-lock-write.bats: 17 tests; all 10 AC/BC postconditions + Invariants 2/3 + EC-009 + CRLF + file-mode + removal-assertion tests. Complete coverage. PASS.
- factory-cas-push.bats: 5 tests; CAS rejection with real bare-repo fixture (not tautological stub), stale-SHA, object-absent, fetch-failure, hook-unchanged. Complete coverage. PASS.
- Red Gate table in story spec v1.3 matches actual @test names in bats files (F-R1-001/F-R1-002 corrections applied). PASS.

**Spec traceability:**
- All 10 ACs (AC-001 through AC-010) have named bats tests. PASS.
- BC-5.40.001 PC1–PC6, Invariants 2/3/5, EC-003 all covered. PASS.
- state-manager.md obligation table: acquire/renew/clear events documented with precise commands and STATE.md field impact. PASS.
- SKILL.md: blind push replaced with `bash plugins/vsdd-factory/bin/factory-cas-push.sh`; internal CAS sequence documented. PASS.

**Demo evidence:**
- 6 GIF recordings covering all 10 ACs (AC-006/AC-008/AC-009 transitively covered; rationale in evidence-report.md). PASS.
- AC-005 and AC-010 use real git fixtures (not stub git) per Demo Plan v1.3 requirement (F-P1-012). PASS.
- evidence-report.md present and complete. PASS.

**Architecture compliance:**
- factory-lock-write.sh and factory-cas-push.sh placed in `plugins/vsdd-factory/bin/` (precedent: resolve-worktree-identity.sh). PASS.
- verify-git-push.sh unchanged (confirmed by bats test_BC_5_40_001_verify_git_push_hook_unchanged). PASS.
- No Rust crate dependencies added. PASS.
- `state-manager` is sole writer of `factory_lock` (single-writer discipline preserved). PASS.

**VERDICT: APPROVE**

No blocking findings. No non-blocking findings. The implementation is production-grade, spec-faithful, and test-complete. All 10 ACs traced; 22/22 bats tests green; 2-pass adversarial convergence to 3-CLEAN; demo evidence complete.
