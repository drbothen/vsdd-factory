# PR #782 — Review Cycle 2 (fix commit `fbf3e640`)

> Posted as a review COMMENT rather than a formal APPROVE: GitHub rejects `addPullRequestReview` with `--approve` on one's own PR (`Can not approve your own pull request`). The verdict below is the reviewer's formal disposition and supersedes the cycle-1 REQUEST_CHANGES.

**Verdict: APPROVE** — both blocking/suggestion findings independently verified as genuinely fixed. No paper-fix, no new bug, no scope creep.

## F-782-001 [BLOCKING] — RESOLVED (verified empirically, not by self-report)

`s21-12-version-and-deny-gate.bats` AC-003 now wraps the capture in `set +e` / `set -e`, with `local metadata_json metadata_exit` declared on a *separate* line (so `$?` is the subshell's status, not `local`'s). I reproduced both variants under real `bats`:

- Old pattern → `not ok ... 'out=$(bash -c "exit 7"); ex=$?' failed with status 7` (errexit trips at the assignment; the `-ne 0` arm was dead code).
- New pattern → `ok`, skip arm reached with `ex=7`.

The fix is structural and matches the pre-existing AC-004 pattern in the same file. Confirmed reachable, not merely renamed or commented.

## F-782-003 [SUGGESTION] — RESOLVED, and the claimed regression-avoidance is real

The exclusion is now anchored to the dynamically-resolved Status column (same header-detection logic as ASSERT 2) and lowercased via `tolower()` before matching. I verified three things independently:

1. **No behavioral regression on the live file.** Ran old and new awk side by side against the real `STORY-INDEX.md`: both emit **188** identical IDs (`diff` → IDENTICAL). Row ordering is safe — the first `| Story ID |` header is at line 217, the first `| S-` row at 219, so no story row precedes header resolution.
2. **The `**bold**` requirement is load-bearing, not decorative.** Real Status cells such as `merged [deprecated by ADR-015 — per-sink resilience retired with sink ecosystem]` (S-4.04) and `merged [superseded by ADR-015]` (S-1.04) *do* contain the words `retired`/`superseded` in the Status column itself. A bare case-insensitive substring match would have silently dropped these MERGED stories from PC4 completeness. Preserving `\*\*retired\*\*` / `\*\*superseded` prevents that. The implementer's stated regression risk is genuine and correctly avoided.
3. **ASSERT 2's `exit` is behaviorally equivalent to the old row-filter.** Previously a retired/superseded row simply failed the row pattern and awk printed nothing; now it `exit`s with no output. Both yield empty `idx_status` → the same `PC2: not found in STORY-INDEX (non-retired)` path. There is no `END` block, so `exit` emits nothing extra. Story IDs are unique, so early termination loses no later match.

Case-normalization asymmetry (`**retired**` requires the closing `**`, `**superseded` does not) is intentional and preserved from the original — it accommodates `**SUPERSEDED (D-1057)**`.

## Test execution (independently run, not taken on trust)

Checked out `fbf3e640` in a throwaway worktree with `.factory/` mounted:

- `sprint-state-format.bats` → 14/14 `ok`
- `s21-12-version-and-deny-gate.bats` → 6/6 `ok`, including `ok 3 AC-003` executing the *real* assertion (no `# skip`), which is the correct outcome locally — the arm is reachable but not taken when `cargo metadata` succeeds.

Total 20/20, matching the self-report.

## Scope

`fbf3e640` touches exactly the two test files under review; nothing else. Full PR file set (`CHANGELOG.md`, `Cargo.lock`, the two `.bats` files) is coherent with the stated purpose. No out-of-scope changes, no production-code drift, no `--no-verify`, no AI attribution in the commit message. Diff is small and well-commented; the added block comments accurately describe the new logic (verified against the code, not just read).

## CI status at time of review

`bats-darwin-leg`, `bats-wave-handoff`, `deny-advisories`, `policy-15-attestation-location`, `attestation-gate-non-vacuity-controls`, `platforms-drift`, `SAST (Semgrep)`, `validate` — **pass**. `bats-full-suite (linux)`, `cargo-host (ubuntu/macos)`, and the five `build-dispatcher` matrix legs were **pending** when this review was written. Approval is contingent on those completing green; merge should not proceed until `gh pr checks 782` is fully green.

No remaining findings. F-782-004 stands as previously assessed (harmless no-op, fine to keep).
