# E-18 Story Cascade — Consistency-Validator Pass-13

**Validator:** fresh-context (orchestrator-dispatched; no prior-burst artifacts in context)
**Cascade:** E-18 STORY (F3 story adversarial cascade + consistency-validator)
**Date:** 2026-06-17
**D-ref:** D-632
**Parent-commit:** e89727ef (D-631 SHA-patch HEAD)

---

## Verdict: INCONSISTENT

**Finding count:** 1 (C-P13-001 MEDIUM)

---

## Findings

### C-P13-001 [MEDIUM] — VP-INDEX VP-091 description stale labels + false changelog claim

**File:** `.factory/specs/verification-properties/VP-INDEX.md` (v2.37), §Full Index, VP-091 row.

**Issue:** The VP-091 row description column reads:
> `...emits dual-channel DelegationRecommended advisory to stderr (B-1) + plugin.log (B-2) on first pattern match (BC-4.15.001 PC-B)...`

The tokens `(B-1)` and `(B-2)` are the PRE-v1.2 labels from BC-4.15.001. BC-4.15.001 v1.2 (D-625) promoted them to canonical subsection headings `PC-B-B1` (stderr) and `PC-B-B2` (plugin.log). VP-091.md v1.1 (D-625) synchronized to these canonical labels.

**False changelog claim:** The VP-INDEX v2.37 changelog entry for D-625 states: "VP-091 Full Index description note updated to record v1.1 label-sync". The description column body was NEVER actually updated — it still contains `(B-1)` and `(B-2)`. The changelog attestation claimed a change that was not performed.

**Canonical source:** BC-4.15.001 v1.2 uses `PC-B-B1` and `PC-B-B2` as section headings. VP-091.md v1.1 uses the same canonical labels in its property body. VP-INDEX Full Index description cell for VP-091 is the only remaining site with stale `(B-1)`/`(B-2)` notation.

**Required fix:** Replace `(B-1)` → `PC-B-B1` and `(B-2)` → `PC-B-B2` in the VP-091 §Full Index description column. Bump VP-INDEX version v2.37 → v2.38 with a correct changelog-array top row.

**Severity:** MEDIUM — description is a normative reference to BC-4.15.001 clause labels; stale labels are unresolvable against current BC-4.15.001 v1.2.

---

## Checks PASS (10/10)

| # | Check | Result |
|---|-------|--------|
| 1 | VP-INDEX frontmatter version matches changelog-array top row | PASS (v2.37 = v2.37; after fix will be v2.38 = v2.38) |
| 2 | VP-091.md v1.1 body labels (PC-B-B1/PC-B-B2) consistent with BC-4.15.001 v1.2 | PASS |
| 3 | STORY-INDEX v4.13 all 12 story version cells match story frontmatter versions | PASS |
| 4 | BC-INDEX v3.07 BC-4.15.001 row version cell v1.2 matches BC file frontmatter | PASS |
| 5 | ARCH-INDEX v2.54 §Subsystem Registry row-sum == Total (1,972) per literal-shell | PASS |
| 6 | S-18.06 body cites BC-4.15.001 v1.2 (propagated D-626) | PASS |
| 7 | S-18.09 v1.11 RAW_LABEL regex `[^ )]+` (F-P11-001 fix) hand-trace PASS | PASS |
| 8 | VP-INDEX §Story Anchors VP-091 wave 5 anchor S-18.06 correct | PASS |
| 9 | BC-INDEX v3.07 changelog-array top row v3.07 matches frontmatter v3.07 | PASS |
| 10 | ARCH-INDEX v2.54 changelog-array top row v2.54 matches frontmatter v2.54 | PASS |

**Deferred (carried from pass-12, non-blocking):**
- C-P12-001 ARCH-INDEX stale "per BC-INDEX v3.06" cite: DEFERRED → next ARCH-INDEX version bump. No regression.
- C-P12-002 disk-count 123 vs 117-file-resident: DEFERRED per D-619 precedent. No regression.

---

## Combined Pass-13 Verdict (per BC-5.39.001)

Adversary: CLEAN (0 BLOCKER/MAJOR/load-bearing MEDIUM/mis-anchor/LOW)
Consistency-validator: INCONSISTENT (C-P13-001 MEDIUM)
**Combined: NOT-CLEAN. Streak RESET 1/3 → 0/3. C-P13-001 fixed in this burst (D-632). Pass-14 NEXT.**
