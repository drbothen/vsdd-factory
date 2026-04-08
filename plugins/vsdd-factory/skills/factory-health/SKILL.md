---
name: factory-health
description: Validate and auto-repair the .factory/ worktree. Run at session start or when .factory/ state seems wrong. Checks orphan branch exists, worktree is mounted, and STATE.md is present.
disable-model-invocation: true
allowed-tools: Bash, Read, Write
---

# Factory Health Check

Validate that the `.factory/` worktree is properly mounted and healthy. Auto-repair common issues.

## Checks (run in order)

### 1. Orphan branch exists

```bash
git branch --list factory-artifacts
```

- **If missing**: Create it.
  ```bash
  git checkout --orphan factory-artifacts
  git rm -rf --cached . 2>/dev/null || true
  git commit --allow-empty -m "chore: initialize factory-artifacts orphan branch"
  git checkout -  # return to previous branch
  ```

### 2. Worktree is mounted

```bash
git worktree list | grep -F '.factory'
```

- **If missing**: Mount it.
  ```bash
  git worktree add .factory factory-artifacts
  ```

- **If mounted but pointing to wrong branch**: Remove and remount.
  ```bash
  git worktree remove .factory --force
  git worktree add .factory factory-artifacts
  ```

### 3. Worktree is on correct branch

```bash
cd .factory && git branch --show-current
```

- Must be `factory-artifacts`. If not, the worktree is corrupt — remove and remount.

### 4. STATE.md exists

```bash
test -f .factory/STATE.md
```

- **If missing**: Create initial STATE.md.
  ```yaml
  ---
  pipeline: INITIALIZED
  phase: pre-1
  product: corverax
  mode: greenfield
  timestamp: <current ISO8601>
  ---
  ```

### 5. Directory structure intact

Verify these directories exist inside `.factory/`:

```
specs/ specs/behavioral-contracts/ specs/verification-properties/
specs/architecture/ specs/prd-supplements/ stories/ cycles/
holdout-scenarios/ holdout-scenarios/wave-scenarios/
holdout-scenarios/evaluations/ semport/ code-delivery/
demo-evidence/ dtu-clones/
```

- **If any missing**: Create them with `.gitkeep`.

### 6. Reference repos (conditional)

```bash
test -f .factory/reference-manifest.yaml
```

- **If manifest exists**: This project has brownfield-ingested repos. Verify `.reference/` is populated:
  - Parse the manifest and check that each listed repo has a corresponding directory in `.reference/`.
  - Report any missing repos with their clone URL so the user can rebuild.
  - Report count: `Reference repos: <N>/<total> present`
- **If no manifest**: Skip — this is a from-scratch project.

### 7. Sync state

```bash
cd .factory && git status --porcelain
```

- **Clean**: All good.
- **Uncommitted changes**: Warn the user — there are uncommitted factory artifacts.
- **Diverged from remote**: Warn — manual resolution needed.

## Output

Report a summary:

```
Factory Health: ✓ HEALTHY
  Branch:    factory-artifacts (orphan)
  Worktree:  .factory/ mounted
  STATE.md:  present (phase: <current phase>)
  Structure: all directories present
  Sync:      clean | uncommitted changes | diverged
```

Or if repairs were made:

```
Factory Health: REPAIRED
  Fixed: <list of what was repaired>
  Current state: <summary>
```
