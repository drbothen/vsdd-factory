---
name: code-delivery
description: >
  Post-convergence code delivery workflow. Pushes verified code to remote,
  creates PRs with structured evidence, waits for CI, and executes merge
  based on autonomy level. Handles greenfield, brownfield, feature, and
  maintenance mode PRs.
---

# Code Delivery: From Convergence to Merge

## When This Skill Runs

- **Greenfield Mode:** Per story within each delivery wave, and for adversarial/hardening/convergence fix PRs
- **Feature Mode:** After Phase F7 delta convergence
- **Maintenance Mode:** After sweep findings pass quality gates (DF-008)
- **Multi-Repo Mode:** Per story within each target repo

## Prerequisites

- Story implementation complete: all tests pass in the worktree
- Worktree exists at `.worktrees/STORY-NNN/` with feature branch
- Demo evidence recorded in worktree (docs/demo-evidence/<STORY-ID>/)
- `.factory/merge-config.yaml` exists (or defaults apply)

## Workflow

### Step 1: Prepare Branch (in Worktree)

Verify the worktree has a clean, pushable state:

1. Read `.factory/STATE.md` to confirm story implementation status
2. Verify the worktree branch exists: `git -C .worktrees/STORY-NNN branch --show-current`
3. Verify all tests pass: run the project's test command from the worktree
4. Verify no uncommitted changes: `git -C .worktrees/STORY-NNN status --porcelain`
5. If uncommitted changes exist, commit with conventional commit message

### Step 2: Per-Story Demo Recording (in Worktree)

Before creating the PR, record demo evidence in the worktree:

1. Read STORY-NNN.md acceptance criteria
2. Detect product type:
   - CLI/TUI (Cargo.toml, no web frontend): use VHS `.tape` → `.gif` + `.webm`
   - Web (package.json with React/Next/Vue): use Playwright `.spec.ts` → `.webm` + `.png`
   - Full-stack: VHS for CLI parts, Playwright for web parts
3. For each AC: create recording script from template, execute, verify output
4. Record BOTH success AND error paths for each AC
5. Generate `docs/demo-evidence/<STORY-ID>/evidence-report.md`
6. Commit to feature branch:
   ```bash
   cd .worktrees/STORY-NNN/
   git add docs/demo-evidence/<STORY-ID>/
   git commit -m "evidence(STORY-NNN): add demo recordings"
   ```

Demo artifacts go to `docs/demo-evidence/<STORY-ID>/` (committed to feature branch, NOT `.factory/`).
The per-story subfolder prevents evidence-report.md collisions across stories.
They appear in the PR diff — `.gif` files render inline on GitHub.

**Gate:** `docs/demo-evidence/<STORY-ID>/evidence-report.md` exists, at least 1 recording
(`.gif`/`.webm`, NOT `.txt`) per AC, both success and error paths.

### Step 3: Push Branch to Remote

Push the worktree branch using `--force-with-lease` for safety:

```bash
git -C .worktrees/STORY-NNN push --force-with-lease origin feature/STORY-NNN
```

**Gate:** The `code-delivery` plugin's `before_tool_call` hook verifies tests have passed
before allowing `git push`. If tests haven't passed, the push is blocked with
`{ skip: true, reason: "Tests must pass before push" }`.

### Step 4: Create PR with Structured Description

The **pr-manager** spawns **github-ops** to create the pull request:

```
Agent(subagent_type="vsdd-factory:github-ops", prompt="cd <project-path> && gh pr create --title 'feat(STORY-NNN): [story title]' --body-file <project-path>/.factory/code-delivery/STORY-NNN/pr-description.md --base develop --head feature/STORY-NNN")
```

The PR description is generated from `templates/pr-description-template.md`,
populated with data from `.factory/` artifacts:

- **Spec traceability:** Read from `.factory/stories/stories/STORY-NNN.md`
- **Test evidence:** Read from test execution output + coverage report
- **Demo evidence:** Embedded GIF thumbnails from `docs/demo-evidence/<STORY-ID>/`
- **Mermaid diagrams:**
  - Architecture change diagram (graph TD)
  - Story dependency diagram (graph LR, showing merged/pending/blocked)
  - Spec traceability flowchart (BC -> AC -> Test -> Code)
- **Test coverage summary table** (unit tests, coverage %, mutation kill rate, holdout)
- **Adversarial review evidence:** (if adversarial refinement available)
- **Convergence metrics:** (if convergence available)

### Step 5: AI PR Review (4th Model Family)

The **pr-reviewer** reviews the PR diff with fresh context using a different
model family from implementer, adversary, and code-reviewer:

**Review Checklist (8 items):**
1. Diff coherence -- all changes relate to this story
2. Description accuracy -- PR body matches actual changes
3. Test coverage -- changed lines have test coverage
4. Demo evidence -- recordings match acceptance criteria
5. Commit quality -- conventional format, story ID
6. Diff size -- flag if >500 lines
7. Missing changes -- story spec vs diff
8. Dependency status -- upstream PRs merged

**Verdict:** APPROVE / REQUEST_CHANGES / COMMENT

pr-reviewer spawns github-ops to post findings as inline PR comments directly,
and writes findings to `<project-path>/.factory/code-delivery/STORY-NNN/pr-review.md`.

### Step 6: Review Convergence Loop (Max 10 Cycles)

If pr-reviewer returns REQUEST_CHANGES:

1. **Triage** (pr-manager): classify findings by severity/category, route to agents
2. **Fix** (implementer/test-writer/demo-recorder): fix in worktree, push
3. **Re-review** (pr-reviewer): evaluate fixes, post updated verdict

Track convergence in `.factory/code-delivery/STORY-NNN/review-findings.md`:

| Cycle | Findings | Blocking | Fixed | Remaining |
|-------|----------|----------|-------|-----------|
| 1 | 5 | 3 | 5 | 0 |
| 2 | 1 | 0 | 1 | 0 |
| 3 | 0 | 0 | 0 | 0 -> APPROVE |

After 10 cycles with blocking findings: escalate to human.

### Step 7: Wait for CI Status Checks

Poll GitHub for CI status via github-ops until all required checks pass or fail:

- Maximum 30 minute wait, poll every 30 seconds
- Maximum 3 CI fix cycles; escalate to human after 3 failures

**If CI fails:**
- Read the failing check's log via `gh run view`
- Route failure details to the implementer agent for fixing
- The implementer pushes fixes to the same branch
- CI re-runs automatically

**Important:** CI failure is NOT the same as adversarial review failure. CI failures
indicate environment-specific issues -- not code quality problems.

### Step 8: Dependency-Ordered Merge Check

Before merge, the pr-manager verifies dependency ordering:

1. Read STORY-INDEX.md `depends_on` for this story
2. Check all dependency PRs via github-ops: `gh pr view DEP_PR --json state`
3. If any dependency PR is NOT merged: WAIT
4. After all deps merged: rebase onto develop
5. Re-run tests in worktree
6. If tests pass: proceed to merge
7. If tests fail: fix, push, re-verify

### Step 9: Merge Decision (Autonomy-Level-Driven)

Read `.factory/merge-config.yaml` for the current autonomy level:

**Level 3 (Human Review):**
- Add `needs-review` label to the PR
- Wait for human approval

**Level 3.5 (Conditional Auto-Merge):**
- **Low risk** (no restricted files, coverage >= 80%, no security findings): Auto-merge
- **Medium risk** (restricted files OR coverage < 80%): Require human review
- **High risk** (security-critical files, breaking changes): Always require human

**Level 4 (Full Auto-Merge):**
- Auto-merge with squash if CI passes
- Feature flag wrapping recommended for high-risk changes

pr-manager spawns github-ops to merge:
```
Agent(subagent_type="vsdd-factory:github-ops", prompt="cd <project-path> && gh pr merge PR_NUMBER --squash --delete-branch")
```

### Step 10: Post-Merge Cleanup

After successful merge (delegated to devops-engineer):

1. Remote branch deleted (`--delete-branch` handles this)
2. Remove local worktree: `git worktree remove .worktrees/STORY-NNN`
3. Update `.factory/STATE.md` with merge status, PR number, and timestamp
4. Write delivery report to `.factory/code-delivery/STORY-NNN/delivery.md`

### Step 11: Merge Conflict Resolution

If the PR has merge conflicts (another story merged first):

1. Fetch latest: `git -C .worktrees/STORY-NNN fetch origin develop`
2. Rebase: `git -C .worktrees/STORY-NNN rebase origin/develop`
   (git rerere handles previously-seen conflicts automatically)
3. If rebase succeeds: push with `--force-with-lease` and re-run CI
4. If rebase fails: resolve conflicts manually, then continue
5. Maximum 3 rebase attempts; escalate to human after 3 failures

## Output Artifacts

Per-story delivery artifacts:
```
.factory/code-delivery/STORY-NNN/
  ├── branch-readiness.md        # Pre-push verification
  ├── pr-description.md          # Generated PR body
  ├── pr-number.txt              # PR number (e.g., "42")
  ├── review-findings.md         # Convergence tracking
  ├── merge-decision.md          # Autonomy level decision
  └── delivery.md                # Post-merge delivery report
```

Committed to feature branch (in PR diff):
```
docs/demo-evidence/<STORY-ID>/
  ├��─ AC-001-[description].gif
  ├── AC-001-[description].webm
  ├── AC-001-[description].tape
  └── evidence-report.md
```

PR on GitHub with structured description, Mermaid diagrams, labels, and review status.
