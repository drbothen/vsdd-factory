---
document_type: pipeline-state
level: ops
version: "3.25"
status: draft
producer: state-manager
timestamp: 2026-06-15T00:00:00Z
phase: D-575-F2-E18-PRE-PASS-14-CONSISTENCY-SWEEP-REMEDIATION-2026-06-15
last_amended: 2026-06-15 (v3.25) — D-575 F2 E-18 PRE-PASS-14 CONSISTENCY-SWEEP REMEDIATION: VP-082-BATS-SPLIT finding closed (VP-082 v1.5→v1.6 — skeleton split into commit-failure LOCAL-only test + push-failure network test; VP-INDEX v2.17→v2.18). Consistency sweep CLEAN. 3-CLEAN streak 0/3 (pass-13 reset; unchanged). 4-index BC v2.82/VP v2.18/STORY v4.01/ARCH v2.37; D-chain cite D-574 per D-419(b); parent-commit 4dc23351 per D-419(b). [Prior: 2026-06-14 (v3.24) — D-574: F2 ADV PASS-13 NOT-CLEAN (4 findings fixed); 4-index BC v2.82/VP v2.17/STORY v4.01/ARCH v2.37. See decision-log.md SoT.]
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
project: vsdd-factory
mode: brownfield
current_step: "D-575 F2 E-18 PRE-PASS-14 CONSISTENCY-SWEEP REMEDIATION 2026-06-15 — Comprehensive consistency-validator sweep over full E-18 package returned NOT-CLEAN with 1 MAJOR: VP-082-BATS-SPLIT (VP-082.md v1.5 last_amended claimed the bats proof-harness skeleton was split into commit-failure + push-failure tests, but the body retained a single mislabeled test — a push-path setup under a commit-failure heading; the genuine push-failure test for Postcondition F / BC-7.07.001 PC6b was absent). Fixed: VP-082.md v1.5→v1.6 (architect): skeleton split into (a) commit-failure LOCAL-only test (chmod 444 .git/objects; push never attempted; HEAD unchanged; no log entry) → exit 2 Postcondition B, and (b) push-failure test (local commit + log append succeed per ADR-026 §F-P10-002, then push fails) → exit 2 Postcondition F; local commit + log retained; retry push-only. VP-INDEX v2.17→v2.18 (VP-082 row version sync). All other sweep dimensions CLEAN. 4-index BC-INDEX v2.82 VP-INDEX v2.18 STORY-INDEX v4.01 ARCH-INDEX v2.37 (literal-shell grep ^version:); trajectory-tail →9→9→11→2; 3-CLEAN streak 0/3 (pass-13 reset; unchanged); consistency sweep DONE+CLEAN; adversary pass-14 NEXT; D-chain cite D-574 per D-419(b); parent-commit 4dc23351 per D-419(b)"
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

  D-430(a) compaction (D-568 burst 2026-06-14):
  Banner tracker D-532..D-566 (35 entries) collapsed to range-reference per D-430(a).
  Decisions Log D-557..D-567 (11 rows) archived to decision-log.md SoT per D-430(a).
  §3 older carries D-549..D-560 (12 entries) retired (topics CLOSED/ARCHIVED) per D-430(a).
  §4 Tier-A Completed Log D-549..D-555 (7 entries) trimmed; reference to decision-log.md SoT.
  All content preserved via: git show ef7eafe2:.factory/STATE.md (pre-compaction D-567 state).

  Line-growth tracker (most recent; older entries archived to git show ef7eafe2:.factory/STATE.md):
  D-532..D-566 tracker entries (35 entries) archived per D-430(a) D-568 burst; preserved at: git show ef7eafe2:.factory/STATE.md lines 40-73.
  D-567-F2-E18-ADV-PASS-6-STATE-MGR-BOOKKEEPING-2026-06-14 433 lines (wc-l; +18 over soft 415; margin 500-433=67 from hard cap; D-446(c) dual-margin form).
  D-568-F2-E18-ADV-PASS-7-FIX-BURST+COMPACTION-2026-06-14 ~370 lines (wc-l; D-430(a) compaction: 35 banner entries + 11 decision rows + 12 §3 carries + 7 §4 entries archived; target; D-446(c) dual-margin form).
  D-573-F2-E18-ADV-PASS-12-CLEAN-2026-06-14 ~370 lines (wc-l; STATE.md-only burst; NO spec changes; margin 500-370=130 from hard cap; D-446(c) dual-margin form).
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
| **Last Updated** | 2026-06-15 — D-575 F2 E-18 PRE-PASS-14 CONSISTENCY-SWEEP REMEDIATION: VP-082-BATS-SPLIT closed (VP-082 v1.6; VP-INDEX v2.18); sweep CLEAN; 4-index BC v2.82/VP v2.18/STORY v4.01/ARCH v2.37; adversary pass-14 NEXT. |
| **Current Phase** | D-575 F2 E-18 PRE-PASS-14 CONSISTENCY-SWEEP REMEDIATION 2026-06-15 — Comprehensive consistency-validator sweep returned NOT-CLEAN with 1 MAJOR: VP-082-BATS-SPLIT. Fixed: VP-082.md v1.5→v1.6 (skeleton split into correct commit-failure LOCAL-only test + push-failure network test; two mutually-distinct bats test cases). VP-INDEX v2.17→v2.18 (VP-082 row version cell sync). All other sweep dimensions CLEAN. Sweep now CLEAN. 3-CLEAN streak 0/3 unchanged. NEXT: adversary pass-14 (fresh-context; need 3 consecutive clean for BC-5.39.001 convergence). |
| **Current Cycle** | v1.0-brownfield-backfill |

## Phase Progress

| Phase | Status | Artifact |
|-------|--------|----------|
| Phases 0-B, Waves 1-11, S-7.03, beta.5-7, W-14, W-15 | **COMPLETE** | `cycles/v1.0-brownfield-backfill/phase-progress-archive.md` |
| Phase D-1..D-4, Waves 12-16, E-9 v1.7 sweep | **COMPLETE** | `cycles/v1.0-brownfield-backfill/` |
| Releases rc.11..rc.18, F3/F4 E-12, S-12.03..S-12.08 | **ARCHIVED 2026-06-01 per D-430(a)** | Full rows: `git show aa1f05c9:.factory/STATE.md` lines 80-93. |
| F5 passes 3-8 cycle-level adversary + fix bursts | **COMPLETE** | Trajectory 11→9→8→7→5; F5 pass-8 verdict MEDIUM; ARCH-INDEX v1.45, D-381. |
| F5 passes 9-17 adversary + fix bursts | **ARCHIVED 2026-06-08 per D-430(a)** | 20 rows archived; trajectory pass-9→17: HIGH→MEDIUM→MEDIUM→MEDIUM→HIGH→MEDIUM→MEDIUM×3; D-382..D-392 codified. Full rows: `git show 688dd1c2:.factory/STATE.md` lines 85-106. |
| D-343..D-523 (E-10 pass-9..14, M3 cascade, S-15.03 PRIORITY-A waves, rc.19, S-15.17 cascade) | **ARCHIVED 2026-06-10 per D-430(a)** | 22 rows archived; E-10 pass-9..14 SEALED D-471; M3 BC 11-pass CONVERGED D-497; S-15.03 PRIORITY-A COMPLETE D-508; rc.19 SHIPPED d15152af; S-15.17 cascade 9-pass SEALED D-522. Full rows: `git show c62c2c03:.factory/STATE.md` lines 82-108. |
| Release v1.0.0-rc.20 | **SHIPPED 2026-06-01** at 2a191314 | PR #166 --merge e00ab1ab; tag e9e38286; marketplace PR #12 squash-merged 862e660d; plugin count 52→53. |
| POST-RC.20 MAINTENANCE SWEEP | **COMPLETE 2026-06-01** D-529 | td-74 worktree removed; Dependabot consolidated; develop b21fd358. |
| E-10 adversarial cascade | **SEALED 2026-06-01 at pass-16 (D-531)** | verdict LOW; 16-pass trend 22→…→3; asymptotic-acceptance per D-471/D-386 Option C; resumption gate = engine-surface material change. |
| D-532..D-548 (2026-06-08..2026-06-11) | **ARCHIVED 2026-06-11 per D-430(a)** | D-532..D-548 archived; S-17.01..S-17.03 MERGED; E-17 3/3 COMPLETE; ADR-025 v1.4; Full rows: decision-log.md SoT. |
| D-549..D-560 (2026-06-11..2026-06-13) | **ARCHIVED 2026-06-14 per D-430(a) D-568** | D-549 SESSION-END; D-550 ADR-025 v1.6 REDIRECT; D-551..D-555 ADR-025 v1.6 adversary corrections; D-556 S-17.04 MERGED 3b2a378c; D-557 SESSION-INTERRUPT; D-558 rc.21 RE-RELEASE; D-559 MARKETPLACE-MERGED; D-560 OPERATOR-INSTALL-VERIFIED rc.21 100% COMPLETE. Full rows: decision-log.md SoT. |
| D-561 F2 E-18 CONTEXT-DURABILITY SPEC EVOLUTION 2026-06-14 | **COMPLETE** | F1-gate APPROVED (D1–D5). ADR-026 ACCEPTED; ARCH-INDEX v2.28; VP-081..085 (VP-INDEX v2.07); 8 BCs (BC-INDEX v2.73; total_bcs 1966); CAP-032; E-18 OPEN. |
| D-562 F2 E-18 ADVERSARIAL PASS-1 FIX BURST 2026-06-14 | **COMPLETE** | 3B+6M+6m resolved: ADR-026 v1.1; VP-086 NEW; VP-INDEX v2.08; ARCH-INDEX v2.29; invariants.md v1.13 (DI-020..025); L2-INDEX v1.0.4; 8 BCs v1.1; BC-INDEX v2.74. |
| D-563 F2 E-18 ADVERSARIAL PASS-2 FIX BURST 2026-06-14 | **COMPLETE** | 2B+4M+3m resolved (phantom current_wave): ADR-026 v1.2 (Decision A/B/C); VP-081..085 v1.2 + VP-086 v1.1; 7 BCs v1.2; BC-INDEX v2.75; VP-INDEX v2.09; ARCH-INDEX v2.30. |
| D-564 F2 E-18 ADVERSARIAL PASS-3 COMPLETE-SWEEP 2026-06-14 | **COMPLETE** | 5B+4M resolved (INCOMPLETE-SIBLING-SWEEP): ADR-026 v1.3; VP-082/084/085 v1.3; invariants.md v1.14; capabilities.md v1.5; 7 BCs v1.3; BC-INDEX v2.76; VP-INDEX v2.10; ARCH-INDEX v2.31. Tree-wide gate PASS (3 sweeps). |
| D-565 F2 E-18 ADVERSARIAL PASS-4 FIX BURST 2026-06-14 | **COMPLETE** | 0B+2-3M resolved: ADR-026 v1.4; ARCH-INDEX v2.32; 8 BCs v1.4 + BC-1.15.001 v1.2; BC-INDEX v2.77; VP-084 v1.4; invariants.md v1.15. |
| D-566 F2 E-18 ADVERSARIAL PASS-5 FIX BURST 2026-06-14 | **COMPLETE** | 1B+3M resolved: ADR-026 v1.5; VP-084 v1.5; VP-INDEX v2.11; ARCH-INDEX v2.33; invariants.md v1.16 (DI-025 WASM static field-4); BC-7.07.001/5.41.003/4.14.001 v1.5; BC-INDEX v2.78. |
| D-567 F2 E-18 ADV PASS-6 STATE-MGR BOOKKEEPING 2026-06-14 | **COMPLETE** | POLICY 19 registered (policies.yaml v1.4.0); VP-084 v1.6 (cite-convention); VP-INDEX v2.12; 2 lessons; O-P6-001 codified; 4-index BC v2.79/VP v2.12/STORY v4.01/ARCH v2.34. |
| **D-568 F2 E-18 ADV PASS-7 FIX BURST + COMPACTION 2026-06-14** | **COMPLETE** | ADR-026 v1.7 (F-P7-001 payload-only EPIC-COMPLETE; F-P7-002 provenance trace); ARCH-INDEX v2.35; 4-index BC v2.79/VP v2.12/STORY v4.01/ARCH v2.35; STATE.md compacted; 3-CLEAN 0/3. |
| **D-569 F2 E-18 ADV PASS-8 FIX BURST 2026-06-14** | **COMPLETE** | BC-4.14.001 v1.6→v1.7 (F-P8-001 sole MAJOR: PC2a PAYLOAD-ONLY discriminator; stale conjunct removed); BC-INDEX v2.79→v2.80; capabilities.md v1.5→v1.6 (O-P8-001 CAP-032 cite-stability); L-F2-payload-only-discriminator-recurrence-gate lesson (O-P8-002); 4-index BC v2.80/VP v2.12/STORY v4.01/ARCH v2.35; 3-CLEAN 0/3; first clean-shot pass-8; adversary pass-9 NEXT. Trajectory P5(1B/3M)→P6(3B/4M)→P7(0B/2M)→P8(0B/1M). |
| **D-570 F2 E-18 ADV PASS-9 FIX BURST 2026-06-14** | **COMPLETE** | ADR-026 v1.7→v1.8 (F-P9-001 MEDIUM: §F-P5-002 PC8+§Decision A SUPERSEDED by SHA-pinned §F-P6-006 form); VP-INDEX v2.12→v2.13 (F-P9-002 LOW: VP-082 row completed); ARCH-INDEX v2.35→v2.36 (amendment row+provenance leg); 4-index BC v2.80/VP v2.13/STORY v4.01/ARCH v2.36; tree-wide gate PASS; 3-CLEAN 0/3 (pass-9 MEDIUM → streak reset; pass-10 = next clean-shot). Trajectory P6(3B/4M)→P7(0B/2M)→P8(0B/1M)→P9(0B/0M/1MED). |
| **D-571 F2 E-18 ADV PASS-10 FIX BURST 2026-06-14** | **COMPLETE** | BC-4.14.001 v1.7→v1.8 (F-P10-001 MAJOR: wave_id payload-only; EC-010 fail-closed); BC-7.07.001 v1.6→v1.7 (F-P10-002 MEDIUM: git push step; commit/push failure distinguished); VP-082 v1.3→v1.4 (F-P10-003 MEDIUM: Postcondition E); BC-INDEX v2.81; VP-INDEX v2.15 (VP-083/VP-081 payload-only framing); ARCH-INDEX v2.37; O-P8-002 → MANDATORY (3rd occurrence; S-18.08 hard req); 4-index BC v2.81/VP v2.15/STORY v4.01/ARCH v2.37; tree-wide gate PASS; 3-CLEAN 0/3 (pass-10 MAJOR → streak reset). Trajectory P7(0B/2M)→P8(0B/1M)→P9(0B/0M/1MED)→P10(0B/1M/2MED). |
| **D-572 F2 E-18 ADV PASS-11 FIX BURST 2026-06-14** | **COMPLETE** | VP-083 v1.2→v1.3 (F-P11-001 MAJOR: Property PAYLOAD-ONLY; fail-closed postcondition + wave_id_absent_fails_closed fixture); VP-081 v1.2→v1.3 (F-P11-002 MEDIUM: shell-caller/WASM-gate division clarified; Postcondition E + integration fixture); VP-INDEX v2.15→v2.16 (version cells updated); ADR-026 v1.9 UNCHANGED; capabilities.md UNCHANGED; O-P8-002 gate extended to VP files (L-F2-payload-only-discriminator-recurrence-gate updated; layer-propagation: BC→VP-INDEX→VP-body); tree-wide gate PASS (0 load-bearing; shell-caller attribution explicitly labeled); 4-index BC v2.81/VP v2.16/STORY v4.01/ARCH v2.37; 3-CLEAN 0/3 (pass-11 MAJOR → streak reset). Trajectory P8(0B/1M)→P9(0B/0M/1MED)→P10(0B/1M/2MED)→P11(0B/1M/1MED). |
| **D-573 F2 E-18 ADV PASS-12 CLEAN 2026-06-14** | **COMPLETE** | Pass-12 (fresh-context) CLEAN: 0 BLOCKER, 0 MAJOR, 0 load-bearing MEDIUM, 0 mis-anchor. Adversary independently re-derived full package; convergence declared. NO spec changes; NO fix burst. 4-index BC v2.81/VP v2.16/STORY v4.01/ARCH v2.37 UNCHANGED. 3-CLEAN streak 0/3→1/3 per BC-5.39.001. Trajectory →P10(0B/1M/2med)→P11(0B/1M/1med)→P12 CLEAN (1/3). |
| **D-574 F2 E-18 ADV PASS-13 NOT-CLEAN 2026-06-14** | **COMPLETE** | Pass-13 NOT-CLEAN: 4 findings (F-P13-001 BLOCKER DI-025 FIELD-4-corruption EXEMPT; F-P13-002 MEDIUM VP-082 push-leg cross-doc sync; F-P13-003 MEDIUM VP-INDEX YAML duplicate-key; F-P13-004 LOW BC-5.41.001 cite convention). Fixed: invariants.md v1.18; BC-5.41.001 v1.7; BC-INDEX v2.82; VP-082 v1.5; VP-INDEX v2.17. 3-CLEAN streak 1/3→0/3 RESET. Trajectory →P11(0B/1M/1med)→P12 CLEAN(1/3)→P13 NOT-CLEAN(0/3). 4-index BC v2.82/VP v2.17/STORY v4.01/ARCH v2.37. NEXT: consistency-validator sweep (full E-18 package) → adversary pass-14. |
| **D-575 F2 E-18 PRE-PASS-14 CONSISTENCY-SWEEP REMEDIATION 2026-06-15** | **COMPLETE** | Sweep NOT-CLEAN (1 MAJOR: VP-082-BATS-SPLIT). Fixed: VP-082 v1.5→v1.6 (skeleton split: commit-failure LOCAL-only + push-failure network; two distinct tests). VP-INDEX v2.17→v2.18 (VP-082 row version sync). All other sweep dimensions CLEAN. Sweep now CLEAN. 4-index BC v2.82/VP v2.18/STORY v4.01/ARCH v2.37. 3-CLEAN streak 0/3 unchanged. NEXT: adversary pass-14. |

## Current Phase Steps

> **Rows before pass-57 archived to** `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md` per STATE.md content-routing rules (keep last 5 only).

| Step | Agent | Status | Output |
|------|-------|--------|--------|
| F5 passes 18-60 fix bursts (archived) | state-manager | ARCHIVED | See `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md`. Passes 57-59: D-437..D-439 (META-LEVEL-12/13/14); pass-60: D-440 META-LEVEL-15 CONFIRMED. |

## Identifier Conventions

| Type | Format | Authoritative Source | Count |
|------|--------|----------------------|-------|
| Subsystem | SS-NN | `specs/architecture/ARCH-INDEX.md` | 10 |
| Behavioral Contract | BC-S.SS.NNN | `specs/behavioral-contracts/ss-NN/` | 1,966 |
| Verification Property | VP-NNN | `specs/verification-properties/VP-INDEX.md` | 86 |
| Story | S-N.MM | `stories/S-N.MM-<short>.md` | 105 file-resident + 15 stub IDs (STORY-INDEX v4.01) |
| Epic | E-N | `stories/epics/E-N-<short>.md` | 18 |
| ADR | ADR-NNN | `specs/architecture/decisions/ADR-NNN.md` | 26 |

## Story Status

105 file-resident + 15 unauthored stub IDs = 120 stories registered.

- **Merged (78):** Includes S-17.01 (PR #181 c64b46d2) + S-17.02 (PR #182 df4f26b8) + S-17.03 (PR #183 60fd0233) + S-17.04 (PR #184 3b2a378c). Full ledger: `cycles/v1.0-brownfield-backfill/merged-stories-ledger.md`
- **In-Flight (0):** —
- **Draft (29 file-resident):** S-5.07; S-10.09; S-11.00; S-14.01..S-14.09 (E-14); S-15.02; S-15.03; S-16.01..S-16.02 (E-16); and others
- **Partial (2):** S-2.05 (hook-sdk-publish); S-3.04 (emit-event-host-function) — superseded by ADR-015
- **Unauthored stub IDs (15):** S-9.01..S-9.07 (W-16); S-11.01..S-11.08 (E-11 W-17 Tier 3)
- **Withdrawn (1):** S-9.30

## Active Branches

| Branch / Tag | SHA | Notes |
|--------------|-----|-------|
| main | caf06c68 | rc.21 bot binary bundle commit 2026-06-13; prior: 2a191314 (rc.20) |
| develop | 7e99f6ef | PR #186 fix a431ff47 + release.yml sync back-merge 2026-06-13; prior: 3b2a378c (D-556) |
| factory-artifacts | [D-575 commit — see §9] | D-575 pre-pass-14 consistency-sweep remediation 2026-06-15 (VP-082 v1.6 + VP-INDEX v2.18 + STATE.md); prior: 2ebaca02 D-574 |
| v1.0.0-rc.21 (tag) | 03054524 | SHIPPED 2026-06-13; FULLY IN OPERATOR MARKETPLACE (marketplace PR #13 MERGED); annotated tag object |
| v1.0.0-rc.20 (tag) | e9e38286 | SHIPPED 2026-06-01; marketplace PR #12 squash-merged 862e660d |
| v1.0.0-rc.19 (tag) | d15152af | SHIPPED 2026-05-28 |

## Concurrent Cycles

| Cycle | Type | Status | Notes |
|-------|------|--------|-------|
| F-block-ai-attribution-message-file-arm | feature | F3 COMPLETE — F4 READY | F1+F2+F3 done 2026-05-12; 2 stories ready; E-16 under SS-07/SS-04; milestone v1.0.0-rc.17 |
| v1.0-brownfield-backfill | brownfield | **D-575 2026-06-15; F2 E-18 PRE-PASS-14 CONSISTENCY-SWEEP REMEDIATION; develop 7e99f6ef; main caf06c68** | rc.21 100% COMPLETE D-560; D-574 ADV PASS-13 NOT-CLEAN FIX BURST (4 findings fixed; 3-CLEAN streak 1/3→0/3 RESET); **D-575 consistency-sweep REMEDIATION** (VP-082-BATS-SPLIT closed; VP-082 v1.6; **VP-INDEX v2.18**; sweep CLEAN); 4-index BC v2.82/**VP v2.18**/STORY v4.01/ARCH v2.37; **Next: adversary pass-14 (need 3 consecutive clean for BC-5.39.001 convergence) → F3 story decomposition (S-18.00..S-18.07+S-18.08).** |
| v1.0-feature-engine-discipline-pass-1 | feature | **PAUSED** | F5 pass-75 adversary complete D-510 2026-05-27; META-LEVEL-30 CANDIDATE-CONFIRMED; trajectory →9→9→9→11. Full-cycle trajectory (75 values ending): →9→9→9→9→11. |
| v1.0-feature-plugin-async-semantics-pass-1 | feature | CLOSED | All PRs merged; rc.14 shipped |

## Decisions Log

> D-001..D-556: `cycles/v1.0-brownfield-backfill/decision-log.md` + `decisions-log-archive.md`
> D-557..D-567 archived to decision-log.md SoT per D-430(a) D-568 compaction burst.
> Latest: D-568 (F2 E-18 pass-7 fix + compaction) + D-569 (F2 E-18 pass-8 fix) + D-570 (F2 E-18 pass-9 fix) + D-571 (F2 E-18 pass-10 fix) + D-572 (F2 E-18 pass-11 fix) + D-573 (F2 E-18 pass-12 CLEAN) — in table below.
> F5 pass-2 architect decisions: `cycles/v1.0-feature-engine-discipline-pass-1/F5-pass-2-architect-decisions.md`
> D-379..D-454 (F5): `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`

| ID | Decision | Phase | Date |
|----|----------|-------|------|
| D-575 | F2 E-18 PRE-PASS-14 CONSISTENCY-SWEEP REMEDIATION 2026-06-15 — Comprehensive consistency-validator sweep over full E-18 package returned NOT-CLEAN with 1 MAJOR: VP-082-BATS-SPLIT (VP-082.md v1.5 last_amended claimed the bats proof-harness skeleton was split into commit-failure + push-failure tests, but the body retained a single mislabeled test — a push-path setup under a commit-failure heading; the genuine push-failure test for Postcondition F / BC-7.07.001 PC6b was absent). Fixed: VP-082.md v1.5→v1.6 (architect): skeleton split into (a) commit-failure LOCAL-only test (chmod 444 .git/objects; push never attempted; HEAD unchanged; no log entry written) → exit 2 Postcondition B, and (b) push-failure test (local commit + log append succeed per ADR-026 §F-P10-002 ordering, then push fails) → exit 2 Postcondition F / BC-7.07.001 PC6b; local commit + log entry retained; retry is push-only per ADR-026 §F-P10-002 + BC-7.07.001 PC6b. VP-INDEX v2.17→v2.18 (VP-082 row version cell sync; row description updated with two distinct test cases per D-575 VP-082-BATS-SPLIT closure). All other sweep dimensions CLEAN (14 index↔body parities, DI-025 EXEMPT three-arm spec, POLICY 19 tree-wide, O-P8-002 payload-only invariant, YAML well-formedness). POLICY 9 propagation: verification-architecture.md and verification-coverage-matrix.md do NOT exist (deferred per VP-INDEX preamble since v2.07; VP-INDEX IS the authoritative coverage table); neither file cites VP-082 version explicitly — NO POLICY 9 changes required. 4-index: BC-INDEX v2.82 / VP-INDEX v2.18 / STORY-INDEX v4.01 / ARCH-INDEX v2.37. Consistency sweep now CLEAN. NEXT: adversary pass-14 (fresh-context; 3-CLEAN streak still 0/3 — pass-13 reset; need 3 consecutive clean from pass-14). D-chain cite D-574 per D-419(b); parent-commit 4dc23351 per D-419(b). | feature-mode-f2-e18-pre-pass-14-consistency-sweep-remediation | 2026-06-15 |
| D-574 | F2 E-18 ADV PASS-13 NOT-CLEAN FIX BURST 2026-06-14 — Pass-13 NOT-CLEAN (4 findings; 3-CLEAN streak 1/3→0/3 RESET). Fixed: (1) F-P13-001 BLOCKER: invariants.md v1.17→v1.18 — DI-025 FIELD-4-corruption outcome corrected from NOT-EXEMPT to EXEMPT (prefix-match-alone). Prior text routed FIELD-4-absent/empty/≠`commit` → "deny the exemption" (NOT-EXEMPT), contradicting BC-5.41.003 PC1 case (b)/(c), Inv1 gate (3), ADR-026 §F-P4-004 EC-003 which all route corruption → stale-entry → prefix-alone → EXEMPT. SHA-mismatch-with-valid-FIELD-4 (anti-forgery) stays NOT-EXEMPT. (2) F-P13-002 MEDIUM: VP-082 v1.4→v1.5 (architect: Postcondition F push-failure added to body; bats skeleton split commit-local/push-network); VP-INDEX v2.16→v2.17 (state-manager cross-doc sync: Full Index VP-082 row description updated with push-failure Postcondition F leg — push failure → exit 2; local commit + log entry retained; retry is push-only per ADR-026 §F-P10-002 + BC-7.07.001 PC6b). (3) F-P13-003 MEDIUM: VP-INDEX YAML structural fix — changelog v1.70/v1.69 entries (formerly sharing one `- date: 2026-05-12` list item with two `change:` keys; YAML duplicate-key violation) corrected to separate `- date:` items; YAML now parses cleanly. (4) F-P13-004 LOW: BC-5.41.001 v1.6→v1.7 (PO: Traceability BC-INDEX cite de-versioned from `per v1.6` to stable `§Decision N` anchor per POLICY 19); BC-INDEX v2.81→v2.82. 4-index: BC-INDEX v2.82 / VP-INDEX v2.17 / STORY-INDEX v4.01 / ARCH-INDEX v2.37 (literal-shell grep ^version:). Trajectory: P11(0B/1M/1med)→P12 CLEAN(1/3)→P13 NOT-CLEAN(2M; streak reset 0/3). NEXT: comprehensive consistency-validator cross-doc sweep (full E-18 package: BC/ADR/DI/VP-body/VP-INDEX/capabilities) to flush remaining sibling-sweep gaps; then adversary pass-14. D-chain cite D-573 per D-419(b); parent-commit 0d70b606 per D-419(b). | feature-mode-f2-adv-pass-13-fix | 2026-06-14 |
| D-573 | F2 E-18 ADV PASS-12 CLEAN 2026-06-14 — Fresh-context adversarial pass-12 returned CLEAN: zero BLOCKER, zero MAJOR, zero load-bearing MEDIUM, zero mis-anchor. Adversary independently re-derived the full E-18 package (ADR-026 v1.9 + BC-4.14.001 v1.8 + BC-7.07.001 v1.7 + BC-5.41.001/5.41.002/5.41.003/6.24.001/7.07.002 v1.6 + BC-1.15.001 v1.2 + VP-082 v1.4 + VP-083 v1.3 + VP-081 v1.3 + VP-084 v1.6 + VP-085/086 + invariants.md v1.16 + capabilities.md v1.6) and declared convergence. NO spec changes; NO fix burst. 4-index BC-INDEX v2.81 / VP-INDEX v2.16 / STORY-INDEX v4.01 / ARCH-INDEX v2.37 UNCHANGED. 3-CLEAN streak advances 0/3→1/3 per BC-5.39.001. Convergence trajectory P9(0B/0M/1med)→P10(0B/1M/2med)→P11(0B/1M/1med)→P12 CLEAN (1/3). Adversary pass-13 NEXT (need 2 more consecutive clean for 3-CLEAN convergence). D-chain cite D-572 per D-419(b); parent-commit be237a89 per D-419(b). | feature-mode-f2-adv-pass-12-clean | 2026-06-14 |
| D-572 | F2 E-18 ADV PASS-11 FIX BURST 2026-06-14 — (1) F-P11-001 MAJOR: VP-083 v1.2→v1.3 — Property §1 and all GateContext/fixture comments rewritten to PAYLOAD-ONLY discriminator: `is_first_wave = (payload.wave_id == 1)` computed from HANDOFF payload — NO prior-HANDOFF.md filesystem read, NO sprint-state.yaml topo-sort. Fail-closed postcondition added: `wave_id` ABSENT from payload → gate FAILS CLOSED (proceeds to full validation, blocking with HandoffIncomplete: ['wave_id']); unit-test fixture `wave_id_absent_fails_closed` added to proof harness. VP-INDEX v2.15→v2.16: VP-083 version cell v1.2→v1.3 (row description unchanged — already reflects payload-only and fail-closed from v2.15). (2) F-P11-002 MEDIUM: VP-081 v1.2→v1.3 — Precondition rewritten to clarify division: shell caller (wave-gate/wave-handoff skill, BC-5.41.001) reads real substrate (sprint-state.yaml topo-sort ordinal OR prior HANDOFF.md presence) to derive wave_id and embed it in payload; WASM gate's first-wave determination is PAYLOAD-ONLY (`payload.wave_id == 1`). Integration fixture comments revised to distinguish SHELL CALLER substrate reads from WASM GATE payload reads. Fail-closed Postcondition E added: wave_id ABSENT from payload → HandoffIncomplete. Integration test fixture `test_wave_id_absent_fails_closed` added. VP-INDEX v2.16: VP-081 version cell v1.2→v1.3 (row description unchanged). (3) ADR-026 v1.9 UNCHANGED — verified clean via tree-grep: `grep -rnE 'prior HANDOFF|wave-group ordinal|wave-group position' .factory/specs/verification-properties/ .factory/specs/behavioral-contracts/ss-04/` → 0 load-bearing gate-behavior violations; all hits are shell-caller attribution explicitly labeled or historical changelog. (4) capabilities.md UNCHANGED (shell-hook attribution). (5) O-P8-002 GATE SCOPE EXTENDED TO VP FILES — L-F2-payload-only-discriminator-recurrence-gate updated with layer-propagation (BC→VP-INDEX-row→VP-body) and extended gate scope: S-18.08 consistency-validator must ALSO verify that VP files whose `source_bc`/`bcs[]` include a pure-parse BC do not describe THAT BC's gate behavior via external-substrate reads (`grep -nE 'sprint-state|prior HANDOFF|wave-group ordinal'` in VP Property/Postcondition/fixture sections → 0 load-bearing hits; shell-caller-attribution explicitly labeled is OK). (6) Input-hash propagation: VP-083 + VP-081 updated (65f3f13). (7) TREE-WIDE GATE PASS (literal-shell): `grep -rnE 'prior HANDOFF|wave-group ordinal|wave-group position' .factory/specs/verification-properties/ .factory/specs/behavioral-contracts/ss-04/` → all VP-081 hits are shell-caller-attribution (explicitly labeled "SHELL CALLER substrate" in fixture comments + Precondition body); VP-INDEX `last_amended` hits are historical changelog; BC-4.14.001 hits in `last_amended`/`changelog` entries are historical — 0 load-bearing gate-behavior violations. `grep -rEn 'ADR-026 v1\.[0-9]' .factory/specs/behavioral-contracts/` → 0 live normative hits in BC body sections (all in last_amended/changelog/BC-INDEX changelog). 4-index: BC-INDEX v2.81 / VP-INDEX v2.16 / STORY-INDEX v4.01 / ARCH-INDEX v2.37. (8) CONVERGENCE: trajectory P8(0B/1M)→P9(0B/0M/1MED)→P10(0B/1M/2MED)→P11(0B/1M/1MED). 3-CLEAN streak 0/3: pass-11 has MAJOR (F-P11-001) → streak reset. Pass-12 = next clean-shot. D-chain cite D-571 per D-419(b); parent-commit 4294b479 per D-419(b). | feature-mode-f2-adv-pass-11-fix | 2026-06-14 |
| D-571 | F2 E-18 ADV PASS-10 FIX BURST 2026-06-14 — (1) F-P10-001 MAJOR: BC-4.14.001 v1.7→v1.8 — wave_id PAYLOAD-ONLY discriminator: gate reads `wave_id` from HANDOFF payload; `is_first_wave = (payload.wave_id == 1)`; wave_id==1 → no-op; wave_id>1 → full validation; wave_id ABSENT → FAILS CLOSED (HandoffIncomplete: ['wave_id']). All sprint-state.yaml/prior-HANDOFF.md/factory-artifacts/wave-context-cannot-be-determined framing removed from gate behavior (Description, PC3, PC4, PC8, Inv3, EC-001..006/010, test vectors, VP-083/VP-081 rows). BC-4.14.001 Invariant 1 pure-parse constraint fully satisfied. BC-INDEX v2.80→v2.81. (2) F-P10-002 MEDIUM: BC-7.07.001 v1.6→v1.7 — explicit `git push` step added to Inv3 canonical execution order (after successful append). PC6 split: git COMMIT failure = LOCAL only (no network) → exit 2, no push, no log entry. New push-failure postcondition (PC6b): git push failure (network/remote) → exit 2; local commit + log entry retained; retry is push-only. EC-003 reattributed to PUSH step. Exit codes {0,2} unchanged. (3) F-P10-003 MEDIUM: VP-082 v1.3→v1.4 — Postcondition E added to VP-082.md body. VP-INDEX v2.14→v2.15: VP-083 Full Index row reframed to payload wave_id discriminator per BC-4.14.001 v1.8; VP-081 Full Index row updated with fail-closed clause. ARCH-INDEX v2.36→v2.37: ADR-026 v1.8→v1.9 amendment row + provenance leg. (4) O-P8-002 ESCALATED TO MANDATORY (3rd occurrence): F-P10-001 is the 3rd occurrence of payload-only-discriminator drift class (F-P7-001 pass-7 + F-P8-001 pass-8 + F-P10-001 pass-10). Per L-F2-payload-only-discriminator-recurrence-gate escalation rule, S-18.08 consistency-validator pure-parse invariant check is MANDATORY (hard requirement). L-F2-payload-only-discriminator-recurrence-gate updated with 3rd-occurrence disposition. (5) TREE-WIDE GATE PASS (literal-shell): sprint-state.yaml/prior-HANDOFF.md in VP-INDEX Full Index VP-083/VP-081 rows = 0 load-bearing; ADR-026 v1.X in BC body = 0 load-bearing; git push in BC-7.07.001 = PRESENT. 4-index: BC-INDEX v2.81 / VP-INDEX v2.15 / STORY-INDEX v4.01 / ARCH-INDEX v2.37. (6) CONVERGENCE: trajectory P7(0B/2M)→P8(0B/1M)→P9(0B/0M/1MED)→P10(0B/1M/2MED). 3-CLEAN streak 0/3: pass-10 has MAJOR → streak reset. Pass-11 = next clean-shot. D-chain cite D-570 per D-419(b); parent-commit 9f0d7053 per D-419(b). | feature-mode-f2-adv-pass-10-fix | 2026-06-14 |
| D-570 | F2 E-18 ADV PASS-9 FIX BURST 2026-06-14 — (1) F-P9-001 MEDIUM: ADR-026 v1.7→v1.8 — §F-P5-002 PO Wording PC8 block annotated as SUPERSEDED by §F-P6-006 SHA-pinned form; §Decision A live text annotated — HEAD~1 tokens replaced/annotated: §Decision A now prescribes the SHA-pinned SHA_B^ guard (per §F-P6-006) and labels the HEAD~1 form as historical-only; §F-P5-002 PC8 block given explicit SUPERSEDED-by-§F-P6-006 banner; test-vector note updated to assert SHA_B^ (not HEAD~1) + concurrent-commit path. Closure gate: `grep -rEn 'reset --soft HEAD~1' .factory/specs/architecture/decisions/ADR-026-*.md` → 0 live normative hits (all hits in changelog/superseded-labeled/rationale context). (2) F-P9-002 LOW: VP-INDEX v2.12→v2.13 — VP-082 Full Index row description amended to include append-failure → SHA-pinned-reset (SHA_B^) + concurrent-commit-guard behavior. (3) ARCH-INDEX v2.35→v2.36: ADR-026 v1.7→v1.8 amendment row added; §Traceability provenance leg appended (ADR v1.7→v1.8; VP-INDEX v2.12→v2.13). (4) TREE-WIDE GATE PASS (literal-shell): HEAD~1 in ADR-026 = 0 live normative; ADR-026 v1.X in BC body = 0 load-bearing. 4-index: BC-INDEX v2.80 / VP-INDEX v2.13 / STORY-INDEX v4.01 / ARCH-INDEX v2.36 (grep ^version:). Input-hash --update folded (single-burst). (5) CONVERGENCE: trajectory P6(3B/4M)→P7(0B/2M)→P8(0B/1M)→P9(0B/0M/1MED). BCs verified correct per orchestrator. Package body remains converged (ADR/VP-INDEX-internal only). 3-CLEAN streak 0/3: pass-9 has MEDIUM → streak reset. Pass-10 = next clean-shot. D-chain cite D-569 per D-419(b); parent-commit a61e0e6e per D-419(b). | feature-mode-f2-adv-pass-9-fix | 2026-06-14 |
| D-569 | F2 E-18 ADV PASS-8 FIX BURST 2026-06-14 — (1) F-P8-001 MAJOR (sole finding): BC-4.14.001 v1.6→v1.7 — PC2a EPIC-COMPLETE discriminator corrected: stale conjunct "AND all story entries in the parsed content have terminal status OR cannot be determined" REMOVED. WASM gate discriminator now PAYLOAD-ONLY: `next_wave_stories: []` → EPIC-COMPLETE branch; non-empty → non-EPIC-COMPLETE branch. BC-4.14.001 Invariant 1 pure-parse constraint satisfied: WASM gate reads only the Write/Edit payload; no git/filesystem read. Terminal-state judgment (broken-sprint vs genuine final wave) remains in shell-context wave-gate BC-5.41.002. Consistent with ADR-026 §Decision 8 + PC2a line-72 practical heuristic. BC-INDEX v2.79→v2.80. (2) O-P8-001: capabilities.md v1.5→v1.6 — CAP-032 Source cite de-versioned to stable anchor `ADR-026 §Decision 8` (was `ADR-026 v1.7 §Decision 8`) per TD-VSDD-091 anti-volatile-pin. (3) O-P8-002 [process-gap] CODIFIED: payload-only-discriminator drift class 2nd occurrence (F-P7-001 pass-7 → F-P8-001 pass-8). Lesson L-F2-payload-only-discriminator-recurrence-gate added to lessons.md. S-18.08 scope extended: consistency-validator must verify that for any BC with Invariant 1 pure-parse, no PC references external substrate (sprint-state.yaml entries, prior HANDOFF.md, git log). 3rd occurrence would require mandatory machine-checkable gate. (4) TREE-WIDE GATE PASS (literal-shell): ADR-026 v1.X in BC body = 0 load-bearing; stale conjunct in BC-4.14.001 body = 0 load-bearing. 4-index: BC-INDEX v2.80 / VP-INDEX v2.12 / STORY-INDEX v4.01 / ARCH-INDEX v2.35. (5) CONVERGENCE: trajectory P5(1B/3M)→P6(3B/4M)→P7(0B/2M)→P8(0B/1M). Adversary confirms package substantially converged. 3-CLEAN streak 0/3: pass-8 = first real clean-shot (0B+1M closed; 2 obs). Adversary pass-9 NEXT — if clean: streak 1/3. D-chain cite D-568 per D-419(b); parent-commit a5d6f2ff per D-419(b). | feature-mode-f2-adv-pass-8-fix | 2026-06-14 |
| D-568 | F2 E-18 ADV PASS-7 FIX BURST + STATE.md COMPACTION 2026-06-14 — ADR-internal fix burst (architect-led; state-manager records). (1) F-P7-001 MAJOR: ADR-026 §Decision 2 EPIC-COMPLETE discriminator changed from filesystem-read prescription (prior HANDOFF.md on factory-artifacts absent OR non-empty next_wave_stories) to PAYLOAD-ONLY discriminator (current payload next_wave_stories: [] → EPIC-COMPLETE; non-empty → non-EPIC-COMPLETE). BC-4.14.001 Invariant 1 pure-parse constraint now satisfied: WASM gate reads only the Write/Edit tool-call payload; no git read; no filesystem read of prior HANDOFF.md. Richer terminal-state judgment (broken-sprint vs genuine final wave) remains in shell-context wave-gate/wave-handoff BC-5.41.002. ADR-026 v1.6→v1.7. (2) F-P7-002 MAJOR: §Traceability downstream-index provenance trace completed: VP-INDEX line appended v2.11→v2.12 leg (pass-6 cite-convention migration); ARCH-INDEX line appended v2.33→v2.34 (ADR-026 v1.5→v1.6) and v2.34→v2.35 (ADR-026 v1.6→v1.7 this pass-7). ARCH-INDEX v2.34→v2.35. VP-INDEX v2.12 UNCHANGED. (3) STATE.md COMPACTION per D-430(a): D-557..D-567 (11 decision rows) archived to decision-log.md SoT; banner tracker D-532..D-566 (35 entries) collapsed to range reference; §3 older carries D-549..D-560 (12) retired; §4 Tier-A D-549..D-555 (7) trimmed; STATE.md 435→~370L (well under soft-target 415). (4) TREE-WIDE GATE PASS (literal-shell): ADR-026 v1.X in BC body files = 0 load-bearing; VP-INDEX Traceability leg = v2.12 CONFIRMED; ARCH-INDEX Traceability leg = v2.35 CONFIRMED; 4-index: BC-INDEX v2.79 / VP-INDEX v2.12 / STORY-INDEX v4.01 / ARCH-INDEX v2.35 (grep ^version:). Input-hash --update folded (single-burst). (5) CONVERGENCE: trajectory P1(3B/6M)→P2(2B/4M)→P3(5B/4M)→P4(0B/3M)→P5(1B/3M)→P6(3B/4M)→P7(0B/2M). Pass-7 package body verified clean — only ADR-internal tails remained (ADR-026 self-updated). Convergence approaching. 3-CLEAN streak 0/3 (pass-7 reset). D-chain cite D-567 per D-419(b); parent-commit ef7eafe2 per D-419(b). | feature-mode-f2-adv-pass-7-fix + state-compaction | 2026-06-14 |
| D-567 | F2 E-18 ADV PASS-6 FIX BURST 2026-06-14 (state-mgr bookkeeping) — POLICY 19 `adr_version_cite_volatile_pin_prohibition` registered (policies.yaml v1.3.6→v1.4.0); VP-084 v1.5→v1.6 (cite de-versioned per POLICY 19/TD-VSDD-091); VP-INDEX v2.11→v2.12; 2 lessons (L-F2-DI-sibling-sweep-unswept-sibling + L-F2-ADR-cite-convention-recurring-stale-cite-class); O-P6-001 process-gap codified; 4-index: BC-INDEX v2.79 VP-INDEX v2.12 STORY-INDEX v4.01 ARCH-INDEX v2.34; tree-wide gate PASS; 3-CLEAN 0/3; D-chain cite D-566 per D-419(b); parent-commit 4332e312 per D-419(b). | feature-mode-f2-adv-pass-6-state-mgr-bookkeeping | 2026-06-14 |
| D-562..D-566 archived | **ARCHIVED 2026-06-14 per D-430(a) D-568** | D-562 F2 adv-pass-1 fix; D-563 F2 adv-pass-2 fix; D-564 F2 adv-pass-3 complete-sweep fix; D-565 F2 adv-pass-4 fix; D-566 F2 adv-pass-5 fix. Full rows: decision-log.md SoT. |
| D-557..D-561 archived | **ARCHIVED 2026-06-14 per D-430(a) D-568** | D-557 SESSION-INTERRUPT; D-558 rc.21 RELEASED; D-559 marketplace MERGED; D-560 operator-install VERIFIED rc.21 100% COMPLETE; D-561 F2 E-18 spec evolution COMPLETE. Full rows: decision-log.md SoT. |
| D-549..D-556 archived | **ARCHIVED 2026-06-11..2026-06-12 per D-430(a)** | D-549..D-555 ADR-025/S-17.04 adversary passes; D-556 S-17.04 MERGED. Full rows: decision-log.md SoT. |
| D-548 | ADR-025 v1.3→v1.4 + S-17.04 AUTO-RENEW WIRING CODIFIED 2026-06-11 — ADR-025 Decision 11; S-17.04 E-17 wave 4; ARCH-INDEX v2.20→v2.21; STORY-INDEX v3.92→v3.93. | story-authoring | 2026-06-11 |
| D-527..D-538 archived | **COMPACTED 2026-06-11 per D-430(a)** | D-527 SESSION-END; D-528 rc.20 SHIPPED; D-529 MAINT SWEEP; D-530 E-10 pass-16 LOW; D-531 E-10 CASCADE SEALED; D-532..D-538 various. Full rows: decision-log.md SoT. |
| D-499..D-509 archived | **COMPACTED 2026-06-08 per D-430(a)** | 11 rows archived. Full rows: `git show 688dd1c2:.factory/STATE.md` lines 249-259. |
| D-413..D-498 archived | **COMPACTED 2026-05-27 per D-430(a)** | 36 rows archived. Full content: decision-log.md. Pre-compaction state: `git show 20cb8e1c:.factory/STATE.md`. |
| D-510+D-522+D-525+D-526 archived | **COMPACTED 2026-06-10 per D-430(a) D-542 burst** | D-510 F5 pass-75; D-522 S-15.17 SEALED; D-525 ADR-023; D-526 S-15.17 SHIPPED. Full rows: decision-log.md SoT. |

## Skip Log

| Step | Skipped? | Justification |
|------|----------|---------------|
| UX Spec | yes | CLI-only product with no UI surfaces |
| Gene Transfection Assessment | yes | Not applicable — engine and product are same repo |

## Blocking Issues

<!-- No open blockers on active stories. -->

## Drift Items / Tech Debt

| Item | Status | Notes |
|------|--------|-------|
| **TD #67** 4 timing-flaky e2e tests | **RESOLVED 2026-05-15 PR #143 + RECURRENCE RESOLVED 2026-05-31 PR #165** | F-P3-008 pattern fully resolved. |
| **TD #68/69/70/71/72/74** | **ALL RESOLVED** 2026-05-13/14/15 | See PRs #114/#116/#117/#140/#138/#139/#141. |
| Ghost BCs: BC-3.07.003/004, BC-1.06.011 | DEFERRED | Missing from BC-INDEX; investigate in future fix-burst |
| **TD-VSDD-061 (F-P6-002)** | OPEN 2026-05-17 | validate-index-cite-refresh + validate-burst-log `host::read_file(...65536...)` against files >64KiB → silent fail-open. Story needed to raise max_bytes to 524288. |
| **TD-VSDD-062/063** | OPEN 2026-05-17/19 | Schema inconsistencies in M2 stories (LOW); deferred VP allocation for BC-5.39.006 9 pending VPs. |
| **PG-S-15.11-bats-prod-registry-parity-gate** | OPEN 2026-05-17 | Bats inline `path_allow` arrays must be byte-identical to production hooks-registry.toml. Target: S-15.03 PRIORITY-A automation wave. |
| **TD-VSDD-095..100 (CODIFIED-LESSONS)** | CODIFIED-AND-FORWARDED-TO-SK-MCP-001 2026-05-17/18 | 6-class META-LEVEL perimeter; TDD micro-commit + registry-priority + compaction-burst-sibling-sweep + own-burst-log-structural-integrity + dim2-pc-must-read-production disciplines. |
| **TD-VSDD-101 (CI env-var paper-fix)** | OPEN 2026-05-18 — anchored S-15.15 | `VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1` skips production STATE.md bats test in CI. |
| **S-15.17-CR-001/002** | ACCEPTED-DEFERRED 2026-05-31 | `check_index_sites` + `rows_after_heading` advisory-arm defects. Revisit on next spec-touch. |
| **test_F_P2_001 timing flake** | OPEN 2026-06-08; RECURRED 2026-06-13 | darwin-x64 at 3761ms / 3179ms vs 3000ms threshold; cleared by re-run; de-flake candidate story. |
| **RUSTSEC-2026-0149** | OPEN 2026-06-11 — wasmtime-wasi HIGH | wasmtime >= 44.0.2 required; awaiting upstream compatibility. Anchor: next rc cycle. |
| **O-PASS16-002 header stale doc-comment** | OPEN 2026-06-08 | validate-trajectory-tail-cell-completeness stale function header. Cosmetic; next spec-touch. |
| **bats-full-suite not in branch-protection required-status-checks** | OPEN 2026-06-13 — D-558 capture | New ci.yml `bats-full-suite (linux)` job runs but NOT in branch-protection required-checks. Follow-up: add to branch-protection settings. |
| **BC-INDEX count reconcile (pre-existing)** | OPEN 2026-06-14 — D-562 capture | disk truth = 1970 BC files, BC-INDEX frontmatter total_bcs=1966; orphan BC-2.02.013; stale SS header counts. Routing: state-manager + product-owner. Anchor: dedicated BC-INDEX reconcile burst. |
| **S-18.08 phantom-field-removal lint gate** | DRAFT-PENDING-AUTHORING 2026-06-14 — D-563 capture | L-F2-phantom-field-gate lesson (D-563): permanent enforcement story. Anchor: E-18 epic, F3 story decomposition. |

## Historical Content

- `cycles/v1.0-brownfield-backfill/burst-log.md` | `session-checkpoints.md` | `lessons.md` | `decision-log.md`
- `cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | `session-checkpoints.md` | `lessons.md`
- `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md` (adversary reviews at `S-12.03/`, `S-12.04/`, `S-12.05/` subdirs)

## Session Resume Checkpoint (2026-06-15 — D-575 F2 E-18 PRE-PASS-14 CONSISTENCY-SWEEP REMEDIATION; sweep CLEAN; 3-CLEAN streak 0/3 unchanged; next: adversary pass-14 → 3-CLEAN → F3)

> **SELF-SUFFICIENT RESUME CONTEXT FOR ZERO-CONTEXT NEW SESSION OR NEW MACHINE**
> Read this section alone to resume. Assumes ZERO prior context. All decisions, directives, and anchors stated explicitly.

### §1. Where We Are

**E-18 CAP-032 context-durability (GitHub issue #173) — Feature Mode, Phase F2 (spec evolution) adversarial 3-CLEAN convergence cascade (BC-5.39.001). D-575 2026-06-15.**

F1-gate APPROVED. F2 spec evolution COMPLETE (D-561). F2 adversarial cascade passes 1-13 COMPLETE (D-562..D-574). Consistency-validator sweep D-575 COMPLETE (CLEAN). 13 adv passes done; pass-12 CLEAN (1/3 streak); pass-13 NOT-CLEAN (4 findings fixed; streak RESET to 0/3). Pre-pass-14 sweep CLEAN.

**D-575 consistency-sweep remediation summary:**
- 1 MAJOR finding: VP-082-BATS-SPLIT (VP-082.md v1.5 claimed skeleton split into commit-failure + push-failure tests, but body retained a single mislabeled test with push-path setup under commit-failure heading; genuine push-failure test for Postcondition F / BC-7.07.001 PC6b was absent).
- Fixed: VP-082.md v1.5→v1.6 — (a) commit-failure LOCAL-only test (chmod 444 .git/objects; push never attempted; HEAD unchanged; no log entry written) → exit 2 Postcondition B; (b) push-failure test (local commit + log append SUCCEED per ADR-026 §F-P10-002, then push fails) → exit 2 Postcondition F / BC-7.07.001 PC6b; local commit + log entry retained; retry is push-only.
- VP-INDEX v2.17→v2.18 (VP-082 row version cell sync; row description updated with two distinct test cases).
- All other sweep dimensions CLEAN.
- POLICY 9: verification-architecture.md and verification-coverage-matrix.md do NOT exist (deferred per VP-INDEX preamble); no POLICY 9 changes required.

**3-CLEAN streak: 0/3** (pass-13 reset; unchanged by sweep). Need 3 NEW consecutive clean passes from pass-14 for F2 convergence.

**Convergence trajectory (last 4 passes per D-433(e)+D-439(c)):** →P11(0B/1M/1med)→P12 CLEAN(1/3)→P13 NOT-CLEAN(0/3).

**4-index at D-575:** BC-INDEX v2.82, VP-INDEX v2.18, STORY-INDEX v4.01, ARCH-INDEX v2.37.

**NEXT ACTION (explicit, in order):**
1. **START HERE: adversary pass-14** (fresh-context; reads package at D-575 versions). Consistency sweep DONE+CLEAN — no further sweep needed before pass-14.
2. Need 3 CONSECUTIVE CLEAN passes from pass-14 onward for F2 convergence (BC-5.39.001). 3-CLEAN streak 0/3.
3. On F2 convergence → human gate → Feature Mode F3 story decomposition (story-writer authors S-18.00..S-18.07 + S-18.08 the codified O-P8-002 gate-story).

**RECURRING LESSON (sibling-sweep):** Every semantic fix must propagate to ALL layers (BC + ADR + DI + VP-body + VP-INDEX row + capabilities). Track via lessons.md L-F2-payload-only-discriminator-recurrence-gate. The recurring detection failure is a single-layer fix that strands siblings.

**Artifact versions at D-575:**
- ADR-026 v1.9; BC-1.15.001 v1.3; BC-4.14.001 v1.8; BC-7.07.001 v1.7; BC-5.41.001 v1.7; BC-5.41.002 v1.6; BC-5.41.003 v1.6; BC-6.24.001 v1.6; BC-7.07.002 v1.6; VP-081 v1.3; VP-082 v1.6; VP-083 v1.3; VP-084 v1.6; VP-085 v1.3; VP-086 (unchanged); invariants.md v1.18 (DI-020..025); capabilities.md v1.6 (CAP-032); BC-INDEX v2.82; VP-INDEX v2.18; ARCH-INDEX v2.37; STORY-INDEX v4.01.

**Key design facts (so restart agent has them without re-deriving):**
- wave-1 no-op = `payload.wave_id == 1` (pure-parse WASM; wave_id absent → fail-closed HandoffIncomplete); EPIC-COMPLETE = `payload.next_wave_stories: []`; WASM gate pure-parse (shell wave-handoff derives wave_id from substrate).
- flush = commit(LOCAL, capture SHA_B) → append-to-log → push(REMOTE); each step failure → exit 2; append-fail → reset --soft SHA_B^ if HEAD==SHA_B else exit 2 (human intervention).
- push-fail (network/remote after successful local commit + log append) → exit 2; local commit + log entry retained; retry is push-only (no re-commit, no re-append) per ADR-026 §F-P10-002 + BC-7.07.001 PC6b.
- precompact-flush-log 4 fields: `<ISO-timestamp> <SHA> <cycle>/<step> commit` (field-2=SHA, field-4=`commit` token); WASM reads fields, no git-exec.
- exemption logic: prefix + field-2 + field-4 all valid → NOT-EXEMPT (anti-forgery); field-4 ≠ `commit` (corruption) → EXEMPT via prefix-alone; field-4 absent/empty = corruption = EXEMPT.
- HANDOFF: 9 base fields + epic_status conditional 10th; terminal = {merged, withdrawn, cancelled}; factory_lock opt-in default null; PostCompact best-effort outside CAP-032; harness >= v2.1.105; POLICY 19 = stable §Decision N anchor (no version pins in BC cites).
- O-P8-002 gate MANDATORY (3rd recurrence D-571): for any BC with Invariant 1 pure-parse, consistency-validator must verify VP files with matching `source_bc`/`bcs[]` do NOT describe gate behavior via external-substrate reads.

rc.21 FULLY SHIPPED D-560 (2026-06-13). main caf06c68. develop 7e99f6ef. tag 03054524. Marketplace #13 MERGED.

### §2. Operating Mode

- vsdd-factory brownfield-onboarding; cycle `v1.0-brownfield-backfill`; self-referential.
- **E-10 CASCADE FULLY SEALED D-531** (2026-06-01; pass-16 asymptotic-acceptance). **Do NOT resume E-10 without engine-surface material change.**
- **F5 PAUSED D-386 Option C** (trajectory →9→9→9→11). **Do NOT resume without explicit human direction.**
- **S-15.03 PRIORITY-A COMPLETE D-508** (2026-05-27; 11 stories; 40pts). **RC.21 FULLY SHIPPED D-560** (2026-06-13; marketplace #13 MERGED).

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
- **D-537 [process-gap] spec-drift routing:** When TDD fix changes ADR-specified behavior, route architect ADR amendment in SAME burst.
- **D-539 multi-family adversary obligation:** prompt-contract + shell-logic issues require cross-family AND same-family Claude adversary passes before convergence.
- **D-575 carry:** F2 E-18 PRE-PASS-14 CONSISTENCY-SWEEP REMEDIATION 2026-06-15. VP-082-BATS-SPLIT closed (VP-082 v1.6; VP-INDEX v2.18). Sweep CLEAN. 3-CLEAN 0/3 unchanged. 4-index BC v2.82/VP v2.18/STORY v4.01/ARCH v2.37. NEXT: adversary pass-14 (need 3 NEW consecutive clean). D-chain cite D-574. parent-commit 4dc23351.
- **D-574 carry:** F2 E-18 ADV PASS-13 NOT-CLEAN FIX BURST. 4 findings fixed (see §1 above). 3-CLEAN 1/3→0/3 RESET. 4-index BC v2.82/VP v2.17/STORY v4.01/ARCH v2.37. Trajectory P13 NOT-CLEAN (0/3). D-chain cite D-573. parent-commit 0d70b606.
- **D-573 carry:** F2 E-18 ADV PASS-12 CLEAN. Pass-12 CLEAN: 0B/0M/0 load-bearing MED. 3-CLEAN 0/3→1/3. 4-index BC v2.81/VP v2.16/STORY v4.01/ARCH v2.37. D-chain cite D-572. parent-commit be237a89.
- **D-571 carry:** F2 E-18 ADV PASS-10 FIX BURST. BC-4.14.001 v1.8 (wave_id payload-only; EC-010 fail-closed). BC-7.07.001 v1.7 (push step). VP-082 v1.4 (Postcondition E). BC-INDEX v2.81. VP-INDEX v2.15. ARCH-INDEX v2.37. O-P8-002 MANDATORY. D-chain cite D-570. parent-commit 9f0d7053.
- **D-560 carry:** OPERATOR-INSTALL-VERIFIED 2026-06-13. rc.21 100% COMPLETE. NO remaining release action.
- **D-556 carry:** S-17.04 MERGED PR #184 3b2a378c. E-17 ALL 4 WAVES COMPLETE. STORY-INDEX v4.01.
- **D-541 carry (partial):** VP IDs TBD per TD-VSDD-063. BC-6.23.001 ACTIVE per POL-14.

### §4. Tier-A Completed Log (most recent first)

- **D-575 (2026-06-15):** F2 E-18 PRE-PASS-14 CONSISTENCY-SWEEP REMEDIATION. 1 MAJOR closed: VP-082-BATS-SPLIT (VP-082 v1.5→v1.6 skeleton split: commit-failure LOCAL-only + push-failure network; two distinct bats tests). VP-INDEX v2.17→v2.18 (VP-082 row version cell sync). Sweep CLEAN. 3-CLEAN streak 0/3 unchanged. 4-index: BC v2.82/VP v2.18/STORY v4.01/ARCH v2.37. NEXT: adversary pass-14.
- **D-574 (2026-06-14):** F2 E-18 ADV PASS-13 NOT-CLEAN FIX BURST. 4 findings fixed: F-P13-001 BLOCKER (invariants.md v1.18 — DI-025 FIELD-4-corruption EXEMPT); F-P13-002 MEDIUM (VP-082 v1.5 push-leg; VP-INDEX v2.17 cross-doc sync); F-P13-003 MEDIUM (VP-INDEX YAML duplicate-key fixed); F-P13-004 LOW (BC-5.41.001 v1.7 cite convention; BC-INDEX v2.82). 3-CLEAN streak 1/3→0/3 RESET. 4-index: BC v2.82/VP v2.17/STORY v4.01/ARCH v2.37. Trajectory P13 NOT-CLEAN (0/3).
- **D-573 (2026-06-14):** F2 E-18 ADV PASS-12 CLEAN. Pass-12 fresh-context CLEAN: 0B/0M/0 load-bearing MED/0 mis-anchor. NO spec changes; NO fix burst. 4-index BC v2.81/VP v2.16/STORY v4.01/ARCH v2.37 UNCHANGED. 3-CLEAN streak 0/3→1/3. Trajectory P12 CLEAN (1/3). Adversary pass-13 NEXT.
- **D-572 (2026-06-14):** F2 E-18 ADV PASS-11 FIX BURST. VP-083 v1.3 (F-P11-001 MAJOR: Property body PAYLOAD-ONLY; fail-closed postcondition; fixture). VP-081 v1.3 (F-P11-002 MEDIUM: shell-caller/WASM-gate division; Postcondition E; fixture). VP-INDEX v2.16 (version cells VP-083/VP-081 updated). ADR-026 v1.9 UNCHANGED. O-P8-002 gate extended to VP files (layer-propagation: BC→VP-INDEX→VP-body). 4-index: BC v2.81/VP v2.16/STORY v4.01/ARCH v2.37. Tree-wide gate PASS (0 load-bearing). P11(0B/1M/1MED). 3-CLEAN 0/3 (pass-11 MAJOR; pass-12 = next clean-shot).
- **D-571 (2026-06-14):** F2 E-18 ADV PASS-10 FIX BURST. BC-4.14.001 v1.8 (F-P10-001 MAJOR: wave_id payload-only; EC-010 fail-closed). BC-7.07.001 v1.7 (F-P10-002 MEDIUM: git push step). VP-082 v1.4 (F-P10-003 MEDIUM: Postcondition E). BC-INDEX v2.81. VP-INDEX v2.15 (VP-083/VP-081 payload-only). ARCH-INDEX v2.37. O-P8-002 → MANDATORY (3rd occurrence; S-18.08 hard req). 4-index: BC v2.81/VP v2.15/STORY v4.01/ARCH v2.37. Tree-wide gate PASS. P10(0B/1M/2MED). 3-CLEAN 0/3 (pass-10 MAJOR; pass-11 = next clean-shot).
- **D-570 (2026-06-14):** F2 E-18 ADV PASS-9 FIX BURST. ADR-026 v1.8 (F-P9-001 MEDIUM: §Decision A+§F-P5-002 PC8 SUPERSEDED by §F-P6-006 SHA-pinned form). VP-INDEX v2.13 (F-P9-002 LOW: VP-082 row completed). ARCH-INDEX v2.36. 4-index: BC v2.80/VP v2.13/STORY v4.01/ARCH v2.36. Tree-wide gate PASS. P9(0B/0M/1MED). 3-CLEAN 0/3 (pass-9 MEDIUM; pass-10 = next clean-shot).
- **D-569 (2026-06-14):** F2 E-18 ADV PASS-8 FIX BURST. BC-4.14.001 v1.7 (F-P8-001 sole MAJOR: PC2a PAYLOAD-ONLY discriminator; stale conjunct removed). BC-INDEX v2.80. capabilities.md v1.6 (CAP-032 cite-stability). L-F2-payload-only-discriminator-recurrence-gate (O-P8-002). 4-index: BC v2.80/VP v2.12/STORY v4.01/ARCH v2.35. Tree-wide gate PASS. P8(0B/1M). 3-CLEAN 0/3 (pass-8 = first clean-shot).
- **D-568 (2026-06-14):** F2 E-18 ADV PASS-7 FIX BURST + STATE.md COMPACTION. ADR-026 v1.7 (payload-only EPIC-COMPLETE + provenance trace). ARCH-INDEX v2.35. 4-index: BC v2.79/VP v2.12/STORY v4.01/ARCH v2.35. STATE.md compacted 435→~370L. Tree-wide gate PASS. P7(0B/2M). 3-CLEAN 0/3.
- **D-567 (2026-06-14):** F2 E-18 ADV PASS-6 FIX BURST (state-mgr bookkeeping). POLICY 19 registered. VP-084 v1.6. VP-INDEX v2.12. 2 lessons. O-P6-001 codified. 4-index: BC v2.79/VP v2.12/STORY v4.01/ARCH v2.34.
- **D-566 (2026-06-14):** F2 E-18 ADV PASS-5 FIX BURST. 1B+3M: invariants.md v1.16; ADR-026 v1.5; VP-084 v1.5; ARCH-INDEX v2.33; BC-INDEX v2.78. P5(1B/3M).
- **D-565 (2026-06-14):** F2 E-18 ADV PASS-4 FIX BURST. 0B+2-3M: ADR-026 v1.4; ARCH-INDEX v2.32; 8 BCs v1.4; BC-INDEX v2.77; VP-084 v1.4.
- **D-564 (2026-06-14):** F2 E-18 ADV PASS-3 COMPLETE-SWEEP. 5B+4M (INCOMPLETE-SIBLING-SWEEP): ADR-026 v1.3; VP-082/084/085 v1.3; capabilities.md v1.5; 7 BCs v1.3; BC-INDEX v2.76; ARCH-INDEX v2.31. Tree-wide gate PASS (3 sweeps).
- **D-563 (2026-06-14):** F2 E-18 ADV PASS-2 FIX BURST. 2B+4M+3m (phantom current_wave): ADR-026 v1.2; 7 BCs v1.2; VP-081..085 v1.2; BC-INDEX v2.75; ARCH-INDEX v2.30.
- **D-562 (2026-06-14):** F2 E-18 ADV PASS-1 FIX BURST. 3B+6M+6m: ADR-026 v1.1; VP-086 NEW; 8 BCs v1.1; BC-INDEX v2.74; ARCH-INDEX v2.29.
- **D-561 (2026-06-14):** F2 E-18 CONTEXT-DURABILITY SPEC EVOLUTION. F2 COMPLETE: ADR-026; VP-081..085; 8 BCs; CAP-032; ARCH-INDEX v2.28; BC-INDEX v2.73.
- **D-560 (2026-06-13):** rc.21 OPERATOR-INSTALL-VERIFIED. Step 9 PASSED. 100% COMPLETE end-to-end.
- **D-559+D-558 (2026-06-13):** rc.21 marketplace MERGED + RELEASED via re-release (PR #186+#188; release.yml all-PASS). main caf06c68; develop 7e99f6ef; tag 03054524.
- **D-556 (2026-06-12):** S-17.04 MERGED PR #184 3b2a378c. E-17 ALL 4 WAVES COMPLETE. STORY-INDEX v4.01.
- **D-549..D-555 archived** to decision-log.md SoT per D-430(a) D-568. D-549 SESSION-END; D-550 REDIRECT; D-551..D-555 ADR-025 v1.6 adversary corrections.
- **D-547 (2026-06-11):** S-17.03 MERGED PR #183. E-17 3/3 COMPLETE. issue #170 CLOSED.
- **D-531 (2026-06-01):** E-10 CASCADE SEALED. D-528 RC.20 SHIPPED. D-508 S-15.03 PRIORITY-A COMPLETE.

### §5. Cumulative Codifications

- F5: D-379..D-454 (76 decisions) — `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`.
- Brownfield: D-001..D-575 — `cycles/v1.0-brownfield-backfill/decision-log.md`. Latest: **D-575 F2 E-18 PRE-PASS-14 CONSISTENCY-SWEEP REMEDIATION 2026-06-15 — VP-082-BATS-SPLIT closed (VP-082 v1.6; VP-INDEX v2.18); sweep CLEAN; 4-index BC v2.82/VP v2.18/STORY v4.01/ARCH v2.37; adversary pass-14 NEXT.**

### §6. Cumulative Lessons

- F5: L-EDP1-001..067 — `cycles/v1.0-feature-engine-discipline-pass-1/lessons.md`.
- Brownfield: TD-VSDD-095..100 + L-M3-BC-cascade + L-E10-pass15 + L-banner-format-drift + L-rc19 + L-S-15.17-SP1..SP9 + L-F-P3-008 + L-session-2026-05-31 + L-session-2026-06-01-rc20 + L-E10-pass16 + L-E10-SEAL + L-session-2026-06-08 + L-issue-128 + L-issue-130 + L-issue-169-176-worktree-identity + L-F2-phantom-field-gate + L-F2-sibling-sweep-tree-wide-gate + L-F2-DI-sibling-sweep-unswept-sibling + L-F2-ADR-cite-convention-recurring-stale-cite-class + **L-F2-payload-only-discriminator-recurrence-gate** — `cycles/v1.0-brownfield-backfill/lessons.md`.

### §7. S-15.03 PRIORITY-A Scope

11-story wave S-15.06..S-15.16. **ALL SHIPPED D-508. 40pts M3 total. COMPLETE.**

### §8. 4-Index State

| Index | Version | Notes |
|-------|---------|-------|
| BC-INDEX | v2.82 | Changed D-574 (F-P13-004: BC-5.41.001 v1.7 cite convention). total_bcs 1966 UNCHANGED. |
| VP-INDEX | v2.18 | Changed D-575 (VP-082-BATS-SPLIT closure: VP-082 row version cell v1.5→v1.6). total_vps 86 UNCHANGED. |
| STORY-INDEX | v4.01 | UNCHANGED at D-561..D-574. E-18 stories S-18.00..S-18.08 NOT YET AUTHORED (F3 next after 3-CLEAN). |
| ARCH-INDEX | v2.37 | UNCHANGED at D-574 (last changed D-571; ADR-026 v1.9 verified clean D-572). |

4-index at D-575 (literal-shell): `grep "^version:" .factory/specs/behavioral-contracts/BC-INDEX.md` → "2.82"; `grep "^version:" .factory/specs/verification-properties/VP-INDEX.md` → "2.18"; `grep "^version:" .factory/stories/STORY-INDEX.md` → "4.01"; `grep "^version:" .factory/specs/architecture/ARCH-INDEX.md` → "2.37".

### §9. Critical Anchors

- **factory-artifacts HEAD:** D-575 commit (see Active Branches — SHA populated post-push); prior: `2ebaca02` D-574 F2 E-18 pass-13 fix burst 2026-06-14; prior-prior: `0d70b606` D-573
- **develop HEAD:** `7e99f6ef` (PR #186 fix + release.yml sync back-merge 2026-06-13)
- **main HEAD:** `caf06c68` (rc.21 bot bundle commit 2026-06-13)
- **v1.0.0-rc.21 tag:** `03054524` (SHIPPED; FULLY IN OPERATOR MARKETPLACE)
- **ADR-025 v1.6 SHIPPED:** guard at `3b2a378c`; ARCH-INDEX v2.27
- **S-17.04 story:** `.factory/stories/S-17.04-mid-burst-heartbeat-renewal-wiring.md` v1.7 MERGED; E-17 W4 COMPLETE; PR #184 3b2a378c
- **Verify on resume:** `git rev-parse --short origin/develop` → expect `7e99f6ef`; `git rev-parse --short origin/main` → expect `caf06c68`

### §10. PR Status

- **0 open feature PRs. 0 open release PRs. 0 open marketplace PRs. rc.21 100% COMPLETE. E-18 F2 spec evolution staged (no PR yet — F2 adversarial passes in progress; F3 next).**
- **marketplace PR drbothen/claude-mp #13 MERGED** 2026-06-13 — rc.21 FULLY SHIPPED.
- **RELEASING.md Step 9 VERIFIED (D-560):** operator cache 1.0.0-rc.21 confirmed (plugin.json + 132 entries). rc.21 end-to-end CLOSED.

### §11. Post-CLEAR/Post-RESET Resume Checklist (zero-context; D-574 refresh)

1. **Verify worktree state:** `git rev-parse --short origin/develop` → expect `7e99f6ef`. `git rev-parse --short origin/main` → expect `caf06c68`. `git -C .factory log -1` (expect D-574 fix burst commit; branch factory-artifacts; clean status).
2. **Read §1-§12 this checkpoint** (all of it; D-574 self-sufficient).
3. **Verify trajectory-tail PC4:** `grep "^current_step:" .factory/STATE.md | grep -oE "trajectory-tail [→0-9]+"` → expect `trajectory-tail →9→9→11→2`. (Unchanged through D-575; sweep bursts do not advance trajectory.)
4. **E-10 CASCADE SEALED D-531.** Do NOT resume without engine-surface material change.
5. **F5 PAUSED** — trajectory →9→9→9→11. Do NOT resume without explicit human direction.
6. **RC.21 100% COMPLETE D-560.** NO remaining release action. Operators: `/plugin update vsdd-factory@claude-mp`.
7. **D-575 F2 E-18 PRE-PASS-14 CONSISTENCY-SWEEP REMEDIATION COMPLETE.** VP-082-BATS-SPLIT closed (VP-082 v1.6; VP-INDEX v2.18). Sweep CLEAN. 3-CLEAN streak **0/3 unchanged**. 4-index BC v2.82/VP v2.18/STORY v4.01/ARCH v2.37. **NEXT: (a) adversary pass-14 (fresh-context; need 3 consecutive clean) → (b) F3 story decomp (S-18.00..S-18.07+S-18.08).**
8. **4-index at D-575:** BC-INDEX v2.82, VP-INDEX v2.18, STORY-INDEX v4.01, ARCH-INDEX v2.37.
9. **ALL dispatches carry:** TD-VSDD-097-EXT + TD-VSDD-099 + TD-VSDD-100 + POLICY 14 5-leg + verification_step 7 4-index gate + INV-019 (a)/(b)/(c) + adversary grep origin/factory-artifacts + D-449(a) literal-shell Dim-2 + POLICY 8 v1.3 parity + POLICY 5 v1.3.1/v1.3.4/v1.3.5/v1.3.6 + D-537 spec-drift routing + D-539 multi-family adversary + O-P8-002 MANDATORY (3rd recurrence).
10. **Latest decision D-575.** F2 pre-pass-14 consistency-sweep REMEDIATION (VP-082-BATS-SPLIT closed; sweep CLEAN; streak 0/3 unchanged). Adversary pass-14 NEXT (need 3 consecutive clean). Then F3 S-18.00..S-18.08.

### §12. Pending Work Items — Strict Resume Ordering (refreshed 2026-06-15 D-575)

| Step | Item | Tier | Gate | Status |
|------|------|------|------|--------|
| ~~1~~-~~prev~~ | ~~rc.21 through E-18 F2 adv passes 1-13~~ | ~~—~~ | ~~—~~ | **ALL CLOSED — D-560..D-574 2026-06-13/14.** |
| ~~1~~ | ~~#173/E-18 consistency-validator sweep~~ | ~~feature~~ | ~~D-574 pass-13 fix complete~~ | **DONE D-575 2026-06-15 — sweep CLEAN.** VP-082-BATS-SPLIT closed; VP-082 v1.6; VP-INDEX v2.18. All other sweep dimensions CLEAN. |
| **1** | **#173/E-18 F2 adversarial re-cascade (pass-14)** | **feature** | Consistency-validator sweep clean (D-575 DONE) | Fresh-context adversary reads package at D-575 versions: ADR-026 v1.9 + BC-4.14.001 v1.8 + BC-7.07.001 v1.7 + BC-5.41.001 v1.7 + BC-5.41.002/5.41.003/6.24.001/7.07.002 v1.6 + BC-1.15.001 v1.2 + VP-082 v1.6 + VP-083 v1.3 + VP-081 v1.3 (VP-INDEX v2.18) + VP-084 v1.6 + VP-085/086 + invariants.md v1.18 + capabilities.md v1.6. **3-CLEAN streak 0/3 (reset)**; need 3 consecutive clean for BC-5.39.001 convergence. **START HERE.** |
| **3** | **#173/E-18 F3 story decomposition** | **feature** | F2 3-CLEAN convergence (or human waiver) | Author S-18.00..S-18.07+S-18.08 (S-18.08 ships mandatory O-P8-002 pure-parse invariant gate). STORY-INDEX v4.01→v4.02+. |
| **4** | **#173 wave-checkpoint** | **implementation** | E-18 F3 done OR human re-sequence | State-durability chain stories S-18.01..S-18.05. Blocked on F3. |
| **5** | **#171 deferred-revalidate** | **implementation** | #173 stories done | Deferred-revalidation story. |
| **6** | **#129 canonical-principle** | **implementation** | human-authorize | Ship canonical-principle in plugin. |
| ~~prior~~ | ~~TD #74/66/67; S-15.03 PRIORITY-A; E-10 cascade; rc.19+rc.20+rc.21; E-17 4 stories; S-15.17~~ | ~~—~~ | ~~—~~ | **ALL COMPLETE/MERGED/SHIPPED** |
| **7c** | **F5 pass-76** | **gated** | EXPLICIT human direction | PAUSED D-386 Option C. Do NOT resume. |
| **8/9** | **UNI-PLUG-001 / SK-MCP-001** | **forward** | human-authorize | PROPOSAL REVIEW-READY |

**[D-414(c) acknowledgment: Section 12 is a non-standard addition for forward-backlog durability.]**

> Previous checkpoint (D-574 F2-E18-ADV-PASS-13-NOT-CLEAN-2026-06-14) archived to: `cycles/v1.0-brownfield-backfill/session-checkpoints.md`
