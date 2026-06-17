---
document_type: adversarial-review
cascade: E-18-story
pass: 8
verdict: NOT-CLEAN
date: 2026-06-17
reviewer: adversary
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/adv-e18-story-pass-7.md
  - .factory/stories/S-18.09-f2-process-gap-lesson-gate-checks.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.15.001.md
  - .factory/specs/verification-properties/VP-091.md
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
streak: 0/3
findings_count: 5
---

# E-18 Story Cascade Adversarial Review — Pass 8

**Date:** 2026-06-17
**Verdict:** NOT-CLEAN
**Streak:** 0/3 (pass-8 NOT-CLEAN)
**Pass-7 closures verified:** F-P7-001 MAJOR (S-18.09 AC-008 silent-inert) CLOSED; F-P7-002 LOW (S-18.08 WARN vs FAIL) CLOSED; C-P7-001..C-P7-005 all CLOSED.

---

## Part A — Findings

### F-P8-001 — BLOCKER: S-18.09 AC-008 awk gate collapses to one-line range

**Severity:** BLOCKER
**File:** `.factory/stories/S-18.09-f2-process-gap-lesson-gate-checks.md`
**Status:** FIXED by story-writer (pass-8 fix burst)

**Description:** The AC-008 gate rewrite in pass-7 used the awk inclusive-range pattern `/^## Postconditions/,/^## /` to extract the Postconditions section. This range collapses when the start pattern also matches the end pattern: `## Postconditions` matches `/^## Postconditions/` (start) AND `/^## /` (end) simultaneously, producing a one-line match that immediately closes. The extracted section is therefore always a single heading line, making the gate permanently FALSE-RED regardless of postcondition content. Fixed by story-writer using flag-form awk (set flag on start, unset on next `^## ` that is NOT the start heading).

**Root cause:** awk inclusive-range syntax `/start/,/end/` closes on the SAME line if start also matches end. The gate was non-vacuous in intention but vacuous in execution — permanently returning FAIL.

**Fix applied:** S-18.09 v1.8 (story-writer): AC-008 awk rewritten to flag-form:
```awk
/^## Postconditions/{p=1} /^## / && p && !/^## Postconditions/{p=0} p{print}
```
This correctly extracts all lines in the Postconditions section without false-collapse.

---

### F-P8-002 — BLOCKER: BC-4.15.001 PC-B sub-emission labels unresolvable from S-18.09 gate

**Severity:** BLOCKER
**File:** `.factory/specs/behavioral-contracts/ss-04/BC-4.15.001.md`
**Status:** FIXED by product-owner (BC-4.15.001 v1.2 — PC-B-B1/PC-B-B2 promoted to citable subsection headings)

**Description:** BC-4.15.001 Postcondition B specifies two sub-emission channels: (B-1) stderr and (B-2) plugin.log. These were previously inline labels within a single PC-B paragraph, not separately addressable. S-18.09 AC-008 gate references "PC-B-B1" and "PC-B-B2" as distinct anchors, but BC-4.15.001 v1.1 had no such headings — the gate cites labels that do not exist in the normative document. A fresh-context adversary attempting to verify the gate against the BC cannot resolve these anchors.

**Root cause:** Gate author used anticipated sub-clause labels before they were promoted to citable headings in the BC.

**Fix applied:** BC-4.15.001 v1.2 (product-owner): PC-B restructured with explicit `#### PC-B-B1` and `#### PC-B-B2` subsection headings, making both anchors citable from S-18.09 AC-008.

---

### F-P8-003 — LOAD-BEARING MEDIUM: ARCH-INDEX Subsystem Registry per-subsystem BC counts stale

**Severity:** MEDIUM (load-bearing — row-sum ≠ Total; blocks row-sum==Total gate)
**File:** `.factory/specs/architecture/ARCH-INDEX.md`
**Section:** §Subsystem Registry table (rows SS-01..SS-10)
**Status:** OPEN → assigned to state-manager (index bookkeeping; D-619 BC-INDEX count reconcile precedent)

**Description:** The ARCH-INDEX Subsystem Registry table shows per-subsystem BC counts that sum to 1,949 but the stated Total is 1,972 (matches BC-INDEX v3.06). The discrepancy arose because per-row counts were not updated when BCs were added during E-18 F2/F3 work (D-612..D-619). The rows showing stale values:

| Subsystem | ARCH-INDEX shows | BC-INDEX v3.06 ground truth | Delta |
|-----------|-----------------|----------------------------|-------|
| SS-03 | 53 | 56 | +3 |
| SS-04 | 39 | 42 | +3 |
| SS-05 | 652 | 655 | +3 |
| SS-06 | 586 | 589 | +3 |
| SS-07 | 198 | 201 | +3 |
| SS-08 | 214 | 222 | +8 |

Ground truth source: BC-INDEX v3.06 §Summary table (state-manager reconciled at D-619). The row-sum-equals-Total invariant is violated (1,949 ≠ 1,972).

**Fix:** State-manager updates ARCH-INDEX Subsystem Registry rows to match BC-INDEX v3.06 Summary. ARCH-INDEX version bump v2.53→v2.54 with POLICY 14 parity.

---

### O-P8-A — OBSERVATION: S-18.09 AC-007 accumulator pattern completeness

**Severity:** OBSERVATION (non-violation)
**File:** `.factory/stories/S-18.09-f2-process-gap-lesson-gate-checks.md`
**Status:** FIXED proactively by story-writer (pass-8 fix burst)

**Description:** AC-007 lesson gate accumulator pattern did not include the `count` variable reset between test iterations in the test harness skeleton. While functionally correct for single-invocation gates, the absence of explicit reset creates ambiguity in harnesses that invoke the gate multiple times. Fixed proactively by story-writer.

---

### F-P8-004 — MEDIUM: S-18.09 AC-008 gate scope statement missing precondition carve-out

**Severity:** MEDIUM
**File:** `.factory/stories/S-18.09-f2-process-gap-lesson-gate-checks.md`
**Status:** FIXED by story-writer (pass-8 fix burst; story v1.8)

**Description:** AC-008 scope statement described the gate as checking "all BCs in the BC-INDEX catalog" without carving out withdrawn BCs. BC-2.02.013 is WITHDRAWN (D-224) and should not trigger a gate failure for missing postcondition registry-block-shape. The scope statement needed explicit carve-out: "all active BCs (status != withdrawn)".

---

### F-P8-005 — MEDIUM: S-18.09 AC-005 subsystem directory pattern uses %d not %02d

**Severity:** MEDIUM
**File:** `.factory/stories/S-18.09-f2-process-gap-lesson-gate-checks.md`
**Status:** FIXED by story-writer (pass-8 fix burst; story v1.8)

**Description:** AC-005 lesson gate pattern for subsystem directory paths used `ss-%d` format string which would match `ss-1` through `ss-9` and `ss-10` but not the canonical zero-padded form `ss-01` through `ss-10`. All actual paths use zero-padding. Fixed to `ss-%02d`.

---

## Part B — Pass-7 Closure Verification

| Finding | Pass-7 Status | Pass-8 Verification |
|---------|--------------|---------------------|
| F-P7-001 MAJOR (S-18.09 AC-008 silent-inert — no FAIL exit path) | FIXED story-writer | VERIFIED CLOSED — exit path present; awk fix in v1.8 supersedes (F-P8-001 is the pass-7 gate rewrite regression) |
| F-P7-002 LOW (S-18.08 WARN vs FAIL) | FIXED story-writer | VERIFIED CLOSED — FAIL used correctly |
| C-P7-001 BLOCKER (VP-086 catalog-row drift) | FIXED state-manager | VERIFIED CLOSED |
| C-P7-002 BLOCKER (bidirectional DAG blocks cells) | FIXED state-manager | VERIFIED CLOSED |
| C-P7-003 MAJOR (ARCH-INDEX POLICY 14 parity) | FIXED state-manager | VERIFIED CLOSED |
| C-P7-004 MED (STORY-INDEX line 190 narrative) | FIXED state-manager | VERIFIED CLOSED |
| C-P7-005 MED (S-18.09 AC-008 WARN vs FAIL echo) | FIXED story-writer | VERIFIED CLOSED |

## Summary

Pass-8 verdict: NOT-CLEAN. 3-CLEAN streak reset to 0/3.

- F-P8-001 BLOCKER: awk-range-collapse regression introduced by pass-7 gate rewrite — FIXED story-writer.
- F-P8-002 BLOCKER: BC-4.15.001 PC-B-B1/B2 label unresolvable — FIXED product-owner (v1.2 heading promotion).
- F-P8-003 MED (load-bearing): ARCH-INDEX subsystem-row BC-count drift — FIXED state-manager (this burst).
- O-P8-A OBS: accumulator reset — FIXED story-writer.
- F-P8-004 MED: gate scope missing withdrawn carve-out — FIXED story-writer.
- F-P8-005 MED: ss-%02d format — FIXED story-writer.

Pass-9 adversary + consistency re-verify NEXT.
