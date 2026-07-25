---
document_type: adversarial-review
level: ops
version: "1.0"
status: complete
producer: adversary
timestamp: 2026-07-25T06:00:00Z
phase: 5
inputs: []
input-hash: "[live-state]"
traces_to: ".factory/stories/S-21.04-story-worktree-write-path-discipline.md"
pass: 3
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-02.md"
story: S-21.04
cycle: v1.0-brownfield-backfill
verdict: NOT-CLEAN
reviewed_head: 2f84f56d
reviewed_branch: feature/S-21.04-story-worktree-write-path-discipline
base_commit: 948f0fb1
date: 2026-07-25
novelty: 0.76
---

# S-21.04 LOCAL Adversary Pass-3 — NOT-CLEAN

## Finding ID Convention

This review uses the project-local finding ID format: `F-S2104-P<PASS>-<SEQ>`

- `F`: Fixed prefix for per-story local findings
- `S2104`: Story identifier (S-21.04)
- `P<PASS>`: Pass number (P1, P2, …)
- `<SEQ>`: Three-digit sequence within the pass (001, 002, …)

Examples: `F-S2104-P3-001` (pass-3 finding 1), `F-S2104-P3-017` (pass-3 finding 17).

Re-opened findings from prior passes retain their original ID in the verification table (Part A) and are assigned a new sequential ID in the current pass's finding list (Part B).

## Reclassification Note (D-897)

Three pass-3 findings are **RECLASSIFIED-RECORD-ARTIFACT** per D-897:

- **F-S2104-P3-002** (BC-6.26.001 §Invariants — "no Invariant 6"): Derived from the fabricated pass-2 record. The real pass-2 adversary never asked for Invariant 6. Not a real defect.
- **F-S2104-P3-003** (ADR-031 §Decision 2/4 — "no write-path-vs-stage-path layer distinction"): Derived from the fabricated pass-2 record claiming a distinction was added. Not a real defect.
- **F-S2104-P3-014** (pass-2 routing table — "≥6 pass-2 IDs recorded CLOSED against wrong defect"): Derived from the fabricated pass-2 routing table. The swaps were artifacts of the reconstructed record, not real routing errors in the implementation.

These three findings are excluded from the streak count. Genuine finding count: **14** (B1/H7/M4/L2). See D-897 codification for full reclassification rationale.

## D-897/D-897-QA Interleaving Narrative

During pass-3 fix dispatch, commit `d975dc84` landed on the factory-artifacts branch (D-897-QA quarantine amendment — VOID-retitling adversary-pass-01/02 fabricated content). This commit does not affect the feature-branch worktree state. The 6 stash drops referenced in D-897-QA are pending human terminal execution (destructive-command-guard applies; not executed by state-manager).

## Summary

**Verdict:** NOT-CLEAN | **Count:** B1/H7/M7/L2 (17 total, 3 RECLASSIFIED-RECORD-ARTIFACT per D-897, 14 genuine) + 5 obs | **Reviewed HEAD:** `2f84f56d` | **Streak:** 0/3 (reset)

Pass-3 adversary reviewed the feature branch at `2f84f56d` (after all pass-2 fix legs). Found 17 findings across BLOCKER/HIGH/MEDIUM/LOW severity classes. Three findings (F-002, F-003, F-014) are RECLASSIFIED-RECORD-ARTIFACT per D-897 (derived from fabricated pass-2 record content — not real implementation defects). The remaining 14 genuine findings were all fixed by 4 agents before this D-898 closure burst. Trajectory: 14→18→17 (pass-1/2/3 counts). Novelty 0.76 HIGH. Streak reset to 0/3 by any finding; pass-4 required.

## Part A — Pass-2 Finding Verification

**Pass-2 verification (verbatim):** 8 CONFIRMED-CLOSED; 2 NOT-CLOSED [record artifacts]; 1 PAPER-FIX-WITH-REGRESSION (F-P2-012→P3-004); 1 CLOSED-WITH-NEW-FABRICATION (F-P2-013→P3-009/010); 3 MISLABELED/CLOSED-BY-DIFFERENT-MEANS; 3 FABRICATED-PREMISE [record artifacts]. Novelty 0.76; verdict FINDINGS_REMAIN.

## Part B — New Findings

| ID | Sev | Location | Description | BC/Policy |
|----|-----|----------|-------------|-----------|
| F-S2104-P3-001 | BLOCKER | step-g-cleanup.md:20-31,48-51 | §G.1 supplies no mechanism to discriminate PC2a sub-case (a) from PC2c; on a normal clean worktree find exits non-zero and PC2c says HALT with no predicate to determine the reason — operational wedge or guessed inversion; bats harness silently supplies the missing logic ([ ! -e ] pre-test) so suite is green because harness compensates for a doc gap | BC PC2 steps 1-2, EC-005; ADR §Decision 4 three-branch; AC-002(a)/AC-006; TD-VSDD-059 |
| F-S2104-P3-002 | HIGH | BC-6.26.001.md §Invariants | BC v1.5 has no Invariant 6 despite pass-2 record claiming "Invariant 6 + caller-side alignment" shipped — fabricated closure attestation [RECLASSIFIED-RECORD-ARTIFACT at D-897: real pass-2 never asked for Invariant 6] | TD-VSDD-059 |
| F-S2104-P3-003 | HIGH | ADR-031 §Decision 2/4 | No write-path-vs-stage-path layer distinction despite record claiming it [RECLASSIFIED-RECORD-ARTIFACT at D-897] | POLICY 19; TD-VSDD-059 |
| F-S2104-P3-004 | HIGH | _shared-context.md:94-95 | Canonical-root resolution space-unsafe (awk $2) with unguarded silent-wrong-root path (empty capture → git -C "" is a no-op → returns story-worktree root, recreating EC-006); contradicts step-d5-adversary-convergence.md:46 + agents/adversary.md:50 space-safe mandate | BC PC1+Inv3; EC-006; TD-VSDD-060 |
| F-S2104-P3-005 | HIGH | 8 caller surfaces | AC-007 propagated 1 of 3 PC branches — every surface authorizes removal on "empty result", which is precisely the PC2c state (empty stdout + non-zero exit); PC2c named nowhere outside step-g-cleanup.md | ADR INV-E21-004 three-branch; BC PC2c; AC-007 |
| F-S2104-P3-006 | HIGH | agents/adversary.md:44,54,59; adversarial-review/SKILL.md:77 | Retracted-premise residue produces self-contradicting instructions in the same document (line 54 corrected model vs lines 44/59 + SKILL:77 stale-snapshot framing) | BC Inv5; ADR v1.9 §Context |
| F-S2104-P3-007 | HIGH | worktree-identity-preflight.bats:101-107 | Sibling gate contractually locks in the retracted premise (test (e) requires the stale-snapshot phrase; sweeping the residue turns it RED) — premise correction structurally blocked | TD-VSDD-060; POLICY 15 |
| F-S2104-P3-008 | HIGH | story bats (10 locs) + fixtures README:7 + CHANGELOG:11 | 12 stale BC-6.26.001 v1.4 cites after BC advanced to v1.5 — third recurrence of the propagation-gap pattern in a new file class | POLICY 8; TD-VSDD-060 |
| F-S2104-P3-009 | MEDIUM | red-gate-log.md:148,154 | Fabricated implementation quote in the D-896 addendum (PREFLIGHT BLOCKED (PC2c) attributed to 19271a65; actual doc says HALT + surface exit code/stderr; harness emits PREFLIGHT HALT (PC2c)) — recurrence of the F-P1-009 class | TD-VSDD-059 |
| F-S2104-P3-010 | MEDIUM | red-gate-log.md:158 | "Invariant TBD" unresolved placeholder in attestation artifact | Canonical Principle Rule 6 |
| F-S2104-P3-011 | MEDIUM | story bats:353-359 vs 648-662 | PC2b HALT-direction ungated while PC2c's gated — direction-inversion mutant passes T-001 | POLICY 15 v1.4.10; TD-VSDD-060 |
| F-S2104-P3-012 | MEDIUM | story bats:369-380 | AC-001(b) PC1 leg gated only by three bare-token greps; no EC-006 WARNING gate; no negative gate on story-worktree rev-parse form | POLICY 15; AC-001(b) |
| F-S2104-P3-013 | MEDIUM | story §File Structure vs diff | adversary.md + adversarial-review/SKILL.md modified in diff but undeclared in File Structure Requirements; outside every AC | POLICY 14/17 |
| F-S2104-P3-014 | MEDIUM [process-gap] | pass-2 routing table | Systematic closure-label swap — ≥6 pass-2 IDs recorded CLOSED against work addressing a different defect [RECLASSIFIED-RECORD-ARTIFACT at D-897 — the swaps were artifacts of the fabricated record] | BC-5.39.001; TD-VSDD-059 |
| F-S2104-P3-015 | LOW | story bats:91-97 | _extract_spec_path_discipline_section exits on ^## only, over-capturing through §Story-Size Gate | POLICY 11 |
| F-S2104-P3-016 | LOW | story AC-007(d) vs File Structure row 14 | rules/worktree-protocol.md is a rules file covered by no AC-007 sub-clause category | POLICY 4 |
| F-S2104-P3-017 | MEDIUM | story Token Budget:130 | Cites ADR-031 v1.3 while ADR is v1.10; only version signal in story | POLICY 8/19; TD-VSDD-060 |

**Observations (verbatim):** O-P3-001 red-gate-log §Summary "3 bats/1..3" historically scoped to 8e3c432e — INFO; O-P3-002 [process-gap] adversary-side literal-evidence rule needed (no finding may cite a grep/content claim not actually executed) — HIGH; O-P3-003 [process-gap] routing table needs REJECTED-WITH-RATIONALE state so correct rejections stop masquerading as closures — MEDIUM; O-P3-004 POLICY 21/13 satisfied, 2>/dev/null residue sweep verified zero — INFO; O-P3-005 find -type f cannot detect directory-only shadow state; future BC evolution note — INFO.

## Fix Mapping Table

All 14 genuine findings fixed before this D-898 closure burst:

| Finding(s) | Agent | Commit SHA | Work |
|------------|-------|-----------|------|
| F-001 BLOCKER (§G.1 discrimination predicate) | implementer (worktree) | be9343d0 | Normative discrimination predicate added to §G.1 |
| F-001 (DOC-PARITY bats gate) | test-writer (worktree) | 62ce8938 | DOC-PARITY gate added to bats suite |
| F-004 (space-unsafe awk $2) | implementer (worktree) | b5677b4a | ${line#worktree } space-safe expansion + non-empty assertion |
| F-005 (three-branch protocol) | implementer (worktree) | 681f6d91 | Three-branch protocol applied at all 8 caller surfaces |
| F-006 (stale-snapshot residue) | implementer (worktree) | d7dc5028 | Stale-snapshot residue swept from agents/adversary.md + adversarial-review/SKILL.md |
| F-007 (worktree-identity-preflight test (e) re-anchor) | test-writer (worktree) | 62ce8938 | Test (e) re-anchored to corrected premise |
| F-008 (bats/README v1.5 cite sweep) | test-writer (worktree) | 62ce8938 | 10 bats locs + fixtures README:7 updated to v1.5 |
| F-008 (CHANGELOG cite) | implementer (worktree) | b44442b2 | CHANGELOG:11 v1.4→v1.5 |
| F-009 (red-gate-log fabricated quote) | state-manager | d975dc84 (D-897) | red-gate-log v1.2 — PC2c quote corrected verbatim |
| F-010 (red-gate-log "Invariant TBD" placeholder) | state-manager | d975dc84 (D-897) | Placeholder removed; verbatim BC text substituted |
| F-011 (PC2b direction gate) | test-writer (worktree) | 62ce8938 | PC2b HALT-direction gate added to bats T-001 |
| F-012 (EC-006 negative gate) | test-writer (worktree) | 62ce8938 | AC-001(b) EC-006 WARNING gate + negative gate added |
| F-013 (File Structure: adversary.md + adversarial-review/SKILL.md) | story-writer | 41b022ac | File Structure rows 15–16 added for both files |
| F-015 (extractor bound over-capture) | test-writer (worktree) | 62ce8938 | _extract_spec_path_discipline_section bound tightened |
| F-016 (AC-007(d) category for rules files) | story-writer | 41b022ac | AC-007(d) sub-clause updated to include rules files |
| F-017 (ADR-031 v1.3 stale cite) | story-writer | 41b022ac | Token Budget:130 cite updated to ADR-031 §Decision 4 (stable anchor) |

**Reclassified (not fixed — not real defects):**
- F-002 → RECLASSIFIED-RECORD-ARTIFACT (D-897)
- F-003 → RECLASSIFIED-RECORD-ARTIFACT (D-897)
- F-014 → RECLASSIFIED-RECORD-ARTIFACT (D-897)

**Worktree HEAD after all genuine fix legs:** `b44442b2` (27 commits over base `948f0fb1`)

**Suite results (orchestrator-verified):** 4/4 + 14/14 + 2/2 green.

## Novelty Assessment

**Novelty score:** 0.76 HIGH

**Basis:** Pass-3 findings are distributed across the genuine-finding classes: BLOCKER (1), HIGH (5 genuine of 7), MEDIUM (4 genuine of 7 — 3 reclassified), LOW (2). The BLOCKER (F-001 §G.1 discrimination predicate gap) is a novel class not present in pass-1 or pass-2. The HIGH findings (F-004 space-unsafe canonical-root resolution, F-005 PC2c empty-result authorization gap, F-006 stale-snapshot residue self-contradiction, F-007 premise-lock in test gate) are all first-occurrence classes. F-008 (v1.4 cite propagation in bats/README/CHANGELOG) is a third recurrence of the propagation-gap pattern but in a new file class. Three reclassified findings (F-002/003/014) were entirely derived from the fabricated record — no prior-pass analog exists in the real finding set.

**Trajectory:** 14 (pass-1) → 18 (pass-2) → 17 (pass-3 total) | 14 genuine pass-3 findings. Finding count remains elevated; structural gaps in the three-branch protocol and space-safety invariant drove the sustained finding rate.
