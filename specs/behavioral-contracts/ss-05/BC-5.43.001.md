---
document_type: behavioral-contract
level: L3
version: "1.2"
status: draft
producer: product-owner
timestamp: 2026-07-19T00:00:00Z
phase: brownfield-backfill
cycle: v1.0-brownfield-backfill
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/e-21-arch-delta-analysis.md
  - plugins/vsdd-factory/skills/factory-health/SKILL.md
  - plugins/vsdd-factory/skills/deliver-story/steps/step-f-pr-lifecycle.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.16.001.md
input-hash: "TBD"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: null
subsystem: "SS-05"
capability: "CAP-034"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill
modified:
  - "2026-07-19 (v1.1) — CAP-034 backfill (product-owner; ARCH-INDEX v3.07): capability frontmatter TBD→CAP-034; §Traceability L2 Capability TBD→CAP-034; Capability Anchor Justification updated to cite CAP-034/ARCH-INDEX v3.07."
  - "2026-07-19 (v1.2) — Research validation precision amendments (product-owner; research validation 2026-07-19): §Description expanded with loss-mode precision (silent delete only when on-disk content matches tracked blob; uncommitted divergence causes git abort — reframed as defense-in-depth for matching-content case); §Description + Invariant 5 added with pre-check semantics precision (HEAD..<target> = endpoint comparison, not merge preview; over-halts conservatively; git merge-tree cited as more precise alternative)."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-5.43.001
section: "5.43"
last_amended: "(v1.2) — Research validation precision amendments (product-owner; research validation 2026-07-19): §Description loss-mode precision + pre-check semantics note; Invariant 5 added. [Prior: (v1.1) — CAP-034 backfill. (v1.0) — Initial authoring; orchestrator merge safety gate; INV-E21-001 safety-net layer. lifecycle_status: draft (POL-14).]"
---

# BC-5.43.001: orchestrator MUST run a `.factory/` path-intersection pre-check before executing any `git merge`, `git pull`, or `git checkout` on the product branch, and MUST STOP if the target tree diff contains a `.factory/` path deletion

## Description

This BC governs the **safety-net layer** for issue #342 (product-branch merge silently `rm`s a
`.factory/` file the nested worktree is serving). It is the companion to BC-4.16.001 (the invariant
layer: `validate-factory-path-staging` WASM guard that prevents dual-tracking at `git add` time).

Whenever the orchestrator or any specialist agent (devops-engineer, pr-manager, state-manager) is
about to execute a `git merge`, `git pull`, or `git checkout` command that changes the HEAD of the
product branch (develop/main/feature/release/maintenance), it MUST first run a path-intersection
pre-check. The pre-check inspects the diff between the current HEAD and the target ref:

```
git diff --name-only HEAD..<target-ref>
```

If the result contains any path matching `^\.factory/` (a `.factory/`-rooted path), the operation
MUST be halted immediately with a visible error. Proceeding without resolution risks silently
destroying content in the nested `.factory/` worktree.

**Loss-mode precision:** The silent delete scenario occurs specifically when the on-disk content
of the `.factory/` file on the product branch matches the blob that develop tracked (the common
starting state of a dual-tracked file that was accidentally staged before BC-4.16.001 was active).
In that case, git's working-tree update proceeds silently with no conflict marker, and the file is
deleted from the `.factory/` physical tree without warning. If the file has uncommitted divergence
(on-disk differs from the tracked blob), git instead ABORTs the merge with "error: Your local
changes to the following files would be overwritten by merge" — which is unpleasant but safe; no
data is lost. This BC therefore provides defense-in-depth for the matching-content silent-delete
case, which is the most dangerous scenario and the one that gives no user-visible signal.

**Rationale for new BC rather than SS-05 amendment:** No existing SS-05 BC governs the
orchestrator's product-branch merge or checkout pre-check step. BC-5.41.001 governs wave-gate
HANDOFF.md writing; BC-5.42.001 governs pr-manager READY-verdict enforcement; neither covers the
orchestrator's merge-precondition step for product branches. Creating BC-5.43.001 as a new BC is
the production-grade choice — the correct way to specify a new mandatory protocol step is to write
the spec for it, not to shoehorn it into a BC with a different behavioral focus.

**Scope note:** This BC covers skill-doc mandates (no new WASM plugin or shell script is required;
POLICY 21 satisfied). The pre-check is expressed as a required orchestrator action documented in
skill step instructions for all agents that perform product-branch merge/pull/checkout operations.

## Preconditions

1. The orchestrator or a specialist agent is about to execute a Bash command that changes the HEAD
   of the product branch via `git merge`, `git pull`, `git pull --ff-only`, `git pull --rebase`,
   or `git checkout <target>` (where `<target>` results in a working-tree update on the product
   branch).

2. The `.factory/` directory is mounted as a git worktree nested inside the product branch's
   working directory (the standard vsdd-factory layout confirmed by `git worktree list`).

3. The `<target-ref>` (the ref being merged/pulled/checked-out) is resolvable by git.

## Postconditions

### PC1 — Safe: no `.factory/` path deletion in target diff → merge/pull/checkout proceeds

When `git diff --name-only HEAD..<target-ref>` returns no line matching `^\.factory/`, the merge,
pull, or checkout operation MUST proceed normally. No additional gate fires. The pre-check is
transparent to normal operations.

### PC2 — Halted: `.factory/` path deletion detected → STOP with actionable error

When `git diff --name-only HEAD..<target-ref>` returns one or more lines matching `^\.factory/`,
the operation MUST be halted BEFORE the merge/pull/checkout command is executed. The agent MUST
emit an actionable error:

```
HALTED: Product-branch merge/checkout would modify .factory/ path(s) owned by the
factory-artifacts worktree.

Paths at risk:
  <path1>
  <path2>
  ...

This indicates a dual-tracking condition (issue #342 class): these paths are tracked
on both the product branch and the factory-artifacts worktree. Proceeding would
silently destroy factory artifact content.

Required actions before proceeding:
  1. On the source branch: ensure these paths are NOT tracked on the product branch
     (`git rm --cached <path>` per companion issue #341).
  2. Verify the factory-artifacts worktree is not serving uncommitted edits to these paths.
  3. Once the dual-tracking is resolved, the merge/checkout is safe to retry.
```

The merge, pull, or checkout command MUST NOT be executed after this error. The agent
escalates to the orchestrator for human review.

**Error variant:** `FactoryPathDeletionInMergeDiff`

### PC3 — Pre-check skipped for non-working-tree operations

Git commands that do NOT modify the working tree of the product branch (e.g., `git fetch`,
`git log`, `git diff`, `git status`, `git stash`, commits on `factory-artifacts`) are NOT in scope
for this pre-check. The pre-check applies ONLY to commands that advance or change HEAD of the
product branch working tree.

## Invariants

1. **Pre-check MUST run before the merge command, not after.** A post-merge check cannot recover
   lost content. The diff command MUST be invoked as the mandatory step immediately preceding
   the merge/pull/checkout command in the orchestrator's protocol.

2. **`.factory/` path presence anywhere in the diff is sufficient to halt.** Even if the diff
   shows an addition (not a deletion) of a `.factory/` path on the product branch, the operation
   is halted — any `.factory/` path in a product-branch diff is anomalous and indicates the
   dual-tracking precondition exists.

3. **This gate supplements, not replaces, BC-4.16.001.** BC-4.16.001 prevents new dual-tracking
   from forming via `git add`. This BC intercepts a merge/pull/checkout that would clobber paths
   already in a dual-tracked state (from a prior accidental staging before BC-4.16.001 was active).
   Both gates are required for defense-in-depth.

4. **Fail-open on git command failure:** If `git diff --name-only HEAD..<target-ref>` fails (network
   error on a remote ref, unresolvable ref), the agent MUST log a warning and proceed with the
   merge/pull/checkout. An unresolvable pre-check is not a blocking condition — it means the pre-check
   could not run, not that a hazard was detected.

5. **Pre-check semantics — endpoint comparison, not merge preview:** `git diff --name-only HEAD..<target-ref>`
   compares the two endpoint snapshots (equivalent to `git diff HEAD <target-ref>`); it is NOT a
   merge preview (which would require `git merge-tree HEAD HEAD <target-ref>` or `git merge --no-commit --no-ff`).
   This means the pre-check conservatively over-halts: a `.factory/` path that appears in the
   endpoint diff but would NOT be written by the actual ORT merge algorithm (because it is identical
   on both sides or already in the merge base) triggers a halt. This is the correct tradeoff — the
   pre-check never misses a genuine hazard, but implementers should know it will occasionally produce
   false-positive halts on paths that are actually safe to merge. If over-halting proves noisy in
   practice, `git merge-tree HEAD HEAD <target-ref>` (git ≥ 2.38) is the more precise alternative
   and can be substituted for the pre-check without changing the postcondition semantics.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `git pull origin develop` — diff shows only `src/lib.rs` changed | Pre-check passes (PC1); merge proceeds |
| EC-002 | `git merge feature/S-21.01` — diff shows `.factory/stories/S-21.01.md` deleted | HALTED: PC2; `FactoryPathDeletionInMergeDiff` error; agent escalates |
| EC-003 | `git diff --name-only` fails (remote unreachable) | Logged warning; merge proceeds (Invariant 4) |
| EC-004 | `git fetch origin develop` (no working-tree change) | Pre-check NOT required (PC3 scope exception) |
| EC-005 | `git checkout factory-artifacts` (switching worktree, not product branch) | Pre-check NOT required (not a product-branch working-tree update) |
| EC-006 | Diff shows `.factory/STATE.md` ADDED on product branch (not deleted) | HALTED: PC2 + Invariant 2; any `.factory/` path in product-branch diff is anomalous |
| EC-007 | Diff contains hundreds of files, `.factory/` path among them | HALTED: PC2; the grep check is O(N) on file count; no performance concern for typical PR sizes |

## Canonical Test Vectors

| Test # | Precondition | Action | Expected Result |
|--------|-------------|--------|----------------|
| T-1 | Target diff contains `src/lib.rs` only | `git merge feature/S-21.01` | Pre-check passes; merge proceeds |
| T-2 | Target diff contains `.factory/STATE.md` | `git merge feature/S-21.01` | HALTED: `FactoryPathDeletionInMergeDiff` |
| T-3 | Target diff contains `src/lib.rs` + `.factory/stories/S-21.01.md` | `git pull origin develop` | HALTED: mixed diff, `.factory/` path detected |
| T-4 | `git fetch origin develop` (no working-tree change) | fetch only | Pre-check skipped; fetch proceeds |
| T-5 | `git diff` fails (remote timeout) | `git pull` | Warning logged; pull proceeds (fail-open) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD) | Skill step includes mandatory pre-check invocation | manual: confirm pre-check step present in all product-branch merge/checkout agent protocols in S-21.01 skill-doc deliverable |
| (TBD) | Pre-check halts on `.factory/` path detection | bats: mock `git diff --name-only` returning `.factory/STATE.md`; assert merge command not invoked + `FactoryPathDeletionInMergeDiff` error |
| (TBD) | Pre-check passes on clean diffs | bats: mock `git diff --name-only` returning only `src/lib.rs`; assert merge command proceeds |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-034 |
| Capability Anchor Justification | CAP-034 registered in ARCH-INDEX v3.07 (ADR-031, commit 14a78515): "Nested Worktree Path Exclusivity — factory-artifacts paths may not be staged or merged on the product branch." BC-4.16.001 (invariant layer, `validate-factory-path-staging` WASM) and BC-5.43.001 (safety-net layer, orchestrator merge pre-check) together implement CAP-034 defense-in-depth. |
| L2 Domain Invariants | none (operational infrastructure) |
| Architecture Module | `plugins/vsdd-factory/skills/deliver-story/steps/` (skill-doc amendment by S-21.01); `plugins/vsdd-factory/agents/pr-manager.md` (amendment); orchestrator merge protocol templates |
| Stories | S-21.01 (E-21 Wave 1) |
| Source Issues | #342 (product-branch merge silently rm's a `.factory/` file) |
| ADR Reference | none |

## Related BCs

- BC-4.16.001 — invariant layer companion; prevents new dual-tracking at `git add` time; this BC is the safety-net for pre-existing dual-tracking
- BC-6.10.002 — governs deliver-story 9-step sequence; this BC adds a mandatory pre-merge pre-check to the orchestration protocol at the SS-05 (Pipeline Orchestration) layer

## Architecture Anchors

- `plugins/vsdd-factory/skills/deliver-story/steps/` — skill-doc step files requiring the pre-check (to be amended by S-21.01)
- `plugins/vsdd-factory/agents/pr-manager.md` — merge-step amendment required (to be amended by S-21.01)
- `plugins/vsdd-factory/skills/factory-health/SKILL.md` — factory-health pre-merge check step addition (to be amended by S-21.01)

## Story Anchor

S-21.01 (E-21 Wave 1 — factory artifact path guard: prevent dual-tracking and intercept product-branch merges that would clobber `.factory/` paths)

## VP Anchors

TBD — VP IDs to be assigned after VP authoring pass.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-07-19 | Initial authoring (product-owner; E-21 factory-state data-loss hardening; issue #342). Orchestrator product-branch merge safety gate: mandatory `git diff --name-only HEAD..<target>` pre-check (PC1 pass/PC2 halt); safety-net layer for INV-E21-001. 1 error variant: `FactoryPathDeletionInMergeDiff`. 7 edge cases EC-001..EC-007. 5 test vectors T-1..T-5. lifecycle_status: draft (POL-14 auto-promotion on S-21.01 PR merge). |
| 1.2 | 2026-07-19 | Research validation precision amendments (product-owner; research validation 2026-07-19). §Description: loss-mode precision added (silent delete only when on-disk content matches tracked blob; uncommitted divergence causes git abort — defense-in-depth for matching-content case explicitly stated). Invariant 5 added: pre-check semantics — `git diff --name-only HEAD..<target>` is endpoint comparison, not merge preview; conservatively over-halts (never misses); `git merge-tree` cited as precise alternative. |
| 1.1 | 2026-07-19 | CAP-034 backfill (product-owner; ARCH-INDEX v3.07, ADR-031, commit 14a78515): capability frontmatter TBD→CAP-034; §Traceability L2 Capability TBD→CAP-034; Capability Anchor Justification updated to cite CAP-034/ARCH-INDEX v3.07; Description crate-name corrected (`validate-artifact-path`→`validate-factory-path-staging`, TD-VSDD-060 sibling-site sweep). |
