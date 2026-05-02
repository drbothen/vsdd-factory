# AC-005: Check 2 — gh pr comment fallback detected

**BC trace:** BC-7.04.043 postcondition 1
**Status:** PASS

## What was verified

Check 2 tests whether the result text contains the literal substring `gh pr comment`.
Using `gh pr comment` instead of `gh pr review` is a known failure mode — it posts
a comment rather than a formal review verdict.

When the result contains `gh pr comment`, error accumulated:
```
Used 'gh pr comment' instead of 'gh pr review' — findings won't show as a formal review verdict
```

This is a simple `.contains("gh pr comment")` check — no regex needed.

Unit test `test_BC_7_04_043_check2_gh_pr_comment_detected` confirms error accumulation.
Bats test AC-007(c) confirms dispatcher-level behavior: exit 0, `hook.block` event emitted, error in stderr.

Note: Check 2 fires independently of Check 1. A result could have `wrote pr-review.md`
(Check 1 passes) but then use `gh pr comment` (Check 2 fires), resulting in a block.

## Recording

[AC-005-check2-gh-pr-comment.gif](AC-005-check2-gh-pr-comment.gif)
