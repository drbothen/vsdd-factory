---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-07T00:00:00Z
phase: 6
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
cycle: v1.0-brownfield-backfill
cascade: E-19-story
pass: 6
previous_review: adv-E19-pass-5.md
perimeter: E-19 epic + S-19.01..S-19.07 + STORY-INDEX
verdict: NOT-CLEAN
blocker_count: 0
high_count: 5
medium_count: 2
low_count: 1
observation_count: 3
streak: 0/3
parent_decision: D-757
---

# Adversarial Review — E-19 Pass 6 (NOT-CLEAN)

**Perimeter:** E-19 epic + S-19.01..S-19.07 + STORY-INDEX E-19 section
**Reviewer:** fresh-context adversary (zero prior context per Iron Law; rubric = policies.yaml read directly; 20 policies per O-P5-001 correction — rubric drift closed)
**Date:** 2026-07-07
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 5 / MEDIUM 2 / LOW 1 (stated by adversary)
**Streak:** 0/3

> **ORCHESTRATOR ADJUDICATION NOTE (D-757):** The adversary's stated verdict line reads B0/H5/M2/L1 (8 total). However, Part A enumerates exactly 5 findings, all HIGH (F-P6-001..F-P6-005). The M2/L1 (3 additional items) in the stated verdict summary have no corresponding enumerated findings in Part B — this is an internal count inconsistency in the adversary's own verdict summary. The 5 enumerated HIGH findings are the actionable set; they are the basis for routing, the fix burst, and the trajectory count. The M2/L1 are not retracted — they simply have no artifact trace and cannot be actioned. Trajectory count for this pass = 5 (enumerated findings only).

---

## Finding ID Convention

This review uses the in-cycle finding ID format established for the `v1.0-brownfield-backfill` cascade:

`F-P<PASS>-<SEQ>` — e.g., `F-P6-001`, `F-P6-002`, etc. Severity is stated inline in the finding header. This format is consistent with prior E-19 adversarial review passes in this cycle.

---

## Part A — Fix Verification (pass >= 2 only)

Pass-5 NOT-CLEAN B0/H3/M4/L1 (8 findings + 6 observations); closed same-burst (D-756; 3 specialist legs). Fresh-context adversary reads only prior Part A — findings F-P5-001..F-P5-008. All 8 findings verified CLOSED by artifact evidence at pass-6 perimeter entry:

- **F-P5-001 CLOSED** (STORY-INDEX S-19.07 Priority cell P1→P2 corrected in v4.136 story-writer leg D-756; cell now reads P2 matching story frontmatter and epic table).
- **F-P5-002 CLOSED** (S-19.04 v1.5→v1.6 narrative corrected to actual release.yml state: REMOVE hello-hook build+copy steps; PRESERVE existing underscore case-arm exclusions; D-756 story-writer leg).
- **F-P5-003 CLOSED** (BC-3.08.001 v1.17→v1.18 Event 6 `plugin.completed` async path catalogued D-756 PO leg; S-19.05 v1.4→v1.5 AC-001 Event 3→Event 6 cite corrected; VP-079 v1.18→v1.19 six-event scope D-756 architect leg).
- **F-P5-004 CLOSED** (S-19.07 v1.0→v1.1 Gate B narrowed to non-comment semantic call-site scope; D-756 story-writer leg).
- **F-P5-005 CLOSED** (S-19.04 v1.5→v1.6 AC-001 internal contradiction resolved; gate now asserts ABSENCE of hello-hook build+copy steps; D-756 story-writer leg).
- **F-P5-006 CLOSED** (S-19.04 v1.5→v1.6 removal path documented; 3 candidate release.yml sites enumerated with preserve/remove designation; D-756 story-writer leg).
- **F-P5-007 CLOSED** (S-19.02 v1.4→v1.5 stale point-in-time size-figure cite replaced with drift-tolerant range-based rationale; D-756 story-writer leg).
- **F-P5-008 CLOSED** (S-19.03 v1.5→v1.6 stub constant NOT_FOUND changed from 0 to -1000; out-of-band non-zero; Red Gate failure guaranteed; D-756 story-writer leg).

New findings from pass-6 review below in Part B.

---

## Part B — New Findings

*Verbatim adversary output per D-448(a) source-attestation. Do not summarize or soften. Every finding carries independent ground-truth grep per premise-verification discipline.*

F-P6-001 — HIGH — [process-gap] S-19.04 cites POLICY 17 as the governance anchor for the bundle-hygiene policy ("release bundle MUST NOT include dev-sample artifacts"), but the registry ground truth is POLICY 20 (`release_bundle_no_dev_samples`). Ground-truth verification: `grep -n "POLICY\|policy\|release_bundle" .factory/stories/S-19.04-bundle-hygiene-tool-filter-anchoring.md | head -20` returns `POLICY 17` cite in the AC-001 governance anchor and §Background section. `grep -n "id:\|name:" .factory/policies.yaml | grep -A1 "17\|18\|19\|20"` returns: id 17 = `nn_n_frontmatter_parity`; id 18 = `oo_input_hash_mechanical_verification`; id 19 = `adr_version_cite_volatile_pin_prohibition`; id 20 = `release_bundle_no_dev_samples`. POLICY 17 is `nn_n_frontmatter_parity` — an entirely unrelated policy governing frontmatter field parity. **ROOT-CAUSE ATTRIBUTION (recorded honestly per L-BB-finding-premise-must-be-verified-before-fix binding scope):** The wrong ID originated in the ORCHESTRATOR's D-753 dispatch brief, which instructed "register as POLICY 17." At the time of that brief, the orchestrator operated under the belief that 16 policies existed (pre-D-753 state); the registry had already grown to 19 policies through earlier bursts in this cycle (policies 17/18/19 registered). State-manager correctly executed at the next free id = 20 (no-collision verified). The story's POLICY cite was never reconciled back to the actual registered id. This is the same class as `L-BB-finding-premise-must-be-verified-before-fix` — that lesson binds the orchestrator as much as it binds specialist agents; premise-verification before codifying any id or anchor into a dispatch brief is mandatory. Fix: story-writer POLICY 17→20 sweep in S-19.04 (all AC cells, §Background, §Traceability; literal grep to confirm 0 live POLICY-17 residuals in S-19.04 scope).

F-P6-002 — HIGH — [process-gap] S-19.04 v1.6 narrative inverts the actual control-flow semantics of the release.yml case arm for the underscore-pair pattern. Ground-truth control-flow trace: in the release.yml artifact-staging loop, a `case "$artifact_name" in ... *_*) ;; ... esac` arm where the body is empty (`;;`) is a PASS-THROUGH arm: the case branch terminates immediately with `;;`, and execution continues after the `esac` to the `cp` artifact-copy command — the file IS COPIED into the bundle (INCLUDED). This is consistent with the rc.22 post-install smoke evidence: both underscore-WASM files shipped in the rc.22 bundle (verified at a04cb303; byte counts 341,975 B and 342,292 B). The pass-5 adversary's Part B F-P5-002 finding text stated the case-arm "ALREADY excludes" the underscore WASMs — this was factually incorrect. The orchestrator's D-756 fix-burst brief propagated the same misread ("PRESERVE existing case-arm exclusions"). S-19.04 v1.6 was written on this incorrect foundation and now instructs the implementer to "preserve existing underscore case-arm exclusions" — but the existing arm is NOT an exclusion, it is a pass-through that includes those files. The pass-6 control-flow trace (this analysis) is definitive. Fix: S-19.04 v1.6→v1.7 narrative must be corrected to pass-through ground truth: the underscore-pair arm currently includes those files by doing nothing; the fix is to move that arm from pass-through to an explicit skip/continue path (change `) ;;` to `) continue ;;` or equivalent shell-portable construct that prevents the cp from executing for those files). The O-P6-002 orchestrator adjudication below provides byte-verified evidence that both files exist as distinct artifacts and confirms the two-file premise stands.

F-P6-003 — HIGH — E-19 epic v1.5 contains stale BC-3.08.001 version cites. Ground-truth grep: `grep -n "BC-3.08.001\|v1\.16\|v1\.17\|v1\.18" .factory/stories/epics/E-19-post-rc22-operator-hardening.md | head -20` returns three load-bearing clause occurrences of `BC-3.08.001 v1.16` and `BC-3.08.001 v1.17` in the §BC Traceability table cells and §Dependency Notes prose. The actual current version is v1.18 (D-756 PO leg). The D-754/D-755/D-756 fix bursts all bumped BC-3.08.001, and each bump should have triggered a sibling-sweep of all artifacts citing BC-3.08.001 by version. The epic was not swept at D-756. POLICY 14 5-leg parity and TD-VSDD-091 anti-volatile-pin requirement apply to epic BC-version cites in load-bearing clauses. Fix: story-writer BC-3.08.001 v1.16/v1.17 → v1.18 sweep in epic (enumerate all occurrences; literal grep to confirm 0 live stale cites after fix; include Event5=7-fields/Event6=9-fields clarifying note per orchestrator-supplied context).

F-P6-004 — HIGH — STORY-INDEX v4.136 S-19.05 row `head_cite` cell shows `BC-3.08.001 v1.17` while the authoritative current version is v1.18 (bumped D-756). Ground-truth grep: `grep -n "S-19.05\|BC-3.08.001" .factory/stories/STORY-INDEX.md | head -10` confirms the S-19.05 row cites v1.17 in the head-cite column. The D-756 fix burst that bumped BC-3.08.001 to v1.18 updated S-19.05's story body (event cite correction) but did not propagate the version bump to the STORY-INDEX head-cite cell for S-19.05. POLICY 14 5-leg parity: all 5 parity sites (version frontmatter, body Changelog, modified[] array, last_amended text-prefix, upstream-index body-table cells) must be updated same-burst. The STORY-INDEX cell is parity site 5; it was missed. Fix: story-writer STORY-INDEX S-19.05 row head-cite v1.17→v1.18.

F-P6-005 — HIGH — STORY-INDEX v4.136 narrative quad cited VP-INDEX v2.52 while the actual VP-INDEX version at time of writing was v2.53 (bumped D-756 architect leg). Ground-truth verification: `grep -n "VP-INDEX\|v2\.52\|v2\.53" .factory/stories/STORY-INDEX.md | head -10` confirms the narrative quad version-cite block shows `VP-INDEX.md:version: "2.52"`. `grep "^version:" .factory/specs/verification-properties/VP-INDEX.md` returns `version: "2.53"`. The STORY-INDEX v4.136 fix burst (story-writer leg at D-756) ran in parallel with the architect VP-INDEX leg (also D-756). At the moment the story-writer captured the VP-INDEX version via grep for the narrative quad, the architect leg had not yet committed the VP-INDEX v2.53 bump — the story-writer read v2.52, which was stale by the time the commit landed. This is the parallel-leg quad race: "grep at the moment of writing" is insufficient when two agents share a burst and both touch the 4-index namespace. Fix: story-writer STORY-INDEX v4.136→v4.137 narrative quad VP-INDEX cell corrected to v2.53. **OPERATIONAL RULE codified per D-757 (cure-extension to `L-BB-parallel-spec-authorship-requires-cross-reconciliation-sweep` via D-497 parsimony; no new lesson ID):** index-writing legs MUST be sequenced, never parallelized — any two agents that will each bump or cite one of the 4 indexes in the same burst MUST run in series, with the later agent re-deriving live index versions from the committed state of the earlier agent's output. Point-in-time grep is insufficient as a race mitigation.

Observations:

O-P6-001 — epic §Trigger prose could be extended to enumerate all 7 trigger scenarios for the E-19 epic (currently underspecified: lists only the post-rc22-smoke functional findings without capturing the governance-hygiene class of triggers like POLICY-17-vs-20 and BC-version-drift). Non-blocking at draft; encoding into epic Trigger section would provide clearer scope for future adversaries.

O-P6-002 — S-19.04 premise verification: the two-file assertion (underscore WASMs ship as two DISTINCT artifacts, not one) requires byte-level verification. **ORCHESTRATOR ADJUDICATION (D-757):** rc22-post-install-smoke.md and byte-verification against a04cb303 (the rc.22 bot bundle commit) confirms: both underscore-WASM files exist as distinct artifacts with different byte counts (341,975 B vs 342,292 B — NOT the same file with two names). The S-19.04 two-file premise is VERIFIED CORRECT. The keep-assertion (both must remain in the bundle as distinct files after the hello-hook removal) STANDS. No devops dispatch needed; no story amendment required for the two-file premise.

O-P6-003 — E-19 epic EAC-003 could be enriched: the current EAC-003 text lists the acceptance condition for STORY-INDEX convergence at E-19 close but does not enumerate the specific story version pins that must hold. As a post-implementation acceptance condition, enumerating expected story versions would make the epic close gate mechanical (grep-verifiable) rather than narrative. Non-blocking at draft; suggests enhancement for epic v1.6.

---

## Verifications That PASSED

The following 14 structural checks were confirmed clean at pass-6 perimeter entry (adversary evidence — grounds for future streak progression once all findings clear):

1. Bidirectional DAG parity PASS: all `depends_on` / `blocks` reciprocals verified for S-19.01..S-19.07.
2. Phase-A/B assignment PASS: BC-4.13.001 v1.6 Phase-A/Phase-B dual-anchor remains correctly structured; S-19.02 (Phase-A) and S-19.07 (Phase-B) correctly reflect the dual-story anchor.
3. ADR/BC/VP version matrix PASS: ADR-025 v1.9 / ADR-030 v1.0 / BC-5.42.001 v1.1 / BC-2.07.001 v1.1 / BC-1.17.001 v1.1 / BC-4.13.001 v1.6 / BC-3.08.001 v1.18 — all consistent with D-756 closure state.
4. S-19.07 New-story consistency PASS: S-19.07 v1.1 depends_on [S-19.02, S-19.06] W3 placement sound; BC-4.13.001 Phase-B anchor correctly cites S-19.07; ADR-025 D18(e) MUST-obligation anchored.
5. STORY-INDEX totals PASS: story_count 130, E-19 45pts correctly reflect 7 stories post-S-19.07 addition.
6. VP-INDEX completeness PASS: VP-094..VP-101 + VP-079 all registered; verification-architecture.md VP-079 row title confirmed stable-anchor form (bare title, no count suffix); no POLICY 9 gap.
7. BC-INDEX total_bcs PASS: 1,977 consistent with prior D-756 PO leg; no orphan BCs in E-19 frontmatter arrays.
8. E-19 epic EAC-005 dual-trace PASS: `satisfied_by: [S-19.04 AC-001, S-19.04 AC-007]` well-formed; both gates load-bearing.
9. S-19.03 Red Gate discipline PASS: stub constant -1000 (non-zero; out-of-band; guaranteed failing Red Gate per F-P5-008 fix at D-756).
10. S-19.02 drift-tolerant rationale PASS: range-based size expression in place; no per-pass stale point-in-time citation.
11. S-19.06 read_prefix dependency chain PASS: depends_on [S-19.03] correctly anchored; BC-1.17.001 v1.1 cited; SS-02 present in subsystems[].
12. S-19.04 removal-site enumeration PASS (pre-F-P6-002 finding): 3 candidate release.yml sites are enumerated in v1.6; however the narrative framing of those sites is inverted (see F-P6-002) — structural enumeration present but semantically wrong.
13. ARCH-INDEX v2.89 unchanged PASS: ADR-030 v1.0 and ADR-025 v1.9 registered; no architectural changes since D-754; ARCH-INDEX correctly held unchanged at D-755/D-756.
14. BC-2.02.011 epic traceability PASS: BC-2.02.011 row added to epic §BC Traceability table at D-756 O-P5-002; bidirectional trace S-19.03 ↔ epic confirmed.

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 5 |
| MEDIUM | 2 (stated in verdict; no enumerated findings) |
| LOW | 1 (stated in verdict; no enumerated findings) |
| Observations | 3 |

*Actionable findings: 5 (the 5 enumerated HIGH findings F-P6-001..F-P6-005). See orchestrator adjudication note above.*

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Severity decay from pass 5 (enumerated findings):** B0/H3/M4/L1 → B0/H5/M0/L0 (enumerated; 8→5; HIGH increased 3→5; MEDIUM+LOW reduced to 0 enumerated; majority class shift from narrative/cite class to POLICY-ID and control-flow-inversion class)

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 6 |
| **New findings** | 5 (enumerated; F-P6-001..F-P6-005) |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (5 / 5) |
| **Median severity** | HIGH |
| **Trajectory (enumerated findings per pass)** | 16 → 14 → 20 → 9 → 8 → 5 |
| **Verdict** | FINDINGS_REMAIN — pass 7 dispatched with fresh context |

**Note on F-P6-001 and F-P6-002 root-cause:** Both findings share a common root class — the orchestrator codified incorrect premises into dispatch briefs (D-753 stale policy-count premise; D-756 misread case-arm control-flow). Lesson `L-BB-finding-premise-must-be-verified-before-fix` binds the orchestrator as author of dispatch briefs, not only specialist agents as fixers. This pass demonstrated that orchestrator-authored premises can propagate through multiple passes before detection.

---

## Fix-Burst Closure Section (D-757)

**Story-writer single leg (one mid-response API death; resumed idempotently from verified delta; zero content loss confirmed). All 5 enumerated findings (F-P6-001..F-P6-005) closed. Orchestrator verified zero live residuals via body-scoped greps before commit.**

**Streak: 0/3** — pass-6 verdict is NOT-CLEAN per D-628/D-448(a); same-burst fixes do not advance streak; pass-7 NEXT with fresh context.

### Story-writer leg

- **S-19.04 v1.6→v1.7 (F-P6-001 + F-P6-002 + O-P6-002 cited):** POLICY 17→20 sweep: all AC cells and §Background prose citing POLICY 17 corrected to POLICY 20 (`release_bundle_no_dev_samples`). Orchestrator literal grep confirmed 0 live POLICY-17 residuals in S-19.04 scope after sweep. Case-arm narrative corrected to pass-through ground truth: the `) ;;` arm for the underscore-pair pattern PASSES THROUGH to `cp` (files INCLUDED in bundle, as confirmed by rc.22 smoke evidence); fix is to move pair to explicit skip/continue path; narrative updated with control-flow trace evidence per F-P6-002 analysis. O-P6-002 orchestrator adjudication cited in §Background: both underscore WASMs verified as distinct artifacts (341,975 B vs 342,292 B at a04cb303); two-file premise and keep-assertion stand. Closes F-P6-001 + F-P6-002.
- **E-19 epic v1.5→v1.6 (F-P6-003 + O-P6-001 + O-P6-003):** BC-3.08.001 v1.16/v1.17 → v1.18 sweep across all load-bearing clauses in §BC Traceability table and §Dependency Notes; enumerated all occurrences; orchestrator grep confirmed 0 live stale BC-3.08.001 version cites after sweep. Event5=7-fields/Event6=9-fields clarifying note added per orchestrator-supplied context (Event 5 = `plugin.abandoned` with 7 mandatory fields; Event 6 = `plugin.completed` async path with 9 mandatory fields including `plugin_version`). O-P6-001 Trigger prose extended to enumerate governance-hygiene trigger class alongside functional-findings class. O-P6-003 EAC-003 enrichment: expected story version pins added to make EAC-003 grep-verifiable at epic close. Closes F-P6-003 + O-P6-001 + O-P6-003.
- **STORY-INDEX v4.136→v4.137 (F-P6-004 + F-P6-005):** S-19.05 head-cite cell v1.17→v1.18 (per F-P6-004). v4.136 narrative quad VP-INDEX version corrected from v2.52 to v2.53 (per F-P6-005 parallel-leg quad race fix; re-derived from live VP-INDEX after fix burst). S-19.04 and epic version cells updated to reflect v1.7 and v1.6 respectively. Orchestrator verified zero stale version cites in STORY-INDEX E-19 section via body-scoped grep after sweep. Closes F-P6-004 + F-P6-005.

### Artifact versions at pass-6 closure

| Artifact | Version |
|----------|---------|
| BC-INDEX | v3.71 (UNCHANGED) |
| VP-INDEX | v2.53 (UNCHANGED) |
| ARCH-INDEX | v2.89 (UNCHANGED) |
| S-19.04 | v1.7 |
| E-19 epic | v1.6 |
| STORY-INDEX | v4.137 |

**Verdict per D-628/D-448(a):** NOT-CLEAN. Streak 0/3. **NEXT:** E-19 adversarial pass-7 (fresh context; 20-policy rubric; trajectory 16→14→20→9→8→5→pass-7).
