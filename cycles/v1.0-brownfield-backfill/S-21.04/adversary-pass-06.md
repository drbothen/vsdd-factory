---
pass: 6
verdict: NOT-CLEAN
reviewed_head: b9c6c784
novelty: 0.72
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-05.md"
---

## Summary

Pass-6 adversarial review of S-21.04 implementation. 11 findings + 1 deferred-but-fixed (B0 / H6 / M4 / L1). Novelty 0.72. Trajectory 14→18→17→12→11→11. Streak: 0/3.

---

## Part A — Findings

### Finding Table

| ID | Severity | Location | Summary | BCs / Policies |
|----|----------|----------|---------|----------------|
| F-S2104-P6-001 | HIGH | step-g-cleanup.md:40; CHANGELOG.md:21 | Symlink→PC2b branch anchored to BC-6.26.001 v1.7 EC-009 — an edge case that does not exist (BC defines EC-001..EC-008); contradicts :49 citing EC-008 for the sibling branch | POLICY 4/19; TD-VSDD-060/091 |
| F-S2104-P6-002 | HIGH | 7 sites: step-g-cleanup.md:96; agents/orchestrator/per-story-delivery.md:47; code-delivery/SKILL.md:203; worktree-manage/SKILL.md:84; fix-pr-delivery/SKILL.md:147; code-delivery.lobster:445; greenfield.lobster:802 | Retired "three-branch protocol" enumeration survives at 7 surfaces incl. §G.1 itself (self-contradiction L18 "Four cases:" vs L96); an executor reconstructing 3 branches from PC labels omits the [ -L ] guard — the F-P5-011 data-loss vector | BC v1.7 PC2; AC-007(b)/(d); TD-VSDD-060 |
| F-S2104-P6-003 | HIGH | story bats:965-967 + :259-294 | The [ -L ] guard has no load-bearing gate: T-006's alternation satisfied by the PC2b header line alone; harness steps 1-3 hardcoded; deleting §G.1 L31-40 undetectable; no [ -L ]-before-find ordering gate | TD-VSDD-059; POLICY 13/15 |
| F-S2104-P6-004 | HIGH | red-gate-log.md v1.3 | Zero Red Gate attestation for T-005, T-006, RG-005; Summary still "3 bats"; D-895 erratum asserts "RG-004/005 do not exist" now contradicted by story v1.8 | POLICY 14/17; TD-VSDD-059 |
| F-S2104-P6-005 | HIGH | devops-engineer.md:358-362 vs ADR-031:369-381; BC §Preconditions | AC-008 executor mandate contradicted ADR-031 §Rationale F-P2-007 ("NOT co-located"); ADR never acknowledged the layer; BC had no executor precondition | POLICY 19; Authority §12; TD-VSDD-060 |
| F-S2104-P6-006 | HIGH | story bats ×17; CHANGELOG:20,21; step-g:40 | 20 live versioned BC pins; v1.6 pins attribute EC-008 semantics to the superseded clause; 5th recurrence of the pin class | TD-VSDD-091/060; POLICY 8 |
| F-S2104-P6-007 | MEDIUM | story bats:1027 | Anti-inline-find regression gate inert against the mandated trailing-slash form (regex required whitespace after .factory) — false-green since BC v1.7 | POLICY 13/15 |
| F-S2104-P6-008 | MEDIUM | step-g-cleanup.md:20-56 | find invocation without restated precondition; BLOCKED branches nested inside the "teardown authorized" PC2a bullet — prose ordering, not stated condition, kept symlinks out of find | BC v1.7 PC2 step 3; POLICY 4 |
| F-S2104-P6-009 | MEDIUM | story:100,212,91,92 | Test-inventory drift: "T-001..T-004" claims vs T-001..T-006+3 gates shipped; AC-007 Gate said manual though automated; AC-008 gate had no T-ID | POLICY 14/17 |
| F-S2104-P6-010 | MEDIUM | fixtures README:20,129-130 | "Five fixture configurations ... five tests" vs six; EC map filed symlink vector as non-EC "PC2b-symlink" contradicting story/BC EC-008 | POLICY 4/14 |
| F-S2104-P6-011 | LOW | story bats:50,935,966 | T-006 provenance mis-cited to F-S2104-P5-007 (gate-weakening finding); traces to F-S2104-P5-011/BC T-7/RG-005 | POLICY 4 LOW |
| F-S2104-P6-D01 | deferred→fixed-in-scope | agents/adversary.md:40 | Perimeter-1 scoped to "bcs:" frontmatter — no story uses that field; real field behavioral_contracts: | system-level; fixed e6e6b26e |

---

## Observations (NOT findings)

- bats:407 comment quoted retired phrasing (fixed 772096f4); identity-preflight whole-file greps pre-existing design out of scope; POLICY 21/1/6/7/16 clean; factory-health remove sites out of AC-007 scope (BC-6.27.001 domain); BC changelog 1.0-row ordering pre-existing non-load-bearing; NOT re-reported: bin/ awk sites, playbook-precedence conflict, operator-cache inertness.

---

## Per-Pass-5 Verification

F-P5-001 CONFIRMED-CLOSED; F-P5-002 PARTIAL(→P6-002); F-P5-003 PARTIAL(→P6-001/006); F-P5-004 CONFIRMED-CLOSED; F-P5-005 CONFIRMED-CLOSED; F-P5-006 CONFIRMED-CLOSED; F-P5-007 PARTIAL(→P6-003); F-P5-008 PARTIAL(→P6-005/009); F-P5-009 CONFIRMED-CLOSED; F-P5-010 CONFIRMED-CLOSED; F-P5-011 PARTIAL(→P6-003); ERE incident CONFIRMED-CLOSED. Novelty 0.72; verdict FINDINGS_REMAIN; meta-note: "the newly-surfaced defect classes are now caused by the v1.7 amendment rather than left over from v1.5 — evidence the spec side has converged and the propagation/gating side has not."

---

## Fix Mapping

| Finding ID | Fix leg | Commit SHA | Disposition |
|------------|---------|------------|-------------|
| F-S2104-P6-001 | phantom EC-009→EC-008 (implementer) | 97cf24e0 | FIXED |
| F-S2104-P6-002 | class-death: branch-count phrases → count-free "§G.1 discrimination-chain protocol" at 9 surfaces + CHANGELOG; whole-repo sweep stdout empty outside §G.1 (implementer) | 0614aadb | FIXED |
| F-S2104-P6-003 | load-bearing literal [ -L ] gate + awk ordering assertion + MUTANT SELF-CHECK: scratch deletion of §G.1 L31-40 → T-006 RED, proven (test-writer) | 772096f4 | FIXED |
| F-S2104-P6-004 | red-gate-log.md v1.3→v1.4: T-005/T-006/RG-005 attestation addenda + Summary 9-test + RG-reconciliation note (state-manager this burst) | D-902 burst | FIXED |
| F-S2104-P6-005 | ADR-031 v1.11→v1.12 (architect): F-005 ADJUDICATION Option A: AC-008 upheld; F-P2-007 qualified (verification-and-delegation ≠ co-location; caller-side stays PRIMARY); BC-6.26.001 v1.7→v1.8 (product-owner): Precondition 3 executor-side verification + Invariant 2 extension | 24c25b9c + 6bf3185c | FIXED |
| F-S2104-P6-006 | CHANGELOG pin versionless (implementer); bats legs 14 pins removed 3 justified historical survivors (test-writer) | a27febe9 + 772096f4 | FIXED |
| F-S2104-P6-007 | trailing-slash-aware regex, mutant-checked (test-writer) | 772096f4 | FIXED |
| F-S2104-P6-008 | §G.1 restructure per BC layout: lift symlink/non-dir BLOCKED branches out of Sub-case (a) (implementer) | 22cce8c5 | FIXED |
| F-S2104-P6-009 | test-inventory sync: Architecture Mapping + File Structure T-001..T-006 + 3 gates; AC-007 Gate → bats+manual-fallback; NEW T-007 ↔ AC-008 (story-writer) | 47dda220 | FIXED |
| F-S2104-P6-010 | README six/T-006 + EC map (test-writer) | 772096f4 | FIXED |
| F-S2104-P6-011 | provenance cites (test-writer) | 772096f4 | FIXED |
| F-S2104-P6-D01 | adversary.md bcs:→behavioral_contracts: — deferred-listed but fixed in-scope as 1-word mechanical correction in a declared deliverable file (implementer) | e6e6b26e | FIXED (in-scope) |
