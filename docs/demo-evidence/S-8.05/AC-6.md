# AC-006: Check 3a/3b; advisory block-mode; hook.block emit; remediation block

**BC trace:** BC-7.04.044 postcondition 1
**Status:** PASS

## What was verified

### Check 3a — formal review posted

Regex pattern: `r"gh pr review|pr review.*posted|review.*posted.*GitHub|APPROVE|REQUEST_CHANGES"`

When the result does NOT match this pattern, error accumulated:
```
No evidence that a formal GitHub review was posted via 'gh pr review'
```

### Check 3b — verdict detected

Gate: result contains literal `gh pr review` AND does NOT match `r"approve|request-changes|APPROVE|REQUEST_CHANGES"`.
This is NOT a re-test of Check 3a's full disjunction — it only fires on the `gh pr review` literal token.

Error accumulated when Check 3b fires:
```
Review posted but no verdict (--approve or --request-changes) detected
```

Case (e) from spec: `"ran gh pr review --no-body"` — Check 3a passes (matches `gh pr review`), Check 3b fires (no approve/request-changes). Single error in stderr.

### Advisory block-mode (on_error=continue semantics preserved)

The hook always returns `HookResult::Continue` — it communicates the block via:
1. `host::emit_event("hook.block", ...)` — dispatched to the event log
2. stderr message with formatted error list
3. `HookResult::Continue` return (exit 0 to Claude Code)

`on_error = "continue"` in the registry governs what the dispatcher does if the WASM plugin CRASHES — completely different from the hook's own block signal. Both semantics preserved.

### Remediation block (verbatim from bash lines 65-67)

```
  pr-reviewer MUST: (1) write pr-review.md, (2) spawn github-ops with
  'gh pr review --approve' or 'gh pr review --request-changes --body-file'.
  NEVER use 'gh pr comment' for review verdicts.
```

### host::emit_event call

```rust
vsdd_hook_sdk::host::emit_event(
    "hook.block",
    &[
        ("hook", "validate-pr-review-posted"),
        ("matcher", "SubagentStop"),
        ("reason", "pr_review_not_posted"),
        ("subagent", agent),
    ],
);
```

Bare statement form (AC-008 requirement). `event_type = "hook.block"` — `"type"` does NOT appear as a field key.

## Recording

[AC-006-check3-advisory-block.gif](AC-006-check3-advisory-block.gif)
