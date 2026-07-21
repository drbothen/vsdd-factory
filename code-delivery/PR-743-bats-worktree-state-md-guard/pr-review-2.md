# PR #743 — Fresh-Eyes Review (Pass 2, re-review after develop merge)

**Verdict: APPROVE (CLEAN)**

- **PR:** #743 `fix(tests): skip live STATE.md snapshot suite in worktrees without factory-artifacts mount`
- **Branch:** `fix/bats-worktree-state-md-guard` → `develop`
- **Reviewed SHA:** `d5f42338687acbd89b9a9a69f53ad30b78d731b0` (HEAD)
- **Prior verdict:** APPROVE CLEAN on `9742c4d6` — invalidated (stale) after `origin/develop` was merged in (PR #725 sprint-state CI skip guard landed on develop).
- **Reviewer:** pr-reviewer (fresh-context, information-asymmetry wall: diff + PR description + CI evidence only)

## Why the re-review

The prior APPROVE was on `9742c4d6`. The HEAD SHA changed to `d5f42338` when
`origin/develop` was merged into the branch. This review confirms the develop
merge introduced no conflicts or regressions, and independently assesses the
`_skip_live_artifact_in_ci` guard that entered the branch via that merge.

## Scope of the merge (d5f42338)

The merge commit `d5f42338` (`Merge: 9742c4d6 8f17eea1`) touched **exactly one
file**: `plugins/vsdd-factory/tests/sprint-state-format.bats` (+30 lines). It
did **not** touch the PR's own subject file
`pass-real-state-md-snapshot.bats`. Confirmed by:

```
git diff d5f42338^1 d5f42338 -- .../pass-real-state-md-snapshot.bats   → empty (byte-identical)
git show --stat d5f42338                                                → 1 file changed, sprint-state-format.bats, +30
```

The PR's **net diff vs develop** remains exactly the original 8-line guard in
`pass-real-state-md-snapshot.bats` (`gh pr diff 743` → 1 file, +8/-0). The
sprint-state guard is not in the net diff because it now exists identically on
both develop and this branch (it merged to develop via PR #725).

## Concern (1): Did the develop merge introduce conflicts/regressions in the bats-worktree-state-md-guard changes?

**No.** The subject-file guard is byte-identical to the previously-approved
`9742c4d6` state. Review of the guard at `d5f42338`:

- The guard is placed correctly in `setup()` **before** the `cp
  "$REPO_ROOT/.factory/STATE.md" ...` line it protects.
- Predicate `[ ! -f "$REPO_ROOT/.factory/STATE.md" ]` checks the exact file the
  subsequent `cp` consumes — precise, no false positives/negatives.
- `REPO_ROOT` is defined earlier in `setup()` before the guard uses it.
- `skip` message is actionable and includes the remediation command
  (`git worktree add .factory origin/factory-artifacts`).
- `teardown()` guards `${WORK:-}` and cleans the temp dir even on the skip path,
  so no temp-dir leak. (`WORK` is created before the guard, but bats runs
  `teardown` after a skip — harmless.)

## Concern (2): Correctness of `_skip_live_artifact_in_ci` as it now appears in the merged branch

**Correct and complete.** (Note: this helper is a develop-side change from PR
#725, already merged to develop; it is not part of PR #743's net diff. Any issue
here would be a pre-existing develop condition, not a #743 blocker. It is sound
regardless.)

- Keys on `[ "${GITHUB_EVENT_NAME:-}" = "pull_request" ]` — `GITHUB_EVENT_NAME`
  is the correct GitHub Actions env var (`pull_request` on PR runs, `push` on
  push runs). Default-empty `:-` makes local runs fall through to full
  enforcement. Correct.
- Rationale is sound: on a `pull_request` run CI mounts the **base** repo's
  `factory-artifacts`, which a PR cannot modify, so base-side artifact drift
  would false-fail unrelated PRs (issue #724 — a real 10-PR mass failure).
  Skipping only `pull_request` runs preserves the **push**-run canary on
  develop, where the mounted artifact is exactly what develop ships.
- **Completeness verified:** all **5** live-artifact `@test`s call the helper
  (lines 275, 968, 1320, 1501, 1725 → `test_sprint_state_stories_list_present`,
  `test_real_production_file_round_trip`,
  `test_real_production_file_completeness_and_status_fidelity`,
  `test_supersession_edge_tolerated_partition_placement`,
  `test_partitions_sorted_by_full_graph_depth_def_b`). This matches the "five
  live-production-file tests" claim in the guard's header comment. The remaining
  9 `@test`s are fixture-based (CI-portable) and correctly do not need the guard.
- Each guarded test still retains its downstream `[ ! -f _PRODUCTION_* ]`
  absent-file skip, so the two guards are complementary (event-based + presence-
  based), not conflicting.

## 8-item checklist

| # | Item | Result |
|---|------|--------|
| 1 | Diff coherence | PASS — net diff is one focused test-infra guard; merge adds only develop's sprint-state guard |
| 2 | Description accuracy | PASS — body accurately describes the 8-line guard and the PR #725 design parallel |
| 3 | Test coverage | PASS (n/a) — test-infra change; validated by full bats suite (245 suites / 2058 ok / 0 failed per body; CI green) |
| 4 | Demo evidence | PASS (n/a) — transparent test-infra fix, no AC/demo required; correctly declared |
| 5 | Commit quality | PASS — conventional `fix(tests):`, clear body, sibling-sweep (TD-VSDD-060) documented; no AI attribution |
| 6 | Diff size | PASS — 8 net lines |
| 7 | Missing changes | PASS — sibling sweep confirms no other suite shares the unguarded live-copy pattern |
| 8 | Dependency status | PASS — same-class design as merged PR #725; develop merged in cleanly (MERGEABLE) |

## CI evidence (SHA d5f42338)

All 13 required checks green: `bats-full-suite (linux)`, `bats-darwin-leg`,
`bats-wave-handoff`, `cargo-host` (macos + ubuntu), all 5 `build-dispatcher`
targets, `SAST (Semgrep)`, `platforms-drift`, `validate`. Mergeable = MERGEABLE.

## Findings

None. No blocking, no warnings, no nits.

## What I verified (no rubber-stamp)

1. Confirmed the develop merge (`d5f42338`) touched only `sprint-state-format.bats`, leaving the subject guard file byte-identical to the previously-approved `9742c4d6`.
2. Confirmed the PR's net diff vs develop is still exactly the 8-line `pass-real-state-md-snapshot.bats` guard.
3. Read the full subject guard file at HEAD and verified placement, predicate precision, message quality, and teardown cleanup on the skip path.
4. Read the full `_skip_live_artifact_in_ci` helper and all 5 call sites; verified the env-var key, the base-mount rationale, and complete coverage of all live-artifact tests.
5. Confirmed all 13 CI checks pass on the reviewed SHA.

**Verdict: APPROVE (CLEAN).**
