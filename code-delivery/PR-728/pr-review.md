# PR Review: #728 — fix(hooks/validate-count-propagation): exclude historical sections from count-drift comparison

**Verdict:** APPROVE

**Reviewed:** 2026-07-22

## Findings

- **[ADVISORY — correctness/robustness]** Historical-heading matching is brittle to whitespace and naming variance. `_is_historical_heading` matches `case "${1,,}"` against literal patterns like `"## changelog"*` and `"## decisions log"*` (single ASCII space). A heading written with a tab or double space after `##`, or a singular variant (`## Decision Log`, `## Change-Log`), would NOT be recognized as historical, so its frozen counts would still be compared and re-introduce the false-positive drift this PR is meant to fix. The heading-detection regex `^##[[:space:]]` is broader (any whitespace) than the case patterns it feeds, leaving a real gap. Fails safe toward over-alerting rather than under-alerting, so not a blocker. Suggest collapsing runs of spaces before the case if heading style ever drifts.

- **[ADVISORY — maintainability]** The historical-section set is a hardcoded denylist inside the hook. Any new frozen/historical H2 section added in the future (e.g. `## Migration Log`, `## Superseded Decisions`) will silently re-introduce the exact alert-fatigue failure mode. Consider a one-line comment pointing maintainers at `_is_historical_heading` as the single place to extend, and keeping it in lockstep with `validate-changelog-monotonicity.sh`'s section convention it claims to mirror.

- **[ADVISORY — coordination]** Textual conflict with PR #716 at two shared lines (the `_extract_counts` loop body and the no-count guard). The PR description documents the resolution well (keep both edits; the guard fix is identical) and reports a verified scratch-branch merge. Flagging so the maintainer enforces the conflict resolution at merge time — whichever merges second must keep BOTH the ID-token-drop line (#716) and the historical-skip block (this PR).

## Basis for APPROVE

- Empty-array guard change (`${#SOURCE_COUNTS[@]} -eq 0` → `-z "${SOURCE_COUNTS[*]:-}"`) is correct and equivalent. `SOURCE_COUNTS` holds numeric count values so `[*]` join is non-empty whenever any entry exists, and `:-` makes the expansion safe under `set -u` for the now-reachable empty-source path.
- Frontmatter counts (`total_bcs: 41`) sit above the first H2 so `in_historical` initializes to 0 and live frontmatter is always scanned. Historical suppression only kicks in after an H2 historical heading.
- H3 subsections inside a historical H2 do not prematurely reset `in_historical` (`^##[[:space:]]` does not match `###`), so nested content stays correctly suppressed until the next H2.
- Live-drift detection is preserved: the GREEN control case (live `## Count Verification` says 39 vs frontmatter 41) still blocks with exit 2.
- `${1,,}` and case-conversion are bash-4 features consistent with the `require_bash4_count_prop` test guard; CI (Linux, bash 4+) exercises all four new cases.
- Diff is ~30 lines, single-file logic change plus tests — coherent, in-scope, well-sized, with RED/GREEN evidence.

No blocking or major findings.
