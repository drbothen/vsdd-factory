# Cross-Cutting Skills

These skills are available at any point in the pipeline. They handle session management, navigation, quality validation, debt tracking, worktree management, and multi-repo coordination.

---

## Session Management

### `/factory-health`

Validate and auto-repair the `.factory/` worktree. Run this at the start of every session. It checks that the `factory-artifacts` orphan branch exists, the worktree is mounted and on the correct branch, STATE.md is present, and the directory structure is intact. Missing directories are created automatically. Uncommitted changes in `.factory/` trigger a warning.

```
/factory-health
```

Produces a health summary: `Factory Health: HEALTHY` or `Factory Health: REPAIRED` with a list of fixes applied.

### `/factory-worktree-health`

Extended worktree validation that runs on every pipeline start as a blocking precondition. In addition to the checks in `/factory-health`, this skill verifies remote branch existence, local worktree mount validity, and sync state (in-sync, ahead, behind, or diverged). For multi-repo projects with a `project.yaml`, it also checks the `.factory-project/` worktree on the `factory-project-artifacts` branch.

```
/factory-worktree-health
```

Includes a workspace isolation guard that prevents accidental operation inside the engine repository instead of the target project.

### `/setup-env`

Validate and provision the development environment. Checks required tools (rustc 1.85+, cargo, rustfmt nightly, clippy, git, gh, just, jq), optional tools (cargo-kani, cargo-fuzz, cargo-mutants, cargo-deny, semgrep, lefthook, hyperfine), MCP server health (Perplexity, Tavily, Playwright, Tally), and git configuration. Run this on first setup or after tooling changes.

```
/setup-env
```

Produces a categorized status report showing installed versions, missing tools with install commands, and MCP server availability.

### `/session-review`

Post-pipeline analysis that reviews the complete factory run and produces improvement proposals. Analyzes 8 dimensions: cost, timing, convergence, agent behavior, gate outcomes, wall integrity, quality signals, and cross-run patterns. Proposals are categorized (cost, timing, convergence, agent, gate, wall, quality, pattern, workflow, template) and routed to the human for approval, deferral, or rejection.

```
/session-review
```

Produces `.factory/session-reviews/review-YYYY-MM-DD-[run-id].md` and `.factory/session-reviews/improvement-proposals-[run-id].md`. Maintains a cross-run pattern database and running benchmarks for comparison.

---

## Navigation

### `/next-step`

Read `.factory/STATE.md` and the active Lobster workflow, then propose the next step to execute. Cross-references completed steps against the workflow dependency graph to find the first uncompleted step whose dependencies are all satisfied. Reports the workflow file, step name, declared agent, and task description. Does not execute -- only proposes.

```
/next-step
```

### `/state-update`

Update `.factory/STATE.md` with pipeline phase transitions and commit to the `factory-artifacts` branch. This is an internal skill called by other skills at phase boundaries -- not invoked directly by users. Updates the YAML frontmatter (phase, pipeline status, timestamp) and appends to the phase history table.

### `/mode-decision-guide`

Decision guide for choosing between Greenfield Mode (new project from brief to release), Brownfield Mode (existing codebase ingestion plus greenfield overlay), and Feature Mode (post-v1 incremental changes). Run this when starting a new project or transitioning between modes.

```
/mode-decision-guide
```

### `/quick-dev-routing`

Route trivially-scoped changes through a compressed Feature Mode pipeline. Preserves regression testing and adversarial review but skips stubs, Red Gate, wave gates, and other ceremony that adds cost without value for small changes. Use for bug fixes and minor enhancements with verified zero blast radius.

```
/quick-dev-routing
```

---

## Quality

### `/validate-consistency`

Cross-file consistency validation across all factory artifacts. Checks BC ID integrity (every BC file is in the index, every referenced BC exists), VP ID integrity, story traceability (every story references a BC, every BC is referenced by a story), architecture cross-references, count consistency (index counts match actual files), status consistency, and naming consistency against the ubiquitous language.

```
/validate-consistency
```

Produces a consistency report with failures, warnings, and passed checks. Run after creating or modifying specs to catch stale references, broken IDs, and mismatched counts.

### `/spec-drift`

Compare implementation against spec documents to detect drift. Runs in a fresh context (forked agent) for objectivity. For each behavioral contract, finds the implementing code and verifies behavior matches. For each architecture decision, verifies the chosen option was implemented. Checks naming consistency against the domain spec. Identifies orphaned code (no spec coverage) and unimplemented specs (no code).

```
/spec-drift
```

Produces `.factory/cycles/<current>/spec-drift-report.md` with drift details, severity classification, and prioritized recommendations.

### `/maintenance-sweep`

Periodic quality sweep with 9 parallel tracks: dependency audit, documentation drift, pattern consistency, holdout scenario freshness, performance regression detection, DTU fidelity drift, spec coherence, tech debt register review, and accessibility regression (UI products only). Can be triggered on a schedule (recommended weekly), manually, or post-deploy.

```
/maintenance-sweep
```

Produces `.factory/maintenance/sweep-report-YYYY-MM-DD.md`. Automated fixes generate PRs through the standard quality gate pipeline. Manual fixes create issues with estimated effort.

---

## Debt Tracking

### `/track-debt add`

Add a new item to the technical debt register at `.factory/tech-debt-register.md`. Assigns a `TD-NNN` identifier and records severity (critical/high/medium/low), category (design/performance/security/testing/documentation/dependency), source, impact, and effort estimate.

```
/track-debt add "Missing retry logic in API client — deferred from STORY-005"
```

### `/track-debt list`

Display all active technical debt items in table format with ID, severity, category, description, effort, and source.

```
/track-debt list
```

### `/track-debt resolve`

Mark a debt item as resolved. Adds a resolution date and the PR or story that fixed it, then moves the item to the Resolved section.

```
/track-debt resolve TD-003
```

---

## Worktree Management

### `/worktree-manage create`

Create a new story worktree in `.worktrees/`. Validates the story exists in `.factory/stories/`, creates the worktree branching from `develop`, and names the branch `feature/STORY-NNN-<description>`.

```
/worktree-manage create STORY-007
```

### `/worktree-manage list`

List all active worktrees with their branch, status (clean/modified), and the factory worktree. Cross-references with `.worktrees/` directory contents.

```
/worktree-manage list
```

### `/worktree-manage cleanup`

Remove a story worktree after merge. Checks for uncommitted changes (aborts if dirty), verifies the branch is merged to develop, removes the worktree, and deletes the feature branch.

```
/worktree-manage cleanup STORY-007
```

---

## Multi-Repo

### `/multi-repo-health`

Scan `.worktrees/` for a multi-repo project layout and report detected repositories with their manifests (Cargo.toml, package.json, pyproject.toml, go.mod). Reports each repo's name, path, current branch, and git status. Cross-checks against `.factory/stories/` to warn about repos with no stories or stories targeting undetected repos. Read-only -- does not modify any repository.

```
/multi-repo-health
```

---

## Research

### `/research`

Conduct external research via the research-agent using Perplexity, Context7, and Tavily MCP servers. Supports domain research (problem space analysis) and general research (technology evaluation, library comparison, security advisory lookup). Always cites sources and verifies library versions against registries.

```
/research domain "CQRS event sourcing patterns for Rust"
/research general "cargo-kani vs proptest for bounded model checking"
```

Produces `.factory/specs/research/domain-<topic>-<date>.md` or `.factory/specs/research/general-<topic>-<date>.md`.

---

## Wave Status

### `/wave-status`

Report current wave readiness from `.factory/stories/sprint-state.yaml`. Shows which stories are ready, in-progress, or blocked for the current wave.

```
/wave-status
```
