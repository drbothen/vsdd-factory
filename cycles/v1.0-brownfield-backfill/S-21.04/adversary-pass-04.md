---
pass: 4
verdict: NOT-CLEAN
reviewed_head: b44442b2
novelty: 0.71
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-03.md"
---

> **Rewritten at D-900 — D-899 persist contained non-verbatim reconstructed content (second occurrence of the D-897 class); orchestrator-certified verbatim below.**

## Summary

Pass-4 adversarial review of S-21.04 implementation. 12 findings (B0 / H6 / M5 / L1). Novelty 0.71. Trajectory 14→18→14→12. Streak: 0/3.

---

## Part A — Findings

### Finding Table

| ID | Severity | Location | Summary | BCs / Policies |
|----|----------|----------|---------|----------------|
| F-S2104-P4-001 | HIGH | agents/orchestrator/per-story-delivery.md:46-48; worktree-manage:82-84; code-delivery:202-204; fix-pr-delivery:146-148; code-delivery.lobster:444-446; greenfield.lobster:801-804 | Three-branch text inlines find as FIRST action with absent-dir/find-error as unordered siblings — on a clean worktree the same observable (exit non-zero + ENOENT) satisfies two contradictory branches; inverts §G.1 absent-dir-first ordering; composes with F-005 where the §G.1 xref doesn't resolve | BC PC2 steps 1-2, PC2a(a), EC-005; INV-E21-004 |
| F-S2104-P4-002 | HIGH | story File Structure rows 16-17 vs agents/adversary.md + skills/adversarial-review/SKILL.md | Rows declare "mandate §G.1 preflight awareness (BC-6.26.001 Invariant 5)" — neither file contained §G.1/BC-6.26.001; declared-but-undelivered clause; covered by no AC | POLICY 14/17; TD-VSDD-059 |
| F-S2104-P4-003 | HIGH | agents/devops-engineer.md:356-361 | Executor agent has unqualified "git worktree remove" with zero preflight/§G.1/BC references; AC-007(d) category excluded agent files | BC Inv2; INV-E21-004; TD-VSDD-060 |
| F-S2104-P4-004 | HIGH | _shared-context.md:60; worktree-identity-preflight.bats:6,179,192-194; step-d5-adversary-convergence.md:79,86 | Retracted stale-snapshot premise survives at 3 sites, 2 in-diff; _shared-context lead bolded sentence self-contradicts corrected model | BC Inv5; INV-E21-002 |
| F-S2104-P4-005 | HIGH | worktree-manage:83; code-delivery:203 (+ bare filenames at fix-pr-delivery:147, worktree-protocol:52,76) | Relative path `steps/step-g-cleanup.md §G.1` doesn't resolve from those directories; fallback is the ambiguous inline text | POLICY 4 |
| F-S2104-P4-006 | HIGH | red-gate-log.md:149 | F-P3-009 fixed at line 155 only; line 149 retained the fabricated PREFLIGHT BLOCKED (PC2b message) attribution for PC2c | TD-VSDD-059 |
| F-S2104-P4-007 | MEDIUM | step-g-cleanup.md:25 vs bats:243 | Doc predicate -d vs harness -e diverge on .factory-as-regular-file; gate alternation accepted either | BC PC2 step 1; POLICY 15 |
| F-S2104-P4-008 | MEDIUM | worktree-identity-preflight.bats vs story | File modified in diff but undeclared in story (recurrence of F-P3-013) | POLICY 14/17 |
| F-S2104-P4-009 | MEDIUM | story bats:440-473 | Only 4 of 10 mandated surfaces had doc-parity regression gates | POLICY 15 |
| F-S2104-P4-010 | MEDIUM | step-g-cleanup.md:69,75 | Normative Dispatch gate still said "after empty preflight result" — phrasing retired at all derivative surfaces | BC PC2a/PC2c |
| F-S2104-P4-011 | MEDIUM | CHANGELOG.md:11-26 | Entry omitted adversary reporting-semantics change + test-(e) re-anchor | Story Task 11 |
| F-S2104-P4-012 | LOW | STORY-INDEX.md:727 | Row annotation "AC-007 fail-closed" mis-descriptive (fail-closed is AC-006/PC2c) | POLICY 4 |

---

## Observations (NOT findings)

- **[deferred: system-level → phase-5]** 6 space-unsafe awk-$2 porcelain-parse sites in bin/ (factory-query:73, emit-event:70, factory-sla:53, factory-report:53, factory-dashboard:73, factory-replay:43; safe pattern at factory-cas-push.sh:78) — PENDING HUMAN story-anchor decision
- **[process-gap]** dual precedence claims over Step 8 teardown gate (_shared-context.md:196 names agents/orchestrator/per-story-delivery.md authoritative vs workflows/phases/per-story-delivery.md self-declaring winner) — OPEN drift item for human/architect adjudication
- delegation-form surfaces are the safe pattern (do not inline three-branch into them) — INFO
- operator-cache inertness until rc.24 — INFO
- changelog row-order non-monotonic — INFO

---

## Fix Mapping

| Finding ID | Fix leg | Commit SHA | Disposition |
|------------|---------|------------|-------------|
| F-S2104-P4-001 | delegation-form, implementer | a317fd77 | FIXED |
| F-S2104-P4-002 | awareness clauses, implementer; gated in bats | 4265c96c + 60f0d2d6 | FIXED |
| F-S2104-P4-003 | executor mandate, implementer + story v1.7 8135b2e5 (AC-007(d) category + row) | 0c0922e1 + 8135b2e5 | FIXED |
| F-S2104-P4-004 | docs (implementer) + bats header residue (test-writer) + story v1.7 row (step-d5) | 35cabb51 + 60f0d2d6 + 8135b2e5 | FIXED |
| F-S2104-P4-005 | fully-qualified refs, implementer | a317fd77 | FIXED |
| F-S2104-P4-006 | red-gate-log v1.3 line-149 correction | ba9ccad4 (D-899) | FIXED |
| F-S2104-P4-007 | BC v1.6 (product-owner) + doc predicate (implementer) + tightened gate + T-005 (test-writer) | fcfce450 + 73c2bade + 60f0d2d6 | FIXED |
| F-S2104-P4-008 | story v1.7 row added | 8135b2e5 | FIXED |
| F-S2104-P4-009 | 6-surface gates, test-writer | 60f0d2d6 | FIXED |
| F-S2104-P4-010 | Normative Dispatch gate phrasing retired, implementer | 73c2bade | FIXED |
| F-S2104-P4-011 | CHANGELOG entry updated, implementer | 72e63769 | FIXED |
| F-S2104-P4-012 | STORY-INDEX label correction | ba9ccad4 (D-899) | FIXED |
