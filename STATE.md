---
document_type: pipeline-state
level: ops
version: "2.0"
status: draft
producer: state-manager
timestamp: 2026-04-27T02:40:00Z
phase: s-7-03-pass-9-fixes-applied
inputs: []
input-hash: "[live-state]"
traces_to: ""
project: vsdd-factory
mode: brownfield
current_step: "S-7.03 pass-9 fixes applied (F-401 VP-063 + F-402 AC-011 expansion). Convergence clock RESET. Next: pass-10 → 1 of 3 NITPICK target."
current_cycle: v1.0-brownfield-backfill
dtu_required: false
dtu_assessment: 2026-04-25
dtu_clones_built: "n/a"
dtu_services: []
---

<!--
  STATE.md SIZE BUDGET: Keep this file under 200 lines.
  Historical content belongs in cycle files, NOT here.
  Run /vsdd-factory:compact-state if this file grows past 200 lines.
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
| **Last Updated** | 2026-04-27 (pass-9 fixes applied) |
| **Current Phase** | s-7-03-pass-9-fixes-applied |
| **Current Cycle** | v1.0-brownfield-backfill |

## Current Cycle: v1.0-brownfield-backfill

**Mode:** brownfield-onboarding — formal VSDD backfill for v1.0 work that shipped as 1.0.0-beta.4  
**Cycle pointer:** `.factory/cycles/v1.0-brownfield-backfill/INDEX.md`

## Phase Progress

| Phase | Status | Artifact |
|-------|--------|----------|
| Phase 0 — Brownfield Ingest | COMPLETE | initial BC migration in pass-3-* + pass-8-final-synthesis.md |
| Phase 1.1 — Architecture Index + ADRs | COMPLETE | ARCH-INDEX (10 SS-NN) + 13 of 13 ADRs (ADR-001..013) |
| Phase 1.2 — Sharded Architecture | COMPLETE | 10 SS-NN-\<name\>.md files |
| Phase 1.3 — L2 Domain Spec | COMPLETE | 8 sharded files (28 CAPs, 17 DIs, 22 DEs, 18 DECs, 35 entities) |
| Phase 1.4 — BC Migration | COMPLETE | 1,891 BC-S.SS.NNN files in 10 ss-NN/ shards (1,878 at closure; +15 E-7 +13 S-7.03 = 1,891 current) + BC-INDEX.md |
| Phase 1.5 — Formal PRD | COMPLETE | 43 FRs (FR-041, FR-042 added; FR-043 added in S-7.03), 76 NFRs, 100% BC traceability |
| Phase 1.6a — DTU Assessment | COMPLETE | DTU_REQUIRED: false |
| Phase 1.6b — Verification Properties | COMPLETE | 64 VPs (all draft, VP-001..VP-064; +2 for E-7; +2 for S-7.03) |
| Phase 1.7 — Extraction Validation R2 | in-progress | Migration fidelity check |
| Phase 1.8 — Story Migration | COMPLETE | 41 stories S-N.MM, 6 epics E-0..E-5 |
| Phase 1d — Adversarial Spec Review | in-progress | 9 passes done; trajectory 25→12→5→2→1→0→0→1→2; convergence clock RESET (0 of 3) |
| Release v1.0.0-beta.5 | COMPLETE | PR #5 merged 2001b97; tag 0a95c8c; bot bundle f1ec5bf; 5 plugins · 110 skills |
| Phase 2 — Story Decomposition | not-started | Unblocked; 45 stories (41 migrated + 4 new E-6/E-7) ready for dependency graph + wave schedule |
| S-6.01 spec convergence (sub-cycle) | COMPLETE | 8 passes, 19→0 trajectory, CONVERGENCE_REACHED at pass-8 |
| E-7 Process Codification spec foundation | COMPLETE | E-7 epic + S-7.01/S-7.02 (status=ready) + 15 BCs + 2 VPs + FR-042 |
| E-7 spec convergence (sub-cycle) | COMPLETE | 7 passes, 12→5→1→2→2→0→0 trajectory, CONVERGENCE_REACHED at pass-7 |
| E-7 GREEN implementation (S-7.01 + S-7.02) | COMPLETE | feat/codify-lessons commit 5b9e4fb; 16/16 bats tests green; 10 plugin source files |
| S-6.01 GREEN implementation | COMPLETE | feat/create-adr-skill commit 5f0b0fa; 25/25 bats tests green; SKILL.md + commands + driver |
| Release v1.0.0-beta.6 | COMPLETE | Tag at ae426cd; PR #8/#10/#11/#12 merged; bot bundle commit atomic per beta.4 cache fix; GH Release published |
| Hotfix: novelty-test fixture path | COMPLETE | PR #10/#11 merged; release workflow re-fire succeeded after fix |
| S-7.03 spec foundation | COMPLETE | 13 BCs + 2 VPs + FR-043 + story (status=ready) + E-7 epic v1.1 |
| S-7.03 pass-1 fix burst | COMPLETE | 25 findings, all addressed; SS-05 +4 / SS-08 -4 BC reanchor; VP-063 method proptest→integration; CAP-016 expanded SS-08; story v1.1 |
| S-7.03 pass-2 fix burst | COMPLETE | 7 substantive findings + 5 obs addressed; BC-INDEX subsystem grouping fixed; PRD narrative reconciled; VP-INDEX Rust-count audited; input-hashes recomputed (3 distinct values) |
| S-7.03 adversarial pass-3 | COMPLETE | 5 findings (0 CRIT, 0 HIGH, 3 MED, 1 LOW, 1 NIT); verdict FINDINGS_REMAIN; convergence clock not yet started |
| S-7.03 pass-3 fix burst | COMPLETE | F-001 (PO), F-002+F-004+F-005 (state-manager), F-003 (story-writer) |
| S-7.03 adversarial pass-4 | COMPLETE | 2 findings (1 MED F-101 GFM table render regression, 1 LOW F-102 spatial reference); verdict FINDINGS_REMAIN |
| S-7.03 pass-4 fix burst | COMPLETE | F-101+F-102 fixed via Option C (blockquote moved below rows) |
| S-7.03 adversarial pass-5 | COMPLETE | 1 finding (F-201 LOW story path prefix); 5 NITPICK obs; verdict FINDINGS_REMAIN |
| S-7.03 pass-5 fix burst | COMPLETE | F-201 fixed; story v1.3 → v1.4 |
| S-7.03 adversarial pass-6 | COMPLETE | 0 substantive findings; 6 NITPICK obs; verdict NITPICK-only (1 of 3 convergence target) |
| S-7.03 adversarial pass-7 | COMPLETE | 0 substantive findings; 8 NITPICK obs (6 carried + 2 novel); verdict NITPICK-only (2 of 3) |
| S-7.03 adversarial pass-8 | COMPLETE | 1 LOW (F-301 Architecture Compliance Rules off-by-one task refs); 8 NITPICK obs; verdict FINDINGS_REMAIN; convergence clock RESETS |
| S-7.03 pass-8 fix burst | COMPLETE | F-301 + Batch A/B sibling sweep; story v1.4→v1.5 |
| S-7.03 adversarial pass-9 | COMPLETE | 2 LOW (F-401 VP-063 task ref sibling miss, F-402 AC-011 enumeration coherence); 8 NITPICK obs; convergence clock RESETS again |
| S-7.03 pass-9 fix burst | COMPLETE | F-401 + F-402 fixed; story v1.5→v1.6; VP-063 timestamp bump |

## Current Phase Steps

| Step | Agent | Status | Output |
|------|-------|--------|--------|
| S-7.03 adversarial pass-7 | adversarial-reviewer | COMPLETE | 0 substantive findings; 8 NITPICK obs (6 carried + 2 novel); NITPICK-only (convergence 2 of 3) |
| S-7.03 adversarial pass-8 | adversarial-reviewer | COMPLETE | 1 LOW (F-301 task-ref off-by-one); 8 NITPICK obs; FINDINGS_REMAIN; convergence clock RESETS |
| S-7.03 pass-8 fix burst | state-manager + story-writer | COMPLETE | F-301 fixed; Batch A/B sibling defect swept; story v1.4→v1.5 |
| S-7.03 adversarial pass-9 | adversarial-reviewer | COMPLETE | 2 LOW (F-401 VP-063 sibling miss, F-402 AC-011 enumeration); 8 NITPICK obs; FINDINGS_REMAIN; convergence clock RESETS |
| S-7.03 pass-9 fix burst | state-manager + story-writer | COMPLETE | F-401 VP-063 task-ref + F-402 AC-011 18-test expansion fixed; story v1.5→v1.6 |

## Identifier Conventions

| Type | Format | Authoritative Source | Count |
|------|--------|----------------------|-------|
| Subsystem | SS-NN | `specs/architecture/ARCH-INDEX.md` | 10 |
| Behavioral Contract | BC-S.SS.NNN (one-per-file) | `specs/behavioral-contracts/ss-NN/` | 1,891 |
| Verification Property | VP-NNN | `specs/verification-properties/VP-INDEX.md` | 64 |
| Capability | CAP-NNN | `specs/domain-spec/capabilities.md` | 28 |
| Domain Invariant | DI-NNN | `specs/domain-spec/invariants.md` | 17 |
| Domain Event | DE-NNN | `specs/domain-spec/domain-events.md` | 22 |
| Story | S-N.MM | `stories/S-N.MM-<short>.md` | 45 |
| Epic | E-N | `stories/epics/E-N-<short>.md` | 8 |
| ADR | ADR-NNN | `specs/architecture/decisions/ADR-NNN.md` | 13 |

## Subsystem Distribution

| SS-ID | Name | BC Prefix | BCs |
|-------|------|-----------|-----|
| SS-01 | Hook Dispatcher Core | BC-1 | 99 |
| SS-02 | Hook SDK and Plugin ABI | BC-2 | 22 |
| SS-03 | Observability Sinks | BC-3 | 49 |
| SS-04 | Plugin Ecosystem | BC-4 | 13 |
| SS-05 | Pipeline Orchestration | BC-5 | 646 |
| SS-06 | Skill Catalog | BC-6 | 585 |
| SS-07 | Hook Bash Layer | BC-7 | 196 |
| SS-08 | Templates and Rules | BC-8 | 218 |
| SS-09 | Configuration and Activation | BC-9 | 5 |
| SS-10 | CLI Tools and Bin | BC-10 | 58 |
| **Total** | | | **1,891** |

## Story Status (45 total)

- **Merged (22):** All Tier A (5), Tier B.0 (1), Tier B.x (8), most Tier C (6 of 7), Tier D (1)
- **Partial (4):** S-2.05 (cargo publish dry-run), S-3.04 (host fn done, bash not retired), S-4.06 (RoutingFilter parsed not wired), S-5.05 (skeleton)
- **Draft / Not Shipped (15):** All Tier E except partials, all Tier F/G/H
- **Ready (4):** S-6.01 (create-adr skill), S-7.01 (agent prompt discipline), S-7.02 (defensive sweep + hook + meta-rule), S-7.03 (TDD discipline hardening)

## Drift Items (open)

| ID | Description | Severity | Disposition |
|----|-------------|----------|-------------|
| DRIFT-001 | read_file host fn stub returns CAPABILITY_DENIED unconditionally | MEDIUM | L-P0-001 fix in beta.5 |
| DRIFT-002 | sink.* internal events declared but never emitted | MEDIUM | tied to S-4.04 retry/breaker |
| DRIFT-003 | Per-sink dedicated threads despite S-1.06 shared-runtime intent | MEDIUM | re-design at rc.1 |
| DRIFT-004 | hooks.json + hooks-registry.toml dual routing tables | MEDIUM-HIGH | L-P0-002 cutover before rc.1 |
| DRIFT-005 | HTTP/Datadog/Honeycomb sinks declared but not implemented | MEDIUM | Tier E (S-4.01..S-4.03) |
| DRIFT-006 | Phase 5 events not wired (SessionStart/End) | MEDIUM | Tier G (S-5.01, S-5.02) |
| DRIFT-007 | DISPATCHER_SHUTTING_DOWN constant declared, never emitted | LOW | Tier G fixup |
| DRIFT-008 | plugin.loaded/load_failed events not wired | LOW | dispatcher cleanup |
| DRIFT-009 | verify-sha-currency.sh is template, not registered hook | RESOLVED | CONV-ABS-1 closed |
| DRIFT-010 | 26 unported bash hooks block Windows native | MEDIUM | Tier E (S-3.01..S-3.04) |

## Active Branches

| Branch / Tag | SHA | Notes |
|--------------|-----|-------|
| main | ae426cd | bot bundle for v1.0.0-beta.6 (PR #11 hotfix + PR #12 back-merge) |
| develop | ae426cd | post-PR-#12 back-merge; plugin.json=1.0.0-beta.6 |
| factory-artifacts | c50bb0f | 10 ADRs commit |
| v1.0.0-beta.5 (tag) | 0a95c8c | SHIPPED 2026-04-26; GitHub Release published |
| v1.0.0-beta.6 (tag) | ae426cd | SHIPPED 2026-04-26; GH Release published; prerelease=true |

## Decisions Log

| ID | Decision | Rationale | Phase | Date | Made By |
|----|----------|-----------|-------|------|---------|
| D-001 | 10-subsystem layout (SS-01..SS-10) | Natural split: Rust compiled (SS-01..04) vs VSDD framework (SS-05..10) | 1.1 | 2026-04-25 | architect |
| D-002 | BC-S.SS.NNN one-per-file sharding | Enables granular traceability and diff-friendly git history | 1.4 | 2026-04-25 | architect |
| D-003 | DTU not required | All external services are HTTP APIs with stable public contracts; no clone needed | 1.6a | 2026-04-25 | architect |
| D-004 | v1.0.0-beta.5 release scope | ADR template + identifier canonicalization phase 1 shipped; phase 2 (test fixtures, workflows, agents) deferred to beta.6 | release | 2026-04-26 | orchestrator |
| D-005 | Add create-adr skill to v1.0.x roadmap | ADR is the only major artifact without a dedicated authoring skill (compare create-prd, create-story, create-architecture, create-domain-spec); 10-ADR backfill exposed pain points (manual ID allocation, ARCH-INDEX drift, no supersession patcher) | post-1.1 | 2026-04-26 | orchestrator + user |
| D-006 | Spec-first authoring discipline restored after S-6.01 gap caught | Story scaffolded without BCs initially; user caught the gap; full upstream artifacts (BCs/VPs/FR/epic) backfilled before TDD continued | 1.5 | 2026-04-26 | orchestrator + user |
| D-007 | Hook validate-novelty-assessment.sh tightened to anchor on cycles/<key>/adversarial-reviews/ directory; ADR-* explicitly skipped | False-positive on ADR-013 (filename contains 'adversarial-review'); fix lands in plugin source for next release | post-adv-pass-1 | 2026-04-26 | orchestrator |
| D-008 | Codify spec-first-then-TDD discipline + defensive-sweep pattern as plugin source rules | User caught "no BCs/no E-6 epic" gap; F-027 (incomplete defensive sweep) caused 2 wasted passes; lessons should land in agent prompts and consistency-validator | post-1.5 | 2026-04-26 | orchestrator + user |
| D-009 | E-7 Process Codification — codify lessons learned from S-6.01 sub-cycle as plugin source rules | Self-referential dogfooding — vsdd-factory uses its own VSDD process to improve itself; lessons table from D-008 driven into prompt/rule/hook deliverables | post-1.5 | 2026-04-26 | orchestrator + user |
| D-010 | E-7 process codification + S-6.01 create-adr skill → bundle into beta.6 release | Both branches ready (specs converged, GREEN tests pass). Bundling reduces release overhead; both deliver self-improvement value (E-7 codifies lessons; S-6.01 closes per-artifact create-* skill gap) | pre-release | 2026-04-26 | orchestrator + user |
| D-011 | Beta.4 cache-staleness fix prevented broken release; hotfix flow validated | Pre-release validation caught E-7 hook tightening test regression. Bot bundle commit was correctly NOT created (no stale-version-with-X-1-binaries cache poisoning). Hotfix-on-main + delete/recreate-tag flow restored release. End-to-end discipline validated. | release-cycle | 2026-04-26 | orchestrator + user |
| D-012 | S-7.03 (TDD Discipline Hardening) added to E-7 in response to Prism Wave 2 stub-as-impl anti-pattern (3 of 5 stub-architects pre-implemented business logic). Self-referential dogfooding pattern continues. | E-7 process codification must prevent stub-as-implementation; 13 BCs across 3 subsystems (SS-05 anti-precedent guard, SS-08 RED_RATIO gate + tdd_mode frontmatter, SS-06 mutation wave-gate) + 2 VPs (VP-063 proptest, VP-064 manual). | spec-foundation | 2026-04-26 | orchestrator + user |
| D-013 | S-7.03 spec foundation pass-1 — 4 BCs reanchored SS-08→SS-05 in frontmatter (files stay in ss-08/ per POLICY 1 append-only); VP-063 method changed proptest→integration (production code is shell, not Rust) | BCs BC-8.29.001/002/003 and BC-8.30.002 describe orchestrator pipeline behavior (wave-gate dispatch, RED_RATIO gate), correctly anchored to SS-05. VP-063 tests validate-red-ratio.sh directly via BATS; proptest is infeasible against Bash. | pass-1-fix-burst | 2026-04-26 | state-manager |
| D-014 | S-7.03 pass-2 — BC-INDEX section grouping moved 4 BCs from SS-08 to SS-05 listing (files stay in ss-08/ per POLICY 1); PRD count narrative reconciled to 1,891 = 1,863 + 15 (E-7) + 13 (S-7.03); input-hashes computed (placeholders detected and replaced) | N-001: BC-INDEX section contradicted frontmatter subsystem. N-004: PRD narrative cited 1,878 pre-E-7 baseline (incorrect; correct is 1,863). N-006: VP-INDEX Rust-count was 47; with VP-063 moved from proptest→integration/bats, correct count is 46. | pass-2-fix-burst | 2026-04-26 | state-manager |
| D-015 | S-7.03 pass-3 — F-001 PRD subsystem labels propagated; F-002 BC-INDEX annotations moved to blockquote (5-column table integrity restored); F-003 E-7 '5 subsystems' typo fixed; F-004 STORY-INDEX status canonicalized; F-005 STATE.md Phase 1.4 milestone annotated | pass-3 review returned 5 findings; all routed by severity; Option B (blockquote) chosen for F-002 as lower-blast-radius than promoting table to 6-column. | pass-3-fix-burst | 2026-04-26 | state-manager |
| D-016 | Pass-1 and pass-2 adversarial review files for s7.03 not persisted (audit trail gap detected at pass-3); only pass-3 retroactively persisted from chat content. Reason: adversary agents reported writing but writes did not commit. Investigate adversary tooling next cycle. | Deferred: pass-1 and pass-2 content is not recoverable from disk; gap noted for tooling investigation. | audit-trail | 2026-04-26 | state-manager |
| D-017 | S-7.03 pass-4 — F-002 Option B (blockquote BEFORE rows) caused GFM table-rendering regression; corrected via Option C (blockquote AFTER rows). Lesson: table annotations should default to SS-08 line 1908 footer-comment pattern (HTML comment after rows), not blockquote before rows. Process-gap O-101 — codify in BC-INDEX template. | In GFM/CommonMark, a blockquote terminates a preceding table block; rows below it become a headerless fragment that renders broken. HTML comments do not terminate tables. Option C (move blockquote after rows) is markdown-native and makes "listed above" phrasing accurate. | pass-4-fix-burst | 2026-04-27 | state-manager |
| D-018 | S-7.03 pass-5 — F-201 (story References section BC path prefix `plugins/vsdd-factory/.factory/specs/...`) fixed; trajectory 25→12→5→2→1; convergence clock not yet started (pass-5 not NITPICK-only, 1 LOW finding remains). Story bumped v1.3→v1.4. | Path prefix was `plugins/vsdd-factory/.factory/specs/behavioral-contracts/...` — directory does not exist; correct prefix is `.factory/specs/behavioral-contracts/...`. Frontmatter `inputs:` was already correct; defect was in human-readable References section only. | pass-5-fix-burst | 2026-04-27 | state-manager |
| D-019 | S-7.03 pass-6 NITPICK-only achieved (0 substantive findings, 6 NITPICK obs); trajectory 25→12→5→2→1→0; convergence step 1 of 3 reached. | Pass-6 is first of 3 consecutive NITPICK-only passes required by ADR-013. Pass-7 and pass-8 must each also be NITPICK-only. No spec/story content changes needed. | adv-pass-6 | 2026-04-27 | state-manager |
| D-020 | S-7.03 pass-7 NITPICK-only achieved (0 substantive, 8 NITPICK obs); convergence step 2 of 3 reached; trajectory continues monotonic decay 25→12→5→2→1→0→0. | Pass-7 is second of 3 consecutive NITPICK-only passes required by ADR-013. Pass-8 must also be NITPICK-only for CONVERGENCE_REACHED. No spec/story content changes needed. | adv-pass-7 | 2026-04-27 | state-manager |
| D-021 | S-7.03 pass-8 — fresh-eyes Dimension 2 (dogfooding readiness) caught F-301 task-ref off-by-one; partial-fix-regression sweep also caught Batch A/B task-range drift. Both fixed. Convergence clock RESETS to 0 of 3. Total passes will reach 11 (vs 8 for S-6.01). | Intra-document Architecture Compliance Rules ↔ Tasks cross-reference axis was unprobed by passes 3-7. Pass-8 Dimension 2 lens exposed it. Sibling sweep caught Batch A/B stranding Task 13 in wrong batch. Both fixed atomically in v1.5 per BC-5.36.005-006 partial-fix discipline. | adv-pass-8 | 2026-04-27 | state-manager |
| D-022 | S-7.03 pass-9 — fresh-context sibling sweep caught F-401 (VP-063 task-ref missed in pass-8 burst) and dogfooding-readiness lens caught F-402 (AC-011 enumeration undercount propagated to DoD/Task 19). Both are novel sub-axes prior passes did not probe. Convergence clock RESETS to 0 of 3. Total passes projected: 12 (S-6.01 was 8). | Inter-document sibling sweep stopped at story-file boundary in pass-8 fix burst; VP-063 was not swept for task-number references. Intra-document AC-vs-AC bats test count coherence was unprobed across all 9 passes. Both defects are real implementer-trap findings. | adv-pass-9 | 2026-04-27 | state-manager |

## Skip Log

| Step | Skipped? | Justification |
|------|----------|---------------|
| UX Spec | yes | CLI-only product with no UI surfaces |
| Gene Transfusion Assessment | yes | Not applicable — engine and product are same repo |

## Blocking Issues

<!-- No open blockers. -->
## Session Resume Checkpoint

| Field | Value |
|-------|-------|
| **Date** | 2026-04-27 |
| **Position** | S-7.03 pass-9 fixes applied (F-401 VP-063 + F-402 AC-011 18-test expansion). Convergence clock RESET to 0 of 3. Story v1.6. |
| **Release** | Tag ae426cd; GH Release published 2026-04-26; prerelease=true |
| **Deferred work** | TD-001 wave-scale BC re-anchoring; TD-010 DTU/CI verification; S-7.03+ tooling stories; Phase 2 wave schedule |
| **Next action** | S-7.03 adversarial pass-10 (1 of 3 NITPICK-only target; projected total 12 passes). |

## Historical Content
Historical detail (burst-log, convergence-trajectory, session-checkpoints, lessons, resolved-blockers, release ladder) lives in `cycles/v1.0-brownfield-backfill/`.
