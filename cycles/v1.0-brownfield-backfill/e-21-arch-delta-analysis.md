---
document_type: architect-delta-analysis
epic_id: "E-21"
epic_working_title: "Factory State Data-Loss Hardening"
version: "v1.0"
status: draft
producer: architect
timestamp: 2026-07-19T00:00:00Z
cycle: v1.0-brownfield-backfill
issues: [342, 365, 358, 523, 588]
feeds_to: [product-owner (BC authoring), story-writer (epic/story decomposition)]
policy_21_note: "POLICY 21 no_new_shell_scripts is BLOCKING — all new mechanics must be WASM hook plugins, Rust helpers, or skill-doc step changes. No new .sh files may be added."
---

# E-21 Architectural Delta Analysis — Factory State Data-Loss Hardening

## Executive Summary

Five confirmed-live pilot-fleet issues form a single loss-of-data family: factory
artifact content can be silently destroyed or made unreachable by normal product
branch operations, story worktree lifecycle operations, and pr-manager flow gaps.
All five issues are confirmed live against the current codebase (none were fully
closed by prior stories). The family divides into three distinct root-cause classes
with overlapping fix vectors.

---

## Per-Issue Analysis

### Issue #342 — Product-branch merge silently `rm`s a `.factory` file the nested worktree is serving

**Root-cause statement (mechanism).**
The `.factory/` directory is a git worktree nested inside the main worktree's
working directory (`<repo>/.factory`). Git's merge/checkout working-tree update
algorithm has no awareness of nested worktrees. When a product-branch merge carries
a "delete this path" tree diff and that path physically lives under `.factory/`,
git removes the on-disk file — bypassing the nested-worktree owning that path —
with no warning. The trigger requires a path to appear in both the product branch's
tracked tree and the `.factory/` physical directory (the dual-tracking condition,
companion issue #341). The invariant protecting against the dual-tracking condition
is enforced by the existing `factory-branch-guard.sh` hook for Edit/Write operations,
but no gate prevents the dual-tracking from forming via a *prior* accidental commit
on the product branch, and no gate intercepts a `git merge`/`git checkout` Bash
command before it runs.

**Current codebase verification.**
`plugins/vsdd-factory/hooks/factory-branch-guard.sh` exists and blocks
Edit/Write tool calls when `.factory/` is on the wrong branch. It does NOT protect
against git working-tree updates triggered by Bash tool merge/checkout commands.
`plugins/vsdd-factory/skills/factory-health/SKILL.md` has no pre-merge intersection
check. No WASM plugin in the hooks-registry intercepts Bash-tool git checkout/merge
calls. Status: **confirmed open, not fixed by any prior story**.

**Solution shape.**
Two layers:

1. **Invariant layer (prevent dual-tracking):** Add an `validate-artifact-path`
   WASM hook plugin (already listed as PLANNED in SS-04 `crates/hook-plugins/validate-artifact-path/`)
   that fires PreToolUse on Bash and blocks any `git add` command that would stage
   a path under `.factory/` on a product branch (develop/main/feature/*). This is
   the structural fix: no dual-tracking → no clobber surface.

2. **Safety net layer (intercept before harm):** Add a skill-doc step to the
   orchestrator's product-branch merge protocol: before any `git merge` or
   `git pull` on the product branch, run `git diff --name-only HEAD..<target>`
   and assert the result contains no path matching `.factory/`. If it does, STOP
   and require manual handling. This is implementable as a mandatory step in the
   orchestrator agent prompt for merge operations (SS-05, skill-doc change).

POLICY 21 note: mechanism (1) uses the PLANNED WASM crate path; no new .sh file
required. Mechanism (2) is a skill-doc change.

**BC/VP impact.**
E-17 BCs (BC-4.13.001 factory-lock guard, BC-5.40.001 STATE.md lock schema,
BC-6.23.001 lock/unlock skill) address *concurrent-session* write races on
`factory-artifacts`. They do not address product-branch merge clobbering of the
nested worktree. **New BC required**: one BC in SS-04 covering the
`validate-artifact-path` WASM plugin's PreToolUse behavior (blocking `git add`
of `.factory/` paths on non-`factory-artifacts` branches), and one BC amendment
to SS-05 (orchestrator merge-gate pre-check).
E-18 BCs address context-window durability, not git-level working-tree destruction.
No reuse available for this issue.

**Proposed story boundary.** One story: S-21.01 "factory artifact path guard — prevent
dual-tracking and intercept product-branch merges that would clobber `.factory/`
paths." The WASM guard crate (`validate-artifact-path`) is already stubbed in the
plugin ecosystem plan; the skill-doc merge-gate step is lightweight. Combined scope
is appropriate since both pieces share the same BC anchor and the same precondition
(no path may be tracked on both a product branch and the `factory-artifacts` branch
simultaneously).

**Estimated points.** 8 (WASM guard crate + registry + bats) + 3 (skill-doc step) = 11.
Wave placement: depends on no prior E-21 story; Wave 1 candidate.

---

### Issue #365 — Rebase auto-merge silently drops production lines

**Root-cause statement (mechanism).**
Git's ORT 3-way merge algorithm resolves textual changes without semantic
understanding. When Branch A adds a new line X between context lines C1 and C2,
and Branch B independently modifies C1 or C2 in a non-conflicting region, ORT
can award Branch B's version of the surrounding context without preserving X —
producing no conflict markers while silently discarding X. This is not a git bug;
it is expected 3-way merge behavior on adjacent-but-not-overlapping hunks. The
factory's per-story delivery orchestration has no post-rebase verification step
that checks for net-negative line counts in files touched by sibling merged stories.

**Current codebase verification.**
`plugins/vsdd-factory/skills/deliver-story/steps/step-f-pr-lifecycle.md` delegates
the PR lifecycle to pr-manager with "Target: develop" in the task description. The
orchestrator per-story-delivery flow has no rebase sub-step after a sibling story
lands. `plugins/vsdd-factory/agents/pr-manager.md` step 8 performs the merge but
has no post-rebase sanity check. No WASM plugin intercepts rebase operations.
Status: **confirmed open, process gap with no current mitigation**.

The issue references an orchestrator-rebase dispatch ("orchestrator-rebase playbook"),
which does not exist as a dedicated skill or step file in the current codebase. The
gap is real: the expected process documentation does not exist at the path implied
by the issue.

**Solution shape.**
Skill-doc step change (SS-05/SS-06): Add a "post-rebase diff sanity gate" sub-step
to the pr-manager agent and/or orchestrator per-story-delivery playbook. After any
`git rebase --continue` reports success on a feature branch that shares file paths
with recently merged sibling stories, the responsible agent must:

1. Run `git diff origin/develop --stat` on the rebased branch.
2. For each file with a net-negative line count that was also modified by a
   recently merged sibling story (detectable via `git log --oneline origin/develop`
   + `git diff-tree`), assert the negative delta is intentional removal, not
   silent drop. If any negative delta is unverified, STOP and require manual review.
3. Only then force-push-with-lease.

This is a skill-doc mandate (no new shell script needed); the check is expressed
as a required orchestrator action documented in the skill step.

**BC/VP impact.**
E-17 and E-18 BCs do not cover rebase safety. **New BC required** in SS-05
(Pipeline Orchestration) covering the post-rebase diff-integrity assertion as a
required gate in the per-story delivery protocol.

**Proposed story boundary.** One story: S-21.02 "post-rebase diff-integrity gate —
detect and surface silent production-code drops before force-push." This is
entirely skill-doc (no new code artifacts); a targeted story is appropriate.

**Estimated points.** 3 (skill-doc step with verifiable AC). Wave placement: Wave 1
candidate (no inter-E-21 dependencies).

---

### Issue #358 — PR base not locked to trunk; orphan merge possible

**Root-cause statement (mechanism).**
`pr-manager` already emits `--base develop` in its `gh pr create` invocation
(confirmed at `plugins/vsdd-factory/agents/pr-manager.md:139`). The explicit-base
half of the fix is in place. The open gap is the absence of two post-action
assertions:

1. **Post-create baseRefName assertion** — after `gh pr create`, read back the
   PR's `baseRefName` and assert it equals the configured trunk. This catches the
   case where `gh` CLI infers a different base from the worktree's tracking branch
   upstream, overriding the `--base` flag.
2. **Post-merge ancestry assertion** — after `gh pr merge`, run
   `git merge-base --is-ancestor <merge_sha> origin/<trunk>` and assert true. This
   catches the case where the PR merged into an off-trunk branch (orphan merge)
   without the post-create assertion catching it (e.g., if the post-create check
   was skipped or passed erroneously).

The concrete issue instance (ArcavenAE/akey PR #14) confirms the gap: `--base` was
not explicitly supplied and the base was inferred from the tracking upstream.
The issue comment (`arcaven`, 2026-06-30) confirms that line 139's `--base develop`
is the intended pattern; what is missing is the assertions (2) specifically the
`--is-ancestor` check.

**Current codebase verification.**
`pr-manager.md` step 3 shows the `gh pr create --base develop` invocation; it has
no subsequent `gh pr view --json baseRefName` assertion step. `pr-manager.md` step 9
("post-merge") instructs "trigger worktree cleanup and state updates" with no
`git merge-base --is-ancestor` check. Status: **confirmed open — post-create and
post-merge assertions both absent**.

**Solution shape.**
Amendment to `plugins/vsdd-factory/agents/pr-manager.md`:

- Step 3: After `gh pr create`, spawn `github-ops` to run
  `gh pr view <num> --json baseRefName` and assert the returned value equals the
  configured trunk (`develop` for greenfield/feature pipelines). Hard-fail the
  burst if mismatched.
- Step 9: Before marking the story delivered, spawn `github-ops` to run
  `git fetch origin <trunk> && git merge-base --is-ancestor <merge_sha> origin/<trunk>`.
  If the exit code is non-zero (not an ancestor), raise immediately as a P0 data
  error — the content did not land on trunk.

No new shell script, WASM plugin, or Rust helper required; these are skill-doc
amendments to the existing pr-manager agent definition.

**BC/VP impact.**
BC-6.10.002 covers the 9-step deliver-story sequence (step 7 = pr-manager 9-step
lifecycle). **BC amendment required**: BC-6.10.002 (or whichever BC specifically
governs the pr-manager 9-step protocol) needs new postconditions: PC-N "PR base
equals configured trunk (verified by baseRefName assertion post-create)" and
PC-N+1 "Merge commit is an ancestor of origin/trunk (verified by merge-base
--is-ancestor post-merge)." E-17/E-18 BCs do not cover PR merge correctness.

**Proposed story boundary.** One story: S-21.03 "pr-manager trunk-assertion
hardening — post-create baseRefName check and post-merge ancestry assertion."
Targeted, bounded, and does not require code changes.

**Estimated points.** 3 (skill-doc amendment with verifiable ACs against a bats
fixture). Wave placement: Wave 1 candidate; no inter-E-21 dependencies.

---

### Issue #523 — Story-worktree `.factory` artifacts silently lost at teardown

**Root-cause statement (mechanism).**
Story agents operate with their CWD set to the story worktree
(`.worktrees/STORY-NNN/`). When an agent resolves `.factory/...` paths relative to
CWD rather than the canonical repo root, it writes into the story worktree's stale
`.factory/` snapshot (populated by `git worktree add` at worktree creation time and
never updated). This shadow tree is not tracked on `factory-artifacts`, so factory
tooling at the main checkout never sees the writes. When Step G runs
`git worktree remove --force`, the shadow `.factory/` tree and all artifacts written
to it are silently destroyed.

The shared-context rule (`deliver-story/steps/_shared-context.md` §Spec-Path
Discipline) mandates canonical absolute paths for spec/BC *reads* but does not
explicitly name factory artifact *writes* (DELIVERY ledger, story-frontmatter
updates, pr-review.md records) as the load-bearing case. Step G (`step-g-cleanup.md`)
issues `git worktree remove` via devops-engineer with no preflight inventory check.

Three confirmed instances across the pilot fleet (DELIVERY ledger in OP, engine-config
file in inverse direction, pr-reviewer pr-review.md in 2026-07-15 comment).

**Current codebase verification.**
`plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md` lines 56-62:
the existing rule names `spec, BC, and ADR files` reads as the mandate scope;
no explicit mention of DELIVERY ledger writes or story-frontmatter updates.
`plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md`: Sub-step G.1
dispatches `devops-engineer` to `git worktree remove --force` with no `.factory/`
inventory preflight. Status: **confirmed open — both gaps present in current code**.

**Solution shape.**
Two targeted changes (both skill-doc, no new shell scripts):

1. **Write discipline extension to `_shared-context.md`**: Extend §Spec-Path
   Discipline to explicitly cover factory artifact writes. Add a "Write Discipline"
   clause: all `.factory/**` artifact writes (DELIVERY ledger, story-frontmatter
   files, pr-review.md, and any other factory-side record) MUST use absolute paths
   anchored to the canonical main-checkout root, not relative paths from the story
   worktree CWD. Name the DELIVERY ledger and pr-review.md as load-bearing cases.

2. **Teardown preflight in `step-g-cleanup.md`**: Before the `git worktree remove`
   dispatch, add a mandatory preflight sub-step: run
   `find <worktree-path>/.factory -type f 2>/dev/null` and assert the result is
   empty. If any files are found, either relocate them to the canonical `.factory/`
   mount (if they are valid factory artifacts) or fail with a visible error. Only
   proceed with worktree removal after an empty result.

**BC/VP impact.**
E-18 BCs (BC-4.14.001, BC-4.15.001, BC-5.41.001-003, etc.) address context-window
durability (PreCompact flush, HANDOFF.md). They do not address write-path discipline
for story-worktree artifact writes. E-17 BCs address lock semantics for concurrent
writes, not wrong-worktree path resolution. **New BC required** in SS-06 (Skill
Catalog) covering the write-discipline invariant and teardown preflight as required
postconditions of the deliver-story protocol.

**Proposed story boundary.** One story: S-21.04 "story-worktree factory artifact
write-path discipline and teardown preflight." The two changes are mechanically
linked (the teardown preflight catches violations that the write discipline should
prevent; both close the same loss window) and share the same BC anchor.

**Estimated points.** 5 (skill-doc extension with bats integration test verifying
the preflight blocks a force-remove when stray `.factory/` content is present).
Wave placement: Wave 2 (can be done in parallel with S-21.01/02/03 but benefits
from S-21.01's validate-artifact-path WASM for defense-in-depth).

---

### Issue #588 — Factory-side PR strands shared `.factory` worktree on chore branch

**Root-cause statement (mechanism).**
When pr-manager needs to update `factory-artifacts` directly via a PR (e.g., VP
anchor true-up, merge-SHA annotation), it checks the shared `.factory/` worktree
onto a chore branch, commits, creates and merges the PR, then returns to the calling
context — without restoring the worktree to `factory-artifacts`, without fast-forward
pulling the just-merged content, and without deleting the local or remote chore
branch. The shared worktree is left stranded. Subsequent state-manager dispatches
inherit the stranded branch silently; `factory-branch-guard.sh` (a PreToolUse hook
for Edit/Write) blocks explicit write attempts, but Bash-tool git operations (e.g.,
`git -C .factory commit`) bypass it. Damage surfaces only when a later commit
lands on the stray branch (factory history is lost at that branch when the branch
is eventually cleaned up) or when local and origin `factory-artifacts` diverge and
a later agent syncs the wrong way.

**Current codebase verification.**
`plugins/vsdd-factory/agents/pr-manager.md` step 9 ("post-merge") reads: "Trigger
worktree cleanup and state updates. Compile the final deliverables report." There is
no factory-side PR protocol section, no restore-original-branch step, no ff-only
pull step, and no local/remote chore-branch deletion step. The issue comment
(`arcaven`, 2026-07-13) confirms: "pr-manager.md has no factory-side PR flow section
even after the S-19.01 hardening (#613)."
`plugins/vsdd-factory/hooks/factory-branch-guard.sh` exists and correctly blocks
Edit/Write tools when `.factory/` is on the wrong branch. It does NOT catch
Bash-mediated git commit operations. Status: **confirmed open**.

**Solution shape.**
Two layers:

1. **Skill-doc protocol addition to `pr-manager.md`**: Add an explicit "factory-side
   PR protocol" section. This protocol governs any PR that modifies `factory-artifacts`
   directly (e.g., VP anchor true-up, merge-SHA annotation). Required steps:
   (a) Before branching: record the current worktree branch (always `factory-artifacts`
   under normal operation);
   (b) After PR merge: restore the worktree via `git -C .factory checkout factory-artifacts`;
   (c) Fast-forward pull: `git -C .factory pull --ff-only origin factory-artifacts`;
   (d) Delete the local chore branch: `git -C .factory branch -d chore/<name>`;
   (e) Delete the remote chore branch: `git push origin --delete chore/<name>`.
   The PR is not "done" until steps (b)–(e) all complete without error.

2. **Dispatch-preamble branch assertion (defense-in-depth)**: Add to every agent
   prompt that writes to `.factory/` (state-manager, pr-manager factory-side flow):
   a mandatory pre-write assertion "verify `git -C .factory branch --show-current`
   equals `factory-artifacts`; STOP and report if not." This is the operational guard
   already adopted post-incident in the switchboard-blue fleet and should be codified
   as a required precondition in the factory-write dispatch preamble templates.

No new shell script or WASM plugin required for (1) or (2); both are skill-doc
changes. Factory-branch-guard.sh already provides a partial defense for Edit/Write
paths; the gap is Bash-mediated commits, which require the precondition assertion
approach (2).

**BC/VP impact.**
BC-6.23.001 covers `/factory-lock` and `/factory-unlock` skills (SS-06, CAP-031,
ADR-025). It does not cover the pr-manager post-merge worktree-restoration protocol.
**BC amendment required**: BC-6.23.001 should be extended with a new postcondition
covering the factory-side PR protocol (restore-original-branch + ff-only pull +
chore-branch cleanup as required post-merge steps for any PR on `factory-artifacts`),
or a new BC-6.26.001 created for the pr-manager factory-side PR behavioral contract.
Given BC-6.25.001 is the current highest in ss-06 (from the BC list), BC-6.26.001
would be the next available slot.

**Proposed story boundary.** One story: S-21.05 "pr-manager factory-side PR
protocol — restore-original-branch, ff-only sync, and chore-branch cleanup."
Bounded to pr-manager agent definition changes plus the dispatch-preamble template
extension.

**Estimated points.** 5 (skill-doc addition with bats integration test verifying
the protocol steps fire in order). Wave placement: Wave 2 (shares mechanism class
with #523 story-worktree teardown; logically adjacent but independent).

---

## Shared Story Boundary Assessment: #523 and #588

Both issues are worktree-lifecycle failures: #523 is story-worktree teardown
(write-path discipline + teardown preflight), #588 is factory-side PR worktree
abandonment (post-merge restore + cleanup). The mechanisms differ:

- #523 root: agents write to the wrong worktree via relative-path resolution.
- #588 root: pr-manager leaves the shared `.factory/` worktree on the wrong branch
  after a factory-side PR.

The fix vectors are different files (`_shared-context.md` + `step-g-cleanup.md`
vs `pr-manager.md`). The BC anchors will be different (story-worktree write
discipline vs pr-manager factory-side protocol). They should remain **separate
stories** (S-21.04 and S-21.05). The only shared element is the
factory-branch-guard.sh defense-in-depth (already exists), and the dispatch-preamble
branch assertion pattern (both stories should codify the same pre-write assertion
idiom, so they may share a BC anchor for that assertion pattern).

Similarly, #342 (product-branch merge clobber) is distinct from #523/#588 in
mechanism (git working-tree update vs agent write-path discipline) and should remain
a separate story (S-21.01).

---

## Epic-Level Risk Statement

The E-21 issue family represents a single compound invariant violation: **no
`factory-artifacts`-managed path may be modified or destroyed by any operation
that does not go through the canonical `factory-artifacts` write path.** All five
issues are instances of this invariant being silently broken. The risk is HIGH because:

- All five issues produce loss that is either unrecoverable (if the artifact had
  uncommitted edits) or requires forensic reconstruction effort.
- All five failures are silent — no error, no warning, no hook fires at the point
  of loss. Detection is post-hoc and requires either luck (manual audit, reflog
  examination) or a human noticing missing state.
- Three issues (#342, #523, #588) share a common structural root: nested worktrees
  and shared mutable state are not protected at the git/CLI operation boundary.
  The existing `factory-branch-guard.sh` covers the Edit/Write tool surface; the
  Bash-tool git-operation surface is unguarded.

The five-issue cluster also reveals a specification gap: there is no formally
stated invariant in any existing BC that governs the lifecycle of the `.factory/`
worktree as a *shared artifact surface with exclusive-path semantics*. E-17 covers
concurrent-session write races (the lock/lease primitive). E-18 covers context-window
compaction durability. Neither epic has a BC that says "no operation outside the
factory-artifacts write path may alter or destroy the physical content under
`.factory/`." E-21 closes this gap.

---

## Cross-Cutting Invariant Candidates

The following invariants emerge from the five-issue pattern and should be expressed
as BCs in E-21 (rather than as ad-hoc skill-doc notes):

**INV-E21-001 (Nested Worktree Path Exclusivity):** No product-branch git operation
(checkout, merge, rebase, reset) may add or delete a path under the `.factory/`
physical directory. Detectable via git pre-receive or pre-merge hook. Fix: validate-
artifact-path WASM guard (Issue #342 mechanism).

**INV-E21-002 (Factory Artifact Write Canonical-Path Discipline):** Any agent
writing to a factory artifact (DELIVERY ledger, story-frontmatter, pr-review.md,
VP anchors, STATE.md) MUST use an absolute path anchored to the canonical main-
checkout `.factory/` mount — not a worktree-relative path. Detectable at write time.
Fix: skill-doc extension (Issues #523, #588 write-path mechanism).

**INV-E21-003 (Factory Worktree Branch Invariant):** The `.factory/` git worktree
MUST be on `factory-artifacts` at the start and end of every factory write operation.
Detectable via `git -C .factory branch --show-current`. Fix: dispatch-preamble
assertion + pr-manager post-merge restore (Issue #588 primary mechanism; defense-in-depth
for all factory writes).

**INV-E21-004 (Story Worktree Teardown Preflight):** `git worktree remove` on a
story worktree MUST be preceded by an inventory check confirming that no `.factory/`
content exists inside the worktree. If content is found, relocate or hard-fail.
Fix: step-g-cleanup skill-doc preflight (Issue #523 teardown mechanism).

**INV-E21-005 (PR Trunk Ancestry):** Every story PR MUST be verified as an ancestor
of `origin/<trunk>` immediately after merge. A `state=MERGED` PR that is not an
ancestor of trunk is a P0 data error. Fix: pr-manager post-merge ancestry assertion
(Issue #358 primary mechanism).

---

## Verification Strategy Sketch

| Issue | Story | Verification approach |
|-------|-------|-----------------------|
| #342 | S-21.01 | Bats: invoke `git add .factory/foo.md` on a non-factory-artifacts branch while the WASM guard is active; assert guard fires (exit code 2). Integration: run a product-branch merge that includes a `.factory/` delete in the diff; assert the skill-doc gate stops the merge before it runs. |
| #365 | S-21.02 | Bats: simulate a `git rebase` that drops a line from a sibling branch's commit; assert the post-rebase diff gate detects the net-negative delta and surfaces a STOP signal. Proptest: fuzz the 3-way merge context-line adjacency scenario to confirm detection coverage. |
| #358 | S-21.03 | Bats fixtures (mirrors the pattern established for pr-manager in S-19.01): stub `gh pr view` to return a mismatched `baseRefName`; assert pr-manager fails at step 3. Stub `git merge-base` to return exit 1; assert pr-manager fails at step 9. Both assertions must fire before the story is marked delivered. |
| #523 | S-21.04 | Bats: create a stray `.factory/` file inside a story-worktree fixture; invoke the teardown preflight step; assert it exits non-zero and reports the stray file path. Regression test: confirm `git worktree remove` is not invoked if the preflight exits non-zero. |
| #588 | S-21.05 | Bats: mock a factory-side PR flow that ends without restoring the branch; assert the post-merge restore steps fire (`git checkout factory-artifacts`, `git pull --ff-only`, branch deletes). Bats: mock a state where `git branch --show-current` returns a chore branch at dispatch preamble; assert the preamble check STOPs before any write. |

Kani/formal verification is not needed for issues #365, #358, #523, #588 (they are
process protocol gaps, not Rust-level memory or concurrency issues). Issue #342's
WASM guard crate (SS-04) should have a proptest or Kani harness for the path-pattern
matching logic to guard against false negatives on path variants.

---

## Wave Structure and Story Dependency Graph

```
Wave 1 (parallel — no inter-story dependencies within E-21):
  S-21.01  factory artifact path guard (WASM + skill-doc merge gate)       11 pts
  S-21.02  post-rebase diff-integrity gate (skill-doc)                       3 pts
  S-21.03  pr-manager trunk-assertion hardening (skill-doc)                  3 pts

Wave 2 (parallel — benefit from S-21.01 defense-in-depth but not blocked):
  S-21.04  story-worktree write-path discipline + teardown preflight         5 pts
  S-21.05  pr-manager factory-side PR protocol (skill-doc)                   5 pts
```

Wave 1 total: 17 pts. Wave 2 total: 10 pts. Epic total: 27 pts.

Wave 1 rationale: S-21.01 provides the structural WASM guard that prevents dual-
tracking (the precondition that makes #342 dangerous); S-21.02 and S-21.03 are
independent process gaps with no prerequisite. All three can ship in parallel.

Wave 2 rationale: S-21.04 and S-21.05 address the worktree lifecycle surface. They
benefit from S-21.01's WASM guard being active (defense-in-depth), but both are
independently viable without it. They are adjacent in theme (shared-mutable-worktree
lifecycle) but use different artifacts and BCs.

---

## Per-Issue Verdict Table

| Issue | Root-cause status | BC anchor | Story boundary | Points |
|-------|------------------|-----------|----------------|--------|
| #342 product-branch clobber | CONFIRMED OPEN — factory-branch-guard covers Edit/Write, not Bash-tool git merge/checkout; no pre-merge intersection check exists | New BC in SS-04 (validate-artifact-path WASM, PreToolUse Bash gate) + SS-05 amendment (merge gate skill-doc) | S-21.01 Wave 1 | 11 |
| #365 rebase silent drop | CONFIRMED OPEN — no post-rebase diff-integrity step exists in deliver-story or pr-manager; path referenced in issue ("orchestrator-rebase playbook") does not exist in codebase | New BC in SS-05 (post-rebase diff-integrity gate as required protocol step) | S-21.02 Wave 1 | 3 |
| #358 PR base not locked | PARTIALLY FIXED (--base develop in place at pr-manager.md:139) / GAP CONFIRMED OPEN (post-create baseRefName assertion absent; post-merge --is-ancestor check absent) | BC amendment to SS-06 (BC-6.10.002 or new BC covering pr-manager 9-step postconditions PC-N and PC-N+1) | S-21.03 Wave 1 | 3 |
| #523 story-worktree teardown loss | CONFIRMED OPEN — shared-context write-path discipline names reads only; step-g-cleanup has no teardown preflight | New BC in SS-06 (story-worktree write-path discipline + teardown preflight as required postconditions) | S-21.04 Wave 2 | 5 |
| #588 factory-side PR branch strand | CONFIRMED OPEN — pr-manager.md has no factory-side PR protocol section; step 9 has no restore-original-branch logic | BC-6.23.001 amendment or new BC-6.26.001 (pr-manager factory-side PR protocol with restore-original-branch, ff-only pull, chore-branch cleanup) | S-21.05 Wave 2 | 5 |

---

## Constraints and Boundary Notes

This analysis is architecture-only. The following artifacts are explicitly OUT OF
SCOPE for this document and must be authored by their respective owners:

- Behavioral contract file bodies (product-owner)
- Story files and STORY-INDEX rows (story-writer)
- STATE.md updates (state-manager)
- 4-index version bumps (state-manager)

POLICY 21 (no_new_shell_scripts, D-836) is a hard constraint on all E-21 stories.
The validate-artifact-path WASM crate for S-21.01 is already PLANNED in the SS-04
subsystem doc (`crates/hook-plugins/validate-artifact-path/`), making it
policy-compliant. All other E-21 fix vectors are skill-doc amendments requiring
no new runtime artifacts.

E-20 (factory-tools class migration for grandfathered .sh scripts) is the
designated migration anchor for any future consolidation of `factory-branch-guard.sh`
into a WASM plugin. E-21 should NOT attempt to replace `factory-branch-guard.sh`
— it is grandfathered, functional, and out of scope for this epic.
