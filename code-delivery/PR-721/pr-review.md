# PR #721 — Fresh-eyes PR review

**PR:** fix(hooks): scope validate-bc-title index lookup to the Title-header table
**Branch:** fix/bc-title-index-scoping → develop
**Issue:** #566
**CI:** GREEN (bats-full-suite linux, bats-darwin-leg, bats-wave-handoff, cargo-host ubuntu+macos, all build-dispatcher targets)

## Verdict: APPROVE

The core fix is correct for the realistic BC-INDEX layout and is backed by a
genuinely discriminating test: the pass/block pair share the same misleading
`CAP-001` first-occurrence cell, so they prove the scoping works rather than
just the happy path. No regression risk versus current behavior. All findings
below are non-blocking.

## What I verified (hand-trace of the awk state machine)

For `-F'|'`, a row `| BC | Title | ... |` yields `$2=" BC "`, `$3=" Title "`,
so `c3` is the 2nd *data* column — the header check `tolower(c3)=="title"` is
the right column. `scoped = c3` stores the trimmed value since `c3 = trim($3)`
runs first. Separator rows `|----|----|` start with `|`, so they do not reset
scope, and neither `c2==bc` nor `tolower(c3)=="title"` matches on the dashes —
skipped cleanly. Section headings / blank lines do not start with `|`, so
`in_title_table` closes correctly. The `scoped==""` guard means the first
title-table match wins and `END` prefers `scoped` over `fallback`, so the
preceding capability-satisfaction cell (`CAP-001`) is correctly ignored when a
nav title exists.

All three added tests trace to their asserted exit codes. No `bash -n` issue
visible; the awk is single-quoted inside `$()` so bash does not parse it, and
the awk constructs (`gsub` alternation in a function, `tolower`, `-F'|'`) are
portable across gawk and the macOS one-true-awk — consistent with CI green on
both platforms.

## Findings

### MINOR / SUGGESTION — coupling to the exact header string "Title" in a hardcoded column
Both header detection and value extraction hardcode column 3 (`c3`). The scope
only opens if the §2 nav header's 2nd data column trims to exactly `title`
(case-insensitive). If the real `BC-INDEX.md` ever uses a different header
label (e.g. "Contract Title", "Name") or places the title in a different
column, `in_title_table` never opens, `scoped` stays empty, and it silently
falls back to first-occurrence — precisely the `CAP-001` cell #566 is about.
Not a regression (degrades to today's behavior), but the fix's correctness
rests on an index-header string not visible in the diff. More robust: on the
header row, record the index of whichever cell equals "title" and read that
same column on data rows, rather than assuming column 3. At minimum, document
the header contract the hook depends on.

### MINOR — test gap
Missing: (1) a headerless-fallback *pass* case (title agrees → exit 0) to lock
in that the fallback path does not over-block; (2) a case where a Title-headed
table exists but the BC appears only in the capability table and is absent from
the nav table — `scoped` stays empty and the hook falls back to the
satisfaction cell, firing a false `bc_h1_index_drift`. Multiple-Title-table
layout is also untested (first match wins) but low value for BC-INDEX.

### MINOR — silent fallback masks a malformed-index signal
When a Title-headed table is present but the BC is not found inside it,
reporting the unrelated fallback cell as "the indexed title" produces a
confusing drift message rather than a clear "BC not present in the Title table"
signal. Consider preferring a not-indexed result over the wrong cell once a
title table has been seen. Low priority.

## Checklist
- Diff coherence, description accuracy, commit scope: consistent; description
  matches the code precisely.
- Diff size: ~90 lines, well under threshold.
- canonical-format-invariant.bats fallback exercise: outside the diff, but the
  full bats suite is green in CI, so the pre-existing test passes under the new
  scoped/fallback logic — empirically satisfied.
