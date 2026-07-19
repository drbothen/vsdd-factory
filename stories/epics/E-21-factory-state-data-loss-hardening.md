---
document_type: epic
epic_id: "E-21"
version: "v1.0"
status: draft
title: "Factory State Data-Loss Hardening — validate-factory-path-staging WASM guard, post-rebase diff-integrity gate, pr-manager trunk assertions, story-worktree write-path discipline, factory-side PR protocol"
prd_capabilities: [CAP-034, CAP-035, CAP-036, CAP-037]
subsystems_affected: [SS-04, SS-05, SS-06]
target_release: "v1.0.0-rc.25"
story_count: 5
producer: story-writer
timestamp: "2026-07-19T00:00:00Z"
phase: brownfield-backfill
cycle: v1.0-brownfield-backfill
depends_on: []
inputs:
  - .factory/cycles/v1.0-brownfield-backfill/e-21-arch-delta-analysis.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.16.001.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.43.001.md
  - .factory/specs/behavioral-contracts/ss-05/BC-5.44.001.md
  - .factory/specs/behavioral-contracts/ss-06/BC-6.10.002.md
  - .factory/specs/behavioral-contracts/ss-06/BC-6.26.001.md
  - .factory/specs/behavioral-contracts/ss-06/BC-6.27.001.md
  - .factory/specs/architecture/decisions/ADR-031-e21-factory-state-data-loss-hardening.md
  - .factory/stories/S-21.01-validate-factory-path-staging.md
  - .factory/stories/S-21.02-post-rebase-diff-integrity-gate.md
  - .factory/stories/S-21.03-pr-manager-trunk-assertion.md
  - .factory/stories/S-21.04-story-worktree-write-path-discipline.md
  - .factory/stories/S-21.05-pr-manager-factory-side-pr-protocol.md
input-hash: "ede9571"
last_amended: "2026-07-19 (v1.0) — Initial authoring (story-writer; E-21 factory-state data-loss hardening; issues #342/#365/#358/#523/#588; INV-E21-001..005; CAP-034..037; ADR-031)."
modified:
  - "v1.0 2026-07-19: Initial authoring"
---

# Epic E-21: Factory State Data-Loss Hardening

## Description

E-21 collects the five hardening stories that close a compound of confirmed-live factory
artifact data-loss issues discovered during the v1.0-brownfield-backfill cycle. The five
issues span three distinct defect classes across subsystems SS-04, SS-05, and SS-06:

1. **Product-branch merge clobber (S-21.01 — issue #342):** When an orchestrator merge
   operation targets a product branch (e.g., `develop`), `git add .` silently stages
   `.factory/` writes alongside product-code changes. There is no PreToolUse guard to
   block `git add .factory/` on product branches, and no orchestrator pre-check to halt
   if the merge diff contains `.factory/` paths. Factory spec state survives only because
   humans catch it in PR review — a process gap, not a mechanical gate.

2. **Rebase silent drop (S-21.02 — issue #365):** A `git rebase` on a product branch
   can silently drop factory artifact changes if rebase conflicts are auto-resolved.
   The `git range-diff` detector (primary) + `git diff --stat` backup (BC-5.44.001)
   are not yet wired into pr-manager's post-rebase checkpoint.

3. **PR base not locked (S-21.03 — issue #358):** pr-manager creates PRs with
   `baseRefName` computed at creation time but never asserted immediately after create
   (step 3 PC2 check) or after merge (step 9 PC3 ancestry check). A race between PR
   creation and merge can land a feature branch onto the wrong base (BC-6.10.002 v1.2).

4. **Story-worktree teardown loss (S-21.04 — issue #523):** When a story worktree is
   removed via `git worktree remove --force`, any `.factory/` files written relative to
   the worktree path (rather than the canonical main-worktree absolute path) are silently
   deleted. Canonical write-path discipline (`CANONICAL_FACTORY_ROOT` or `git -C
   <main-worktree> rev-parse --show-toplevel`) is not enforced, and the teardown preflight
   (`find <worktree>/.factory -type f`) is absent (BC-6.26.001).

5. **Factory-side PR strand (S-21.05 — issue #588):** After pr-manager merges a
   factory-side PR (chore branch on `factory-artifacts`), the `.factory/` shared worktree
   is stranded on the chore branch. Subsequent state-manager dispatches inherit the stale
   branch silently. The 5-step restore sequence (checkout `factory-artifacts`, `pull
   --ff-only`, delete local + remote chore branch, assert) and dispatch-preamble branch
   assertion are absent (BC-6.27.001).

All five issues are closed by new BCs (FINAL, all v1.2) and their implementing stories.
INV-E21-001..005 (cross-cutting invariants catalogued in ADR-031 §Decision 1) govern the
full solution space.

## Trigger / Motivation

The trigger is the confirmed-live five-issue compound discovered during the
v1.0-brownfield-backfill cycle (2026-07-13..2026-07-18) and formally catalogued in
`e-21-arch-delta-analysis.md` (v1.1, commit ca79c886). Each issue has a confirmed
failure scenario with evidence of silent data loss or silent stranding:

- Issue #342: no mechanical guard on `git add .factory/` from product branches
- Issue #365: post-rebase diff-integrity gate absent from pr-manager
- Issue #358: pr-manager PR lifecycle lacks step 3 base-assertion + step 9 ancestry check
- Issue #523: story-worktree teardown with stray `.factory/` file confirmed deletable
- Issue #588: factory-side PR restore absent; pr-manager.md has no factory-side PR flow section

The five issues share a root-cause taxonomy: every one relies on human review rather than
a mechanical gate to prevent factory artifact loss. ADR-031 formalises the invariant
catalog and the two-layer defense strategy.

Human authorization for E-21 was granted with the delivery of `e-21-arch-delta-analysis.md`
v1.1, ADR-031 v1.0, and all six BCs at v1.2.

## Epic Placement Justification

E-20 is the immediately preceding reserved epic in the index. E-21 is the next free ID
under POLICY 1 (append-only numbering; STORY-INDEX confirmed no E-21 row at time of
creation 2026-07-19). The five data-loss issues are logically cohesive — they all
concern factory artifact integrity in the `.factory/` worktree — and warrant a new epic
because they span three subsystems (SS-04, SS-05, SS-06) and introduce one new WASM crate
(ADR-031 §Decision 3) alongside four skill-doc amendments. Grouping them under E-19
(operator hardening) would conflate runtime-defect fixes with write-path integrity
hardening.

**Sequencing context:** `depends_on: []` reflects that E-19 is COMPLETE at time of E-21
authoring. E-21 does not require any E-19 work to be in-progress or gated.

## PRD Capabilities Covered

E-21 introduces four new PRD capabilities, all defined in ADR-031 §Decision 7:

- **CAP-034 — Nested-worktree path exclusivity** (`validate-factory-path-staging` WASM
  PreToolUse guard + orchestrator merge pre-check): BC-4.16.001 v1.2 + BC-5.43.001 v1.2.
  Implemented by S-21.01 (new WASM crate `crates/hook-plugins/validate-factory-path-staging/`
  per ADR-031 §Decision 3 — MUST NOT reuse `validate-artifact-path/`). POLICY 21
  enforced: no new `.sh` files.

- **CAP-035 — Post-rebase diff-integrity gate** (`git range-diff` primary + `git diff
  --stat` backup): BC-5.44.001 v1.2. Implemented by S-21.02 (pr-manager.md skill-doc
  amendment only).

- **CAP-036 — Story-worktree write-path discipline** (canonical-path mandate + teardown
  preflight): BC-6.26.001 v1.2. Implemented by S-21.04 (skill-doc amendment to
  `_shared-context.md` + `step-g-cleanup.md`).

- **CAP-037 — Factory-side PR protocol** (5-step restore + dispatch-preamble assertion):
  BC-6.27.001 v1.2. Implemented by S-21.05 (pr-manager.md skill-doc amendment).

BC-6.10.002 v1.2 (S-21.03 — PR base assertion + ancestry check) amends an existing
capability (no new CAP ID); it adds PC2 (post-create baseRefName assertion) and PC3
(post-merge ancestry check) to the deliver-story 9-step dispatch.

## Acceptance Criteria

| ID | Criterion | Validation Method | Test Scenarios |
|----|-----------|-------------------|----------------|
| EAC-001 | All five stories S-21.01..S-21.05 shipped and merged to `develop` within this epic's cycle | All story PRs CI-green and merged | S-21.01..S-21.05 PR merge confirmations |
| EAC-002 | `validate-factory-path-staging` WASM PreToolUse guard blocks `git add .factory/` on any non-`factory-artifacts` branch | CI bats integration test: mock `git add .factory/STATE.md` on `develop` → `block_intent=true`; mock on `factory-artifacts` → pass; mock non-.factory/ arg → pass | S-21.01 AC-001..AC-004 test suite |
| EAC-003 | pr-manager post-rebase diff-integrity gate fires `UnverifiedNetNegativeDelta` when `git range-diff` detects a dropped commit | CI bats test: inject mock range-diff output with net-negative delta → halt with `UnverifiedNetNegativeDelta` | S-21.02 AC-002 test suite |
| EAC-004 | pr-manager step 3 asserts `baseRefName` immediately after PR creation; step 9 asserts merged commit is ancestor of trunk | CI bats test: mock post-create PR with wrong baseRefName → `BaseRefNameMismatch` hard-fail; mock post-merge with non-ancestor → `MergeNotAncestorOfTrunk` P0 error | S-21.03 AC-001..AC-003 test suite |
| EAC-005 | Story worktree teardown preflight detects stray `.factory/` file and halts before `git worktree remove --force` | CI bats test: create stray `.factory/test.md` in worktree; trigger teardown → halt with `StrayFactoryFilesDetected` | S-21.04 AC-004 test suite |

## Stories

| Story ID | Title | Wave | Points | BCs |
|----------|-------|------|--------|-----|
| S-21.01 | validate-factory-path-staging WASM guard + orchestrator merge pre-check | W1 | 11 | BC-4.16.001, BC-5.43.001 |
| S-21.02 | post-rebase diff-integrity gate (range-diff primary) | W1 | 3 | BC-5.44.001 |
| S-21.03 | pr-manager base assertion + post-merge ancestry check | W1 | 3 | BC-6.10.002 |
| S-21.04 | story-worktree write-path discipline + teardown preflight | W2 | 5 | BC-6.26.001 |
| S-21.05 | pr-manager factory-side PR protocol: restore-original-branch, ff-only sync, chore-branch cleanup | W2 | 5 | BC-6.27.001 |

**Total:** 5 stories, 27 story points.

> **Maintenance tally drift-check:** Compute story count + points from the 5 linked story frontmatters and assert equals the Stories-table totals (5 / 27); run at every epic amendment.

**Sequencing rationale:**

- Wave 1 (S-21.01, S-21.02, S-21.03 — 17 pts): The three gate-layer fixes.
  S-21.01 is P1-CRITICAL (new WASM guard; highest blast radius — product-branch merge
  clobber). S-21.02 and S-21.03 are P1 (rebase and PR-base integrity; process gaps with
  data-loss risk if race occurs). All three are fully independent; they can run in parallel
  within W1. S-21.01 creates the `validate-factory-path-staging/` crate; S-21.02 and
  S-21.03 amend `pr-manager.md` (different sections — no merge conflict risk).

- Wave 2 (S-21.04, S-21.05 — 10 pts): The write-path discipline and factory-side PR
  restore stories. S-21.04 benefits from the W1 `validate-factory-path-staging` WASM guard
  being active as defense-in-depth (INV-E21-001 Layer 1 already blocks product-branch
  staging), but is independently viable and has no formal `depends_on` on S-21.01.
  S-21.05 is independent of all other E-21 stories; wave-2 placement reflects thematic
  adjacency with S-21.04 (both address shared-mutable-worktree branch integrity). Both
  W2 stories can run in parallel.

**Wave model note:** W1 and W2 group by risk tier; intra-wave sequencing is expressed
solely via `depends_on` (empty for all E-21 stories). The scheduler honors `depends_on`,
not wave co-membership. W2 may begin as soon as any W1 story merges if resource allows.

## Dependency Graph

```mermaid
graph LR
  S-21.01
  S-21.02
  S-21.03
  S-21.04
  S-21.05
```

All five nodes are isolated. No story in E-21 formally blocks another. W1 runs in parallel;
W2 runs in parallel after W1 starts (priority sequencing only — no hard edge). Acyclic
confirmed.

> **Note on soft coupling:** S-21.04 and S-21.05 each contain a narrative note that they
> benefit from S-21.01's WASM guard (INV-E21-001 Layer 1) as defense-in-depth. This is
> architectural context, not a `depends_on` constraint. Both W2 stories are deliverable
> before S-21.01 merges.

## Dependencies (External)

| System | Capability Needed | Readiness |
|--------|------------------|-----------|
| None | E-21 is self-contained within the vsdd-factory codebase (`crates/hook-plugins/`, `plugins/vsdd-factory/agents/`, `plugins/vsdd-factory/skills/`). No external systems, APIs, or third-party services are required. | N/A |

## Out of Scope

- **`factory-branch-guard.sh` modifications:** The existing `factory-branch-guard.sh`
  bash hook is grandfathered (E-20 scope). E-21 MUST NOT modify it. The new
  `validate-factory-path-staging` WASM crate (S-21.01) is a SEPARATE guard covering the
  `git add .factory/` surface on product branches (ADR-031 §Decision 2 two-layer defense).

- **`validate-artifact-path/` crate:** E-21 MUST NOT modify the existing
  `validate-artifact-path/` crate (serves BC-4.11.001/S-13.01). The new crate
  `validate-factory-path-staging/` is distinct per ADR-031 §Decision 3.

- **BC-6.23.001 cooperative lock/unlock (ADR-025 §Decision 15):** Factory-side PR
  protocol (CAP-037, S-21.05) is distinct from the lock/unlock capability. E-21 does not
  modify BC-6.23.001 or the lock/unlock skills.

- **WASM fuel-budget increase for lessons.md:** D-442(e) documents the WASM fuel exhaustion
  issue on large lessons.md files. This is NOT part of E-21; it is tracked under S-15.03
  PRIORITY-A and is a separate concern.

- **BC files, BC-INDEX, VP files, ARCH-INDEX, STATE.md:** E-21 stories (S-21.01..S-21.05)
  MUST NOT touch these artifacts. All upstream BCs are FINAL at v1.2 and ADR-031 is v1.0;
  no spec amendments are required.

## Behavioral Contract Traceability

All BCs listed here are FINAL at the versions shown. No E-21 story amends a BC.

| BC ID | Version | Title (abbreviated) | Capability | Implementing Story |
|-------|---------|---------------------|------------|-------------------|
| BC-4.16.001 | v1.2 | `validate-factory-path-staging` WASM PreToolUse guard: block `git add .factory/` on non-`factory-artifacts` branches; pass all other operations | CAP-034 | S-21.01 (new WASM crate `validate-factory-path-staging/`; POLICY 21: no new .sh) |
| BC-5.43.001 | v1.2 | Orchestrator merge pre-check: halt merge if diff contains `.factory/` paths (two-layer defense with BC-4.16.001) | CAP-034 | S-21.01 (orchestrator skill-doc amendment) |
| BC-5.44.001 | v1.2 | Post-rebase diff-integrity gate: `git range-diff` (primary) + `git diff --stat` (backup); halt on `UnverifiedNetNegativeDelta` | CAP-035 | S-21.02 (pr-manager.md amendment) |
| BC-6.10.002 | v1.2 | deliver-story 9-step dispatch: PC2 post-create baseRefName assertion (`BaseRefNameMismatch` hard-fail) + PC3 post-merge ancestry check (`MergeNotAncestorOfTrunk` P0 error) | (amends existing deliver-story BC) | S-21.03 (pr-manager.md amendment, steps 3 + 9) |
| BC-6.26.001 | v1.2 | Story-worktree write-path discipline (canonical absolute path via `CANONICAL_FACTORY_ROOT`) + teardown preflight (`find <worktree>/.factory -type f` before `git worktree remove --force`) | CAP-036 | S-21.04 (skill-doc: `_shared-context.md` + `step-g-cleanup.md`) |
| BC-6.27.001 | v1.2 | pr-manager factory-side PR protocol: 5-step restore sequence (checkout `factory-artifacts`, `pull --ff-only`, delete local/remote chore branch, final assertion) + dispatch-preamble branch assertion (PC2) | CAP-037 | S-21.05 (pr-manager.md amendment) |

**INV-E21-001..005 cross-cutting invariants** (ADR-031 §Decision 1):

- **INV-E21-001** — Factory artifact isolation: `.factory/` mutations MUST NOT be
  staged on product branches under any orchestrator operation.
- **INV-E21-002** — Canonical write-path: all `.factory/` writes in story worktrees
  MUST use the canonical main-worktree absolute path.
- **INV-E21-003** — Factory worktree branch integrity: the `.factory/` worktree MUST
  be on branch `factory-artifacts` before and after any pr-manager dispatch.
- **INV-E21-004** — Teardown completeness: a story worktree MUST have zero stray
  `.factory/` files before `git worktree remove --force`.
- **INV-E21-005** — Rebase integrity: a post-rebase product branch MUST be
  verifiably non-negative-delta relative to its pre-rebase HEAD for all factory artifacts.

## Changelog

| Version | Date | Author | Summary |
|---------|------|--------|---------|
| v1.0 | 2026-07-19 | story-writer | Initial authoring. E-21 epic; 5 stories; 27 pts; 2 waves; 6 BCs; issues #342/#365/#358/#523/#588; INV-E21-001..005; CAP-034..037. |
