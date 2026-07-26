---
name: step-g-cleanup
description: Dispatch devops-engineer to remove the worktree and local branch, then update sprint state.
---

# Step G: Cleanup + State Update

> **Shared context:** Read `./_shared-context.md` before executing this step — it contains dispatch discipline and verification rules.

## Sub-step G.1: Worktree Cleanup

### Mandatory Teardown Preflight (BC-6.26.001 PC2, Invariants 2 and 5)

Before dispatching `devops-engineer` to remove the story worktree, the orchestrator MUST run a
`.factory/` inventory preflight on the worktree path. This step is mandatory with no exceptions —
not even when the agent is confident no shadow writes occurred (BC-6.26.001 Invariant 2).

Outcomes:

**PC2a — No stray files (teardown authorized):**

- *Sub-case (a) — `.factory/` path absent:* FIRST, test whether anything exists at the `.factory/`
  path in the story worktree using the normative existence predicate:

      [ ! -e "<worktree-path>/.factory" ]

  If this test passes (path absent), no stray factory artifacts can exist. This is the expected
  clean state (BC-6.26.001 EC-005; path-absent is NOT a PC2c error — a missing path is distinct
  from a `find` traversal error). Proceed to the Dispatch section below.

- *Sub-case (b) — `find` exits 0, empty output:* If `.factory/` exists AND is a real directory —
  not a symlink (confirmed by the guards below) — run `find "<worktree-path>/.factory/" -type f`
  (see the Preflight command section below). If `find` exits 0 with empty output, no stray factory
  artifacts exist. Proceed to the Dispatch section below.

**Symlink at `.factory/` path → PC2b BLOCKED (without running `find`):** If `[ ! -e ]` is FALSE
(something occupies the path), test whether the inode is a symbolic link:

    [ -L "<worktree-path>/.factory" ]

If this test passes, a symlink occupies the path. `test -d` follows symlinks — a symlink-to-dir
satisfies `[ -d ]` by dereferencing — but `find` does NOT descend symlinks, so a symlink-to-dir
at the path would produce empty find output and falsely authorize teardown. Go directly to
PC2b BLOCKED; list the path; do NOT invoke `find` on a symlink inode
(symlink → PC2b BLOCKED, BC-6.26.001 EC-008).

**Non-directory at `.factory/` path → PC2b BLOCKED (without running `find`):** If `[ -L ]` is
FALSE and the inode at `<worktree-path>/.factory` is NOT a directory — note: `[ ! -d ]` alone
MUST NOT be the sole check (it is wrong as the only predicate: it passes for regular files and
for symlink-to-dirs by dereferencing); it is valid here only after `[ ! -e ]` and `[ -L ]`
have confirmed occupancy and non-symlink above — this is stray shadow content subject to the
same rm-rf destruction risk as files inside a shadow `.factory/` directory tree. Go directly to
PC2b BLOCKED; list the path; do NOT invoke `find` on a non-directory inode
(non-directory → PC2b BLOCKED, BC-6.26.001 EC-008).

**PC2b — `find` returns paths, or a symlink or non-directory inode occupies the path (teardown BLOCKED):** If `find` returns one or more file
paths, or if the path-existence checks above yielded a symlink or non-directory inode, emit a
`PREFLIGHT BLOCKED` message for each stray path and HALT teardown. Do NOT proceed
to `git worktree remove`. Log each stray path using the following template:

    PREFLIGHT BLOCKED: Found factory artifact(s) in story worktree shadow .factory/:
      <path1>
      <path2>
      ...
    These files were written to the wrong worktree (issue #523 class) and would be
    permanently destroyed by git worktree remove. Manual intervention required:
      Option A: Relocate to canonical .factory/ mount, verify content, then retry teardown.
      Option B: Discard (only if files are confirmed redundant copies already committed on factory-artifacts).

Story cleanup MUST NOT complete until a retry preflight returns a PASS result.

**PC2c — Preflight error (fail-closed):** If `find` exits non-zero (any non-zero exit;
e.g., permission denial, traversal error), teardown MUST HALT. Surface the exact find exit code
and stderr to the operator. `git worktree remove` is NOT executed — find errors must not silently
authorize removal of unverified worktree content (BC-6.26.001 PC2c).

**Why this preflight is load-bearing — gitignored-shadow false-negative (BC-6.26.001 Invariant 5):**
`.factory/` is listed in `.gitignore` on the product branch, so the shadow `.factory/` content
inside the story worktree is **gitignored** (not untracked). Git's clean-state check inside
`git worktree remove` gates on untracked files only — gitignored files are explicitly excluded.
The check therefore passes silently as a false negative even when the shadow tree contains stray
factory artifacts, and the underlying `rm -rf <worktree-path>` destroys the gitignored shadow
content with no warning. The `find`-based preflight is load-bearing because `find` reads the
filesystem without gitignore filtering — it is the only mechanism that surfaces this class of
stray content before destruction. No git-level check (`git status`, `git ls-files`) would catch
gitignored content in this scenario.

**Preflight command (PC2a sub-case (b) — `.factory/` exists as real directory):** After the
discrimination chain above confirms `.factory/` is present and is a real directory (not a symlink,
not a non-directory inode), run:

    find "<worktree-path>/.factory/" -type f

If `find` exits 0 with empty output → PC2a sub-case (b): no stray factory artifacts; proceed to
the Dispatch below. If `find` exits 0 with non-empty output → PC2b BLOCKED. If `find` exits
non-zero → PC2c HALT.

### Dispatch (PC2a only — after a PASS preflight result)

**Agent:** `devops-engineer` (model tier: Fast)

**Task:** "Remove worktree `.worktrees/STORY-NNN/` and delete local branch `feature/STORY-NNN-<desc>`."

Dispatch this task ONLY after the preflight above returns a PASS result (PC2a per the §G.1
discrimination-chain protocol above). The devops-engineer must run plain `git worktree remove <worktree-path>`.

The `--force` flag is prohibited by BC-6.26.001 PC2a — it strips git's built-in unclean-worktree
protection for non-gitignored untracked files; this prohibition is a BC mandate, not a
guard-enforced constraint.

**Exit condition:** `git worktree list` no longer shows the worktree; `git branch --list 'feature/STORY-NNN-*'` returns empty for this story.

## Sub-step G.2: State Update

Update `.factory/stories/sprint-state.yaml`: story status → `merged`.
Update `.factory/stories/STORY-INDEX.md`: status column for this story.
Commit to `factory-artifacts` branch: `factory(phase-3): STORY-NNN delivered`.

## Artifacts

- Worktree removed
- Local branch deleted
- `sprint-state.yaml` updated
- `STORY-INDEX.md` updated
- Commit on factory-artifacts

## After Delivery

Tell the user:

```
Story STORY-NNN delivered:
  Red Gate:       PASSED (see .factory/cycles/<cycle-id>/<story-id>/implementation/red-gate-log.md)
  Implementation: <N> micro-commits
  Demos:          <N> artifacts in docs/demo-evidence/<STORY-ID>/
  PR:             #<N> merged to develop
  Worktree:       cleaned up
  State:          sprint-state.yaml updated

Next: /wave-gate wave-N when all wave stories are complete.
```
