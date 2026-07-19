---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-07-19T00:00:00Z
phase: brownfield-backfill
cycle: v1.0-brownfield-backfill
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/e-21-arch-delta-analysis.md
  - plugins/vsdd-factory/agents/pr-manager.md
  - plugins/vsdd-factory/skills/deliver-story/steps/step-f-pr-lifecycle.md
input-hash: "TBD"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: null
subsystem: "SS-05"
capability: "TBD — E-21 CAP pending ARCH-INDEX registration by architect"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.44.001
section: "5.44"
last_amended: "2026-07-19 (v1.0) — Initial authoring (product-owner; E-21 factory-state data-loss hardening; issue #365). Post-rebase diff-integrity gate: required orchestrator/pr-manager step after any git rebase on a feature branch touching sibling-story files; net-negative-delta detection before force-push. lifecycle_status: draft (POL-14 auto-promotion on implementing PR merge)."
---

# BC-5.44.001: pr-manager and orchestrator MUST run a post-rebase diff-integrity gate after any `git rebase` on a feature branch, asserting that no file touched by a recently-merged sibling story shows an unverified net-negative line-count delta before force-push-with-lease

## Description

Git's ORT 3-way merge algorithm can silently drop production lines during a rebase when Branch A
and Branch B both modify adjacent (non-overlapping) regions of the same file. No conflict markers
appear; `git rebase --continue` reports success; the dropped lines are gone from the branch with
no trace. This is standard 3-way merge behavior, not a git bug — but the factory's per-story
delivery protocol has no detection gate for it.

This BC governs the **post-rebase diff-integrity gate** required by issue #365. After any
`git rebase`, `git rebase --continue`, or `git pull --rebase` completes on a feature branch that
shares modified files with recently-merged sibling stories on the target branch (`origin/develop`),
the responsible agent (pr-manager or orchestrator) MUST:

1. Run `git diff origin/develop --stat` on the rebased feature branch.
2. For each file showing a net-negative line count in that diff, determine whether sibling-story
   commits on `origin/develop` also modified that file (via `git log --oneline origin/develop` +
   `git diff-tree --name-only`).
3. For any file that is both (a) net-negative in the feature branch diff and (b) also modified by
   a recently-merged sibling story, verify the delta is intentional (a deliberate removal, not a
   silent drop). If any such file cannot be verified as intentional, STOP and require manual review.
4. Only after all net-negative deltas are either verified as intentional OR confirmed unaffected
   by sibling-story changes may the agent proceed to force-push-with-lease.

No new shell script or WASM plugin is required (POLICY 21 satisfied). This is a skill-doc mandate
expressed as a required orchestrator and pr-manager action.

## Preconditions

1. A `git rebase`, `git rebase --continue`, or `git pull --rebase` has completed on a feature
   branch (exit code 0 — rebase reported clean, no conflicts remaining).

2. The feature branch's pre-rebase history shares at least one modified file path with commits
   on `origin/develop` that were merged after the feature branch was created (sibling-story commits).
   If there are no shared file paths, the gate passes trivially (PC1).

3. The agent is about to execute `git push --force-with-lease` on the rebased feature branch.
   The gate runs between "rebase complete" and "force-push-with-lease".

## Postconditions

### PC1 — Clean: no unverified net-negative delta → force-push-with-lease proceeds

When, for every file in `git diff origin/develop --stat` that shows a net-negative line count,
EITHER the file was NOT modified by any recently-merged sibling story, OR the delta has been
confirmed as intentional removal (by agent inspection of the diff), the gate passes. The agent
may proceed to `git push --force-with-lease`.

### PC2 — Halted: unverified net-negative delta in sibling-touched file → STOP before force-push

When at least one file in `git diff origin/develop --stat` shows a net-negative line count AND
that file was also modified by a recently-merged sibling story AND the delta cannot be confirmed
as intentional, the gate MUST halt the force-push. The agent MUST emit a STOP signal:

```
STOP: Post-rebase diff-integrity gate detected an unverified net-negative line-count
delta in a file also modified by a recently-merged sibling story.

File(s) at risk:
  <filename>: <+added/-removed> lines (net: <N> lines)
  Modified by sibling story: <commit SHA> — <commit message>

This may indicate a silent drop from ORT 3-way merge (issue #365 class).
Required actions before force-push:
  1. Run `git diff origin/develop -- <filename>` and inspect the delta manually.
  2. Confirm each net-negative change is an intentional deletion, not a silent drop.
  3. If silent drops are found, restore the dropped lines and re-commit.
  4. Re-run the post-rebase diff-integrity gate after any corrections.
```

The `git push --force-with-lease` command MUST NOT be executed until the gate passes.

**Error variant:** `UnverifiedNetNegativeDelta`

### PC3 — Gate not required: no sibling-story file overlap

When `git diff origin/develop --stat` shows no file that is also in the recently-merged sibling
story commit set, the gate passes trivially with no action beyond the stat check. This is the
common case for stories operating in non-overlapping file spaces.

### PC4 — Gate not required: rebase on isolated branch with no sibling merges since branch creation

When no sibling-story commits have landed on `origin/develop` since the feature branch was created
(detectable by comparing `git merge-base HEAD origin/develop` to the branch creation point), the
gate passes trivially with no action.

## Invariants

1. **Gate runs between rebase completion and force-push — no exceptions.** Running the gate
   post-force-push cannot recover silently dropped lines. The `git push --force-with-lease`
   command MUST NOT execute until the gate completes cleanly (PC1 or PC3 or PC4).

2. **The full GUT/cargo test suite MUST run after any rebase before force-push.** The diff-integrity
   gate is a necessary but not sufficient check — it detects net-negative deltas but cannot detect
   all semantic regressions. The full test suite is the load-bearing regression check. This BC
   mandates the diff-integrity gate; the full suite mandate is in BC-6.10.002 and related TDD BCs.

3. **"Recently-merged sibling story" scope:** A commit on `origin/develop` is a "recently-merged
   sibling story commit" if (a) it is not an ancestor of the feature branch tip (i.e., it was
   merged after the feature branch was cut or last rebased) AND (b) it modifies at least one file
   also in the feature branch diff. The lookback window is unbounded — all commits between the
   branch point and `origin/develop` HEAD are considered.

4. **Agent judgment for intentional removals:** "Confirmed as intentional" means the agent
   explicitly inspects the diff hunk and asserts the removal is a deliberate code change present
   in the feature branch's commit history. It is NOT sufficient to assume intentionality from
   the absence of conflict markers — that is the exact failure mode this BC closes.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Rebase clean; diff shows only `src/lib.rs` +5/-2; no sibling story touched `src/lib.rs` | Gate passes (PC3); force-push proceeds |
| EC-002 | Rebase clean; diff shows `autoload.gd` +3/-7; sibling story S-20.01 also modified `autoload.gd` | Agent inspects diff; -7 lines confirmed intentional removal; PC1 passes |
| EC-003 | Rebase clean; diff shows `autoload.gd` +0/-4; sibling story S-20.01 also modified `autoload.gd`; agent cannot confirm -4 lines are intentional | STOPPED: PC2; `UnverifiedNetNegativeDelta`; manual review required |
| EC-004 | No sibling story commits since branch creation | Gate passes trivially (PC4) |
| EC-005 | `git diff origin/develop --stat` fails (network error) | Logged warning; agent treats as gate failure; escalates rather than force-pushing blind |
| EC-006 | Feature branch adds a file and removes another; removed file was touched by sibling story | STOPPED: removal-of-sibling-touched-file is a net-negative delta requiring verification |
| EC-007 | Silent drop of 4 lines from sibling story — agent inspects and confirms the drop is NOT in the feature branch's own commits | STOPPED: agent escalates; manual restoration required before retry |

## Canonical Test Vectors

| Test # | Scenario | Expected Result |
|--------|---------|----------------|
| T-1 | Clean rebase; diff shows only feature-branch files; no sibling overlap | Gate passes; force-push proceeds |
| T-2 | Rebase; diff shows `-4` lines in `autoload.gd`; S-20.01 also modified `autoload.gd`; agent confirms intentional | Gate passes; force-push proceeds |
| T-3 | Rebase; diff shows `-4` lines in `autoload.gd`; S-20.01 also modified `autoload.gd`; agent cannot confirm | STOPPED: `UnverifiedNetNegativeDelta`; force-push not executed |
| T-4 | `git diff origin/develop` reveals 4 silently dropped lines matching issue #365 exact repro | STOPPED: concrete detection of issue #365 scenario |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD) | Gate step present in pr-manager agent protocol between rebase and force-push | manual: confirm gate step in S-21.02 skill-doc deliverable |
| (TBD) | Gate detects net-negative delta in sibling-touched file | bats: simulate rebase scenario with mocked `git diff --stat` returning `-N` on sibling file; assert force-push blocked |
| (TBD) | Gate passes cleanly when no sibling file overlap | bats: mock no-overlap diff; assert force-push proceeds |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | TBD — E-21 CAP pending ARCH-INDEX registration |
| Capability Anchor Justification | New capability for post-rebase diff-integrity; no existing CAP covers rebase-integrity assertion. |
| L2 Domain Invariants | none (operational infrastructure) |
| Architecture Module | `plugins/vsdd-factory/agents/pr-manager.md` (step 8 amendment; to be amended by S-21.02); orchestrator per-story delivery playbook (skill-doc change) |
| Stories | S-21.02 (E-21 Wave 1) |
| Source Issues | #365 (rebase auto-merge silently drops production lines) |
| ADR Reference | none |

## Related BCs

- BC-5.43.001 — sibling SS-05 gate for product-branch merge safety (different mechanism: this BC governs rebase-then-force-push; BC-5.43.001 governs merge/pull/checkout pre-check)
- BC-5.42.001 — pr-manager READY-verdict enforcement; this BC adds a pre-force-push gate to the same pr-manager agent protocol

## Architecture Anchors

- `plugins/vsdd-factory/agents/pr-manager.md` — step 8 ("push") to be amended: insert diff-integrity gate before `git push --force-with-lease`
- orchestrator per-story delivery playbook (skill-doc) — rebase sub-step to be added with gate invocation

## Story Anchor

S-21.02 (E-21 Wave 1 — post-rebase diff-integrity gate: detect and surface silent production-code drops before force-push)

## VP Anchors

TBD — VP IDs to be assigned after VP authoring pass.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-07-19 | Initial authoring (product-owner; E-21 factory-state data-loss hardening; issue #365). Post-rebase diff-integrity gate: mandatory `git diff origin/develop --stat` + sibling-story overlap check + intentional-delta verification before `git push --force-with-lease` (PC1 pass / PC2 halt / PC3/PC4 trivial-pass). 1 error variant: `UnverifiedNetNegativeDelta`. 7 edge cases EC-001..EC-007. 4 test vectors T-1..T-4. lifecycle_status: draft (POL-14 auto-promotion on S-21.02 PR merge). |
