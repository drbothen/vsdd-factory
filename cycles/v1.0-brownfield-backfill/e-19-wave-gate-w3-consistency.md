---
document_type: consistency-audit
audit_scope: E-19 epic perimeter closure (COMPLETE 9/9)
develop_sha: 6db4c9fc
story_index_version: "4.218"
auditor: vsdd-factory:consistency-validator
timestamp: 2026-07-17
status: complete
---

# E-19 Wave Gate W3 — Consistency Audit Report

**Audit date:** 2026-07-17  
**Develop HEAD:** 6db4c9fc (PR #670 squash-merge, S-19.07, E-19 COMPLETE 9/9)  
**STORY-INDEX version:** v4.218  
**Verdict:** 1 HIGH, 2 MEDIUM, 3 LOW findings. No BLOCKERs. Perimeter is correctly bounded with two additions.

---

## Summary by Severity

| Severity | Count | Findings |
|----------|-------|---------|
| BLOCKER | 0 | — |
| HIGH | 1 | F-001 story frontmatter status stale for 8 of 9 S-19 stories |
| MEDIUM | 2 | F-002 merged_count 107 vs table count 114; F-003 E-19 epic status=draft post-completion |
| LOW | 3 | F-004 BC-2.02.011 status/lifecycle_status field mismatch; F-005 ARCH-INDEX ADR-025 description cell stale; F-006 VP files all status=draft |
| CLEAN | 9 | See §Clean Checks |

---

## Clean Checks

All of the following passed with no discrepancy:

1. **BC-INDEX version cells vs BC file frontmatter** — all 8 BCs in scope: BC-4.13.001 (v1.17 ✓), BC-1.17.001 (v1.7 ✓), BC-3.08.001 (v1.24 ✓), BC-5.40.001 (v1.3 ✓), BC-5.42.001 (v1.8 ✓), BC-2.07.001 (v1.6 ✓), BC-2.02.011 (v1.7 ✓), BC-7.03.079 (v1.5 ✓). BC-INDEX table version cells match BC file frontmatter versions on all 8.

2. **VP-INDEX version cells vs VP file frontmatter** — VP-094 (v1.5 ✓), VP-095 (v1.5 ✓), VP-096 (v1.2 ✓), VP-097 (v1.5 ✓), VP-098 (v1.2 ✓), VP-099 (v1.1 ✓), VP-100 (v1.2 ✓), VP-101 (v1.3 ✓). All 8 VP files match VP-INDEX table rows.

3. **ARCH-INDEX version cells vs ADR file frontmatter** — ADR-025 file v1.18 matches ARCH-INDEX last recorded v1.18 ✓. ADR-030 file v1.4 matches ARCH-INDEX last recorded v1.4 ✓.

4. **POL-14 promotion for E-19 BCs** — BC-5.42.001 promoted D-833 (S-19.01 PR #613) ✓; BC-2.07.001 promoted D-834 (S-19.03 PR #611) ✓; BC-1.17.001 promoted D-843 (S-19.06 PR #657) ✓; BC-4.13.001 PASS-ALREADY-ACTIVE D-851 ✓; BC-3.08.001 PASS-ALREADY-ACTIVE D-848 ✓; BC-5.40.001 PASS-ALREADY-ACTIVE D-842 ✓; BC-7.03.079 retroactive D-838 ✓.

5. **sprint-state.yaml parity** — All 9 S-19 stories (S-19.01..S-19.09) listed as `status: merged`. ✓

6. **BC-4.13.001 Phase-B vs code (develop 6db4c9fc)** — `crates/hook-plugins/verify-factory-lock/src/lib.rs` confirmed: `host::read_prefix` is the production read path (line 510, 337); `STATE_MD_MAX_BYTES` constant absent; `TooLarge`/`OutputTooLarge` handling absent. Matches BC-4.13.001 §Precondition 3 Phase-B exactly. ✓

7. **VP-095 harness test function claims vs lib.rs** — All 5 claimed test functions present: `test_S1907_T003_read_prefix_262144_large_fixture_foreign_lock_blocks` (line 1717), `test_S1907_T004_read_prefix_262144_large_fixture_no_lock_continues_without_warns` (line 1760), `test_S1907_EC001_read_prefix_262144_delimiter_at_boundary_blocks_on_foreign_lock` (line 1809), `test_S1907_FP1002_real_shape_35kb_frontmatter_foreign_lock_blocks` (line 1983), `test_S1907_FP1002_real_shape_35kb_frontmatter_no_lock_continues_without_warns` (line 2070). ✓

8. **ADR-025 D19 (read_prefix production-path registration) vs invoke.rs** — `crates/factory-dispatcher/src/invoke.rs` line 754 comment: "ADR-025 §Decision 16: production-path fill for vsdd::read_prefix"; line 761 registers `"read_prefix"`; line 778 calls `crate::host::read_prefix::prepare`. Registered within `setup_host_on_store_data`, the production dispatch path. ✓

9. **verification-architecture.md and verification-coverage-matrix.md versions** — verification-architecture.md v1.11 matches ARCH-INDEX POLICY 9 record (v1.8→v1.11 cumulative at v3.04) ✓. verification-coverage-matrix.md v1.8 matches ARCH-INDEX POLICY 9 record ✓.

---

## Findings

### F-001 (HIGH) — Story frontmatter `status` not updated to `merged` for 8 of 9 S-19 stories

**Surface:** Story file frontmatter — `status:` field  
**Route to:** state-manager

**Detail:** Post-PR-merge, the story frontmatter `status` field should advance to `merged` (the S-19.07 precedent: D-851 state-manager burst explicitly sets `status: merged`, version 1.26→1.27). Only S-19.07 received this update. The other 8 merged stories retain their pre-merge convergence status:

| Story | fm_status | fm_version | PR | Merged at |
|-------|-----------|------------|----|-----------|
| S-19.01 | **draft** | 1.18 | #613 8d1721f7 | 2026-07-13T14:49:50Z |
| S-19.02 | **draft** | 1.18 | #610 f5ea12e9 | 2026-07-13T14:33:57Z |
| S-19.03 | **draft** | 1.20 | #611 091ce499 | 2026-07-13T15:54:21Z |
| S-19.04 | **draft** | 1.21 | #639 d4a23a02 | 2026-07-14T14:39:00Z |
| S-19.05 | **draft** | 1.22 | #640 7b35c8e4 | 2026-07-14T14:39:33Z |
| S-19.06 | **ready** | 1.22 | #657 9787c056 | 2026-07-15T14:53:16Z |
| S-19.07 | merged ✓ | 1.27 | #670 6db4c9fc | 2026-07-17T22:25:06Z |
| S-19.08 | **ready** | 1.3 | (D-842) | 2026-07-14 |
| S-19.09 | **ready** | 1.6 | #659 13ece92c | 2026-07-16T04:01:30Z |

The STORY-INDEX table and sprint-state.yaml both correctly show all 9 as `merged`. The gap is specifically in the story file frontmatter. S-19.01..S-19.05 were still in `draft` at their W1 merge (POL-14 ran, BC promotion recorded, but story file not updated). S-19.06/S-19.08/S-19.09 reached `ready` via local adversarial convergence but the post-merge `ready→merged` advancement was not applied.

**Proposed fix:** State-manager to run post-merge status burst for S-19.01..S-19.06, S-19.08, S-19.09: bump story frontmatter `status: draft/ready → merged`, version increment, `last_amended` record per D-851 pattern. STORY-INDEX table rows already show `merged`; no table change needed.

---

### F-002 (MEDIUM) — merged_count 107 claimed but STORY-INDEX table contains 114 `merged` rows

**Surface:** STATE.md + STORY-INDEX last_amended narrative vs STORY-INDEX table  
**Route to:** state-manager

**Detail:** The last_amended at v4.218 records "merged_count 106→107" (and prior increments follow similar pattern). A direct count of STORY-INDEX table rows with `| merged |` or `| **merged** |` status yields 114 rows. The sprint-state.yaml has 132 `status: merged` entries. Three different counts for "merged":

| Source | Count |
|--------|-------|
| STORY-INDEX last_amended (merged_count) | 107 |
| STORY-INDEX table rows with merged status | 114 |
| sprint-state.yaml `status: merged` entries | 132 |

The delta of 7 between 107 and 114 has no documented explanation. Possible causes: (a) merged_count began tracking at a later baseline, excluding 7 early merged stories; (b) some merged table rows are in a category excluded from the counter (e.g., pre-VSDD E-0 legacy stories, retired/superseded rows); (c) the counter was incremented incorrectly at some past merge. The merged_count definition is embedded only in the narrative, not as a structured field.

**Proposed fix:** State-manager to document the merged_count definition (which story set it covers) and reconcile the 7-story discrepancy. If the discrepancy is due to intentional exclusion of a category, add an explanatory note to the STORY-INDEX frontmatter.

---

### F-003 (MEDIUM) — E-19 epic file status=draft after E-19 COMPLETE 9/9

**Surface:** `.factory/stories/epics/E-19-post-rc22-operator-hardening.md` frontmatter  
**Route to:** state-manager

**Detail:** The E-19 epic file has `status: draft`, `version: "v1.30"`, `story_count: 9`. Last_amended is v1.30 from 2026-07-15 when S-19.09 was added. The D-851 post-merge burst that declared E-19 COMPLETE 9/9 (STATE.md, STORY-INDEX) did not update the epic file. The epic file has no `completion_date`, `epic_status: complete`, or any closure record.

**Proposed fix:** State-manager to advance E-19 epic file: `status: complete`, add `completion_date: 2026-07-17`, version v1.31, last_amended recording D-851 closure.

---

### F-004 (LOW) — BC-2.02.011 `status: ready` while `lifecycle_status: active` 

**Surface:** `.factory/specs/behavioral-contracts/ss-02/BC-2.02.011.md` frontmatter  
**Route to:** state-manager

**Detail:** BC-2.02.011 frontmatter shows:
- `status: ready` (spec convergence state — not updated on POL-14 promotion)
- `lifecycle_status: active` (POL-14 promotion recorded correctly)

All other BCs in the audit set that have been POL-14 promoted show `status: active` and `lifecycle_status: active` in parity. BC-2.02.011's `status` field was not advanced from `ready` to `active` when POL-14 ran (at S-8.10's merge). The BC-INDEX table shows `active` in the Status column (using `lifecycle_status`), which is correct. But the BC file itself is inconsistent internally.

Additionally, no version cell entry in BC-INDEX records the POL-14 promotion event for BC-2.02.011 (the visible version history goes v1.3→v1.4→v1.5→v1.6→v1.7 with no POL-14 row). If the promotion happened at v1.1 or v1.2 (before BC-INDEX version recording began), a historical note should confirm this.

**Proposed fix:** State-manager to set BC-2.02.011 `status: active` to match `lifecycle_status: active`; add a note in last_amended recording when POL-14 was applied (or confirm the version it occurred at).

---

### F-005 (LOW) — ARCH-INDEX ADR-025 description cell opening paragraph stale after Phase-B

**Surface:** `.factory/specs/architecture/ARCH-INDEX.md` line 537 (ADR-025 table row description column)  
**Route to:** architect

**Detail:** The ARCH-INDEX table row for ADR-025 opens with the Phase-A design description: "uses host::read_file + exec_subprocess binary_allow=['git']". After S-19.07 (Phase-B migration, develop 6db4c9fc), the production guard uses `host::read_prefix` not `host::read_file`. The version changelog embedded in the same cell accurately captures the migration via AMENDED v1.16 (Decision 16 re: production-path gap) and v1.18, but the opening sentence of the description field was not updated.

Convention check required: if ARCH-INDEX description columns are immutable historical summaries (first-publication form), this is intentional and not a defect — the AMENDMENTs trail is the canonical update mechanism. If the description is intended to reflect current state, it needs "uses host::read_prefix" replacing "uses host::read_file" in the opening.

**Proposed fix:** Architect to clarify convention and, if the description is a current-state field, update "uses host::read_file" → "uses host::read_prefix" in the ADR-025 description cell.

---

### F-006 (LOW) — VP-094..VP-101 all have `status: draft`

**Surface:** All 8 VP files in `.factory/specs/verification-properties/`  
**Route to:** spec-steward

**Detail:** All 8 VP files (VP-094 through VP-101) have `status: draft` in frontmatter, confirmed at versions v1.5, v1.5, v1.2, v1.5, v1.2, v1.1, v1.2, v1.3 respectively. These VPs were authored during E-19 (VP-094 per S-19.01, VP-095/096 per S-19.02, etc.) and have not been advanced to any `accepted` or `active` lifecycle state after the stories merged.

This may be by convention — VP files may not follow the same POL-14 promotion lifecycle as BC files. The VP-INDEX itself does not show a lifecycle status column analogous to BC-INDEX's Status column. No evidence of prior VP files being advanced from `draft` was found in scope.

**Proposed fix:** Spec-steward to confirm whether VP files have a lifecycle promotion convention and, if so, advance VP-094..VP-101 accordingly. If VP files are expected to remain `draft` perpetually, document this convention explicitly in VP-INDEX.

---

## Perimeter Additions (artifacts touched during W3 not in original audit scope)

**P-001 — E-19 epic file** `.factory/stories/epics/E-19-post-rc22-operator-hardening.md`:  
In scope of F-003 above. Status=draft post-completion. Route to state-manager.

**P-002 — design-brief-post-e19-host-abi-fixes.md** `.factory/plans/design-brief-post-e19-host-abi-fixes.md`:  
Created during S-19.09 planning (referenced in ADR-025 v1.16 as F-WG-002+F-WG-003 routing target). It is a design input artifact, not an index artifact. No consistency finding; exists and is complete. No action required.

**P-003 — verification-architecture.md and verification-coverage-matrix.md** (in `.factory/specs/architecture/`):  
Touched by POLICY 9 propagation during S-19.07 adversarial cycle (v1.8→v1.11 and v1.5→v1.8 respectively). Both verified consistent with ARCH-INDEX records. ✓ No finding.

---

## Finding Count by Routing

| Route to | Severity | Finding |
|----------|----------|---------|
| state-manager | HIGH | F-001 (8 story frontmatter status fields) |
| state-manager | MEDIUM | F-002 (merged_count 107 vs 114) |
| state-manager | MEDIUM | F-003 (E-19 epic status=draft) |
| state-manager | LOW | F-004 (BC-2.02.011 status/lifecycle mismatch) |
| architect | LOW | F-005 (ARCH-INDEX ADR-025 description stale) |
| spec-steward | LOW | F-006 (VP lifecycle convention) |
