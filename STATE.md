---
document_type: pipeline-state
level: ops
version: "6.98"
status: draft
producer: state-manager
timestamp: 2026-08-07T20:51:00Z
phase: D-961-RECORDING-BURST-PIPELINE-PAUSED
last_amended: "2026-08-07 (v6.98) — D-961-RECORDING-BURST (state-manager): D-961 codified (8 sub-clauses a-h); 3 lessons; burst-log D-961 8 blocks; ARCH-INDEX v3.47 (ADR-041; total_adrs 40→41; pipe-escape pre-existing defect fixed); policies.yaml v1.4.21 (POLICY 16 ALLOCATOR-CEILING GATE bullet); BC-INDEX v4.52 (product-owner); STORY-INDEX v4.289 (story-writer). SHA-drift corrected: feature/S-21.07 fbb5183c→67ffbdcc LOCAL UNPUSHED (1 ahead of origin 37022ecc). 2 new branches registered: fix/nested-factory-path-derivation @ 09f052a9; fix/d999-sentinel-code-migration @ bf642fd9. ADR-041 sentinel migration COMPLETE (8 code/fixture sites; bf642fd9); POLICY 16 GATE added; D-960(e) SUPERSEDED. E-22 DISSOLVED (S-21.12 re-anchored E-21 W4; epic file deletion PENDING operator execution per D-961(c)). F-S2107-P8-006/-007/-013 CLOSED (implementation-verified at 67ffbdcc); -014 GRANDFATHERED; -016 CLOSED (09f052a9). E-22 security scope → Drift Items (SEC-001; RUSTSEC-2026-0222; RUSTSEC-2026-0204; 7 Dependabot alerts; EAC-002; ADR-033 namespace ambiguity). push.default=current SET. sprint-state sync COMPLETE 14/14. 4-INDEX BC v4.52/VP v2.76/STORY v4.289/ARCH v3.47. streak 0/3 UNCHANGED; trajectory-tail →25→24→20→16 UNCHANGED. pass-9 adversary NEXT. [Prior: 2026-08-07 (v6.97) — SESSION-WRAP-2026-08-07 (state-manager): PIPELINE PAUSED per human /wrap directive; D-960 SRC archived; pass-8 fix burst NEXT; 3 OPEN: -006/-007/-013; -014 GRANDFATHERED; streak 0/3; PR #770 MERGED (wasmtime-wasi 44.0.3; develop 700b4dd3); RUSTSEC-2026-0149+RUSTSEC-2026-0182 CLEARED; 4-INDEX UNCHANGED BC v4.51/VP v2.76/STORY v4.288/ARCH v3.46. [Prior: 2026-08-07 (v6.96) — D-960-S-21.07-PASS-8-RECORD-BURST-INDEX-SYNCS (state-manager): adversary pass-7 COMPLETE (NOT-CLEAN B2/H5/M7/L2; 16 findings; IMPROVING 20→16); D-960 codified; 2 lessons; policies.yaml v1.4.19→v1.4.20; BC-INDEX v4.50→v4.51; STORY-INDEX v4.287→v4.288; ARCH-INDEX v3.45→v3.46; VP-INDEX v2.75→v2.76. 4-INDEX BC v4.51/VP v2.76/STORY v4.288/ARCH v3.46. [Prior: see decision-log.md for full chain]]]"
inputs: []
input-hash: "[live-state]"
traces_to: prd.md
project: vsdd-factory
mode: brownfield
pipeline: PAUSED
current_step: "D-961-RECORDING-BURST (state-manager SHA-patch: e2bfec65 DONE). D-961 codified (8 sub-clauses a-h): (a) ADR-041 ratified — ADR-041 canonical sentinel migration COMPLETE; POLICY 16 ALLOCATOR-CEILING GATE added to policies.yaml; D-960(e) SUPERSEDED; (b) 8 code/fixture sites migrated (bf642fd9 fix/d999-sentinel-code-migration); BC-5.39.007 v1.7; S-15.12 v1.5; (c) S-21.12 re-anchored E-22→E-21 W4; E-22 DISSOLVED; epic file deletion PENDING operator execution (D-961/B proceeds WITHOUT git rm); (d) push.default=current SET; (e) NO REBASE S-21.07/S-21.04 per human ruling; (f) F-S2107-P8-016 CLOSED 3 sites (09f052a9); (g) pass-8 all findings resolved (-006/-007/-013 CLOSED; -014 GRANDFATHERED); (h) sprint-state sync 14/14. 3 lessons appended. ARCH-INDEX v3.47; policies.yaml v1.4.21; BC-INDEX v4.52 (product-owner); STORY-INDEX v4.289 (story-writer). SHA-drift: S-21.07 fbb5183c→67ffbdcc LOCAL UNPUSHED (1 ahead of origin 37022ecc). E-22 security scope → Drift Items (SEC-001; RUSTSEC-2026-0222; RUSTSEC-2026-0204; 7 Dependabot alerts; EAC-002; ADR-033 ambiguity). streak 0/3 UNCHANGED; trajectory-tail →25→24→20→16 UNCHANGED. pass-9 adversary dispatch NEXT (F-S2107-P8-006/-007/-013 adversarial re-verification required under iron law). 4-INDEX: BC v4.52; VP v2.76; STORY v4.289; ARCH v3.47. parent-commit: 9c54f35e."
current_cycle: v1.0-brownfield-backfill
dtu_required: false
dtu_assessment: 2026-04-25
dtu_clones_built: "n/a"
dtu_services: []
---

<!--
  STATE.md SIZE BUDGET (per D-421(c) + D-422(c) reconciliation):
  Soft target: <=415 lines; hard cap: 500 lines (validate-state-md-size hook enforcement).
  D-446(c) dual-margin form: margin from soft-target = 500 - 415 = 85; margin from actual tracked below.
  Historical content belongs in cycle files, NOT here.

  D-430(a) compaction history D-532..D-808 (see decision-log.md for full range) COLLAPSED 2026-07-12 (range 393..500 lines; 49+ entries archived; full per-burst notes in factory-artifacts git log; SoT: git show 903aa863:.factory/STATE.md for D-828 pre-compaction state).
  D-862 E21-PHASE-3-W1-DISPATCH-APPROVED 2026-07-20; v6.08→v6.09; SM governance/dispatch burst; PIPELINE UNPAUSED; 4-index ALL UNCHANGED BC v4.11/VP v2.72/STORY v4.227/ARCH v3.11. Full per-burst wc-l history D-819..D-861 (see decision-log.md for full range) archived; SoT: decision-log.md + git show 9debd920:.factory/STATE.md for D-861 pre-compaction state.
  351 lines (wc-l post-update; D-890 W1-WAVE-GATE-BOOKKEEPING-FIX 2026-07-24; v6.26→v6.27; soft-target margin 415-351=+64 UNDER-SOFT-TARGET)
  ~255 lines (estimated post-D-943 (see decision-log.md for full range) PASS-28-RECORD-BURST 2026-07-29; v6.73→v6.74; soft-target margin 415-255=+160 WELL-UNDER-SOFT-TARGET)
  ~260 lines (estimated post-D-944 PASS-29-RECORD-BURST 2026-07-30; v6.74→v6.75; soft-target margin 415-260=+155 WELL-UNDER-SOFT-TARGET)
  ~268 lines (estimated post-D-945 BC-5.39.010-ADR-035-S-21.07-DESIGN-ARC 2026-07-30; v6.75→v6.76; soft-target margin 415-268=+147 WELL-UNDER-SOFT-TARGET)
  ~270 lines (estimated post-D-946 PASS-30-RECORD-BURST 2026-07-31; v6.76→v6.77; soft-target margin 415-270=+145 WELL-UNDER-SOFT-TARGET)
  ~273 lines (estimated post-D-947 PASS-30-FIX-BURST 2026-07-31; v6.77→v6.78; soft-target margin 415-273=+142 WELL-UNDER-SOFT-TARGET)
  ~281 lines (estimated post-D-948 SESSION-WRAP-PAUSED 2026-08-03; v6.78→v6.79; soft-target margin 415-281=+134 WELL-UNDER-SOFT-TARGET)
  ~290 lines (estimated post-D-949 S-21.07-PASS-1-FIX-BURST-PARTIAL 2026-08-03; v6.79→v6.80; soft-target margin 415-290=+125 WELL-UNDER-SOFT-TARGET)
  ~291 lines (estimated post-D-949-erratum STORY-INDEX v4.279 bats-recharacterization 2026-08-03; v6.80→v6.81; soft-target margin 415-291=+124 WELL-UNDER-SOFT-TARGET)
  ~278 lines (estimated post-D-950 S-21.07-INTEGRATION-CLOSURE 2026-08-03; v6.81→v6.82; soft-target margin 415-278=+137 WELL-UNDER-SOFT-TARGET)
  ~284 lines (estimated post-D-951 S-21.07-PASS-2-RECORD-BURST 2026-08-03; v6.82→v6.83; soft-target margin 415-284=+131 WELL-UNDER-SOFT-TARGET)
  ~295 lines (estimated post-D-952 ADR-036-HASH-AUTHORITY-MIGRATION body-update 2026-08-03; v6.84; soft-target margin 415-295=+120 WELL-UNDER-SOFT-TARGET)
  ~318 lines (estimated post-D-953 ADR-037-VOLATILE-INPUTS-RULING-BC-5.39.010-V1.6-CLASS-D-DESCOPE body-update 2026-08-04; v6.85; soft-target margin 415-318=+97 WELL-UNDER-SOFT-TARGET)
  ~302 lines (estimated post-SESSION-WRAP-2026-08-04 PIPELINE PAUSED 2026-08-04; v6.85→v6.86; soft-target margin 415-302=+113 WELL-UNDER-SOFT-TARGET)
  ~322 lines (estimated post-D-954 S-21.07-PASS-3-RECORD-BURST 2026-08-04; v6.86→v6.87; soft-target margin 415-322=+93 WELL-UNDER-SOFT-TARGET)
  ~335 lines (estimated post-D-955 S-21.07-PASS-4-RECORD-BURST-INDEX-SYNCS 2026-08-04; v6.87→v6.88; soft-target margin 415-335=+80 UNDER-SOFT-TARGET)
  ~310 lines (estimated post-D-956 S-21.07-PASS-5-RECORD-BURST-INDEX-SYNCS 2026-08-05; v6.88→v6.89; soft-target margin 415-310=+105 UNDER-SOFT-TARGET)
  ~308 lines (estimated post-SESSION-WRAP-2026-08-05 PIPELINE PAUSED 2026-08-05; v6.89→v6.90; soft-target margin 415-308=+107 UNDER-SOFT-TARGET)
  314 lines (wc-l post-update; D-957-sha-patch SHA-patch follow-up 2026-08-06; v6.91; soft-target margin 415-314=+101 UNDER-SOFT-TARGET)
  314 lines (wc-l post-update; SESSION-WRAP-2026-08-06 PIPELINE PAUSED 2026-08-06; v6.91; soft-target margin 415-314=+101 UNDER-SOFT-TARGET)
  315 lines (wc-l post-update; SESSION-WRAP-2026-08-06-ERRATUM SRC push-status correction 2026-08-06; v6.91; soft-target margin 415-315=+100 UNDER-SOFT-TARGET)
  ~325 lines (estimated post-D-958 S-21.07-PASS-7-RECORD-BURST-INDEX-SYNCS 2026-08-06; v6.91→v6.92; soft-target margin 415-325=+90 UNDER-SOFT-TARGET)
  ~345 lines (estimated post-D-959 ADVERSARIAL-AUTHORSHIP-INTEGRITY-CORRECTION 2026-08-06; v6.92→v6.93; soft-target margin 415-345=+70 UNDER-SOFT-TARGET)
  331 lines (wc-l post-update; SESSION-WRAP-2026-08-06 PIPELINE PAUSED 2026-08-06; v6.93→v6.94; soft-target margin 415-331=+84 UNDER-SOFT-TARGET)
  334 lines (wc-l post-update; push-status-correction SESSION-WRAP-2026-08-06 2026-08-06; v6.94→v6.95; soft-target margin 415-334=+81 UNDER-SOFT-TARGET)
  ~336 lines (estimated post-D-960 S-21.07-PASS-8-RECORD-BURST-INDEX-SYNCS 2026-08-07; v6.95→v6.96; soft-target margin 415-336=+79 UNDER-SOFT-TARGET)
  329 lines (wc-l post-D-960 S-21.07-PASS-8-RECORD-BURST-INDEX-SYNCS SHA-patch 2026-08-07; v6.96; Commit E 46b7cef2 pushed; soft-target margin 415-329=+86 UNDER-SOFT-TARGET)
  338 lines (wc-l post-SESSION-WRAP-2026-08-07 PIPELINE PAUSED 2026-08-07; v6.96→v6.97; soft-target margin 415-338=+77 UNDER-SOFT-TARGET)
  337 lines (wc-l post-D-961 RECORDING-BURST SHA-patch 2026-08-07; v6.98; soft-target margin 415-337=+78 UNDER-SOFT-TARGET)
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
| **Last Updated** | 2026-08-07 — D-961-RECORDING-BURST PIPELINE PAUSED: all pass-8 findings resolved (-006/-007/-013 CLOSED implementation-verified; -016 CLOSED; -014 GRANDFATHERED); E-22 DISSOLVED (epic file deletion pending operator); ADR-041 sentinel COMPLETE; 4-INDEX BC v4.52/VP v2.76/STORY v4.289/ARCH v3.47; streak 0/3; trajectory-tail →25→24→20→16; pass-9 adversary NEXT. |
| **Current Phase** | **D-961-RECORDING-BURST (2026-08-07). PIPELINE PAUSED. All pass-8 findings resolved. E-22 DISSOLVED (epic file deletion pending operator execution). ADR-041 sentinel migration COMPLETE. streak 0/3 (7 true adversary passes; 0 CLEAN); trajectory-tail →25→24→20→16; last D-NNN: D-961. develop 700b4dd3 (PR #770 MERGED; wasmtime-wasi 44.0.3 live). main 80e5cd7b. merged_count 107. 4-INDEX BC v4.52/VP v2.76/STORY v4.289/ARCH v3.47.** |
| **Current Cycle** | v1.0-brownfield-backfill |

## Phase Progress

| Phase | Status | Artifact |
|-------|--------|----------|
| Phases 0-B..D-647 ALL COMPLETE/ARCHIVED: Waves 1-11, rc.11..rc.20, E-10 SEALED D-531, E-17 3/3, S-15.03/S-15.17, F2 passes 1-43 CONVERGED D-606, F3 integration, F4 W1-W2 CONVERGED D-647 | **ALL COMPLETE / ARCHIVED** | git show 903aa863:.factory/STATE.md Phase Progress for pre-compaction 18-row table. SoT: decision-log.md + burst-log.md. |
| v1.0.0-rc.20 SHIPPED 2026-06-01; v1.0.0-rc.21 SHIPPED 2026-06-13; v1.0.0-rc.22 SHIPPED 2026-07-03 | **ALL SHIPPED** | PRs merged; marketplace published; v1.0.0-rc.22 tag `e4285fe5`. |
| D-856 RC23-SHIPPED 2026-07-18: v1.0.0-rc.23 SHIPPED; PR #688 --merge 45050445; recovery PR #689; retag 0f8b2a89; bot commit 80e5cd7b; POLICY 20 34/34 WASMs; marketplace claude-mp#18 MERGED 2026-07-18T22:48:17Z; STATE.md v6.02→v6.03 | **SHIPPED** | GitHub Release v1.0.0-rc.23 (prerelease); marketplace vsdd-factory 1.0.0-rc.23 |
| D-862 E21-PHASE-3-W1-DISPATCH-APPROVED 2026-07-20: E-21 Phase-3 W1 dispatch APPROVED SEQUENTIAL; input-drift resolved; PIPELINE UNPAUSED; STATE.md v6.08→v6.09 | **COMPLETE** | PIPELINE UNPAUSED — S-21.01..S-21.04 delivered |
| D-941 SESSION-WRAP-PAUSED 2026-07-29: S-21.04 pass-27 CLOSED; streak 0/3; trajectory-tail →6→17→11→7 | **COMPLETE** | PIPELINE PAUSED (session wrap) |
| D-943..D-947 (see decision-log.md for full range) PASSES-28-30 2026-07-29..2026-07-31: passes 28/29/30 record+fix; BC v4.38→v4.43; STORY v4.272→v4.277; ARCH v3.37→v3.40; trajectory-tail →17→11→7→17→13→16; streak 0/3 | **COMPLETE** | PIPELINE ACTIVE |
| D-948..D-953 (see decision-log.md for full range) SESSION-WRAP + PASSES-1-2 + ADR-036/037 2026-08-03..2026-08-04: S-21.07 pass-1+2; ADR-036 ALGORITHM-DIVERGENT; ADR-037 Class D DESCOPED; BC v4.43→v4.46; ARCH v3.40→v3.42; STATE.md v6.78→v6.85 | **COMPLETE** | PIPELINE ACTIVE |
| D-954 S-21.07-PASS-3-RECORD-BURST 2026-08-04: NOT-CLEAN B3/H7/M12/L3 (25 findings); REGRESSION 18→25; trajectory-tail →47→18→25→25 | **COMPLETE** | PIPELINE ACTIVE — pass-4 fix burst NEXT |
| D-955 S-21.07-PASS-4-RECORD-BURST-INDEX-SYNCS 2026-08-04: NOT-CLEAN B4/H9/M9/L3 (25 findings); FLAT; BC v4.47; STORY v4.284 | **COMPLETE** | PIPELINE ACTIVE |
| D-956 S-21.07-PASS-5-RECORD-BURST-INDEX-SYNCS 2026-08-05 (SHA `e2789993`): NOT-CLEAN B3/H8/M10/L3 (24 findings); FLAT-MINUS-ONE; trajectory-tail →18→25→25→24 | **COMPLETE** | PIPELINE ACTIVE |
| D-957 S-21.07-PASS-6-RECORD-BURST-INDEX-SYNCS 2026-08-05 (SHA-patch f0f25194): NOT-CLEAN B4/H7/M8/L1 (20 findings); IMPROVING 24→20; trajectory-tail →25→25→24→20 | **COMPLETE** | PIPELINE ACTIVE |
| D-958 S-21.07-PASS-7-RECORD-BURST-INDEX-SYNCS 2026-08-06 [D-959 RETRACTION: not adversary pass]: FIX-BURST-CLOSURE-VERIFICATION; BC v4.50; ARCH v3.45; VP v2.75; STORY v4.287 | **COMPLETE** | D-959 RETRACTION applied |
| D-959 ADVERSARIAL-AUTHORSHIP-INTEGRITY-CORRECTION 2026-08-06: D-958 retracted; streak 0/3; trajectory-tail →25→25→24→20; correction commit a0d87706 | **COMPLETE** | PIPELINE ACTIVE |
| SESSION-WRAP-2026-08-06 PIPELINE PAUSED 2026-08-06 | **COMPLETE** | 4-INDEX UNCHANGED BC v4.50/VP v2.75/STORY v4.287/ARCH v3.45 |
| D-960 S-21.07-PASS-8-RECORD-BURST-INDEX-SYNCS 2026-08-07: adversary pass-7 COMPLETE (NOT-CLEAN B2/H5/M7/L2; 16 findings; IMPROVING 20→16); D-960 codified (7 sub-clauses); policies.yaml v1.4.19→v1.4.20; BC v4.51; STORY v4.288; ARCH v3.46; VP v2.76; trajectory-tail →25→24→20→16; SHA-patch DONE: 46b7cef2. | **COMPLETE** | PIPELINE ACTIVE — D-961 recording burst NEXT |
| SESSION-WRAP-2026-08-07 PIPELINE PAUSED 2026-08-07 (SM 9750700d+SHA-patch) | **COMPLETE** | 4-INDEX UNCHANGED BC v4.51/VP v2.76/STORY v4.288/ARCH v3.46; parent-commit: ada929d4. |
| D-961-RECORDING-BURST 2026-08-07: multi-specialist recording+fix burst (single Commit B e2bfec65); D-961 codified (8 sub-clauses a-h); 3 lessons; ARCH-INDEX v3.47; policies.yaml v1.4.21; BC-INDEX v4.52; STORY-INDEX v4.289. All pass-8 findings resolved. E-22 DISSOLVED (epic file deletion PENDING operator). ADR-041 sentinel COMPLETE. 4-INDEX: BC v4.52/VP v2.76/STORY v4.289/ARCH v3.47. streak 0/3 UNCHANGED; trajectory-tail →25→24→20→16 UNCHANGED. parent-commit: 9c54f35e. SHA-patch: e2bfec65 DONE. | **COMPLETE** | pass-9 adversary dispatch NEXT |
| **E-18 CAP-002 context-durability epic (#173): waves 1-9 + prereqs, S-18.00..S-18.14, 18 stories** | **EPIC COMPLETE 2026-07-01 D-744** | Final story S-18.12 MERGED PR #384 ec05606a. All 18 E-18 stories + 2 prereqs merged; merged_count 95→96. Full ledger: `cycles/v1.0-brownfield-backfill/merged-stories-ledger.md`. |

## Current Phase Steps

> **Rows through D-953 archived to** `cycles/v1.0-brownfield-backfill/burst-log.md` + `decision-log.md` per STATE.md content-routing rules (keep last 5 only; advanced SESSION-WRAP-2026-08-05 2026-08-05).

| Step | Agent | Status | Output |
|------|-------|--------|--------|
| D-807..D-953 (see decision-log.md for full range) (archived) | state-manager | ARCHIVED | See `cycles/v1.0-brownfield-backfill/burst-log.md`. D-807..D-835: passes 51-61 CONVERGED. D-836..D-953 (see decision-log.md for full range): W2/W3 delivery, passes 28-30, S-21.07 passes 1-2, ADR-036/ADR-037. |
| D-957 S-21.07-PASS-6-RECORD-BURST-INDEX-SYNCS 2026-08-05 (pass-6; SHA-patch f0f25194) | state-manager | COMPLETE | NOT-CLEAN B4/H7/M8/L1 (20+6); IMPROVING 24→20; BC-INDEX v4.49; ARCH-INDEX v3.44; STORY-INDEX v4.286; streak 0/3 (6 passes); trajectory-tail →25→25→24→20. parent-commit: e3defa50. |
| D-960 S-21.07-PASS-8-RECORD-BURST-INDEX-SYNCS 2026-08-07 (pass-8; SHA-patch DONE: 46b7cef2) | state-manager | COMPLETE | adversary-pass-7.md CREATED (NOT-CLEAN B2/H5/M7/L2; 16+9 obs; IMPROVING 20→16). BC-INDEX v4.50→v4.51; STORY-INDEX v4.287→v4.288; ARCH-INDEX v3.45→v3.46; VP-INDEX v2.75→v2.76. 4-INDEX: BC v4.51; VP v2.76; STORY v4.288; ARCH v3.46. streak 0/3 (7 true adversary passes). trajectory-tail →25→24→20→16. parent-commit: 67a8e3e6. |
| SESSION-WRAP-2026-08-07 PIPELINE PAUSED 2026-08-07 (SM 9750700d+SHA-patch) | state-manager | COMPLETE | PIPELINE PAUSED per human /wrap. D-960 SRC archived. PAUSED SRC written. 4-INDEX UNCHANGED: BC v4.51/VP v2.76/STORY v4.288/ARCH v3.46. trajectory-tail →25→24→20→16. parent-commit: ada929d4. SHA-patch DONE: this-commit. |
| D-961-RECORDING-BURST 2026-08-07 (multi-specialist; Commit B e2bfec65; SHA-patch e2bfec65 DONE) | state-manager | COMPLETE | (1) POLICY 16 GLOBAL-MAX GATE: D-960 confirmed max → D-961 allocated; gate PASS. (2) D-961 codified (8 sub-clauses a-h); 3 lessons; burst-log D-961 8 blocks. (3) ARCH-INDEX v3.47; policies.yaml v1.4.21; BC-INDEX v4.52 (product-owner); STORY-INDEX v4.289 (story-writer). (4) SHA-drift: S-21.07 fbb5183c→67ffbdcc LOCAL UNPUSHED (1 ahead of origin 37022ecc). (5) 2 new branches: fix/nested-factory-path-derivation @ 09f052a9; fix/d999-sentinel-code-migration @ bf642fd9. (6) ADR-041 sentinel migration COMPLETE (8 sites). POLICY 16 GATE added. (7) E-22 DISSOLVED per human ruling; epic file deletion PENDING operator execution; D-961/B proceeds WITHOUT git rm. (8) F-S2107-P8-006/-007/-013 CLOSED; -016 CLOSED; -014 GRANDFATHERED. E-22 security scope → Drift Items. sprint-state 14/14. (9) 4-INDEX: BC v4.52; VP v2.76; STORY v4.289; ARCH v3.47. streak 0/3 UNCHANGED; trajectory-tail →25→24→20→16 UNCHANGED. pass-9 adversary NEXT. parent-commit: 9c54f35e. SHA-patch: e2bfec65 DONE. |

## Identifier Conventions

| Type | Format | Authoritative Source | Count |
|------|--------|----------------------|-------|
| Subsystem | SS-NN | `specs/architecture/ARCH-INDEX.md` | 10 |
| Behavioral Contract | BC-S.SS.NNN | `specs/behavioral-contracts/ss-NN/` | 1,982 (BC-INDEX v4.52 D-961; decision-log.md SoT) |
| Verification Property | VP-NNN | `specs/verification-properties/VP-INDEX.md` | 102 (VP-INDEX v2.76 D-960; decision-log.md SoT) |
| Story | S-N.MM | `stories/S-N.MM-<short>.md` | 129 file-resident + 15 stub IDs (STORY-INDEX v4.289 D-961; decision-log.md SoT) |
| Epic | E-N | `stories/epics/E-N-<short>.md` | 20 (pre-D-961 baseline; E-22 dissolution PENDING operator execution — file on disk, agent git rm blocked per D-961(c); count becomes 21 once file is removed) |
| ADR | ADR-NNN | `specs/architecture/decisions/ADR-NNN.md` | 41 (ADR-041 NEW D-961; ADR-040 NEW D-960; ADR-039 NEW D-958; ADR-038 NEW D-958; ADR-037 v1.0 D-953) |
| Merged Count | merged_count | `stories/sprint-state.yaml` (canonical predicate); `STATE.md` (explicit counter) | 107 (STATE.md explicit counter as of D-851; sprint-state predicate-based count: 113; canonical definition codified D-853) |

## Story Status

128 file-resident + 15 unauthored stub IDs = 143 stories registered. E-18 EPIC COMPLETE D-744 2026-07-01. E-22 DISSOLVED D-961 2026-08-07 (epic file deletion PENDING operator execution). Full ledger: `cycles/v1.0-brownfield-backfill/merged-stories-ledger.md`.

- **Merged (107):** S-19.07 MERGED PR #670 6db4c9fc D-851 (E-19 COMPLETE 9/9). S-19.09 MERGED PR #659 13ece92c D-848. S-19.06 MERGED PR #657 9787c056 D-843. S-19.08 MERGED PR #646 1304d280 D-842. S-19.05 MERGED PR #640 7b35c8e4 D-841. S-19.04 MERGED PR #639 d4a23a02 D-841. S-19.03 MERGED PR #611 091ce499 D-834. S-19.01 MERGED PR #613 8d1721f7 D-833. S-19.02 MERGED PR #610 f5ea92e9 D-832. Also S-17.01..S-17.04 + S-18.00..S-18.14 (E-18 EPIC COMPLETE). Full ledger: `cycles/v1.0-brownfield-backfill/merged-stories-ledger.md`
- **In-Flight (0):** --
- **E-21 (SPEC CONVERGED — Phase-3 W1 dispatch APPROVED D-862):** S-21.01 (W1, P0, 11pts, CAP-034, issue #342); S-21.02 (W1, P1, 3pts, CAP-035, issue #365); S-21.03 (W1, P1, 3pts, CAP-038, issue #358); S-21.04 (W2, P1, 5pts, CAP-036, issue #523); S-21.05 (W2, P1, 5pts, CAP-037, issue #588); S-21.07 (W4, 11pts, POLICY 21 WASM crate; pass-8 all resolved; branch 67ffbdcc LOCAL UNPUSHED (1 ahead origin 37022ecc; NO REBASE per D-961(e))); S-21.09 (wasm-artifact-restore-and-registry-parity; NO branch/PR — must deliver BEFORE S-21.07); S-21.10 (failure-policy-registry-schema-extension, v1.0 d9d5e33); S-21.11 (exhaustion-fail-closed-calibration, v1.0 2fb1b75); S-21.12 (re-anchored E-22→E-21 W4 per D-961(c); E-22 dissolved; epic file deletion PENDING operator execution).
- **Draft (30 file-resident):** S-4.11; S-5.07; S-10.09; S-11.00; S-14.01..S-14.09 (E-14); S-15.02; S-15.03; S-16.01..S-16.02 (E-16); and others
- **Partial (2):** S-2.05 (hook-sdk-publish); S-3.04 (emit-event-host-function) — superseded by ADR-015
- **Withdrawn (1):** S-9.30

## Active Branches

| Branch / Tag | SHA | Notes |
|--------------|-----|-------|
| main | 80e5cd7b | rc.23 bot binary bundle commit 2026-07-18 |
| develop | 700b4dd3 | fix(security): bump wasmtime-wasi 44.0.1→44.0.3 (#770 squash-merge 2026-08-07T18:44:24Z). Local stale at 948f0fb1 — pull on resume. |
| factory-artifacts | e2bfec65 | D-961/B SHA-patch DONE. |
| feature/S-21.04 | 323f440f | feat(S-21.04): pass-30 fix burst COMPLETE; bats 11/11 + 16/16 GREEN; NO PR open (mid-cascade) |
| feature/S-21.07 | 67ffbdcc | feat(S-21.07): pass-8 all findings resolved (implementation-verified). LOCAL ONLY — 1 ahead of origin 37022ecc. UNPUSHED per human ruling D-961(e) — NO REBASE. MERGE-ORDER: S-21.09 MUST land first. |
| fix/nested-factory-path-derivation | 09f052a9 | F-S2107-P8-016 CLOSED — 3 unguarded .join(".factory") sites fixed + both-polarity tests. LOCAL, NOT pushed, no PR. Off develop. |
| fix/d999-sentinel-code-migration | bf642fd9 | ADR-041 sentinel migration — 8 code/fixture sites migrated. LOCAL in worktree .worktrees/d999-migration, NOT pushed, no PR. |
| fix/wasmtime-wasi-cve-2026-47261 | 23e307bb | MERGED via PR #770 2026-08-07T18:44:24Z; remote branch auto-deleted. Dead local branch — delete on resume (git checkout develop first). |
| feature/S-21.01 | DELETED (was merged) | MERGED via PR #759 2026-07-23 (D-879); branch deleted at merge |
| feature/S-21.02 | DELETED (was merged) | MERGED via PR #760 2026-07-24 (D-880); branch deleted at merge |
| feature/S-21.03 | DELETED (was merged) | MERGED via PR #761 2026-07-24 (D-881); branch deleted at merge |
| v1.0.0-rc.23 (tag) | 0f8b2a89 | SHIPPED 2026-07-18; FULLY IN OPERATOR MARKETPLACE |
| v1.0.0-rc.22 (tag) | e4285fe5 | SHIPPED 2026-07-03; FULLY IN OPERATOR MARKETPLACE |
| v1.0.0-rc.21 (tag) | 03054524 | SHIPPED 2026-06-13; FULLY IN OPERATOR MARKETPLACE |

## Concurrent Cycles

| Cycle | Type | Status | Notes |
|-------|------|--------|-------|
| F-block-ai-attribution-message-file-arm | feature | F3 COMPLETE — F4 READY | F1+F2+F3 done 2026-05-12; 2 stories ready; E-16 under SS-07/SS-04; milestone v1.0.0-rc.17 |
| v1.0-brownfield-backfill | brownfield | D-961-RECORDING-BURST PIPELINE PAUSED. develop 700b4dd3 (PR #770 MERGED — wasmtime-wasi 44.0.3 live); main 80e5cd7b; merged_count 107; BC-INDEX v4.52; VP-INDEX v2.76; STORY-INDEX v4.289; ARCH-INDEX v3.47; streak 0/3 (7 true adversary passes); trajectory-tail →25→24→20→16 (LENGTH=4). All pass-8 findings resolved. E-22 DISSOLVED (epic file pending operator). ADR-041 sentinel COMPLETE. pass-9 adversary NEXT. | D-961-RECORDING-BURST 2026-08-07; D-960 S-21.07-PASS-8-RECORD-BURST-INDEX-SYNCS 2026-08-07; D-959 ADVERSARIAL-AUTHORSHIP-INTEGRITY-CORRECTION 2026-08-06; D-958 S-21.07-PASS-7-RECORD-BURST-INDEX-SYNCS 2026-08-06 [RETRACTED]. |
| v1.0-feature-engine-discipline-pass-1 | feature | PAUSED | F5 pass-75 adversary complete D-510 2026-05-27; META-LEVEL-30 CANDIDATE-CONFIRMED; trajectory →9→9→9→11. |
| v1.0-feature-plugin-async-semantics-pass-1 | feature | CLOSED | All PRs merged; rc.14 shipped |

## Decisions Log

> D-001..D-606 (see decision-log.md for full range): `cycles/v1.0-brownfield-backfill/decision-log.md` + `decisions-log-archive.md`. D-379..D-454 (see decision-log.md for full range) (F5): `cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md`. D-607..D-961 (see decision-log.md for full range): this Decisions Log (D-956..D-961 live) + decision-log.md SoT.

| ID | Decision | Summary | Phase | Date |
|----|----------|---------|-------|------|
| D-961 | D-961-RECORDING-BURST. Multi-specialist recording+fix burst (single-commit Commit B; TD-VSDD-053) 2026-08-07. D-961 codified (8 sub-clauses): (a) ADR-041 ratified — ADR-041 canonical sentinel migration COMPLETE; POLICY 16 ALLOCATOR-CEILING GATE added; D-960(e) SUPERSEDED; (b) 8 code/fixture sites migrated (bf642fd9); BC-5.39.007 v1.7; S-15.12 v1.5; (c) S-21.12 re-anchored E-22→E-21 W4; E-22 DISSOLVED; epic file deletion PENDING operator execution (D-961/B WITHOUT git rm); (d) push.default=current SET; (e) NO REBASE S-21.07/S-21.04 per human ruling; (f) F-S2107-P8-016 CLOSED 3 sites (09f052a9); (g) all pass-8 findings resolved (-006/-007/-013 CLOSED; -014 GRANDFATHERED); (h) sprint-state sync 14/14. 3 lessons. ARCH-INDEX v3.47. policies.yaml v1.4.21. BC-INDEX v4.52. STORY-INDEX v4.289. SHA-drift: S-21.07 fbb5183c→67ffbdcc LOCAL UNPUSHED. E-22 security scope → Drift Items. 4-INDEX: BC v4.52; VP v2.76; STORY v4.289; ARCH v3.47. streak 0/3 UNCHANGED; trajectory-tail →25→24→20→16 UNCHANGED. parent-commit: 9c54f35e. SHA-patch: e2bfec65 DONE. | D-961-RECORDING-BURST 2026-08-07; all pass-8 resolved; E-22 dissolved (file pending operator); ADR-041 sentinel COMPLETE; 4-INDEX BC v4.52/VP v2.76/STORY v4.289/ARCH v3.47; streak 0/3 | D-961-RECORDING-BURST | 2026-08-07 |
| D-960 | S-21.07-PASS-8-RECORD-BURST-INDEX-SYNCS. Single-commit RECORD+INDEX-SYNC burst (TD-VSDD-053) 2026-08-07; Commit-E 46b7cef2. adversary-pass-7.md CREATED (NOT-CLEAN B2/H5/M7/L2; 16 findings + 9 obs; IMPROVING 20→16; reviewed HEAD fbb5183c). D-960 codified (7 sub-clauses a-g). 2 lessons. policies.yaml v1.4.19→v1.4.20. BC-INDEX v4.50→v4.51. STORY-INDEX v4.287→v4.288. ARCH-INDEX v3.45→v3.46 (ADR-040). VP-INDEX v2.75→v2.76. 4-INDEX: BC v4.51; VP v2.76; STORY v4.288; ARCH v3.46. streak 0/3 (7 true adversary passes). trajectory-tail →25→24→20→16. SHA-patch DONE: 46b7cef2. | S-21.07-PASS-8-RECORD-BURST-INDEX-SYNCS 2026-08-07; NOT-CLEAN B2/H5/M7/L2 (16+9); IMPROVING 20→16; BC-INDEX v4.51; ARCH-INDEX v3.46; streak 0/3 (7 passes) | D-960-S-21.07-PASS-8-RECORD-BURST-INDEX-SYNCS | 2026-08-07 |
| D-959 | ADVERSARIAL-AUTHORSHIP-INTEGRITY-CORRECTION. Single-commit correction burst 2026-08-06; correction commit a0d87706; SHA-patch 159bb192. D-958 self-attestation retracted: adversary-pass-7.md → fix-burst-closure-verification-pass-7.md RENAMED; streak 0/3 REVERTED; trajectory-tail →0 append REVERTED. 4-INDEX UNCHANGED: BC v4.50/VP v2.75/STORY v4.287/ARCH v3.45. Iron Law codified: adversary pass may ONLY be authored by vsdd-factory:adversary under fresh context. | ADVERSARIAL-AUTHORSHIP-INTEGRITY-CORRECTION 2026-08-06; D-958 RETRACTED; streak 0/3; trajectory-tail →25→25→24→20; correction commit a0d87706 | D-959-ADVERSARIAL-AUTHORSHIP-INTEGRITY-CORRECTION | 2026-08-06 |
| D-958 | [D-959 RETRACTION: adversary-pass-7.md was state-manager self-verification (Iron Law violated); CLEAN verdict VOID; streak NOT advanced.] S-21.07-PASS-7-RECORD-BURST-INDEX-SYNCS. FIX-BURST-CLOSURE-VERIFICATION. SHA-patch DONE: 61e23c44. BC v4.49→v4.50; VP v2.74→v2.75; STORY v4.286→v4.287; ARCH v3.44→v3.45. streak 0/3 UNCHANGED (6 true adversary passes). | [D-959 RETRACTED]; implementation-verified only; BC-INDEX v4.50; ARCH-INDEX v3.45; streak 0/3 UNCHANGED | D-958-S-21.07-PASS-7-RECORD-BURST-INDEX-SYNCS | 2026-08-06 |
| D-957 | S-21.07-PASS-6-RECORD-BURST-INDEX-SYNCS. Single-commit (TD-VSDD-053) 2026-08-05; SHA-patch f0f25194. NOT-CLEAN B4/H7/M8/L1 (20+6); IMPROVING 24→20. BC-INDEX v4.48→v4.49; ARCH-INDEX v3.43→v3.44; STORY-INDEX v4.285→v4.286. streak 0/3 (6 passes). trajectory-tail →25→25→24→20. | S-21.07-PASS-6-RECORD-BURST-INDEX-SYNCS 2026-08-05; IMPROVING 24→20; BC-INDEX v4.49; ARCH-INDEX v3.44; streak 0/3 (6 passes) | D-957-S-21.07-PASS-6-RECORD-BURST-INDEX-SYNCS | 2026-08-05 |
| D-956 | S-21.07-PASS-5-RECORD-BURST-INDEX-SYNCS. Single-commit (TD-VSDD-053) 2026-08-05; SHA `e2789993`. NOT-CLEAN B3/H8/M10/L3 (24+5); FLAT-MINUS-ONE 25→24. BC-INDEX v4.47→v4.48; ARCH-INDEX v3.42→v3.43; STORY-INDEX v4.284→v4.285. streak 0/3 (5 passes). trajectory-tail →18→25→25→24. | S-21.07-PASS-5-RECORD-BURST-INDEX-SYNCS 2026-08-05; FLAT-MINUS-ONE; BC-INDEX v4.48; ARCH-INDEX v3.43; streak 0/3 (5 passes) | D-956-S-21.07-PASS-5-RECORD-BURST-INDEX-SYNCS | 2026-08-05 |
| D-413..D-955 (see decision-log.md for full range) | **ARCHIVED** | Full detail in decision-log.md SoT. | ARCHIVED | 2026-06-14..2026-08-04 |

## Skip Log

| Step | Skipped? | Justification |
|------|----------|---------------|
| UX Spec | yes | CLI-only product with no UI surfaces |
| Gene Transfection Assessment | yes | Not applicable — engine and product are same repo |

## Blocking Issues

| Blocker | Status | Risk Statement |
|---------|--------|----------------|
| **[P0] `validate-factory-path-staging` WASM guard inert since 2026-07-23** | **OPEN** | Guard has fired 0 times since deployment vs 889 invocations by sibling guards. `on_error = "continue"` makes missing plugin indistinguishable from passing. Impacts BC-4.16.001 cross-site-correspondence validation enforcement. Fix story: S-21.09. MUST land before S-21.07. |
| **rc.23 RELEASE-GATE BLOCKER** | **RESOLVED — v1.0.0-rc.23 SHIPPED 2026-07-18 D-856** | Retired. |

## Drift Items / Tech Debt

| Item | Status | Notes |
|------|--------|-------|
| **TD #67/68/69/70/71/72/74 ALL RESOLVED** | **RESOLVED** | ARCHIVED per D-754 compaction. decision-log.md SoT. |
| Ghost BCs: BC-3.07.003/004, BC-1.06.011 | DEFERRED | Missing from BC-INDEX; investigate in future fix-burst |
| **TD-VSDD-061 (F-P6-002)** | OPEN 2026-05-17 | validate-index-cite-refresh + validate-burst-log `host::read_file(...65536...)` against files >64KiB — silent fail-open. |
| **TD-VSDD-062/063** | OPEN 2026-05-17/19 | Schema inconsistencies in M2 stories (LOW); deferred VP allocation for BC-5.39.006 9 pending VPs. |
| **PG-S-15.11-bats-prod-registry-parity-gate** | OPEN 2026-05-17 | Bats inline `path_allow` arrays must be byte-identical to production hooks-registry.toml. |
| **TD-VSDD-095..100 (CODIFIED-LESSONS)** | CODIFIED-AND-FORWARDED-TO-SK-MCP-001 | 6-class META-LEVEL perimeter disciplines. |
| **TD-VSDD-101 (CI env-var paper-fix)** | OPEN 2026-05-18 — anchored S-15.15 | `VSDD_SKIP_PRODUCTION_STATE_MD_TEST=1` skips production STATE.md bats test in CI. |
| **S-15.17-CR-001/002** | ACCEPTED-DEFERRED 2026-05-31 | `check_index_sites` + `rows_after_heading` advisory-arm defects. |
| **test_F_P2_001 / resolver-integration timing flake** | **RESOLVED 2026-07-02 — PR #431 35b345f4 (D-749)** | wall-clock lower-bound replaced with InternalLog JSONL behavioral assertion. |
| **RUSTSEC-2026-0149** | **RESOLVED 2026-08-07 — PR #770 MERGED (wasmtime-wasi 44.0.3)** | CVE-2026-47261/GHSA-2r75-cxrj-cmph CLEARED. RUSTSEC-2026-0182 also CLEARED. |
| **O-PASS16-002 header stale doc-comment** | OPEN 2026-06-08 | Stale function header. Cosmetic; next spec-touch. |
| **VP-087 DEFERRED (null-SHA hard-block VP)** | DEFERRED 2026-06-15 | Create VP-087 if future adversary flags missing coverage. |
| **[D-703 drift-1] stale precompact-flush.sh ref** | **RESOLVED 2026-06-27 — PR #304 e10dedc0 (D-709)** | Tree-wide TD-VSDD-060 sibling-sweep. |
| **bats-full-suite not in branch-protection required-status-checks** | OPEN 2026-06-13 | `bats-full-suite (linux)` runs but NOT in required-checks. |
| **[D-703 drift-2] S-18.07 docs ADR-028 §Decision citation** | **CLOSED-ACCEPTED (D-709)** | ADR-028 §Decision 2 prose accurate. |
| **[system-level deferral] ARCH-INDEX §Future Sections** | **RESOLVED 2026-06-16 — D-607** | verification-architecture.md + verification-coverage-matrix.md materialized. |
| **[tool-fix] compute-input-hash awk+resolver bug (D-616)** | **RESOLVED D-618** — PR #189 SQUASH-MERGED c000b06f 2026-06-16. | CWE-22 guard + awk exit-condition bug fixed. |
| **BC-INDEX count reconcile (pre-existing)** | **RESOLVED 2026-06-17 — D-619** | total_bcs 1968→1972. |
| **S-18.08 phantom-field-removal lint gate** | DRAFT-PENDING-AUTHORING | Anchor: E-18 F3. |
| **[process-gap] BC-Precondition registry-block shape validator gate** | OPEN 2026-06-15 | Anchor: E-18 F3. |
| **[process-gap] Cross-reference title/code/phrase sweep gate** | CODIFIED D-582; UPGRADED D-589 | MECHANICAL GATE NOW MANDATORY. Anchor: S-18.08. |
| **[process-gap] Canonical-scope-verification discipline** | CODIFIED 2026-06-15 — D-587 | WASM reads field-4 statically. |
| **[process-gap] Stale-term-deferral-unsafe discipline** | CODIFIED 2026-06-15 — D-594 FULL BACKLOG CLEARANCE | Stale terms in normative present-tense prose MUST be fixed in-scope. |
| **[forward-track] F3 VP obligations** | FORWARD-TRACKED — anchor E-18 F3. | E-18 F3 story decomposition. |
| **O-P9-001 + L-S18-macos-ci-leg-caught-runtime-portability** | **ANCHORED S-18.11+S-18.12 (MERGED)** | sprint-state.yaml producer + portability-lint guard. |
| **[process-gap] input_hash placeholder not gated on draft→ready** | OPEN 2026-06-22 — D-684 | Anchor: E-18 F3 family. |
| **[process-gap] ADR/BC-version pin lint missing — BOTH word-orders required** | OPEN 2026-06-22 — D-685 | Anchor: E-18 F3. |
| **[process-gap] CI-green-attestation gate — premature 'CI N/N GREEN'** | OPEN 2026-06-23 — D-692 | |
| **[process-gap] scripts/generate-registry-from-hooks-json.sh + legacy JSON tombstone** | OPEN 2026-06-25 | Anchor: E-18 F3. |
| **[process-gap] WASM hook stories must build real .wasm + run bats before TDD-green** | OPEN 2026-06-24 — D-693 | RULE: MUST build actual .wasm + run bats as pre-green gate. |
| **[D-743] sprint-state.yaml status not auto-synced on STORY-INDEX transitions** | **CLOSED D-961(h) — sprint-state sync COMPLETE 14/14** | S-21.10/S-21.11/S-21.12 added; S-21.06 depth corrected; sprint-state-format.bats 14/14. |
| **[D-749 process-gap] merge-race-ready-report-stale-head** | OPEN 2026-07-02 | PR-cycle READY verdicts MUST pin exact covered HEAD SHA. |
| **[D-750 process-gap] release-PR merge-strategy not mechanically enforced** | OPEN 2026-07-04 — AWAITING HUMAN AUTHORIZATION | Proposed cure: repo ruleset `main-merge-commits-only`. |
| **[D-751 functional] verify-factory-lock silently degraded + 3 orphan WASMs** | **RESOLVED — S-19.02/S-19.04 MERGED** | |
| **[D-750 process-gap] simulation-shell-dialect gap** | OPEN 2026-07-04 | `mapfile` bash 4.0+ vs macOS 3.2. |
| **[D-762 hook false-positive] validate-count-propagation regex** | OPEN 2026-07-07 | Regex false-positive on changelog narrative. Root fix: scope regex. |
| **[D-766 O-P15-01] BC frontmatter `cycle:` field inconsistent** | OPEN 2026-07-08 | Human adjudication required. |
| **[D-773] Legacy epic pre-existing template drift (6 epics)** | OPEN 2026-07-08 | E-8/E-9/E-10/E-12/E-15/E-17/E-18 missing sections. |
| **[O-P35-001 + D-805] E-17-lineage 3-tool-form sites** | OPEN | ADR-025 v1.2 volatile-pins + stale 3-tool form. |
| **[O-P60-001] ADR-025 §Decision intro "ten numbered decisions" vs 15 present** | OPEN — D-821 | Route: architect at next ADR-025 touch. |
| **[O-P61-001] VP-097 §Source Contract §Invariant 1 over-broad NOT_FOUND clause** | OPEN — D-823 | Route: architect at next VP-097 touch. |
| **[D-826 W1-tracked] Kani infra gap + VP-097 spec-drift** | OPEN 2026-07-11 | Kani toolchain incompatibility; VP-097 stale signature. |
| **[D-838 process-gap] DI-001..018 in invariants.md missing Cited-by lines** | OPEN 2026-07-13 | Anchor: next maintenance sweep. |
| **[D-863 hook false-positive] validate-dispatch-advance regex** | OPEN 2026-07-20 | word-final-D + `-YEAR` pattern triggers bogus D-NNNN token. |
| **[D-945] ADR-035 §Decision 5 fuel budget advisory** | OPEN 2026-07-30 | May need revision after S-21.07 benchmarks. |
| **[D-945] create-adr skill defect** | OPEN 2026-07-30 — root fix PENDING | ADR-035 row omitted; manually remediated. |
| **[D-945] VP-102..VP-118 pending allocation** | DEFERRED 2026-07-30 — anchored S-21.07 post-merge | 17 VPs per BC-5.39.010 §VP Anchors. |
| **[D-952] compute-input-hash operator cache binary divergence** | BOOTSTRAP-MIGRATED 2026-08-03 — anchored rc.24 #715 | Self-heals at rc.24. |
| **[D-953] 27 unparseable frontmatter files — systemic corpus defect** | OPEN 2026-08-04 | 27 of 3,572 `.factory` files fail strict YAML parser. Needs remediation story. |
| **[D-953] ADR-037 19-story volatile-inputs remediation obligation** | OPEN 2026-08-04 | 19 stories must have volatile cycle-artifact entries removed. S-19.01 CRITICAL. |
| **[D-954] F-S2107-P3-001 `arm_a1` None-conflation** | **CLOSED — implementation-verified at fbb5183c; adversary pass-7 COMPLETE (D-960)** | Adversarially re-confirmed at pass-7. |
| **[D-954] F-S2107-P3-002 `is_volatile_path` 3-way drift vs ADR-037** | **CLOSED — implementation-verified at fbb5183c; adversary pass-7 COMPLETE (D-960)** | Adversarially re-confirmed at pass-7. |
| **[D-954] Duplicate T-038 test ID** | OPEN 2026-08-04 | Two distinct test cases share ID T-038; POLICY 1 violated. Anchor: pass-9 adversary or next fix burst. |
| **[D-954] decision-log.md >14,800 lines — compaction overdue** | OPEN 2026-08-04 | Soft cap 2,500 / hard cap 3,500 lines. WASM validators time out on every edit. |
| **[D-954] S-21.09 authored this burst** | OPEN 2026-08-04 — v1.0 input-hash cf3a0c6 | S-21.09 `wasm-artifact-restore-and-registry-parity` authored; not yet started. |
| **[D-955] 8 Dependabot vulnerabilities — GHSA series** | OPEN 2026-08-04 | Security alerts not previously recorded. Anchor: next maintenance sweep or human triage. |
| **[D-956] F-S2107-P6-001/002/003/004** | SUPERSEDED-BY-PASS-6 (D-957) | Pass-6 adversary re-assessed; see F-S2107-P7-NNN / F-S2107-P8-NNN for current finding sets. |
| **[D-957] F-S2107-P7-002/003/004** | **CLOSED — implementation-verified at fbb5183c; adversarially re-confirmed pass-7 (D-960)** | Closed per implementation verification; adversary pass-7 found no regression. |
| **[D-957] F-S2107-P7-019 D-693 attestation cites stale WASM size** | OPEN 2026-08-05 | D-693 commit message (`b78b27ef`) names 226,794 bytes; actual deployed .wasm per current build. Anchor: pass-9 fix burst or next SHA-patch. |
| **[D-958] O-P8-01 CWE tag correction** | NON-BLOCKING OBS | CWE-778 → CWE-636/CWE-390 for audit-log omission. Documentary; no code change needed. |
| **[D-958] 60 of 158 stories lack tdd_mode — template-compliance gap** | OPEN 2026-08-06 | validate-template-compliance blocks edits to affected stories. Anchor: S-15.03 PRIORITY-A. |
| **[D-958] GATE DEFECT: validate-pr-review-posted scans prose not GitHub state** | OPEN 2026-08-06 | Paper-gate: hook reads PR description, not GitHub API merge state. |
| **[D-958] GATE DEFECT: validate-changelog-monotonicity header-skip grep misreads data rows** | OPEN 2026-08-06 | Forced artifact degradation when changelog data rows trigger header pattern. |
| **[D-958] GitHub Actions MAJOR OUTAGE** | **RESOLVED 2026-08-07** | CI resumed; PR #770 fully green including build-dispatcher (windows-x64). |
| **[D-961] E-22 epic file deletion PENDING** | OPEN 2026-08-07 — operator execution required | E-22 DISSOLVED 2026-08-07 per human ruling (D-961(c)); agent `git rm` blocked by permission classifier; file `.factory/stories/epics/E-22-dependency-security-hardening.md` remains on disk. D-961/B commit proceeds WITHOUT this deletion. Epic count stays at 20 until operator executes `git rm` separately. |
| **[D-961] SEC-001 sequencing constraint (E-22 scope preserved)** | OPEN 2026-08-07 — SECURITY | wasmtime >= 46.0.2 required before S-21.07 merges; currently at 44.0.3. Full E-21 W4 SEC-001 constraint enforced at S-21.07 merge. |
| **[D-961] RUSTSEC-2026-0222 (E-22 scope preserved)** | OPEN 2026-08-07 — SECURITY | Previously tracked under E-22; re-anchored to E-21 W4 per D-961(c). |
| **[D-961] RUSTSEC-2026-0204 (E-22 scope preserved)** | OPEN 2026-08-07 — SECURITY | Previously tracked under E-22; re-anchored to E-21 W4 per D-961(c). |
| **[D-961] 7 batched Dependabot alerts (E-22 scope preserved)** | OPEN 2026-08-07 | Previously tracked under E-22; re-anchored to E-21 W4 per D-961(c). See also [D-955] entry. |
| **[D-961] EAC-002 (E-22 scope preserved)** | OPEN 2026-08-07 | Enhanced assurance control previously in E-22 scope; re-anchored to E-21 W4 per D-961(c). |
| **[D-961] ADR-033 E-22 namespace ambiguity (E-22 scope preserved)** | OPEN 2026-08-07 | ADR-033 referenced E-22 namespace; E-22 dissolved per D-961(c). Route: architect at next ADR-033 touch. |
| **[D-961] fix/nested-factory-path-derivation LOCAL ONLY** | OPEN 2026-08-07 | 09f052a9 not pushed; F-S2107-P8-016 CLOSED implementation-side. Changes need delivery to feature/S-21.07 or separate PR per team-lead sequencing. |
| **[D-961] fix/d999-sentinel-code-migration LOCAL ONLY** | OPEN 2026-08-07 | bf642fd9 in worktree .worktrees/d999-migration, not pushed. ADR-041 sentinel migration code needs delivery per team-lead sequencing. |

## Historical Content

- `cycles/v1.0-brownfield-backfill/burst-log.md` | `session-checkpoints.md` | `lessons.md` | `decision-log.md`
- `cycles/v1.0-feature-plugin-async-semantics-pass-1/burst-log.md` | `session-checkpoints.md` | `lessons.md`
- `cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md` (adversary reviews at `S-12.03/`, `S-12.04/`, `S-12.05/` subdirs)

## Session Resume Checkpoint (2026-08-07 — D-961-RECORDING-BURST — PIPELINE PAUSED; pass-9 adversary NEXT; streak 0/3; trajectory-tail →25→24→20→16)

> **SELF-SUFFICIENT RESUME CONTEXT — ASSUMES ZERO PRIOR CONTEXT. PIPELINE PAUSED.**

### §1 Position

Cycle `v1.0-brownfield-backfill`. **D-961 recording burst COMPLETE** (single Commit B `e2bfec65` + SHA-patch DONE). factory-artifacts tip **`e2bfec65`** (D-961/B; SHA-patch DONE). 4-INDEX: **BC v4.52 / VP v2.76 / STORY v4.289 / ARCH v3.47**. `policies.yaml` **v1.4.21**. All pass-8 findings resolved (implementation-verified). E-22 dissolved per D-961(c); epic file deletion PENDING operator execution — agent `git rm` blocked; D-961/B commit proceeds WITHOUT E-22 deletion; operator executes `git rm` separately post-commit. ADR-041 sentinel migration COMPLETE. pass-9 adversary dispatch is NEXT.

### §2 Convergence

Streak **0/3**. **7 true adversary passes, ZERO CLEAN verdicts.** Trajectory `47 → 18 → 25 → 25 → 24 → 20 → 16`; tail LENGTH=4 → `→25→24→20→16`. UNCHANGED (no adversary pass this session).

### §3 Pass-8 Findings — ALL RESOLVED

All 16 pass-8 findings resolved. Previously closed: `-001` through `-005`, `-008` through `-012`, `-015`. **Closed this session (D-961):** **`-006`** (bc_index_row_contains_version helper DELETED; direct comparison; 3 bypass-mutant tests retained; feature/S-21.07 @ 67ffbdcc), **`-007`** (fabricated echo attestation removed; runtime accounting delegated to run-all.sh TAP; feature/S-21.07 @ 67ffbdcc), **`-013`** (production-scale 576,396B fixture added; plugin.completed asserted present; plugin.timeout asserted absent; feature/S-21.07 @ 67ffbdcc), **`-016`** (3 unguarded .join(".factory") sites fixed + both-polarity tests; fix/nested-factory-path-derivation @ 09f052a9). **`-014` GRANDFATHERED** (human ruling D-960(c)). **CRITICAL:** `-006`/`-007`/`-013` are implementation-verified only — adversarial re-verification required under the iron law. pass-9 adversary dispatch is NEXT.

### §4 Branch States

- `factory-artifacts` @ `e2bfec65` (D-961/B SHA-patch DONE)
- `feature/S-21.07` @ **`67ffbdcc`** — LOCAL ONLY, 1 ahead of origin `37022ecc` (UNPUSHED per human ruling D-961(e); NO REBASE); all pass-8 findings resolved; NO PR; MERGE-ORDER: S-21.09 MUST land first
- `feature/S-21.04` @ `323f440f` — behind develop by 1, pass-31 pending, no PR
- `fix/nested-factory-path-derivation` @ `09f052a9` — LOCAL, NOT pushed, no PR; F-S2107-P8-016 CLOSED
- `fix/d999-sentinel-code-migration` @ `bf642fd9` — LOCAL in worktree `.worktrees/d999-migration`, NOT pushed, no PR; ADR-041 sentinel 8 sites migrated
- `develop` @ **`700b4dd3`** (remote; local stale at `948f0fb1` — pull on resume)

### §5 PR #770 — MERGED

Squash-merged `700b4dd3` at 2026-08-07T18:44:24Z. **`wasmtime-wasi = 44.0.3` verified live on develop**, clearing `RUSTSEC-2026-0149` and `RUSTSEC-2026-0182`. CI fully green including `build-dispatcher (windows-x64)`. Remote branch auto-deleted.

### §6 CRITICAL — Main Checkout on Dead Branch

The main working tree sits on `fix/wasmtime-wasi-cve-2026-47261` @ `23e307bb`, which is merged and whose remote is deleted. **First action on resume: `git checkout develop && git pull`, then `git branch -d fix/wasmtime-wasi-cve-2026-47261`.** Uncommitted there: `.claude/scheduled_tasks.lock` (M) and `plugins/vsdd-factory/tests/report.tap` (??) — harness artifacts, do not commit.

### §7 Recommended Merge Order

**S-21.12 → S-21.09 → S-21.07**. S-21.12 now anchored E-21 W4 (D-961(c)). fix/nested-factory-path-derivation (09f052a9) and fix/d999-sentinel-code-migration (bf642fd9) are LOCAL ONLY; team-lead decides delivery path.

### §8 Pending Human Decisions

1. **E-22 epic file deletion** — operator must execute `git -C .factory rm stories/epics/E-22-dependency-security-hardening.md` separately after D-961/B commit. NOT a pre-commit gate.
2. **fix/nested-factory-path-derivation + fix/d999-sentinel-code-migration delivery** — both LOCAL ONLY; team-lead decides: squash into S-21.07, separate PRs, or cherry-pick.
3. **E-22 security scope sequencing** — SEC-001, RUSTSEC-2026-0222, RUSTSEC-2026-0204, 7 Dependabot alerts, EAC-002 re-anchored E-21 W4. Confirm sequencing plan.

### §9 New Findings This Session

- **E-22 epic file deletion BLOCKED** — permission classifier blocked agent git rm; D-961/B commit proceeds WITHOUT deletion; operator executes git rm separately post-commit.
- **ARCH-INDEX.md pre-existing pipe-escape defect** — FIXED in-scope: ADR-041 table row had unescaped `|` in GFM table cell; escaped to `\|` and `\|\|`.
- **Large-file PostToolUse fail-closed timeouts** — burst-log, ARCH-INDEX, policies.yaml, STATE.md all triggered WASM fuel exhaustion; writes land on PostToolUse; verified on disk. Feeds S-15.03 PRIORITY-A.

### §10 Cautions

- **E-22 epic file exists on disk** — dissolution ruled, physical deletion PENDING operator execution; NOT a gate on D-961/B commit.
- Do NOT run `compute-input-hash --scan --update` — 418-file blast radius per D-936. Single-file only.
- Use **explicit refspecs** for all pushes (`git push origin HEAD:<branch>`).
- fix/nested-factory-path-derivation @ 09f052a9 and fix/d999-sentinel-code-migration @ bf642fd9 are LOCAL ONLY — not pushed.
- Open PRs: **#769** (green), **#768** (NO_CHECKS/action_required), **#767** (dependabot postcss, green), **#729** (green), **#632** (DRAFT/FAILING, pre-existing).

### §11 Resume Command

`/vsdd-factory:next-step`. D-961/B committed @ `e2bfec65` + SHA-patch committed. factory-artifacts tip: `e2bfec65`. First actions on resume: (1) `git checkout develop && git pull` + delete dead local branch `fix/wasmtime-wasi-cve-2026-47261`; (2) push factory-artifacts: `git push origin HEAD:factory-artifacts`; (3) operator executes E-22 deletion separately: `git -C .factory rm stories/epics/E-22-dependency-security-hardening.md` then commit; (4) dispatch pass-9 adversary (iron law fresh-context; re-verify -006/-007/-013). POLICY 16 ALLOCATOR-CEILING GATE result: PASS (see burst-log.md Dim-2 for captured stdout).
