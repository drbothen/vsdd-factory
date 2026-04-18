# Configuration

This guide covers the directory layout, configuration files, and customization options
for the VSDD factory plugin.

---

## Directory layout

When the VSDD plugin is active on a project, it creates and manages a `.factory/` directory
inside your project root. This directory is a git worktree on the `factory-artifacts`
orphan branch.

```
your-project/
  src/                          # Your source code (on develop/main)
  tests/                        # Your tests (on develop/main)
  .factory/                     # Pipeline state (on factory-artifacts branch)
    STATE.md                    # Pipeline progress tracker (CRITICAL)
    specs/
      product-brief.md          # L1 product brief
      domain-spec/              # L2 domain specification (sharded)
      prd.md                    # L3 core PRD
      prd-supplements/          # Error taxonomy, interface defs, NFRs
      behavioral-contracts/     # BC-INDEX.md + BC-S.SS.NNN.md files
      verification-properties/  # VP-INDEX.md + VP-NNN.md files
      architecture/             # ARCH-INDEX.md + ARCH-NN-<section>.md files
      research/                 # Research reports
      module-criticality.md     # CRITICAL/HIGH/MEDIUM/LOW classification
    stories/
      STORY-INDEX.md            # Story registry
      STORY-NNN.md              # Individual stories
      epics.md                  # Epic groupings
      dependency-graph.md       # Story dependency graph
      sprint-state.yaml         # Current wave/story states
    cycles/
      vX.Y.Z-<mode>/           # Per-pipeline-run artifacts
        cycle-manifest.md
        adversarial-reviews/
        convergence-report.md
        wave-schedule.md
    holdout-scenarios/          # Hidden acceptance scenarios
      HS-INDEX.md
      wave-scenarios/
      evaluations/
    semport/                    # Brownfield ingestion artifacts
    demo-evidence/              # Per-story demo reports
    dtu-clones/                 # Digital twin universe clones
    tech-debt-register.md       # Tracked debt items
    reference-manifest.yaml     # External codebase references
  .reference/                   # Cloned reference codebases (gitignored)
  .worktrees/                   # Per-story worktrees (gitignored)
```

---

## The factory-artifacts branch

The `factory-artifacts` branch is an **orphan branch** -- it has no parent commit and shares
no history with `main` or `develop`. This keeps pipeline artifacts completely separate from
your source code.

### Setting it up manually

The `/vsdd-factory:factory-health` command creates this automatically. If you need to set it up manually:

```bash
# Create the orphan branch
git checkout --orphan factory-artifacts
git rm -rf --cached . 2>/dev/null || true
git commit --allow-empty -m "chore: initialize factory-artifacts orphan branch"
git checkout -

# Mount the worktree
git worktree add .factory factory-artifacts
```

### Committing to the factory

All `.factory/` changes commit to the `factory-artifacts` branch, not to `main` or `develop`:

```bash
cd .factory
git add -A
git commit -m "factory(phase-1): add product brief and domain spec"
```

Commit messages use the format `factory(<phase>): <description>`.

---

## STATE.md

`STATE.md` is the single source of truth for pipeline progress. Every skill reads it
before starting and updates it after phase transitions. The file tracks:

- Current phase and step
- Operating mode (greenfield, brownfield, feature, maintenance)
- Completed phases with timestamps
- Active wave and story statuses
- Cycle version identifier
- Known blockers

Example structure:

```markdown
# Pipeline State

## Current
- Phase: 3 (TDD Delivery)
- Mode: greenfield
- Cycle: v1.0.0-greenfield
- Wave: 2

## Completed
- Phase 0: skipped (greenfield)
- Phase 1: 2025-03-15 (specs locked, adversary converged pass 3)
- Phase 2: 2025-03-17 (12 stories, 3 waves)

## Wave 2 Status
- STORY-005: in-progress
- STORY-006: ready
- STORY-007: ready
```

Do not edit STATE.md by hand unless recovering from a corrupt state. Use `/vsdd-factory:state-update`
or let skills manage it automatically.

---

## reference-manifest.yaml

When `/vsdd-factory:brownfield-ingest` clones external codebases into `.reference/`, it records each
entry in `.factory/reference-manifest.yaml`:

```yaml
references:
  - name: dark-factory
    url: https://github.com/org/dark-factory
    commit: abc123f
    date: 2025-03-10
    path: .reference/dark-factory
```

This manifest is the source of truth for rebuilding `.reference/` on a new machine. The
actual cloned directories are gitignored.

---

## Plugin settings

### Activating the orchestrator

The plugin does not hijack your default Claude Code persona on install. To opt in to the
VSDD orchestrator agent for a specific project:

```
/activate
```

This writes `{"agent": "vsdd-factory:orchestrator"}` to `.claude/settings.local.json`.
To revert:

```
/deactivate
```

### Environment variables

| Variable | Purpose |
|----------|---------|
| `CLAUDE_PLUGIN_ROOT` | Set automatically by Claude Code. Points to the installed plugin directory. Used by skills and agents to resolve template paths. |

Skills reference templates as `${CLAUDE_PLUGIN_ROOT}/templates/<name>.md`. This variable
resolves correctly regardless of where the plugin is installed.

---

## Template customization

The plugin ships 108 templates in `plugins/vsdd-factory/templates/`. These define the
exact output format for every artifact type: behavioral contracts, architecture sections,
PRDs, adversarial findings, holdout evaluations, demo reports, convergence reports, and more.

Templates are read-only during pipeline execution. If you need to customize output formats:

1. The templates directory includes subdirectories for design-system templates and
   specialized artifact formats.
2. Skills reference templates by path from `${CLAUDE_PLUGIN_ROOT}/templates/`.
3. To override a template, you would need to fork the plugin and modify the template
   files directly. There is no per-project override mechanism.

---

## Hook behavior

The plugin registers 16 hooks across four lifecycle events. Each hook enforces a specific
discipline.

### PreToolUse hooks (Edit|Write)

| Hook | What it enforces |
|------|-----------------|
| `brownfield-discipline.sh` | Blocks edits to `.reference/**` (reference codebases are read-only) |
| `protect-vp.sh` | Blocks edits to verification properties with `Status: green` (immutable once proven) |
| `protect-bc.sh` | Blocks edits to behavioral contracts with `Status: green` |
| `red-gate.sh` | Enforces TDD red-before-green when `.factory/red-gate-state.json` declares strict mode |
| `factory-branch-guard.sh` | Blocks writes to `.factory/` when not mounted as worktree on `factory-artifacts` branch |

### PreToolUse hooks (Bash)

| Hook | What it enforces |
|------|-----------------|
| `destructive-command-guard.sh` | Blocks `rm -rf` on protected paths (.factory/, src/, tests/), `rm` on INDEX/STATE files, `git reset --hard`, `git clean -f`, `git checkout -- .` |
| `verify-git-push.sh` | Blocks force push (`--force`/`-f`) and direct push to protected branches (main, master, develop) |
| `check-factory-commit.sh` | Reminds to update STATE.md when committing to `.factory/` |

### PostToolUse hooks (Edit|Write)

| Hook | What it enforces |
|------|-----------------|
| `purity-check.sh` | Warns on side-effect patterns in pure-core paths (`*/pure/**`, `*/core/**`) |
| `validate-vp-consistency.sh` | Validates VP-INDEX ↔ verification-architecture ↔ coverage-matrix consistency (Policy 9) |
| `validate-subsystem-names.sh` | Verifies BC/story subsystem fields match ARCH-INDEX canonical names (Policy 6) |
| `validate-bc-title.sh` | Verifies BC file H1 heading matches BC-INDEX title (Policy 7) |
| `validate-story-bc-sync.sh` | Verifies story frontmatter bcs: ↔ body BC table ↔ AC traces bidirectional completeness (Policy 8) |

### PostToolUse hooks (Bash)

| Hook | What it enforces |
|------|-----------------|
| `regression-gate.sh` | Records test outcomes to `.factory/regression-state.json`, warns on pass-to-fail transitions |

### SubagentStop hooks

| Hook | What it enforces |
|------|-----------------|
| `handoff-validator.sh` | Warns on empty or truncated subagent output |

### Stop hooks

| Hook | What it enforces |
|------|-----------------|
| `session-learning.sh` | Appends session-end markers to `.factory/sidecar-learning.md` |

### Disabling hooks

Hooks cannot be individually disabled through configuration. They are wired in
`plugins/vsdd-factory/hooks/hooks.json`. To disable a hook, you would need to edit
`hooks.json` directly (not recommended -- the hooks exist to prevent common failure modes).

The `red-gate.sh` hook is opt-in: it only activates when `.factory/red-gate-state.json`
exists and declares strict mode. Other hooks are always active.

---

## Agent Permission Model

Each agent has a tool profile that determines what it can do. See [Agents Reference](agents-reference.md#agent-permission-model) for the full matrix.

Key points:
- **Spec producers** (product-owner, story-writer, architect) write markdown files but cannot execute shell commands. State-manager commits their work.
- **Code producers** (implementer, test-writer) have full shell access to compile, run tests, and commit in worktrees.
- **Tool-based reviewers** (accessibility-auditor) have `full` profile because they run automated tools (axe-core, lighthouse, pa11y) but still delegate commits to state-manager.
- **State-manager** has scoped shell access for git operations in `.factory/` only. It runs LAST in every burst to prevent version-race regressions.
- **pr-manager** delegates all GitHub CLI operations to github-ops via subagent dispatch.

The permission model is documented in [FACTORY.md](../../plugins/vsdd-factory/docs/FACTORY.md) under "Agent Permission Model."
