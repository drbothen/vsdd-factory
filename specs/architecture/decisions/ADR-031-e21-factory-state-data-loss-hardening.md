---
document_type: architecture-decision-record
level: L3
adr_id: ADR-031
version: "1.0"
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
last_amended: "2026-07-19 (v1.0) — Initial authorship (architect; E-21 factory state data-loss hardening; issues #342, #365, #358, #523, #588; 5 invariants INV-E21-001..INV-E21-005; two-layer defense for INV-E21-001; validate-factory-path-staging crate naming decision; POLICY 21 compliance; CAP-034..CAP-037 allocated)."
modified:
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

**Issue #358 — Stale ARCH-INDEX `[PLANNED]` annotation on `validate-artifact-path/`.** The
crate exists at `crates/hook-plugins/validate-artifact-path/` (serving BC-4.11.001, S-13.01
Edit/Write/MultiEdit path-registry guard) but ARCH-INDEX SS-04 carried a stale `[PLANNED]`
annotation. Corrected in ARCH-INDEX v3.07 (this version bump).

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

**Decision 2 — Two-layer defense for INV-E21-001.** Defense-in-depth with two independent
layers:

- **Layer 1 — Invariant layer (runtime enforcement):** New native WASM PreToolUse plugin
  `validate-factory-path-staging` (crate: `crates/hook-plugins/validate-factory-path-staging/`,
  tool regex = `^Bash$`, priority = 140, tier = sync, on_error = block) fires before any Bash
  tool call. The plugin inspects the command for `git add`, `git stage`, or equivalent staging
  operations targeting `.factory/` paths on non-`factory-artifacts` branches and blocks them.
  POLICY 21 compliance: native WASM, not a shell script.

- **Layer 2 — Safety-net layer (skill-doc enforcement):** BC-5.43.001 mandates that the
  orchestrator, pr-manager, devops-engineer, and state-manager run
  `git diff --name-only HEAD..<target-ref>` before any `git merge`, `git pull`, or `git checkout`
  that advances HEAD on the product branch. If the result contains any path matching
  `^\.factory/`, the operation MUST be halted with `FactoryPathDeletionInMergeDiff`. This layer
  intercepts pre-existing dual-tracking that formed before the runtime guard was active.

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
1. `git diff origin/develop --stat` on the rebased feature branch.
2. For each file with a net-negative line count: check whether any recently-merged sibling story
   commit on `origin/develop` also modified that file.
3. For any file matching both criteria: the agent must explicitly verify the delta is intentional.
   If any such file cannot be confirmed, the gate halts with `UnverifiedNetNegativeDelta`;
   `git push --force-with-lease` is blocked.

**Decision 7 — CAP allocation.** Four new capability entries registered in `capabilities.md`
(v1.8 → v1.9) at the next available IDs after CAP-033:

| CAP-ID | Description | Subsystems | BCs |
|--------|-------------|------------|-----|
| CAP-034 | Factory artifact nested-worktree path exclusivity enforcement (INV-E21-001 two-layer defense) | SS-04, SS-05 | BC-4.16.001, BC-5.43.001 |
| CAP-035 | Post-rebase diff-integrity gate (INV-E21-005) | SS-05 | BC-5.44.001 |
| CAP-036 | Story-worktree write-path discipline and teardown preflight (INV-E21-002 + INV-E21-004) | SS-06 | BC-6.26.001 |
| CAP-037 | Factory worktree branch integrity — dispatch-preamble assertion + factory-side PR restore protocol (INV-E21-003) | SS-06 | BC-6.27.001 |

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

**Crate naming (Decision 3):** The naming decision is production-grade under the "no MVP
deferrals" principle. Reusing `validate-artifact-path` would be the cheap path; creating a
distinct crate with a semantically accurate name is the correct path and avoids the concrete
failure modes enumerated in Decision 3.

## Consequences

1. **S-21.01 deliverable:** Must create `crates/hook-plugins/validate-factory-path-staging/`
   (new crate) and register `validate-factory-path-staging.wasm` in `hooks-registry.toml` with
   `event = "PreToolUse"`, `tool = "^Bash$"`, `priority = 140`, `on_error = "block"`. MUST NOT
   modify the existing `validate-artifact-path` crate.

2. **BC-4.16.001 amendment (product-owner):** The `capability:` frontmatter field must be
   updated from `"TBD"` to `"CAP-034"`. The `§Architecture Anchors` section must be amended to
   reference `crates/hook-plugins/validate-factory-path-staging/` (not `validate-artifact-path/`)
   as the crate backing the Bash guard.

3. **BC capability field backfill (product-owner):** All five BCs must have `capability:`
   updated from `"TBD — E-21 CAP pending ARCH-INDEX registration by architect"` to:
   BC-4.16.001 → `"CAP-034"`, BC-5.43.001 → `"CAP-034"`, BC-5.44.001 → `"CAP-035"`,
   BC-6.26.001 → `"CAP-036"`, BC-6.27.001 → `"CAP-037"`.

4. **ARCH-INDEX correction:** SS-04 module listing stale `[PLANNED]` annotation on
   `validate-artifact-path/` corrected (crate exists since S-13.01); new entry
   `validate-factory-path-staging/` [PLANNED S-21.01] added. Incorporated in ARCH-INDEX v3.07.

5. **POLICY 21 attestation:** All four skill-doc BCs (BC-5.43.001, BC-5.44.001, BC-6.26.001,
   BC-6.27.001) introduce no new shell scripts. BC-4.16.001 Layer 1 (the new WASM guard) uses
   native WASM per ADR-014 standing mandate.

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
| E-21 | Epic anchor for this ADR |
| ADR-031 | This record |

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-07-19 | Initial authorship (architect; E-21 factory state data-loss hardening; issues #342, #365, #358, #523, #588). 5 invariants INV-E21-001..INV-E21-005 defined. 7 decisions: (1) invariant catalog, (2) two-layer INV-E21-001 defense, (3) validate-factory-path-staging crate naming, (4) INV-E21-002+004 skill-doc, (5) INV-E21-003 skill-doc, (6) INV-E21-005 skill-doc, (7) CAP-034..CAP-037 allocated. ARCH-INDEX v3.06→v3.07. capabilities.md v1.8→v1.9. |
