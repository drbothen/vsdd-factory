---
story: S-21.02
title: "post-rebase diff-integrity gate: detect and surface silent production-code drops before force-push-with-lease"
version: "1.6"
bc_version: "BC-5.44.001 v1.4"
evidence_produced: "2026-07-24"
produced_by: demo-recorder
method: scripted-terminal-capture
worktree_head: "8abf24e29d290060c4e8546c35708dfafc198977"
---

# S-21.02 Per-AC Demo Evidence Report

**Story:** S-21.02 — post-rebase diff-integrity gate: detect and surface silent production-code drops before force-push-with-lease
**Epic:** E-21 — Factory State Data-Loss Hardening
**Story version:** v1.6 (5 ACs)
**BC:** BC-5.44.001 v1.4
**ACs covered:** AC-001 through AC-005 (all 5)
**Method note:** This is a CLI/skill-doc artifact. VHS is not installed; evidence uses scripted
terminal captures (grep output logs + bats run output). This note is included per
the demo-recorder instruction ("note which").

---

## Evidence Artifact Index

| Artifact file | Contents | ACs covered |
|---------------|----------|-------------|
| `ac-001-gate-grep.txt` | grep captures from devops-engineer.md: token presence (range-diff + UnverifiedNetNegativeDelta) + section ordering (rebase → gate → force-with-lease) | AC-001 |
| `ac-002-step-f-grep.txt` | grep captures from step-f-pr-lifecycle.md: gate reference + Role ownership block + range-diff token | AC-002 |
| `ac-003-ac005-bats-run.txt` | Full bats run (5/5 ok); T-001..T-005 with test-to-AC mapping and edge coverage labels | AC-003, AC-004, AC-005 |

---

## AC Coverage Table

| AC | Requirement | Artifact(s) | Capture command | Status |
|----|-------------|-------------|-----------------|--------|
| AC-001 | `devops-engineer.md §Inter-Wave Rebase` contains mandatory post-rebase diff-integrity gate between `git rebase origin/develop` and `git push --force-with-lease`: (a) `git range-diff` as primary detector; (b) `--stat` fallback; (c) `UnverifiedNetNegativeDelta` STOP signal | `ac-001-gate-grep.txt` | `grep -n 'range-diff\|UnverifiedNetNegativeDelta' plugins/vsdd-factory/agents/devops-engineer.md`; `grep -n 'rebase origin/develop\|force-with-lease\|range-diff\|UnverifiedNetNegativeDelta\|Inter-Wave Rebase' plugins/vsdd-factory/agents/devops-engineer.md` | PASS |
| AC-002 | `step-f-pr-lifecycle.md` references the post-rebase diff-integrity gate as required when rebase occurs before force-push-with-lease | `ac-002-step-f-grep.txt` | `grep -n 'role\|Role\|devops-engineer\|diff-integrity\|range-diff\|force-with-lease' plugins/vsdd-factory/skills/deliver-story/steps/step-f-pr-lifecycle.md` | PASS |
| AC-003 | Bats fixture: unverified net-negative delta in sibling-touched file → gate halts; `git push --force-with-lease` NOT invoked; `UnverifiedNetNegativeDelta` in output | `ac-003-ac005-bats-run.txt` | `bats plugins/vsdd-factory/tests/post-rebase-diff-integrity-gate.bats` (T-001: ok 1) | PASS |
| AC-004 | Bats fixture: no sibling file overlap → gate passes; force-push proceeds (PC3 path) | `ac-003-ac005-bats-run.txt` | `bats plugins/vsdd-factory/tests/post-rebase-diff-integrity-gate.bats` (T-002: ok 2) | PASS |
| AC-005 | Bats fixture: confirmed intentional removal → gate passes; force-push proceeds (PC1 path) | `ac-003-ac005-bats-run.txt` | `bats plugins/vsdd-factory/tests/post-rebase-diff-integrity-gate.bats` (T-003: ok 3) | PASS |

---

## Test Execution Summary

### Bats suite

```
bats plugins/vsdd-factory/tests/post-rebase-diff-integrity-gate.bats
1..5
ok 1 T-001 S-21.02 AC-003: gate halts on unverified net-negative delta in sibling-touched file
ok 2 T-002 S-21.02 AC-004: gate passes — no sibling file overlap (PC3)
ok 3 T-003 S-21.02 AC-005: gate passes — confirmed intentional removal (PC1)
ok 4 T-004 S-21.02 PC4/EC-006: gate passes trivially — no sibling commits since branch creation
ok 5 T-005 S-21.02 EC-005: detector failure and merge-base failure escalate — push never invoked blind
EXIT: 0
```

T-001..T-003 cover AC-003/AC-004/AC-005 respectively.
T-004 (PC4/EC-006) and T-005 (EC-005) are additive edge rows, now enumerated in the v1.6
Test Plan table; labeled as additive edge coverage throughout this report.

### Grep captures (AC-001)

- `range-diff` appears at lines 230, 245, 253, 255, 264, 301 of `devops-engineer.md`
- `UnverifiedNetNegativeDelta` appears at line 333
- Ordering confirmed: `git rebase origin/develop` (line 231) → gate (lines 243-333) → `git push --force-with-lease` (line 347)

### Grep captures (AC-002)

- `diff-integrity gate` named at lines 22, 25 of `step-f-pr-lifecycle.md`
- Role ownership block at lines 20-22: names `devops-engineer.md §Inter-Wave Rebase` as required source
- `range-diff` cited at line 30 as primary detector
- Gate ordering constraint explicit at line 27: "gate MUST run before force-push; it cannot run after"

---

## POLICY 10 Compliance

All artifacts committed to the feature branch (`feature/S-21.02-post-rebase-diff-integrity-gate`)
under the story-scoped subfolder `docs/demo-evidence/S-21.02/` per POLICY 10.
No flat files placed at `docs/demo-evidence/*.md`.
