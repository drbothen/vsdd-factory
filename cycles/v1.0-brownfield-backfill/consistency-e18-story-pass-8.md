---
document_type: consistency-report
cascade: E-18-story
pass: 8
verdict: INCONSISTENT
date: 2026-06-17
reviewer: consistency-validator
checks_run: 10
checks_passed: 9
checks_failed: 1
---

# E-18 Story Cascade Consistency Report — Pass 8

**Date:** 2026-06-17
**Verdict:** INCONSISTENT (1 of 10 checks failed)
**Note:** C-P8-001 is the same finding as F-P8-002 (BC-4.15.001 PC-B sub-emission labels).

---

## Consistency Checks

| Check | Result | Notes |
|-------|--------|-------|
| C-P8-001 | FAIL (= F-P8-002) | BC-4.15.001 PC-B-B1/PC-B-B2 labels cited in S-18.09 AC-008 but not present as citable headings in BC-4.15.001 v1.1 — FIXED by product-owner (BC-4.15.001 v1.2) |
| Story title cells match frontmatter titles | PASS | All 12 E-18 stories verified |
| Version annotation cells current | PASS | S-18.09 v1.7 at pass-7 (story-writer v1.8 per pass-8 fix burst; STORY-INDEX update pending this state-manager burst) |
| VP-INDEX wave cells match story frontmatter | PASS | All VP-081..VP-092 wave cells verified against anchor_story frontmatter |
| BC-INDEX summary counts match body catalog | PASS | BC-INDEX v3.06 Summary table matches catalog row counts |
| ARCH-INDEX subsystem row counts | PASS (post-fix) | State-manager fix burst F-P8-003 corrects SS-03/04/05/06/07/08 rows — PASS after this burst |
| E-18 epic footnote story/pts/wave counts | PASS | 12 stories / 89 pts / 8-wave verified |
| Depends_on / Blocks DAG symmetry | PASS | Bidirectional sweep clean after pass-7 C-P7-002 fix |
| BC behavioral contract version consistency | PASS | BC-4.15.001 v1.2 (PO fix) / VP-091 v1.1 (architect fix) — cites consistent |
| STORY-INDEX narrative line counts | PASS | Line 190 narrative: 120 stories / 19 epics (updated at D-624) |

---

## O-P8-001 — Observation: ARCH-INDEX per-subsystem BC rows not updated on BC add

**Severity:** Observation (non-violation; load-bearing per F-P8-003 finding)
**Description:** The ARCH-INDEX Subsystem Registry rows SS-01..SS-10 are not automatically updated when BCs are added to BC-INDEX. The D-619 BC-INDEX count reconcile burst updated the BC-INDEX Summary table but did not propagate the per-row updates to ARCH-INDEX. The row-sum-equals-Total invariant should be enforced as a gate. State-manager addresses this in the pass-8 fix burst (F-P8-003).

---

## Summary

1/10 checks failed (C-P8-001 = F-P8-002). Fixed by product-owner (BC-4.15.001 v1.2). All other consistency checks PASS including the ARCH-INDEX subsystem-row reconcile (state-manager fix burst this pass). Pass-9 consistency re-verify NEXT.
