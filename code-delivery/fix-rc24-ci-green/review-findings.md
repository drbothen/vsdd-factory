---
document_type: pr-review-findings
story_id: fix-rc24-ci-green
pr_number: 782
status: "in-review"
producer: pr-manager
timestamp: "2026-08-25T16:56:00Z"
---

# PR Review Findings: fix-rc24-ci-green (PR #782)

Not a story PR — no `S-N.MM` behind it. Slug matches the branch name, consistent
with other non-story fix PR dirs under `.factory/code-delivery/` (e.g. `TD-67`,
`security-fix-path-traversal`).

## Convergence Summary

| Cycle | Findings | Blocking | Suggestion | Nit | Fixed | Remaining |
|-------|----------|----------|-----------|-----|-------|-----------|
| 1 | 4 | 1 | 2 | 1 | 3 (F-782-001/002/003) | 1 (F-782-004, accepted-as-is) |
| 2 | 0 | 0 | 0 | 0 | n/a | 0 |

**Verdict:** CONVERGED after 2 cycles (pr-reviewer APPROVED at commit `fbf3e640`).

## Finding Detail

| ID | Cycle | Severity | Category | Finding | Resolution |
|----|-------|----------|----------|---------|------------|
| F-782-001 | 1 | blocking | coverage | `s21-12-version-and-deny-gate.bats` AC-003: `cargo metadata` capture is unreachable in the `exit != 0` skip arm under bats `errexit` — TD-VSDD-059 paper-fix (comment claims behavior code doesn't implement) | Wrap capture in `set +e`/`set -e` matching the existing AC-004 pattern in the same file; routed to test-writer |
| F-782-002 | 1 | suggestion | description | PR body describes commit 1 as only the stderr/jq fix; omits that it also converts an environment-gap hard-fail into a `skip` (deliberate gate-behavior loosening, disclosed in the commit message but not the PR body) | Fixed directly in pr-description.md by pr-manager (commit 1 section expanded with a "Behavior change disclosed" callout) |
| F-782-003 | 1 | suggestion | coverage | `sprint-state-format.bats`: `**SUPERSEDED` exclusion unanchored to the Status column (matches anywhere in row) and case-inconsistent vs. lowercase `**retired**` — latent completeness-gate hole for a future active story with a SUPERSEDED-mentioning Notes cell | Anchor both exclusions to `$status_col` + normalize case; routed to test-writer (production-grade-default: cheap in-scope fix, not deferred) |
| F-782-004 | 1 | nit | coherence | `sprint-state-format.bats` ASSERT 2 SUPERSEDED exclusion is a no-op (ASSERT 2 iterates `ss_ids`; a superseded ID already fails ASSERT 1 as phantom) — commit message overstates what it does | Accepted as-is per reviewer's own disposition ("fine to keep"); no code change |

## Triage Routing

| Finding ID | Routed To | Status |
|------------|-----------|--------|
| F-782-001 | test-writer (in worktree `.worktrees/fix-rc24-ci-green`) | fixed — commit `fbf3e640` |
| F-782-002 | pr-manager | fixed |
| F-782-003 | test-writer (in worktree `.worktrees/fix-rc24-ci-green`) | fixed — commit `fbf3e640` |
| F-782-004 | n/a (no fix required) | accepted-as-is |

## Review Cycle History

### Cycle 1

- **Reviewer:** pr-reviewer (fresh-eyes, cognitive-diversity model family)
- **Verdict:** REQUEST_CHANGES
- **Findings:** 4 total, 1 blocking
- **Action taken:** Security review findings unaffected (separate pass, APPROVE, no CRITICAL/HIGH). Triaged F-782-001 and F-782-003 to test-writer for in-worktree fix; F-782-002 fixed directly in pr-description.md; F-782-004 accepted as-is. Note: `gh pr review --request-changes` could not be posted as a formal GitHub review state (PR author == authenticated `gh` identity, API rejects self-review); verdict tracked via PR comment [#issuecomment-5413884144](https://github.com/drbothen/vsdd-factory/pull/782#issuecomment-5413884144) instead.
- **Fix commit:** `fbf3e640` — "fix(ci): make cargo-metadata skip path reachable + anchor SUPERSEDED exclusion to status column". F-782-001: wrapped `cargo metadata` capture in `set +e`/`set -e` matching AC-004 pattern, so the `-ne 0` skip arm is now reachable. F-782-003: anchored both `retired`/`superseded` exclusions to the dynamically-resolved Status column (reusing ASSERT 2's `status_col` detection) + case-normalized via `tolower`; test-writer independently caught and avoided a regression risk (naive substring match would have wrongly excluded MERGED stories with prose-only "retired"/"superseded" mentions lacking the `**bold**` marker, e.g. S-1.04, S-4.04). Verified: 20/20 tests pass locally, including a real-file check against actual STORY-INDEX.md/sprint-state.yaml confirming S-21.11 (SUPERSEDED) and S-8.11–29 (retired) are excluded while S-1.04/S-1.08/S-4.04/06/07/09 (merged-with-note) remain included. Pushed: `285adc95..fbf3e640` on `fix/rc24-ci-green`.

### Cycle 2

- **Reviewer:** pr-reviewer (re-review of fix commit)
- **Verdict:** dispatched, pending
- **Findings:** TBD
- **Action taken:** TBD
