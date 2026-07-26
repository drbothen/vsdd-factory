---
pass: 13
verdict: NOT-CLEAN
reviewed_head: 264f53b6
fixes_landed_head: 09cfce81
novelty: 0.55
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-12.md"
---

## Summary

Pass-13 adversarial review of S-21.04 implementation. 10 findings + 1 deferred cross-story fixed in-scope by orchestrator adjudication (B0 / H3 / M5 / L2). Novelty 0.55. Trajectory 14→18→17→12→11→11→9→9→10→11→7→10→10. Streak: 0/3.

---

## Part A — Findings

### Finding Table

| ID | Severity | Location | Summary | BCs / Policies |
|----|----------|----------|---------|----------------|
| F-S2104-P13-001 | HIGH | bats:487-489 vs _shared-context:66-70,113 | Pass-12 fix closed a paper-gate with a new paper-gate: 'are FORBIDDEN' polarity-blind; inverting the prohibition (mandate CWD-relative, forbid absolute) left all 9 tests GREEN — BC PC1 core still ungated | BC PC1; POLICY 11, 15; TD-VSDD-059 |
| F-S2104-P13-002 | HIGH | red-gate-log vs bats:476-492 | Two new T-001 assertion sites landed at 264f53b6 with zero red-gate-log attestation and no mutant record; :288 count stale (5 groups/9 sites vs true 6/11); 2nd-generation recurrence of the F-P10-007/F-P12-002 class the same wave was closing | POLICY 15; TD-VSDD-059 |
| F-S2104-P13-003 | HIGH | STORY-INDEX:719,:748,:761 | F-P12-D1 closure PARTIAL — three live sites still assert 5 stories/27 pts/2 waves, one inside the same blockquote line now saying 6/35/3 | POLICY 14/17; TD-VSDD-060 |
| F-S2104-P13-004 | MEDIUM | red-gate-log:66-67,:96-97 vs story:239-240 | Story v1.16 multi-AC notation did not propagate to red-gate-log §Traces/§Bats tables — 13 pairs story-side vs 9 log-side | POLICY 14/17 |
| F-S2104-P13-005 | MEDIUM | story:105,128 vs BC:321-327,311 | Awareness group carries three files; BC anchors name two — step-d5 attributed to an anchor list it is absent from | POLICY 4, 5 |
| F-S2104-P13-006 | MEDIUM | bats:53,:1143 vs story:99 | Header inventory maps T-009 → AC-007(d)/AC-009; story AC-007 explicitly excludes the awareness surfaces (AC-009's domain) | POLICY 4 |
| F-S2104-P13-007 | MEDIUM | bats:488,:1210-1215 | New bare "line 60" pin introduced by the P12-003 fix inside an emitted message, in the same burst that closed P12-008 for the identical class; siblings line 77/93 | TD-VSDD-091/060 |
| F-S2104-P13-008 | MEDIUM | story:93 | AC-001 Gate cell not extended for the gates added at 264f53b6 — AC-001(a) still had no gate cited | POLICY 14/17 |
| F-S2104-P13-009 | LOW | bats:926,:933 | P12-007 residue: two T-005 sites retain historical opening without closing SHA | POLICY 15; TD-VSDD-060 |
| F-S2104-P13-010 | LOW (pending intent, orchestrator-adjudicated: mark complete with evidence) | story:181-194 | Tasks 12-14 [x] but Tasks 1-11 [ ] though demonstrably complete | TD-VSDD-060 |
| F-S2104-P13-D1 | deferred→fixed-in-scope | epics/E-21:45,46,81,87,98,151,168,210,283 | S-21.06 registration (D-891) never propagated to the epic file — "five hardening stories", EAC-001 five, 5/27 totals, five isolated nodes | — |

---

## Observations (NOT findings)

Behavioral axis holds (full chain reproduced; ordering gate load-bearing; extractors bound correctly); all four primary-path gates satisfied; T-007/T-008/T-009 satisfied; retired-class sweeps CLEAN; AC-007(d) surface-set completeness re-derived — no gap; [process-gap] second consecutive wave shipped gate-strengthening without red-gate-log attestation — codify: any fix wave adding/strengthening a bats assertion site MUST append its mutant record to the red-gate-log in the same burst and reconcile the running group/site count; CHANGELOG accurate; POLICY 19 cites hold. Diagnosis: "each fix wave closes the named site and re-seeds the class one hop away … until a fix wave is required to (a) enumerate the class members it swept and (b) append the attestation for every assertion site it touched."

---

## Per-Pass-12 Verification

| Finding | Status | Notes |
|---------|--------|-------|
| F-P12-001 | CONFIRMED-CLOSED | Scope note ADR-versionless/BC-versioned distinction correct; no residue |
| F-P12-002 | CONFIRMED-CLOSED | Group description .md-qualified; P11-003 mutant record present; Summary HEAD cite updated |
| F-P12-003 | PARTIAL/REGRESSED-IN-KIND | Two gates added (FORBIDDEN polarity-blind + Forbidden: marker); INVERSION vector not gated → P13-001/002/008 |
| F-P12-004 | CONFIRMED-CLOSED | Both rows now AC-001..AC-006; no residue |
| F-P12-005 | PARTIAL | T-001 multi-AC trace added to story; not yet propagated to log → P13-004 |
| F-P12-006 | PARTIAL | Attribution rewrite landed; step-d5 attribution stale → P13-005 |
| F-P12-007 | PARTIAL | 1 of 13 sites corrected; 12 siblings retained historical-opening form → P13-009 |
| F-P12-008 | PARTIAL | 3 :531 pins corrected; new "line 60" pin introduced in same burst → P13-007 |
| F-P12-009 | CONFIRMED-CLOSED-with-mis-anchor | T-008 anchor added; header maps T-009 to wrong AC-007(d) → P13-006 |
| F-P12-010 | CONFIRMED-CLOSED | D-907 date monotonicity restored |
| F-P12-D1 | PARTIAL | :719 stats corrected; three sibling sites remain → P13-003/D1 |

Tally: 5 CONFIRMED-CLOSED / 6 PARTIAL / 0 REGRESSED + 1 regressed-in-kind (F-P12-003→P13-001/002/008).

---

## Fix Mapping

| Finding | Status | Fix Agent / Commit |
|---------|--------|-------------------|
| F-S2104-P13-001 | FIXED 09cfce81 | test-writer — 3 mutants incl. inversion vector |
| F-S2104-P13-002 | FIXED this burst | state-manager D-909 |
| F-S2104-P13-003 | FIXED this burst | state-manager D-909 |
| F-S2104-P13-004 | FIXED this burst | state-manager D-909 |
| F-S2104-P13-005 | FIXED 106bb5f5 | story-writer — step-d5 annotated story-scoped |
| F-S2104-P13-006 | FIXED 09cfce81 | test-writer — T-009 → AC-009 only |
| F-S2104-P13-007 | FIXED 09cfce81 | test-writer — zero bare line pins; predicate stdout |
| F-S2104-P13-008 | FIXED 106bb5f5 | story-writer — AC-001 Gate extended with AC-001(a) prohibition gates |
| F-S2104-P13-009 | FIXED 09cfce81 | test-writer — closing SHAs complete; predicate every hit " at <8-hex>" |
| F-S2104-P13-010 | FIXED 106bb5f5 | story-writer — Tasks 1-11 [x] with per-task closing SHAs |
| F-S2104-P13-D1 | FIXED 106bb5f5 | story-writer (orchestrator-adjudicated fix-in-scope): epic fully synced to 6 stories/35 pts/3 waves incl. mermaid node, EAC-001, predicate one exempt provenance hit |
