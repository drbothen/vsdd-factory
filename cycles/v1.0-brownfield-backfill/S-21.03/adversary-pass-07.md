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
pass: 7
previous_review: "adversary-pass-06.md"
story: S-21.03
cycle: v1.0-brownfield-backfill
verdict: CLEAN
reviewed_head: 194e98fe
reviewed_branch: feature/S-21.03-pr-manager-orphan-merge
base_commit: a4a79f09
date: 2026-07-24
---

# S-21.03 LOCAL Adversary Pass-7 — CLEAN

**Date:** 2026-07-24
**Story:** S-21.03 — PR-manager orphan-merge handling
**Pass:** 7 of BC-5.39.001 cascade
**Result:** CLEAN — streak 1/3
**Severity breakdown:** B0 / H0 / M0 / L0 / NITPICK0 / OBS1
**Total findings:** 0 findings + 1 observation
**Reviewed diff:** HEAD 194e98fe on feature/S-21.03-pr-manager-orphan-merge vs base a4a79f09
**Note:** Implementation HEAD unchanged since pass 5 — pass-6 fix burst ae1ad428 was story-spec-only (§File Structure Requirements Notes cell range update)

---

## Finding ID Convention

Finding IDs for this story's local cascade use the format: `F-S2103-P<PASS>-<SEQ>`

- `F`: Fixed prefix for factory local adversary findings
- `S2103`: Story identifier (S-21.03 compact form)
- `P<PASS>`: Pass number (e.g., `P7`)
- `<SEQ>`: Three-digit sequence within the pass (e.g., `001`)

Examples: `F-S2103-P7-001` (hypothetical finding, pass 7, first finding).

---

## Part A — Prior Pass Finding Resolution

### F-S2103-P6-001 — File Structure Requirements Notes cell: stale "T-001..T-005" vs delivered 7-test suite

- **Pass-6 verdict:** MEDIUM
- **Pass-7 resolution:** CONFIRMED-FIXED
- **Evidence:** Adversary independently verified the §File Structure Requirements table Notes cell for the bats test file row at story v1.8 (fixed at ae1ad428). Cell confirmed reads "covers T-001..T-007" — correctly reflecting the 7-test suite delivered. Cross-check against §Test Plan table: 7 entries T-001..T-007 confirmed present; count matches. Cross-check against bats test file at 194e98fe: 7 test functions confirmed; range matches. Zero residual range/count sweep: grep across all story sections for "T-001..T-005" range notation returned zero normative hits — the stale descriptor is fully absent. Fixtures row (GIT_FETCH_FAIL_ONCE stub + base-setup helper) confirmed accurate and unchanged. This is not a paper-fix: the cell content was structurally updated to describe delivered reality; the §Test Plan table and bats implementation corroborate the corrected count from independent sources.

---

## Part B — New Findings

No findings at BLOCKER, HIGH, MEDIUM, LOW, or NITPICK severity.

Full independent re-derivation of all four substantive surfaces performed under fresh context:

**Surface 1 — Doc-parity grep targets:** All doc-parity grep targets cited in the bats test harness (Step 8-post-A label, BC-6.10.002 EC-007, Step A/Step B split labels) confirmed present in their extracted sections. No grep target resolves to an absent or renamed anchor. Parity between bats assertion targets and spec text is CONVERGED.

**Surface 2 — Ordering invariant on bold headings:** Story §Architecture Mapping bold heading sequence independently re-derived against ADR-031 §Decision 8 ordering. All bold heading anchors present in correct order; no transposition, insertion, or deletion detected. Ordering invariant CONVERGED.

**Surface 3 — Step A/B retry-once harness fidelity:** Independent re-derivation of the Step A (git fetch trunk) → Step B (merge-base ancestry check) decomposition in pr-manager.md against BC-6.10.002 §Decision 8 + EC-007 mandate. GIT_FETCH_FAIL_ONCE stub priority and hermeticity verified: stub is loaded before the git executable on PATH, intercepted once, then removed; subsequent git commands resolve to real git. T-006 (TrunkFetchFailed mandate: pre-impl fails on step-A guard) and T-007 (retry-succeed: Step A succeeds after one-shot failure, Step B proceeds) correctly exercise the full two-path harness. No inert guards detected (carry-forward from P4-002 mutation verification). Harness fidelity CONVERGED.

**Surface 4 — Deletion-agnostic recovery:** BC-6.10.002 headRefOid-based deletion-agnostic recovery (introduced F-S2103-P3-002 / BC v1.4) independently re-derived in pr-manager.md at 194e98fe. Recovery path uses headRefOid captured at merge time, not branch name; verified not contingent on delete_branch_on_merge setting. Recovery mechanism CONVERGED.

**Semantic-anchoring audit:** All BC-Trace citations at 194e98fe independently verified: SS-06 (Step 8-post-A placement anchor), CAP-038 (orphan-merge detection capability), §Decision 8 (ADR-031 v1.8 governing decision for BC-6.10.002 Step 8 placement), BC H1 (behavioral contract title). All anchors resolve correctly to their source documents. No mis-anchor, stale-anchor, or phantom-anchor detected.

---

### Observations (ACCEPTED-WITH-RECORD)

#### OBS-P7-1 — Trunk-parameterization asymmetry: Step 3 error body hardcodes `develop`

- **Observation:** The Step 3 error body in pr-manager.md hardcodes the branch name `develop` at the point where it reports a failed feature-PR create flow, rather than using the BC PC2 `<trunk>` placeholder or the configured trunk variable. BC-6.10.002 PC2 specifies the trunk branch as a configurable parameter; the assertion instruction in the story and test harness correctly says "configured trunk." The Step 3 hardcode is a trunk-parameterization asymmetry: the assertion target resolves correctly because the test environment configures `develop` as trunk, but the hardcode would be incorrect in a deployment where trunk is named differently (e.g., `main`).
- **Classification:** Defensible-by-design for the current story scope. Step 3 is the feature-PR create flow (creating the PR against the feature branch, not the orphan-merge detection path). The hardcode is pre-existing in the pr-manager.md base implementation and falls outside the story diff for S-21.03, which addresses the orphan-merge detection step (Step 8-post-A). Changing the Step 3 hardcode would require a story that explicitly addresses trunk-parameterization in the PR-create flow; no such story is in scope for this cascade. The assertion instruction correctly says "configured trunk" and the test environment alignment makes the assertion functionally correct at current HEAD.
- **Disposition:** ACCEPTED. Recorded informational. Adjudicated defensible-by-design — Step 3 is outside the story diff; pre-existing hardcode not introduced by S-21.03; assertion instruction correct. Not dispatching a fix in-cascade. Noted for wave-gate consideration as a trunk-parameterization hardening item.

---

### Clean Axes Verified

- **All P6 findings CONFIRMED-FIXED:** F-S2103-P6-001 independently verified as non-paper-fix: cell content updated to match delivered 7-test suite; §Test Plan + bats cross-check corroborate; zero residual range sweep clean.
- **Scope discipline:** Fix commit ae1ad428 confined to §File Structure Requirements Notes cell update (story v1.8). No lateral scope expansion detected.
- **TD-VSDD-091:** All spec narrative at 194e98fe uses behavioral anchors (BC clause identifiers, step labels, EC-NNN error codes). No `file.rs:NNN` line-number citations found in normative sections.
- **TD-VSDD-060 sibling sweep (carry-forward):** 22-hit Step-9 grep from P6 re-confirmed at story v1.8; Notes cell update does not introduce new sibling sites. No missed callsites.
- **Mutation coverage (carry-forward):** P4-002 mutation kill classes (overrun, misclassification, mandate-deletion) re-verified at HEAD — no regression.
- **Inert-idiom audit (carry-forward):** File-wide `false` occurrence audit at 194e98fe — all 30 confirmed load-bearing; no `&&{false;}||true` anti-pattern residue.
- **OBS-P6-1 / OBS-P6-2 (carry-forward):** Both accepted-with-record observations re-confirmed as non-normative at HEAD. EC numbering alignment convention absent from preamble but all qualified cites correct; single-shot vs two-step helper asymmetry present but outcomes correct for both paths.
- **Novelty:** LOW — zero actionable findings. Single defensible-by-design observation (trunk-parameterization asymmetry in pre-existing out-of-scope code). All substantive structural axes CONVERGED across passes 4–7. Residue trend: CONVERGED.

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0 |
| HIGH     | 0 |
| MEDIUM   | 0 |
| LOW      | 0 |
| NITPICK  | 0 |
| OBS      | 1 |

**Overall Assessment:** CLEAN (0 actionable findings; 1 observation ACCEPTED)
**Convergence:** streak 1/3 — iterate (2 more CLEAN passes required for BC-5.39.001 convergence)
**Readiness:** CLEAN — no revision required; OBS-P7-1 noted for wave-gate
**Next reviewed HEAD:** 194e98fe (unchanged — no fix burst dispatched; pass-8 will re-review same HEAD)

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 7 |
| **New findings** | 0 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | N/A (no findings) |
| **Median severity** | N/A |
| **Trajectory** | 3→3→3→5→3→1→0 |
| **Verdict** | CONVERGED — all substantive axes confirmed clean; single defensible-by-design observation outside story diff; streak 1/3 |
