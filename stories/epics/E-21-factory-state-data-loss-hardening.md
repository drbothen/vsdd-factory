---
document_type: epic
epic_id: "E-21"
version: "v1.4"
status: draft
title: "Factory State Data-Loss Hardening — validate-factory-path-staging WASM guard, post-rebase diff-integrity gate, pr-manager trunk assertions, story-worktree write-path discipline, factory-side PR protocol"
prd_capabilities: [CAP-034, CAP-035, CAP-036, CAP-037, CAP-038]
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
input-hash: "d575ad6"
last_amended: "2026-07-19 (v1.4) — adv pass-5 fix burst (F-P5-001): added missing v1.3 Changelog row to body ## Changelog table; frontmatter/body changelog parity restored (5 modified[] entries now matched by 5 Changelog table rows). [Prior: 2026-07-19 (v1.3) — adv pass-3 fix burst (F-P3-003): CAP-036 stale BC-6.26.001 v1.2 cite corrected to v1.3; TD-VSDD-060 full grep sweep confirms no other stale v1.2 cites of BC-5.44.001, BC-6.10.002, BC-6.26.001, or BC-5.43.001. [Prior: 2026-07-19 (v1.2) — adv pass-2 fix burst (F-P2-003): INV-E21-005 'post-rebase product branch' → 'post-rebase feature branch' (authoritative ADR-031 INV-E21-005 scope); BC-5.43.001 → v1.3; ADR-031 version cites → v1.3 (all occurrences); Description item 2 'product branch' + 'pr-manager post-rebase checkpoint' corrected to 'feature branch' + 'devops-engineer.md §Inter-Wave Rebase checkpoint'.]]"
modified:
  - "v1.0 2026-07-19: Initial authoring"
  - "v1.1 2026-07-19: adv pass-1 fix burst (F-P1-008/009/011/013)"
  - "v1.2 2026-07-19: adv pass-2 fix burst (F-P2-003) — INV-E21-005 feature branch fix; BC-5.43.001 → v1.3; ADR-031 cites → v1.3; Description item 2 corrected"
  - "v1.3 2026-07-19: adv pass-3 fix burst (F-P3-003) — CAP-036 BC-6.26.001 v1.2 → v1.3; TD-VSDD-060 full grep sweep confirmed clean"
  - "v1.4 2026-07-19: adv pass-5 fix burst (F-P5-001) — added missing v1.3 Changelog row; frontmatter/body changelog parity restored"
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

2. **Rebase silent drop (S-21.02 — issue #365):** A `git rebase` on a feature branch
   can silently drop factory artifact changes if rebase conflicts are auto-resolved.
   The `git range-diff` detector (primary) + `git diff --stat` backup (BC-5.44.001)
   are not yet wired into the devops-engineer.md §Inter-Wave Rebase checkpoint.

3. **PR base not locked (S-21.03 — issue #358):** pr-manager creates PRs with
   `baseRefName` computed at creation time but never asserted immediately after create
   (step 3 PC2 check) or after merge (step 9 PC3 ancestry check). A race between PR
   creation and merge can land a feature branch onto the wrong base (BC-6.10.002 v1.3 +
   CAP-038; ADR-031 v1.3 §Decision 8).

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

All five issues are closed by new BCs (draft; auto-promote to active per POL-14 when story PRs merge) and their implementing stories.
INV-E21-001..006 (cross-cutting invariants catalogued in ADR-031 v1.3 §Decision 1) govern
the full solution space.

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
v1.1, ADR-031 v1.3, and all six BCs (BC-4.16.001/BC-5.43.001/BC-6.27.001 at v1.2;
BC-5.44.001/BC-6.10.002/BC-6.26.001 at v1.3).

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

E-21 introduces five new PRD capabilities, defined in ADR-031 §Decision 7 (CAP-034..037) and ADR-031 v1.3 §Decision 7 (CAP-038):

- **CAP-034 — Nested-worktree path exclusivity** (`validate-factory-path-staging` WASM
  PreToolUse guard + orchestrator merge pre-check): BC-4.16.001 v1.2 + BC-5.43.001 v1.3.
  Implemented by S-21.01 (new WASM crate `crates/hook-plugins/validate-factory-path-staging/`
  per ADR-031 §Decision 3 — MUST NOT reuse `validate-artifact-path/`). POLICY 21
  enforced: no new `.sh` files.

- **CAP-035 — Post-rebase diff-integrity gate** (`git range-diff` primary + `git diff
  --stat` backup): BC-5.44.001 v1.3. Implemented by S-21.02 (devops-engineer.md
  §Inter-Wave Rebase skill-doc amendment; ADR-031 v1.3 §Consequences #5 confirms this
  is the only codebase location with a `git rebase origin/develop` + `git push
  --force-with-lease` sequence on a feature branch).

- **CAP-036 — Story-worktree write-path discipline** (canonical-path mandate + teardown
  preflight): BC-6.26.001 v1.3. Implemented by S-21.04 (skill-doc amendment to
  `_shared-context.md` + `step-g-cleanup.md`).

- **CAP-037 — Factory-side PR protocol** (5-step restore + dispatch-preamble assertion):
  BC-6.27.001 v1.2. Implemented by S-21.05 (pr-manager.md skill-doc amendment).

- **CAP-038 — PR trunk ancestry integrity** (post-create `baseRefName` assertion + post-merge
  `--is-ancestor` check): BC-6.10.002 v1.3 (ADR-031 v1.3 §Decision 7 + §Decision 8).
  Implemented by S-21.03 (pr-manager.md skill-doc amendment, Step 3 PC2 + Step 9 PC3).

## Acceptance Criteria

| ID | Criterion | Validation Method | Test Scenarios |
|----|-----------|-------------------|----------------|
| EAC-001 | All five stories S-21.01..S-21.05 shipped and merged to `develop` within this epic's cycle | All story PRs CI-green and merged | S-21.01..S-21.05 PR merge confirmations |
| EAC-002 | `validate-factory-path-staging` WASM PreToolUse guard blocks `git add .factory/` on any non-`factory-artifacts` branch | CI bats integration test: mock `git add .factory/STATE.md` on `develop` → `block_intent=true`; mock on `factory-artifacts` → pass; mock non-.factory/ arg → pass | S-21.01 AC-001..AC-004 test suite |
| EAC-003 | devops-engineer.md §Inter-Wave Rebase post-rebase diff-integrity gate fires `UnverifiedNetNegativeDelta` when `git range-diff` detects a dropped commit | CI bats test: inject mock range-diff output with net-negative delta → halt with `UnverifiedNetNegativeDelta` | S-21.02 AC-003 test suite |
| EAC-004 | pr-manager step 3 asserts `baseRefName` immediately after PR creation; step 9 asserts merged commit is ancestor of trunk | CI bats test: mock post-create PR with wrong baseRefName → `BaseRefNameMismatch` hard-fail; mock post-merge with non-ancestor → `MergeNotAncestorOfTrunk` P0 error | S-21.03 AC-001..AC-003 test suite |
| EAC-005 | Story worktree teardown preflight detects stray `.factory/` file and halts before `git worktree remove` | CI bats test: create stray `.factory/test.md` in worktree; trigger teardown → halt with `PREFLIGHT BLOCKED` per BC-6.26.001 PC2b | S-21.04 AC-003 test suite |

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
  S-21.01 is P0 (new WASM guard; highest blast radius — product-branch merge clobber).
  S-21.02 and S-21.03 are P1 (rebase and PR-base integrity; process gaps with data-loss
  risk if race occurs). All three are fully independent; they can run in parallel within
  W1. S-21.01 creates the `validate-factory-path-staging/` crate; S-21.02 amends
  `devops-engineer.md` §Inter-Wave Rebase; S-21.03 amends `pr-manager.md` (different
  agent files — no merge conflict risk).

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

**POLICY 21 compliance note (F-P1-013):** S-21.03 (`plugins/vsdd-factory/tests/fixtures/pr-manager-trunk/`)
and S-21.05 prescribe bats test fixture scripts that stub `gh`/`git` commands. Per POLICY 21
(no new `.sh` files), all new test fixture mock scripts MUST use extensionless filenames
(e.g., `mock-gh`, `stub-git`) or `.bash` extension. No `.sh` fixture files are permitted.

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
  MUST NOT touch these artifacts. All upstream BCs are at their pre-implementation versions
  (v1.2 or v1.3 per ADR-031 v1.3) and no spec amendments are required.

## Behavioral Contract Traceability

All BCs listed here are draft; they auto-promote to active per POL-14 when their implementing story's PR merges. No E-21 story amends a BC — each BC is already at its final pre-implementation version.

| BC ID | Version | Title (abbreviated) | Capability | Implementing Story |
|-------|---------|---------------------|------------|-------------------|
| BC-4.16.001 | v1.2 | `validate-factory-path-staging` WASM PreToolUse guard: block `git add .factory/` on non-`factory-artifacts` branches; pass all other operations | CAP-034 | S-21.01 (new WASM crate `validate-factory-path-staging/`; POLICY 21: no new .sh) |
| BC-5.43.001 | v1.3 | Orchestrator merge pre-check: halt merge if diff contains `.factory/` paths (two-layer defense with BC-4.16.001) | CAP-034 | S-21.01 (orchestrator/per-story-delivery.md §Main-Checkout Sync Protocol amendment; ADR-031 v1.3 §Decision 2) |
| BC-5.44.001 | v1.3 | Post-rebase diff-integrity gate: `git range-diff` (primary) + `git diff --stat` (backup); halt on `UnverifiedNetNegativeDelta` | CAP-035 | S-21.02 (devops-engineer.md §Inter-Wave Rebase amendment; ADR-031 v1.3 §Consequences #5) |
| BC-6.10.002 | v1.3 | deliver-story 9-step dispatch: PC2 post-create baseRefName assertion (`BaseRefNameMismatch` hard-fail) + PC3 post-merge ancestry check (`MergeNotAncestorOfTrunk` P0 error) | CAP-038 | S-21.03 (pr-manager.md amendment, steps 3 + 9) |
| BC-6.26.001 | v1.3 | Story-worktree write-path discipline (canonical absolute path via `CANONICAL_FACTORY_ROOT`) + teardown preflight (`find <worktree>/.factory -type f` before `git worktree remove`) | CAP-036 | S-21.04 (skill-doc: `_shared-context.md` + `step-g-cleanup.md`) |
| BC-6.27.001 | v1.2 | pr-manager factory-side PR protocol: 5-step restore sequence (checkout `factory-artifacts`, `pull --ff-only`, delete local/remote chore branch, final assertion) + dispatch-preamble branch assertion (PC2) | CAP-037 | S-21.05 (pr-manager.md amendment) |

**INV-E21-001..006 cross-cutting invariants** (ADR-031 v1.3 §Decision 1):

- **INV-E21-001** — Factory artifact isolation: `.factory/` mutations MUST NOT be
  staged on product branches under any orchestrator operation.
- **INV-E21-002** — Canonical write-path: all `.factory/` writes in story worktrees
  MUST use the canonical main-worktree absolute path.
- **INV-E21-003** — Factory worktree branch integrity: the `.factory/` worktree MUST
  be on branch `factory-artifacts` before and after any pr-manager dispatch.
- **INV-E21-004** — Teardown completeness: a story worktree MUST have zero stray
  `.factory/` files before `git worktree remove`.
- **INV-E21-005** — Post-rebase diff integrity: a post-rebase feature branch MUST be
  verifiably non-negative-delta relative to its pre-rebase HEAD for all files touched by
  recently-merged sibling stories (enforced by S-21.02 via BC-5.44.001).
- **INV-E21-006** — PR trunk ancestry: every story PR MUST be verified as an ancestor of
  `origin/develop` immediately after merge; the PR's `baseRefName` MUST equal the
  configured trunk immediately after `gh pr create` (enforced by S-21.03 via BC-6.10.002;
  ADR-031 v1.3 §Decision 8 + CAP-038).

## Changelog

| Version | Date | Author | Summary |
|---------|------|--------|---------|
| v1.0 | 2026-07-19 | story-writer | Initial authoring. E-21 epic; 5 stories; 27 pts; 2 waves; 6 BCs; issues #342/#365/#358/#523/#588; INV-E21-001..005; CAP-034..037. |
| v1.1 | 2026-07-19 | story-writer | adv pass-1 fix burst (F-P1-008/009/011/013): EAC-005 → "PREFLIGHT BLOCKED" (BC-6.26.001 PC2b) + AC trace → S-21.04 AC-003; BC statuses draft per POL-14; BC versions BC-5.44.001/BC-6.10.002/BC-6.26.001 → v1.3; S-21.01 priority P0; INV-E21-001..006 (INV-E21-006 + CAP-038 added per ADR-031 v1.1); S-21.02 gate host → devops-engineer.md §Inter-Wave Rebase; POLICY 21 fixture annotation. |
| v1.2 | 2026-07-19 | story-writer | adv pass-2 fix burst (F-P2-003): INV-E21-005 "post-rebase product branch" → "post-rebase feature branch" (authoritative ADR-031 INV-E21-005 scope; BC-5.44.001 operates on feature branch where devops-engineer rebases + force-with-lease-pushes); BC-5.43.001 → v1.3 (BC Traceability + CAP-034 inline); ADR-031 v1.1 → v1.3 in all epic cites (Description item 2/3, Trigger, PRD Capabilities CAP-035/CAP-038, Out of Scope, INV catalog cross-ref); Description item 2 "pr-manager post-rebase checkpoint" → "devops-engineer.md §Inter-Wave Rebase checkpoint". |
| v1.3 | 2026-07-19 | story-writer | adv pass-3 fix burst (F-P3-003): CAP-036 BC-6.26.001 v1.2→v1.3 (lone stale cite in PRD Capabilities); TD-VSDD-060 full grep sweep confirmed no other stale v1.2 cites of BC-5.44.001/BC-6.10.002/BC-6.26.001/BC-5.43.001. |
| v1.4 | 2026-07-19 | story-writer | adv pass-5 fix burst (F-P5-001): added missing v1.3 Changelog row; frontmatter/body changelog parity restored (5 modified[] entries now matched by 5 Changelog table rows). |
