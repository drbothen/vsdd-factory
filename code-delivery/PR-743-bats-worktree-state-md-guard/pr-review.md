# PR Review — #743

**Title:** fix(tests): skip live STATE.md snapshot suite in worktrees without factory-artifacts mount
**Branch:** fix/bats-worktree-state-md-guard → develop
**Reviewer:** pr-reviewer (fresh-eyes, different-model cognitive diversity)
**Verdict:** APPROVE — no blocking findings

## Summary

Single-file test-infra fix. Adds a `setup()`-time skip guard to
`pass-real-state-md-snapshot.bats` so the suite skips cleanly (instead of
aborting `setup()`) when the `factory-artifacts` orphan-branch worktree is not
mounted at `REPO_ROOT/.factory`. Same local-only design class as PR #725.

## Checklist findings

### 1. Skip-guard correctness — PASS
The guard tests `[ ! -f "$REPO_ROOT/.factory/STATE.md" ]`, which is the exact
path the subsequent `cp` (line 40) consumes. Guarding on the actual required
file is more robust than a mount-directory heuristic. Placement is correct:
after the `mktemp`/`mkdir` scaffolding, before the `cp`. `REPO_ROOT` is already
a precondition of the pre-existing `cp`, so no new dependency is introduced; an
unset `REPO_ROOT` resolves to a safe skip. `skip` invoked in `setup()`
correctly skips every test in the file ("entire suite").

### 2. Skip-message clarity — PASS
Message names the missing file, states the root cause (factory-artifacts
worktree not mounted), and provides the exact remediation command
(`git worktree add .factory origin/factory-artifacts`). Informative for a
debugging reader.

### 3. Sibling-sweep completeness — PASS (independently verified)
`grep -rln 'cp "$REPO_ROOT'` across the entire `plugins/vsdd-factory/tests`
tree returns only `pass-real-state-md-snapshot.bats`. Every other suite that
copies a `STATE.md` sources it from a checked-in fixture
(`$FIXTURE_SRC` / `$FIXTURE_VALID` / `$FIXTURE_STATE_INVALID`), not the live
factory-artifacts mount, so none share the unguarded live-copy defect. The PR's
sweep claim holds.

### 4. Scope cleanliness — PASS
Diff touches exactly one file and adds only the 8-line guard block (comment +
conditional). No unrelated changes, no formatting churn, no stray edits.

## Notes
- Untracked `plugins/vsdd-factory/tests/report.tap` in the local working tree is
  not part of this PR diff and does not affect the review.

## Verdict
APPROVE. No BLOCKING / MAJOR / MINOR findings. Test-infra correctness fix,
scoped and clean, sibling sweep independently confirmed.
