---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-08-20T00:00:00Z
phase: pre-TDD
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/adv-s21.19-local-pass-3.md
  - .factory/stories/S-21.19-executor-decision-function-core.md
  - .factory/stories/STORY-INDEX.md
  - .factory/specs/behavioral-contracts/BC-INDEX.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.03.017.md
input-hash: "c1bae4e"
traces_to: S-21.19-executor-decision-function-core.md
pass: 4
cascade: S-21.19-local
previous_review: .factory/cycles/v1.0-brownfield-backfill/adv-s21.19-local-pass-3.md
---

# Adversarial Review — S-21.19 (LOCAL pre-TDD cascade, pass 4) — NOT-CLEAN

Artifacts reviewed: story `S-21.19-executor-decision-function-core.md` v1.3 (input-hash `e6f82f2`,
UNCHANGED since pass 3); `STORY-INDEX.md` v4.376 (D-1057 sub-schedule blockquote); `BC-INDEX.md`
v4.85 (BC-1.03.017 Stories column). Rubric: full `.factory/policies.yaml` (POLICY 1-22). This is
pass 4 of the S-21.19 LOCAL pre-TDD cascade, reviewing the D-1061 pass-3 remediation (Task 2
BC-1.03.017 v1.19 cite sweep + `blocks:` bidirectional parity) applied against pass 3's findings,
and independently auditing the wider STORY-INDEX/BC-INDEX cross-reference surface the story
participates in.

## Verdict: NOT-CLEAN

1 MEDIUM finding, located in `STORY-INDEX.md` (not the S-21.19 story file itself), plus 2
non-resetting cross-story observations recorded for state-manager awareness. LOCAL streak 0/3 (a
NOT-CLEAN pass resets the streak per BC-5.39.001).

## Part A — Fix Verification (pass 3 findings)

**F-S2119-P3-001** (Task 2 stale `BC-1.03.017 v1.18` cite): **VERIFIED FIXED, no recurrence**.
S-21.19 v1.3 Task 2's `test_no_on_error_block_without_fail_closed_when_3arg_executor` cite now
reads `BC-1.03.017 v1.19 PC11`. Whitespace-normalized re-derivation of the same detector
(`tr '\n' ' ' | grep -oE 'BC-1\.03\.017[^v]{0,80}v1\.[0-9]+' | sort | uniq -c`) against the current
v1.3 body finds 0 live `v1.18` residuals — only the historical Changelog rows (v1.2, v1.3) that
narrate the sweep itself, which are exempt. `BC-1.01.016 v1.3` remains correctly anchored
throughout, no drift.

**F-S2119-P3-002** (`blocks:`/`depends_on:` asymmetry): **VERIFIED FIXED, no recurrence**. S-21.19
v1.3 frontmatter `blocks: [S-21.20, S-21.21, S-21.22, S-21.23, S-21.24]` now carries the reciprocal
`S-21.24` entry, matching S-21.24's `depends_on:` which already listed S-21.19. The direct
S-21.19→S-21.24 edge is present in the STORY-INDEX D-1057 sub-schedule DAG narrative and remains
acyclic (same direction as the existing transitive path through S-21.20-23).

Both prior structural-fix sets — ADR-044 capstone-flip split (D-1058, pass 1) and Invariant-7
wiring re-key + AC-009 `#[ignore]` gate (D-1060, pass 2) — CONFIRMED HELD at this pass, no
recurrence. The story file itself (v1.3) is unchanged this round; this pass's finding is located
entirely in STORY-INDEX.md, discovered by auditing the D-1057 sub-schedule blockquote's own
internal point-value consistency against the catalog rows it summarizes.

## Part B — New Findings

### MEDIUM

#### F-S2119-P4-001: D-1057 sub-schedule blockquote's mid-list points stale since the D-1058 redistribution
- **Severity:** MEDIUM
- **Category:** cross-document consistency / stale narrative value (not a story-file defect)
- **Location:** `STORY-INDEX.md`, the D-1057 sub-schedule blockquote (~line 751), the W6/W8
  mid-list point annotations for S-21.19 and S-21.24
- **Description:** The blockquote's W6 clause read `S-21.19 (9 pts; depends_on [S-21.10])` and its
  W8 clause read `S-21.24 (3 pts; depends_on [...])`. Both values are pre-D-1058: the ADR-044
  capstone-owned-flip remediation (pass-1 BLOCKER F-S2119-P1-001) redistributed points 9→7
  (S-21.19) and 3→5 (S-21.24), and both the catalog rows (line 741, 746) and both stories'
  frontmatter `points:` fields were correctly updated to 7 and 5 at that time. Only the blockquote's
  own mid-list prose retained the pre-redistribution 9/3 values. The drift persisted through passes
  2 and 3 undetected because the blockquote's stated aggregate — "7 new stories, 40 pts (S-21.19..
  S-21.24 = 35 pts ...)" — is points-neutral under the swap (9+3=12=7+5), so the top-line total
  arithmetic never surfaced the inconsistency.
- **Evidence:** Catalog row `S-21.19 | ... | E-21 | 7 | P1 | ...` (points column) and
  `S-21.24 | ... | E-21 | 5 | P1 | ...`; S-21.19 frontmatter `points: "7"`, S-21.24 frontmatter
  `points: "5"` — all four already correct. Blockquote text prior to this fix read `S-21.19
  (9 pts...)` and `S-21.24 (3 pts...)`, contradicting all four.
- **Proposed Fix:** Sweep the blockquote's mid-list `S-21.19 (9 pts)` → `(7 pts)` and
  `S-21.24 (3 pts)` → `(5 pts)`; verify against every catalog row's points column.
- **Status:** RESOLVED this burst (D-1062) — state-manager (this finding's content is
  STORY-INDEX-domain bookkeeping, not story content, so no story-writer dispatch was required)
  swept both values in the blockquote. Re-verification against the catalog rows and against
  S-21.19/S-21.24 frontmatter confirms all four now read consistently: 7 pts (S-21.19), 5 pts
  (S-21.24). S-21.19 story file itself is UNCHANGED this round (stays v1.3) — this was purely an
  index-artifact correction.

## Cross-Story Observations (non-resetting — recorded for awareness, not counted in this pass's
finding total or severity, since neither touches S-21.19's own content or correctness)

**O-S2119-P4-001 (deferred, not a defect to fix this burst):** S-21.20, S-21.21, and S-21.22 each
still cite `BC-1.03.017 v1.18` in both frontmatter and STORY-INDEX catalog row (self-consistent
with each other, but BC-1.03.017 itself has advanced to v1.19 via S-21.19's own D-1060 remediation).
This is real drift, not a false alarm — but per operator/orchestrator direction it is explicitly
deferred to those three stories' own Wave-7 convergence pass (they have not yet entered their own
adversarial cascade), not swept opportunistically here. Recorded so the drift is visible and does
not silently persist unnoticed.

**O-S2119-P4-002 (found during this pass, routed and resolved — BC-INDEX, not S-21.19 domain):**
`BC-INDEX.md`'s BC-1.03.017 Stories column listed S-21.23 as a citer. Cross-checked against
S-21.23's own frontmatter (`behavioral_contracts: ["BC-1.03.018 v1.1"]`) — S-21.23 never cited
BC-1.03.017; it is BC-1.03.018's own sibling BC (per its own catalog-row narrative). This is a
BC-INDEX bookkeeping error unrelated to S-21.19's own content, surfaced during this pass's wider
cross-reference audit. RESOLVED this burst (D-1062) — state-manager removed S-21.23 from the
Stories column.

## Disposition

F-S2119-P4-001 is STORY-INDEX-domain content (the D-1057 sub-schedule blockquote is
state-manager-authored index bookkeeping, not story-file content) — routed directly to
state-manager, no story-writer dispatch needed, consistent with the Agent Routing Table
("`.factory/STATE.md` updates, `.factory/` commits, cycle bookkeeping` → state-manager"; STORY-INDEX
catalog/blockquote maintenance is the same domain). Resolved same burst (D-1062). The two
cross-story observations are recorded, not counted against this pass's severity total: O-P4-001 is
an explicit, named, human/orchestrator-directed deferral (satisfies the Canonical Principle Rule 3
three-part test — explicit direction, concrete future dependency [Wave-7 convergence], attached to
specific stories); O-P4-002 was fixed in the same burst as a BC-INDEX-domain correction.

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0     |
| HIGH     | 0     |
| MEDIUM   | 1     |
| LOW      | 0     |

**Overall Assessment:** block (pre-remediation, STORY-INDEX v4.376 blockquote) — RESOLVED same
burst via state-manager STORY-INDEX + BC-INDEX remediation (D-1062). S-21.19 story file itself
(v1.3) UNCHANGED and CONFIRMED HELD throughout this pass.
**Convergence:** findings remain — iterate. LOCAL streak 0/3 (resolving the MEDIUM finding does not
itself advance the streak per BC-5.39.001; pass 5 required to confirm CLEAN).
**Readiness:** requires re-review (pass 5, against S-21.19 v1.3 + STORY-INDEX v4.377 + BC-INDEX
v4.86) before TDD dispatch.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 4 |
| **New findings** | 1 (+ 2 non-resetting observations) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (1/1) |
| **Median severity** | 2.0 (MEDIUM) |
| **Trajectory** | 1 → 2 → 2 → 1 |
| **Verdict** | FINDINGS_REMAIN — resolved this burst (D-1062); pass 5 required to confirm CLEAN against the remediated STORY-INDEX/BC-INDEX. Note: this pass's finding located entirely in index artifacts, not the story file — S-21.19 v1.3's own substantive content is CONFIRMED HELD, unchanged, across passes 3 and 4. |
