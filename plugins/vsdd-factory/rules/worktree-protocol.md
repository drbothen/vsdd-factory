<!-- Worktree and branching rules for story development. -->

# Worktree Protocol

## Branch Strategy

```
main              ← production releases only
  └── develop     ← integration branch, PRs target here
       └── feature/S-N.MM-<desc>  ← per-story work
```

- `factory-artifacts` is an **orphan branch** — no relationship to main/develop.
- Never commit directly to `main` or `develop`.
- All story work happens in feature branches via worktrees.

## Story Worktrees

### Location

All story worktrees live in `.worktrees/` at the project root:

```
.worktrees/
├── S-1.01/    # git worktree, branch: feature/S-1.01-<desc>
├── S-1.02/
└── S-1.03/
```

### Creating a Worktree

```bash
git worktree add .worktrees/S-N.MM -b feature/S-N.MM-<desc> develop
```

- Always branch from `develop`.
- Branch name must match pattern: `feature/S-N.MM-<short-description>`.
- One worktree per story — never share worktrees between stories.

### Working in a Worktree

- All implementation for a story happens inside its worktree.
- Micro-commits per test pass (TDD progression visible in git history).
- Commit message format: `feat(S-N.MM): <description>` or `test(S-N.MM): <description>`.

### Merging a Story

1. All tests pass in the worktree.
2. PR created targeting `develop`.
3. PR reviewed (adversarial + code review).
4. Squash merge to `develop`.
5. Worktree removed: `git worktree remove .worktrees/S-N.MM`.
6. Branch cleaned up: `git branch -d feature/S-N.MM-<desc>`.

### Wave Integration

After all stories in a wave are merged to `develop`:

1. Full test suite passes on `develop`.
2. Adversarial review of wave diff.
3. Holdout evaluation runs against merged code.
4. Wave gate passes → next wave begins.

## Factory Worktree

The `.factory/` worktree is **permanent** — never remove it.

- Mounted on `factory-artifacts` orphan branch.
- Commits happen within `.factory/` directory.
- Validate health with `/factory-health` before each session.

## Cleanup Rules

- Remove worktrees promptly after merge — stale worktrees waste disk and cause confusion.
- Never force-remove a worktree with uncommitted changes — commit or stash first.
- `git worktree list` to audit active worktrees.
- `.worktrees/` is gitignored — worktrees are ephemeral, not tracked.
