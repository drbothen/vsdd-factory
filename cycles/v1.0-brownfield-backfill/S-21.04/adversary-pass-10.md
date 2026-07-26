---
pass: 10
verdict: NOT-CLEAN
reviewed_head: 2c8eff8b
novelty: 0.62
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-09.md"
---

## Summary

Pass-10 adversarial review of S-21.04 implementation. 11 findings (B0 / H3 / M6 / L2). Novelty 0.62. Trajectory 14→18→17→12→11→11→9→9→10→11. Streak: 0/3.

---

## Part A — Findings

### Finding Table

| ID | Severity | Location | Summary | BCs / Policies |
|----|----------|----------|---------|----------------|
| F-S2104-P10-001 | HIGH | decision-log:11838 vs STORY-INDEX:732 | First application of the BLAST-RADIUS gate produced a FALSE-GREEN: recorded ADR-031 stdout lists a hit at line 724 (no such hit) and omits line 732's TWO live ADR-031 v1.3 pins, then asserts "zero live stale versioned pins in the BC coverage blockquote" — false; F-P9-009 residue survives in the line the fix edited (+4 stale BC pins same line, pending intent) | POLICY 15; TD-VSDD-059/060; L-EDP1-070; POLICY 19 |
| F-S2104-P10-002 | MEDIUM | S-21.05:132 | Live ADR-031 v1.3 Token Budget pin (stale by 9 versions) in the artifact this burst edited — adjacent row was bumped | TD-VSDD-091/060 |
| F-S2104-P10-003 | MEDIUM | bats:537 vs :531 | Dead assertion introduced this burst: :537 strict superset of strengthened :531 — can never fail; tautology | POLICY 11; TD-VSDD-059 |
| F-S2104-P10-004 | HIGH | story:91-94 vs :95-98 | Gate-column class 4/5 unswept: AC-003/004/005 no T-cites while AC-006..009 all have them; AC-002 says "manual:" listing exactly the greps T-001/T-002/T-005/T-006 automate — retired F-P6-009(b) recurrence one AC over | POLICY 14/17/8; TD-VSDD-060 |
| F-S2104-P10-005 | HIGH | BC:310,320-323 vs :138-145,:256; ADR:223-245 | §Traceability Architecture rows list only 2 skill-docs though Precondition 3 + Invariant 5/AC-009 made devops-engineer + adversary + adversarial-review normative surfaces; v1.10 "no other row contradicts" claim falsified; ADR §Decision 4 same 2-surface enumeration | POLICY 4/19; TD-VSDD-060 |
| F-S2104-P10-006 | MEDIUM | bats:454-462 vs _shared-context:72-73 | AC-001(c) names three load-bearing artifacts; gate asserts only DELIVERY — deleting pr-review.md undetectable | TD-VSDD-059; POLICY 15 |
| F-S2104-P10-007 | MEDIUM | red-gate-log:41 | Summary attests "All GREEN at 9d896bf5" after v1.7 recorded gate strengthening at 2992b53d+3326e4dd; no suite-GREEN for the changed gate set | POLICY 15/14 |
| F-S2104-P10-008 | MEDIUM | red-gate-log:285 | "5 additional bare survivors strengthened, 8 mutants" — count-bearing UNNAMED crossref (retired class), unreconcilable against the artifact | POLICY 15; POLICY 4/14 |
| F-S2104-P10-009 | LOW | red-gate-log:267,273 | P9-004 fix dropped the version pin every sibling trace row carries — parity drift introduced by the correction | POLICY 14/17 |
| F-S2104-P10-010 | LOW | bats ~:635,:891,:1075-1095 | Present-tense state labels falsified by shipped artifacts (7+ sites) | POLICY 4; TD-VSDD-060 |
| F-S2104-P10-011 | MEDIUM | story:154-165 vs :209-217 | §Tasks plans authoring for T-001..T-004 only; T-005/T-006 have no task — the unswept sixth table of the F-P6-009 fix | POLICY 14/17; POLICY 1 |

---

## Observations (NOT findings)

Behavioral axis re-derived and confirmed converged (§G.1 byte-faithful to BC/ADR; all 12 teardown sites guarded; POLICY 13/21 clean; zero shadow content); bats:892/:897 near-duplicate non-directory gates both satisfied by :49 — not provably vacuous, not filed; T-009's three adversary.md gates all satisfied by single line :54 — non-independent but mutant-adequate; "§G.4" comment-anchor cosmetic; AC-006 wording laggard semantically equivalent; blast-radius discipline measurably reduced propagation misses but did not close the META-class — moved failure from "sweep not run" to "sweep stdout not reproducible" + new sub-class "the strengthening's own blast radius"; recommended: post-state re-execution + sibling-assertion implication check (ADOPTED); NOT re-reported: standing exclusion list.

---

## Per-Pass-9 Verification

F-P9-001 CONFIRMED-CLOSED (residue→P10-003/008); F-P9-002 PARTIAL(→P10-001/002); F-P9-003 CONFIRMED-CLOSED; F-P9-004 CONFIRMED-CLOSED (residue→P10-009); F-P9-005 CONFIRMED-CLOSED; F-P9-006 PARTIAL(→P10-004); F-P9-007 CONFIRMED-CLOSED (residue→P10-008); F-P9-008 CONFIRMED-CLOSED; F-P9-009 PARTIAL(→P10-001); F-P9-010 CONFIRMED-CLOSED. Tally 7/3/0 + 1 falsified attestation. Novelty 0.62; FINDINGS_REMAIN.

---

## Fix Mapping

| Finding ID | Fix leg | Commit SHA | Disposition |
|------------|---------|------------|-------------|
| F-S2104-P10-001 | STORY-INDEX line-732 class-death (2 ADR-031 v1.3 + 4 BC pins versionless); decision-log D-906 erratum block appended to D-905 (blast-radius gate false-GREEN; phantom line-724 hit; missed line-732 pins) | this burst (D-906) | FIXED |
| F-S2104-P10-002 | S-21.05 v1.5→v1.6: ADR-031 v1.3 Token Budget pin made versionless | f5b11d43 | FIXED |
| F-S2104-P10-003 | bats :537 dead assertion differentiated: requires .md-qualified form (strict SUBSET not superset of :531); implication check zero dead/redundant | 2c8eff8b | FIXED |
| F-S2104-P10-004 | S-21.04 v1.13→v1.14: Gate column AC-003/004/005 T-cites added; AC-002 "manual:" scope corrected | f5b11d43 | FIXED |
| F-S2104-P10-005 | BC-6.26.001 v1.10→v1.11: §Traceability Architecture five-surface enumeration (devops-engineer + adversary + adversarial-review added); v1.10 attestation gap acknowledged. ADR-031 v1.12→v1.13: §Decision 4 five-surface enumeration mirroring BC v1.11 | 8b6f9880 (BC) + 3192a208 (ADR) | FIXED |
| F-S2104-P10-006 | bats: pr-review.md + story-frontmatter gates added to AC-001(c) assertion; mutant-proven: scratch-delete pr-review.md from clause → RED, restore → GREEN | 2c8eff8b | FIXED |
| F-S2104-P10-007 | red-gate-log v1.7→v1.8: Summary line HEAD cite → 2c8eff8b with suite-level verification (orchestrator ran bats → 9/9 ok + 14/14 ok at 2c8eff8b, 2026-07-26) | this burst (D-906) | FIXED |
| F-S2104-P10-008 | red-gate-log :285: "5 additional bare survivors strengthened, 8 mutants RED/restore GREEN" replaced with named list (5 named gates + "8 scratch mutants RED / 8 restores GREEN recorded by test-writer") | this burst (D-906) | FIXED |
| F-S2104-P10-009 | red-gate-log :267/:273: version pin restored to current — BC-6.26.001 v1.11 PC2 + Invariant 2; traces_to → v1.11; all sibling v1.10 cites → v1.11 | this burst (D-906) | FIXED |
| F-S2104-P10-010 | bats: 22 stale present-tense labels → historical "was RED at 60f0d2d6" form; class sweep zero survivors | 2c8eff8b | FIXED |
| F-S2104-P10-011 | S-21.04 v1.13→v1.14: §Tasks 12-14 added for T-005/T-006/T-007-009 | f5b11d43 | FIXED |

---

## Incident Note

Input-hash convention inversion caught pre-adversary by orchestrator report review (story-writer commit d9b178df stored operator-cache authoritative values; dev-CLI values were the divergent ones — corrected per D-892 convention). Not a finding; fix committed before pass-10 adversary dispatch.
