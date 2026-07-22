# PR #722 Review — fix(hooks): normalize leading v prefix in changelog frontmatter cross-check

**Verdict: APPROVE**

Branch: `fix/changelog-v-prefix` → `develop` · Closes #660 · CI: GREEN (all matrices)

## Summary

The fix is correct, minimal, and correctly scoped. `${FM_VERSION#[vV]}` strips at
most one leading `v`/`V` (shortest-match prefix removal), applied symmetrically to
both operands, so both migration directions are handled by the same two lines. I
reviewed every changed line in both files and verified all seven review dimensions
against the actual hook source. No blocking or major issues.

## What I verified (no rubber-stamp)

- **Both-directions symmetry.** The identical `#[vV]` strip is applied to both
  `FM_VERSION` and `FIRST_VERSION`, so all three semantic pairs resolve correctly:
  `("1.5","v1.5")` equal, `("v1.1","1.1")` equal, `("v1.1","v1.3")` unequal. The
  "un-fixable tracked file" scenario (normalize frontmatter to bare while the row
  stays `v`-prefixed) is genuinely unblocked.
- **Error-message fidelity.** Keeping raw `$FM_VERSION`/`$FIRST_VERSION` in the
  message (not the normalized forms) is the correct choice. When the normalized
  comparison fires it is a genuine numeric mismatch, and echoing the user's actual
  entered strings is more actionable than echoing stripped forms.
- **FIRST_CODE.** `changelog_frontmatter_mismatch` is right — post-normalization the
  check can only fire on a real version mismatch, and the mapped recommendation
  ("Set frontmatter 'version:' to match the top changelog row") is accurate.
- **Normalization scope.** `vv1.5 → v1.5` is a non-issue; real versions never carry a
  double `v`. Single-char strip is the right granularity.
- **Minimal change.** Only the cross-check block (lines 146–152) changed;
  ordering/date logic untouched.

## Findings

### [MINOR] test-coverage — uppercase-`V` path untested (corpus-lint.bats)
Both new equality tests use lowercase `v` only. The code handles `[vV]`, but no test
exercises the uppercase-`V` path (e.g. frontmatter `V1.5` vs row `1.5`). Logic is
trivially symmetric so risk is low, but a fourth test with `version: "V1.5"` against a
bare `1.5` row asserting `status -eq 0` would lock in the `V` half of the class.

### [MINOR] scope-asymmetry (observation) — validate-changelog-monotonicity.sh:126
The frontmatter-vs-top-row check now treats `v1.5` and `1.5` as equal, but the
adjacent-row duplicate detector (`[[ "$PREV_VERSION" == "$VERSION" ]]`) still compares
raw strings. During a mixed-prefix migration, two rows that are the same semantic
version (`v1.5` then `1.5`) would slip past duplicate detection. This is a pre-existing
weakness (that check already only catches exact string equality, not true monotonic
decrease), so the PR is correctly scoped to the frontmatter cross-check and this should
NOT block. Flagged only so it's a conscious deferral rather than an oversight; worth a
follow-up if convention-migration files pass through this hook mid-migration.

## Scoping note (monotonicity ordering)
The row-ordering checks do not need v-normalization for this PR's stated goal, so
limiting the change to the frontmatter cross-check is the right call.

No BLOCKER or MAJOR findings. CI green across all matrices. Recommend merge; the two
MINOR items can ship as-is or fold into a follow-up.
