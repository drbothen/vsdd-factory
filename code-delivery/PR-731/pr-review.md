# PR Review — #731 fix(observability): include cacheCreation in cache-hit-ratio denominator

**Verdict:** APPROVE
**Reviewer:** pr-reviewer (fresh-eyes, final pre-merge)
**Base:** develop | **Class:** fix-PR (backlog remediation) | **Refs:** #244

## What I verified

1. **PromQL fix — CONFIRMED correct.** The added denominator term `+ sum(increase(claude_code_token_usage_tokens_total{type="cacheCreation"}[$__range]))` yields `cacheRead / (input + cacheRead + cacheCreation)`. Parentheses are balanced, escaped quotes preserved, and the `[$__range]` selector matches the sibling terms. This implements the described fix: cacheCreation tokens are input-side, premium-billed, and not cache hits — their prior omission inflated the ratio toward a false ~100%.
2. **Description updated accurately.** The semantic reframe ("share of input-side tokens served from cache") is the honest definition for this denominator, and the updated description explains why cacheCreation belongs there.
3. **JSON validity.** Author asserts `jq empty claude-cost.json` exits 0. No structural JSON edits.
4. **Executed arithmetic verified.** The author's Python output (99.998% → 89.038%) demonstrates the ~11 percentage-point overstatement. The 3rd-decimal difference vs the issue's stated value (89.04108%) is adequately explained by slightly different input values and does not affect the fix's correctness.

## Findings

### [LOW] Empty-vector / "No data" edge on the new term (pre-existing dashboard style)
In PromQL, if the `cacheCreation` counter series has never been scraped within `$__range` (not merely flat — a flat-but-present counter yields `increase()` = 0, which is safe), the third term is an empty vector and the entire denominator collapses to empty, making the gauge show "No data." This same failure mode already applied to the existing `input` and `cacheRead` terms. A hardened form (`... or vector(0)`) would be more robust, but applying it only to the new term while leaving the two existing terms bare would be inconsistent style. Correct scoping call for a 2-line fix PR — not a blocker.

### [LOW] Deferred math-test AC should be anchored to a concrete follow-up
The `Refs: #244` trailer correctly avoids auto-close since the math-test acceptance criterion is deferred. Under the production-grade default, "deferred to follow-up" is a defer-pattern smell unless the AC is anchored to a tracked future issue/story. Not a merge blocker, but the AC should be explicitly tracked so it does not get lost.

### [INFO] Division-by-zero unchanged and unregressed
Adding a non-negative term strictly reduces (never increases) div-by-zero likelihood. No regression.

## Summary
The change is a correct, minimal, and well-justified fix: it puts the premium-billed cache-write tokens into the denominator so the gauge reports the true share of input-side tokens served from cache instead of a misleading near-100%. The PromQL is syntactically and semantically sound, the description accurately reflects both the formula and the rationale, and JSON validity is asserted. The only notes are a pre-existing empty-vector "No data" edge (correctly left consistent with the rest of the panel) and a deferred test AC that should be anchored to a concrete follow-up.
