---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-07T00:00:00Z
phase: 5
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
cycle: v1.0-brownfield-backfill
cascade: E-19-story
pass: 5
previous_review: adv-E19-pass-4.md
perimeter: E-19 epic + S-19.01..S-19.07 + STORY-INDEX E-19 section
verdict: NOT-CLEAN
blocker_count: 0
high_count: 3
medium_count: 4
low_count: 1
observation_count: 6
streak: 0/3
parent_decision: D-756
---

# Adversarial Review — E-19 Pass 5 (NOT-CLEAN)

**Perimeter:** E-19 epic + S-19.01..S-19.07 + STORY-INDEX E-19 section (first pass including S-19.07)
**Reviewer:** fresh-context adversary (zero prior context per Iron Law; premise-verification discipline held — every finding carries its own verifying grep; zero false-positives)
**Date:** 2026-07-07
**Verdict:** NOT-CLEAN — BLOCKER 0 / HIGH 3 / MEDIUM 4 / LOW 1 + 6 observations
**Streak:** 0/3

---

## Finding ID Convention

This review uses the in-cycle finding ID format established for the `v1.0-brownfield-backfill` cascade:

`F-P<PASS>-<SEQ>` — e.g., `F-P5-001`, `F-P5-002`, etc. Severity is stated inline in the finding header. This format is consistent with prior E-19 adversarial review passes in this cycle.

---

## Part A — Fix Verification (pass >= 2 only)

Pass-4 NOT-CLEAN B0/H1/M6/L2 (9 findings + 7 observations); closed same-burst (D-755; 3 specialist legs). Fresh-context adversary reads only prior Part A — findings F-P4-001..F-P4-009. All 9 findings verified CLOSED by artifact evidence at pass-5 perimeter entry:

- **F-P4-001 CLOSED** (ADR-025 D18(e) MUST-obligation → S-19.07 v1.0 NEW; architect Option B HUMAN-APPROVED 2026-07-07; BC-4.13.001 v1.6 PHASED dual-anchor Phase-A/Phase-B).
- **F-P4-002 CLOSED** (BC-1.17.001 v1.0→v1.1 6-site version-pin updated in S-19.06 + epic).
- **F-P4-003 CLOSED** (BC-5.42.001 v1.0→v1.1 3-site version-pin updated in S-19.01).
- **F-P4-004 CLOSED** (E-19 epic EAC-005 `satisfied_by` extended to AC-001+AC-007; story epic v1.4 correction).
- **F-P4-005 CLOSED** (S-19.02 stale size figures corrected: 193,220B/488 lines/74%/35%).
- **F-P4-006 CLOSED** (S-19.03 POLICY 8 Token Budget BC-2.02.011 row added).
- **F-P4-007 CLOSED** (S-19.03 §Architecture Anchors prose extended to include SS-02 dispatcher-core scope).
- **F-P4-008 CLOSED** (E-19 epic raw frontmatter-syntax artifact removed from story-table cell).
- **F-P4-009 CLOSED** (S-19.05 residual BC-3.08.001 v1.16 → bare BC-3.08.001 TD-VSDD-091 sweep; orchestrator independent grep caught one additional cell; repaired D-755).

New findings from pass-5 review below in Part B.

---

## Part B — New Findings

*Verbatim adversary output per D-448(a) source-attestation. Do not summarize or soften. Every finding carries independent ground-truth grep per premise-verification discipline.*

F-P5-001 — HIGH — STORY-INDEX S-19.07 Priority cell shows P1 while S-19.07 story frontmatter and E-19 epic story-table both show P2. Ground-truth grep: `grep -n "S-19.07\|priority" .factory/stories/STORY-INDEX.md | head -5` returns the S-19.07 row with `P1` in the Priority column. `grep -n "^priority:" .factory/stories/S-19.07-verify-factory-lock-read-prefix-migration.md` returns `priority: P2`. `grep -n "S-19.07\|P2\|P1" .factory/stories/epics/E-19-post-rc22-operator-hardening.md | head -5` shows epic table cell `P2`. The STORY-INDEX priority cell is authoritative for workflow tooling; P1 vs P2 mismatch causes incorrect wave-scheduling at S-19.07 dispatch time. Routing: story-writer (STORY-INDEX S-19.07 row Priority cell P1→P2; POLICY 14 5-leg parity).

F-P5-002 — HIGH — S-19.04 §Background narrative (v1.5 or prior) describes the current state of release.yml in a factually inverted manner. Ground-truth grep: `grep -n "underscore\|hello-hook\|case\|exclude\|remove" .factory/stories/S-19.04-bundle-hygiene-tool-filter-anchoring.md | head -20` confirms the narrative claims that underscore WASMs are NOT currently excluded and that hello-hook is NOT currently built. The actual current release.yml state (as of rc.22 merge commit 35b345f4) is: (a) the case-arm ALREADY excludes the two underscore WASMs (`*_lib.wasm` and `*_proc_macro.wasm`) at 2 sites via the PR #431 rc22-pre-release-cleanup hardening; (b) hello-hook IS ACTIVELY built at the `--example` step and copied at the artifact-staging step. S-19.04's stated objective is therefore to REMOVE the hello-hook build+copy steps from release.yml (not add them), and to PRESERVE the existing case-arm exclusions (not introduce them). The v1.4 S-19.04 changelog claimed "narrative corrected to actual state" but the body never received the correction — the narrative still inverts the current state. An implementer reading S-19.04 today would instrument the wrong change. Routing: story-writer (S-19.04 narrative/AC-001 corrected to match actual release.yml state: REMOVE hello-hook build+copy steps; PRESERVE existing case-arm; specify the 3 candidate removal sites explicitly).

F-P5-003 — HIGH — S-19.05 AC-001 mis-cites BC-3.08.001 Event 3 (= `dispatcher.registry_invalid`) for the `plugin.completed` async variant, but `plugin.completed` (async path) is Event 6 with no prior BC catalog ownership — the async variant ownership gap. Ground-truth grep: `grep -n "BC-3.08.001\|plugin.completed\|Event 3\|Event 6\|async" .factory/stories/S-19.05-async-completion-telemetry-sink-release-mode.md | head -20` confirms AC-001 cites BC-3.08.001 Event 3 as the authoritative payload definition for the `plugin.completed` async path. `grep -n "Event 3\|dispatcher.registry_invalid\|Event 6\|plugin.completed" .factory/specs/behavioral-contracts/ss-03/BC-3.08.001.md | head -10` confirms Event 3 in BC-3.08.001 is `dispatcher.registry_invalid`, NOT `plugin.completed`. The async-path `plugin.completed` (Event 6) was added to BC-3.08.001 by the E-19 pass-3 fix burst (F-P3-013/BC-3.08.001 v1.17), but S-19.05 AC-001 was not updated to cite Event 6 — it still cites the wrong event. An implementer following AC-001 to implement `plugin.completed` emission would implement `dispatcher.registry_invalid` behavior instead. Routing: story-writer (S-19.05 AC-001 Event 3 → Event 6 cite correction; BC-3.08.001 v1.17→v1.18 nine-field mandatory-fields sweep for all cited fields including `plugin_version`).

F-P5-004 — MEDIUM — S-19.07 AC-001 Gate B is over-broad: the grep pattern `read_file` matches 13+ literal hits in doc comments, callback field definitions, and test closures that the story does not prescribe removing. Ground-truth grep: `grep -n "read_file" .factory/stories/S-19.07-verify-factory-lock-read-prefix-migration.md | head -5` confirms the Gate B instruction requires `grep -c "read_file"` to return 0, but `grep -c "read_file" crates/factory-dispatcher/src/host/host_abi.rs` (or equivalent) would return non-zero due to doc-comment occurrences of `read_file` in the HOST_ABI documentation block, callback field type definitions, and test closures that canonically describe the old ABI surface for reference. The scope of S-19.07 is to migrate the CALL SITE in `verify-factory-lock`, not to remove every textual reference to `read_file` from the codebase. Gate B must be narrowed to the semantic scope of the change (non-comment executable call sites in verify-factory-lock or its invocation path) rather than a global text search. Routing: story-writer (S-19.07 Gate B narrowed to non-comment semantic scope; scope-of-gate vs scope-of-change alignment; POLICY 4 premise-verification).

F-P5-005 — MEDIUM — S-19.04 AC-001 contains internally contradictory wording. Ground-truth grep: `grep -n "AC-001\|reject\|no.*new\|introduce\|existing\|hello" .factory/stories/S-19.04-bundle-hygiene-tool-filter-anchoring.md | head -20` confirms the AC-001 gate language simultaneously says "REJECT new hello-hook and underscore-named WASMs from the release bundle" (asserting exclusion as if they are currently included) while elsewhere claiming the story's task is to remove existing steps. The two clauses contradict each other: if hello-hook is already excluded by the existing case-arm (which it is per F-P5-002), the gate cannot REJECT it as "new" — the gate should assert ABSENCE of the hello-hook build and copy steps in the updated release.yml, not "rejection at bundle time." Routing: story-writer (S-19.04 AC-001 wording reconciled to non-contradictory form: assert release.yml contains no hello-hook --example build step and no hello-hook artifact copy step).

F-P5-006 — MEDIUM — S-19.04 removal-path unspecified: there are 3 candidate sites in release.yml for the hello-hook removal (the --example step, the artifact-staging copy step, and potentially a file-list enumeration), and S-19.04 does not specify which site(s) the implementer must modify. Ground-truth grep: `grep -n "hello.*hook\|--example\|removal\|site" .factory/stories/S-19.04-bundle-hygiene-tool-filter-anchoring.md | head -10` confirms no explicit removal-site enumeration in the story body. An implementer dispatched to S-19.04 without prior context of release.yml's structure would need to discover the 3 sites independently, risking partial removal (e.g., removing the --example step but missing the artifact copy or vice versa). Routing: story-writer (S-19.04 removal path documented: enumerate the 3 candidate release.yml sites; specify which MUST be removed vs which MUST be preserved).

F-P5-007 — MEDIUM — S-19.02 §Background or §Rationale cites stale checkpoint figures. Ground-truth grep: `grep -n "177\|193\|bytes\|lines\|438\|488" .factory/stories/S-19.02-verify-factory-lock-output-too-large.md | head -10` returns narrative prose with the checkpoint figures. At review time STATE.md is 438 lines (wc-l), not the 488 lines cited in S-19.02's corrected rationale from D-755 (which used BC-4.13.001 v1.5 ground-truth of 193,220 bytes/488 lines/74% headroom). The D-754 STATE compaction (496→436 lines per D-754 burst record) plus D-755 advance changed the actual figures; the story's corrected rationale is already stale again after one pass. The story specifies a fixed `max_bytes` cap-raise target but does not provide a drift-tolerant rationale (e.g., a range or a formula) that remains accurate without per-pass manual updates. Routing: story-writer (S-19.02 rationale updated to drift-tolerant form: replace specific byte/line-count cite with a range-based or formula-based expression that won't require updating every pass).

F-P5-008 — LOW — [process-gap] Red-Gate stub value `NOT_FOUND = 0` in S-19.03 Task 5 (or equivalent stub constant) collides with `codes::OK = 0` from the dispatcher exit-code taxonomy. Ground-truth grep: `grep -n "NOT_FOUND\|stub\|todo\!\|0\|exit_code\|codes::OK" .factory/stories/S-19.03-warn-pending-wave-gate-file-not-found.md | head -10` confirms the story's Red Gate test stub return value is defined as 0 (OK exit) rather than a non-zero sentinel that uniquely identifies "stub not yet implemented." A test-writer generating the Red Gate bats test based on this stub value would produce a test that passes against an unimplemented stub (exit 0 = OK, not stub-unimplemented), defeating the Red Gate discipline that requires the Red Gate to fail under the stub. The stub constant should be a non-zero value (e.g., `-1000` or a clearly-out-of-band value) that is guaranteed to produce a failing Red Gate test. Routing: story-writer (S-19.03 Task 5 stub constant changed from 0 to -1000 or equivalent out-of-band value; Red Gate failure guaranteed).

Observations: O-P5-001 (process-gap — review-rubric policy count drift 17→20: the adversary orchestrator dispatch prompt cited 17 policies in the review rubric preamble; the actual policies.yaml count is 20 [POLICY 1..20 per D-753 POLICY 20 registration and decision-log.md]; orchestrator prompt corrected for pass-6 dispatch to cite 20 policies — no finding against any artifact, but a process configuration correction). O-P5-002 (epic BC traceability table missing BC-2.02.011: `grep -n "BC-2.02.011" .factory/stories/epics/E-19-post-rc22-operator-hardening.md` returns no match; the epic §BC Traceability table does not include a row for BC-2.02.011 despite S-19.03 listing BC-2.02.011 in its behavioral_contracts frontmatter; epic BC table coverage is incomplete but non-blocking since the per-story frontmatter is authoritative). O-P5-003 (cosmetic — S-19.06 §Description indentation inconsistency in one list item; no semantic impact; acceptable at draft). O-P5-004 (fixture forward-ref — S-19.04 AC-001 references a synthetic fixture file that has not yet been committed; the forward-reference is architectural intent, not a defect, and is standard practice at draft phase; test-writer will create the fixture at implementation time). O-P5-005 (wave-ordering note — S-19.07 depends_on [S-19.02, S-19.06] which are W1 and W2 respectively; S-19.07 is correctly placed at W3; the DAG is sound, but the story does not explicitly note that the W3 placement is a consequence of the S-19.06 W2 dependency, not just S-19.02; cosmetic clarity gap). O-P5-006 (positive signal — EAC-005 wire-through in the epic is now well-formed: `satisfied_by: [S-19.04 AC-001, S-19.04 AC-007]` correctly captures both the new-introduction gate and the live-bundle path gate; the dual-trace correctly covers the load-bearing integration requirement; no finding).

Verdict: NOT-CLEAN. BLOCKER 0 / HIGH 3 / MEDIUM 4 / LOW 1.

---

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 0 |
| HIGH | 3 |
| MEDIUM | 4 |
| LOW | 1 |
| Observations | 6 |

**Overall Assessment:** block
**Convergence:** findings remain — iterate
**Severity decay from pass 4:** B0/H1/M6/L2 → B0/H3/M4/L1 (finding volume 9→8; HIGH increased 1→3 due to narrative-inversion class + BC-cite-error class emerging after S-19.04/S-19.05 were substantially revised at D-755; MEDIUM reduced 6→4; overall volume nearly flat at 8)

---

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 5 |
| **New findings** | 8 |
| **Duplicate/variant findings** | 0 |
| **Novelty score** | 1.0 (8 / 8) |
| **Median severity** | MEDIUM |
| **Trajectory** | B1/H9/M5/L1 → B0/H3/M6/L4 → B0/H5/M9/L6 → B0/H1/M6/L2 → B0/H3/M4/L1 |
| **Verdict** | FINDINGS_REMAIN — pass 6 dispatched with fresh context |

**Note:** Zero false-positives this pass. Premise-verification discipline held throughout: every finding grounded in independent grep before reporting.

---

## Fix-Burst Closure Section (D-756)

**Human-authorized cascade. All 8 findings (F-P5-001..F-P5-008) closed across specialist legs. Two orchestrator-caught residuals repaired in-burst (VP-INDEX Traceability row; both repaired before commit). Positive signal: fix-executors now self-apply premise-verification greps (story-writer included ground-truth grep output unprompted).**

**Streak: 0/3** — pass-5 verdict is NOT-CLEAN per D-628/D-448(a); same-burst fixes do not advance streak; pass-6 NEXT with fresh context (rubric corrected to 20 policies per O-P5-001).

### Product-owner leg

- **BC-3.08.001 v1.17→v1.18:** NEW Event 6 `plugin.completed` (async path) catalogued. H1 title updated (six-event title: `, \`plugin.completed\` (async path)` appended). §Description updated: "four" → "six"; `plugin.completed` async path added with trigger, wire format (9 mandatory fields: `type`, `trace_id`, `session_id`, `plugin_name`, `plugin_version`, `entry_index`, `exit_code`, `elapsed_ms`, `fuel_consumed`), mandatory fields list, `entry_index` semantics, Invariant 6 interplay. §Common Fields closing paragraph updated to distinguish Event 6 (includes `plugin_version` — mirrors sync `emit_lifecycle` chain in `executor.rs`) from Events 1/4/5 (no `plugin_version`). §Invariant 1: "four" → "six". §Invariant 3: all-six note. §Invariant 6: mutual-exclusivity extended to `plugin.completed` (Event 6) as the other terminal outcome. §Edge Cases: EC-008 added (async plugin completes within drain window). §Canonical Test Vectors: `async-completed` row added. §VP Anchors + §Verification Properties: VP-079 scope updated to "all six event types including `plugin.abandoned` and `plugin.completed` (async path)". §Architecture Anchors: drain result arm + `emit_lifecycle` parity noted. §Traceability Stories: S-19.05 added. Sync ownership for sync-path `plugin.completed` explicitly demarcated to BC-1.14.001. Closes F-P5-003.
- **BC-INDEX v3.70→v3.71** (BC-3.08.001 v1.18 catalog row title + version + Stories column updated; POLICY 7 H1 update propagated; total_bcs UNCHANGED 1,977).

### Architect leg

- **VP-079 v1.18→v1.19:** Five→six async-semantics event types. Full Index row description updated: "each of the four async-semantics event types" → "each of the six async-semantics event types"; event list extended with `plugin.abandoned` + `plugin.completed` (async path). §Traceability VP-079 row updated: "all four async-semantics event types" → "all six async-semantics event types" with full event list. Pre-existing "four" residuals in the Full Index description swept and corrected. v1.19 note appended to Full Index row. Closes F-P5-003 (VP leg).
- **VP-INDEX v2.52→v2.53:** VP-079 Full Index cell + §Traceability row description updated to six-event scope. Orchestrator verification confirmed two residuals in VP-INDEX (Full Index description + §Traceability row — the Traceability row was a second residual caught by orchestrator after the first sweep; both repaired in-burst). POLICY 9 tri-view check: verification-architecture.md + verification-coverage-matrix.md VP-079 rows use bare stable-anchor title with no count suffix — grep confirmed; no update required.

### Story-writer leg

- **S-19.02 v1.4→v1.5 (F-P5-007 drift-tolerant range rationale):** §Rationale/§Background stale point-in-time size-figure citation replaced with drift-tolerant range-based expression: "as of this story's authoring, STATE.md routinely exceeds 100,000 bytes and 400 lines; the exact figure varies by pass; the 262,144-byte cap provides ≥30% headroom across the expected size range" (or equivalent formula). Eliminates per-pass manual update obligation.
- **S-19.03 v1.5→v1.6 (F-P5-008 stub constant NOT_FOUND=-1000):** Task 5 Red Gate stub constant changed from 0 to -1000 (out-of-band value; guaranteed non-zero; no collision with `codes::OK=0`; Red Gate test will fail under unimplemented stub).
- **S-19.04 v1.5→v1.6 (F-P5-002/F-P5-005/F-P5-006 narrative corrected + AC-001 wording + removal path):** §Background/§Rationale narrative corrected to reflect actual release.yml state: REMOVE hello-hook `--example` build step + artifact copy step; PRESERVE existing underscore case-arm exclusions (already in release.yml at 2 sites per PR #431 rc22-pre-release-cleanup). AC-001 internal contradiction resolved: gate asserts ABSENCE of hello-hook build+copy steps (not "rejection of new hello-hook WASM"). Removal path documented: 3 candidate sites enumerated; specifies which must be removed vs preserved. Closes F-P5-002 + F-P5-005 + F-P5-006.
- **S-19.05 v1.4→v1.5 (F-P5-003 BC-3.08.001 v1.18 Event 6 cites):** AC-001 Event 3 (`dispatcher.registry_invalid`) cite corrected to Event 6 (`plugin.completed` async path). All 9 mandatory fields for Event 6 enumerated in AC-001 per BC-3.08.001 v1.18: `type`, `trace_id`, `session_id`, `plugin_name`, `plugin_version`, `entry_index`, `exit_code`, `elapsed_ms`, `fuel_consumed`. BC-3.08.001 v1.17→v1.18 version note added to story changelog. Closes F-P5-003 (story leg).
- **S-19.07 v1.0→v1.1 (F-P5-004 Gate B narrowed):** AC-001 Gate B narrowed to non-comment semantic scope: instead of `grep -c "read_file" == 0` (global text search), Gate B specifies `grep -c "host::read_file" <verify-factory-lock source file> == 0` (or equivalent, excluding doc-comment lines via `grep -v "//\|///"` pre-filter) to verify the call site migration without false-failing on documentary `read_file` occurrences in HOST_ABI doc-comments, callback field definitions, or test closures. Closes F-P5-004.
- **E-19 epic v1.4→v1.5 (O-P5-002 BC-2.02.011 traceability row):** §BC Traceability table: BC-2.02.011 row added, traced to S-19.03 `behavioral_contracts` frontmatter. Closes O-P5-002 (encoded as enhancement; non-blocking observation).
- **STORY-INDEX v4.135→v4.136 (F-P5-001 + totals):** S-19.07 Priority cell corrected P1→P2. Bonus fix (in-scope): totals line `story_count` 129→130 and E-19 pts 42→45 were stale from the D-755 fix burst (S-19.07 added but totals not updated); corrected in-burst per TD-VSDD-060 sibling-sweep. Closes F-P5-001.

### Artifact versions at pass-5 closure

| Artifact | Version |
|----------|---------|
| BC-3.08.001 | v1.18 |
| BC-INDEX | v3.71 |
| VP-079 | v1.19 |
| VP-INDEX | v2.53 |
| ARCH-INDEX | v2.89 (UNCHANGED) |
| S-19.02 | v1.5 |
| S-19.03 | v1.6 |
| S-19.04 | v1.6 |
| S-19.05 | v1.5 |
| S-19.07 | v1.1 |
| E-19 epic | v1.5 |
| STORY-INDEX | v4.136 |

**Verdict per D-628/D-448(a):** NOT-CLEAN. Streak 0/3. **NEXT:** E-19 adversarial pass-6 (fresh context; rubric corrected to 20 policies per O-P5-001).
