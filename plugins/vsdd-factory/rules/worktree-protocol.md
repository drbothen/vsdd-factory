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
5. Run §G.1 preflight — **MUST** succeed before removal (plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md §G.1 — BC-6.26.001 PC2; caller-side per ADR-031).
   Worktree removed: `git worktree remove .worktrees/S-N.MM`.
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
- Always run the §G.1 preflight (plugins/vsdd-factory/skills/deliver-story/steps/step-g-cleanup.md §G.1, BC-6.26.001 PC2) before every
  `git worktree remove`. Git's own clean-state check is blind to gitignored shadow `.factory/`
  content (BC-6.26.001 Invariant 5) — stray factory artifacts in the worktree's shadow `.factory/`
  subtree pass git's check silently and are permanently destroyed at teardown. The §G.1 preflight
  closes this blind spot. Do NOT rely on git's built-in check as a substitute.

> **Gate-imposed authoring constraint (T-008 / F-S2104-P22-003):** The combined pattern `find … .factory/ … -type f` is forbidden in this file. Any text matching `find[[:space:]]+[^[:space:]]*\.factory/?[^[:space:]]*[[:space:]].*-type[[:space:]]+f` — including inside code fences — triggers the T-008 bats anti-pattern gate. Reference §G.1 step-g-cleanup.md for the authorized preflight command form.

- `git worktree list` to audit active worktrees.
- `.worktrees/` is gitignored — worktrees are ephemeral, not tracked.
