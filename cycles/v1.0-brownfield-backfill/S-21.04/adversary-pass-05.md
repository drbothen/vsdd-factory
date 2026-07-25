---
pass: 5
verdict: NOT-CLEAN
reviewed_head: 72e63769
novelty: 0.61
previous_review: "cycles/v1.0-brownfield-backfill/S-21.04/adversary-pass-04.md"
---

## Summary

Pass-5 adversarial review of S-21.04 implementation. 11 findings (B0 / H4 / M4 / L3). Novelty 0.61. Trajectory 14→18→17→12→11. Streak: 0/3.

---

## Part A — Findings

### Finding Table

| ID | Severity | Location | Summary | BCs / Policies |
|----|----------|----------|---------|----------------|
| F-S2104-P5-001 | HIGH | ADR-031 §INV-E21-004 + §Decision 4 PC2 | Anchored ADR still specified the pre-v1.6 find-first three-case protocol; no existence pre-test, no non-directory branch; contradicted every caller surface it governs; on the BC-named symlink-to-file vector the ADR model returns empty find output → proceed — the opposite of BC PC2b | BC v1.6 PC2/EC-008; POLICY 8/19; TD-VSDD-060 |
| F-S2104-P5-002 | HIGH | skills/deliver-story/SKILL.md:114-117; workflows/phases/per-story-delivery.md:193-197 | Retired -d framing at both AC-007 primary surfaces: "absent-dir → proceed" + non-directory branch omitted from the enumeration | BC v1.6 PC2a(a)/EC-008; AC-002(a); TD-VSDD-060 |
| F-S2104-P5-003 | HIGH | CHANGELOG.md:11; story bats:327,545,656,736 | 4th recurrence of BC-version-pin propagation gap (v1.5 pins vs delivered v1.6); CHANGELOG also omitted -e predicate, non-directory branch, T-005, devops-engineer surface | POLICY 8; TD-VSDD-060 |
| F-S2104-P5-004 | HIGH | story:76 §Narrative | 3 retired phrase classes in one paragraph (find-first ordering; no non-directory branch; "after an empty result") while v1.7 claimed propagation complete | POLICY 8 |
| F-S2104-P5-005 | MEDIUM | worktree-identity-preflight.bats:103,183 | Pass-4 F-004 stale-snapshot residue at 2 in-file sites incl. test (e)'s own header | BC Inv5; TD-VSDD-059/060 |
| F-S2104-P5-006 | MEDIUM | workflows/phases/per-story-delivery.md:194 | 6th F-P4-005 site — the precedence-winning playbook retained a non-resolving relative §G.1 path | POLICY 4 |
| F-S2104-P5-007 | MEDIUM | story bats:479,487,495,506 | 4 primary-surface gates satisfiable by the bare word "preflight" — the mechanism that let F-005/F-006 residues survive gated | POLICY 13/15 |
| F-S2104-P5-008 | MEDIUM | story AC-007 vs devops-engineer File Structure row | Delivered executor-side verify-before-execute obligation had no AC stating it (AC-007 scopes caller-side-before-dispatch) | POLICY 4 |
| F-S2104-P5-009 | LOW | step-g-cleanup.md:47 | PC2b header/condition still find-only; BC v1.6 widened PC2b to the non-directory inode | BC v1.6 PC2b |
| F-S2104-P5-010 | LOW | step-g-cleanup.md:62 | PC2c kept "for a non-path-absent reason"; BC v1.6 fires on any non-zero exit | BC v1.6 PC2c |
| F-S2104-P5-011 | LOW | BC v1.6 EC-008/T-6; story EC-007/T-005 | Symlink-to-DIRECTORY escaped all four cases (test -d follows symlinks; find doesn't descend) → clean preflight then rm -rf destroys the link target | BC PC2 fail-closed claim |

---

## Observations (NOT findings)

- re-confirmed unchanged and NOT re-reported per dispatch: 6 bin/ space-unsafe awk sites (pending human anchor); dual playbook-precedence conflict (open drift item); operator-cache inertness until rc.24. Harness PC2b echo "empty result" cosmetic drift (fixed at 93ec340a). POLICY 21 clean. POLICY 1/6/7/16 spot-checks clean. factory-worktree-health/factory-health remove sites correctly out of AC-007 scope (BC-6.27.001 domain).

---

## Per-Pass-4 Verification

F-P4-001 PARTIAL (→P5-002/001/004); F-P4-002 CONFIRMED-CLOSED; F-P4-003 PARTIAL (→P5-008/003); F-P4-004 PARTIAL (→P5-005); F-P4-005 PARTIAL (→P5-006/007); F-P4-006 CONFIRMED-CLOSED; F-P4-007 PARTIAL (→P5-002/009/010/011); F-P4-008 CONFIRMED-CLOSED (19-artifact parity re-verified); F-P4-009 CONFIRMED-CLOSED; F-P4-010 CONFIRMED-CLOSED at §G.1 (story surface →P5-004c); F-P4-011 PARTIAL (→P5-003); F-P4-012 CONFIRMED-CLOSED. Tally: 6 CONFIRMED-CLOSED, 6 PARTIAL, zero paper-fixes, zero false closures. Novelty 0.61; verdict FINDINGS_REMAIN; structural note: "the sweep boundary is moving one hop per pass rather than closing."

---

## Incident Note (caught pre-adversary, recorded per D-901 dispatch)

Pass-5 test-writer gates at bats 503/537/545 used ERE `\|` (literal pipe — POLICY 13 defect); implementer initially satisfied them by contorting doc text with pipe-bearing "Chain lines" (TD-VSDD-059 teaching-to-a-buggy-test class); orchestrator caught it from the implementer's own disclosure note, routed gate repair (94a627ee) + doc un-contortion (b9c6c784); all gates now pass on natural text.

---

## Fix Mapping

| Finding ID | Fix leg | Commit SHA | Disposition |
|------------|---------|------------|-------------|
| F-S2104-P5-001 | ADR-031 v1.10→v1.11 (architect) | 7862b490 | FIXED |
| F-S2104-P5-002 | enumeration rewrite (implementer); gates 93ec340a | 43ea70ba + 93ec340a | FIXED |
| F-S2104-P5-003 | CHANGELOG v1.7 semantics + versionless pin (implementer); bats legs (test-writer) | 2b95b246 + 93ec340a | FIXED |
| F-S2104-P5-004 | story Narrative 4-step rewrite (story-writer) | 04aa9ff3 | FIXED |
| F-S2104-P5-005 | identity-bats residue rewrite (test-writer) | 93ec340a | FIXED |
| F-S2104-P5-006 | qualified path (implementer); gate 93ec340a | 43ea70ba + 93ec340a | FIXED |
| F-S2104-P5-007 | 4 gates de-weakened + enumeration gates + qualified-path gate (test-writer); ERE repair (test-writer) | 93ec340a + 94a627ee | FIXED |
| F-S2104-P5-008 | AC-008 executor-side obligation (story-writer) | 04aa9ff3 | FIXED |
| F-S2104-P5-009 | BC v1.7 PC2b clause precision (product-owner + implementer) | 92415d80 + 4833a642 | FIXED |
| F-S2104-P5-010 | BC v1.7 PC2c clause precision (product-owner + implementer) | 92415d80 + 4833a642 | FIXED |
| F-S2104-P5-011 | BC v1.7 symlink-at-path→PC2b (product-owner + implementer + test-writer T-006) | 92415d80 + 4833a642 + 93ec340a | FIXED |
