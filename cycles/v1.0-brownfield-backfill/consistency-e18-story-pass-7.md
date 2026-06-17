---
document_type: consistency-validation-report
cycle: v1.0-brownfield-backfill
cascade: E-18-story
pass: 7
date: 2026-06-17
validator: consistency-validator
verdict: INCONSISTENT
finding_count: 5
findings_by_severity:
  blocker: 2
  major: 1
  medium: 2
  low: 0
parent_pass: 6
parent_decision: D-623
---

# E-18 Story Cascade Consistency Report — Pass 7

**Date:** 2026-06-17
**Verdict:** INCONSISTENT
**Finding count:** 5 (2 BLOCKER + 1 MAJOR + 2 MED)
**All findings closed by story-writer + state-manager fix burst (D-624)**

## Findings

### C-P7-001 [BLOCKER] — VP-086 catalog row drift: anchor_story annotation stale

**Type:** Cross-document consistency
**Severity:** BLOCKER

**Finding:**
VP-086 in VP-INDEX Full Index table shows `anchor_story: S-18.00` which is correct, but the VP body row description annotation in the story S-18.00 BCs cell shows `VP-086` without the current VP-086 version. VP-INDEX v2.36 body shows VP-086 v1.4 as the latest, but S-18.00 STORY-INDEX annotation does not carry a VP version cite (it only says `VP-086` bare). This is a catalog-row drift: VP-INDEX body version v1.4 is not propagated to the story's annotation summary.

**Scope:** VP-INDEX.md Full Index row for VP-086 + STORY-INDEX S-18.00 BCs cell annotation.

**Required fix:** State-manager sync — STORY-INDEX version annotation must carry VP-086 version cite (or be documented as intentionally bare). Verified: VP-086 v1.4 (`grep '^version:' VP-086.md` → "1.4"). Catalog-row cite convention per L-F2-index-cell-and-version-cite-sibling-sweep: index annotations must carry current version cite for tracked VPs.

**Disposition:** CLOSED by state-manager in D-624 STORY-INDEX v4.09 — S-18.00 annotation updated to `VP-086 (v1.4)`.

---

### C-P7-002 [BLOCKER] — Bidirectional DAG blocks sweep incomplete: 4 stories missing reverse edges

**Type:** DAG consistency
**Severity:** BLOCKER

**Finding:**
Bidirectional DAG invariant: for every `A depends_on B`, the story B must list A in its `blocks:` array, and the STORY-INDEX Blocks cell for B must include A.

Sweep of all 12 E-18 stories vs. STORY-INDEX Blocks cells revealed 4 asymmetric entries in STORY-INDEX (story frontmatter already updated by story-writer but STORY-INDEX Blocks cells not yet synced):

1. **S-18.00 Blocks cell:** frontmatter blocks: [S-18.01, S-18.04a, S-18.05] but STORY-INDEX shows [S-18.01, S-18.04a] — missing S-18.05.
2. **S-18.04a Blocks cell:** frontmatter blocks: [S-18.03, S-18.04b, S-18.07, S-18.08] but STORY-INDEX shows [S-18.04b, S-18.08] — missing S-18.03, S-18.07.
3. **S-18.04b Blocks cell:** frontmatter blocks: [S-18.03, S-18.07, S-18.08] but STORY-INDEX shows [S-18.08] — missing S-18.03, S-18.07.
4. **S-18.07 Blocks cell:** frontmatter blocks: [S-18.08, S-18.10] but STORY-INDEX shows [S-18.08] — missing S-18.10.

**Required fix:** STORY-INDEX Blocks cells for S-18.00, S-18.04a, S-18.04b, S-18.07 must be synced to match frontmatter blocks: arrays.

**Disposition:** CLOSED by state-manager in D-624 STORY-INDEX v4.09.

---

### C-P7-003 [MAJOR] — ARCH-INDEX body BC-count text stale (architect edit without version bump)

**Type:** Version-parity gap
**Severity:** MAJOR

**Finding:**
ARCH-INDEX body line 384 reads "Total BCs: 1,972 (per BC-INDEX v3.06; ...)" — this was edited by architect to update the BC count from 1,949/v1.84 to 1,972/v3.06 as part of this pass-7 fix burst. However, ARCH-INDEX `version:` frontmatter still shows "2.52" — the version was NOT bumped to match the body edit. POLICY 14 5-leg quintuple parity is violated: body Changelog row is absent for this edit; `version:` frontmatter was not advanced; `last_amended:` was not updated.

**Required fix:** State-manager must bump ARCH-INDEX v2.52 → v2.53 with full POLICY 14 5-leg parity. Also §Document Map verification-architecture.md annotation must be updated v1.3 → v1.4 (architect bumped verification-architecture.md to v1.4 in this pass-7 burst).

**Disposition:** CLOSED by state-manager in D-624 ARCH-INDEX v2.53.

---

### C-P7-004 [MED] — STORY-INDEX line 190 narrative stale: "99 stories across 17 epics (E-0 through E-16)"

**Type:** Stale narrative
**Severity:** MED

**Finding:**
STORY-INDEX body line 190 contains the authoritative story-count narrative: "99 stories across 17 epics (E-0 through E-16)." This was written when the story count was 99. As of D-614/D-615/D-616, the story count is 120 across 19 epics (E-0 through E-18). E-17 was added (4 stories: S-17.01..S-17.04) and E-18 was added (12 stories: S-18.00..S-18.10). The narrative has not been updated for either epic addition.

**Required fix:** State-manager updates line 190 to "120 stories across 19 epics (E-0 through E-18)" and appends E-17/E-18 addition notes per the pattern established by the existing E-10 through E-16 entries.

**Disposition:** CLOSED by state-manager in D-624 STORY-INDEX v4.09.

---

### C-P7-005 [MED] — S-18.09 AC-008 gate specification: WARN vs FAIL — silent-inert class (consistency echo of F-P7-001)

**Type:** Specification consistency
**Severity:** MED

**Finding:**
Consistency-validator independently confirmed F-P7-001 via cross-document check: S-18.09 AC-008 specifies a gate but does not carry a failure-path specification. Cross-checking S-18.09 v1.6 AC-008 text against the L-F2-ac-pc-parity-sibling-sweep lesson (codified in lessons.md) confirms the class: a "gate" that cannot exit non-zero is structurally inert. The consistency-validator assigns MED severity because this is a spec-quality issue (the gate exists but is incomplete) rather than a behavioral contradiction.

**Required fix:** Story-writer must add FAIL exit-path clause to AC-008 in S-18.09. (Coordinated with F-P7-001 fix by story-writer.)

**Disposition:** CLOSED by story-writer v1.7 in this D-624 burst.
