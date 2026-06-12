---
document_type: pipeline-state
level: ops
version: "3.02"
status: draft
producer: state-manager
timestamp: 2026-06-12T02:00:00Z
phase: D-552-ADR-025-V1.6-PASS-2-CORRECTIONS-S-17.04-V1.4-CODIFIED-2026-06-12
last_amended: 2026-06-12 (v3.02) — D-552 ADR-025 v1.6 Gemini cross-family adversary pass 2 corrections + S-17.04 v1.3→v1.4 CODIFIED: §12.3 LockExpiryStale absent+empty subcases → Block; §12.7 R6 step 5 `..` segment-stack; verify-factory-lock +MultiEdit; SKILL.md +MultiEdit; bats plugins_run=1 guard-ran; clarity note timestamp vs last_amended; impl green 1d92d847 28/28+6/6; D-539 multi-family satisfied; ARCH-INDEX v2.24→v2.25; STORY-INDEX v3.96→v3.97; BC impact NONE; D-chain cite D-551; parent-commit ce277f92. [Prior: 2026-06-12 (v3.01) — D-551 ADR-025 v1.6 adversary-pass-1 corrections + S-17.04 v1.2→v1.3 CODIFIED: payload field fix (new_content→reconstruct semantics: Write content / Edit old_string+new_string / MultiEdit edits[]); registry caps path_allow-only (ReadFileCaps deny_unknown_fields); explicit priorities 142/143; canonical-path normalization; block_with_fix message format; AC-005/006/010 corrected + AC-011..015 added; Red Gate 12→19 tests; validated Perplexity + dispatcher-log ground truth (new_content 0× in 5000+ events); LOCAL adversary streak 0/3 (C2/H4/M4/L1; re-cascade pending); ARCH-INDEX v2.23→v2.24; STORY-INDEX v3.95→v3.96; BC impact NONE; D-chain cite D-550; parent-commit 8f19bab2. [Prior: 2026-06-11 (v3.00) — D-550 ADR-025 v1.5→v1.6 + S-17.04 v1.1→v1.2 REDIRECT (human-approved): Decision 11 Mechanism 2 bash gate WITHDRAWN; Decision 12 verify-state-timestamp-refresh WASM PreToolUse guard ADOPTED (fires on Edit/Write to .factory/STATE.md; blocks TimestampStale + LockExpiryStale; on_error=continue fail-open); push-time cas-push chokepoint dropped; D15 factory-lock-parse shared crate + D16 guard + D17 tests; S-17.04 v1.2 8pts SS-04+SS-05 10ACs 12 Red Gate tests; ARCH-INDEX v2.22→v2.23; STORY-INDEX v3.94→v3.95; BC impact NONE; D-chain cite D-549; parent-commit 29ee394b. [Prior: 2026-06-11 (v2.99) — D-549 SESSION-END DURABILITY BURST: ADR-025 v1.4→v1.5. S-17.04 v1.0→v1.1. ARCH-INDEX v2.22. STORY-INDEX v3.94. OPEN DESIGN DECISION. [Prior: 2026-06-11 (v2.98) — D-548 ADR-025 v1.3→v1.4 + S-17.04 CODIFIED. [Prior: 2026-06-11 (v2.97) — D-547 S-17.03 DELIVERED/MERGED PR #183 60fd0233; BC-6.23.001 ACTIVE; issue #170 CLOSED; E-17 3/3 COMPLETE. [Prior: 2026-06-11 (v2.96) — D-546 S-17.03 v1.1 helpers. [Prior: 2026-06-11 (v2.95) — D-545 S-17.02 MERGED df4f26b8. [Prior: 2026-06-11 (v2.94) — D-544 S-17.01 MERGED c64b46d2. [Prior: 2026-06-10 (v2.93) — D-543 S-17.01 v1.1 helpers. [Prior: 2026-06-10 (v2.92) — D-542 E-17+3 stories. [Prior: 2026-06-10 (v2.91) — D-541 3-BCs. [Prior: 2026-06-10 (v2.90) — D-540 ADR-025 ADOPTED.]]]]]]]]]]]]]]
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
project: vsdd-factory
mode: brownfield
current_step: "D-552 ADR-025-V1.6-PASS-2-CORRECTIONS-S-17.04-V1.4 2026-06-12 — GEMINI CROSS-FAMILY ADVERSARY PASS 2 CORRECTIONS CODIFIED: ADR-025 v1.6 §12.3 LockExpiryStale table expanded (lock-held + proposed expires_at absent → Block; lock-held + proposed expires_at empty → Block; previously only byte-identical stale triggered Block); §12.7 R6 step 5 added (.. segment-stack resolution: split on /, pop on .., above-root escape discarded); §12.9 clarity note (timestamp: sole guard-gated field; last_amended: POLICY-14 discipline only — NOT independently gated); verify-factory-lock tool matcher Edit|Write|Agent → Edit|Write|MultiEdit|Agent (lock-identity guard parity); SKILL.md anti-pattern +MultiEdit; bats guard-ran assertion (plugins_run=1 — on_error=continue exit-0 insufficient alone); impl green 1d92d847 28/28 unit + 6/6 bats; D-539 multi-family obligation satisfied; S-17.04 v1.3→v1.4 [AC-016/017 added; EC-006 ..; AC-006 scope note; AC-001/010 amended; Red Gate 19→24 (19 Rust unit + 5 bats)]; LOCAL adversary streak 0/3 (re-cascade REQUIRED after TDD re-implementation); 4-index: BC-INDEX v2.72 UNCHANGED VP-INDEX v2.06 UNCHANGED STORY-INDEX v3.96→v3.97 ARCH-INDEX v2.24→v2.25; trajectory-tail →9→9→9→11; maintain all 5 BC-5.39.006 v1.7 PCs per TD-VSDD-097-EXT; D-chain cite D-551 per D-419(b); parent-commit ce277f92 per D-419(b). SIZE BUDGET: see banner tracker row D-552"
current_cycle: v1.0-brownfield-backfill
dtu_required: false
dtu_assessment: 2026-04-25
dtu_clones_built: "n/a"
dtu_services: []
---

<!--
  STATE.md SIZE BUDGET (per D-421(c) + D-422(c) reconciliation):
  Soft target: ≤415 lines; hard cap: 500 lines (validate-state-md-size hook enforcement).
  D-446(c) dual-margin form: margin from soft-target = 500 - 415 = 85; margin from actual tracked below.
  Historical content belongs in cycle files, NOT here.

  D-430(a) compaction authorization (D-532 burst 2026-06-08):
  Pre-D-520 banner tracker entries (D-504..D-519, 16 entries) archived per D-430(a);
  Phase Progress F5 pass-9..17 adversary+fix-burst rows (20 rows) archived per D-430(a);
  Decisions Log D-499..D-509 (11 rows) archived per D-430(a).
  All content preserved via: git show 688dd1c2:.factory/STATE.md (pre-compaction state).
  Pre-D-504 tracker preserved at: git show 20cb8e1c:.factory/STATE.md.

  D-430(a) compaction (D-538 burst 2026-06-10):
  Decisions Log D-527+D-528 (2 rows, ~8 lines) archived to decision-log.md SoT per D-430(a).
  All content preserved via decision-log.md SoT (rows present from prior entry).

  Line-growth tracker (D-532 onward; D-520..D-531 archived per D-430(a) 2026-06-10 compaction):
  Pre-D-520 tracker entries (D-504..D-519) preserved at: git show 688dd1c2:.factory/STATE.md.
  D-520..D-531 tracker entries (12 entries, all OVER soft-target) archived per D-430(a) 2026-06-10; preserved at: git show c62c2c03:.factory/STATE.md lines 36-51.
  D-532..D-535 tracker entries (4 entries; all UNDER soft-target) archived per D-430(a) D-542 burst; preserved at: git show ba471c58:.factory/STATE.md lines 43-46.
  D-536..D-538 (archived per D-430(a) D-543 burst): D-536 416L +1 over; D-537 421L +6 over; D-538 408L -7 under (D-430(a) compaction §1-§12 refresh).
  D-539-ISSUE-169-176-PR-180-MERGED-2026-06-10 401 lines (wc-l; D-430(a) D-529+D-530+D-531 archived; 14 UNDER soft; D-446(c)).
  D-540-ADR-025-ADOPTED-ISSUE-170-DESIGN-2026-06-10 409 lines (wc-l; ADR-025 v1.2; ARCH-INDEX v2.19; 6 UNDER soft; D-446(c)).
  D-541-BC-AUTHORING-ISSUE-170-3-BCS-AUTHORED-2026-06-10 415 lines (wc-l; 3 BCs + CAP-031 + BC-INDEX v2.66; AT soft-target; D-446(c)).
  D-542-STORY-DECOMPOSITION-ISSUE-170-3-STORIES-E17-AUTHORED-2026-06-10 415 lines (wc-l; epic E-17 + S-17.01/02/03; STORY-INDEX v3.85; D-430(a) compaction: D-510+D-522+D-525+D-526+D-532..D-535 archived; §1-§4-§5-§8-§9-§10-§11-§12 Session Resume refresh; margin 500-415=85 from hard cap; margin 415-415=AT soft-target; D-446(c) dual-margin form).
  D-543-S-17.01-V1.1-EXECUTABLE-HELPER-REFINEMENT-2026-06-10 418 lines (wc-l; STORY-INDEX v3.86; D-543 row + §1/§3/§4/§5/§8/§9/§11/§12 refresh; D-430(a) D-536..D-538 banner archived; +3 over soft-target; margin 500-418=82 from hard cap; D-446(c) dual-margin form).
  D-544-S-17.01-DELIVERED-MERGED-2026-06-11 430 lines (wc-l; D-544 row + BC-5.40.001 POL-14 active + §1/§3/§4/§5/§8/§9/§10/§11/§12 refresh; STORY-INDEX v3.88; BC-INDEX v2.67; develop c64b46d2; +15 over soft-target; margin 500-430=70 from hard cap; D-446(c) dual-margin form).
  D-545-S-17.02-DELIVERED-MERGED-2026-06-11 409 lines (wc-l; D-545 row + BC-4.13.001 POL-14 active + ADR-025 v1.3 + §1/§3/§4/§8/§9/§10/§11/§12 refresh; STORY-INDEX v3.90; BC-INDEX v2.70; ARCH-INDEX v2.20; develop df4f26b8; D-430(a) compaction Phase Progress + Decisions Log D-532..D-543 archived; -6 under soft 415; margin 500-409=91 from hard cap; D-446(c) dual-margin form).
  D-546-S-17.03-V1.1-EXECUTABLE-HELPER-REFINEMENT-2026-06-11 413 lines (wc-l; D-546 row + STORY-INDEX v3.91 + §1/§3/§4/§5/§8/§9/§10/§11/§12 refresh; BC-INDEX v2.70 UNCHANGED; ARCH-INDEX v2.20 UNCHANGED; -2 under soft 415; margin 500-413=87 from hard cap; D-446(c) dual-margin form).
  D-547-S-17.03-DELIVERED-MERGED-2026-06-11 415 lines (wc-l; D-547 row + BC-6.23.001 POL-14 active + §1/§3/§4/§5/§8/§9/§10/§11/§12 refresh; issue #170 CLOSED; E-17 3/3 COMPLETE; STORY-INDEX v3.91→v3.92; BC-INDEX v2.71→v2.72; develop 60fd0233; D-430(a) compaction: Phase Progress D-544+D-545 rows → D-532..D-545 archive row; Decisions Log D-527..D-531 folded; AT soft 415; margin 500-415=85 from hard cap; D-446(c) dual-margin form).
  D-548-ADR-025-V1.4-S-17.04-AUTO-RENEW-WIRING-CODIFIED-2026-06-11 412 lines (wc-l; D-548 row + ADR-025 v1.4 Decision 11 + S-17.04 E-17 wave 4 draft + ARCH-INDEX v2.21 + STORY-INDEX v3.93; §1/§3/§4/§5/§8/§9/§10/§11/§12 refresh; D-430(a): dup banner entry + D-545-carry-superseded + 2 PR rows compacted; -3 under soft 415; margin 500-412=88 from hard cap; D-446(c) dual-margin form).
  D-549-SESSION-END-DURABILITY-BURST-2026-06-11 340 lines (wc-l; ADR-025 v1.4→v1.5 + S-17.04 v1.0→v1.1 codified; ARCH-INDEX v2.21→v2.22; STORY-INDEX v3.93→v3.94; §1-§12 Session Resume Checkpoint full refresh; OPEN DESIGN DECISION as resume entry point; D-430(a): D-532..D-548 Phase Progress rows archived to single row; D-430(a): §4-§7 compacted; -75 under soft 415; margin 500-340=160 from hard cap; D-446(c) dual-margin form).
  D-550-ADR-025-V1.6-S-17.04-V1.2-REDIRECT-CODIFIED-2026-06-11 336 lines (wc-l; ADR-025 v1.5→v1.6 + S-17.04 v1.1→v1.2 REDIRECT; ARCH-INDEX v2.22→v2.23; STORY-INDEX v3.94→v3.95; §1/§3/§4/§5/§8/§9/§11/§12 Session Resume refresh; OPEN DESIGN DECISION RESOLVED; D-550 row added; -79 under soft 415; margin 500-336=164 from hard cap; D-446(c) dual-margin form).
  D-552-ADR-025-V1.6-PASS-2-CORRECTIONS-S-17.04-V1.4-CODIFIED-2026-06-12 346 lines (wc-l; ADR-025 v1.6 Gemini cross-family pass 2 corrections + S-17.04 v1.3→v1.4; ARCH-INDEX v2.24→v2.25; STORY-INDEX v3.96→v3.97; §1/§3/§4/§5/§8/§9/§11/§12 Session Resume refresh; D-552 row added; -69 under soft 415; margin 500-346=154 from hard cap; D-446(c) dual-margin form).
  D-551-ADR-025-V1.6-PASS-1-CORRECTIONS-S-17.04-V1.3-CODIFIED-2026-06-12 341 lines (wc-l; ADR-025 v1.6 adversary-pass-1 content corrections + S-17.04 v1.2→v1.3; ARCH-INDEX v2.23→v2.24; STORY-INDEX v3.95→v3.96; §1/§3/§4/§5/§8/§9/§11/§12 Session Resume refresh; D-551 row added; -74 under soft 415; margin 500-341=159 from hard cap; D-446(c) dual-margin form).
-->

# Pipeline State: vsdd-factory

> **Self-referential note:** vsdd-factory IS the project being onboarded. Engine and product are the same repository.

## Project Metadata

| Field | Value |
|-------|-------|
| **Product** | vsdd-factory |
| **Repository** | /Users/jmagady/Dev/vsdd-factory |
| **Mode** | brownfield-onboarding |
| **Language** | Rust + Bash + Markdown |
| **Started** | 2026-04-25 |
| **Last Updated** | 2026-06-12 — D-552 ADR-025 v1.6 Gemini cross-family adversary pass 2 corrections: §12.3 LockExpiryStale absent+empty → Block; §12.7 R6 step 5 `..` segment-stack; verify-factory-lock +MultiEdit; bats guard-ran R5; clarity note timestamp vs last_amended; impl green 1d92d847 28/28+6/6 bats; D-539 multi-family satisfied; S-17.04 v1.3→v1.4 (17 ACs; 24 Red Gate tests); ARCH-INDEX v2.24→v2.25; STORY-INDEX v3.96→v3.97; LOCAL adversary streak 0/3; re-cascade pending. |
| **Current Phase** | D-552 ADR-025-V1.6-PASS-2-CORRECTIONS-S-17.04-V1.4 2026-06-12 — Gemini cross-family adversary pass 2 corrections codified: §12.3 LockExpiryStale absent+empty → Block; §12.7 R6 step 5 `..` segment-stack; verify-factory-lock +MultiEdit; bats guard-ran R5; clarity note; impl green 1d92d847 28/28+6/6; D-539 satisfied. S-17.04 v1.4 (17 ACs; 24 Red Gate tests). LOCAL streak 0/3. Next: TDD re-implementation → LOCAL re-cascade 0/3 → PR → rc.21. |
| **Current Cycle** | v1.0-brownfield-backfill |

## Phase Progress

| Phase | Status | Artifact |
|-------|--------|----------|
| Phases 0-B, Waves 1-11, S-7.03, beta.5-7, W-14, W-15 | **COMPLETE** | `cycles/v1.0-brownfield-backfill/phase-progress-archive.md` |
| Phase D-1..D-4, Waves 12-16, E-9 v1.7 sweep | **COMPLETE** | `cycles/v1.0-brownfield-backfill/` |
| Releases rc.11..rc.18, F3/F4 E-12, S-12.03..S-12.08 | **ARCHIVED 2026-06-01 per D-430(a)** | Full rows: `git show aa1f05c9:.factory/STATE.md` lines 80-93. |
| F5 passes 3-8 cycle-level adversary + fix bursts | **COMPLETE** | Trajectory 11→9→8→7→5; F5 pass-8 verdict MEDIUM; ARCH-INDEX v1.45, D-381. |
| F5 passes 9-17 adversary + fix bursts | **ARCHIVED 2026-06-08 per D-430(a)** | 20 rows archived; trajectory pass-9→17: HIGH→MEDIUM→MEDIUM→MEDIUM→HIGH→MEDIUM→MEDIUM×3; D-382..D-392 codified; L-EDP1-007/009 captured. Full rows: `git show 688dd1c2:.factory/STATE.md` lines 85-106. |
| D-343..D-523 (E-10 pass-9..14, M3 cascade, S-15.03 PRIORITY-A waves, rc.19, S-15.17 cascade) | **ARCHIVED 2026-06-10 per D-430(a)** | 22 rows archived; all COMPLETE/SEALED/SHIPPED; Full rows: `git show c62c2c03:.factory/STATE.md` lines 82-108. Summary: E-10 pass-9..14 SEALED D-471; M3 BC 11-pass CONVERGED D-497; S-15.03 PRIORITY-A COMPLETE D-508 (all 11 stories + 40pts); rc.19 SHIPPED d15152af; S-15.17 cascade 9-pass SEALED D-522; 7/7 uncertainties CONFIRMED D-523. |
| Release v1.0.0-rc.20 | **SHIPPED 2026-06-01** at 2a191314 | PR #166 --merge e00ab1ab; tag e9e38286; run 26738809372 all 6 jobs PASS; GitHub Release prerelease; marketplace PR #12 squash-merged 862e660d; S-15.17 hook + MCP fleet-sweep reach operator cache; plugin count 52→53; develop sync 9ed17b1d→474a2731 |
| POST-RC.20 MAINTENANCE SWEEP | **COMPLETE 2026-06-01** D-529 | td-74 worktree+branch removed; Dependabot: #3+#156+#157 MERGED, #152/#125/#2+#167 closed-redundant; develop 474a2731→b21fd358; zero open PRs |
| E-10 pass-16 adversary + fix-burst PR #168 | **COMPLETE 2026-06-01** D-530 | verdict LOW (0C+0H+0M+3L); trend 22→…→8→3; F-PASS16-002 CI-count-floor FIXED PR #168 82163b7f (derived from crate count); F-PASS16-001+003 ACCEPTED-AT-FLOOR per D-471. |
| E-10 adversarial cascade | **SEALED 2026-06-01 at pass-16 (D-531)** | verdict LOW; 16-pass trend 22→11→16→16→12→2→1→4→5→4→6→7→5→8→8→3; asymptotic-acceptance per D-471/D-386 Option C; S-7.02 SATISFIED; resumption gate = engine-surface material change |
| D-526 S-15.17 SHIPPED — PR #164 9ed17b1d | **SHIPPED 2026-05-31** | validate-trajectory-tail-cell-completeness WASM hook; priority 158; BC-5.39.009 POL-14 draft→active; ADV-EDP1-P75-HIGH-002 CLOSED; BC-INDEX v2.65; STORY-INDEX v3.84; develop HEAD 9ed17b1d |
| D-532..D-548 (2026-06-08..2026-06-11) | **ARCHIVED 2026-06-11 per D-430(a)** | D-532 SESSION-END; D-535 #128 MERGED f6ce4b7c; D-537 #130 MERGED 89fbe2d6 ADR-024 v1.2; D-539 #169+#176 MERGED 0f4793f1; D-540 ADR-025 ADOPTED; D-541 3-BCS; D-542 E-17+stories; D-543 S-17.01 v1.1; D-544 S-17.01 MERGED c64b46d2 BC-5.40.001 ACTIVE E-17 1/3; D-545 S-17.02 MERGED df4f26b8 BC-4.13.001 ACTIVE ADR-025 v1.3 E-17 2/3; D-546 S-17.03 v1.1 helpers; D-547 S-17.03 MERGED 60fd0233 BC-6.23.001 ACTIVE E-17 3/3 issue #170 CLOSED; D-548 ADR-025 v1.4+S-17.04 CODIFIED ARCH-INDEX v2.21 STORY-INDEX v3.93. Full rows: decision-log.md SoT. |
| D-549 SESSION-END DURABILITY BURST 2026-06-11 | **COMPLETE** | ADR-025 v1.4→v1.5 (F-1701-001 gate-trigger + block-message + D12-jq 4th-footgun); S-17.04 v1.0→v1.1 (AC-002 belt-and-suspenders + 4 EC Red Gate tests; count 8→12); ARCH-INDEX v2.21→v2.22; STORY-INDEX v3.93→v3.94; #170/E-17 COMPLETE recorded; S-17.04 IN-FLIGHT @ f627a1c5; OPEN DESIGN DECISION (Mechanism 2 chokepoint A/B/C) as resume entry point |
| D-552 ADR-025-V1.6-PASS-2-CORRECTIONS-S-17.04-V1.4 2026-06-12 | **COMPLETE** | ADR-025 v1.6 Gemini cross-family adversary pass 2 corrections (content additions; version stays 1.6): §12.3 LockExpiryStale absent+empty → Block; §12.7 R6 step 5 `..` segment-stack; verify-factory-lock +MultiEdit; bats guard-ran R5; clarity note timestamp vs last_amended; impl green 1d92d847 28/28+6/6 bats; D-539 multi-family SATISFIED. S-17.04 v1.3→v1.4 (17 ACs; 24 Red Gate tests; AC-016/017 added). ARCH-INDEX v2.24→v2.25; STORY-INDEX v3.96→v3.97; BC impact NONE; LOCAL adversary streak 0/3 |
| D-551 ADR-025-V1.6-PASS-1-CORRECTIONS-S-17.04-V1.3 2026-06-12 | **COMPLETE** | ADR-025 v1.6 Decision 12 adversary-pass-1 corrections (content revision; version stays 1.6): payload-field fix new_content→reconstruct (Write content / Edit old_string+new_string / MultiEdit edits[]); registry path_allow-only (ReadFileCaps deny_unknown_fields); priorities 142/143; canonical-path normalization; block_with_fix format; tool=Edit|Write|MultiEdit. S-17.04 v1.2→v1.3 (15 ACs; 19 Red Gate tests; AC-011..015 added). ARCH-INDEX v2.23→v2.24; STORY-INDEX v3.95→v3.96; BC impact NONE; LOCAL adversary streak 0/3 |
| D-550 ADR-025-V1.6-S-17.04-V1.2-REDIRECT 2026-06-11 | **COMPLETE** | ADR-025 v1.5→v1.6 (human-approved): Decision 11 Mechanism 2 bash gate WITHDRAWN; Decision 12 verify-state-timestamp-refresh WASM PreToolUse guard ADOPTED; D15 factory-lock-parse shared crate + D16 plugin + D17 tests; S-17.04 v1.1→v1.2 (10 ACs; 12 Red Gate tests; 8pts; SS-04+SS-05); ARCH-INDEX v2.22→v2.23; STORY-INDEX v3.94→v3.95; BC impact NONE; OPEN DESIGN DECISION RESOLVED |

## Current Phase Steps

> **Rows before pass-57 archived to** `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md` per STATE.md content-routing rules (keep last 5 only).

| Step | Agent | Status | Output |
|------|-------|--------|--------|
| F5 passes 18-60 fix bursts (archived) | state-manager | ARCHIVED | See `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md`. Passes 57-59: D-437..D-439 (META-LEVEL-12/13/14 CANDIDATES; trajectory →8→8→9); pass-60: D-440 META-LEVEL-15 CONFIRMED. |

## Identifier Conventions

| Type | Format | Authoritative Source | Count |
|------|--------|----------------------|-------|
| Subsystem | SS-NN | `specs/architecture/ARCH-INDEX.md` | 10 |
| Behavioral Contract | BC-S.SS.NNN | `specs/behavioral-contracts/ss-NN/` | 1,958 |
| Verification Property | VP-NNN | `specs/verification-properties/VP-INDEX.md` | 80 |
| Story | S-N.MM | `stories/S-N.MM-<short>.md` | 105 file-resident + 15 stub IDs (STORY-INDEX v3.86) |
| Epic | E-N | `stories/epics/E-N-<short>.md` | 17 |
| ADR | ADR-NNN | `specs/architecture/decisions/ADR-NNN.md` | 24 |

## Story Status

105 file-resident + 15 unauthored stub IDs = 120 stories registered.

- **Merged (77):** Includes all prior + S-15.04 (PR #142 fdc7da16) + S-15.05 (PR #143 224fa184) + S-15.08 (PR #144 c62f952c) + S-15.07 (PR #145 6fe7de4c) + S-15.11 (PR #146 6e0d5407) + S-15.09 (PR #147 6e2d7805) + S-15.14 (PR #148 6d2ba5ad) + S-15.16-Part-B (PR #153 c1c81603) + S-15.10 (PR #154 a36ab711) + S-15.12 (PR #155 fba7e1cd) + S-15.15 (PR #158 24cc2ba6) + S-15.13 (PR #159 ced39c82) + S-15.17 (PR #164 9ed17b1d) + **S-17.01 (PR #181 c64b46d2)** + **S-17.02 (PR #182 df4f26b8)** + **S-17.03 (PR #183 60fd0233)**. Full ledger: `cycles/v1.0-brownfield-backfill/merged-stories-ledger.md`
- **In-Flight (0):** —
- **Draft (30 file-resident):** S-5.07; S-10.09; S-11.00; S-14.01..S-14.09 (E-14); S-15.02; S-15.03; S-16.01..S-16.02 (E-16); **S-17.04** (E-17 wave 4 auto-renew wiring D-548); and others
- **Partial (2):** S-2.05 (hook-sdk-publish); S-3.04 (emit-event-host-function) — superseded by ADR-015
- **Unauthored stub IDs (15):** S-9.01..S-9.07 (W-16); S-11.01..S-11.08 (E-11 W-17 Tier 3)
- **Withdrawn (1):** S-9.30

## Active Branches

| Branch / Tag | SHA | Notes |
|--------------|-----|-------|
| main | 2a191314 | rc.20 SHIPPED 2026-06-01; bot binary commit on top of --merge from develop; prior: 43afbfa7 (rc.19 2026-05-28) |
| develop | 60fd0233 | D-547 PR #183 S-17.03 SQUASH-MERGED 2026-06-11; issue #170 CLOSED; E-17 3/3 COMPLETE; prior: df4f26b8 (D-545 PR #182 S-17.02 2026-06-11) |
| factory-artifacts | `TBD-D552` | D-552 ADR-025 v1.6 pass-2 corrections + S-17.04 v1.4 2026-06-12; prior: `f7a4cc1a` D-551; prior-prior: `8f19bab2` D-550 |
| v1.0.0-rc.20 (tag) | e9e38286 | SHIPPED 2026-06-01; annotated tag object; GitHub Release prerelease; marketplace PR drbothen/claude-mp #12 squash-merged 862e660d |
| v1.0.0-rc.19 (tag) | d15152af | SHIPPED 2026-05-28; GitHub Release prerelease 2026-05-28T15:10:56Z; marketplace PR #11 squash-merged |
| v1.0.0-rc.18 (tag) | 666d689f | SHIPPED 2026-05-13 PR #135 |

## Concurrent Cycles

| Cycle | Type | Status | Notes |
|-------|------|--------|-------|
| F-block-ai-attribution-message-file-arm | feature | F3 COMPLETE — F4 READY | F1+F2+F3 done 2026-05-12; 2 stories ready (S-16.01 5pts PostToolUse HEAD verify, S-16.02 3pts PreToolUse -F arm); E-16 under SS-07/SS-04; milestone v1.0.0-rc.17; BC-7.03.094/095/001, VP-080, ARCH SS-07 v1.3/SS-04 v1.4 registered |
| v1.0-brownfield-backfill | brownfield | **D-552 ADR-025 v1.6 pass-2 corrections + S-17.04 v1.4 CODIFIED 2026-06-12; LOCAL adversary streak 0/3; S-17.04 v1.4 awaiting TDD re-cascade** | S-15.03 PRIORITY-A COMPLETE D-508; rc.20 SHIPPED D-528; #128+#130+#169+#176+#170(S17.01+S17.02+S17.03) all MERGED; E-17 delivery complete: BC-5.40.001+BC-4.13.001+BC-6.23.001 ACTIVE; ADR-025 v1.6 (Decision 12 WASM guard pass-1+pass-2 corrected; Decision 11 Mech-2 WITHDRAWN; D-539 multi-family satisfied); S-17.04 v1.4 CORRECTED (17 ACs; 8pts; SS-04+SS-05; D15/D16/D17; feature/S-17.04 @ f627a1c5 IN-FLIGHT NOT merged; branch contains v1.1 spec — must rebase to v1.4 before TDD); BC-INDEX v2.72; STORY-INDEX v3.97; ARCH-INDEX v2.25; **rc release PENDING** to ship all to operator cache. |
| v1.0-feature-engine-discipline-pass-1 | feature | **PAUSED** | F5 pass-75 adversary complete D-510 2026-05-27; META-LEVEL-30 CANDIDATE-CONFIRMED; trajectory →9→9→9→11 (tick-up from 35-consecutive 9s; 14-day pause cost); 4 structural ACCEPTED-AT-FLOOR per D-386 Option C extension; S-15.17 anchors HIGH-002 cure; L-EDP1-067 captured; BC-INDEX v2.53; STORY-INDEX v3.71. Full-cycle trajectory (75 values ending): →9→9→9→9→11. |
| v1.0-feature-plugin-async-semantics-pass-1 | feature | CLOSED | All PRs merged; rc.14 shipped |

## Decisions Log

> D-001..D-549: `cycles/v1.0-brownfield-backfill/decision-log.md` + `decisions-log-archive.md` (D-550+D-551+D-552 also in decision-log.md SoT)
> F5 pass-2 architect decisions: `cycles/v1.0-feature-engine-discipline-pass-1/F5-pass-2-architect-decisions.md` (factory-artifacts 7b83ef58)
> D-379..D-454 (F5): `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md` <!-- D-452(e) umbrella-range-auto-advance; D-511..D-552 per-burst D-range advances archived to decision-log.md; D-552 ADR-025-PASS-2-CORRECTIONS 2026-06-12 D-range→D-552 -->

| ID | Decision | Phase | Date |
|----|----------|-------|------|
| D-552 | ADR-025 v1.6 GEMINI CROSS-FAMILY ADVERSARY PASS 2 CORRECTIONS + S-17.04 v1.3→v1.4 CODIFIED 2026-06-12 — §12.3 LockExpiryStale table expanded (lock-held + absent/empty proposed expires_at → Block; closes enforcement asymmetry); §12.7 R6 step 5 added (`..` segment-stack resolution in canonical-path algorithm); §12.9 clarity note (timestamp: sole guard-gated field; last_amended: POLICY-14 discipline only); verify-factory-lock tool matcher +MultiEdit (lock-identity guard parity); SKILL.md +MultiEdit; bats plugins_run=1 guard-ran assertion; impl green 1d92d847 28/28 unit + 6/6 bats; D-539 multi-family obligation satisfied; S-17.04 v1.3→v1.4 (AC-016/017 added; EC-006 `..` resolution; AC-006 scope note; AC-001/010 amended; Red Gate 19→24 tests); ARCH-INDEX v2.24→v2.25; STORY-INDEX v3.96→v3.97; BC impact NONE; D-chain cite D-551 per D-419(b); parent-commit ce277f92. | D-552 pass-2 corrections; ARCH-INDEX v2.25; STORY-INDEX v3.97; BC impact NONE; D-539 satisfied | adversary-pass-2-correction | 2026-06-12 |
| D-551 | ADR-025 v1.6 ADVERSARY-PASS-1 CORRECTIONS + S-17.04 v1.2→v1.3 CODIFIED 2026-06-12 — ADR-025 v1.6 Decision 12 payload-field fix (new_content→reconstruct semantics; Write tool_input.content / Edit old_string+new_string reconstruct / MultiEdit edits[] sequential; new_content 0× in 5000+ real events validated Perplexity + dispatcher-log); registry caps path_allow-only (ReadFileCaps deny_unknown_fields; max_bytes+timeout_ms removed); explicit priorities 142/143; canonical-path normalization; block_with_fix message format; tool matcher Edit|Write|MultiEdit; S-17.04 v1.2→v1.3 (AC-005/006/010/EC-006 corrected + AC-011..015 added; Red Gate 12→19 tests); LOCAL adversary streak 0/3 (C2/H4/M4/L1; re-cascade pending); BC impact NONE; ARCH-INDEX v2.23→v2.24; STORY-INDEX v3.95→v3.96; D-chain cite D-550; parent-commit 8f19bab2. | ADR-025 v1.6 pass-1 corrections codified; S-17.04 v1.3; ARCH-INDEX v2.24; STORY-INDEX v3.96; LOCAL 0/3 | adversary-pass-1-correction | 2026-06-12 |
| D-550 | ADR-025 v1.5→v1.6 + S-17.04 v1.1→v1.2 REDIRECT (human-approved) 2026-06-11 — Decision 11 Mechanism 2 bash gate WITHDRAWN; Decision 12 verify-state-timestamp-refresh WASM PreToolUse guard ADOPTED (fires on Edit/Write to .factory/STATE.md; blocks TimestampStale + LockExpiryStale; on_error=continue fail-open); push-time cas-push chokepoint dropped; D15 factory-lock-parse shared crate + D16 guard + D17 tests; S-17.04 v1.2 8pts SS-04+SS-05; BC impact NONE; ARCH-INDEX v2.22→v2.23; STORY-INDEX v3.94→v3.95; D-chain cite D-549; parent-commit 29ee394b. | D-550 ADR-025 v1.6; S-17.04 v1.2; ARCH-INDEX v2.23; STORY-INDEX v3.95; OPEN DESIGN DECISION RESOLVED | architecture-redirect | 2026-06-11 |
| D-549 | SESSION-END DURABILITY BURST + S-17.04 SPEC-EVOLUTION CODIFIED 2026-06-11 — ADR-025 v1.4→v1.5 (F-1701-001 gate-trigger: primary factory-cas-push + secondary git push; block-message one-liner; D12 jq 4th-footgun); S-17.04 v1.0→v1.1 (AC-002 belt-and-suspenders; 4 EC fail-open Red Gate tests; count 8→12); ARCH-INDEX v2.21→v2.22; STORY-INDEX v3.93→v3.94; #170/E-17 COMPLETE (D-544+D-545+D-547 all merged); S-17.04 IN-FLIGHT feature/S-17.04 @ f627a1c5; OPEN DESIGN DECISION (Mechanism 2 chokepoint A/B/C pending user choice); D-chain cite D-548; parent-commit cedeb825. | D-549 SESSION-END; ADR-025 v1.5; S-17.04 v1.1; ARCH-INDEX v2.22; STORY-INDEX v3.94; #170 COMPLETE; S-17.04 IN-FLIGHT; OPEN DESIGN DECISION | session-end-durability | 2026-06-11 |
| D-548 | ADR-025 v1.3→v1.4 + S-17.04 AUTO-RENEW WIRING CODIFIED 2026-06-11 — ADR-025 Decision 11 (auto heartbeat renewal: SKILL renew step + verify-lock-renewal.sh PreToolUse gate); Decision 5 corrected; D10–D14 deliverables; BC-5.40.001 PC4 UNAFFECTED; S-17.04 authored E-17 wave 4 5pts 7ACs; ARCH-INDEX v2.20→v2.21; STORY-INDEX v3.92→v3.93; develop stale-checkout resolved 60fd0233; D-chain cite D-547; parent-commit 0f122e70. | ADR-025 v1.4 Decision 11; S-17.04 E-17 wave 4; ARCH-INDEX v2.21; STORY-INDEX v3.93 | story-authoring | 2026-06-11 |
| D-547 | S-17.03 DELIVERED/MERGED 2026-06-11 — PR #183 squash-merged 60fd0233; CI run 27343001859 26/26 bats 5-platform all-green; LOCAL adversary 3-CLEAN (refusal-msg guard-parity + CRLF cross-component parity + subshell CRLF temp-file leak); security 0-findings; pr-reviewer APPROVE; /factory-lock + /factory-unlock skills + 3-state lock-status in factory-health/factory-worktree-health via shared factory-lock-status.sh; 3 bats-tested bin helpers reusing S-17.01 factory-lock-write.sh/factory-cas-push.sh/emit-event; BC-6.23.001 POL-14 draft→active v1.1→v1.2; issue #170 CLOSED; E-17 (Factory State Durability & Concurrency) 3/3 DELIVERED; BC-INDEX v2.71→v2.72; STORY-INDEX v3.91→v3.92; D-chain cite D-546; parent-commit 2d5b1c98. | S-17.03 MERGED PR #183 60fd0233; BC-6.23.001 POL-14 active; issue #170 CLOSED; E-17 3/3 COMPLETE | story-merge-closure | 2026-06-11 |
| D-546 | S-17.03 v1.0→v1.1 EXECUTABLE-HELPER REFINEMENT 2026-06-11 — delivery-prep applied S-17.01 precedent: 3 bin/ helpers (factory-lock-status.sh shared three-state display, factory-lock-acquire-precheck.sh, factory-unlock-decide.sh) + 3 bats; SKILL.md thin orchestrators delegating STATE.md write to state-manager via S-17.01 helpers; BC-6.23.001 UNCHANGED; 14 ACs preserved; STORY-INDEX v3.90→v3.91; D-chain cite D-545; parent-commit e9a22a0b. | S-17.03 v1.1 executable-helper model: 3 helpers + 3 bats; BC-6.23.001 UNCHANGED; 14 ACs preserved; STORY-INDEX v3.90→v3.91 | story-refinement | 2026-06-11 |
| D-545 | S-17.02 DELIVERED/MERGED 2026-06-11 — PR #182 squash-merged df4f26b8; CI 13/13 bats green; trend 1H+2M+4L→1M→0→0→0 3-CLEAN; pr-reviewer APPROVE; feature DELETED+VERIFIED; develop c64b46d2→df4f26b8; BC-4.13.001 POL-14 draft→active; ADR-025 v1.2→v1.3 (env_allow footgun — 3rd silent-no-op vector); issue #170 partial-close (S-17.03 remains); STORY-INDEX v3.89→v3.90; BC-INDEX v2.69→v2.70; ARCH-INDEX v2.19→v2.20; D-chain cite D-544; parent-commit 37414e5a. | S-17.02 MERGED PR #182 df4f26b8; BC-4.13.001 POL-14 active; ADR-025 v1.3; develop df4f26b8; E-17 2/3; issue #170 partial-close | story-merge-closure | 2026-06-11 |
| D-544 | S-17.01 DELIVERED/MERGED 2026-06-11 — PR #181 squash-merged c64b46d2; CI 22/22 bats green; trend 9→3→0→0→0 3-CLEAN; pr-reviewer APPROVE; feature DELETED+VERIFIED; develop 0f4793f1→c64b46d2; BC-5.40.001 POL-14 draft→active; issue #170 REOPENED (S-17.02 Wave 2 next); STORY-INDEX v3.87→v3.88; BC-INDEX v2.66→v2.67; D-chain cite D-543; parent-commit b84a6886. | S-17.01 MERGED; BC-5.40.001 POL-14 active; develop c64b46d2; E-17 1/3; issue #170 REOPENED | story-merge-closure | 2026-06-11 |
| D-543 | S-17.01 v1.0→v1.1 EXECUTABLE-HELPER REFINEMENT 2026-06-10 — delivery-prep Red-Gate-feasibility defect: v1.0 tested prose (SKILL.md + state-manager.md) with Rust-style test names + no host module; refined per L-issue-169-176-worktree-identity(b): factory-lock-write.sh (D3) + factory-cas-push.sh (D6) under plugins/vsdd-factory/bin/; factory-lock-write.bats + factory-cas-push.bats under tests/; all 10 ACs + BC-5.40.001 PC/EC traces UNCHANGED; STORY-INDEX v3.85→v3.86; D-chain cite D-542; parent-commit 0601fdb1. | S-17.01 v1.1 executable-helper model: factory-lock-write.sh+factory-cas-push.sh+bats; STORY-INDEX v3.85→v3.86; BC-5.40.001 UNCHANGED | story-refinement | 2026-06-10 |
| D-542 | STORY-DECOMPOSITION FOR ISSUE-170 FACTORY LOCK 2026-06-10 — epic E-17 (Factory State Durability and Concurrency) + 3 stories: S-17.01 (factory_lock schema+CAS; 5pts; BC-5.40.001; W1; SS-05; acyclic), S-17.02 (verify-factory-lock WASM guard; 8pts; BC-4.13.001; W2; SS-04; deps [S-17.01]), S-17.03 (/factory-lock+/factory-unlock+factory-health; 8pts; BC-6.23.001; W3; SS-06; deps [S-17.01, S-17.02]); 21pts/39ACs; STORY-INDEX v3.84→v3.85; D-chain cite D-541; parent-commit ba471c58. | epic E-17 + 3 stories S-17.01/02/03 (21pts; acyclic); STORY-INDEX v3.84→v3.85 | story-decomposition | 2026-06-10 |
| D-541 | BC-AUTHORING FOR ISSUE-170 FACTORY LOCK/LEASE 2026-06-10 — 3 BCs authored draft per ADR-025 v1.2: BC-4.13.001 (verify-factory-lock WASM guard, SS-04, 8PCs, 15ECs, 10TVs); BC-5.40.001 (factory_lock schema+TTL+CAS, SS-05, 6PCs, 9ECs); BC-6.23.001 (/factory-lock+/factory-unlock+factory-health, SS-06, 8PCs, 10ECs, 10TVs); CAP-031 registered capabilities.md v1.3; BC-INDEX v2.65→v2.66; SS-04 39→40 SS-05 656→657 SS-06 586→587 total_bcs 1955→1958; VP IDs TBD (TD-VSDD-063); POLICY 8 propagation deferred to implementing-story; 4-index BC bumped VP/STORY/ARCH UNCHANGED; D-chain cite D-540; parent-commit c7277468. | bc-authoring | 2026-06-10 |
| D-540 | ADR-025 ADOPTED FOR ISSUE-170 FACTORY LOCK/LEASE DESIGN 2026-06-10 — ADR-025 v1.2 ACCEPTED: local native-WASM PreToolUse guard verify-factory-lock as primary enforcement (frontmatter factory_lock block, git-email identity, block-mutations/allow-reads, TTL 45min mid-burst-renewed + audited force-unlock break-glass, fail-open-on-crash); --force-with-lease push-CAS complementary mitigation; git-ref refs/factory-lock CAS deferred; NO dispatcher-binary/host-ABI change (host_abi=1 unchanged); research-verified APPROVE-WITH-FIXES all 5 fixes landed; 9 deliverables enumerated; ARCH-INDEX v2.18→v2.19; 4-index BC/VP/STORY UNCHANGED; human-approved for implementation; D-chain cite D-539 per D-419(b); parent-commit ba6844c1 per D-419(b). | adr-adoption | 2026-06-10 |
| D-539 | ISSUE-169+176 WORKTREE-IDENTITY COUPLE MERGED 2026-06-10 — PR #180 "fix(adversary): worktree-identity engine fix — eliminate phantom findings (#169 + #176)" SQUASH-MERGED 0f4793f1; CI run 27309724791 11/11 GREEN (cargo-host ubuntu+macos, 5× build-dispatcher, validate, SAST, platforms-drift); issues #169+#176 AUTO-CLOSED; feature/issue-169-176-worktree-identity DELETED+VERIFIED (git ls-remote --exit-code exit 2); develop 89fbe2d6→0f4793f1; POL-14 no-op; 4-index ALL UNCHANGED (BC-INDEX v2.65, VP-INDEX v2.06, STORY-INDEX v3.84, ARCH-INDEX v2.18); requires rc release for operator cache (agents/adversary.md + skills/adversarial-review/SKILL.md + skills/deliver-story/steps/shared-context+step-d5-adversary-convergence + NEW bin/resolve-worktree-identity.sh + 2 NEW bats test files); multi-family 3-CLEAN at LOCAL SHA 5ea02ecf (Gemini cross-family 7-iter ~20 defects + Claude canonical 3-pass CRITICAL CWD-relative repo-root bug caught); lesson L-issue-169-176-worktree-identity codified; D-538 checkpoint archived to session-checkpoints.md; D-chain cite D-538 per D-419(b); parent-commit 9eb53aab per D-419(b). | issue-169-176-merge-closure | 2026-06-10 |
| D-527..D-538 archived | **COMPACTED 2026-06-11 per D-430(a)** | D-527 SESSION-END; D-528 rc.20 SHIPPED e9e38286; D-529 MAINT SWEEP; D-530 E-10 pass-16 LOW PR #168; D-531 E-10 CASCADE SEALED; D-532 SESSION-END; D-533 issue-validation; D-534 #128 in-flight; D-535 #128 MERGED f6ce4b7c; D-536 ADR-024 ADOPTED; D-537 #130 MERGED 89fbe2d6 ADR-024 v1.2; D-538 SESSION-END. Full rows: decision-log.md SoT. |
| D-527+D-528 archived | **COMPACTED 2026-06-10 per D-430(a)** | D-527 SESSION-END DURABILITY BURST 2026-05-31; D-528 v1.0.0-rc.20 SHIPPED 2026-06-01 (PR #166 --merge e00ab1ab; tag e9e38286; main 2a191314; marketplace #12; plugin 52→53). Full rows in decision-log.md SoT. |
| D-499..D-509 archived | **COMPACTED 2026-06-08 per D-430(a)** | 11 rows archived. Full rows: `git show 688dd1c2:.factory/STATE.md` lines 249-259. |
| D-413..D-498 archived | **COMPACTED 2026-05-27 per D-430(a)** | 36 rows archived. Full content: decision-log.md (F5 + brownfield). Pre-compaction state: `git show 20cb8e1c:.factory/STATE.md`. |
| D-510+D-522+D-525+D-526 archived | **COMPACTED 2026-06-10 per D-430(a) D-542 burst** | 4 rows archived to decision-log.md SoT (D-510 F5 pass-75; D-522 S-15.17 SEALED; D-525 ADR-023; D-526 S-15.17 SHIPPED). Full rows in decision-log.md SoT. |

## Skip Log

| Step | Skipped? | Justification |
|------|----------|---------------|
| UX Spec | yes | CLI-only product with no UI surfaces |
| Gene Transfusion Assessment | yes | Not applicable — engine and product are same repo |

## Blocking Issues

<!-- No open blockers on active stories. -->

## Drift Items / Tech Debt

| Item | Status | Notes |
|------|--------|-------|
| **TD #67** 4 timing-flaky e2e tests | **RESOLVED 2026-05-15 PR #143 + RECURRENCE RESOLVED 2026-05-31 PR #165 f34b7567** | F-P3-008 pattern fully resolved across all known bats suites. |
| **TD #68/69/70/71/72/74** | **ALL RESOLVED** 2026-05-13/14/15 | See PRs #114/#116/#117/#140/#138/#139/#141. |
| Ghost BCs: BC-3.07.003/004, BC-1.06.011 | DEFERRED | Missing from BC-INDEX; investigate in future fix-burst |
| **TD-VSDD-061 (F-P6-002)** | OPEN 2026-05-17 | validate-index-cite-refresh + validate-burst-log have `host::read_file(...65536...)` callsites against files >64KiB → silent fail-open. RECOMMENDED ACTION: follow-up story targeting both crates to raise max_bytes to 524288 + add oversize regression tests. |
| **TD-VSDD-062/063** | OPEN 2026-05-17/19 | Schema inconsistencies in M2 stories (LOW); deferred VP allocation for BC-5.39.006 9 pending VPs. |
| **PG-S-15.11-bats-prod-registry-parity-gate** | OPEN 2026-05-17 | Bats inline `path_allow` arrays must be byte-identical to production hooks-registry.toml entry. Target: S-15.03 PRIORITY-A automation wave (CI lint). |
| **TD-VSDD-095..100 (CODIFIED-LESSONS)** | CODIFIED-AND-FORWARDED-TO-SK-MCP-001 2026-05-17/18 | 6-class META-LEVEL perimeter; TDD micro-commit + registry-priority + compaction-burst-sibling-sweep + own-burst-log-structural-integrity + dim2-pc-must-read-production disciplines. |
| **TD-VSDD-101 (CI env-var paper-fix)** | OPEN 2026-05-18 — anchored S-15.15 | `VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1` env-var skips production STATE.md bats test in CI. Structural fix options: (a) mount factory worktree; (b) capability-check skip; (c) local-only harness. |
| **S-15.17-CR-001** | ACCEPTED-DEFERRED 2026-05-31 | `check_index_sites` uses `has_trajectory_tail` on full table rows (advisory-arm only; unreachable in production). Revisit if INDEX.md layout changes. |
| **S-15.17-CR-002** | ACCEPTED-DEFERRED 2026-05-31 | `rows_after_heading` duplicate-heading `continue` branch does not reset `seen_separator` (advisory-arm only; impossible in production). Revisit if INDEX.md gains duplicate headings. |
| **test_F_P2_001 timing flake** | OPEN 2026-06-08 | darwin-x64 test_F_P2_001 observed at 3761ms vs 3000ms threshold; same class as TD #67/F-P3-008 (wall-clock assertion); PR #165 fixed TC-9 sibling only; this test not yet de-flaked. Candidate de-flake follow-up story (same strategy: event-observation structural rewrite). D-532 capture. |
| **RUSTSEC-2026-0149** | OPEN 2026-06-11 — wasmtime-wasi HIGH | `cargo audit` on PR #182 CI: RUSTSEC-2026-0149 wasmtime-wasi HIGH advisory (path_open TRUNCATE bypass; CVE pending). Pre-existing; not introduced by S-17.02. Fix: wasmtime >= 44.0.2 required; awaiting upstream compatibility validation. Anchor: next rc release cycle or explicit cargo-audit remediation story. |
| **O-PASS16-002 header stale doc-comment** | OPEN 2026-06-08 | validate-trajectory-tail-cell-completeness src: `extract_per_pass_trajectory_flag`/`check_state_md_with_flag` function doc-comment headers still describe old extraction approach (hook shipped green + correct, but header comment stale). Cosmetic cleanup on next spec-touch of S-15.17 or next adversary sweep. D-532 capture. |
| **F-P3-007 / F-P4-001 / F-P4-002** | OPEN-DEFERRED 2026-05-17 | STATE.md `phase:` field cap; story v1.1 PC numbering; BC v1.2 changelog phrasing. Anchor: next BC-5.39.006 amendment. |
| **L-EDP1-067-CANDIDATE-INV-015** | FORWARDED-TO-SK-MCP-001-APPENDIX-D 2026-05-18 | Adversary-fresh-context-must-grep-canonical-source. |

## Historical Content

- `cycles/v1.0-brownfield-backfill/burst-log.md` | `session-checkpoints.md` | `lessons.md`
- `cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | `session-checkpoints.md` | `lessons.md`
- `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md` (adversary reviews at `S-12.03/`, `S-12.04/`, `S-12.05/` subdirs)

## Session Resume Checkpoint (2026-06-12 — D-552 ADR-025 v1.6 pass-2 corrections + S-17.04 v1.4 CODIFIED; ARCH-INDEX v2.25; STORY-INDEX v3.97; LOCAL adversary streak 0/3; resume = S-17.04 v1.4 TDD re-implementation + LOCAL adversary re-cascade)

> **SELF-SUFFICIENT RESUME CONTEXT FOR ZERO-CONTEXT NEW SESSION ON A DIFFERENT MACHINE**
> Read this section alone to resume the orchestrator after full CLEAR, new session, or new machine. All context needed is here.
> Assumes ZERO prior context. Every decision, directive, and anchor is stated explicitly below.

### §1. Where We Are

**ADR-025 v1.6 GEMINI CROSS-FAMILY ADVERSARY PASS 2 CORRECTIONS CODIFIED (D-552 2026-06-12).** D-539 multi-family adversary obligation SATISFIED. Three substantive corrections from Gemini Flash agy cross-family pass (R2/R4/R1 slices): (R2) LockExpiryStale enforcement asymmetry closed — lock-held + proposed `expires_at` absent OR empty now Blocks LockExpiryStale (previously only byte-identical stale triggered Block; absent/empty cases slipped through as Continue); §12.3 5-row table replaces 3-row. (R4) `..` segment-stack resolution added to canonical-path algorithm as step 5: split on `/`, push non-`..` segments, pop on `..` (above-root escape silently discarded per fail-open); §12.7 R6 step 5 added. (R1) verify-factory-lock tool matcher `Edit|Write|Agent` → `Edit|Write|MultiEdit|Agent` codified in §12.9 R1 directive (lock-identity guard parity with verify-state-timestamp-refresh). Two secondary items: (R3) SKILL.md anti-pattern row `Edit/Write` → `Edit/Write/MultiEdit`; (R5) bats allow-path tests must assert `plugins_run=1` (on_error=continue means crash exits 0 — exit-code alone insufficient). Clarity note added (§12.9): `timestamp:` is sole independently-gated freshness field; `last_amended:` is POLICY-14 state-manager discipline, NOT independently gated. LOCAL adversary cascade streak **still 0/3** — re-cascade REQUIRED after TDD re-implementation with v1.4 spec.

**S-17.04 v1.4 CODIFIED — TDD RE-IMPLEMENTATION NEXT.** Branch `feature/S-17.04-mid-burst-heartbeat-renewal-wiring` @ `f627a1c5` still pushed to origin. Worktree at `.worktrees/S-17.04`. **Note:** the branch still contains the v1.1 spec (bash-gate approach). The worktree must be rebased/force-updated to match the v1.4 story spec before test-writer dispatch. S-17.04 v1.4 deliverables: D10 (SKILL renew step retained), D13 (state-manager.md cross-ref updated for D16), D15 (`factory-lock-parse` shared crate), D16 (`verify-state-timestamp-refresh` WASM plugin + registry; correct payload fields per AC-011..015 + AC-016/017; `..` normalization; verify-factory-lock +MultiEdit), D17 (19 Rust unit + 5 bats tests). 17 ACs, 8 pts, SS-04+SS-05.

**ADR-025 v1.6 DECISION 12 CORRECTED+EXTENDED (content revision, 2026-06-12).** Per-tool payload extraction: Write→`tool_input.content` (full file body); Edit→on-disk STATE.md with `old_string` replaced by `new_string` (first occurrence; `replace_all` honored); MultiEdit→on-disk STATE.md with each `edits[]` element applied sequentially. Trigger field is always `file_path` (never `new_content`). Registry `[hooks.capabilities.read_file]` is `path_allow`-ONLY. Priorities: verify-factory-lock=142, verify-state-timestamp-refresh=143. Canonical-path step 5: `..` segment-stack resolution. LockExpiryStale: blocks when lock held AND proposed `expires_at` absent OR empty OR byte-identical stale.

**#170/E-17 (Factory State Durability & Concurrency) COMPLETE.** 3 stories MERGED: S-17.01 PR #181 c64b46d2 + S-17.02 PR #182 df4f26b8 + S-17.03 PR #183 60fd0233. BC-5.40.001+BC-4.13.001+BC-6.23.001 ALL ACTIVE (POL-14). Issue #170 CLOSED D-547 2026-06-11. S-17.04 is the enforcement-wiring follow-up (E-17 wave 4, not part of #170 closure).

- **D-range:** D-001..D-552.
- **develop HEAD:** `60fd0233` (D-547 PR #183 S-17.03 squash-merge 2026-06-11).
- **4-index (post-D-552):** BC-INDEX v2.72, VP-INDEX v2.06, STORY-INDEX v3.97, ARCH-INDEX v2.25.
- **BC content:** BC-5.39.005 v1.3 + BC-5.39.006 v1.7 + BC-5.39.007 v1.6 + BC-5.39.008 v1.5 + BC-5.39.009 v1.9 + BC-7.04.051 v1.1 + BC-4.13.001 v1.3 + BC-5.40.001 v1.1 + BC-6.23.001 v1.2 — all ACTIVE. policies.yaml v1.3.6 SEALED.

### §2. Operating Mode

- vsdd-factory brownfield-onboarding; cycle `v1.0-brownfield-backfill`; self-referential.
- **E-10 CASCADE FULLY SEALED D-531** (2026-06-01; pass-16 asymptotic-acceptance; resumption gate = engine-surface material change). **Do NOT resume E-10 without material change.**
- **F5 PAUSED D-386 Option C** (2026-05-13; trajectory →9→9→9→11). **Do NOT resume without explicit human direction.**
- **S-15.03 PRIORITY-A COMPLETE D-508** (2026-05-27; 11 stories; 40pts). **RC.20 SHIPPED D-528** (2026-06-01; tag e9e38286; marketplace PR #12).

### §3. User Directives (Carry Across CLEAR)

ALL ACTIVE AND MANDATORY on every dispatch:
- **TD-VSDD-097-EXT:** current_step: MUST satisfy ALL 5 BC-5.39.006 v1.7 PCs simultaneously.
- **TD-VSDD-099:** Every burst-log entry MUST include all 4 Dim blocks (Dim-2+Dim-5+Dim-6+Dim-7); Dim-6 MUST contain literal-shell count with captured stdout.
- **TD-VSDD-100:** Dim-2 PC attestations MUST read production artifact (`grep ^current_step: .factory/STATE.md`); synthetic echo/printf FORBIDDEN.
- **POLICY 14 5-leg quintuple parity MANDATORY** on all BC/VP/story/epic version bumps: (1) version: frontmatter, (2) body Changelog row, (3) modified[] array, (4) last_amended: text-prefix, (5) upstream-index body-table cells.
- **Verification_step 7** literal-shell 4-index gate MANDATORY (D-494).
- **INV-019 cure (a)/(b)/(c) MANDATORY** in ALL BC changelog rows AND persisted adversary reports.
- **INV-020 / POLICY 14:** Cross-BC parity sweep whenever ANY BC in a group is modified.
- **Adversary MUST grep `origin/develop` or `factory-artifacts`** for literal-shell evidence (NOT stale local main; per L-EDP1-067-CANDIDATE).
- **Cure-extension parsimony (D-497):** EXTEND existing cure for same-class META-LEVEL recurrence; no new INV-N abstraction.
- **POLICY 8 v1.3 EC-mirror routing-rule (D-517); bidirectional AC↔PC parity (D-515+D-516); POLICY 5 v1.3.1/v1.3.3/v1.3.4/v1.3.5/v1.3.6 SDK-grounding + sibling-sweep mandates.**
- **D-537 [process-gap] spec-drift routing:** When TDD fix changes ADR-specified behavior, route architect ADR amendment in SAME burst. Codified ADR-024 v1.2 Process note.
- **D-539 multi-family adversary obligation:** prompt-contract + shell-logic issues require cross-family AND same-family Claude adversary passes before convergence.
- **D-552 carry:** ADR-025 v1.6 PASS-2 CONTENT-ADDITIONS (§12.3 LockExpiryStale absent/empty → Block; §12.7 R6 step 5 `..` segment-stack; §12.9 clarity note + verify-factory-lock +MultiEdit; D-539 multi-family SATISFIED). S-17.04 v1.4 CODIFIED. feature/S-17.04 @ f627a1c5 PUSHED (branch contains v1.1 spec — must rebase to v1.4 before TDD re-dispatch). ARCH-INDEX v2.25. STORY-INDEX v3.97. LOCAL adversary streak 0/3 — do NOT dispatch test-writer/implementer until worktree rebased to v1.4 spec.
- **D-551 carry (prior):** ADR-025 v1.6 CONTENT-CORRECTED (Decision 12 payload-field fix; registry path_allow-only; priorities 142/143; canonical-path normalization step 1-4; block_with_fix format; tool=Edit|Write|MultiEdit). See D-552 carry for step 5 `..` extension.
- **D-550 carry (prior):** OPEN DESIGN DECISION RESOLVED — do NOT re-open without explicit human direction.
- **D-541 carry (partial):** VP IDs TBD per TD-VSDD-063. POLICY 8 propagation — BC-6.23.001 now ACTIVE per POL-14.

### §4. Tier-A Completed Log (most recent first)

- **D-552 (2026-06-12):** ADR-025 v1.6 GEMINI CROSS-FAMILY ADVERSARY PASS 2 CORRECTIONS. S-17.04 v1.3→v1.4. ARCH-INDEX v2.24→v2.25. STORY-INDEX v3.96→v3.97. D-539 multi-family SATISFIED. LOCAL adversary streak 0/3. Re-cascade pending.
- **D-551 (2026-06-12):** ADR-025 v1.6 ADVERSARY-PASS-1 CORRECTIONS. S-17.04 v1.2→v1.3. ARCH-INDEX v2.23→v2.24. STORY-INDEX v3.95→v3.96. LOCAL adversary streak 0/3. Re-cascade pending.
- **D-550 (2026-06-11):** ARCHITECTURE REDIRECT. ADR-025 v1.5→v1.6. S-17.04 v1.1→v1.2. ARCH-INDEX v2.23. STORY-INDEX v3.95. OPEN DESIGN DECISION RESOLVED.
- **D-549 (2026-06-11):** SESSION-END DURABILITY BURST. ADR-025 v1.4→v1.5. S-17.04 v1.0→v1.1. ARCH-INDEX v2.22. STORY-INDEX v3.94.
- **D-547 (2026-06-11):** S-17.03 DELIVERED/MERGED PR #183 60fd0233. BC-6.23.001 ACTIVE. issue #170 CLOSED. E-17 3/3 COMPLETE.
- **D-544+D-545 (2026-06-11):** S-17.01+S-17.02 MERGED PRs #181 c64b46d2 + #182 df4f26b8. BC-5.40.001+BC-4.13.001 ACTIVE. ADR-025 v1.3 env_allow footgun.
- **D-532..D-543 (2026-06-08..2026-06-11):** SESSION-END; #128 MERGED f6ce4b7c; #130 MERGED 89fbe2d6 ADR-024 v1.2; #169+#176 MERGED 0f4793f1; ADR-025 v1.2 ADOPTED; 3-BCs-AUTHORED; E-17+stories-AUTHORED; S-17.01+S-17.03 v1.1 helpers. Full rows: decision-log.md SoT.
- **D-531 (2026-06-01):** E-10 CASCADE SEALED. D-528 RC.20 SHIPPED e9e38286. D-526 S-15.17 SHIPPED PR #164. D-508 S-15.03 PRIORITY-A COMPLETE.

### §5. Cumulative Codifications

- F5: D-379..D-454 (76 decisions) — `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`.
- Brownfield: D-001..D-552 — `cycles/v1.0-brownfield-backfill/decision-log.md`. Latest: D-551 ADR-025 v1.6 ADVERSARY-PASS-1 CORRECTIONS; **D-552 ADR-025 v1.6 GEMINI CROSS-FAMILY ADVERSARY PASS 2 CORRECTIONS + S-17.04 v1.3→v1.4 2026-06-12 — §12.3 LockExpiryStale absent+empty → Block; §12.7 R6 step 5 `..` segment-stack; verify-factory-lock +MultiEdit; bats guard-ran R5; clarity note; impl green 1d92d847; D-539 multi-family SATISFIED; ARCH-INDEX v2.24→v2.25; STORY-INDEX v3.96→v3.97; LOCAL streak 0/3.**

### §6. Cumulative Lessons

- F5: L-EDP1-001..067 — `cycles/v1.0-feature-engine-discipline-pass-1/lessons.md`.
- Brownfield: TD-VSDD-095..100 + L-M3-BC-cascade + L-E10-pass15 + L-banner-format-drift + L-rc19 + L-S-15.17-SP1..SP9 + L-F-P3-008 + L-session-2026-05-31 + L-session-2026-06-01-rc20 + L-E10-pass16 + L-E10-SEAL + L-session-2026-06-08 + L-issue-128 + L-issue-130 + **L-issue-169-176-worktree-identity** — `cycles/v1.0-brownfield-backfill/lessons.md`.

### §7. S-15.03 PRIORITY-A Scope

11-story wave S-15.06..S-15.16. **ALL SHIPPED D-508. 40pts M3 total. COMPLETE.**

### §8. 4-Index State

| Index | Version | Notes |
|-------|---------|-------|
| BC-INDEX | v2.72 | D-547 BC-6.23.001 POL-14 draft→active (PR #183 S-17.03 merged); total_bcs 1958 UNCHANGED |
| VP-INDEX | v2.06 | UNCHANGED (18 VPs pending architect per TD-VSDD-063) |
| STORY-INDEX | v3.97 | D-552: S-17.04 v1.3→v1.4 (adversary pass-2 corrections; 17 ACs; 24 Red Gate tests; AC-016/017 added; EC-006 `..` resolution; verify-factory-lock +MultiEdit) |
| ARCH-INDEX | v2.25 | D-552: ADR-025 v1.6 pass-2 content additions (§12.3 absent/empty expires_at → Block; §12.7 R6 step 5 `..` segment-stack; §12.9 clarity note; verify-factory-lock +MultiEdit) |

### §9. Critical Anchors

- **factory-artifacts HEAD:** `TBD-D552` D-552 burst; prior: `f7a4cc1a` D-551; prior-prior: `8f19bab2` D-550
- **develop HEAD:** `60fd0233` (D-547 PR #183 S-17.03 squash-merge 2026-06-11)
- **feature/S-17.04 HEAD:** `f627a1c5` PUSHED to origin; worktree `.worktrees/S-17.04` (contains v1.1 spec — rebase to v1.3 before TDD dispatch)
- **main HEAD:** `2a191314` (rc.20 bot binary commit 2026-06-01)
- **v1.0.0-rc.20 tag:** `e9e38286` (prerelease; marketplace PR #12 squash-merged 862e660d)
- **ADR-025 v1.6 REVISED:** `.factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md` — D-552 content additions (same version; §12.3 absent/empty expires_at → Block + §12.7 R6 step 5 `..` segment-stack + §12.9 clarity note + verify-factory-lock +MultiEdit); ARCH-INDEX v2.25
- **S-17.04 story:** `.factory/stories/S-17.04-mid-burst-heartbeat-renewal-wiring.md` v1.4 DRAFT — STORY-INDEX v3.97; E-17 W4; 17 ACs; 24 Red Gate tests (19 Rust unit + 5 bats); 8pts; SS-04+SS-05; ADR-025 v1.6 Decision 12 corrected+extended; LOCAL adversary streak 0/3
- **BC-5.40.001:** `.factory/specs/behavioral-contracts/ss-05/BC-5.40.001.md` v1.1 ACTIVE (SS-05; factory_lock schema+TTL+CAS)
- **BC-4.13.001:** `.factory/specs/behavioral-contracts/ss-04/BC-4.13.001.md` v1.3 ACTIVE (SS-04; verify-factory-lock WASM guard; env_allow PC7)
- **BC-6.23.001:** `.factory/specs/behavioral-contracts/ss-06/BC-6.23.001.md` v1.2 ACTIVE (SS-06; /factory-lock+/factory-unlock+factory-health)
- Verify on resume: `git rev-parse --short origin/develop` → expect `60fd0233`; `git -C .worktrees/S-17.04 rev-parse --short HEAD` → expect `f627a1c5`

### §10. PR Status

- **0 open source PRs. 1 feature branch open: feature/S-17.04 @ f627a1c5 (NOT merged — OPEN DESIGN DECISION gates merge).**
- **RC RELEASE PENDING** for #128+#130+#169+#176+#170(S17.01+S17.02+S17.03) to reach operator cache. S-17.04 excluded from rc until OPEN DESIGN DECISION resolved + re-converged.
- **MERGED (issue #170 E-17 3/3):** PRs #183 60fd0233 + #182 df4f26b8 + #181 c64b46d2 (all 2026-06-11; branches deleted+verified; ALL REQUIRE rc for operator cache).
- **MERGED (prior bugs):** PR #180 0f4793f1 (#169+#176) + PR #179 89fbe2d6 (#130 ADR-024 v1.2) + PR #178 f6ce4b7c (#128) — all REQUIRE rc for operator cache.

### §11. Post-CLEAR Resume Checklist (zero-context)

1. **Verify worktree state:** Main: `git rev-parse --abbrev-ref HEAD` → expect `develop` (HEAD `60fd0233`). Factory: `git -C .factory log -1` + `git -C .factory status` (expect clean; branch factory-artifacts). Feature: `git -C .worktrees/S-17.04 rev-parse --short HEAD` → expect `f627a1c5` (note: branch has v1.1 spec; REBASE to v1.4 before TDD re-dispatch).
2. **Read §1-§12 this checkpoint** (all of it).
3. **Verify trajectory-tail PC4:** `grep "^current_step:" .factory/STATE.md | grep -oE "trajectory-tail [→0-9]+"` → expect `trajectory-tail →9→9→9→11`.
4. **Verify develop HEAD:** `git rev-parse --short origin/develop` → expect `60fd0233`.
5. **E-10 CASCADE SEALED D-531.** Do NOT resume without engine-surface material change.
6. **F5 PAUSED** — trajectory →9→9→9→11. Do NOT resume without explicit human direction.
7. **RC.20 SHIPPED D-528** (run 26738809372; tag e9e38286; main 2a191314; marketplace #12; plugin 52→53). **NEXT RC RELEASE HELD until S-17.04 merges**: ships #128+#130+#169+#176+#170(S17.01+S17.02+S17.03)+S-17.04 to operator cache.
8. **LOCAL adversary streak 0/3 (D-552 — pass-2 corrections codified).** ADR-025 v1.6 Decision 12 corrected+extended + S-17.04 v1.4 codified. Worktree rebase to v1.4 → test-writer 24 Red Gate stubs → implementer D15/D16/D10/D13 → re-cascade LOCAL 3-CLEAN → PR → merge.
9. **4-index confirmed D-552:** BC-INDEX v2.72, VP-INDEX v2.06, STORY-INDEX v3.97, ARCH-INDEX v2.25.
10. **ALL dispatches carry:** TD-VSDD-097-EXT + TD-VSDD-099 + TD-VSDD-100 + POLICY 14 5-leg + verification_step 7 4-index gate + INV-019 (a)/(b)/(c) + adversary grep origin/factory-artifacts + D-449(a) literal-shell Dim-2 + POLICY 8 v1.3 parity + POLICY 5 v1.3.1/v1.3.4/v1.3.5/v1.3.6 + D-537 spec-drift routing + D-539 multi-family adversary.
11. **Latest decision D-552.** All #128+#130+#169+#176+#170(S17.01+S17.02+S17.03) MERGED; REQUIRE rc.21+ for operator reach. S-17.04 v1.4 IN-FLIGHT NOT merged (TDD re-implementation next).

### §12. Pending Work Items — Strict Resume Ordering (refreshed 2026-06-12 D-552)

| Step | Item | Tier | Gate | Status |
|------|------|------|------|--------|
| **1** | **S-17.04 v1.4 TDD re-implementation** | **implementation** | worktree rebased to v1.4 spec | Rebase feature/S-17.04 worktree to v1.4 spec; test-writer 24 Red Gate stubs (19 Rust unit + 5 bats) → implementer D15/D16/D10/D13 (correct payload fields per AC-011..015; AC-016/017 LockExpiryStale absent+empty; AC-010 +MultiEdit; `..` normalization step 5) → LOCAL adversary re-cascade 0/3 → 3-CLEAN → PR → merge |
| **2** | **rc.21 release** | **release** | S-17.04 merged | Ships #128+#130+#169+#176+#170(S17.01+S17.02+S17.03)+S-17.04 to operator cache |
| **3** | **#173 wave-checkpoint** | **implementation** | #170 CLOSED | State-durability chain: factory-artifacts orphan-branch checkpoint/restore |
| **4** | **#171 deferred-revalidate** | **implementation** | #173 done | Deferred-revalidation story |
| **5** | **#129 canonical-principle** | **implementation** | human-authorize | Ship canonical-principle in plugin |
| ~~prior~~ | ~~TD #74/66/67; S-15.03 PRIORITY-A; E-10 pass-16; rc.19+rc.20; #128+#130+#169+#176+#170-S17.01/02/03~~ | ~~—~~ | ~~—~~ | **ALL COMPLETE/MERGED** (full rows decision-log.md SoT) |
| **5c** | **F5 pass-76** | **gated** | EXPLICIT human direction | PAUSED D-386 Option C. Do NOT resume. |
| **6/7** | **UNI-PLUG-001 / SK-MCP-001** | **forward** | human-authorize | PROPOSAL REVIEW-READY |

**[D-414(c) acknowledgment: Section 12 is a non-standard addition for forward-backlog durability.]**

> Previous checkpoint (D-551 ADR-025-V1.6-PASS-1-CORRECTIONS-S-17.04-V1.3-2026-06-12) archived to: `cycles/v1.0-brownfield-backfill/session-checkpoints.md`
