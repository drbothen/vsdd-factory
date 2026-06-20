# PR #196 — Cycle-2 Fresh-Eyes Review (post security-hardening)

**Verdict: APPROVE**
**Blocking findings: 0** (0 warning, 0 nit)

Reviewing commit `9df466e9` "fix(S-18.13): security hardening — explicit input
validation guards (SEC-001/002/003/005)" on top of the cycle-1 APPROVE.

## What the security fixes look like

The commit adds defense-in-depth input-validation guards across three files. No
behavioral logic was rewritten — every change is an additive guard positioned
after the variable is set and before it is used:

1. **`wave-handoff.sh` (SEC / CWE-73 path validation)** — Validates `ARTIFACTS_WT`
   is an existing, accessible git worktree before any I/O. Platform-aware: GNU
   `realpath -e` canonicalizes + existence-checks on Linux; on BSD/macOS it does
   an existence check only (`-d`) and deliberately skips symlink resolution. The
   inline comment correctly explains why: BSD `realpath` resolves `/var → /private/var`,
   which would break the relative-path stripping in write-handoff.sh. Then
   `git -C "$ARTIFACTS_WT" rev-parse --git-dir` confirms it is a real repo/worktree.

2. **`write-handoff.sh` (SEC / CWE-116 YAML-injection guard)** — Validates
   `factory_lock_holder` against `^[a-zA-Z0-9:._/-]+$` before YAML interpolation.
   The guard is correctly gated behind `[[ -n "${factory_lock_holder:-}" ]]` so an
   absent lock holder is skipped rather than rejected.

3. **`parse-sprint-state.sh` (SEC / CWE-116 + CWE-20 guards)** — Three guard classes:
   story-id pattern (`^S-[0-9]+\.[0-9a-zA-Z._-]+$`), status allowlist (10 statuses),
   and wave_id integer-range `[1, 9999]` on both the derive and the fallback path.

## Checklist verification

1. **Correctness without breaking tests** — `bats wave-handoff.bats` = **63/63 pass,
   0 fail** after the commit. Cycle-1's count is preserved exactly.

2. **Guard positioning** — Verified correct. Every guard runs after the variable
   is assigned (`current_id`/`current_status` via awk, `_ordinal` computed,
   `factory_lock_holder` resolved, `ARTIFACTS_WT` from args) and before downstream
   use (broken/next-wave classification, YAML emission, git operations).

3. **No incorrect rejection of valid production inputs** — Independently exercised
   each regex/allowlist:
   - `factory_lock_holder` regex ACCEPTS `vsdd-factory:state-manager`,
     `vsdd-factory:orchestrator`, `agent-123`, `session/abc-def`; REJECTS spaces,
     `;`, `$(...)`. Empty is skipped (gated). Correct.
   - story-id regex ACCEPTS `S-18.13`, `S-5.39`, `S-12.08`, `S-1.0-feature`;
     REJECTS `E-18`, `S-18`, `S-NNN.NNN`. Correct.
   - status allowlist `{merged,withdrawn,cancelled,pending,draft,in-progress,
     review-pending,ready,complete,blocked}` includes both test fixtures
     (`review-pending` line 248, `in-progress` line 257) that drive the
     BrokenSprintState / AC-018 paths. These still flow to the `else`/broken branch
     (the allowlist does NOT mask the broken-state detection — it sits earlier).
     `test_review_pending_triggers_broken_sprint_state` (#20) and the broken-sprint
     canonical-message test (#19) both pass.
   - ARTIFACTS_WT git guard: the test harness creates ARTIFACTS_WT via
     `git worktree add` (line 80), mirroring production where ARTIFACTS_WT = the
     factory-artifacts worktree root. `git rev-parse --git-dir` succeeds inside
     worktrees/subdirs, so the guard validates correctly. Confirmed against the
     real `.factory` worktree.

4. **`set -euo pipefail` intact** — Present in all three files (line 5 in
   wave-handoff.sh, line 6 in both libs). Unchanged.

5. **New issues introduced by the commit** — None found. `bash -n` syntax-clean on
   all three files. `shellcheck -S warning` produces only two pre-existing SC2034
   "appears unused" warnings on cross-file globals (`BROKEN_STORY_IDS`,
   `CLASSIFY_RESULT`) — these are NOT in the changed lines and predate this commit.
   The new guard lines themselves are shellcheck-clean.

## Behavioral-change note (non-blocking, MORE strict)

The status allowlist changes behavior for genuinely-unknown statuses (e.g. a typo
like `pendign`): pre-commit these reached the `else`→broken-sprint-state path;
post-commit they hard-`exit 1` with an explicit error at parse time. This is a
fail-louder, earlier-detection improvement consistent with the production-grade
default — not a regression. All known/valid statuses retain their prior
classification behavior.

## Conclusion

The security hardening is production-grade: additive, well-positioned,
platform-aware, with accurate explanatory comments and no regressions. The MEDIUM
findings (SEC-001/002/003) and a LOW (SEC-005) are addressed with explicit guards
rather than paper-fixes (each guard is load-bearing and exercised by the test
substrate). 63/63 bats green. Cycle-1 APPROVE stands; the security work does not
disturb it.

**APPROVE — 0 blocking findings.**
