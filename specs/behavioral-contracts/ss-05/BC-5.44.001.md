---
document_type: behavioral-contract
level: L3
version: "1.3"
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
capability: "CAP-035"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill
modified:
  - "2026-07-19 (v1.1) — CAP-035 backfill (product-owner; ARCH-INDEX v3.07): capability frontmatter TBD→CAP-035; §Traceability L2 Capability TBD→CAP-035; Capability Anchor Justification updated to cite CAP-035/ARCH-INDEX v3.07."
  - "2026-07-19 (v1.2) — Research validation precision amendments (product-owner; research validation 2026-07-19): §Description restructured — `git range-diff` promoted to PRIMARY detector (step 1a); --stat heuristic demoted to backup signal (step 1b); §Description Invariant 5 added with known limitation (heuristic misses drops when branch additions offset dropped lines; adjacent-edit cases empirically conflict rather than silently drop; real silent drops require larger/moved-code diffs)."
  - "2026-07-19 (v1.3) — adv pass-1 fix burst (F-P1-006) per ADR-031 v1.1 §Consequences #5 (product-owner): §Architecture Anchors pr-manager.md step 8 anchor removed (ground truth: Step 8 = Execute merge; no rebase/force-push in pr-manager.md); replaced with devops-engineer.md §Inter-Wave Rebase (the only codebase site with rebase+force-with-lease). §Traceability Architecture Module corrected to match. §Related BCs BC-5.42.001 description updated (gate host is devops-engineer, not same pr-manager protocol)."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.44.001
section: "5.44"
last_amended: "(v1.3) — adv pass-1 fix burst (F-P1-006) per ADR-031 v1.1 §Consequences #5 (product-owner): §Architecture Anchors corrected pr-manager step 8→devops-engineer.md §Inter-Wave Rebase; §Traceability Architecture Module corrected; §Related BCs BC-5.42.001 updated. [Prior: (v1.2) — research validation; range-diff primary detector; ease-of-trigger temper. (v1.1) — CAP-035 backfill. (v1.0) — Initial authoring.]"
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

1a. **Primary detector — `git range-diff` (canonical rebase integrity check):** Run
    `git range-diff <pre-rebase-tip>...<post-rebase-tip>` to compare the replayed commits
    before and after the rebase. `git range-diff` is the canonical tool for detecting whether
    the rebase altered commit content during replay (dropped lines, changed context). Any commit
    pair showing `modified` or `changed` status that touches a file also modified by a
    recently-merged sibling story MUST be inspected before proceeding to force-push.

1b. **Backup heuristic — `git diff origin/develop --stat`:** If `git range-diff` is unavailable
    (git < 2.19) or yields inconclusive results, fall back to running `git diff origin/develop --stat`
    on the rebased feature branch. For each file showing a net-negative line count in that diff,
    determine whether sibling-story commits on `origin/develop` also modified that file (via
    `git log --oneline origin/develop` + `git diff-tree --name-only`). **Known limitation:** this
    heuristic misses drops when the feature branch's own additions in the same file offset the
    dropped sibling-story lines (per-file net line count ≥ 0). Use `git range-diff` as the
    primary detector to avoid this blind spot.

2. For any file flagged by step 1a or 1b, verify the delta is intentional (a deliberate removal
   present in the feature branch's own commit history, not a silent replay drop). If any such
   file cannot be verified as intentional, STOP and require manual review.

3. Only after all flagged deltas are either verified as intentional OR confirmed unaffected
   by sibling-story changes may the agent proceed to force-push-with-lease.

**Ease-of-trigger temper:** The ORT 3-way merge silent-drop scenario requires the feature branch
and the sibling story to have modified ADJACENT but non-overlapping regions of the same file, and
for those regions to be large enough or moved enough to avoid raising a textual conflict. Simple
adjacent line additions typically DO conflict (surface to the user). Real silent drops are genuine
but empirically rare; they require larger diffs, moved code blocks, or refactored context. The gate
is warranted as defense-in-depth, but implementers should not expect to trigger it on most rebases.

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
| L2 Capability | CAP-035 |
| Capability Anchor Justification | CAP-035 registered in ARCH-INDEX v3.07 (ADR-031, commit 14a78515): "Post-Rebase Diff-Integrity — mandatory diff-integrity gate between `git rebase` completion and `git push --force-with-lease` on feature branches." BC-5.44.001 is the sole implementing BC for CAP-035. |
| L2 Domain Invariants | none (operational infrastructure) |
| Architecture Module | `plugins/vsdd-factory/agents/devops-engineer.md` §Inter-Wave Rebase (the only codebase site with `git rebase origin/develop` + `git push --force-with-lease`; to be amended by S-21.02 per ADR-031 §Consequences #5) |
| Stories | S-21.02 (E-21 Wave 1) |
| Source Issues | #365 (rebase auto-merge silently drops production lines) |
| ADR Reference | none |

## Related BCs

- BC-5.43.001 — sibling SS-05 gate for product-branch merge safety (different mechanism: this BC governs rebase-then-force-push; BC-5.43.001 governs merge/pull/checkout pre-check)
- BC-5.42.001 — pr-manager READY-verdict enforcement; this gate (BC-5.44.001) lives in devops-engineer.md §Inter-Wave Rebase, NOT the pr-manager protocol; BC-5.42.001 governs the READY-verdict + merge-strategy enforcement in the separate pr-manager lifecycle

## Architecture Anchors

- `plugins/vsdd-factory/agents/devops-engineer.md` §Inter-Wave Rebase — the only codebase site with a `git rebase origin/develop` + `git push --force-with-lease` sequence; the diff-integrity gate (step 1a `git range-diff` primary detector + step 1b `git diff --stat` backup) is to be inserted between rebase completion and `git push --force-with-lease` by S-21.02 (per ADR-031 v1.1 §Consequences #5). Note: `pr-manager.md` Step 8 = "Execute merge" — no rebase or force-push sub-step exists there; it is NOT a valid amendment target for this BC.

## Story Anchor

S-21.02 (E-21 Wave 1 — post-rebase diff-integrity gate: detect and surface silent production-code drops before force-push)

## VP Anchors

TBD — VP IDs to be assigned after VP authoring pass.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-07-19 | Initial authoring (product-owner; E-21 factory-state data-loss hardening; issue #365). Post-rebase diff-integrity gate: mandatory `git diff origin/develop --stat` + sibling-story overlap check + intentional-delta verification before `git push --force-with-lease` (PC1 pass / PC2 halt / PC3/PC4 trivial-pass). 1 error variant: `UnverifiedNetNegativeDelta`. 7 edge cases EC-001..EC-007. 4 test vectors T-1..T-4. lifecycle_status: draft (POL-14 auto-promotion on S-21.02 PR merge). |
| 1.3 | 2026-07-19 | adv pass-1 fix burst (F-P1-006) per ADR-031 v1.1 §Consequences #5 (product-owner). §Architecture Anchors: pr-manager.md step 8 anchor removed (Step 8 = Execute merge; no rebase/force-push in pr-manager.md); replaced with devops-engineer.md §Inter-Wave Rebase (only codebase site with rebase+force-with-lease). §Traceability Architecture Module corrected to match. §Related BCs BC-5.42.001 description updated: gate host = devops-engineer, not pr-manager protocol. |
| 1.2 | 2026-07-19 | Research validation precision amendments (product-owner; research validation 2026-07-19). §Description protocol restructured: `git range-diff <pre>...<post>` promoted to PRIMARY detector (step 1a) — canonical rebase integrity check; `git diff --stat` heuristic demoted to backup signal (step 1b). Known limitation documented: heuristic misses drops when branch additions offset dropped lines (per-file net ≥ 0). Ease-of-trigger temper added: adjacent-edit cases empirically conflict (surface); real silent drops require larger/moved-code diffs — genuine but rare. |
| 1.1 | 2026-07-19 | CAP-035 backfill (product-owner; ARCH-INDEX v3.07, ADR-031, commit 14a78515): capability frontmatter TBD→CAP-035; §Traceability L2 Capability TBD→CAP-035; Capability Anchor Justification updated to cite CAP-035/ARCH-INDEX v3.07. |
