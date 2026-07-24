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
pass: 3
previous_review: "adversary-pass-02.md"
story: S-21.03
cycle: v1.0-brownfield-backfill
verdict: NOT-CLEAN
reviewed_head: 9bd75ab3
reviewed_branch: feature/S-21.03-pr-manager-orphan-merge
base_commit: a4a79f09
date: 2026-07-24
---

# S-21.03 LOCAL Adversary Pass-3 — NOT-CLEAN

**Date:** 2026-07-24
**Story:** S-21.03 — PR-manager orphan-merge handling
**Pass:** 3 of BC-5.39.001 cascade
**Result:** NOT-CLEAN — streak 0/3
**Severity breakdown:** B0 / H0 / M3 / L0 / NITPICK0 / OBS4
**Total findings:** 3 findings + 4 observations
**Reviewed diff:** HEAD 9bd75ab3 on feature/S-21.03-pr-manager-orphan-merge vs base a4a79f09

---

## Finding ID Convention

Finding IDs for this story's local cascade use the format: `F-S2103-P<PASS>-<SEQ>`

- `F`: Fixed prefix for factory local adversary findings
- `S2103`: Story identifier (S-21.03 compact form)
- `P<PASS>`: Pass number (e.g., `P3`)
- `<SEQ>`: Three-digit sequence within the pass (e.g., `001`)

Examples: `F-S2103-P3-001` (MEDIUM finding, pass 3, first finding).

---

## Part A — Prior Pass Finding Resolution

### F-S2103-P2-002 — PC3 doc-parity greps salt Step 9 back-reference, not the relocated mandate

- **Pass-2 verdict:** HIGH
- **Pass-3 resolution:** CONFIRMED-FIXED
- **Evidence:** Adversary independently re-ran mutation reasoning: `_extract_step8_post_a_section` extraction function is present and anchored to bold-heading markers; ordering assertion verifies ancestry check precedes deletion invocation; a salted copy with the Step 8-post-A block deleted fails T-003/T-004/T-005 as required. The bold-heading anchor approach (`/^\*\*Step 8[^-]/`) is robust against the historical "Step 9 → Step 8-post-A" back-reference strings that previously allowed false-green. The mutation verification confirms structural delete → test failure.

---

### F-S2103-P2-001 — `--delete-branch` at Step 8-pre-B defeats recovery affordance

- **Pass-2 verdict:** MEDIUM
- **Pass-3 resolution:** CONFIRMED-FIXED (with second-order finding F-S2103-P3-002)
- **Evidence:** `--delete-branch` removed from Step 8-pre-B `gh pr merge` invocation; Steps 8b/8c/8d are now sole deletion mechanism. Recovery affordance preserved: remote head intact at Step 8-post-A execution time. Second-order issue: the recovery affordance now depends on the repo's `delete_branch_on_merge` setting. If true, GitHub's automatic deletion fires at merge time regardless of the `--delete-branch` flag — the `--delete-branch` removal alone does not guarantee the affordance. Orchestrator grounded: `gh api` confirms `delete_branch_on_merge=true` for this deployment; however the BC recovery path must be deletion-agnostic (use `headRefOid` from PR metadata rather than assuming live remote head). This prompted F-S2103-P3-002.

---

### F-S2103-P2-003 — Story and ADR-031 §Decision 8 cite "Step 9 amendment" vs correct Step 8-post-A placement

- **Pass-2 verdict:** MEDIUM
- **Pass-3 resolution:** PARTIALLY-FIXED
- **Evidence:** ADR-031 §Decision 8 leg confirmed fixed at c97faaae (ADR-031 v1.7). Story leg: non-historical placement cites corrected at cdc60e8c (story v1.4). BC Traceability Step leg (BC-6.10.002 Traceability section): deferred to F-S2103-P3-003 (confirmed this burst at 36d69804 + BC v1.4). CLOSED this burst.

---

## Part B — New Findings

### MEDIUM

#### F-S2103-P3-001 — Previous Story Intelligence cell still contains stale "step 9" reference

- **Severity:** MEDIUM
- **Category:** spec-fidelity (S-7.01(c) partial-fix class)
- **Location:** S-21.03 story spec §Previous Story Intelligence section
- **Description:** Following the Step 9 → Step 8-post-A relocation, the story spec §Previous Story Intelligence cell retained a "step 9" reference describing the ancestry assertion placement. This created a documentation divergence within the same story spec: the §Tasks section correctly cited Step 8-post-A, while §Previous Story Intelligence contradicted it with "step 9". This is a S-7.01(c) partial-fix pattern: the primary fix sites were corrected but one cross-section site was missed.
- **Evidence:** §Previous Story Intelligence verbatim text contained "step 9" in the context of describing the ancestry assertion placement, inconsistent with the corrected §Tasks and §Architecture Compliance Rules sections.
- **Fix:** Update §Previous Story Intelligence to reference Step 8-post-A consistently with all other corrected sections.
- **Routing:** story-writer
- **Status:** FIXED at 9b020a98 — story v1.5; §Previous Story Intelligence updated to Step 8-post-A; input-hash e5c4e8d→56e51ee.

---

#### F-S2103-P3-002 — Recovery affordance contingent on repo auto-delete setting; promise is not deletion-agnostic

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** BC-6.10.002 PC3 recovery path; pr-manager.sh Step 8-post-A recovery block
- **Description:** After the `--delete-branch` removal (F-S2103-P2-001 fix), the recovery affordance remained implicitly contingent on the remote head branch being intact at recovery time. With `delete_branch_on_merge=true` on this deployment, GitHub auto-deletes the remote head at merge time regardless of whether `--delete-branch` was passed. The documented operator recovery path (git reset + force-push) therefore fails silently if invoked after a repo with auto-delete enabled. The BC's recovery affordance promise must be deletion-agnostic: recovery should succeed whether or not the remote head exists, using the PR's `headRefOid` (immutable, retained by GitHub post-merge regardless of branch deletion).
- **Evidence:** Orchestrator grounded via `gh api repos/...` — `delete_branch_on_merge=true` for this deployment. The `headRefOid` field on merged PRs is permanently retained by GitHub. A deletion-agnostic recovery path (fetch from PR headRefOid via `gh pr view --json headRefOid`) is feasible and provides invariant recovery regardless of branch lifecycle.
- **Fix:** BC-6.10.002 Traceability Step 8-post-A recovery block: specify `headRefOid`-based recovery (via `gh pr view --json headRefOid`) as the primary recovery path. pr-manager.sh Step 8-post-A recovery block: mirror the deletion-agnostic recovery path (store headRefOid before merge, use it in BLOCK handler). BC v1.4 + pr-manager mirror.
- **Routing:** product-owner (BC-6.10.002 v1.4); pr-manager implementer mirror
- **Status:** FIXED at 36d69804 (BC-6.10.002 v1.4 — deletion-agnostic PC3 recovery via PR-retained headRefOid) + 96627798 (pr-manager.sh mirror).

---

#### F-S2103-P3-003 — git fetch && merge-base conflation causes false orphan-merge P0 on any non-ff merge

- **Severity:** MEDIUM
- **Category:** correctness
- **Location:** pr-manager.sh Step A/Step B ancestry check logic
- **Description:** The Step 8-post-A ancestry check used `git fetch` followed immediately by `git merge-base --is-ancestor` without separating the trunk-fetch failure case from the ancestry-check failure case. If `git fetch` fails transiently (network issue, auth timeout), the subsequent `merge-base` check would operate on a stale trunk ref and potentially produce a false-positive BLOCK (reporting orphan-merge P0 when no orphan merge occurred). Additionally, any non-fast-forward merge (legitimate octopus merge, merge with rebase) would trigger the same BLOCK path as a genuine orphan merge, with no retry-once mechanism to distinguish transient fetch failures from genuine ancestry violations.
- **Evidence:** Code path: `git fetch origin "$trunk"` exit code not separately checked before `git merge-base --is-ancestor "$trunk_sha" "$pr_sha"`. A transient fetch failure leaves `$trunk_sha` as a stale cached ref. The merge-base then compares stale trunk against PR head, which may yield false BLOCK if the stale trunk is behind. No separate `TrunkFetchFailed` transient error variant exists.
- **Fix:** Split Step A (fetch trunk) from Step B (ancestry check). Add `TrunkFetchFailed` transient error variant with retry-once logic. On fetch failure: retry once; on second failure: emit `TrunkFetchFailed` advisory (not BLOCK). BC-6.10.002 Step A/B split + EC-007 TrunkFetchFailed + T-006 coverage.
- **Routing:** product-owner (BC-6.10.002 v1.4 Step A/B + EC-007); implementer mirror; test-writer (T-006)
- **Status:** FIXED at 36d69804 (BC-6.10.002 v1.4 — Step A/B split + TrunkFetchFailed EC-007) + 96627798 (pr-manager.sh mirror) + 12282097 (T-006 coverage; T-006 title initially mis-cited nonexistent AC-006, caught by orchestrator pre-pass-4, corrected at 2a6fa036 to EC-007).

---

### Observations (non-blocking)

#### OBS-P3-1 — Extractor anchor patterns fragile against step-label reformatting

- **Verdict:** FIXED at 12282097
- **Evidence:** Bold-heading anchors (`/^\*\*Step 8[^-]/`) and `/^\*\*Step 8-post-A/` regex patterns added; belt-and-braces empty+overrun guards added to extractor function; mutation re-verified against salted corpus.

---

#### OBS-P3-2 — Weak ordering assertion: checks structural position but not behavioral completeness

- **Verdict:** FIXED at 12282097
- **Evidence:** Ordering assertion strengthened; extractor now verifies both presence and structural integrity of Step 8-post-A block; mutation coverage expanded.

---

#### OBS-P3-3 — Bats harness re-implements a subset of pr-manager.sh logic

- **Verdict:** NOTED-ACCEPTED
- **Evidence:** Harness re-implementation is inherent to the bats integration-test architecture for this story. The extraction-function approach mitigates the staleness risk. Accepted as-is; no structural change warranted.

---

#### OBS-P3-4 — Step 8c rationale section cited potentially-stale `--delete-branch` justification

- **Verdict:** FIXED at 96627798
- **Evidence:** Rationale re-grounded to `delete_branch_on_merge=true` auto-delete deployment reality; Step 8c rationale updated to reflect deletion-agnostic recovery via headRefOid.

---

### Orchestrator Addendum (post-pass-3, pre-pass-4)

T-006 test title initially mis-cited nonexistent AC-006 (pr-manager.sh implements ACs, not EC-NNN identifiers). Caught by orchestrator pre-pass-4 review. Fixed at 2a6fa036: T-006 title updated to correctly reference EC-007 (TrunkFetchFailed). Pass-4 reviewed HEAD is 2a6fa036.

---

### Clean Axes Verified

- **Scope discipline:** Files touched confined to pr-manager.sh, bats test file, story spec, BC-6.10.002, and ADR-031. No lateral scope expansion.
- **TD-VSDD-091:** All spec narrative cites use behavioral anchors (BC-6.10.002 clause identifiers, step labels, EC-NNN error codes) rather than line numbers.
- **POLICY 19:** No hardcoded credentials or secrets in fixture stubs.
- **F-S2103-P2-002 CONFIRMED-FIXED:** `_extract_step8_post_a_section` bold-heading anchors confirmed; mutation verification confirmed structural delete → test failure.
- **F-S2103-P2-001 CONFIRMED-FIXED:** `--delete-branch` removed; recovery affordance preserved via headRefOid path (F-S2103-P3-002 closes the deletion-agnostic gap).
- **F-S2103-P2-003 CONFIRMED-FIXED (this burst):** ADR-031 leg confirmed pass-2; BC Traceability leg closed at 36d69804; story leg S-7.01(c) residual closed at 9b020a98.
- **Novelty:** MEDIUM (deletion-agnostic recovery gap + fetch/ancestry-check conflation are second-order consequences of pass-2 fixes; P3-001 is a missed-site in story spec).

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER  | 0 |
| HIGH     | 0 |
| MEDIUM   | 3 |
| LOW      | 0 |
| NITPICK  | 0 |
| OBS      | 4 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Readiness:** requires revision (fix burst dispatched; all findings FIXED prior to this persist)

**Fix burst dispatch:**
1. **story-writer** — F-S2103-P3-001 (§Previous Story Intelligence Step 8-post-A cite): FIXED at 9b020a98 (story v1.5; input-hash 56e51ee)
2. **product-owner** — F-S2103-P3-002 (BC-6.10.002 v1.4 deletion-agnostic headRefOid recovery): FIXED at 36d69804
3. **product-owner** — F-S2103-P3-003 (BC-6.10.002 v1.4 Step A/B split + EC-007 TrunkFetchFailed): FIXED at 36d69804
4. **implementer** — F-S2103-P3-002/003 pr-manager.sh mirror: FIXED at 96627798
5. **test-writer** — T-006 (TrunkFetchFailed EC-007 coverage): FIXED at 12282097; T-006 title corrected from AC-006 to EC-007 at 2a6fa036

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 3 |
| **New findings** | 3 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (3 / (3 + 0)) |
| **Median severity** | MEDIUM |
| **Trajectory** | 3→3→3 |
| **Verdict** | FINDINGS_REMAIN |
| **Novelty class** | MEDIUM — deletion-agnostic recovery gap + fetch/ancestry-check conflation are second-order consequences of pass-2 fixes; P3-001 is a missed citation site in story spec |
