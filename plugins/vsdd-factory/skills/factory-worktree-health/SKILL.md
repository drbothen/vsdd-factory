---
name: factory-worktree-health
description: >
  Validates factory worktree health on pipeline start. Checks .factory/ (all modes)
  and .factory-project/ (multi-repo only). Verifies remote branch existence, local
  worktree mount, and sync state. Auto-repairs if possible. Runs on ALL pipeline
  modes (greenfield, brownfield, feature, resume). Executed by devops-engineer.
---

# Factory Worktree Health Check

## When This Skill Runs

On **every pipeline start**, before mode detection or any `.factory/` reads.
This is a blocking precondition — no pipeline work proceeds until this passes.

Applies to: greenfield, brownfield, feature mode, and pipeline resume.

## Worktrees to Check

| Worktree | Branch | Required When |
|----------|--------|---------------|
| `.factory/` | `factory-artifacts` | Always (all modes) |
| `.factory-project/` | `factory-project-artifacts` | Multi-repo only (project.yaml exists) |

Run the health check sequence below for **each required worktree**.
If `project.yaml` exists in the repo root, check BOTH worktrees.
If `project.yaml` does not exist, check only `.factory/`.

## Executor

devops-engineer (requires `exec` tool for git commands)

## Health Check Sequence

### Step 0: Workspace Isolation Guard (BLOCKING)

Before ANY git commands, verify you are in the target project — NOT the engine:

```bash
# 1. Confirm cwd is NOT inside dark-factory
CWD=$(pwd)
if [[ "$CWD" == *"dark-factory"* ]]; then
  echo "FATAL: Running in dark-factory engine directory ($CWD). Refusing to proceed."
  echo "Fix: orchestrator must set cwd to the resolved project path in Agent tool."
  exit 1
fi

# 2. Verify git remote points to the target project, not the engine
REMOTE_URL=$(git remote get-url origin 2>/dev/null || echo "none")
if [[ "$REMOTE_URL" == *"dark-factory"* ]]; then
  echo "FATAL: Git remote ($REMOTE_URL) points to dark-factory engine repo."
  echo "Fix: you are in the wrong repository."
  exit 1
fi

# 3. If .factory/ exists, verify its .git file points to this repo (not engine)
if [[ -f .factory/.git ]]; then
  GITDIR_PATH=$(cat .factory/.git | sed 's/gitdir: //')
  if [[ "$GITDIR_PATH" == *"dark-factory"* ]]; then
    echo "FATAL: .factory/.git points to dark-factory ($GITDIR_PATH)."
    echo "Fix: remove .factory/ and re-run worktree setup."
    exit 1
  fi
fi
```

If ANY check fails, report `FACTORY_WORKTREE_HEALTH: FAIL` with the error and STOP.

Run the following steps for each worktree/branch pair. Use these variables:

| Variable | `.factory/` | `.factory-project/` |
|----------|-------------|---------------------|
| `WORKTREE_DIR` | `.factory` | `.factory-project` |
| `BRANCH_NAME` | `factory-artifacts` | `factory-project-artifacts` |

### Step 1: Check Remote Branch

```bash
git ls-remote --heads origin ${BRANCH_NAME}
```

- **Branch exists:** Proceed to Step 2
- **Branch does NOT exist:** Create it:
  ```bash
  git checkout --orphan ${BRANCH_NAME}
  git rm -rf .
  git commit --allow-empty -m "chore: initialize ${BRANCH_NAME} branch"
  git push origin ${BRANCH_NAME}
  git checkout develop
  ```
  Then proceed to Step 2.

### Step 2: Check Local Worktree

```bash
git -C ${WORKTREE_DIR} rev-parse --git-dir 2>/dev/null
```

- **Succeeds (worktree is valid):** Proceed to Step 3
- **Fails or directory doesn't exist:**
  - If directory exists as a regular directory (no `.git` marker):
    ```bash
    mv ${WORKTREE_DIR} ${WORKTREE_DIR}-backup-$(date +%s)
    ```
  - Mount the worktree:
    ```bash
    git worktree add ${WORKTREE_DIR} ${BRANCH_NAME}
    ```
  - If backup was created and had contents:
    ```bash
    cp -r ${WORKTREE_DIR}-backup-*/* ${WORKTREE_DIR}/ 2>/dev/null || true
    rm -rf ${WORKTREE_DIR}-backup-*
    ```
  - Proceed to Step 3

### Step 3: Verify Worktree Branch

```bash
git -C ${WORKTREE_DIR} branch --show-current
```

- **Shows `${BRANCH_NAME}`:** Proceed to Step 4
- **Shows anything else:** STOP. Report error:
  ```
  ERROR: ${WORKTREE_DIR}/ worktree is on branch '[branch]' instead of '${BRANCH_NAME}'.
  Manual intervention required:
    git worktree remove ${WORKTREE_DIR}
    git worktree add ${WORKTREE_DIR} ${BRANCH_NAME}
  ```

### Step 4: Check Sync State

```bash
git -C ${WORKTREE_DIR} fetch origin ${BRANCH_NAME} 2>/dev/null
LOCAL=$(git -C ${WORKTREE_DIR} rev-parse HEAD)
REMOTE=$(git -C ${WORKTREE_DIR} rev-parse origin/${BRANCH_NAME} 2>/dev/null || echo "none")
BASE=$(git -C ${WORKTREE_DIR} merge-base HEAD origin/${BRANCH_NAME} 2>/dev/null || echo "none")
```

Evaluate:

| Condition | Meaning | Action |
|-----------|---------|--------|
| `LOCAL == REMOTE` | In sync | Healthy — proceed |
| `REMOTE == "none"` | Remote branch has no commits yet | Healthy (fresh init) — proceed |
| `LOCAL != REMOTE` and `BASE == REMOTE` | Local is ahead | Warn and push: `git -C ${WORKTREE_DIR} push origin ${BRANCH_NAME}` |
| `LOCAL != REMOTE` and `BASE == LOCAL` | Local is behind | Pull: `git -C ${WORKTREE_DIR} pull --ff-only origin ${BRANCH_NAME}` |
| `LOCAL != REMOTE` and `BASE != LOCAL` and `BASE != REMOTE` | Diverged | STOP. Report error with both SHAs. Human must resolve. |

### Step 5: Report Result

Report to orchestrator (one report per worktree checked):

```
FACTORY_WORKTREE_HEALTH: PASS
  Worktree: ${WORKTREE_DIR}/
  Remote branch: ${BRANCH_NAME} (exists)
  Local worktree: ${WORKTREE_DIR}/ (mounted)
  Sync state: [in-sync | pushed-local | pulled-remote | fresh-init]
  Commits: [N] on ${BRANCH_NAME}
```

Or on failure:

```
FACTORY_WORKTREE_HEALTH: FAIL
  Worktree: ${WORKTREE_DIR}/
  Error: [description]
  Recovery: [actionable command]
```

## Failure Is Blocking

If this skill reports FAIL for ANY worktree, the orchestrator MUST NOT proceed.

`.factory/` is required for:
- STATE.md (state-manager)
- All spec artifacts (Phase 1+)
- Artifact backup at phase gates
- Pipeline resume/crash recovery

## Output

Health check report to orchestrator (one per worktree):
```
FACTORY_WORKTREE_HEALTH: PASS | FAIL
  Worktree: [dir]
  Remote branch: [name] (exists | created)
  Sync state: [in-sync | pushed-local | pulled-remote | fresh-init]
```

## Quality Gate

- [ ] FACTORY_WORKTREE_HEALTH: PASS reported for `.factory/` worktree
- [ ] FACTORY_WORKTREE_HEALTH: PASS reported for `.factory-project/` worktree (if multi-repo)
- [ ] Sync state resolved (in-sync, pushed, or pulled) for all required worktrees
- [ ] No manual intervention required (all auto-repairs succeeded)

`.factory-project/` is required for (multi-repo only):
- Project-level STATE.md (cross-repo coordination)
- Unified specs (L1 brief, L2 domain spec, L3 PRD)
- Cross-repo dependency graph and wave plans
- Cross-repo integration gate results
- Project-level cost tracking
