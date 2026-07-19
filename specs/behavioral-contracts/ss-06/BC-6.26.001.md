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
  - plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md
  - plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md
input-hash: "TBD"
traces_to: .factory/specs/architecture/ARCH-INDEX.md
origin: brownfield
extracted_from: null
subsystem: "SS-06"
capability: "CAP-036"
lifecycle_status: draft
introduced: v1.0-brownfield-backfill
modified:
  - "2026-07-19 (v1.1) — CAP-036 backfill (product-owner; ARCH-INDEX v3.07): capability frontmatter TBD→CAP-036; §Traceability L2 Capability TBD→CAP-036; Capability Anchor Justification updated to cite CAP-036/ARCH-INDEX v3.07."
  - "2026-07-19 (v1.2) — Research validation precision amendments (product-owner; research validation 2026-07-19): §Description preflight rationale clarified — explicit statement that the preflight compensates for `--force` usage stripping stock git's unclean-worktree protection; Invariant 5 added documenting the `--force` stripping mechanism."
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-6.26.001
section: "6.26"
last_amended: "(v1.2) — Research validation precision amendments (product-owner; research validation 2026-07-19): §Description --force rationale clarified; Invariant 5 added. [Prior: (v1.1) — CAP-036 backfill. (v1.0) — Initial authoring; story-worktree write-path discipline (INV-E21-002) + teardown preflight (INV-E21-004). lifecycle_status: draft (POL-14).]"
---

# BC-6.26.001: deliver-story step agents MUST write all `.factory/**` artifacts using absolute paths anchored to the canonical main-checkout `.factory/` mount, and step-G cleanup MUST run a worktree `.factory/` inventory preflight before `git worktree remove`

## Description

Story agents execute with their CWD set to the story worktree (`.worktrees/<STORY-ID>/`). When
an agent writes a `.factory/` artifact using a relative path (e.g., `.factory/stories/S-NNN-DELIVERY.md`
resolved from the story worktree CWD), the write lands in the story worktree's stale `.factory/`
shadow tree — a copy populated at `git worktree add` time that is neither tracked on
`factory-artifacts` nor ever updated. When step G runs `git worktree remove --force`, the shadow
tree and all artifacts written to it are permanently destroyed with no warning.

This BC governs two complementary protocol requirements that close the loss window.

**Why `--force` requires a preflight:** Stock git already protects against inadvertent worktree removal
— `git worktree remove` (without `--force`) refuses to remove a worktree that has uncommitted changes
or untracked files ("fatal: 'worktrees/<name>' contains modified or untracked files, use --force to
override"). The factory's deliver-story step G uses `--force` to handle legitimate cases (e.g., the
worktree has build artifacts or other non-factory untracked files that can be discarded). This strips
exactly the protection that would catch stray `.factory/` artifacts. The teardown preflight in this BC
re-establishes that protection specifically for the `.factory/` subdirectory — the category where
silent loss causes unrecoverable data loss.

**Write-path discipline (INV-E21-002 instantiation):** Every agent operating within the
deliver-story skill protocol that writes to a factory artifact MUST use an absolute path anchored
to the canonical main-checkout root. Relative paths from story worktree CWD are FORBIDDEN for
factory artifact writes. The load-bearing cases named explicitly: the DELIVERY ledger
(`.factory/stories/<STORY-ID>-DELIVERY.md`), story-frontmatter files (`.factory/stories/<STORY-ID>.md`),
and the pr-reviewer's pr-review.md record. The canonical main-checkout `.factory/` mount is always
at `<repo-root>/.factory/`; agents determine it via `git rev-parse --show-toplevel` from the
main worktree (NOT from the story worktree CWD) or from the orchestrator-provided `CANONICAL_FACTORY_ROOT`
environment variable.

**Teardown preflight (INV-E21-004 instantiation):** Before any `git worktree remove` command on
a story worktree, step G MUST run a `.factory/` inventory check on the worktree path:

```
find <worktree-path>/.factory -type f 2>/dev/null
```

If the result is non-empty (any files found), step G MUST NOT proceed with `git worktree remove`.
Instead it MUST either: (a) relocate each found file to the canonical `.factory/` mount (if valid
factory artifacts), then verify the shadow tree is empty, then proceed with teardown; or (b) STOP
with a visible error requiring manual intervention.

No new shell script or WASM plugin required (POLICY 21 satisfied). Both requirements are skill-doc
mandates.

## Preconditions

### Write-discipline precondition

1. A specialist agent (implementer, demo-recorder, pr-reviewer, or state-manager) is about to
   write a factory artifact (DELIVERY ledger, story-frontmatter, pr-review.md, or any
   `.factory/**` file) while operating with its CWD inside a story worktree
   (`.worktrees/<STORY-ID>/`).

### Teardown preflight precondition

2. Step G (devops-engineer cleanup step) is about to execute `git worktree remove` on a story
   worktree path (`.worktrees/<STORY-ID>/`).

## Postconditions

### PC1 — Write-path discipline: all `.factory/**` writes use canonical absolute paths

Every agent writing to any `.factory/**` path during deliver-story execution MUST target the
canonical absolute path under the main-checkout root. Concretely:

- **Correct:** `Write(file_path="/abs/path/to/repo/.factory/stories/S-NNN-DELIVERY.md", ...)`
- **Forbidden:** `Write(file_path=".factory/stories/S-NNN-DELIVERY.md", ...)` (relative to story
  worktree CWD — silently writes to shadow tree)
- **Forbidden:** `Write(file_path="../../.factory/stories/S-NNN-DELIVERY.md", ...)` (relative
  traversal — brittle and error-prone)

The canonical root is determined exactly once per dispatch via `git -C <story-worktree-path> rev-parse --show-toplevel`
or via the `CANONICAL_FACTORY_ROOT` variable provided by the orchestrator dispatch preamble. It
MUST NOT be assumed from CWD.

**INV-E21-002 instantiation:** Factory artifact writes MUST use canonical absolute paths. A write
to a story-worktree shadow `.factory/` is undetectable as incorrect at write time but results in
permanent data loss at teardown.

### PC2 — Teardown preflight: shadow `.factory/` must be empty before `git worktree remove`

Step G MUST run `find <worktree-path>/.factory -type f 2>/dev/null` before any `git worktree remove`
command. Two cases:

**PC2a — Empty result (normal case):** The `find` command returns no output. Step G proceeds with
`git worktree remove --force` normally.

**PC2b — Non-empty result (stray factory artifacts found):** The `find` command returns one or
more file paths. Step G MUST NOT proceed with `git worktree remove`. It MUST:

1. Log each stray file path with the message:
   ```
   PREFLIGHT BLOCKED: Found factory artifact(s) in story worktree shadow .factory/:
     <path1>
     <path2>
     ...
   These files were written to the wrong worktree (issue #523 class) and would be
   permanently destroyed by git worktree remove. Manual intervention required:
     Option A: Relocate to canonical .factory/ mount, verify content, then retry teardown.
     Option B: Discard (only if files are confirmed redundant copies already committed on factory-artifacts).
   ```
2. Halt teardown. `git worktree remove` is NOT executed.
3. The story cleanup MUST NOT complete until the preflight returns an empty result.

**INV-E21-004 instantiation:** `git worktree remove` on a story worktree MUST be preceded by an
empty-`.factory/` assertion.

## Invariants

1. **INV-E21-002 (Factory Artifact Write Canonical-Path Discipline):** Any agent writing to a
   factory artifact MUST use an absolute path anchored to the canonical main-checkout `.factory/`
   mount. Worktree-relative paths are categorically forbidden for `.factory/**` writes. The
   _shared-context.md `§Spec-Path Discipline` rule (which covers reads) MUST be extended to
   cover writes explicitly, naming the DELIVERY ledger and pr-review.md as load-bearing cases.

2. **INV-E21-004 (Story Worktree Teardown Preflight):** `git worktree remove` on a story worktree
   MUST be preceded by the `find <worktree>/.factory -type f` inventory check. No exceptions —
   not even when the agent is confident no `.factory/` writes occurred (the preflight is the
   mechanical gate, not agent confidence).

3. **Canonical root determination is orchestrator-provided or git-derived, never CWD-assumed.**
   The canonical `.factory/` root MUST be determined via `git -C <worktree-path> rev-parse --show-toplevel`
   on the MAIN worktree (not the story worktree, which would return the story worktree root) or
   from `CANONICAL_FACTORY_ROOT`. CWD-relative resolution is the root cause of issue #523 and
   is forbidden.

4. **Write discipline covers all factory-artifact categories.** The mandate applies to: DELIVERY
   ledgers (`*-DELIVERY.md`), story-frontmatter files, pr-review.md records, STATE.md updates,
   VP anchor files, and any other file under `.factory/**`. It is NOT limited to DELIVERY ledgers.

5. **`--force` strips git's built-in unclean-worktree protection; this preflight restores it.**
   Without `--force`, `git worktree remove` refuses to proceed if the target worktree has modified
   or untracked files (git docs: "Only clean worktrees... can be removed without `--force`"). Step G
   uses `--force` to bypass this refusal for non-factory untracked artifacts (build outputs, temp
   files). This bypass also removes the protection for stray `.factory/` shadow files. This BC's
   teardown preflight is the targeted replacement: it checks ONLY the `.factory/` subdirectory
   (the high-stakes category) and re-establishes the block before `--force` bypasses git's own guard.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Agent writes `Write(".factory/stories/S-021-DELIVERY.md")` with CWD = `.worktrees/S-021/` | FORBIDDEN: relative path resolves to shadow tree; violates PC1 (INV-E21-002) |
| EC-002 | Agent writes `Write("/abs/repo/.factory/stories/S-021-DELIVERY.md")` | CORRECT: absolute path anchored to canonical root; PC1 compliant |
| EC-003 | `find .worktrees/S-021/.factory -type f` returns empty | Teardown proceeds (PC2a) |
| EC-004 | `find .worktrees/S-021/.factory -type f` returns `.worktrees/S-021/.factory/stories/S-021-DELIVERY.md` | BLOCKED: PC2b; relocate or discard then retry |
| EC-005 | Story worktree has no `.factory/` directory at all (clean worktree) | `find` exits non-zero or returns empty; treated as PC2a (no `.factory/` subtree = no stray files) |
| EC-006 | Agent correctly writes DELIVERY ledger to canonical path, but a prior agent also wrote to the shadow tree | Preflight catches the stray copy; step G blocked until resolved |
| EC-007 | pr-reviewer writes `pr-review.md` using a relative path from its CWD (story worktree) | FORBIDDEN: violates PC1; shadow-tree write; would be lost at teardown |

## Canonical Test Vectors

| Test # | Precondition | Action | Expected Result |
|--------|-------------|--------|----------------|
| T-1 | Agent CWD = `.worktrees/S-021/` | `Write(".factory/stories/S-021-DELIVERY.md")` | FORBIDDEN: violates PC1 (caught by adversarial review or step check) |
| T-2 | Agent CWD = `.worktrees/S-021/` | `Write("/abs/repo/.factory/stories/S-021-DELIVERY.md")` | Correct: PC1 compliant; file lands on canonical mount |
| T-3 | Shadow tree empty | Step G: `find .worktrees/S-021/.factory -type f` | Returns empty; teardown proceeds |
| T-4 | Shadow tree has `S-021-DELIVERY.md` | Step G: `find .worktrees/S-021/.factory -type f` | Non-empty: teardown BLOCKED; PC2b error message |
| T-5 | Shadow tree has stray file; file relocated to canonical mount; re-run find | Step G retry: find returns empty | Teardown proceeds after relocation |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD) | _shared-context.md §Spec-Path Discipline clause explicitly covers `.factory/**` writes | manual: confirm clause present in S-21.04 skill-doc deliverable |
| (TBD) | Teardown preflight step present in step-g-cleanup.md | manual: confirm preflight sub-step in S-21.04 deliverable |
| (TBD) | Preflight blocks teardown when shadow `.factory/` contains files | bats: create stray `.factory/` file in test worktree fixture; invoke teardown; assert `git worktree remove` NOT called |
| (TBD) | Preflight passes when shadow `.factory/` is empty | bats: no stray files; assert teardown proceeds normally |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-036 |
| Capability Anchor Justification | CAP-036 registered in ARCH-INDEX v3.07 (ADR-031, commit 14a78515): "Story-Worktree Write-Path Discipline — factory artifact writes within story worktrees MUST target the canonical main-checkout `.factory/` mount via absolute paths; teardown preflight asserts shadow `.factory/` is empty before `git worktree remove`." BC-6.26.001 is the sole implementing BC for CAP-036 (INV-E21-002 + INV-E21-004). |
| L2 Domain Invariants | none (operational infrastructure) |
| Architecture Module | `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md` (write-discipline clause extension); `plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md` (preflight sub-step addition) |
| Stories | S-21.04 (E-21 Wave 2) |
| Source Issues | #523 (story-worktree `.factory` artifacts silently lost at teardown) |
| ADR Reference | none |

## Related BCs

- BC-6.10.002 — deliver-story 9-step dispatch; this BC adds write-path discipline and teardown-preflight requirements to steps within that sequence
- BC-6.27.001 — sibling BC governing factory-side PR worktree lifecycle (shared-mutable-worktree class; different trigger and fix vector)

## Architecture Anchors

- `plugins/vsdd-factory/skills/deliver-story/steps/_shared-context.md` — §Spec-Path Discipline to be extended with write-discipline clause (to be amended by S-21.04)
- `plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md` — teardown preflight sub-step to be added before `git worktree remove` dispatch (to be amended by S-21.04)

## Story Anchor

S-21.04 (E-21 Wave 2 — story-worktree factory artifact write-path discipline and teardown preflight)

## VP Anchors

TBD — VP IDs to be assigned after VP authoring pass.

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-07-19 | Initial authoring (product-owner; E-21 factory-state data-loss hardening; issue #523; S-21.04). PC1: write-path discipline — all `.factory/**` writes MUST use canonical absolute paths anchored to main-checkout root (INV-E21-002). PC2a/PC2b: teardown preflight — `find <worktree>/.factory -type f` before `git worktree remove`; non-empty result blocks teardown (INV-E21-004). 4 invariants. 7 edge cases EC-001..EC-007. 5 test vectors T-1..T-5. lifecycle_status: draft (POL-14 auto-promotion on S-21.04 PR merge). |
| 1.2 | 2026-07-19 | Research validation precision amendments (product-owner; research validation 2026-07-19). §Description: preflight rationale clarified — teardown preflight compensates for `git worktree remove --force` stripping stock git's unclean-worktree protection (git docs: "Only clean worktrees can be removed without --force"). Invariant 5 added: `--force` stripping mechanism documented; preflight as targeted replacement for factory-artifact category only. |
| 1.1 | 2026-07-19 | CAP-036 backfill (product-owner; ARCH-INDEX v3.07, ADR-031, commit 14a78515): capability frontmatter TBD→CAP-036; §Traceability L2 Capability TBD→CAP-036; Capability Anchor Justification updated to cite CAP-036/ARCH-INDEX v3.07. |
