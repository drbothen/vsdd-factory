---
pass: 9
verdict: NOT-CLEAN
reviewed_head: 9d896bf5
novelty: 0.60
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-08.md"
---

## Summary

Pass-9 adversarial review of S-21.04 implementation. 10 findings (B0 / H3 / M4 / L3). Novelty 0.60. Trajectory 14→18→17→12→11→11→9→9→10. Streak: 0/3.

---

## Part A — Findings

### Finding Table

| ID | Severity | Location | Summary | BCs / Policies |
|----|----------|----------|---------|----------------|
| F-S2104-P9-001 | HIGH | bats:1113,1122 vs AC-009 | T-009 paper-gate: bare-token alternation satisfiable by any incidental mention; AC-009's primary obligation asserted by nothing; byte-identical pattern retired at pass-7 for T-007; sibling 30 lines above never swept | TD-VSDD-059/060; POLICY 15 |
| F-S2104-P9-002 | HIGH | STORY-INDEX:728; S-21.05 story:101,130 | Pass-8 sibling fix bumped BC-6.27.001 v1.3→v1.4; only BC-INDEX swept; three dependent parity fields still v1.3 — the sweep's own blast radius was never swept | POLICY 14/17; TD-VSDD-060 |
| F-S2104-P9-003 | HIGH | story:140 vs :11,:46,:113 | Token Budget row v1.9 while BC table/BC-INDEX/BC/red-gate-log all v1.10; story's own v1.12 entries FALSELY attest the Token Budget bump | POLICY 8; TD-VSDD-059 |
| F-S2104-P9-004 | MEDIUM | red-gate-log:266,272 | NEW §T-008 addendum traces AC-007 to "BC-6.26.001 v1.10 Invariant 5 (caller-side propagation)" — AC-007 traces to PC2 + Invariant 2; Invariant 5 is the gitignored-mechanism invariant; fix-wave-introduced mis-anchor | POLICY 4; 14/17 |
| F-S2104-P9-005 | MEDIUM | CHANGELOG:26-32 | (a) step-d5 listed among dispatch surfaces (zero worktree-remove occurrences; received only the corrected model) — F-P8-005's category error re-instantiated; (b) "T-007 exercises the chain" — it is a doc-parity mandate gate | POLICY 4; Task 11 |
| F-S2104-P9-006 | MEDIUM | story:95 | AC-007 Gate cell lacks the T-008 cite its siblings carry | POLICY 14/17/8 |
| F-S2104-P9-007 | MEDIUM | red-gate-log:276-290 | §T-009 addendum records no mutant vector — appended in the same burst that recorded T-007's and T-008's proofs; exact F-P8-007 recurrence one sibling later; would have exposed P9-001 | POLICY 15; TD-VSDD-059 |
| F-S2104-P9-008 | LOW | README:20 | "the two remaining propagation gates" — count-bearing unnamed crossref to IDs named in the same burst | POLICY 4/14 |
| F-S2104-P9-009 | LOW (pending intent verification) | STORY-INDEX:732 | E-21 "BC coverage:" blockquote pins BC-6.26.001 v1.3 / BC-6.27.001 v1.3 / "ADR-031 v1.3 governs" — stale by 7/1/9 versions; authorial intent unadjudicated | POLICY 8; TD-VSDD-060 |
| F-S2104-P9-010 | LOW | bats:1051 | Volatile step-g-cleanup.md:54 file:line comment pin introduced by the pass-7 fix | TD-VSDD-091 |

---

## Observations (NOT findings)

behavioral axis converged (independent full re-derivation); pass-8's class-bounded prescription applied to exactly one artifact and produced a genuinely clean result there; remaining findings all in artifacts where it was NOT run; META-class named: "a fix's own blast radius is not treated as part of the fix"; recommended intervention = literal-shell dependents grep per changed identifier recorded in the closure (ADOPTED at this fix burst per human ruling); pass-08 reviewed_head oddity noted (unverifiable read-only); D-902 RG-reconciliation note describes RG-005 as "AC-007/AC-008 gate" (erratum-history metadata, excluded class); NOT re-reported: standard exclusion list.

---

## Per-Pass-8 Verification

F-P8-001 CONFIRMED-CLOSED (whole-file RG map independently re-verified at all 9 mention sites); F-P8-002 PARTIAL(→P9-004/007); F-P8-003 CONFIRMED-CLOSED (both BCs; downstream gap raised fresh as P9-002); F-P8-004 PARTIAL(→P9-003/006); F-P8-005 PARTIAL(→P9-005); F-P8-006 CONFIRMED-CLOSED; F-P8-007 CONFIRMED-CLOSED; F-P8-008 CONFIRMED-CLOSED; F-P8-009 PARTIAL(→P9-008). Tally 4/5/0/0. Novelty 0.60; FINDINGS_REMAIN.

---

## Fix Mapping

| Finding ID | Fix leg | Commit SHA | Disposition |
|------------|---------|------------|-------------|
| F-S2104-P9-001 | bats T-009 gates: obligation-asserting (3 per file), mutant-proven; bare alternation class-completed (5 additional survivors) | 2992b53d + 3326e4dd | FIXED |
| F-S2104-P9-002 | STORY-INDEX S-21.05 row BC cite v1.3→v1.4 + story v1.5 + input-hash; E-21 blockquote versionless pins (this burst) | 92e87330 + this burst | FIXED |
| F-S2104-P9-003 | S-21.04 v1.12→v1.13 Token Budget row v1.9→v1.10; error-acknowledgment in v1.13 changelog; blast-radius: 4 dependents verified | 92e87330 | FIXED |
| F-S2104-P9-004 | red-gate-log v1.6→v1.7: §T-008 BC-trace corrected to PC2 + Invariant 2 (caller-side dispatch gate), per story AC-007 (this burst) | this burst | FIXED |
| F-S2104-P9-005 | CHANGELOG: step-d5 moved to shadow-write-model sentence; T-007 labeled doc-parity mandate gate; 16 blast-radius verifications recorded | 32cacbd6 | FIXED |
| F-S2104-P9-006 | S-21.04 v1.12→v1.13: AC-007 Gate cell T-008 cite added | 92e87330 | FIXED |
| F-S2104-P9-007 | red-gate-log v1.6→v1.7: §T-009 mutant evidence recorded verbatim (this burst) | this burst | FIXED |
| F-S2104-P9-008 | bats fixtures README: "the two remaining propagation gates" → T-008/T-009 named | 2992b53d | FIXED |
| F-S2104-P9-009 | STORY-INDEX E-21 BC coverage blockquote: three versioned pins made versionless (this burst) | this burst | FIXED |
| F-S2104-P9-010 | bats: step-g-cleanup.md:54 volatile file:line pin → stable §G.1 anchor class sweep (zero live file:line pins confirmed) | 2992b53d | FIXED |

---

## Human Ruling

AskUserQuestion (orchestrator session, 2026-07-25): "Keep looping to 3-CLEAN" — asymptotic acceptance REJECTED; BLAST-RADIUS RULE adopted for all fix legs (every changed identifier/version/gate gets a dependents-grep with recorded stdout).
