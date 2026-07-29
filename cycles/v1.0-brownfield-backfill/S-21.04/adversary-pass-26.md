---
document_type: adversarial-review
level: adversary
version: "1.0"
status: complete
producer: vsdd-factory:adversary
timestamp: 2026-07-28T23:59:00Z
phase: 3
inputs:
  - stories/S-21.04-story-worktree-write-path-discipline.md
  - specs/architecture/decisions/ADR-031-e21-factory-state-data-loss-hardening.md
  - specs/behavioral-contracts/ss-06/BC-6.26.001.md
  - cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md
input-hash: "1fa6cc2"
traces_to: "BC-6.26.001 v1.14; story v1.28"
pass: 26
verdict: NOT-CLEAN
reviewed_head: "4dc27251"
fixes_landed_head: "7c3338e7"
novelty: high
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-25.md"
findings_count: 11
severity_breakdown: "B2/H4/M3/L2"
streak: "0/3"
trajectory_append: 11
model_override: false
model_resolved: "claude-opus-5"
adr033_deviation: "ADR-033 cross-family limitation — cross-family claim (GPT-5) NOT satisfied; ran on Claude, same family as authoring agents; fresh context + information asymmetry intact, cross-family independence absent"
asymmetry_enforcement: "pass-25 Part A inlined; prior pass files and cycle INDEX.md off-limits; adversary confirmed it opened none"
policy22_note: "Two-direction record: orchestrator concern 1 (retained lexical exclusions) CONFIRMED as B01; orchestrator concern 2 (removing ^ over-matches, fail-closed direction) REFUTED with reasoning — evidence adversary is not ratifying orchestrator hypotheses"
---

# Adversary Pass 26 — S-21.04 story-worktree write-path discipline

**Date:** 2026-07-28
**Reviewed HEAD:** `4dc27251`
**Fixes-landed HEAD:** `7c3338e7`
**Verdict:** NOT-CLEAN — B2/H4/M3/L2 = 11 findings
**Streak:** 0/3 (BC-5.39.001)

## Finding ID Convention

Finding IDs for this cascade use the format: `F-S2104-P<PASS>-<SEV><SEQ>` (project-local convention established at pass-1). Map to template ADV severity abbreviations: B→BLOCKER, H→HIGH, M→MEDIUM, L→LOW. Full format example: `F-S2104-P26-B01`.

## Mandatory Provenance Disclosure

**(1) Model override:** NO model override applied. Frontmatter `model: opus` → `claude-opus-5`. Fresh context enforced.

**(2) ADR-033 cross-family limitation:** ADR-033 §Decision 2 cross-family independence (GPT-5) NOT satisfied. The reviewing model is in the same family as the authoring agents. Fresh context and information asymmetry are intact; cross-family cognitive diversity is absent. Disclosed per ADR-033 §Decision 3.

**(3) Information asymmetry enforcement:** Pass-25 Part A findings were inlined as permitted context. Prior pass files (pass-1 through pass-24) and the cycle `INDEX.md` were off-limits. The adversary confirms it opened none.

**(4) POLICY 22 — both directions recorded:**
- Orchestrator concern 1 (retained lexical exclusions): **CONFIRMED** → B01
- Orchestrator concern 2 (removing `^` over-matches, fail-closed direction): **REFUTED with reasoning** — removing `^` expands the match to non-line-start positions; it is a fail-closed widening. The refutation confirms the adversary is not merely ratifying orchestrator hypotheses.

## Part A — Fix Verification (Pass-25 Closures)

| ID | Pass-25 Severity | Status | Notes |
|----|-----------------|--------|-------|
| P25-B01 | BLOCKER | GENUINELY-CLOSED-with-new-defect | Closed but retained lexical exclusion re-opened anchor class → B01/H01/M02 |
| P25-B02 | BLOCKER | GENUINELY-CLOSED-but-not-sibling-swept | Closed for named guards only; sibling guards (d)(f)(l) not swept → B02 |
| P25-B03 | BLOCKER | GENUINELY-CLOSED | `^` removed; fail-closed direction confirmed correct (refutation of orchestrator concern) |
| P25-H01 | HIGH | GENUINELY-CLOSED | POLICY 15 evidence persisted; Summary HEAD advanced |
| P25-H02 | HIGH | PARTIAL/RECURRENT | 1 of 3 mechanisms closed; 2 re-opened → H03 |
| P25-H03 | HIGH | GENUINELY-CLOSED | F-S2104-P25-001 IDs re-anchored; bats + story corrected |
| P25-H04 | HIGH | GENUINELY-CLOSED | factory path refs canonicalized; routing to state-manager corrected |
| P25-M01 | MEDIUM | GENUINELY-CLOSED | ordering gate bound to `g1_mandated_lineno` |
| P25-M02 | MEDIUM | GENUINELY-CLOSED | folded YAML check added |
| P25-M03 | MEDIUM | GENUINELY-CLOSED | ADR-031 lead-in five→six (architect v1.14) |
| P25-M04 | MEDIUM | GENUINELY-CLOSED | 6 SHA transposition sites corrected |
| P25-M05 | MEDIUM | GENUINELY-CLOSED | Ordinal cites eliminated; ID-anchored form |
| P25-M06 | MEDIUM | GENUINELY-CLOSED | Volatile pins replaced with behavioral anchors |
| P25-M07 | MEDIUM | GENUINELY-CLOSED | Sentinels D-936/D-937 replaced; grep proof zero |
| P25-M08 | MEDIUM | GENUINELY-CLOSED | T-003 DOC-PARITY before-mv assertions added |
| P25-L01 | LOW | GENUINELY-CLOSED | BC-6.26.001 row 1.0 moved to bottom |
| P25-L02 | LOW | PARTIAL/RECURRENT | ADR-031 §Decision 4 anchor added but re-introduced unbalanced paren → L02 |
| P25-deferred | — | DEFERRED (unchanged) | BC-5.44.001/S-21.02 ADR version pin → S-21.WG2-001 |

**Summary:** 13 GENUINELY-CLOSED · 1 GENUINELY-CLOSED-with-new-defect · 1 GENUINELY-CLOSED-but-not-sibling-swept · 1 PARTIAL/RECURRENT (P25-H02→H03) · 1 PARTIAL/RECURRENT (P25-L02→L02) · 1 DEFERRED.

## Part B — New Findings (or all findings for pass 1)

### BLOCKER

#### F-S2104-P26-B01: Mandate Anchor Target Never Gated in 26 Passes

- **Severity:** BLOCKER
- **Category:** coverage-gap
- **Location:** `plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` T-001 gate chain; write-directive gate; `_shared-context.md` §Spec-Path Discipline
- **Description:** AC-001(b) requires anchoring to the MAIN worktree; no gate asserted this. Orchestrator-verified: `grep -n "main-checkout"` over the 2971-line bats file returned ONE hit — a fixture comment. Mutating `anchored to the main-checkout root` → `anchored to the story worktree's own .factory/ subtree` passed all 21 gates + PW-B + probe Leg D. The write-directive gate — which WOULD have caught it — was suppressed by the retained `grep -Ev 'MUST use canonical absolute'` clause exclusion applied unconditionally BEFORE the harm predicate. The mutant instructs every delivery agent to write `.factory/**` into the story-worktree shadow subtree: the exact issue-#523 data-loss configuration BC-6.26.001 PC1 forbids, at 25/25 GREEN. Twenty-six passes hardened gate plumbing while the semantic core of the mandate sat unasserted.
- **Evidence:** Orchestrator-verified mutation result: `not ok 1 T-001 S-21.04 AC-003: stray-file-blocks — PREFLIGHT BLOCKED non-zero; git worktree remove NOT called` + `not ok 2 BC-6.26.001 pipeline probe: real-fixture mutations exercise production domain-construction path`
- **Proposed Fix:** New Gates 1(e)/1(f) asserting the canonical anchor token; write-directive escapes reordered after prohibited-target filter; lexical exclusions must follow harm predicate.
- **References:** BC-6.26.001 PC1; POLICY 13; TD-VSDD-059

#### F-S2104-P26-B02: Guard Family Sweep Applied Only to Named Guards, Not Predicate Family

- **Severity:** BLOCKER
- **Category:** coverage-gap
- **Location:** `plugins/vsdd-factory/tests/worktree-identity-preflight.bats` guards (d)(f)(l) + guard (e) sub-gates; corpus test T-015
- **Description:** The pass-25 fail-closed fix was applied to exactly the two guards the finding named (`_guard_e_checks_out_nothing` and `_guard_g_path_corroborated`) and no others sharing the predicate. Guard (f) was live fail-open: `case-insensitive` occurs on 3 lines of `adversary.md`; only one being Rule 5. In-place nullification left two incidental occurrences affirmative → GREEN. Guards (d) and (l) were safe only by accident. The corpus test T-015 covered only the two named guards; the full `any-affirmative` family was not enrolled.
- **Evidence:** Orchestrator-verified guard-(f) nullification: `not ok 1 test_BC_adversary_id_bearing_globs_must_be_case_insensitive` + `not ok 1 test_BC_B01_corpus_regression_guards_e_co_and_g_pc`
- **Proposed Fix:** Factor guards into named helpers; zero-nullify all members of the any-affirmative family; extend corpus to 7/7 guards with M5–M9 vectors.
- **References:** TD-VSDD-060; POLICY 13

### HIGH

#### F-S2104-P26-H01: Story AC-001 Gate Cell Desynced from HEAD (Eighth Same-Burst Coupling)

- **Severity:** HIGH
- **Category:** spec-fidelity
- **Location:** `stories/S-21.04-story-worktree-write-path-discipline.md` §Acceptance Criteria AC-001 Gate cell
- **Description:** Desynced at three sites from HEAD `ac478e41`: (1) `^` still shown in `PWBD_DIRECTIVE_CLASS` twice (HEAD removed it after B03 fix); (2) escape clause (i) "prohibition token" did not exist in the code — only the `**Forbidden:**` bold-label form. The eighth failed same-burst coupling in the cascade.
- **Evidence:** HEAD `ac478e41` diff shows `^` absent from PWBD_DIRECTIVE_CLASS; story gate cell retains `^` at two sites. Escape clause (i) references "prohibition token" which has no corresponding code symbol.
- **Proposed Fix:** Resync AC-001 Gate cell to HEAD ac478e41; correct `^` removal at both PWBD_DIRECTIVE_CLASS sites; replace phantom prohibition-token escape with the real `**Forbidden:**` label form.
- **References:** POLICY 8; TD-VSDD-060

#### F-S2104-P26-H02: ADR-031 Retains Three Retracted BC-6.26.001 Mechanism Claims

- **Severity:** HIGH
- **Category:** contradictions
- **Location:** `specs/architecture/decisions/ADR-031-e21-factory-state-data-loss-hardening.md` §Decision 1 (INV-E21-004 step-2, step-3) and §Decision 4 (PC2 step-2)
- **Description:** Retained three mechanism claims BC-6.26.001 retracted at v1.12/v1.13 as empirically false: (1) `find` returns empty for symlinks; (2) `rm -rf` destroys the target; (3) trailing-slash as "defense-in-depth." ADR-031 self-declaredly mirrors BC-6.26.001 PC2 as normative source while contradicting it. Two prior sweeps for this class stayed inside the BC file and never entered the mandated sibling surface.
- **Evidence:** BC-6.26.001 v1.12 §Description step 2 + §Postconditions non-directory/symlink-path paragraph states the verified mechanism (trailing-slash dereferences symlink-to-dir via POSIX pathname resolution; rm -rf removes only symlink entry). ADR-031 §Decision 1 still contains the retracted claims.
- **Proposed Fix:** Correct all three sites in ADR-031 to the verified mechanism per BC-6.26.001 v1.12/v1.13.
- **References:** POLICY 4; TD-VSDD-060

#### F-S2104-P26-H03: Pipeline Probe Leg E Closed 1 of 3 Mechanisms and Re-Opened a Second

- **Severity:** HIGH
- **Category:** coverage-gap
- **Location:** `plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` T-010 pipeline probe Leg E; `_build_nosplit()` helper; `_prose_vars_check` exclusion logic
- **Description:** The new `grep -v '_nosplit'` exclusion excluded exactly the variables the finding cited — and every gate consumes the `*_nosplit` form. A revert building `write_discipline_prose_nosplit` from a bare `tr` was invisible. Mechanism 3 (trailing-comment defeat of `grep -v '_build_'`) was untouched.
- **Evidence:** F-S2104-P25-H02 named 3 mechanisms; pass-25 fix addressed mechanism 1 and introduced a new exclusion for mechanism 2's variables, making mechanism 2 evasion viable again. Mechanism 3 unchanged.
- **Proposed Fix:** Wire `_build_nosplit()` through a `_build_*` function; remove `grep -v '_nosplit'` exclusion; close mechanism 3 via comment-stripping.
- **References:** POLICY 13; F-S2104-P25-H02

#### F-S2104-P26-H04: red-gate-log §Pass-25 Missing Per-Guard Mutant Verification for 5 Closures

- **Severity:** HIGH
- **Category:** coverage-gap
- **Location:** `.factory/cycles/v1.0-brownfield-backfill/S-21.04/implementation/red-gate-log.md` §Pass-25 assertion-site attestation
- **Description:** POLICY 15 / D-889: five of the prior burst's own seven closures (`P25-B02`, `P25-H02`, `P25-M01`, `P25-M02`, `P25-M08`) carried zero per-guard mutant verification — the same evidence gap `P25-H01` named, reproduced one pass later on the new closures.
- **Evidence:** red-gate-log.md §Pass-25 section contains B01 orchestrator evidence but no per-guard mutant records for B02, H02, M01, M02, M08.
- **Proposed Fix:** Per POLICY 15 / D-889, each guard added or modified in a burst must have its own mutant injection command + `not ok` stdout in the pass attestation section. **OPEN — records requested twice and not produced; record as OPEN.** Pass-27 priority item #1.
- **References:** POLICY 15; D-889; POLICY 22 (self-prediction confirmed)

### MEDIUM

#### F-S2104-P26-M01: T-015 Corpus Test Unregistered in Four Story Inventories

- **Severity:** MEDIUM
- **Category:** spec-fidelity
- **Location:** `stories/S-21.04-story-worktree-write-path-discipline.md` §Test Plan, §Architecture Mapping, §File Structure Requirements, §Tasks
- **Description:** The new executable corpus test T-015 (`test_BC_B01_corpus_regression_guards_e_co_and_g_pc`) was added to the bats suite but not registered in any of the four story inventories, contrary to the precedent set when T-010 (pipeline probe) was added.
- **Evidence:** T-010 appears in all four inventories at story v1.27. T-015 absent from all four at story v1.28.
- **Proposed Fix:** Register T-015 in §Test Plan, §Architecture Mapping, §File Structure Requirements, §Tasks with 7/7 guards and M5–M9 vectors.
- **References:** POLICY 4

#### F-S2104-P26-M02: Write-Directive Gate Scope Wider Than Authoring-Constraint Scope

- **Severity:** MEDIUM
- **Category:** contradictions
- **Location:** `plugins/vsdd-factory/tests/story-worktree-write-path-discipline.bats` write-directive gate; `_shared-context.md` §Spec-Path Discipline `#### Write Discipline` blockquote
- **Description:** Gate fires on correct RFC-2119 prohibition prose anywhere in `### Spec-Path Discipline`, while the authoring-constraint blockquote scoped the requirement only to the `#### Write Discipline` subsection — constraint under-scoped relative to the gate imposing it. Fifth recurrence of gate-fires-on-correct-prose.
- **Evidence:** Gate extraction bound to `### Spec-Path Discipline`; authoring-constraint blockquote says "only the `#### Write Discipline` subsection" — mismatch in scope direction.
- **Proposed Fix:** Correct the authoring-constraint blockquote scope from `#### Write Discipline` to `### Spec-Path Discipline` (gate scope is correct; constraint description was under-scoped).
- **References:** POLICY 13

#### F-S2104-P26-M03: Preflight Guard (k) Carries Explicit Fail-Open Fallback

- **Severity:** MEDIUM
- **Category:** contradictions
- **Location:** `plugins/vsdd-factory/tests/worktree-identity-preflight.bats` guard (k)
- **Description:** Guard (k) carried an explicit fail-open fallback: empty section extraction degraded to a whole-file grep its own comment described as defeatable. POLICY 13 requires ambiguity → BLOCK. The fallback path creates a soft bypass channel.
- **Evidence:** Guard (k) code contains else-branch that falls through to whole-file grep when section extraction returns empty.
- **Proposed Fix:** Remove guard (k) fail-open fallback; empty extraction must BLOCK per POLICY 13.
- **References:** POLICY 13

### LOW

#### F-S2104-P26-L01: "5 Legs" Description Suspected Stale — Adjudicated NON-DEFECT

- **Severity:** LOW
- **Category:** spec-fidelity
- **Location:** `stories/S-21.04-story-worktree-write-path-discipline.md` §Test Plan, §Architecture Mapping, §File Structure Requirements — T-010 "5 legs" description
- **Description:** "5 legs" suspected stale after mixed-marker sub-probes were added to Leg B.
- **Evidence:** Sub-probes are a `for` loop inside the Leg B region; the count of named legs (A through E) is still 5. Leg B descriptions were extended for clarity but the leg count is correct.
- **Proposed Fix:** ADJUDICATED NON-DEFECT. Leg B extended with B03 mixed-marker sub-probes; evidence provided. Record as adjudicated-non-defect with evidence note.
- **References:** —

#### F-S2104-P26-L02: Unbalanced Parenthesis Introduced by Pass-25 L02 Fix

- **Severity:** LOW
- **Category:** code-quality
- **Location:** `plugins/vsdd-factory/agents/devops-engineer.md` §Worktree Cleanup
- **Description:** Unbalanced parenthesis introduced by the pass-25 `L02` fix itself — 2 open parentheses, 1 close parenthesis.
- **Evidence:** Pass-25 L02 fix added content to devops-engineer.md §Worktree Cleanup; the resulting text has mismatched parentheses.
- **Proposed Fix:** Balance parentheses; preserve `ADR-031 §Decision 4` anchor.
- **References:** —

## Fix Mapping (11 of 11 CLOSED; POLICY 15 restore-leg residual OPEN — pass-27 anchor)

| Finding | Owner | Status at `7c3338e7` | Notes |
|---------|-------|---------------------|-------|
| B01 | test-writer | CLOSED at `ac478e41` | New Gate 1(e)/1(f); write-directive escapes reordered; canonical anchor token required |
| B02 | test-writer | CLOSED at `ac478e41` | 5 zero-nullified helpers; corpus extended to 7/7 guards with M5–M9 vectors |
| H01 | story-writer | CLOSED at `ac478e41` + `7c3338e7` | Gate cell resynced to HEAD; 21→23 gates; `^` removed; escape clauses corrected |
| H02 | architect | CLOSED at `ac478e41` | ADR-031 v1.14→v1.15; three retracted claims corrected; ARCH-INDEX v3.36→v3.37 |
| H03 | test-writer | CLOSED at `ac478e41` | `_build_nosplit()` wires `_nosplit` vars; `grep -v '_nosplit'` removed; mechanism 3 closed |
| **H04** | state-manager | **CLOSED at `a8ec290e` (D-939 SUPERSESSION)** | 7/8 per-guard records produced in-flight during D-938 burst (committed by `a8ec290e`); P25-M06 NOT PRODUCIBLE (editorial TD-VSDD-091 fix; no executable guard); D-938 asserted OPEN per honest-recording on evidence available at dispatch time; D-939 supersedes. POLICY 15 restore-leg residual OPEN (restore commands present as prose but not in `$ `-prefixed format; pass-27 anchor). |
| M01 | story-writer | CLOSED at `ac478e41` | T-015 registered in §Test Plan/§Architecture Mapping/§File Structure Requirements/§Tasks |
| M02 (gate) | test-writer | CLOSED at `ac478e41` | Meta-aware `MUST (NOT\|not) (use\|write\|store\|…)` escape added |
| M02 (doc) | story-writer | CLOSED at `7c3338e7` | Constraint scope corrected `#### Write Discipline` → `### Spec-Path Discipline` |
| M03 | test-writer | CLOSED at `ac478e41` | Guard (k) fail-open fallback removed; empty extraction now BLOCKs |
| L01 | story-writer | CLOSED (adjudicated non-defect) | 5-legs count correct; Leg B extended for clarity with evidence |
| L02 | devops-engineer | CLOSED at `7c3338e7` | Parenthesis balanced; ADR-031 §Decision 4 anchor preserved |

## Summary

| Severity | Count |
|----------|-------|
| BLOCKER | 2 |
| HIGH | 4 |
| MEDIUM | 3 |
| LOW | 2 |

**Total findings:** 11 (B2/H4/M3/L2)

**Overall Assessment:** block — two BLOCKERs require fix before next adversary pass.

**Convergence:** findings remain — iterate. Streak: 0/3 (B2 resets).

**Readiness:** requires revision. Pass-27 NEXT.

**Open finding:** H04 (red-gate-log §Pass-25 per-guard mutant evidence gap) is OPEN. Record this gap precisely; do not manufacture closure. This is the production-grade outcome.

**Pass-25 closure summary:** 13 GENUINELY-CLOSED · 1 GENUINELY-CLOSED-with-new-defect (P25-B01→B01/H01/M02) · 1 GENUINELY-CLOSED-but-not-sibling-swept (P25-B02→B02) · 1 PARTIAL/RECURRENT (P25-H02→H03) · 1 PARTIAL/RECURRENT (P25-L02→L02) · 1 DEFERRED.

## Novelty Assessment

| Field | Value |
|-------|-------|
| **Pass** | 26 |
| **New findings** | 11 |
| **Duplicate/variant findings** | 0 (B01=new class; B02=new class; all 11 novel) |
| **Novelty score** | 11/11 = 1.0 |
| **Median severity** | HIGH |
| **Trajectory** | 14→18→17→12→11→11→9→9→10→11→7→10→10→13→7→6→7→7→12→3→3→12→14→6→17→**11** |
| **Verdict** | FINDINGS_REMAIN — streak 0/3; two BLOCKERs; pass-27 required |

## Completeness Statement

**Covered:** the complete `worktree-identity-preflight.bats` (599 lines, guards (a)–(n) each audited individually, including (m)/(n) `head -1` mandate-token sites — order-independent, filter precedes them, no defect); the complete main bats suite (2971 lines, T-004/T-005/T-006/T-009 bodies audited per mandate); `_shared-context.md` §Spec-Path Discipline + `#### Write Discipline` traced clause-by-clause; `adversary.md` L30-119 + full token census for all seven guard anchors; `step-d5-adversary-convergence.md`; `devops-engineer.md` §Worktree Cleanup; `rules/worktree-protocol.md` + `code-delivery.lobster` §G.1 lines; the entire story spec incl. §Behavioral Contracts table, §Token Budget, §Tasks; BC-6.26.001 frontmatter + full `modified[]`; ADR-031 §Decision 1/4/7; red-gate-log attestation blocks.

**NOT covered — pass-27 carry-forward as first two items:** (1) H04 missing evidence: BC-6.26.001 §Description/§Preconditions/§Postconditions/§Invariants/§Canonical Test Vectors bodies, unswept for a second consecutive pass (context budget went to 2971-line bats audit as priority 1); (2) full bodies of `worktree-manage`/`code-delivery`/`fix-pr-delivery`/`deliver-story` SKILL.md, `greenfield.lobster`, `step-g-cleanup.md` §G.1, `adversarial-review/SKILL.md`.
