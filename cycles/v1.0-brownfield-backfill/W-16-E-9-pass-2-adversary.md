# W-16 E-9 pass-2 adversarial review

Date: 2026-05-03
Adversary agentId: ad6078c99b7a1477f
Target: .factory/stories/epics/E-9-tier-2-native-wasm-migration.md (v1.1, ~462L)
Verdict: SUBSTANTIVE (fix-burst-introduced regressions found)
Pass: 2 of N (per ADR-013, need 3 NITPICK_ONLY for convergence; clock resets to 0_of_3)

---

## Pass 1 Closure Audit

| Pass-1 ID | Severity | Status (v1.1) | Evidence |
| --- | --- | --- | --- |
| F-1 | HIGH | PARTIAL | E-9 v1.1 risk header (line 288–291) claims "R-W16-001 through R-W16-005 match ADR-014's definitions" — but ADR-014 §"Audit Risk Items Carried Forward" (lines 299–307) defines `R-W16-002 = WASI preopens` and `R-W16-004 = bats/WASM test infrastructure`, while E-9 v1.1 line 295/298 defines `R-W16-002 = Behavioral divergence` and `R-W16-004 = YAML parsing fidelity`. Direct semantic collision still exists. |
| F-2 | HIGH | CLOSED | E-9 v1.1 lines 44–48 add the supersession note. E-8 v1.10 D-13 (E-8 lines 633–634) has W-16/W-17 rows struck-through. Verified. |
| F-3 | HIGH | PARTIAL | E-9 v1.1 line 341 added `du` row (good). However, line 342 still lists `hyperfine` even though the v1.1 changelog (line 422) said hyperfine was "corrected to Latency benchmarking harness". The hyperfine retention itself is fine; the related concern surfaced as F-P2-007 (vsdd-hook-sdk Purpose column). Closed. |
| F-4 | HIGH | CLOSED | AC-6 (line 313) now reads `grep -n 'pub const HOST_ABI_VERSION: u32 = 1' crates/hook-sdk/src/lib.rs returns exactly one match`. No line-number anchor. Verified. |
| F-5 | HIGH | CLOSED | R-W16-005 added (E-9 line 299) covering WASI preopens / path_allow. Both R-W16-005 and R-W16-006 (Windows CI reserved) present. |
| F-6..F-13 | MEDIUM | MIXED | Most closed; F-13 partially open (see F-P2-002 below). |
| F-14..F-18 | LOW | CLOSED | Verified by inspection. |

---

## Fresh Findings

| ID | Severity | Category | Location | Description | Suggested Fix |
| --- | --- | --- | --- | --- | --- |
| F-P2-001 | HIGH | cross-doc / fix-burst regression | E-9 v1.1 lines 237–251 (D-9.2 section) + line 270 (D-9.4) | D-9.4 contradicts the D-9.2 amendment. E-9 D-9.4 says: "Exception: `host::run_subprocess` requires BC-2.02.013 (authored by PO in D-3). … If a port reveals unspecified behavior, a new BC is drafted under the existing BC-7.xx sub-family for the relevant hook (not a new BC-7.02.x migration family)." This describes BC-2.02.013/run_subprocess as still active. But D-9.2 (lines 237–251) says BC-2.02.013 is withdrawn. The fix burst rewrote D-9.2 but failed to update D-9.4's "Exception" clause that still treats run_subprocess as required. Implementer reading D-9.4 will think run_subprocess is still in scope. | Remove the "Exception: `host::run_subprocess` requires BC-2.02.013" sentence from D-9.4. Replace with: "All Tier 2 hooks reuse existing BC-7.xx anchors. The S-9.07 subprocess use case is covered by existing BC-1.05.001..034 plus the additive BC-1.05.035 + BC-1.05.036 (per ADR-014 Amendment 2026-05-03)." |
| F-P2-002 | HIGH | cross-doc / fix-burst regression | E-9 v1.1 line 374 (Architecture section files) | Stale architecture reference contradicts D-9.2 withdrawal. Line 374 says: "`architecture/SS-02-hook-sdk.md` — host::run_subprocess entry in Schema Evolution table (after S-9.30 merges)". S-9.30 will never merge (withdrawn). The Architecture Mapping callout at line 366–370 correctly notes SS-02 components removed, but line 374 still refers to host::run_subprocess as a future Schema Evolution entry. Direct internal contradiction. | Replace line 374 with: "`architecture/SS-02-hook-sdk.md` — host::run_subprocess section marked WITHDRAWN per ADR-014 Amendment 2026-05-03 (gap-analysis-w16-subprocess.md §7)" or remove the line entirely. |
| F-P2-003 | HIGH | cross-doc / semantic anchoring | E-9 v1.1 line 38 + lines 250–251 + Goal #6 (line 198–200) | Stale BC-2.02.005 reference for exec_subprocess after gap-analysis correction. E-9 v1.1 line 38: "`validate-wave-gate-prerequisite` invokes `verify-sha-currency.sh` via the existing `host::exec_subprocess` ABI (BC-2.02.005)". Same in line 159 ("BC-2.02.005; S-9.30 withdrawn") and Goal #4 (line 194: "existing host::exec_subprocess / BC-2.02.005 used"). However, ADR-014's "Correction 2026-05-03" (lines 30–34) and gap-analysis "Correction 2026-05-03" (lines 13–19) explicitly state BC-2.02.005 actually documents the SDK `read_string` re-call protocol — the exec_subprocess BCs live in SS-01 cluster (BC-1.05.001..034) plus the new BC-1.05.035 + BC-1.05.036. E-9 v1.1 propagated none of this correction. Implementer reading E-9 will read BC-2.02.005 (read_string protocol) instead of BC-1.05.001..034 + BC-1.05.035/036. POLICY 4 (semantic_anchoring_integrity) and POLICY 7 (BC H1 source-of-truth) violation. | Replace every E-9 v1.1 reference to "BC-2.02.005" with "BC-1.05.001..034 (existing) + BC-1.05.035 + BC-1.05.036 (additive, ADR-014 Amendment 2026-05-03)". Specifically: line 38, line 159, AC-5 commentary, Goals #4 and #6 (lines 194, 198–200), D-9.2 prose at line 248. |
| F-P2-004 | HIGH | semantic anchoring / process-gap | E-9 v1.1 line 270 (D-9.4) | D-9.4 misattributes BC-2.02.013 authorship. "Exception: `host::run_subprocess` requires BC-2.02.013 (authored by PO in D-3)." But D-3 is referenced nowhere else in E-9 — this appears to reference an E-8 D-3, not an E-9 decision. Combined with F-P2-001, this is a doubly stale callout. | Drop entire "Exception" line (it is invalidated by D-9.2 withdrawal). |
| F-P2-005 | MEDIUM | arithmetic drift | E-9 v1.1 lines 41 + 86–96 + 99 | "7 capability-cluster batches" arithmetic still cited but story count is 8. Line 41 says: "Delivered in 7 capability-cluster batches (S-9.01..S-9.07) plus one perf baseline story (S-9.00)." That is 7+1=8 — internally consistent. But line 99 says "S-9.01..S-9.04 are Burst 2; S-9.05..S-9.07 are Burst 3" while line 454 (changelog) says "burst-vs-wave language removed" — fix-burst is incomplete. Burst language still present at line 99. | Replace line 98–99 with: "Story-writer authors S-9.01..S-9.07 in subsequent bursts following adversarial convergence per ADR-013." |
| F-P2-006 | MEDIUM | inconsistency / fix-burst regression | E-9 v1.1 line 71 (SS-04 partial anchor) + line 113 (frontmatter `subsystems_affected: [SS-04, SS-07]`) | SS-07 anchor justification missing despite frontmatter declaration. The "Capability Anchor Justification" section describes SS-04 anchor and "SS-07 (Hook Bash Layer) partial anchor", but the body says SS-07 owns the .sh files being ported and "E-9 does NOT delete SS-07 artifacts." If the only SS-07 work is registry TOML edits + .sh file disabling (no code), the SS-07 anchor is borderline-mechanical. | Strengthen SS-07 anchor with concrete artifacts (which `.sh` registry entries get touched, where), or apply [process-gap] stretch-anchor disclosure in the SS-07 paragraph. |
| F-P2-007 | MEDIUM | cross-doc / fix-burst regression | E-9 v1.1 line 339 + line 86 (Stories table) | `vsdd-hook-sdk` library row references `exec_subprocess` ABI in Purpose, but `0.2.0 (post-S-8.10)` pin is incorrect for E-9. Line 339 says "vsdd-hook-sdk | 0.2.0 (post-S-8.10) | Plugin ABI (read_file, emit_event, log, exec_subprocess) | S-9.01". Per the hook inventory only S-9.07 uses subprocess. Misleading "First Story = S-9.01" mapping. | Split the row: list `exec_subprocess` for First Story = S-9.07 (only consumer); other shim functions for First Story = S-9.01. |
| F-P2-008 | MEDIUM | semantic / fix-burst regression | E-9 v1.1 lines 326–327 (OQ-3) | OQ-3 references "exec_subprocess" capability with `binary_allow = ["bash"]` plus `env_allow = ["PATH"]` — but the actual `ExecSubprocessCaps` struct (per gap-analysis line 62–69) requires field name `binary_allow` (correct), `env_allow` (correct), AND `cwd_allow: Vec<String>`. OQ-3 omits `cwd_allow` from the registry block. | Add `cwd_allow = []` (empty allow-list, since validate-wave-gate-prerequisite uses `$SHA_PROJECT_ROOT` flag, not cwd) to the OQ-3 example registry snippet, OR cite gap-analysis §7 step 5 verbatim. |
| F-P2-009 | MEDIUM | semantic / contradiction | E-9 v1.1 line 161 (Block-mode callout) + Stories table inventory | Block-mode callout cites only 3 hooks but `validate-wave-gate-prerequisite` is a PreToolUse:Agent gate that must hard-block a wave-gate violation. Lines 161–164: "3 of 23 Tier 2 validators use `on_error = "block"`: validate-factory-path-root, validate-input-hash, and validate-template-compliance." But validate-wave-gate-prerequisite (line 159) is `PreToolUse:Agent` with column `Block-mode | no` — yet the original bash `validate-wave-gate-prerequisite.sh` exits non-zero on prerequisite failure (per audit-w16.md line 54 + gap-analysis §4 line 152), which under PreToolUse semantics blocks Agent invocation. If `Block-mode = no` is the registered behavior, this is a behavior-change risk vs. bash. Also `validate-pr-merge-prerequisites` (line 127) is PreToolUse:Agent with Block-mode "no" — same concern. | Audit the actual `on_error` field in `hooks-registry.toml` for the existing 23 hooks; correct the inventory `Block-mode` column accordingly. If PreToolUse:Agent uses block by default per Claude Code semantics, document that explicitly. |
| F-P2-010 | LOW | editorial | E-9 v1.1 line 27 | `input-hash: "[pending-recompute]"` — frontmatter still has placeholder hash after v1.1 fix burst. POLICY 3 (state-manager-runs-last) demands canonical input-hash freshness. | Recompute SHA after final convergence per state-manager handoff. |
| F-P2-011 | LOW | editorial | E-9 v1.1 line 87 | Stories table includes S-9.30 (~~struck-through~~) AFTER S-9.00 and BEFORE S-9.01. Per POLICY 1, retired entries stay in indexes — correct. But ordering jarring (S-9.30 between S-9.00 and S-9.01 violates monotonic ID listing). | Move the withdrawn S-9.30 row to the END of the Stories table with a clear "Withdrawn" subsection header. |
| F-P2-012 | LOW | editorial | E-9 v1.1 line 12 + STORY-INDEX line 87 | Pass-1 F-14 LOW noted `status:` should flip to `in-review` while convergence is underway — v1.1 changelog claims F-14 closed but the `status:` field is unchanged. | Either flip `status: in-review` per pass-1 F-14 suggestion, or remove the F-14 closure claim from changelog. |
| F-P2-013 | LOW | (pending intent verification) | E-9 v1.1 frontmatter line 19 (`depends_on: ["E-8"]`) | E-9 declares `depends_on: ["E-8"]` at the epic level. S-9.00 frontmatter also declares `depends_on: ["E-8"]`. Sibling-mirror — not necessarily wrong. | Verify intent: keep both, or scope the epic-level dep at the epic and let stories inherit. |

---

## Policy Rubric Audit

| Policy | Status | Evidence |
| --- | --- | --- |
| 1 append_only_numbering | PASS | S-9.30 retained in Stories table with strikethrough + "withdrawn" status. POLICY 1 satisfied. |
| 2 lift_invariants_to_bcs | N/A | Epic-level doc; DI citations live on BCs not epics. |
| 3 state_manager_runs_last | FLAG (LOW) | input-hash placeholder unresolved (F-P2-010). |
| 4 semantic_anchoring_integrity | FLAG (HIGH) | F-P2-003: BC-2.02.005 mis-anchor for exec_subprocess. F-P2-001/002/004: stale run_subprocess/BC-2.02.013 anchors after withdrawal. |
| 5 creators_justify_anchors | FLAG (MEDIUM) | F-P2-006: SS-07 anchor justification thin. |
| 6 architecture_subsystem_source_of_truth | PASS | SS-04 / SS-07 names match ARCH-INDEX usage in E-8 v1.10 and E-9. SS-02 correctly dropped. |
| 7 bc_h1_source_of_truth | FLAG (HIGH) | F-P2-003: BC-2.02.005 H1 (per BC-INDEX) is read_string SDK protocol, not exec_subprocess. E-9 v1.1 cites BC-2.02.005 as if it documents exec_subprocess invariants — direct H1 mis-anchor. |
| 8 bc_array_changes_propagate | N/A | Epic doesn't have body BC table or AC-trace cells; story-level concern. |
| 9 vp_index_source_of_truth | N/A | No VP citations in E-9. |
| 10 demo_evidence_story_scoped | N/A | Epic spec, no demo evidence. |
| 11 no_test_tautologies | N/A | Spec authoring layer. |
| 12 bc_tv_emitter_consistency | N/A | Code/BC TV pairing; not epic-level. |

---

## Convergence Status

- Pass 2 verdict: SUBSTANTIVE (3 HIGH + 5 MEDIUM + 4 LOW = 12 fresh findings; pass-1 F-1 PARTIAL not fully closed)
- Closure rate: 16 of 18 pass-1 findings genuinely closed; F-1 partial (R-W16-002 / R-W16-004 semantic collision with ADR-014); F-3 nominal closed but library-table ergonomics (F-P2-007) re-emerged.
- Fresh findings: 12 (3 HIGH, 5 MEDIUM, 4 LOW)
- ADR-013 clock: resets to 0_of_3 on SUBSTANTIVE
- Recommended next action: Fix burst then pass-3. Priority order:
  1. F-P2-003 (BC-2.02.005 mis-anchor — propagate ADR-014/gap-analysis Correction 2026-05-03 verbatim into E-9 prose)
  2. F-P2-001 + F-P2-004 (D-9.4 Exception clause invalidated by D-9.2 withdrawal — drop Exception)
  3. F-P2-002 (Architecture section files line 374 — host::run_subprocess SS-02 evolution stale)
  4. F-1 partial (rename E-9 risks to use a non-colliding ID space, OR adopt ADR-014 R-W16-002/004 definitions verbatim and re-letter E-9's redefined risks)
  5. F-P2-009 (validate-wave-gate-prerequisite Block-mode column — verify TOML on disk and align)
  6. F-P2-005..008 + LOWs in subsequent fix sweep

Notes for orchestrator:
- The fix-burst from v1.0 → v1.1 closed all 5 HIGH pass-1 findings AT THE LOCATION FLAGGED but failed to propagate the corrections to sibling references elsewhere in the same document (F-P2-001/002/003/004). This is the canonical "Partial-Fix Regression Discipline" (S-7.01) gap.
- The BC-2.02.005 mis-anchor (F-P2-003) is the most consequential because it propagates from E-9 → S-9.07 (when authored).
