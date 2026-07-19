---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: product-owner
timestamp: 2026-07-19T00:00:00Z
phase: brownfield-backfill
cycle: v1.0-brownfield-backfill
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/e-21-arch-delta-analysis.md
  - plugins/vsdd-factory/agents/pr-manager.md
  - .factory/specs/behavioral-contracts/ss-06/BC-6.23.001.md
input-hash: "TBD"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: null
subsystem: "SS-06"
capability: "CAP-037"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill
modified:
  - "2026-07-19 (v1.1) — CAP-037 backfill (product-owner; ARCH-INDEX v3.07): capability frontmatter TBD→CAP-037; §Traceability L2 Capability TBD→CAP-037; Capability Anchor Justification updated to cite CAP-037/ARCH-INDEX v3.07."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-6.27.001
section: "6.27"
last_amended: "(v1.1) — CAP-037 backfill (product-owner; ARCH-INDEX v3.07): capability frontmatter TBD→CAP-037; §Traceability L2 Capability + Capability Anchor Justification updated. [Prior: (v1.0) — Initial authoring (product-owner; E-21 factory-state data-loss hardening; issue #588). pr-manager factory-side PR protocol: 5-step restore sequence + dispatch-preamble branch assertion. New BC (not BC-6.23.001 amendment). lifecycle_status: draft (POL-14 auto-promotion on implementing PR merge).]"
---

# BC-6.27.001: pr-manager factory-side PR protocol MUST restore the `.factory/` worktree to `factory-artifacts`, pull `--ff-only`, and delete both the local and remote chore branch after merging any PR that modifies `factory-artifacts` directly, and MUST assert `factory-artifacts` is the current branch before any `.factory/` write

## Description

When pr-manager needs to update `factory-artifacts` directly via a PR (e.g., VP anchor true-up,
merge-SHA annotation, spec housekeeping), it checks the SHARED `.factory/` worktree onto a chore
branch, commits the changes, creates and merges the PR. Without a post-merge restoration protocol,
the shared worktree is left stranded on the chore branch. Subsequent agents that write to `.factory/`
(state-manager closing bursts, spec passes) inherit the stranded branch silently — git gives no
warning, `factory-branch-guard.sh` cannot catch Bash-mediated commits, and factory history
accumulates on the stray branch until it is eventually lost.

This BC governs two complementary protocol requirements:

**Factory-side PR protocol (post-merge restore sequence):** Every factory-side PR (a PR whose
head branch is a chore branch of `factory-artifacts`) MUST end with a mandatory 5-step restore
sequence. The PR is not "done" until all 5 steps complete without error.

**Dispatch-preamble branch assertion (defense-in-depth):** Every agent that writes to `.factory/`
(state-manager, pr-manager in factory-side PR flows) MUST assert `git -C .factory branch --show-current`
equals `factory-artifacts` as the very first action of each dispatch preamble. If the assertion
fails (the worktree is on a wrong branch), the dispatch MUST STOP and report the stranding before
any `.factory/` write or commit occurs.

**New BC rather than BC-6.23.001 amendment — rationale:** BC-6.23.001 governs the `/factory-lock`
and `/factory-unlock` skill behaviors (ADR-025 Decisions 5/6/8, CAP-031 cooperative lock
acquire/release). The factory-side PR protocol is a pr-manager concern: it governs what pr-manager
does AFTER merging a factory-side PR, not how locks are acquired or displayed. The two behaviors
are independent — a factory-side PR can occur during an unlocked session, and lock/unlock can occur
without any factory-side PRs. BC-6.27.001 is the correct container for the pr-manager factory-side
flow; BC-6.23.001 is preserved as the lock-lifecycle BC.

## Preconditions

### Factory-side PR protocol preconditions

1. pr-manager has executed a `gh pr merge` on a factory-side PR (a PR whose head branch is
   `chore/<name>` targeting `factory-artifacts` as the base branch, created from the shared
   `.factory/` worktree).

2. The PR merge has completed successfully (`state: MERGED`).

3. The `.factory/` worktree is currently checked out on the chore branch (the state left by the
   factory-side PR flow before the restore sequence).

### Dispatch-preamble branch assertion preconditions

4. Any agent (state-manager, pr-manager factory-side flow) is about to execute a git commit,
   Edit, or Write operation targeting a path under `.factory/`.

## Postconditions

### PC1 — Factory-side PR protocol: 5-step restore sequence MUST complete before PR is "done"

Immediately after a factory-side PR merges, pr-manager MUST execute the following 5-step restore
sequence in order. The PR MUST NOT be declared "done" until all 5 steps succeed:

**Step 1 — Restore the worktree to `factory-artifacts`:**
```
git -C .factory checkout factory-artifacts
```
The worktree MUST be on `factory-artifacts` after this step. If the checkout fails (dirty tree,
locked by another process), pr-manager MUST report the failure and halt.

**Step 2 — Fast-forward pull to true-up against the just-merged origin:**
```
git -C .factory pull --ff-only origin factory-artifacts
```
This advances the local `factory-artifacts` ref to the just-merged PR content on origin. If `--ff-only`
fails (non-fast-forward condition indicating divergence), pr-manager MUST report the failure — this
indicates a concurrent write landed between the PR merge and the ff-only pull; manual resolution
is required before proceeding.

**Step 3 — Delete the local chore branch:**
```
git -C .factory branch -d chore/<name>
```
If the local branch does not exist (already deleted), this step succeeds trivially (exit 0).

**Step 4 — Delete the remote chore branch:**
```
git push origin --delete chore/<name>
```
If the remote branch does not exist (already deleted by auto-delete on merge), this step succeeds
trivially. If the delete fails for another reason (network error, permission), pr-manager MUST log
a warning and continue — remote chore-branch cleanup is a best-effort cleanup, not a blocking gate.

**Step 5 — Verify worktree is on `factory-artifacts`:**
```
git -C .factory branch --show-current
```
Assert the output equals `factory-artifacts`. If not, halt and report. This is the final gate
confirming the restore sequence completed correctly.

**INV-E21-003 instantiation:** The `.factory/` git worktree MUST be on `factory-artifacts` at the
end of every factory write operation. This postcondition closes the worktree-stranding window for
factory-side PR flows.

**Error variants:** `CheckoutRestoreFailed`, `FFOnlyPullFailed`, `FinalBranchAssertionFailed`

### PC2 — Dispatch-preamble branch assertion: verify `factory-artifacts` before any `.factory/` write

Every agent dispatch that writes to `.factory/` MUST include the following assertion as the FIRST
step of its dispatch preamble (before any file read, any state check, any write):

```
ASSERT: git -C .factory branch --show-current == "factory-artifacts"
```

If the assertion fails (output is anything other than `factory-artifacts`), the dispatch MUST
immediately STOP with the following report:

```
STOP: .factory/ worktree is on branch '<actual_branch>', expected 'factory-artifacts'.
This indicates a prior factory-side PR or lifecycle operation left the worktree stranded
(issue #588 class). Do NOT write to .factory/ until the worktree is restored.
Required recovery:
  1. git -C .factory checkout factory-artifacts
  2. git -C .factory pull --ff-only origin factory-artifacts
  3. (If the stray branch has uncommitted work) evaluate whether to commit or discard.
  4. Retry this dispatch after the worktree is restored.
```

**INV-E21-003 instantiation:** The `.factory/` worktree MUST be on `factory-artifacts` at the
START of every factory write dispatch. This assertion catches every stranding scenario — not just
factory-side PR stranding but any mechanism that could leave the worktree on an unexpected branch.

**Error variant:** `FactoryWorktreeOnWrongBranch`

## Invariants

1. **INV-E21-003 (Factory Worktree Branch Invariant):** The `.factory/` worktree MUST be on
   `factory-artifacts` at the start AND end of every factory write operation. The dispatch-preamble
   assertion (PC2) enforces the start condition; the 5-step restore sequence (PC1) enforces the
   end condition for factory-side PR flows.

2. **The 5-step restore sequence is NOT optional.** A factory-side PR that ends at step 9 of the
   pr-manager 9-step protocol without executing the restore sequence leaves the worktree stranded.
   The restore sequence is part of the step 9 definition for factory-side PRs.

3. **`--ff-only` failure is a STOP signal, not a recoverable warning.** A non-fast-forward
   condition after a factory-side PR merge indicates a concurrent write race. Forcing a merge or
   rebase in this condition could corrupt the factory-artifacts history. The agent MUST stop and
   require human intervention.

4. **PC2 applies to ALL `.factory/` writes, including Bash-mediated git commits.** The existing
   `factory-branch-guard.sh` (BC-4.13.001) blocks Edit/Write tool calls when `.factory/` is on
   the wrong branch. PC2 closes the Bash-tool git-commit gap: even `git -C .factory commit` (which
   bypasses the Edit/Write guard) must be preceded by the branch assertion.

5. **INV-E21-002 applies within the factory-side PR flow too.** Any factory-artifact writes made
   while on the chore branch MUST use absolute paths anchored to the canonical `.factory/` mount
   (BC-6.26.001 PC1). The write discipline and the branch restore are independent requirements.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Factory-side PR merges; restore sequence runs; all 5 steps succeed | PR declared done; `.factory/` on `factory-artifacts`; local + remote chore branch deleted |
| EC-002 | `git checkout factory-artifacts` fails (dirty tree) | `CheckoutRestoreFailed`; pr-manager halts; requires manual stash + retry |
| EC-003 | `git pull --ff-only` fails (concurrent write landed between merge and pull) | `FFOnlyPullFailed`; pr-manager halts; requires manual resolution |
| EC-004 | Local chore branch already deleted (auto-cleanup) | Step 3 succeeds trivially (branch absent is not an error) |
| EC-005 | Remote chore branch already deleted (GitHub auto-delete on merge) | Step 4 succeeds trivially; warning logged if deletion attempt fails for other reason |
| EC-006 | Dispatch preamble assertion: worktree is on `factory-artifacts` | Assertion passes; dispatch proceeds |
| EC-007 | Dispatch preamble assertion: worktree is on `chore/vp-anchor-trueup-pr115-factory` | `FactoryWorktreeOnWrongBranch` STOP; dispatch halted; recovery instructions emitted |
| EC-008 | Factory-side PR merged; restore sequence attempted; `git branch --show-current` returns unexpected branch after step 5 | `FinalBranchAssertionFailed`; escalate to orchestrator |
| EC-009 | No factory-side PR in current story delivery (normal story PR to develop) | PC1 and dispatch-preamble PC2 are independent — state-manager factory-write dispatches still carry the PC2 assertion regardless of whether a factory-side PR occurred |

## Canonical Test Vectors

| Test # | Precondition | Action | Expected Result |
|--------|-------------|--------|----------------|
| T-1 | Factory-side PR merged; worktree on `chore/name` | PR declared done (trigger PC1) | 5-step restore: checkout → pull → local-delete → remote-delete → verify; all succeed; worktree on `factory-artifacts` |
| T-2 | `git pull --ff-only` would fail (non-fast-forward) | Restore sequence step 2 | `FFOnlyPullFailed`; pr-manager halts |
| T-3 | Dispatch preamble; worktree on `factory-artifacts` | Assert `git -C .factory branch --show-current` | Returns `factory-artifacts`; dispatch continues |
| T-4 | Dispatch preamble; worktree on `chore/vp-trueup-pr115` | Assert `git -C .factory branch --show-current` | `FactoryWorktreeOnWrongBranch` STOP; dispatch halted |
| T-5 | Dispatch preamble check passes; agent commits on `factory-artifacts` | `git -C .factory commit ...` | Succeeds; branch remains `factory-artifacts` |
| T-6 | Remote chore branch already deleted before step 4 | `git push origin --delete chore/name` | Step 4 exits non-zero; warning logged; continue (EC-005) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD) | Factory-side PR protocol section present in pr-manager.md | manual: confirm section present in S-21.05 skill-doc deliverable |
| (TBD) | 5-step restore fires in order after factory-side PR merge | bats: mock factory-side PR flow; assert steps 1-5 execute in sequence |
| (TBD) | Dispatch preamble assertion blocks write on wrong-branch worktree | bats: mock `git branch --show-current` returning chore branch; assert dispatch halts before any write |
| (TBD) | `--ff-only` failure halts restore sequence | bats: mock `git pull --ff-only` returning non-zero; assert `FFOnlyPullFailed` and no further steps |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-037 |
| Capability Anchor Justification | CAP-037 registered in ARCH-INDEX v3.07 (ADR-031, commit 14a78515): "PR-Manager Factory-Side PR Protocol — post-merge restore sequence (checkout factory-artifacts → pull --ff-only → delete chore branches → assert branch) plus dispatch-preamble branch assertion before any `.factory/` write." BC-6.27.001 is the sole implementing BC for CAP-037 (INV-E21-003). Distinct from CAP-031 (BC-6.23.001 lock/unlock skill behaviors, ADR-025). |
| L2 Domain Invariants | none (operational infrastructure) |
| Architecture Module | `plugins/vsdd-factory/agents/pr-manager.md` (factory-side PR protocol section; to be added by S-21.05); dispatch-preamble templates for state-manager and pr-manager |
| Stories | S-21.05 (E-21 Wave 2) |
| Source Issues | #588 (factory-side PR strands shared `.factory/` worktree on chore branch) |
| ADR Reference | none |

## Related BCs

- BC-6.23.001 — sibling SS-06 BC governing `/factory-lock`/`/factory-unlock` lock lifecycle (CAP-031, ADR-025 D4/D5/D7/D8); orthogonal to this BC (lock lifecycle vs. worktree restore); BC-6.27.001 created as a separate BC because the factory-side PR protocol has no coupling to the lock acquire/release mechanism
- BC-6.26.001 — sibling worktree-lifecycle BC governing story-worktree write-path discipline + teardown preflight (issue #523); same shared-mutable-worktree problem class, different trigger and agents

## Architecture Anchors

- `plugins/vsdd-factory/agents/pr-manager.md` — factory-side PR protocol section to be added (to be amended by S-21.05)
- `plugins/vsdd-factory/hooks/factory-branch-guard.sh` — existing guard covering Edit/Write tool surface; PC2 closes the Bash-mediated git-commit gap that this guard cannot cover; grandfathered per E-20 scope

## Story Anchor

S-21.05 (E-21 Wave 2 — pr-manager factory-side PR protocol: restore-original-branch, ff-only sync, and chore-branch cleanup)

## VP Anchors

TBD — VP IDs to be assigned after VP authoring pass.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-07-19 | Initial authoring (product-owner; E-21 factory-state data-loss hardening; issue #588; S-21.05). PC1: factory-side PR 5-step restore sequence (checkout factory-artifacts → pull --ff-only → delete local chore branch → delete remote chore branch → final branch assertion). PC2: dispatch-preamble branch assertion before any `.factory/` write (INV-E21-003). 3 error variants: `CheckoutRestoreFailed`, `FFOnlyPullFailed`, `FactoryWorktreeOnWrongBranch`, `FinalBranchAssertionFailed`. 9 edge cases EC-001..EC-009. 6 test vectors T-1..T-6. New BC (not BC-6.23.001 amendment): different behavioral surface from lock/unlock skills; rationale documented inline. lifecycle_status: draft (POL-14 auto-promotion on S-21.05 PR merge). |
| 1.1 | 2026-07-19 | CAP-037 backfill (product-owner; ARCH-INDEX v3.07, ADR-031, commit 14a78515): capability frontmatter TBD→CAP-037; §Traceability L2 Capability TBD→CAP-037; Capability Anchor Justification updated to cite CAP-037/ARCH-INDEX v3.07. |
