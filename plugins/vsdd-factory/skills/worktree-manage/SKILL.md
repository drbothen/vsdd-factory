---
name: worktree-manage
description: Create, list, or cleanup story worktrees in .worktrees/. Use when starting a new story, checking active worktrees, or cleaning up after merge. Usage - /worktree-manage create STORY-NNN, /worktree-manage list, /worktree-manage cleanup STORY-NNN
argument-hint: "[create|list|cleanup] [STORY-NNN]"

allowed-tools: Bash, Read
---

# Worktree Manager

Manage per-story git worktrees in `.worktrees/`.

## Commands

Parse `$ARGUMENTS` to determine the action:

### `create STORY-NNN [description]`

1. **Validate** story exists in `.factory/stories/STORY-NNN.md` (warn if not, but allow creation anyway).

2. **Check** worktree doesn't already exist:
   ```bash
   test -d .worktrees/STORY-NNN
   ```

3. **Ensure** `develop` branch exists. If not, create from `main`:
   ```bash
   git branch --list develop || git branch develop main
   ```

4. **Create** the worktree:
   ```bash
   mkdir -p .worktrees
   git worktree add .worktrees/STORY-NNN -b feature/STORY-NNN-<description> develop
   ```
   - If no description provided, use a slug from the story title.
   - Branch name: `feature/STORY-NNN-<short-kebab-case-desc>`

5. **Update** `.factory/stories/sprint-state.yaml` with worktree path (if file exists).

6. **Report**:
   ```
   Worktree created:
     Path:   .worktrees/STORY-NNN/
     Branch: feature/STORY-NNN-<desc>
     Base:   develop
   ```

### `list`

1. **List** all active worktrees:
   ```bash
   git worktree list
   ```

2. **Cross-reference** with `.worktrees/` directory contents.

3. **Report** in table format:
   ```
   Active Worktrees:
     STORY-NNN  feature/STORY-NNN-desc  [clean|modified]
     STORY-NNN  feature/STORY-NNN-desc  [clean|modified]
   
   Factory:
     .factory/  factory-artifacts  [clean|modified]
   ```

### `cleanup STORY-NNN`

1. **Check** for uncommitted changes:
   ```bash
   cd .worktrees/STORY-NNN && git status --porcelain
   ```
   - If dirty: **abort** and warn. User must commit or stash first.

2. **Check** if branch is merged to develop:
   ```bash
   git branch --merged develop | grep -F "feature/STORY-NNN"
   ```
   - If not merged: **warn** but allow cleanup if user confirms.

3. **Preflight required (BC-6.26.001 PC2, INV-E21-004):** Run the
   `plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md §G.1` preflight; proceed only on
   a PASS result (the §G.1 discrimination-chain protocol). Note: the `git status --porcelain` check above is
   BLIND to gitignored shadow `.factory/` content — the §G.1 preflight is the only gate that catches it
   (BC-6.26.001 Invariant 5).

   > **Gate-imposed authoring constraint (T-008 / F-S2104-P22-003):** The combined pattern `find … .factory/ … -type f` is forbidden in this file. Any text matching `find[[:space:]]+[^[:space:]]*\.factory/?[^[:space:]]*[[:space:]].*-type[[:space:]]+f` — including inside code fences — triggers the T-008 bats anti-pattern gate. Reference §G.1 step-g-cleanup.md for the authorized preflight command form.

   **Remove** worktree (only after PASS preflight result):
   ```bash
   git worktree remove .worktrees/STORY-NNN
   ```

4. **Delete** branch (if merged):
   ```bash
   git branch -d feature/STORY-NNN-<desc>
   ```

5. **Report**:
   ```
   Worktree cleaned up:
     Removed: .worktrees/STORY-NNN/
     Branch:  feature/STORY-NNN-<desc> deleted
   ```

## Error Handling

- If `.worktrees/` doesn't exist, create it.
- If `develop` doesn't exist and `main` is the only branch, create `develop` from `main`.
- Never force-remove a worktree with uncommitted changes.
- Never delete an unmerged branch without user confirmation.
