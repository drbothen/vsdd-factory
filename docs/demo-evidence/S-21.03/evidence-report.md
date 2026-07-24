---
story: S-21.03
title: "pr-manager trunk-assertion hardening: post-create baseRefName check and post-merge ancestry assertion"
version: "1.8"
bc_version: "BC-6.10.002 v1.5"
evidence_produced: "2026-07-24"
produced_by: demo-recorder
method: scripted-terminal-capture
worktree_head: "194e98fe89ac5042defad0e803cbd46b9ef2a7a9"
---

# S-21.03 Per-AC Demo Evidence Report

**Story:** S-21.03 — pr-manager trunk-assertion hardening: post-create baseRefName check and post-merge ancestry assertion
**Epic:** E-21 — Factory State Data-Loss Hardening
**Story version:** v1.8 (5 ACs + EC-007 additive coverage)
**BC:** BC-6.10.002 v1.5
**ACs covered:** AC-001 through AC-005 (all 5); T-006/T-007 labeled additive EC-007 coverage
**Method note:** This is a CLI/skill-doc artifact. VHS is not installed; evidence uses scripted
terminal captures (grep output logs + bats run output). This note is included per
the demo-recorder instruction ("note which").

---

## Evidence Artifact Index

| Artifact file | Contents | ACs covered |
|---------------|----------|-------------|
| `ac-001-002-step3-grep.txt` | grep captures from pr-manager.md §Step 3: section heading + Step 3-post-A bold heading + mandate tokens (baseRefName, BaseRefNameMismatch, does not equal, configured trunk, MUST NOT be merged, STEP_COMPLETE step=3) | AC-001, AC-002 |
| `ac-003-005-step8-post-a-grep.txt` | grep captures from pr-manager.md §Step 8-post-A: bold heading line numbers (ordering: Step 8-post-A line 327 < Step 8b line 404) + mandate tokens (merge-base --is-ancestor, MergeNotAncestorOfTrunk, non-zero exit, P0 DATA ERROR, MUST NOT be marked delivered, mergeCommit.oid, TrunkFetchFailed, Retry once, UNANSWERED, orphan-merge recovery) + Step 9 back-reference | AC-003, AC-004, AC-005 |
| `bats-run.txt` | Full bats run (7/7 ok); T-001..T-007 with test-to-AC mapping and EC-007 additive coverage labels | AC-001, AC-002, AC-003, AC-004, AC-005 |

---

## AC Coverage Table

| AC | Requirement | Artifact(s) | Capture command | Status |
|----|-------------|-------------|-----------------|--------|
| AC-001 | After `gh pr create --base <trunk>` completes, pr-manager MUST assert `baseRefName` equals configured trunk; hard-fail with `BaseRefNameMismatch` (actual + expected) if not; PR MUST NOT be merged | `ac-001-002-step3-grep.txt` | `grep -n '### Step 3:\|Step 3-post-A\|baseRefName\|BaseRefNameMismatch\|does not equal\|configured trunk\|MUST NOT be merged\|STEP_COMPLETE: step=3' plugins/vsdd-factory/agents/pr-manager.md` | PASS |
| AC-002 | When `gh pr view --json baseRefName` returns the correct trunk, assertion passes and step 7 (deliver-story) proceeds normally to merge phase | `ac-001-002-step3-grep.txt`, `bats-run.txt` | same grep (line 170: STEP_COMPLETE step=3 note=baseRefName assertion passed); bats T-002 ok | PASS |
| AC-003 | After `gh pr merge` completes MERGED, run `git fetch origin <trunk> && git merge-base --is-ancestor <merge_sha> origin/<trunk>`; if non-zero exit, raise P0 `MergeNotAncestorOfTrunk`; story MUST NOT be marked delivered | `ac-003-005-step8-post-a-grep.txt`, `bats-run.txt` | `grep -n '^\*\*Step 8-post-A\|^\*\*Step 8b\|merge-base --is-ancestor\|MergeNotAncestorOfTrunk\|non-zero exit\|P0 DATA ERROR\|MUST NOT be marked delivered\|...' plugins/vsdd-factory/agents/pr-manager.md`; bats T-003 ok | PASS |
| AC-004 | When `git merge-base --is-ancestor` returns exit 0, assertion passes and story may be marked delivered | `ac-003-005-step8-post-a-grep.txt`, `bats-run.txt` | same grep (lines 374, 401); bats T-004 ok | PASS |
| AC-005 | When `gh pr view --json mergeCommit` returns `null`, treat as `MergeNotAncestorOfTrunk` P0 error | `ac-003-005-step8-post-a-grep.txt`, `bats-run.txt` | same grep (lines 336, 340, 345); bats T-005 ok | PASS |

---

## Test Execution Summary

### Bats suite

```
bats plugins/vsdd-factory/tests/pr-manager-trunk-assertion.bats
1..7
ok 1 T-001 S-21.03 AC-001: BaseRefNameMismatch on wrong baseRefName — merge NOT invoked
ok 2 T-002 S-21.03 AC-002: baseRefName assertion passes on correct trunk — proceeds to merge
ok 3 T-003 S-21.03 AC-003: MergeNotAncestorOfTrunk when merge-base exits 1 — story NOT delivered
ok 4 T-004 S-21.03 AC-004: ancestry assertion passes on merge-base exit 0 — story delivered
ok 5 T-005 S-21.03 AC-005: null mergeCommit.oid treated as MergeNotAncestorOfTrunk
ok 6 T-006 S-21.03 EC-007: TrunkFetchFailed on fetch failure — MergeNotAncestorOfTrunk NOT raised; HALT; NOT delivered
ok 7 T-007 S-21.03 EC-007: TrunkFetchFailed retry-succeed — Step B reached; no HALT; story delivered
EXIT: 0
```

T-001..T-005 cover AC-001/AC-002/AC-003/AC-004/AC-005 respectively.
T-006 (EC-007 fetch-fail HALT) and T-007 (EC-007 retry-succeed) are additive edge rows
per the v1.8 Test Plan table; labeled as additive EC-007 coverage throughout this report.

### Grep captures (AC-001 / AC-002)

- `### Step 3: Create PR` section heading at line 134
- `Step 3-post-A` bold heading at line 145: post-create baseRefName assertion mandate
- `gh pr view <pr_number> --json baseRefName` command at line 150
- `baseRefName value equals the configured trunk` assertion at lines 153–154
- `does not equal the configured trunk` → hard-fail `BaseRefNameMismatch` at lines 154–155
- `HARD FAIL: BaseRefNameMismatch` error body at lines 158–159 (actual + expected trunk)
- `MUST NOT be merged` hard-fail consequence at line 164 (BC-6.10.002 PC2 Invariant 2)
- `STEP_COMPLETE: step=3 ... baseRefName assertion passed` happy-path note at line 170 (AC-002)

### Grep captures (AC-003 / AC-004 / AC-005)

- `**Step 8-post-A` bold heading at line 327: ancestry assertion mandate (BC-6.10.002 PC3)
- `**Step 8b` bold heading at line 404: branch-deletion step
- **Ordering confirmed:** Step 8-post-A (line 327) precedes Step 8b (line 404) — ancestry assertion mandate is before branch deletion per BC-6.10.002 PC3 ordering invariant
- `mergeCommit.oid` extraction at line 330; null guard at lines 336, 340, 345 (AC-005 / EC-006)
- `P0 DATA ERROR: MergeNotAncestorOfTrunk` null-SHA path at line 340
- `MUST NOT be marked delivered` for null SHA at line 345
- `git merge-base --is-ancestor` Step B command at line 374 (AC-003 / AC-004)
- `non-zero exit code` → `MergeNotAncestorOfTrunk` at lines 379, 383 (AC-003)
- `MUST NOT be marked delivered` hard-fail consequence at line 401 (BC-6.10.002 PC3 Invariant 2)
- `Step 9` back-reference to Step 8-post-A at lines 499–514 (Step 9 confirms ancestry gate)
- `TrunkFetchFailed`, `Retry once`, `UNANSWERED`, `orphan-merge recovery` at lines 358–369 (EC-007)

---

## POLICY 10 Compliance

All artifacts committed to the feature branch (`feature/S-21.03-pr-manager-trunk-assertion`)
under the story-scoped subfolder `docs/demo-evidence/S-21.03/` per POLICY 10.
No flat files placed at `docs/demo-evidence/*.md`.
