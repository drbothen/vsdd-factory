# PR #716 — Fresh-Eyes Review

**Title:** fix(hooks): stop validate-count-propagation reading epic/ID digits as counts
**Branch:** `fix/count-propagation-epic-id` → `develop`
**Reviewer:** pr-reviewer (fresh-context, different-model cognitive diversity)
**Verdict:** APPROVE

---

## Summary

Fixes #690: `validate-count-propagation.sh` mis-parses the digits inside an
identifier token (e.g. the `11` in `E-11`) as a numeric count, firing a
spurious `count_propagation_drift` block. The remedy tokenizes and drops
identifier tokens (`<letters>-<digits/dots>`) from each line before the
count matchers run, via a pure-bash extglob substitution, and hardens the
"no counts found" guard against empty-associative-array expansion under
`set -u`. Adds a 4-case regression section to `hooks.bats`.

I verified the core logic by hand against the diff:
- `"5 E-11 stories"` → `E-11` dropped → `"5  stories"` → single-digit `5`
  fails Pattern A's two-digit requirement (`[0-9][0-9,]+`) → empty
  `SOURCE_COUNTS` → new `[[ -z "${SOURCE_COUNTS[*]:-}" ]]` guard exits 0
  (the #690 repro, matches GREEN evidence).
- `"E-11 delivered 13 stories"` → id dropped, genuine `13` survives and
  still drifts against `42` → exit 2 (matches T48).
- Multi-part ids (`BC-2.1.001`) consumed whole by `+([0-9.])`, closing the
  `001`-as-phantom subclass.
- Commas in real counts (`1,234`) are not in the drop set, so genuine
  comma-grouped counts survive.
- The flat extglob pattern is deliberately chosen over a nested one to
  avoid super-linear backtracking — correct security/perf call for
  per-line repo-controlled content.

Diff is coherent, single-purpose, well under the 500-line threshold.
Conventional commit subject with `Refs: #690`. Class 0 (develop-tracked
files only); no story AC, so demo-evidence is N/A.

## Findings

| # | Severity | Category | Finding | Suggestion |
|---|----------|----------|---------|------------|
| 1 | ADVISORY | description | Empty-array crash claim is overstated. PR body says `${#arr[@]}` on an empty associative array errors under `set -u` in "bash 4+" and that the hook "already crashed on any counts-free file." This was fixed in bash 4.4; on the ubuntu `validate` CI interpreter (bash 5.x) the old guard returns `0` and does not crash. Holds only for bash ≤4.3. | Correct the rationale to "bash ≤4.3" so future readers don't chase a non-reproducing crash. Fix itself is correct/safe on all versions. |
| 2 | ADVISORY | coherence | `shopt -s extglob` is enabled process-wide, not scoped to `_extract_counts`. It changes glob semantics for any later `[[ == pattern ]]`, `case`, or param-substitution glob in the file. Cannot confirm no downstream effect from the diff alone. | Confirm no other glob/case in the file relies on extglob being off; consider scoping via subshell or `shopt -u` after the loop. |
| 3 | MINOR | coherence | The id-drop pattern `+([A-Za-z])-+([0-9.])` also deletes legitimate `word-NN` prose tokens (e.g. `top-10 stories`), which could mask a genuine count on such a line (false negative). Documented, deliberate tradeoff; low risk on repo-controlled content. | Accept consciously; the ReDoS-safe flat pattern is the right call. |
| 4 | MINOR | coverage | Pre-existing (not introduced): Pattern A requires two digits, so single-digit counts are never detected. Out of scope; noted for completeness. | None required this PR. |

No blocking findings.

## Checklist

1. Diff coherence — PASS (single-purpose #690 fix).
2. Description accuracy — PASS with finding #1 (imprecise crash rationale).
3. Test coverage — PASS (4 regression cases; RED/GREEN evidence provided).
4. Demo evidence — N/A (Class 0 hook fix, no story AC).
5. Commit quality — PASS (conventional, `Refs: #690`).
6. Diff size — PASS (well under 500 lines).
7. Missing changes — PASS.
8. Dependency status — PASS (no upstream deps).

## Verdict

**APPROVE.** The fix is sound and matches the maintainer's stated remedy.
All findings are non-blocking; correcting the empty-array rationale (#1)
and confirming the global `extglob` has no downstream effect (#2) would
tighten it but are not merge blockers.
