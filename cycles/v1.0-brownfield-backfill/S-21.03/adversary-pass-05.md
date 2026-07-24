---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-24T00:00:00Z
phase: 5
inputs: []
input-hash: "[live-state]"
traces_to: ".factory/stories/STORY-S-21.03-pr-manager-orphan-merge.md"
pass: 5
previous_review: "adversary-pass-04.md"
story: S-21.03
cycle: v1.0-brownfield-backfill
verdict: NOT-CLEAN
reviewed_head: fc08a70a
reviewed_branch: feature/S-21.03-pr-manager-orphan-merge
base_commit: a4a79f09
date: 2026-07-24
---

# S-21.03 LOCAL Adversary Pass-5 — NOT-CLEAN

**Date:** 2026-07-24
**Story:** S-21.03 — PR-manager orphan-merge handling
**Pass:** 5 of BC-5.39.001 cascade
**Result:** NOT-CLEAN — streak 0/3
**Severity breakdown:** B0 / H0 / M3 / L0 / NITPICK0 / OBS0
**Total findings:** 3 findings
**Reviewed diff:** HEAD fc08a70a on feature/S-21.03-pr-manager-orphan-merge vs base a4a79f09

---

## Finding ID Convention

Finding IDs for this story's local cascade use the format: `F-S2103-P<PASS>-<SEQ>`

- `F`: Fixed prefix for factory local adversary findings
- `S2103`: Story identifier (S-21.03 compact form)
- `P<PASS>`: Pass number (e.g., `P5`)
- `<SEQ>`: Three-digit sequence within the pass (e.g., `001`)

Examples: `F-S2103-P5-001` (MEDIUM finding, pass 5, first finding).

---

## Part A — Prior Pass Finding Resolution

### F-S2103-P4-001 — BC §Architecture Anchors "step 9" residual

- **Pass-4 verdict:** HIGH
- **Pass-5 resolution:** CONFIRMED-FIXED
- **Evidence:** Adversary performed independent full-document re-derivation of the inert-idiom audit across all BC-6.10.002 normative sections. Zero residue: all 30 `false` occurrences are load-bearing (negative assertion predicates, not inert guards). §Architecture Anchors confirmed updated to Step 8-post-A at d7b7f1c2 (BC v1.5); no cross-section contradiction detected.

---

### F-S2103-P4-002 — Five bats guards structurally inert (`&&{false;}||true`)

- **Pass-4 verdict:** HIGH
- **Pass-5 resolution:** CONFIRMED-FIXED
- **Evidence:** Adversary independently re-derived all three mutation kill classes (overrun mutant, misclassification mutant, mandate-deletion mutant) against fc08a70a. All five previously-inert guards replaced with structurally correct assertion forms; each mutation class now produces a detectable failure. Mutation kill evidence confirmed present in bats harness with verbatim failing-line output per D-449(a) discipline.

---

### F-S2103-P4-003 — ADR-031 §Decision 8 stale rationale

- **Pass-4 verdict:** MEDIUM
- **Pass-5 resolution:** CONFIRMED-FIXED
- **Evidence:** ADR-031 v1.8 at 8f4d725c confirmed: §Decision 8 rationale re-grounded to (a) Step 8-post-A ordering guarantee and (b) deletion-agnostic `headRefOid` anchor. §Consequences #5 ("ADR governs BC-6.10.002 enforcement ordering") confirmed frozen — no drift from the v1.8 text. v1.7 changelog entry preserved as historical per TD-VSDD-091.

---

### F-S2103-P4-004 — Bats test file header stale: references §Step 9 + missing EC-007/T-006

- **Pass-4 verdict:** MEDIUM
- **Pass-5 resolution:** CONFIRMED-FIXED (body) — sibling gap detected (→ F-S2103-P5-001)
- **Evidence:** Bats header confirmed updated to Step 8-post-A; EC-007 and T-006 added to coverage summary at fc08a70a. However adversary extended the sweep to inline comment lines within test bodies (RG-002 and RG-003) and found residual "§Step 9" references in comment-only lines — the P4-004 fix was header-scope only; the sibling comment lines were not swept. Classified F-S2103-P5-001 MEDIUM.

---

### F-S2103-P4-005 — Story Token Budget ADR-031 v1.3 cite drift

- **Pass-4 verdict:** LOW
- **Pass-5 resolution:** CONFIRMED-FIXED
- **Evidence:** Story v1.6 at fd861d94 confirmed Token Budget ADR-031 cite updated to v1.8; §Decision 8 anchor unchanged.

---

### OBS-P4-1 — Retry-succeed path for TrunkFetchFailed not tested

- **Pass-4 verdict:** FIXED at fc08a70a
- **Pass-5 resolution:** CONFIRMED-FIXED
- **Evidence:** T-007 confirmed present with `GIT_FETCH_FAIL_ONCE` stub; test verifies single-retry-succeed path does not trigger TrunkFetchFailed advisory. Stub confirmed load-bearing under mutation (removing stub causes T-007 to incorrectly pass — stub is necessary).

---

### OBS-P4-2 — SS-05/SS-06 cross-plane tension

- **Pass-4 verdict:** ADJUDICATED at 8f4d725c
- **Pass-5 resolution:** CONFIRMED-ADJUDICATED
- **Evidence:** ADR-031 §Decision 8 adjudication note at 8f4d725c confirmed present; SS-06 BC-6.10.002 anchoring rationale preserved. No re-adjudication required.

---

## Part B — New Findings

### MEDIUM

#### F-S2103-P5-001 — Bats comment lines in RG-002/RG-003 retain stale "§Step 9" (missed sibling of P4-004 header sweep)

- **Severity:** MEDIUM
- **Category:** spec-fidelity (documentation parity — sibling sweep gap; TD-VSDD-060 class)
- **Location:** Bats test file — comment lines inside RG-002 and RG-003 test bodies
- **Description:** The P4-004 fix updated the bats header section to reference Step 8-post-A, but the sweep was header-scope only. Adversary extended the review to inline comment lines within test bodies and found 4 comment lines in RG-002 and RG-003 still referencing "§Step 9". These include 2 siblings that could have been caught by a document-wide grep at P4-004 fix time. The "§Step 9" comment text is inconsistent with the Step 8-post-A language now established across all normative sections, and introduces reviewer confusion about the actual step placement.
- **Evidence:** Adversary performed 28-hit classified grep across all bats comment lines for "step 9" / "Step 9" occurrences; 4 lines identified as stale normative comment text (non-historical); 0 identified as historical changelog commentary. Lines confirmed in RG-002 and RG-003 test body comment blocks.
- **Fix:** Update 4 comment lines in RG-002/RG-003 test bodies to reference Step 8-post-A. Perform document-wide sweep to confirm zero residual stale comment references.
- **Routing:** test-writer
- **Status:** FIXED at 194e98fe — 4 comment lines updated (2 primary + 2 proactively-swept siblings); 28-hit classified grep re-run confirms zero residue.

---

#### F-S2103-P5-002 — Dangling RG-004 ID cited in bats but undefined in story Red Gate Test Plan

- **Severity:** MEDIUM
- **Category:** spec-fidelity (traceability gap — dangling reference)
- **Location:** Bats test file — RG-004 reference in test body; story Red Gate Test Plan (absence)
- **Description:** The bats test file at fc08a70a referenced `RG-004` in a test assertion context, but the story S-21.03 Red Gate Test Plan defined only RG-001, RG-002, and RG-003. RG-004 was not registered in the story's Red Gate Test Plan table, creating a dangling forward-reference: the bats file cited a test-plan entry that did not exist. This breaks the BC-5.39.001 traceability discipline requiring each bats test to correspond to a registered Red Gate Test Plan entry. Additionally, the orphan bats reference had no corresponding EC/T trace, meaning EC-007 / T-006 / T-007 were not properly anchored to the gate identifier.
- **Evidence:** Story Red Gate Test Plan at fc08a70a: three entries RG-001, RG-002, RG-003 — no RG-004. Bats file: reference to RG-004 in assertion comment without story-side definition.
- **Fix:** Register RG-004 row in story Red Gate Test Plan: TrunkFetchFailed red-gate (stub git fetch failing → pre-impl FAIL; no Step A/TrunkFetchFailed mandate in pr-manager.md); traces to BC-6.10.002 EC-007 / story EC-007; covered by T-006/T-007.
- **Routing:** story-writer
- **Status:** FIXED at 3bf8463d — RG-004 row registered in story Red Gate Test Plan; traces EC-007/T-006/T-007 anchored.

---

#### F-S2103-P5-003 — Story §Decision 6 mis-citation: §Decision 6 governs S-21.02's INV-E21-005, not S-21.03

- **Severity:** MEDIUM
- **Category:** spec-fidelity (cross-story citation error)
- **Location:** S-21.03 story spec — §Architecture Mapping context sentence + Token Budget row parenthetical
- **Description:** Two locations in story S-21.03 cited "§Decision 6" as the governing ADR-031 decision for the post-merge ancestry assertion (INV-E21-006 enforcement). ADR-031 §Decision 6 governs S-21.02's INV-E21-005 (post-rebase diff-integrity gate), not S-21.03. The correct governing decision is §Decision 8 ("PR-manager: orphan-merge detection step 8-post-A placement"), which explicitly records: "No new WASM plugin or shell script (POLICY 21 satisfied)" and anchors INV-E21-006. The §Consequences #6 entry ("BC-6.10.002 amendment is skill-doc only — the aggregate POLICY 21 attestation for the amendment") further grounds the S-21.03 architectural decision, not §Decision 6.
- **Evidence:** Story §Architecture Mapping context sentence verbatim: "...governed by ADR-031 §Decision 6..." Story Token Budget row parenthetical: "(ADR-031 §Decision 6 anchor)". ADR-031 §Decision 6 header: "post-rebase diff-integrity gate" — definitively S-21.02 scope.
- **Fix:** Replace §Decision 6 with §Decision 8 + §Consequences #6 in both locations. Add anchor justification: §Decision 8 records POLICY 21 satisfaction for BC-6.10.002 amendment; §Consequences #6 names the aggregate attestation.
- **Routing:** story-writer
- **Status:** FIXED at 3bf8463d — §Decision 8 + §Consequences #6 anchors substituted in both locations with justification note.

---

### Clean Axes Verified

- **Scope discipline:** All five P4 fix commits (d7b7f1c2, fc08a70a, 8f4d725c, fd861d94) confined to their declared artifacts. No lateral scope expansion.
- **TD-VSDD-091:** All spec narrative uses behavioral anchors (BC clause identifiers, step labels, EC-NNN error codes), not line numbers.
- **Mutation coverage:** Adversary independently re-derived all three P4-002 mutation kill classes at fc08a70a — zero residue; guards now structurally correct.
- **Inert-idiom audit:** File-wide re-audit of all `false` occurrences across bats harness — all 30 confirmed load-bearing; no residual `&&{false;}||true` anti-pattern.
- **BC sweep classification:** §Architecture Anchors sweep re-verified; BC-6.10.002 v1.5 normative sections clean.
- **ADR §Consequences #5 frozen:** ADR-031 v1.8 §Consequences #5 text confirmed unchanged from v1.7; only §Decision 8 rationale updated.
- **Novelty:** MEDIUM — P5 findings are citation/traceability hygiene class (dangling ID, cross-story mis-cite, sibling comment sweep gap) rather than structural defects. Substantive surfaces (guard load-bearingness, mutation coverage, deletion-agnostic recovery, ordering) CONVERGED at P4; residue trend is approaching convergence.

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0 |
| HIGH     | 0 |
| MEDIUM   | 3 |
| LOW      | 0 |
| NITPICK  | 0 |
| OBS      | 0 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate (streak 0/3)
**Readiness:** requires revision (fix burst dispatched; all findings FIXED in-burst prior to this persist)
**Next reviewed HEAD:** 194e98fe

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 5 |
| **New findings** | 3 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.00 (3 / (3 + 0)) |
| **Median severity** | 2.0 (MEDIUM) |
| **Trajectory** | 3→3→3→5→3 |
| **Verdict** | FINDINGS_REMAIN — approaching convergence; substantive surface axes converged; residue is citation hygiene |
