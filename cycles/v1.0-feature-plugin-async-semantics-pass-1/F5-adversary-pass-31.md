---
document_type: adversary-review
cycle: v1.0-feature-plugin-async-semantics-pass-1
pass: 31
verdict: HIGH
adr_013_clock_after_pass: 0_of_3
producer: adversary
timestamp: 2026-05-09T18:00:00Z
strategic_recommendation: continue-protocol-per-user-directive
---

# F5 Pass-31 Adversary Review

## Verdict

**HIGH** (2H + 2M). 14th consecutive non-NIT pass. ADR-013 RESETS.

## Findings

### F-P31-001 [HIGH] VP-074 source-vs-VP-INDEX Breakdown drift — 3rd L-P28-001 META-recurrence at NEW layer
- VP-074.md:19 says `proof_method: kani-proof` (closed in fix-burst-28).
- VP-INDEX:116 Breakdown lists VP-074 in `integration` row (count 22).
- VP-INDEX:119 kani-proof row count = 3 (VP-070, VP-071, VP-077) — VP-074 missing.
- VP-INDEX:199 Full Index VP-074 row says Proof Method = integration.
- L-P28-001 grep was scoped to source frontmatter; VP-INDEX Breakdown table is different artifact class not covered.
- **Fix:** Decide canonical method (architect adjudicates VP-074's primary method); update either source or VP-INDEX Breakdown + Full Index to match.

### F-P31-002 [HIGH] STORY-INDEX Status Summary stale by 5 stories (30-pass survivor)
- STORY-INDEX:195 claims merged | 57. Actual = 62.
- STORY-INDEX:197 claims draft | 28. Actual = 23.
- Net +5/-5 from post-W-15 merges (S-9.00, S-13.01, S-12.01, S-12.02, S-12.06, S-15.01) plus S-3.04 reclassification.
- Total row sum still matches (88), masking per-row drift.
- STORY-INDEX:185 prose says "15 epics (E-0 through E-15)" but enumerates 16 IDs (E-0..E-15).
- **Fix:** STORY-INDEX:195 57→62; :197 28→23; :185 prose corrected; bump v2.56→v2.57.

### F-P31-003 [MEDIUM] STATE.md Identifier Conventions Epic count says 15; actual 16
- STATE.md:115 Epic row: `| 15 |`
- Glob `epics/E-*.md` returns 16 (E-0..E-15)
- Sibling to F-P30-001 (same table; ADR row was 19→20 last burst). L-P21-002 sibling-fix discipline violated — fix-burst-29 only patched ADR row.
- **Fix:** STATE.md:115 → `| 16 |`. Audit ALL identifier-table rows.

### F-P31-004 [MEDIUM] STATE.md:186 Session Resume Checkpoint says "pass-29 HIGH" but most-recent reset trigger is pass-30
- STATE.md:186 still references pass-29; should be pass-30 per fix-burst-29's update of other STATE.md lines.
- Partial-fix regression of fix-burst-29 STATE.md sweep.
- **Fix:** STATE.md:186 → "pass-30 HIGH resets".

## Notable observations

- Fix-burst-29 closures verified clean (F-P30-001..003).
- 14-pass non-NIT streak. Body-table layer (status summaries, breakdown tables, identifier-table counts) is still drifting.
- [process-gap] L-P28-001 corpus-sweep clause needs Breakdown-table audit step.
- [process-gap] STORY-INDEX self-validation discipline missing — Total-row sum hides per-row drift.

## Convergence assessment

14th non-NIT pass. Per user directive: continue protocol. ADR-013 RESETS.
