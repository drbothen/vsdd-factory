---
pass: 7
verdict: NOT-CLEAN
reviewed_head: 3c3788d7
novelty: 0.68
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-06.md"
---

## Summary

Pass-7 adversarial review of S-21.04 implementation. 9 findings (B0 / H4 / M3 / L2). Novelty 0.68. Trajectory 14→18→17→12→11→11→9. Streak: 0/3.

---

## Part A — Findings

### Finding Table

| ID | Severity | Location | Summary | BCs / Policies |
|----|----------|----------|---------|----------------|
| F-S2104-P7-001 | HIGH | red-gate-log.md §T-005 addendum L186-204, §T-006 addendum L212-234, §Traces L94-95, Summary L38 | Pass-6 F-004 attestation addenda mis-attribute ACs (T-005/T-006 → "AC-007"; story says AC-002), fabricate non-existent RG ID "RG-004a" (4 sites), misdescribe T-005's behavior (attests a path-absent/PC2a-proceed test; actual fixture is a regular FILE at the path asserting PC2b BLOCKED), and omit T-007/AC-008 despite Summary claiming "T-001..T-007 + 2 gates"; third fabrication incident in this file (D-895, D-897, D-902) — pattern | POLICY 14/17; POLICY 1; TD-VSDD-059 |
| F-S2104-P7-002 | HIGH | story bats:1053 | Anti-inline-find gate STILL inert against the canonical QUOTED trailing-slash form — P6-007's fix added /? but not quote tolerance; [[:space:]] cannot match the closing quote; gate passes vacuously; second consecutive false-green on a gate protecting an rm -rf path | POLICY 13/15; TD-VSDD-059 |
| F-S2104-P7-003 | HIGH | story bats:1154; story AC-008 Gate + T-007 | The gate designated T-007 for AC-008/Precondition 3 is a bare-token alternation satisfied by any incidental BC-6.26.001 mention; neither AC-008 obligation asserted; mapping-only closure for a newly-authored AC | TD-VSDD-059; POLICY 15; BC PC 3 |
| F-S2104-P7-004 | HIGH | adversary.md:40 vs :276-279 | F-P6-D01 fixed bcs: at one site; four identical occurrences survive in the same file's Story Frontmatter-Body Coherence Axis, contradicting L40 and making the axis a no-op | TD-VSDD-060; POLICY 4 |
| F-S2104-P7-005 | MEDIUM | red-gate-log.md:216 | T-006 addendum inverts the trailing-slash mechanism (claims trailing-slash find "would follow the link and report empty"; BC states the trailing slash FORCES traversal entry; the empty-output escape comes from find WITHOUT -H/-L) | POLICY 4; BC §Description step 3; story EC-008 |
| F-S2104-P7-006 | MEDIUM | step-g-cleanup.md:18 | Residual "Four cases:" matches neither its own five blocks nor BC's "Three cases:"; L18 was named in F-P6-002 but the sweep was scoped "outside §G.1" | TD-VSDD-060; POLICY 4; BC PC2 |
| F-S2104-P7-007 | MEDIUM | devops-engineer.md:358 | Executor mandate scoped "After story PR merges" — narrower than unconditional Precondition 3/AC-008; Story Split Recovery removes worktrees without a merge | BC PC 3 + Inv2; AC-008; POLICY 4 |
| F-S2104-P7-008 | LOW | devops-engineer.md:361 | Sole Precondition-3-instantiating surface cites only Invariant 2 | POLICY 5/8 |
| F-S2104-P7-009 | LOW (pending intent verification) | story bats ×16; README:90 | P6-006 closure attested "3 justified historical survivors"; 16 v1.x mentions present, 7 pinning CURRENT-normative behavior to v1.7 while BC is v1.8 | TD-VSDD-091; POLICY 15 attestation accuracy |

---

## Observations (NOT findings)

T-007/AC-008 anchored on ordinal "test 9" — volatile under insertion (name-anchor added in bats at 052620dc); step-g PC2b message template worded for file inventories vs harness case-specific wording — non-load-bearing; harness steps 1-3 hardcoded with doc-parity gates as coupling — mitigation accepted, cannot detect semantic reorder keeping literal+ordering; NOT re-reported: bin/ awk sites, playbook-precedence conflict, operator-cache inertness, identity-preflight whole-file-grep design, BC changelog 1.0-row ordering. Positive confirmations: POLICY 21 clean; AC-001 fully satisfied; AC-002 grep set present; worktree-protocol corrections landed; ADR v1.12 whole-file sweep attested.

---

## Per-Pass-6 Verification

F-P6-001 CONFIRMED-CLOSED; F-P6-002 PARTIAL(→P7-006); F-P6-003 CONFIRMED-CLOSED (mutant-proven); F-P6-004 PARTIAL(→P7-001/005); F-P6-005 CONFIRMED-CLOSED spec side (impl →P7-007/008); F-P6-006 PARTIAL(→P7-009); F-P6-007 PARTIAL(→P7-002); F-P6-008 CONFIRMED-CLOSED; F-P6-009 PARTIAL(→P7-001/003); F-P6-010 CONFIRMED-CLOSED; F-P6-011 CONFIRMED-CLOSED; F-P6-D01 PARTIAL(→P7-004). Novelty 0.68; verdict FINDINGS_REMAIN; meta: "attestation-vs-artifact divergence in closure records is itself becoming the recurring signal."

---

## Fix Mapping

| Finding ID | Fix leg | Commit SHA | Disposition |
|------------|---------|------------|-------------|
| F-S2104-P7-001 | red-gate-log.md v1.4→v1.5: attestation corrections verbatim-authored by orchestrator (state-manager this burst) | D-903 burst | FIXED |
| F-S2104-P7-002 | quote-tolerant find gate (mutant stdout: quoted canonical OLD NO-MATCH/NEW MATCH; unquoted both forms still match) (test-writer) | 052620dc | FIXED |
| F-S2104-P7-003 | obligation-asserting AC-008 gates (4 assertions: qualified-path, verify, PASS-result, not-evident-run-yourself; all mutant-proven RED on the reduction mutant) (test-writer) | 052620dc | FIXED |
| F-S2104-P7-004 | adversary.md bcs:→behavioral_contracts: ×4, sweep 0 (implementer) | a087ee7b | FIXED |
| F-S2104-P7-005 | red-gate-log.md v1.4→v1.5: T-006 trailing-slash mechanism corrected (state-manager this burst) | D-903 burst | FIXED |
| F-S2104-P7-006 | doc leg "Four cases:"→"Outcomes:", sweep 0 (implementer) | 6a3ab8d9 | FIXED |
| F-S2104-P7-007 | devops-engineer merge-qualifier removed — mandate unconditional (implementer) | eee4741a | FIXED |
| F-S2104-P7-008 | PC2+Precondition 3 cites added (implementer) | 3c3788d7 | FIXED |
| F-S2104-P7-009 | 7 converted; 10 bats + 2 README justified-historical survivors listed (test-writer) | 052620dc | FIXED |
