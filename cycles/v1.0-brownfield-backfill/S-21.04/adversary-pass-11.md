---
pass: 11
verdict: NOT-CLEAN
reviewed_head: 2c8eff8b
fixes_landed_head: 92f986ab
novelty: 0.56
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-10.md"
---

> **Process-gap note (reviewed_head/fixes_landed_head dual-field convention — D-907):** `reviewed_head` records the HEAD the adversary actually reviewed (pre-fix-wave: 2c8eff8b). `fixes_landed_head` records the worktree HEAD after that pass's fix wave (post-fix: 92f986ab). Pass-08 and pass-10 frontmatter conflated the two fields (pass-10 cited the pass-10 FIX commit 2c8eff8b as reviewed_head; the tree at 2c8eff8b contains F-P10 comments that could not exist in the reviewed tree). Retro-note recorded here; those files are not rewritten (frozen-historical).

## Summary

Pass-11 adversarial review of S-21.04 implementation. 7 findings (B0 / H3 / M3 / L1). Novelty 0.56. Trajectory 14→18→17→12→11→11→9→9→10→11→7 (plateau broken downward). Streak: 0/3.

---

## Part A — Findings

### Finding Table

| ID | Severity | Location | Summary | BCs / Policies |
|----|----------|----------|---------|----------------|
| F-S2104-P11-001 | HIGH | story:115,142 vs BC:4/BC-INDEX:2053/STORY-INDEX:727/red-gate-log:13 | Same-burst quintuple-parity break: BC bumped to v1.11 but story BC table + Token Budget still v1.10 — identical two rows F-P9-003 corrected one version earlier; story edited to v1.14 in the same burst without the sync | POLICY 8; 14/17; TD-VSDD-060 |
| F-S2104-P11-002 | HIGH | story:91 (+ :107) vs bats:454-471 | Gate-column class not swept to completion: AC-001 still manual-only listing exactly the greps T-001 automates; Architecture Mapping range excludes AC-001; F-P10-004 recurrence one AC over in the same table the fix edited | POLICY 14/17; 11; TD-VSDD-060 |
| F-S2104-P11-003 | HIGH | bats:502,521,530 vs :548,:1079 | P10-003 strengthening applied to one of four structurally identical primary gates; three siblings bare while guarded artifacts fully-qualified and secondary surfaces gated at full strength; mutant-proven gap (extensionless degradation leaves suite GREEN) | BC PC2/AC-007(a)-(c); POLICY 15; TD-VSDD-059/060 |
| F-S2104-P11-004 | MEDIUM | story:103 vs :227-245; BC:311; ADR:248-274 | Five-surface propagation stopped at BC+ADR: story Architecture Mapping still opens "amends two skill-doc files" against 13 modify rows and the five-surface classification | POLICY 4; 14/17; D-902 count class |
| F-S2104-P11-005 | MEDIUM | red-gate-log:286 | P10-008 named but not reconciled: 5 semicolon groups spanning 9 assertion sites vs "8 scratch mutants" — 5/9/8 unreconcilable | POLICY 15; 4/14 |
| F-S2104-P11-006 | MEDIUM | bats:16 | P10-010 "zero survivors" falsified: header layer comment still present-tense "RED for winning playbook until implementer propagates" | POLICY 4; TD-VSDD-059/060 |
| F-S2104-P11-007 | LOW (pending intent) | STORY-INDEX:725,726 | Two live ADR-031 v1.3 pins in catalog rows of the file whose line-732 was declared class-death — catalog rows not covered by the changelog-entry exclusion | POLICY 19; TD-VSDD-091/060 |

---

## Observations (NOT findings)

Behavioral axis re-derived converged (all four awk extractors hand-traced; §G.1 faithful; no 13th ungated teardown site); whole-repo retired-class sweeps clean; P10-003 attestation imprecision benign (subsumed weak-then-strong arrangement correct, phrasing over-claimed) — recorded not filed; [process-gap] reviewed_head records post-fix HEAD not reviewed HEAD (pass-10 frontmatter cites the pass-10 FIX commit; tree contains F-P10 comments that couldn't exist in the reviewed tree) — recurrence beyond the pass-08 instance, codification needed: record BOTH reviewed_head (pre-fix-wave) and fixes_landed_head; standing exclusion list applied in full. Novelty 0.56; FINDINGS_REMAIN; diagnosis: "Convergence is not reachable while sweep scope is declared in prose rather than derived from the class predicate."

---

## Per-Pass-10 Verification

F-P10-001 CONFIRMED-CLOSED (residue→P11-007); F-P10-002 CONFIRMED-CLOSED; F-P10-003 CONFIRMED-CLOSED (residue→P11-003); F-P10-004 PARTIAL(→P11-002); F-P10-005 CONFIRMED-CLOSED (residue→P11-004); F-P10-006 CONFIRMED-CLOSED; F-P10-007 CONFIRMED-CLOSED; F-P10-008 PARTIAL(→P11-005); F-P10-009 CONFIRMED-CLOSED; F-P10-010 PARTIAL(→P11-006); F-P10-011 CONFIRMED-CLOSED. Tally 8/3/0 + 2 falsified sweep attestations.

---

## Fix Mapping

| Finding ID | Fix leg | Commit SHA | Disposition |
|------------|---------|------------|-------------|
| F-S2104-P11-001 | S-21.04 v1.14→v1.15: BC table + Token Budget → v1.11; predicate all live pins == BC frontmatter, PASS | 139c9313 | FIXED |
| F-S2104-P11-002 | S-21.04 v1.14→v1.15: AC-001 Gate automation in gate column; Architecture Mapping range AC-001..AC-006; whole-column manual survivors justified per hit | 139c9313 | FIXED |
| F-S2104-P11-003 | bats: three sibling primary-path gates strengthened to .md-qualified form; 3 mutants RED/restore GREEN; implication check no death; class predicate every primary-path pattern .md-qualified or justified | 92f986ab | FIXED |
| F-S2104-P11-004 | S-21.04 v1.14→v1.15: count-free five-group Architecture Mapping; count-phrase predicate zero hits | 139c9313 | FIXED |
| F-S2104-P11-005 | red-gate-log v1.8→v1.9: :286 sentence count clause replaced — 5 gate GROUPS / 9 assertion sites reconciled with 8 mutants | this burst (D-907) | FIXED |
| F-S2104-P11-006 | bats: header label corrected to historical "Was RED at 93ec340a until … a4d4ffab/43ea70ba"; predicate zero present-tense survivors | 92f986ab | FIXED |
| F-S2104-P11-007 | STORY-INDEX.md lines ~725/726: two ADR-031 v1.3 catalog-row pins made versionless; scope sentence added; orchestrator adjudication: catalog rows are LIVE traceability surfaces → class-death applies | this burst (D-907) | FIXED |
