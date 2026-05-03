# W-16 E-9 pass-4 adversarial review

Date: 2026-05-03
Adversary agentId: a9f4e486d919139dc
Target: .factory/stories/epics/E-9-tier-2-native-wasm-migration.md (v1.3, 598L)
Verdict: SUBSTANTIVE (1 HIGH cross-doc + 1 MEDIUM in-document + 1 LOW = 3 fresh findings)
Pass: 4 of N (clock holds at 0_of_3 on SUBSTANTIVE)

---

## Pass 3 Closure Audit

| Pass-3 ID | Severity | Status (v1.3) | Evidence |
| --- | --- | --- | --- |
| F-P3-001 (E-9 own: R-W16-005 vs R-W16-002 duplication) | MEDIUM | CLOSED | E-9 v1.3 line 329 R-W16-005 description rewritten to focus on failure-mode semantics distinct from R-W16-002. Cross-reference "See also R-W16-002 (canonical WASI preopens per-hook coverage requirement)" added. Two IDs are now semantically distinct. |
| F-P3-002 (E-9 own: Risk header wording) | LOW | CLOSED | E-9 v1.3 lines 312-321 risk header replaces self-contradictory "R-W16-001..006 use ADR-014's verbatim definitions" with "R-W16-001..004 use ADR-014's verbatim definitions; R-W16-005 and R-W16-006 are E-9-original additive IDs." |
| F-P3-001 (cross-doc S-9.00: Library Table line 373 du → wc) | HIGH | CLOSED | E-9 v1.3 line 374 reads "wc (POSIX) | system | Bundle-size measurement (wc -c < file); portable across macOS BSD and Linux GNU. NOT du -sb (GNU-only -b flag; macOS du uses -k for kibibytes)." |

Closure rate: 3 of 3 pass-3 findings genuinely closed (100%).

---

## Fresh Findings

| ID | Severity | Category | Location | Description | Suggested Fix |
| --- | --- | --- | --- | --- | --- |
| F-P4-001 | HIGH | content drift / cross-doc propagation gap [POLICY 4 semantic_anchoring_integrity] | STORY-INDEX.md line 282 | The S-9.07 row in the E-9 stories table reads: "(pending Burst 3; uses exec_subprocess/BC-2.02.005 for wave-gate-prerequisite; S-9.30 withdrawn)". This is the same BC-2.02.005 mis-anchor pattern that pass-2 F-P2-003 closed across 6 sites in E-9 v1.2. The fix did not propagate to STORY-INDEX. BC-2.02.005 is the SDK read_string re-call protocol — NOT exec_subprocess. The exec_subprocess BCs are BC-1.05.001..034 + BC-1.05.035 + BC-1.05.036 per ADR-014 Amendment 2026-05-03. | Update STORY-INDEX.md line 282 to: "(pending Burst 3; uses exec_subprocess/BC-1.05.001..034 + BC-1.05.035 + BC-1.05.036 for wave-gate-prerequisite; S-9.30 withdrawn)". Same propagation discipline as F-P2-003 v1.2 burst. |
| F-P4-002 | MEDIUM | self-contradiction within same section [POLICY 1 append-only consistency] | E-9 v1.3 lines 377-382 (Library Table v1.1 changelog note) | The v1.1 changelog note immediately below the v1.3 Library Table reads: "hyperfine row corrected to 'Latency benchmarking harness' (was incorrectly listed as 'Bundle-size measurement harness'; **du is the bundle-size tool**)." The trailing clause "du is the bundle-size tool" now directly contradicts the current line 374 (which says wc, NOT du). | Update the v1.1 changelog note: rewrite the parenthetical clause to acknowledge v1.3 supersession. Alternative: append a v1.3 line explicitly noting that the v1.1's "du" reference is superseded by v1.3's wc replacement. |
| F-P4-003 | LOW | wording precision (pending intent verification) | E-9 v1.3 line 318 (Risk header) | The phrase "distinct from R-W16-002's scope" in the v1.3 header note is correct but slightly imprecise — R-W16-005 covers a distinct failure-mode framing, not a different scope. Both share the same scope (WASI preopens / path_allow coverage); they differ in framing axis (per-hook coverage vs runtime fail-mode). | Consider rewording. This is a refinement; the current wording is adequate. (pending intent verification) |

---

## Policy Rubric Audit

| Policy | Status | Evidence |
| --- | --- | --- |
| 1 append_only_numbering | PASS | S-9.30 retained as withdrawn entry. R-W16-007 + R-W16-008 retained per pass-2 fix burst. v1.3 introduces no new IDs. |
| 2 lift_invariants_to_bcs | N/A | Epic-level doc. |
| 3 state_manager_runs_last | PASS | input-hash 3458e0a recomputed post-v1.3 fix burst. |
| 4 semantic_anchoring_integrity | FAIL | E-9 v1.3 itself: PASS — all 6 in-document exec_subprocess BC anchors reference BC-1.05.001..034 + BC-1.05.035 + BC-1.05.036. However, STORY-INDEX.md line 282 retains a stale BC-2.02.005 mis-anchor in S-9.07's Notes column. See F-P4-001. |
| 5 creators_justify_anchors | PASS | SS-04 anchor + SS-07 stretch-anchor disclosure block remain intact. |
| 6 architecture_subsystem_source_of_truth | PASS | SS-04 / SS-07 names match ARCH-INDEX. |
| 7 bc_h1_source_of_truth | PASS | BC-1.05.035 + BC-1.05.036 H1 descriptions match E-9's references. BC-2.02.005 H1 confirmed = read_string protocol. |
| 8 bc_array_changes_propagate | N/A | Epic doesn't have body BC table or AC-trace cells. |
| 9 vp_index_source_of_truth | N/A | No VP citations in E-9. |
| 10 demo_evidence_story_scoped | N/A | Epic spec, no demo evidence. |
| 11 no_test_tautologies | N/A | Spec authoring layer. |
| 12 bc_tv_emitter_consistency | N/A | Code/BC TV pairing; not epic-level. |

---

## Convergence Status

- Pass 4 verdict: SUBSTANTIVE (1 HIGH cross-doc + 1 MEDIUM in-document + 1 LOW)
- Closure rate: 3 of 3 pass-3 findings genuinely closed (100%)
- Fresh findings: 3 (1 HIGH cross-doc, 1 MEDIUM, 1 LOW pending intent verification)
- ADR-013 clock: holds at 0_of_3 on SUBSTANTIVE verdict (no advancement)
- Recommended next action:
  1. Story-writer fix burst on F-P4-001 (STORY-INDEX line 282 BC-2.02.005 → BC-1.05.001..034 + BC-1.05.035 + BC-1.05.036). Same propagation discipline as F-P2-003.
  2. Sweep F-P4-002 (Library Table v1.1 changelog note) in same burst.
  3. Pass-5 — strong path to NITPICK_ONLY since both fresh findings are propagation gaps, not novel content defects in E-9 v1.3.

Convergence trajectory: v1.0 (18) → v1.1 (12) → v1.2 (2) → v1.3 (3 pass-4; 1 cross-doc-to-STORY-INDEX) — E-9 epic body itself converged at v1.3; only ancillary text carries residual drift.

[process-gap] F-P2-003's 6-site fix did not include STORY-INDEX sweep. Discipline could codify "when fixing BC anchor drift in epic body, grep STORY-INDEX for parallel references."

Novelty assessment: F-P4-001 genuinely novel — pass-3 fresh-context did not include STORY-INDEX line 282 in cross-doc verification. F-P4-002 novel because v1.3 introduced the new contradiction. F-P4-003 is a refinement, not new defect.
