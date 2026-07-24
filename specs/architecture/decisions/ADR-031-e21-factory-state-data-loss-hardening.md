---
document_type: architecture-decision-record
level: L3
adr_id: ADR-031
version: "1.8"
title: "ADR-031: E-21 factory state data-loss hardening — nested-worktree path exclusivity protection model"
status: accepted
date: 2026-07-19
producer: architect
timestamp: 2026-07-19T00:00:00Z
deciders:
  - architect
supersedes: null
superseded_by: null
traces_to: .factory/specs/architecture/ARCH-INDEX.md
related_adrs:
  - ADR-025 (single-writer factory lock/lease — factory-artifacts write discipline; branch invariant precedent)
  - ADR-026 (wave-boundary checkpoint+reset — lossless context-window transitions; factory-artifacts worktree discipline precedent)
  - ADR-014 (Tier-2 native WASM migration — standing mandate: new hooks MUST be native WASM; POLICY 21)
  - ADR-030 (pr-manager merge-operation integrity enforcement — factory-artifacts commit discipline precedent)
anchors:
  - SS-04
  - SS-05
  - SS-06
subsystems_affected:
  - SS-04
  - SS-05
  - SS-06
last_amended: "(v1.8) — F-S2103-P4-003 closure (architect): §Decision 8 recovery-affordance rationale re-grounded — --delete-branch omission does NOT guarantee intact head branch (GitHub delete_branch_on_merge=true auto-deletes at merge regardless of flags; BC-6.10.002 v1.4+ grounds this fact); recovery affordance preserved by (a) Step 8-post-A ordering guarantee (assertion runs before pr-manager deletion sequence 8b/8c/8d) + (b) deletion-agnostic headRefOid anchor (PR-retained field, survives auto-delete). OBS-P4-2 adjudication note added to §Decision 8 (SS-05/SS-06 split correct; no re-anchoring needed). [Prior: (v1.7) — F-S2103-P2-003 ADR leg: §Decision 8 post-merge ancestry assertion placement corrected Step 9→Step 8-post-A (immediately after merge-state confirmation 8a, before branch deletion 8b/8c/8d; BC-6.10.002 PC3 'immediately after state: MERGED'; F-S2103-P2-001 --delete-branch removal preserves orphan-merge recovery affordance). [Prior: (v1.6) — OBS-P5-1 closure: §Decision 6 gate procedure updated — range-diff PRIMARY detector (step 1a) + git diff --stat BACKUP heuristic (step 1b), consistent with BC-5.44.001 v1.2+ refinement; stale --stat-primary 3-step procedure replaced. [Prior: 2026-07-23 (v1.5) — S-21.01 pass-5 gate (human-approved): §Decision 2 Layer-1 TARGET-AWARE branch detection for CWD-redirection vector (git -C / git -c core.worktree= naming a .factory-class path branch-detects in target dir; block product branch / pass factory-artifacts / fail-open on error). §Rationale: CWD-redirection boundary note added (state-manager canonical git -C .factory workflow preserved; residual server-side origination vector unchanged). ARCH-INDEX v3.25→v3.26. [Prior: 2026-07-19 (v1.4) — pass-4 O-1 (architect): §Consequences duplicate '4.' numbering corrected; 4a/4b lettering used to preserve §Consequences #5 = post-rebase gate host (cited by BC-5.44.001 v1.3 and S-21.02 v1.1 as ADR-031 v1.1 §Consequences #5; renaming second '4.' to '5' would shift current #5 to #6, breaking those cites). ARCH-INDEX v3.10→v3.11. [Prior: 2026-07-19 (v1.3) — F-P2-001 correction (orchestrator counter-evidence accepted): §Decision 2 Layer-2 'EMPTY host-set' retracted; corrected to undocumented ad-hoc orchestrator/operator Bash on main checkout; enforcement site named (per-story-delivery.md main-checkout sync protocol = S-21.01 Layer-2 deliverable); Layer-1 scope confirmed narrow (git add/stage only); server-side origination residual risk documented in §Rationale. [Prior: 2026-07-19 (v1.2) — F-P2 adversary adjudications: §Decision 2 Layer-2 EMPTY host-set (retracted at v1.3); §Decision 7 Four→Five; §Rationale F-P2-007 teardown dispatch-point ruling. Prior metadata continued: §Decision 2 Layer-2 host-set corrected to EMPTY: pr-manager (server-side gh pr merge, excluded by BC-5.43.001 PC3), devops-engineer (rebase on story worktree, .factory/ not mounted there), state-manager (git -C .factory only) all removed; forward-looking mandate documented; (2) §Decision 7 count fixed Four→Five (F-P2-002: CAP-038 count sweep missed at v1.1); (3) §Rationale: F-P2-001 zero-host analysis + F-P2-007 teardown dispatch-point ruling added. [Prior: 2026-07-19 (v1.1) — F-P1 adversary adjudications: on_error block→continue; INV-E21-006 added; §Context #358 corrected; CAP-038 allocated.]]]]"
modified:
  - "2026-07-24 (v1.8)"
  - "2026-07-24 (v1.7)"
  - "2026-07-24 (v1.6)"
  - "2026-07-23 (v1.5)"
  - "2026-07-19 (v1.4)"
  - "2026-07-19 (v1.3)"
  - "2026-07-19 (v1.2)"
  - "2026-07-19 (v1.1)"
  - "2026-07-19 (v1.0)"
---

# ADR-031: E-21 factory state data-loss hardening — nested-worktree path exclusivity protection model

## Context

Five independent failure modes were identified and documented in the E-21 root-cause analysis
(`cycles/v1.0-brownfield-backfill/e-21-arch-delta-analysis.md`):

**Issue #342 — Product-branch merge silently removes a `.factory/` file (INV-E21-001).** When
a `.factory/**` file is accidentally staged and committed on the product branch (dual-tracking
condition), a subsequent `git merge` or `git pull` on the product branch can resolve the path
deletion in favor of the product branch, silently destroying the factory artifact. No existing
runtime guard prevents `git add .factory/**` on the product branch.

**Issue #365 — Rebase auto-merge silently drops production lines (INV-E21-005).** Git's ORT
3-way merge algorithm produces clean rebases that silently delete production lines when Branch A
and Branch B modify adjacent (non-overlapping) regions of the same file. No conflict markers
appear; `git rebase --continue` reports success; the dropped lines are gone from the branch.
No detection gate existed between rebase completion and `git push --force-with-lease`.

**Issue #358 — PR base not locked to trunk; orphan merge possible (INV-E21-006).** The
`pr-manager` agent explicitly passes `--base develop` in its `gh pr create` invocation
(`plugins/vsdd-factory/agents/pr-manager.md` Step 3). However, two post-action assertions are
absent: (a) no post-create step reads back the PR's `baseRefName` to confirm it matches the
configured trunk (`develop`), and (b) no post-merge step runs `git merge-base --is-ancestor` to
confirm the merge commit landed on trunk. Without (a), a modified invocation path that omits
`--base` silently resolves the base from the `gh-merge-base` git config or the GitHub repository
default branch — potentially an off-trunk base. Without (b), an orphan merge (PR merged into the
wrong branch) is not detected until a human audits the commit graph. The concrete issue instance
(`ArcavenAE/akey` PR #14, `arcaven` comment 2026-06-30) confirms the gap.

Note: the stale ARCH-INDEX `[PLANNED]` annotation on `validate-artifact-path/` (a separate doc
hygiene finding discovered during CAP analysis) was corrected independently in ARCH-INDEX v3.07
and is not part of issue #358.

**Issue #523 — Story-worktree shadow `.factory/` misdirects CWD-relative writes (INV-E21-002
+ INV-E21-004).** When a story worktree is created with `git worktree add`, git populates a
shadow `.factory/` subdirectory at `<worktree-path>/.factory/`. An agent writing to
`.factory/**` using a CWD-relative path from the story-worktree root silently mutates the shadow
copy rather than the main-checkout `.factory/` worktree. Additionally, `git worktree remove`
silently deletes the shadow subtree without warning if it contains untracked files.

**Issue #588 — Factory-side PR merge leaves `.factory/` worktree on the wrong branch
(INV-E21-003).** When state-manager creates a chore branch on `factory-artifacts`, PRs it, and
the PR is merged via `gh pr merge`, the local `.factory/` worktree remains checked out on the
chore branch — not on `factory-artifacts`. Subsequent writes from a dispatch that omits the
post-merge restore sequence write to the stale chore branch.

No existing ADR (ADR-001 through ADR-030) covers nested-worktree path exclusivity, ORT
post-rebase integrity, story-worktree write-path discipline, or factory-side PR restore protocol.
ADR-025 (single-writer lock) and ADR-026 (wave-boundary context durability) address related but
distinct concerns.

## Decision

**Decision 1 — E-21 invariant catalog.** Define five cross-cutting invariants as the
authoritative failure-mode registry for the E-21 compound:

- **INV-E21-001 (Nested Worktree Path Exclusivity):** No path under `.factory/**` may appear in
  a product-branch diff. The `factory-artifacts` orphan branch and the product branches
  (develop, main, feature/*, release/*, maintenance/*) are disjoint file trees. Any overlap
  indicates dual-tracking and risks content destruction on merge.

- **INV-E21-002 (Write-Path Anchoring):** All `.factory/**` writes must use canonical absolute
  paths anchored to `$(git -C .factory rev-parse --show-toplevel)`, never CWD-relative paths
  derived from a story-worktree root. Story worktrees contain a shadow `.factory/` subdirectory
  that silently captures CWD-relative writes.

- **INV-E21-003 (Factory Worktree Branch Integrity):** At the time of any `.factory/**` write
  or state-manager dispatch, `git -C .factory branch --show-current` MUST equal
  `factory-artifacts`. A detached `.factory/` worktree (post-PR chore-branch checkout) misdirects
  writes to an inactive branch.

- **INV-E21-004 (Worktree Teardown Preflight):** Before `git worktree remove <story-worktree>`,
  `find <worktree-path>/.factory -type f 2>/dev/null` must confirm the shadow `.factory/`
  directory is empty or absent. The git worktree command silently deletes the shadow subtree.

- **INV-E21-005 (Post-Rebase Diff Integrity):** After any `git rebase`, `git rebase --continue`,
  or `git pull --rebase` on a feature branch, a diff-integrity gate must run before
  `git push --force-with-lease`. The gate detects unverified net-negative line-count deltas in
  files also modified by recently-merged sibling stories on `origin/develop`.

- **INV-E21-006 (PR Trunk Ancestry):** Every story PR MUST be verified as an ancestor of
  `origin/<trunk>` immediately after merge. Additionally, the PR's `baseRefName` MUST be
  confirmed equal to the configured trunk immediately after `gh pr create`. A `state=MERGED`
  PR whose merge commit is not an ancestor of `origin/<trunk>` is a P0 data error — the story
  content did not land on the delivery integration branch. Covers issue #358.

**Decision 2 — Two-layer defense for INV-E21-001.** Defense-in-depth with two independent
layers:

- **Layer 1 — Invariant layer (runtime enforcement):** New native WASM PreToolUse plugin
  `validate-factory-path-staging` (crate: `crates/hook-plugins/validate-factory-path-staging/`,
  tool regex = `^Bash$`, priority = 140, tier = sync, on_error = continue) fires before any Bash
  tool call. The plugin inspects the command for `git add`, `git stage`, or equivalent staging
  operations targeting `.factory/` paths on non-`factory-artifacts` branches and blocks them.
  POLICY 21 compliance: native WASM, not a shell script. `on_error = continue` (fail-open) is
  the required setting — see Rationale for full adjudication (F-P1-002); summary: the spec wins
  (BC-4.16.001 Precondition 3 + Invariant 2 mandate fail-open); blocking all Bash tool use on
  a WASM crash is operationally disproportionate; the two-layer defense absorbs Layer 1 failures
  via Layer 2.
  **CWD-redirection vector (TARGET-AWARE extension; S-21.01 pass-5, human-approved 2026-07-23):**
  Commands of the form `git -C <path> add …` and `git -c core.worktree=<path> add …` bypass the
  plugin's literal-argument path matching when `<path>` names a `.factory`-class directory: the
  staging target differs from the shell CWD, so a purely argument-literal match misses the scope.
  Layer-1 is therefore extended with target-aware branch detection: when the plugin detects a
  `-C` or `-c core.worktree=` value that names a `.factory`-class directory, it branch-detects
  in that target directory (via its existing `exec_subprocess` git allowance) and applies the
  same rule — block on a product branch, pass on `factory-artifacts`, fail-open on subprocess
  error. **Boundary:** state-manager's canonical `git -C .factory add …` workflow on mounted
  checkouts continues to pass unmodified — that workflow operates on the `factory-artifacts`
  branch by invariant (INV-E21-003), so the target-aware check resolves to "pass" without any
  change to state-manager behavior.

- **Layer 2 — Safety-net layer (skill-doc enforcement):** BC-5.43.001 mandates that any
  agent or skill performing a local `git merge` or `git pull` on the main product checkout
  (the checkout where `.factory/` is physically mounted as a nested worktree) MUST run
  `git diff --name-only HEAD..<target-ref>` before the operation. If the result contains any
  path matching `^\.factory/`, the operation MUST be halted with `FactoryPathDeletionInMergeDiff`.
  This layer intercepts pre-existing dual-tracking that formed before the runtime guard was active.
  **CURRENT HOST: undocumented ad-hoc orchestrator/operator Bash on the main product checkout.**
  The orchestrator and operators regularly issue `git pull origin develop`, `git checkout develop`,
  and post-merge sync operations on the main checkout as operational Bash — not documented in any
  single agent protocol file, which is why grep of agent docs finds nothing. This is the precise
  clobber vector issue #342's field report describes.
  Specific documented-protocol exclusions (still apply): (a) `gh pr merge` in pr-manager.md is
  server-side, excluded by BC-5.43.001 PC3; (b) `git rebase origin/develop` in devops-engineer.md
  §Inter-Wave Rebase operates on the story worktree where `.factory/` is NOT mounted; (c)
  state-manager uses only `git -C .factory` on the factory-artifacts branch.
  **Enforcement site:** Add an explicit "main-checkout sync protocol" constraint to
  `orchestrator/per-story-delivery.md` — any `git pull`/`git merge` on the main checkout issued by
  the orchestrator (documented step or ad-hoc cleanup) MUST be preceded by the Layer-2 pre-check.
  This is the S-21.01 Layer-2 deliverable (skill-doc addition to per-story-delivery.md).
  **Layer-1 scope stays narrow (git add/stage only):** Extending the WASM guard to intercept
  `git pull`/`git merge` would blur the two-layer separation; Layer-2 is the correct defense for
  the merge-delivery vector. SW must NOT expand S-21.01 Layer-1 scope to cover pull/merge.
  [F-P2-001 corrected at v1.3: "EMPTY" retracted; corrected to undocumented ad-hoc surface;
  enforcement site named; see §Rationale for server-side origination residual risk analysis.]

**Decision 3 — New WASM crate naming (MANDATORY — do not reuse `validate-artifact-path`).** The
S-21.01 Bash-targeting guard MUST be implemented in a NEW crate:
`crates/hook-plugins/validate-factory-path-staging/`, compiling to
`hook-plugins/validate-factory-path-staging.wasm`.

It MUST NOT reuse or modify the existing `validate-artifact-path` crate or binary. The existing
`validate-artifact-path.wasm` (BC-4.11.001, S-13.01) fires on `^(Edit|Write|MultiEdit)$`
PreToolUse events and validates tool call file paths against
`plugins/vsdd-factory/config/artifact-path-registry.yaml`. Its function is completely disjoint
from the new Bash guard. Sharing the crate name would produce: (a) `hooks-registry.toml`
ambiguity (two entries cannot share a plugin filename), (b) Cargo workspace build conflict (same
output path), and (c) observability confusion (events carrying
`plugin: "validate-artifact-path"` would refer to two functionally different guards).

Consequence for BC-4.16.001: that BC's `§Architecture Anchors` section cites
`crates/hook-plugins/validate-artifact-path/` as the backing crate for the Bash guard — a naming
error from initial BC authoring. The product-owner must amend BC-4.16.001 to reference
`validate-factory-path-staging/` in `§Architecture Anchors` and set the `capability:` frontmatter
field to `CAP-034`.

**Decision 4 — INV-E21-002 + INV-E21-004 enforcement (skill-doc, SS-06).** Both invariants are
enforced via skill-doc mandate in BC-6.26.001. No new WASM plugin or shell script (POLICY 21
satisfied). Two required agent actions:

- **PC1 — Write-path anchoring:** All `.factory/**` writes use canonical absolute paths derived
  from `$(git -C .factory rev-parse --show-toplevel)` (or the equivalent pre-computed absolute
  root), never paths relative to `$(pwd)` when the CWD is a story-worktree root.

- **PC2 — Teardown preflight:** Before `git worktree remove <story-worktree>`, run
  `find <worktree-path>/.factory -type f 2>/dev/null` and confirm the output is empty before
  proceeding.

**Decision 5 — INV-E21-003 enforcement (skill-doc, SS-06).** Factory worktree branch integrity
is enforced via skill-doc mandate in BC-6.27.001. No new WASM plugin or shell script (POLICY 21
satisfied). Two required protocol elements:

- **Dispatch-preamble assertion:** Every state-manager dispatch begins with
  `ASSERT: git -C .factory branch --show-current == "factory-artifacts"` as the first step,
  before any `.factory/**` write.

- **Factory-side PR restore sequence (5 steps):** After any `gh pr merge` from a chore branch
  on `factory-artifacts`:
  1. `git -C .factory checkout factory-artifacts`
  2. `git -C .factory pull --ff-only`
  3. Delete local chore branch (`git -C .factory branch -d <chore>`)
  4. Delete remote chore branch (`git -C .factory push origin --delete <chore>`)
  5. Final assertion: `ASSERT: git -C .factory branch --show-current == "factory-artifacts"`

This invariant is explicitly distinct from CAP-031 (cooperative lock/lease per ADR-025). CAP-031
prevents concurrent developer races on `factory-artifacts`; INV-E21-003 prevents single-developer
writes to the wrong branch after a factory-side PR restore.

**Decision 6 — INV-E21-005 enforcement (skill-doc, SS-05).** Post-rebase diff-integrity is
enforced via skill-doc mandate in BC-5.44.001. No new WASM plugin or shell script (POLICY 21
satisfied). The gate runs between rebase completion and `git push --force-with-lease`:
1a. **Primary detector — `git range-diff` (canonical rebase integrity check):** Run
    `git range-diff <pre-rebase-tip>...<post-rebase-tip>` to compare replayed commits
    before and after the rebase. Any commit pair showing `modified` or `changed` status
    that touches a file also modified by a recently-merged sibling story MUST be inspected
    before proceeding to force-push.
1b. **Backup heuristic — `git diff origin/develop --stat`:** If `git range-diff` is
    unavailable (git < 2.19) or yields inconclusive results, run
    `git diff origin/develop --stat` on the rebased feature branch and check for
    net-negative line-count deltas in files also modified by a recently-merged sibling story.
2. For any file flagged by step 1a or 1b: the agent must explicitly verify the delta is
   intentional. If any such file cannot be confirmed, the gate halts with
   `UnverifiedNetNegativeDelta`; `git push --force-with-lease` is blocked.

**Decision 7 — CAP allocation.** Five new capability entries registered in `capabilities.md`
(v1.8 → v1.9) at the next available IDs after CAP-033:
[F-P2-002 fix: count corrected from "Four" — CAP-038 was added at v1.1 but the aggregation cell was not swept; five is the correct total.]

| CAP-ID | Description | Subsystems | BCs |
|--------|-------------|------------|-----|
| CAP-034 | Factory artifact nested-worktree path exclusivity enforcement (INV-E21-001 two-layer defense) | SS-04, SS-05 | BC-4.16.001, BC-5.43.001 |
| CAP-035 | Post-rebase diff-integrity gate (INV-E21-005) | SS-05 | BC-5.44.001 |
| CAP-036 | Story-worktree write-path discipline and teardown preflight (INV-E21-002 + INV-E21-004) | SS-06 | BC-6.26.001 |
| CAP-037 | Factory worktree branch integrity — dispatch-preamble assertion + factory-side PR restore protocol (INV-E21-003) | SS-06 | BC-6.27.001 |
| CAP-038 | PR trunk ancestry integrity — post-create baseRefName assertion + post-merge ancestry guard (INV-E21-006) | SS-05 | BC-6.10.002 (amendment) |

**Decision 8 — INV-E21-006 enforcement (skill-doc amendment, SS-05).** PR trunk ancestry
integrity is enforced via skill-doc amendment to BC-6.10.002 (the existing orchestrator
deliver-story 9-step protocol BC). No new WASM plugin or shell script (POLICY 21 satisfied).
Two required post-action assertions added to the pr-manager 9-step lifecycle:

- **Post-create baseRefName assertion (Step 3 amendment):** After `gh pr create`, spawn
  `github-ops` to run `gh pr view <num> --json baseRefName` and assert the returned value equals
  the configured trunk (`develop` for greenfield/feature pipelines). Hard-fail the burst if
  mismatched. This catches cases where `--base` was omitted in a modified invocation path, or
  where `gh` resolved the base from the `gh-merge-base` git config rather than the intended trunk.

- **Post-merge ancestry assertion (Step 8-post-A):** Immediately after `gh pr merge` confirms
  `state: MERGED` (Step 8a), spawn `github-ops` to assert: (i) `mergeCommit.oid` is non-null,
  and (ii) `git fetch origin <trunk> && git merge-base --is-ancestor <mergeCommit.oid> origin/<trunk>`
  exits 0. If either check fails, raise immediately as a P0 data error and HALT — branch
  deletion steps (8b/8c/8d) MUST NOT proceed until this assertion passes. Placement per
  BC-6.10.002 PC3: "immediately after state: MERGED" mandates Step 8-post-A, not Step 9.
  The orphan-merge recovery affordance is preserved by two deletion-agnostic mechanisms:
  (a) the Step 8-post-A assertion runs before pr-manager's own deletion sequence
  (ordering guarantee — HALT enforced before 8b/8c/8d executes), and (b) recovery
  anchors on the PR-retained `headRefOid` field, which GitHub preserves in PR metadata
  even when the head branch is auto-deleted. Per F-S2103-P2-001, `--delete-branch` is
  omitted from the merge invocation; this ensures pr-manager does not initiate deletion
  pre-assertion, but it does NOT guarantee an intact head branch — GitHub
  `delete_branch_on_merge=true` auto-deletes at merge regardless of whether
  `--delete-branch` was passed (confirmed on this repository; BC-6.10.002 v1.4+ reflects
  this grounded fact).

This invariant is distinct from CAP-033 (READY-verdict SHA pinning and release-branch
merge-strategy guard per ADR-030), which addresses stale-verdict races and squash-merge
prevention. INV-E21-006 addresses PR base targeting and post-merge ancestry confirmation — a
different failure class. Capability: CAP-038. Story: S-21.03.

**SS-05/SS-06 subsystem allocation (OBS-P4-2 adjudication):** The "(SS-05)" label in this
Decision's heading refers to the enforcement-agent plane — pr-manager is an SS-05 (Pipeline
Orchestration) agent. BC-6.10.002's `subsystem: SS-06` (Skill Catalog) is correct and
unchanged: the deliver-story skill is an SS-06 artifact, and its governing BC anchors to
SS-06 per the VSDD BC-S.SS.NNN naming convention. CAP-038's SS-05 assignment in §Decision 7
registers the capability's enforcement-agent location, not the contract host. Story S-21.03's
`subsystems: [SS-06]` is likewise correct; the primary deliverable is the BC-6.10.002
amendment, an SS-06 artifact. The split — contract in SS-06, enforcement agent in SS-05 — is
the correct architectural expression of an SS-06 skill delegating to an SS-05 agent with
protocol guarantees; no re-anchoring is required.

## Rationale

**Two-layer vs. single-layer for INV-E21-001:** A WASM guard alone (Layer 1 only) cannot
retrospectively intercept pre-existing dual-tracking that formed before the guard was deployed.
A skill-doc mandate alone (Layer 2 only) cannot prevent an automated agent from running
`git add .factory/**` without the pre-check when the procedure is not followed. Only both layers
together provide full coverage: the WASM guard prevents new dual-tracking at write-time; the
skill-doc merge pre-check intercepts pre-existing dual-tracking before it causes data loss.

**Skill-doc for INV-E21-002/003/004/005:** These four invariants govern agent procedure, not
tool call content. Skill-doc mandates with accompanying BCs are the correct enforcement mechanism
for behavioral constraints of this class. POLICY 21 compliance is automatically satisfied: no
new shell scripts.

**`on_error = continue` for the Bash guard (Decision 2, F-P1-002 adjudication):** The spec wins
(Standing Rule for VSDD): BC-4.16.001 Precondition 3 and Invariant 2 explicitly mandate
`on_error = "continue"` with rationale: "a broken guard is disruptive but never wedges the
session." Three additional factors confirm fail-open: (a) BC-4.16.001 Precondition 4 documents
that on mounted worktrees `git add .factory/` is already a silent no-op (git itself refuses
staging from a nested worktree path), so the guard's primary value is on unmounted checkouts (CI,
bare clones); (b) the two-layer defense model means a Layer 1 crash is caught by Layer 2
(BC-5.43.001 pre-merge intersection check), which is independent of the WASM; (c) a fail-closed
`^Bash$` hook would block ALL Bash tool use on a WASM crash — an operational catastrophe
disproportionate to the marginal risk on mounted developer sessions. ADR-031 v1.0 incorrectly
specified `on_error = block`; corrected at v1.1.

**Crate naming (Decision 3):** The naming decision is production-grade under the "no MVP
deferrals" principle. Reusing `validate-artifact-path` would be the cheap path; creating a
distinct crate with a semantically accurate name is the correct path and avoids the concrete
failure modes enumerated in Decision 3.

**Teardown preflight at dispatch-point vs. co-location (Decision 2, F-P2-007 adjudication):**
The Layer-2 teardown preflight for INV-E21-004 (BC-6.26.001 / S-21.04) is hosted in
`step-g-cleanup.md` (the dispatch-point skill-doc), NOT co-located in `devops-engineer.md
§Worktree Cleanup`. Ruling: keep dispatch-point gating. Rationale: (1) `step-g-cleanup.md` is
the factory-specific skill that owns the teardown protocol and is the correct host for
factory-invariant assertions; (2) `devops-engineer.md` is a general-purpose agent whose
§Worktree Cleanup section describes developer workflow mechanics — embedding `.factory/`
inventory logic there couples it to factory-specific knowledge it should not own; (3) this is
NOT symmetric with F-P1-006 (which moved the rebase gate INTO devops-engineer §Inter-Wave
Rebase): the rebase gate guards a developer-workflow operation performed by devops-engineer
itself (same agent, same context), while the teardown preflight is a CALLER-SIDE
factory-invariant check that step-g-cleanup.md performs before dispatching devops-engineer —
the caller-callee split is intentional.
[Closes adversary F-P2-007.]

**Layer-2 live-surface analysis (Decision 2, F-P2-001 corrected at v1.3):**
The v1.2 "EMPTY host-set" ruling was incorrect. It searched documented agent protocol files
and found nothing — but the live surface is not protocol files; it is ad-hoc orchestrator/
operator Bash on the main product checkout. The orchestrator regularly issues
`git pull origin develop` for post-merge sync and resume operations. These are operational,
not codified in a single agent doc. This was the exact gap issue #342's field report describes.

**Server-side origination residual risk:** BC-4.16.001's Layer-1 WASM guard intercepts Bash-tool
`git add`/`git stage` calls. It does NOT intercept a contributor PR adding `.factory/`-pathed
files merged server-side via GitHub — no local Bash call fires, so the guard never triggers.
The next local `git pull origin develop` on the main checkout then delivers the tracked
`.factory/` content to the working tree. This is the primary threat vector Layer-2 guards
against: it must intercept pre-existing dual-tracking regardless of how that tracking formed
(local or server-side). BC-4.16.001 Precondition 4 (CI/bare-clone coverage) provides partial
mitigation for CI pipelines but does not cover the server-side origination path. This is an
acknowledged residual risk for Layer-1's coverage perimeter.

**Layer-1 scope must stay narrow:** Extending the WASM guard to intercept `git pull`/`git merge`
commands would blur the two-layer separation. Layer-1 prevents new dual-tracking at write-time
(git add); Layer-2 intercepts pre-existing dual-tracking before merge delivery. These are
independent and complementary. Merging them into Layer-1 makes the WASM guard responsible
for runtime diff analysis before every pull/merge, which is operationally expensive and
architecturally conflates the two defense layers. SW must NOT expand S-21.01 Layer-1 scope.

**Enforcement site for Layer-2:** Add a mandatory protocol constraint to
`orchestrator/per-story-delivery.md` under a "Main-Checkout Sync Protocol" heading: before
any `git pull`/`git merge` on the main product checkout, the orchestrator MUST run the
Layer-2 pre-check. This makes the requirement visible in the canonical orchestrator playbook.
This is the S-21.01 Layer-2 deliverable alongside the WASM crate.
[Closes F-P2-001 correction; PO: BC-5.43.001 framing should reflect ad-hoc Bash surface
and server-side origination vector; SW: S-21.01 must include AC for adding the Layer-2
pre-check constraint to per-story-delivery.md.]

**CWD-redirection vector boundary (Decision 2 TARGET-AWARE extension, v1.5):**
The target-aware branch detection in Layer-1 preserves state-manager's canonical
`git -C .factory` workflow on mounted checkouts without modification: since that workflow
invariably operates on the `factory-artifacts` branch (INV-E21-003; enforced by BC-6.27.001),
the target-aware check resolves to a pass for every state-manager staging call. The Layer-1
extension does not alter Layer-2's coverage: the residual server-side origination vector
(contributor PRs merging `.factory/`-pathed files server-side, bypassing Layer-1 entirely)
remains the primary Layer-2 threat vector and is unchanged by this extension.

## Consequences

1. **S-21.01 deliverable:** Must create `crates/hook-plugins/validate-factory-path-staging/`
   (new crate) and register `validate-factory-path-staging.wasm` in `hooks-registry.toml` with
   `event = "PreToolUse"`, `tool = "^Bash$"`, `priority = 140`, `on_error = "continue"`. MUST NOT
   modify the existing `validate-artifact-path` crate.

2. **BC-4.16.001 amendment (product-owner):** The `capability:` frontmatter field must be
   updated from `"TBD"` to `"CAP-034"`. The `§Architecture Anchors` section must be amended to
   reference `crates/hook-plugins/validate-factory-path-staging/` (not `validate-artifact-path/`)
   as the crate backing the Bash guard.

3. **BC capability field backfill (product-owner):** All five BCs must have `capability:`
   updated from `"TBD — E-21 CAP pending ARCH-INDEX registration by architect"` to:
   BC-4.16.001 → `"CAP-034"`, BC-5.43.001 → `"CAP-034"`, BC-5.44.001 → `"CAP-035"`,
   BC-6.26.001 → `"CAP-036"`, BC-6.27.001 → `"CAP-037"`.

4a. **ARCH-INDEX correction:** SS-04 module listing stale `[PLANNED]` annotation on
   `validate-artifact-path/` corrected (crate exists since S-13.01); new entry
   `validate-factory-path-staging/` [PLANNED S-21.01] added. Incorporated in ARCH-INDEX v3.07.

4b. **BC-6.10.002 L2 Capability field (product-owner):** The existing BC-6.10.002 (orchestrator
   9-step deliver-story sequence) must have its `capability:` frontmatter field set to `"CAP-038"`.
   The trunk-ancestry assertions added by S-21.03 (Decision 8 of this ADR) are the implementation
   of INV-E21-006 in the deliver-story protocol.

5. **Post-rebase gate host surface (F-P1-006 ruling):** The diff-integrity gate for INV-E21-005
   (BC-5.44.001) belongs in `plugins/vsdd-factory/agents/devops-engineer.md` §Inter-Wave Rebase
   — between `git rebase origin/develop` and `git push --force-with-lease`. This is the only
   location in the current codebase where a rebase + force-with-lease sequence exists.
   `pr-manager.md` Step 8 = "Execute merge" (not a rebase step). The product-owner must amend
   BC-5.44.001 §Architecture Anchors from "pr-manager.md step 8" to
   "devops-engineer.md §Inter-Wave Rebase." Story-writer must update S-21.02 ACs accordingly.

6. **POLICY 21 attestation:** All four skill-doc BCs (BC-5.43.001, BC-5.44.001, BC-6.26.001,
   BC-6.27.001) introduce no new shell scripts. BC-4.16.001 Layer 1 (the new WASM guard) uses
   native WASM per ADR-014 standing mandate. BC-6.10.002 amendment is skill-doc only.

## Alternatives Considered

**A1 — Single WASM crate covering both Bash and Edit/Write/MultiEdit:** Rejected. The two guards
have disjoint trigger conditions, disjoint payloads, and disjoint logic. Coupling them creates a
single point of WASM fuel exhaustion, ambiguous observability events, and no clean separation of
timeout configs in hooks-registry.toml.

**A2 — Shell script for the Bash git-staging guard:** Rejected. POLICY 21 (D-836
no_new_shell_scripts) mandates native WASM for all new mechanics. A shell hook requires the
legacy-bash-adapter and does not run natively on Windows.

**A3 — Skill-doc only for INV-E21-001 (no WASM guard):** Rejected. Skill-doc alone cannot
prevent automated agents from running `git add .factory/**` without the pre-check. The WASM
guard provides runtime enforcement independent of agent procedure discipline.

**A4 — Reuse `validate-artifact-path` crate name for the Bash guard:** Rejected per Decision 3
rationale. Naming collision produces ambiguous registry entries, Cargo build conflict, and
observability confusion.

## Source / Origin

E-21 architect delta analysis (`cycles/v1.0-brownfield-backfill/e-21-arch-delta-analysis.md`);
issues #342, #365, #358, #523, #588.

## ARCH-INDEX subsystem

SS-04 (Plugin Ecosystem — `validate-factory-path-staging` WASM plugin, new crate planned for
S-21.01), SS-05 (Pipeline Orchestration — BC-5.43.001 merge pre-check and BC-5.44.001
post-rebase gate), SS-06 (Skill Catalog — BC-6.26.001 write-path discipline and BC-6.27.001
factory-side PR restore protocol).

## Traceability

| Field | Value |
|-------|-------|
| CAP-034 | Factory artifact nested-worktree path exclusivity enforcement (BC-4.16.001 + BC-5.43.001) |
| CAP-035 | Post-rebase diff-integrity gate (BC-5.44.001) |
| CAP-036 | Story-worktree write-path discipline and teardown preflight (BC-6.26.001) |
| CAP-037 | Factory worktree branch integrity — dispatch-preamble assertion + factory-side PR restore (BC-6.27.001) |
| CAP-038 | PR trunk ancestry integrity — post-create baseRefName assertion + post-merge ancestry guard (BC-6.10.002 amendment) |
| E-21 | Epic anchor for this ADR |
| ADR-031 | This record |

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.8 | 2026-07-24 | F-S2103-P4-003 closure (architect): §Decision 8 recovery-affordance rationale re-grounded — `--delete-branch` omission does not guarantee intact head branch (GitHub `delete_branch_on_merge=true` auto-deletes regardless of flags; BC-6.10.002 v1.4+ grounds this); recovery affordance preserved by (a) Step 8-post-A ordering guarantee (assertion before pr-manager deletion sequence 8b/8c/8d) and (b) deletion-agnostic `headRefOid` anchor (PR-retained field, survives auto-delete). OBS-P4-2 subsystem adjudication note added to §Decision 8 (SS-05/SS-06 split correct; no re-anchoring needed). ARCH-INDEX bump → state-manager follow-up. |
| 1.7 | 2026-07-24 | F-S2103-P2-003 ADR leg (architect): §Decision 8 post-merge ancestry assertion placement corrected — Step 9 → Step 8-post-A (immediately after merge-state confirmation 8a, before branch deletion 8b/8c/8d). BC-6.10.002 PC3 mandate ("immediately after state: MERGED") is SoT; "Step 9" was stale placement. Null-mergeCommit.oid guard and HALT-before-deletion made explicit. F-S2103-P2-001: `--delete-branch` removed from merge invocation; orphan-merge recovery affordance preserved. |
| 1.6 | 2026-07-24 | OBS-P5-1 closure (architect): §Decision 6 gate procedure updated — `git range-diff <pre-rebase-tip>...<post-rebase-tip>` promoted to PRIMARY detector (step 1a); `git diff origin/develop --stat` demoted to BACKUP heuristic (step 1b, git < 2.19 or range-diff inconclusive); consistent with BC-5.44.001 v1.2+ refinement. Stale 3-step `--stat`-primary procedure replaced with 2-step 1a/1b structure. Skill-doc-mandate framing preserved. |
| 1.5 | 2026-07-23 | S-21.01 pass-5 gate (human-approved). §Decision 2 Layer-1 extended with TARGET-AWARE branch detection for CWD-redirection vector: `git -C <path>` and `git -c core.worktree=<path>` forms now branch-detect in the target dir when `<path>` names a `.factory`-class directory (block product branch / pass factory-artifacts / fail-open on error). §Rationale: boundary note added — state-manager canonical `git -C .factory` workflow preserved; residual server-side origination vector unchanged. ARCH-INDEX v3.25→v3.26. |
| 1.4 | 2026-07-19 | pass-4 O-1 (architect). §Consequences duplicate '4.' numbering corrected via 4a/4b lettering: first item 4 renamed 4a (ARCH-INDEX correction); second item 4 renamed 4b (BC-6.10.002 L2 Capability field). 4a/4b lettering chosen to preserve §Consequences #5 = post-rebase gate host (cited by BC-5.44.001 v1.3 + S-21.02 v1.1 as "ADR-031 v1.1 §Consequences #5"; monotonic renumber would shift #5→#6 breaking those cites). ARCH-INDEX v3.10→v3.11. |
| 1.3 | 2026-07-19 | F-P2-001 correction (orchestrator counter-evidence accepted). §Decision 2 Layer-2 "EMPTY host-set" retracted — corrected to "undocumented ad-hoc orchestrator/operator Bash on main checkout." Enforcement site named: per-story-delivery.md main-checkout sync protocol constraint = S-21.01 Layer-2 deliverable. Layer-1 scope confirmed narrow (git add/stage only; no extension to pull/merge). §Rationale: server-side origination residual risk documented (contributor PR server-side merge bypasses Layer-1; Layer-2 is primary guard for that vector). |
| 1.2 | 2026-07-19 | F-P2 adversary adjudications (architect). §Decision 2 Layer-2 host-set corrected to EMPTY: pr-manager, devops-engineer (story worktree, .factory/ not mounted), state-manager removed from named-host list; forward-looking mandate + individual exclusion rationale documented (F-P2-001). §Decision 7 count fixed Four→Five (F-P2-002: CAP-038 count not swept at v1.1). §Rationale: F-P2-001 zero-host analysis + F-P2-007 teardown dispatch-point ruling (keep dispatch-point gating; not symmetric with F-P1-006 co-location) added. ARCH-INDEX v3.07→v3.08 (F-P2-004 companion bump). |
| 1.1 | 2026-07-19 | F-P1 adversary adjudications (architect). INV-E21-006 (PR Trunk Ancestry) appended to Decision 1 catalog; Decision 8 added (INV-E21-006 enforcement; BC-6.10.002 amendment; CAP-038 allocated). on_error corrected block→continue (fail-open; spec-wins; BC-4.16.001 PC3+Inv2 authoritative; two-layer defense absorbs Layer 1 crash; blocking all Bash disproportionate). §Context issue #358 corrected (stale-annotation premise wrong; real: PR base not locked, post-create baseRefName + post-merge --is-ancestor assertions absent). §Consequences: BC-6.10.002 CAP-038 ruling + F-P1-006 host-surface ruling (devops-engineer.md §Inter-Wave Rebase) added. |
| 1.0 | 2026-07-19 | Initial authorship (architect; E-21 factory state data-loss hardening; issues #342, #365, #358, #523, #588). 6 invariants INV-E21-001..INV-E21-006 as corrected (INV-E21-005=Post-Rebase, INV-E21-006=PR Trunk Ancestry). 8 decisions. CAP-034..CAP-038 allocated. ARCH-INDEX v3.06→v3.07. capabilities.md v1.8→v1.9. |
