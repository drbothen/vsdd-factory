---
pass: 8
verdict: NOT-CLEAN
reviewed_head: 9d896bf5
novelty: 0.58
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-07.md"
---

## Summary

Pass-8 adversarial review of S-21.04 implementation. 9 findings (B0 / H3 / M4 / L2). Novelty 0.58. Trajectory 14→18→17→12→11→11→9→9. Streak: 0/3.

---

## Part A — Findings

### Finding Table

| ID | Severity | Location | Summary | BCs / Policies |
|----|----------|----------|---------|----------------|
| F-S2104-P8-001 | HIGH | red-gate-log §Bats Tests table L61-63 vs §Traces + addenda | Primary attestation table still carries the F-P1-009 fabricated RG-IDs (T-001→RG-003, T-002→RG-004, T-003→RG-005); only §Traces was corrected at D-895; RG-004/RG-005 since legitimately allocated elsewhere → same file attributes RG-004 to both T-002 and T-004, RG-005 to both T-003 and T-006; 6-pass survival — every correction scoped to the cited line, not the cited defect | POLICY 1/4; TD-VSDD-059 |
| F-S2104-P8-002 | HIGH | red-gate-log — no attestation for the F-P4-009/F-P4-002 gates | 7 of 9 tests attested under tdd_mode: strict; Summary claims 9-test All-GREEN; both omitted gates had real unrecorded RED baselines; third recurrence of fix-only-the-tests-this-pass-named | POLICY 14/17/15; TD-VSDD-060; Iron Law |
| F-S2104-P8-003 | HIGH | BC-6.26.001 L312 vs L307+L143-144+L243 | §Traceability "ADR Reference \| none" contradicts two live ADR-031 body anchors added at v1.8; ADR-impact sweeps would miss this BC | POLICY 4/19; TD-VSDD-060 |
| F-S2104-P8-004 | MEDIUM | story Test Plan/RG Plan vs 9 @tests; File Structure L230-231 | 2 of 9 tests unregistered (no T-ID/RG row); F-P4-002 gate AC-orphaned (AC-007(d) dispatch-scoped is vacuous for awareness-only surfaces) | POLICY 14/17/8; TD-VSDD-060 |
| F-S2104-P8-005 | MEDIUM | CHANGELOG L27-31 | devops-engineer listed among caller-side "dispatch surfaces" — collapses the caller-PRIMARY vs executor-defense-in-depth distinction ADR-031 v1.12 establishes; AC-008/Precondition 3/T-007 absent from the entry; step-d5 omitted | POLICY 4; story Task 11 |
| F-S2104-P8-006 | MEDIUM | CHANGELOG L26 | Live v1.7 pin on current-normative content; survivor of the a27febe9 sweep; 7th pin-class recurrence | TD-VSDD-091; POLICY 8 |
| F-S2104-P8-007 | MEDIUM | red-gate-log §T-007 L245 | Asserts "with mutant-proof" but records no vector/target/RED-GREEN observations (contrast §T-006's recorded proof); F-P7-002 mutant stdout also unrecorded — unverifiable by the next fresh-context pass | POLICY 15; TD-VSDD-059 |
| F-S2104-P8-008 | LOW | bats:906 | Stale "three-way" count label in T-005 harness message; only surviving count-label in the suite after the "sweep 0" attestation | POLICY 4; TD-VSDD-060 |
| F-S2104-P8-009 | LOW | fixtures README:20 | "the six tests" definite-article suite-inventory claim vs 9 tests; F-P6-010 class one increment later | POLICY 4/14 |

---

## Observations (NOT findings)

zero shadow .factory/ content in worktree at review (Invariant 5 clean); POLICY 21 clean; AC-001/AC-002 fully satisfied; spec-side coherence at BC v1.9/ADR v1.12 genuinely converged; [ -L ]-before-find ordering gate resolves correctly; precedence-winning playbook carries fully-qualified count-free mandate; volatile "test 9" ordinal on story side noted (name-anchor is a comment; F-S2104-P4-003 ID is in the @test name — recommend dropping ordinal); BC VP placeholders TBD blocked on VP authoring pass — not filed; BC §Architecture Anchors "to be amended" correct while lifecycle_status: draft pre-merge; NOT re-reported: bin/ awk sites, playbook-precedence conflict, operator-cache inertness, identity-preflight design, BC changelog row order, hardcoded harness steps mitigation, RG-004a history metadata.

---

## Per-Pass-7 Verification

F-P7-001 PARTIAL(→P8-001/002/007); F-P7-002 CONFIRMED-CLOSED (independently corroborated: regex matches canonical quoted form; zero matches on 6 gated surfaces); F-P7-003 CONFIRMED-CLOSED; F-P7-004 CONFIRMED-CLOSED (sweep 0; remaining repo bcs: hits legitimately document legacy field); F-P7-005 CONFIRMED-CLOSED; F-P7-006 PARTIAL(→P8-008); F-P7-007 CONFIRMED-CLOSED; F-P7-008 CONFIRMED-CLOSED; F-P7-009 CONFIRMED-CLOSED bats/README scope (CHANGELOG survivor raised fresh as P8-006). Tally 6 CONFIRMED-CLOSED, 3 PARTIAL, zero paper-fixes, zero false closures. Novelty 0.58; verdict FINDINGS_REMAIN; meta: record axis produces findings because fixes are scoped to cited lines; "a record-axis sweep bounded by defect class rather than cited line is the intervention this trajectory is asking for."

---

## Fix Mapping

| Finding ID | Fix leg | Commit SHA | Disposition |
|------------|---------|------------|-------------|
| F-S2104-P8-001 | red-gate-log.md v1.5→v1.6: §Bats Tests table T-001/T-002/T-003 RG cells corrected to RG-001/RG-002/RG-003; † footnote extended (state-manager this burst) | D-904 burst | FIXED |
| F-S2104-P8-002 | red-gate-log.md v1.5→v1.6: NEW T-008/T-009 attestation addenda (state-manager this burst) | D-904 burst | FIXED |
| F-S2104-P8-003 | BC-6.26.001 v1.9→v1.10: ADR Reference traceability row added (+ sibling BC-6.27.001 v1.3→v1.4 class sweep) (product-owner) | 116df361 | FIXED |
| F-S2104-P8-004 | story v1.11→v1.12: T-008/RG-008 + T-009/RG-009 registered; AC-009 awareness-surface anchor added; AC-007(d) awareness-note; BC v1.10 pins (story-writer) | 4478a5a4 | FIXED |
| F-S2104-P8-005 | CHANGELOG v1.5→v1.6: devops-engineer moved out of dispatch-surface list; AC-008/Precondition-3 sentence + T-007 + step-d5 mentions added (implementer) | 9d896bf5 | FIXED |
| F-S2104-P8-006 | CHANGELOG v1.5→v1.6: versionless pin (entry sweep 0) (implementer) | 9d896bf5 | FIXED |
| F-S2104-P8-007 | red-gate-log.md v1.5→v1.6: T-007 mutant-proof recorded verbatim (state-manager this burst) | D-904 burst | FIXED |
| F-S2104-P8-008 | bats: "three-way" count label removed from T-005 harness message (test-writer) | 82aa8883 | FIXED |
| F-S2104-P8-009 | fixtures README: "the six tests" corrected to "the nine tests" (test-writer) | 82aa8883 | FIXED |
