# W-16 E-9 pass-3 adversarial review

Date: 2026-05-03
Adversary agentId: abcd7e11dd633dca5
Target: /Users/jmagady/Dev/vsdd-factory/.factory/stories/epics/E-9-tier-2-native-wasm-migration.md (v1.2, 567L)
Verdict: SUBSTANTIVE (1 MEDIUM fresh; 1 LOW fresh; pass-2 closures all genuinely closed)
Pass: 3 of N (per ADR-013, need 3 NITPICK_ONLY for convergence; clock currently 0_of_3)

---

## Pass 2 Closure Audit

| Pass-2 ID | Severity | Status (v1.2) | Evidence |
| --- | --- | --- | --- |
| F-1 PARTIAL (R-W16 alignment) | HIGH | CLOSED | E-9 v1.2 line 312-321 risk header explicitly adopts ADR-014 verbatim definitions for R-W16-001..004; R-W16-007 + R-W16-008 created append-only for E-9-redefined risks. R-W16-002 (line 325) text matches ADR-014 line 301-304; R-W16-004 (line 327) text matches ADR-014 line 305-307. POLICY 1 satisfied. |
| F-P2-001 (D-9.4 Exception clause) | HIGH | CLOSED | E-9 v1.2 D-9.4 (lines 288-301) contains no "Exception" clause. Replaced with: "All Tier 2 hooks reuse existing BC-7.xx anchors. The S-9.07 subprocess use case is covered by existing BC-1.05.001..034 plus the additive BC-1.05.035 + BC-1.05.036 (per ADR-014 Amendment 2026-05-03)." Verified line 293-294. |
| F-P2-002 (line 374 SS-02) | HIGH | CLOSED | E-9 v1.2 line 406 reads "host::run_subprocess section marked WITHDRAWN per ADR-014 Amendment 2026-05-03 (gap-analysis-w16-subprocess.md §7); no Schema Evolution entry required." Stale "Schema Evolution" reference removed. |
| F-P2-003 (BC-2.02.005 mis-anchor) | HIGH | CLOSED | All exec_subprocess context references replaced. Spot-check: line 39, 91, 173, 215, 222, 264 — all read "BC-1.05.001..034 (existing) + BC-1.05.035 + BC-1.05.036 (additive, ADR-014 Amendment 2026-05-03)". Only remaining BC-2.02.005 reference at line 524 (v1.2 changelog historical note) — correct preservation. |
| F-P2-004 (D-9.4 BC-2.02.013 attribution) | HIGH | CLOSED | "BC-2.02.013 (authored by PO in D-3)" clause removed from D-9.4. |
| F-P2-005 (Burst language line 99) | MEDIUM | CLOSED | Line 112-113 reads "Story-writer authors S-9.01..S-9.07 in subsequent bursts following adversarial convergence per ADR-013." |
| F-P2-006 (SS-07 anchor) | MEDIUM | CLOSED | Lines 71-79 strengthen SS-07 anchor with concrete artifacts: plugins/vsdd-factory/hooks/validate-*.sh (23 files), [[hooks]] registry entries, registry-edit-only scope. [process-gap] stretch-anchor disclosure block at lines 83-85. |
| F-P2-007 (vsdd-hook-sdk Library Table) | MEDIUM | CLOSED | Library Table lines 370-371 split: general row (S-9.01..S-9.06) + subprocess facet row (S-9.07). |
| F-P2-008 (OQ-3 cwd_allow) | MEDIUM | CLOSED | OQ-3 resolution (line 358) includes cwd_allow = []. Matches gap-analysis line 67 ExecSubprocessCaps schema. |
| F-P2-009 (Block-mode 5 of 23) | MEDIUM | CLOSED | Block-mode callout lines 175-184 lists 5 hooks; all 5 TOML line citations independently verified on-disk. AC-8 line 346 enumerates all 5. Independent on-disk audit performed across all 23 Tier 2 hooks; no 6th block-mode hook missed. |
| F-P2-010 (input-hash placeholder) | LOW | CLOSED | Frontmatter line 27 reads input-hash: "e3055a6". |
| F-P2-011 (S-9.30 row ordering) | LOW | CLOSED | Stories table lines 97-110 has S-9.30 row at end under "— Withdrawn —" subsection separator. |
| F-P2-012 (status: in-review) | LOW | CLOSED | Frontmatter line 6 reads status: in-review. |
| F-P2-013 (epic-level depends_on intent) | LOW | CLOSED | v1.2 changelog line 561-565 documents intent. |

Closure rate: 14 of 14 pass-2 findings genuinely closed (100%).

---

## Fresh Findings

| ID | Severity | Category | Location | Description | Suggested Fix |
| --- | --- | --- | --- | --- | --- |
| F-P3-001 | MEDIUM | duplication / risk-table redundancy | E-9 v1.2 lines 325 (R-W16-002) and 328 (R-W16-005) | R-W16-002 and R-W16-005 cover the same risk with overlapping language. R-W16-002: "WASI preopens: 19 of 23 hooks read FILE_PATH... path_allow declarations". R-W16-005: "WASI preopens / path_allow coverage detail: 19 of 23 hooks need path_allow = [.factory/]... runtime read_file denial". Both have same mitigation text. The pass-1 F-5 fix introduced R-W16-005 because at that time R-W16-002 was redefined to "Behavioral divergence". After v1.2 reverted R-W16-002 to ADR-014's "WASI preopens" definition, R-W16-005 became a duplicate — but was retained per POLICY 1 append-only. This is the residual cost of POLICY 1: dead-but-preserved IDs need an explicit superseded-by marker. | Mark R-W16-005 with status "merged-into-R-W16-002" or add a one-line note: "R-W16-005 originally distinguished from R-W16-002 when R-W16-002 carried the redefined Behavioral divergence content; after v1.2's verbatim-ADR-014 alignment, R-W16-005 is semantically identical to R-W16-002. Retained per POLICY 1; future references should cite R-W16-002 as the canonical ID." Alternative: edit R-W16-005 description to highlight a meaningful incremental detail (e.g., "registry-TOML-specific failure mode: silent fail vs incorrect block") so the two IDs are distinct in scope. |
| F-P3-002 | LOW | wording precision | E-9 v1.2 line 312-321 (Risk ID alignment note) | The note states "R-W16-001 through R-W16-006 use ADR-014's verbatim definitions" but ADR-014 only defines R-W16-001 through R-W16-004. R-W16-005 and R-W16-006 do NOT appear in ADR-014; the note then admits "R-W16-005 = path_allow coverage (additive per pass-1 F-5), R-W16-006 = Windows CI gap (additive per pass-1 F-5)." The "through R-W16-006" claim is contradicted by the same paragraph's next clause. | Replace "R-W16-001 through R-W16-006 use ADR-014's verbatim definitions" with "R-W16-001 through R-W16-004 use ADR-014's verbatim definitions; R-W16-005 and R-W16-006 are additive E-9-original IDs (per pass-1 F-5). E-9-specific risks that collided with ADR-014 IDs renumbered: Behavioral divergence → R-W16-007; YAML parsing fidelity → R-W16-008 (append-only per POLICY 1)." |

---

## Policy Rubric Audit

| Policy | Status | Evidence |
| --- | --- | --- |
| 1 append_only_numbering | PASS | S-9.30 retained as withdrawn entry. R-W16-007 + R-W16-008 are fresh IDs (verified — no prior assignment found). R-W16-002/004 reassignment to ADR-014 verbatim definitions does not constitute renumbering. POLICY 1 borderline-acceptable; v1.1 → v1.2 rebinding documented in changelog at line 501-506. |
| 2 lift_invariants_to_bcs | N/A | Epic-level doc; DI citations live on BCs not epics. |
| 3 state_manager_runs_last | PASS | input-hash e3055a6 recomputed per F-P2-010 closure. |
| 4 semantic_anchoring_integrity | PASS | All exec_subprocess BC anchors reference BC-1.05.001..034 + BC-1.05.035 + BC-1.05.036 (verified at 6 sites). BC-1.05.035 + BC-1.05.036 H1s match E-9's references. No remaining BC-2.02.005 references in exec_subprocess context. |
| 5 creators_justify_anchors | PASS | SS-04 anchor justification cites concrete artifact path; SS-07 partial-anchor justification lists specific .sh files + registry entries; [process-gap] stretch-anchor disclosure block present. |
| 6 architecture_subsystem_source_of_truth | PASS | SS-04 / SS-07 names match ARCH-INDEX usage. SS-02 correctly dropped from anchors per D-9.2 withdrawal. |
| 7 bc_h1_source_of_truth | PASS | BC-1.05.035 H1 + BC-1.05.036 H1 accurately cover the path traversal guard + success telemetry E-9 cites. BC-2.02.005 H1 confirmed = read_string protocol (NOT exec_subprocess). |
| 8 bc_array_changes_propagate | N/A | Epic doesn't have body BC table or AC-trace cells. |
| 9 vp_index_source_of_truth | N/A | No VP citations in E-9. |
| 10 demo_evidence_story_scoped | N/A | Epic spec, no demo evidence. |
| 11 no_test_tautologies | N/A | Spec authoring layer. |
| 12 bc_tv_emitter_consistency | N/A | Code/BC TV pairing; not epic-level. |

---

## Cross-Doc Verification

| Cross-Doc | Status | Evidence |
| --- | --- | --- |
| ADR-014 R-W16-001..006 verbatim alignment | PASS | Lines 282, 287 (Consequences); 301, 305 (Audit Risk Items). R-W16-001/003 in Consequences match E-9. R-W16-002/004 in Audit Risk Items match E-9 verbatim. R-W16-005/006 are E-9-original (additive). |
| ADR-014 R-8.09 Amendment cites R-8.08 (not R-8.10) | PASS | ADR-014 line 44 explicitly says "(inherited from S-8.00 / E-8 R-8.08; note: original amendment cited R-8.10 in error — E-8 v1.10 risk table is the source of truth)". |
| ADR-014 BC-1.05.035 + BC-1.05.036 references | PASS | ADR-014 lines 18, 26, 33-34 reference both BCs. Consistent with E-9. |
| BC-1.05.035 file exists with documented invariants | PASS | H1 + Postconditions + Invariants present; Story Anchor = S-9.07. |
| BC-1.05.036 file exists with documented invariants | PASS | H1 + Postconditions + Invariants present; Story Anchor = S-9.07. |
| BC-2.02.005 H1 = read_string protocol (NOT exec_subprocess) | PASS | BC-2.02.005 H1: "SDK-side read_string re-call protocol — host returns required size; SDK reallocates and re-calls". Confirms F-P2-003 fix correctness. |
| BC-INDEX registers BC-1.05.035 + BC-1.05.036 | PASS | BC-INDEX lines 120-121. SS-01 count 103 (101 active + 2 retired); BC-INDEX changelog line 17 documents the relocation. |
| E-8 v1.10 Tier 2/3 supersession marks intact | PASS | E-8 lines 633-634 show W-16/W-17 rows struck-through. R-8.08 = "Cumulative WASM startup overhead" with 500ms p95 ceiling at AC-7b confirmed (line 607, 622). |
| S-9.00 v1.2 references R-8.08 (not R-8.10) | PASS | S-9.00 lines 164, 185, 208 cite R-8.08. AC-7 explicitly notes ADR-014 typo correction. |
| hooks-registry.toml block-mode citation accuracy | PASS | All 5 cited line numbers verified on-disk: 231 (validate-factory-path-root), 291 (validate-input-hash), 471 (validate-template-compliance), 774 (validate-pr-merge-prerequisites), 794 (validate-wave-gate-prerequisite). Independent scan found NO 6th block hook. |
| OQ-3 cwd_allow field present | PASS | E-9 line 358 includes cwd_allow = []. Schema matches gap-analysis line 67 ExecSubprocessCaps. |

---

## Convergence Status

- Pass 3 verdict: SUBSTANTIVE (1 MEDIUM + 1 LOW; ADR-013 clock requires NITPICK_ONLY/CLEAN to advance)
- Closure rate: 14 of 14 pass-2 findings genuinely closed (100%); F-1 partial residual fully closed
- Fresh findings: 2 (0 HIGH, 1 MEDIUM, 1 LOW)
- ADR-013 clock: resets/holds at 0_of_3 on SUBSTANTIVE verdict
- Recommended next action:
  1. Story-writer fix burst on F-P3-001 (R-W16-005 vs R-W16-002 duplication — recommend "edit R-W16-005 description to highlight a distinct facet")
  2. Sweep F-P3-002 (wording precision in risk header) in same burst
  3. Recompute input-hash post-burst per POLICY 3
  4. Pass-4 (fresh-context discipline) — likely path to NITPICK_ONLY given v1.2's substantial closure rate

Convergence trajectory: v1.0 (18 pass-1 findings) → v1.1 (12 pass-2 findings, fix-burst regressions visible) → v1.2 (2 pass-3 findings, both originate from genuinely additive-ID housekeeping rather than fix-burst regressions). Trajectory 18 → 12 → 2 indicates strong content convergence with residual minor housekeeping. v1.2 is one fix burst away from NITPICK_ONLY.

Note on novelty: F-P3-001 is novel; the duplication only became visible after R-W16-002's pass-2 reversion to "WASI preopens". Pass-2 could not have flagged it because R-W16-005 was distinct under v1.1's R-W16-002="Behavioral divergence" definition. Fresh-context discipline surfaced it.
