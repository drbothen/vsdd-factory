# E-18 Story Cascade — Consistency-Validator Pass-10 Report

**Date:** 2026-06-17
**Cascade:** E-18 Story (F3 story adversarial cascade)
**Pass:** 10 (CYCLE-BREAKING — exhaustive 4-index changelog-array backfill)
**Validator:** consistency-validator (fresh-context)

---

## Verdict: INCONSISTENT

**Findings: C-P10-001 (MAJOR)**

### C-P10-001 (MAJOR) — VP-INDEX changelog-array gap

VP-INDEX frontmatter declared `version: "2.37"` but the `changelog:` array top entry referenced `v2.34`. Three version bumps (v2.35 at D-616, v2.36 at D-620, v2.37 at D-625) each advanced `version:` + `last_amended:` in VP-INDEX.md without appending the corresponding `changelog:` array row. This is the same index-sync-leg partial-fix regression class as F-P9-001 (BC-INDEX at pass-9), now manifesting in VP-INDEX with an accumulated three-row gap.

**Consistency checks that PASSED:**

1. **BC-INDEX changelog array top row == frontmatter version:** v3.07 == v3.07 PASS (BC-INDEX was repaired at D-626 for the same class)
2. **ARCH-INDEX changelog array top row == frontmatter version:** v2.54 == v2.54 PASS (partial — only checked top row; F-P10-004 identified v2.51/v2.52 gaps below, not visible from top-row check alone)
3. **STORY-INDEX version:** v4.11 (exempt from structured array per D-448(b)/S-15.03) PASS
4. **S-18.09 STORY-INDEX cell version:** v1.9 matches story frontmatter v1.9 PASS

---

## Part B — Post-D-627 Closure Note

D-627 CYCLE-BREAKING FIX BURST (2026-06-17) addressed C-P10-001 and the full class:

1. **VP-INDEX changelog array top row == frontmatter version:** v2.37 == v2.37 PASS (v2.35/v2.36/v2.37 rows backfilled)
2. **BC-INDEX changelog array top row == frontmatter version:** v3.07 == v3.07 PASS
3. **ARCH-INDEX changelog array top row == frontmatter version:** v2.54 == v2.54 PASS (v2.51/v2.52 rows also backfilled)
4. **STORY-INDEX version:** v4.12 (exempt from structured array per D-448(b)/S-15.03) PASS
5. **S-18.09 STORY-INDEX cell version:** v1.10 matches story frontmatter v1.10 PASS
6. **VP-INDEX v2.35/v2.36/v2.37 all present in array:** confirmed PASS
7. **BC-INDEX v3.05/v3.06/v3.07 all present in array:** confirmed PASS
8. **ARCH-INDEX v2.51/v2.52/v2.53/v2.54 all present in array:** confirmed PASS
9. **VP-091 changelog descending order (v1.1 above v1.0):** PASS (architect fix F-P10-002)
10. **S-18.09 v1.10 fence-strip self-scan carve-out:** PASS (story-writer fix F-P10-003)

**Pass-10 INCONSISTENT (C-P10-001 MAJOR) → streak RESET 0/3; D-627 fix burst applied; post-fix all 10 checks PASS; pass-11 re-verify NEXT.**
