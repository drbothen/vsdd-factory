---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-24T00:00:00Z
phase: 6
inputs: []
input-hash: "[live-state]"
traces_to: ".factory/stories/STORY-S-21.03-pr-manager-orphan-merge.md"
pass: 6
previous_review: "adversary-pass-05.md"
story: S-21.03
cycle: v1.0-brownfield-backfill
verdict: NOT-CLEAN
reviewed_head: 194e98fe
reviewed_branch: feature/S-21.03-pr-manager-orphan-merge
base_commit: a4a79f09
date: 2026-07-24
---

# S-21.03 LOCAL Adversary Pass-6 — NOT-CLEAN

**Date:** 2026-07-24
**Story:** S-21.03 — PR-manager orphan-merge handling
**Pass:** 6 of BC-5.39.001 cascade
**Result:** NOT-CLEAN — streak 0/3
**Severity breakdown:** B0 / H0 / M1 / L0 / NITPICK0 / OBS2
**Total findings:** 1 finding + 2 observations
**Reviewed diff:** HEAD 194e98fe on feature/S-21.03-pr-manager-orphan-merge vs base a4a79f09

---

## Finding ID Convention

Finding IDs for this story's local cascade use the format: `F-S2103-P<PASS>-<SEQ>`

- `F`: Fixed prefix for factory local adversary findings
- `S2103`: Story identifier (S-21.03 compact form)
- `P<PASS>`: Pass number (e.g., `P6`)
- `<SEQ>`: Three-digit sequence within the pass (e.g., `001`)

Examples: `F-S2103-P6-001` (MEDIUM finding, pass 6, first finding).

---

## Part A — Prior Pass Finding Resolution

### F-S2103-P5-001 — Bats comment lines in RG-002/RG-003 retain stale "§Step 9"

- **Pass-5 verdict:** MEDIUM
- **Pass-6 resolution:** CONFIRMED-FIXED
- **Evidence:** Adversary performed independent 22-hit classified grep across all bats files for "step 9" / "Step 9" occurrences at HEAD 194e98fe. All 22 hits confirmed legitimate: either (a) historical changelog commentary exempt from update per TD-VSDD-091, (b) load-bearing assertion text where "step 9" refers to a different step in a different flow, or (c) the correct Step 8-post-A language. Zero residual stale "§Step 9" normative comment text detected in RG-002 or RG-003 test bodies. The 4 comment lines fixed at 194e98fe confirmed absent; proactive sibling sweep confirmed complete.

---

### F-S2103-P5-002 — Dangling RG-004 ID cited in bats but undefined in story Red Gate Test Plan

- **Pass-5 verdict:** MEDIUM
- **Pass-6 resolution:** CONFIRMED-FIXED
- **Evidence:** Story Red Gate Test Plan at 194e98fe confirmed: RG-004 row present — "TrunkFetchFailed red-gate (stub git fetch failing → pre-impl FAIL; no Step A/TrunkFetchFailed mandate in pr-manager.md)". Traces to BC-6.10.002 EC-007 / story EC-007; covered by T-006/T-007. Row confirmed load-bearing: adversary independently verified that T-006 (TrunkFetchFailed assertion) and T-007 (retry-succeed path) both reference the RG-004 gate entry; removal of RG-004 row would break traceability for two tests. Dangling-reference condition fully resolved.

---

### F-S2103-P5-003 — Story §Decision 6 mis-citation

- **Pass-5 verdict:** MEDIUM
- **Pass-6 resolution:** CONFIRMED-FIXED
- **Evidence:** Adversary independently re-derived the §Decision 8 + §Consequences #6 anchor justification against ADR-031 v1.8 at HEAD 194e98fe. §Decision 8 header confirmed: "PR-manager: orphan-merge detection step 8-post-A placement" — correct governing decision for BC-6.10.002 / INV-E21-006 enforcement. §Consequences #6 confirmed present: "BC-6.10.002 amendment is skill-doc only — the aggregate POLICY 21 attestation for the amendment". Story §Architecture Mapping context sentence confirmed updated to §Decision 8; Token Budget row parenthetical confirmed updated to §Decision 8 + §Consequences #6 with anchor justification note. No §Decision 6 references remain in S-21.03 normative scope.

---

## Part B — New Findings

### MEDIUM

#### F-S2103-P6-001 — File Structure Requirements Notes cell: stale "T-001..T-005" vs delivered 7-test suite

- **Severity:** MEDIUM
- **Category:** spec-fidelity (deliverable-manifest descriptor drift — range/count mismatch)
- **Location:** S-21.03 story spec — §File Structure Requirements table, Notes cell for the bats test file row
- **Description:** The §File Structure Requirements table Notes cell for the bats test harness cited "T-001..T-005" as the test range delivered by the implementation. However, the actual delivered test suite comprises 7 tests: T-001 through T-007 (T-006 TrunkFetchFailed-mandate + T-007 retry-succeed path were added during the cascade to cover EC-007 per F-S2103-P3-003 / F-S2103-P5-002 fix bursts). The Notes cell was not updated when the test count expanded, leaving a stale range descriptor that understates delivery by 2 tests. The fixtures row in the same table correctly identifies both fixture files (GIT_FETCH_FAIL_ONCE stub + base-setup helper), confirming the table was partially updated. This is a deliverable-manifest descriptor drift class finding: the story's own file structure manifest does not accurately describe what was delivered. The finding survived five prior passes because it presents as a minor Notes annotation rather than a normative contract; however, it misleads any reviewer auditing story completeness against the deliverable manifest.
- **Evidence:** Story §File Structure Requirements Notes cell verbatim: "covers T-001..T-005". Story §Test Plan table: 7 entries (T-001..T-007). Bats test file at 194e98fe: 7 test functions. Classified range/count sweep: "T-001..T-005" appears only in the Notes cell; "T-001..T-007" appears in §Test Plan table. Count mismatch confirmed. Fixtures row: accurate (lists `git_fetch_fail_once.bash` + `setup_helpers.bash`).
- **Fix:** Update §File Structure Requirements Notes cell for the bats test file row: "T-001..T-005" → "T-001..T-007". Perform range/count sweep to confirm no other stale range descriptors.
- **Routing:** story-writer
- **Status:** FIXED at ae1ad428 — story v1.8; Notes cell updated to T-001..T-007; classified range/count sweep clean; fixtures row verified accurate.

---

### Observations (ACCEPTED-WITH-RECORD)

#### OBS-P6-1 — Story-local vs BC EC numbering: offset-by-one readability trap

- **Observation:** S-21.03 uses story-local EC numbering (EC-001..EC-010) that maps to BC-6.10.002's EC numbering with an offset: story EC-001 corresponds to BC EC-001, but certain story-local ACs use EC-NNN labels that superficially appear to reference a different BC EC when cited without qualification. Specifically, story EC-007 (TrunkFetchFailed) resolves correctly to BC-6.10.002 EC-007 under the "story EC-NNN = BC EC-NNN" convention, but the convention is not stated in the story preamble, creating a readability trap for reviewers unfamiliar with the story's EC convention.
- **Classification:** NOT a mis-anchor. All BC-Trace cites in the story and bats file are explicitly qualified ("BC-6.10.002 EC-007") and resolve correctly. The offset-by-one description was imprecise: the numbering is actually aligned (not offset), but the absence of a preamble stating the alignment convention creates reader confusion.
- **Disposition:** ACCEPTED-WITH-RECORD. This is a pre-existing structural artifact of the story format. Adding a preamble convention statement is a readability improvement; it does not affect normative correctness. Not fixing in-cascade to avoid introducing second-order findings from preamble wording. Noted for wave-gate consideration.

---

#### OBS-P6-2 — T-003/T-004 use single-shot ancestry helper vs two-step Step-A/B helper used by T-006/T-007

- **Observation:** T-003 and T-004 use a single-shot ancestry verification helper that checks the post-merge ancestry state in one operation, while T-006 and T-007 use the two-step Step-A/B helper that mirrors the BC-6.10.002 Step A (fetch trunk) → Step B (ancestry check) decomposition introduced at F-S2103-P3-003 / EC-007. The test outcomes are correct for both paths: T-003/T-004 cover the nominal ancestry-check path; T-006/T-007 cover the TrunkFetchFailed / retry-succeed paths which require the two-step decomposition. However, the two-step path exercises a fuller harness implementation than the single-shot path for the same conceptual operation.
- **Classification:** Outcomes correct. The two-step path is covered by T-006/T-007. The single-shot helper in T-003/T-004 is a pre-existing implementation choice that predates the Step-A/B decomposition; migrating T-003/T-004 to the two-step helper would be a harness re-implementation, not a fix.
- **Disposition:** ACCEPTED-WITH-RECORD under the standing harness-re-implementation item. Orchestrator disposition: further harness churn risks more second-order findings than it removes; noted for wave-gate. No fix dispatched in this cascade.

---

### Clean Axes Verified

- **All P5 findings CONFIRMED-FIXED:** Independent re-derivation under fresh context confirmed all three P5 fix targets (comment sweep, RG-004 traceability, §Decision 8 re-anchor) fully resolved at 194e98fe.
- **Scope discipline:** All P5 fix commits (194e98fe, 3bf8463d) confined to their declared artifacts. No lateral scope expansion detected.
- **TD-VSDD-091:** All spec narrative at 194e98fe uses behavioral anchors (BC clause identifiers, step labels, EC-NNN error codes). No `file.rs:NNN` line-number citations found in normative sections.
- **TD-VSDD-060 sibling sweep:** The 22-hit Step-9 grep confirmed complete sibling coverage for F-S2103-P5-001 fix; no missed callsites.
- **Mutation coverage (carry-forward):** P4-002 mutation kill classes re-verified at HEAD — no regression.
- **Inert-idiom audit (carry-forward):** File-wide `false` occurrence audit at 194e98fe — all 30 confirmed load-bearing; no `&&{false;}||true` anti-pattern residue.
- **Novelty:** MEDIUM — single low-blast-radius spec-hygiene finding (deliverable-manifest descriptor drift). Substantive structural axes (guard correctness, mutation coverage, deletion-agnostic recovery, ordering, EC traceability, citation accuracy) all CONVERGED at P4/P5. Residue trend: converging.

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0 |
| HIGH     | 0 |
| MEDIUM   | 1 |
| LOW      | 0 |
| NITPICK  | 0 |
| OBS      | 2 |

**Overall Assessment:** block (1 MEDIUM finding)
**Convergence:** findings remain — iterate (streak 0/3)
**Readiness:** requires revision (fix burst dispatched; F-S2103-P6-001 FIXED at ae1ad428 story v1.8 prior to this persist; OBS-P6-1/OBS-P6-2 ACCEPTED-WITH-RECORD)
**Next reviewed HEAD:** 194e98fe (unchanged — fix ae1ad428 is story-spec-only; implementation HEAD unchanged)

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 6 |
| **New findings** | 1 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.00 (1 / (1 + 0)) |
| **Median severity** | 2.0 (MEDIUM) |
| **Trajectory** | 3→3→3→5→3→1 |
| **Verdict** | FINDINGS_REMAIN — single low-blast-radius spec-hygiene finding; all substantive axes converged; strongly converging |
