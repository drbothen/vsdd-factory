# Cross-Cutting Skills

These skills are available at any point in the pipeline. They handle session management, navigation, quality validation, debt tracking, worktree management, and multi-repo coordination.

---

## Session Management

### `/vsdd-factory:factory-health`

Validate and auto-repair the `.factory/` worktree. Run this at the start of every session. It checks that the `factory-artifacts` orphan branch exists, the worktree is mounted and on the correct branch, STATE.md is present, and the directory structure is intact. Missing directories are created automatically. Uncommitted changes in `.factory/` trigger a warning.

```
/vsdd-factory:factory-health
```

Produces a health summary: `Factory Health: HEALTHY` or `Factory Health: REPAIRED` with a list of fixes applied.

### `/vsdd-factory:factory-worktree-health`

Extended worktree validation that runs on every pipeline start as a blocking precondition. In addition to the checks in `/vsdd-factory:factory-health`, this skill verifies remote branch existence, local worktree mount validity, and sync state (in-sync, ahead, behind, or diverged). For multi-repo projects with a `project.yaml`, it also checks the `.factory-project/` worktree on the `factory-project-artifacts` branch.

```
/vsdd-factory:factory-worktree-health
```

Includes a workspace isolation guard that prevents accidental operation inside the engine repository instead of the target project.

### `/vsdd-factory:setup-env`

Validate and provision the development environment. Checks required tools (rustc 1.85+, cargo, rustfmt nightly, clippy, git, gh, just, jq), optional tools (cargo-kani, cargo-fuzz, cargo-mutants, cargo-deny, semgrep, lefthook, hyperfine), MCP server health (Perplexity, Tavily, Playwright, Tally), and git configuration. Run this on first setup or after tooling changes.

```
/vsdd-factory:setup-env
```

Produces a categorized status report showing installed versions, missing tools with install commands, and MCP server availability.

### `/vsdd-factory:scaffold-claude-md`

Auto-detect project context and generate a `CLAUDE.md` at the project root. Inspects your project for language markers (`Cargo.toml`, `package.json`, `pyproject.toml`, `go.mod`), task runners (`Justfile`, `Makefile`), CI configs, git branch strategy, and documentation. Presents a draft for confirmation before writing.

```
/vsdd-factory:scaffold-claude-md
```

The plugin provides methodology, principles, and rules automatically. This skill generates project-specific context only: build/test/lint commands, git workflow, and reference links. Re-run anytime to regenerate.

### `/vsdd-factory:session-review`

Post-pipeline analysis that reviews the complete factory run and produces improvement proposals. Analyzes 8 dimensions: cost, timing, convergence, agent behavior, gate outcomes, wall integrity, quality signals, and cross-run patterns. Proposals are categorized (cost, timing, convergence, agent, gate, wall, quality, pattern, workflow, template) and routed to the human for approval, deferral, or rejection.

```
/vsdd-factory:session-review
```

Produces `.factory/vsdd-factory:session-reviews/review-YYYY-MM-DD-[run-id].md` and `.factory/vsdd-factory:session-reviews/improvement-proposals-[run-id].md`. Maintains a cross-run pattern database and running benchmarks for comparison.

---

## Navigation

### `/vsdd-factory:next-step`

Read `.factory/STATE.md` and the active Lobster workflow, then propose the next step to execute. Cross-references completed steps against the workflow dependency graph to find the first uncompleted step whose dependencies are all satisfied. Reports the workflow file, step name, declared agent, and task description. Does not execute -- only proposes.

```
/vsdd-factory:next-step
```

### `/vsdd-factory:state-update`

Update `.factory/STATE.md` with pipeline phase transitions and commit to the `factory-artifacts` branch. This is an internal skill called by other skills at phase boundaries -- not invoked directly by users. Updates the YAML frontmatter (phase, pipeline status, timestamp) and appends to the phase history table.

### `/vsdd-factory:mode-decision-guide`

Decision guide for choosing between Greenfield Mode (new project from brief to release), Brownfield Mode (existing codebase ingestion plus greenfield overlay), and Feature Mode (post-v1 incremental changes). Run this when starting a new project or transitioning between modes.

```
/vsdd-factory:mode-decision-guide
```

### `/vsdd-factory:quick-dev-routing`

Route trivially-scoped changes through a compressed Feature Mode pipeline. Preserves regression testing and adversarial review but skips stubs, Red Gate, wave gates, and other ceremony that adds cost without value for small changes. Use for bug fixes and minor enhancements with verified zero blast radius.

```
/vsdd-factory:quick-dev-routing
```

---

## Quality

### `/vsdd-factory:validate-consistency`

Cross-file consistency validation across all factory artifacts. Checks BC ID integrity (every BC file is in the index, every referenced BC exists), VP ID integrity, story traceability (every story references a BC, every BC is referenced by a story), architecture cross-references, count consistency (index counts match actual files), status consistency, and naming consistency against the ubiquitous language.

```
/vsdd-factory:validate-consistency
```

Produces a consistency report with failures, warnings, and passed checks. Run after creating or modifying specs to catch stale references, broken IDs, and mismatched counts.

### `/vsdd-factory:spec-drift`

Compare implementation against spec documents to detect drift. Runs in a fresh context (forked agent) for objectivity. For each behavioral contract, finds the implementing code and verifies behavior matches. For each architecture decision, verifies the chosen option was implemented. Checks naming consistency against the domain spec. Identifies orphaned code (no spec coverage) and unimplemented specs (no code).

```
/vsdd-factory:spec-drift
```

Produces `.factory/cycles/<current>/vsdd-factory:spec-drift-report.md` with drift details, severity classification, and prioritized recommendations.

### `/vsdd-factory:maintenance-sweep`

Periodic quality sweep with 9 parallel tracks: dependency audit, documentation drift, pattern consistency, holdout scenario freshness, performance regression detection, DTU fidelity drift, spec coherence, tech debt register review, and accessibility regression (UI products only). Can be triggered on a schedule (recommended weekly), manually, or post-deploy.

```
/vsdd-factory:maintenance-sweep
```

Produces `.factory/maintenance/sweep-report-YYYY-MM-DD.md`. Automated fixes generate PRs through the standard quality gate pipeline. Manual fixes create issues with estimated effort.

---

## Debt Tracking

### `/vsdd-factory:track-debt add`

Add a new item to the technical debt register at `.factory/tech-debt-register.md`. Assigns a `TD-NNN` identifier and records severity (critical/high/medium/low), category (design/performance/security/testing/documentation/dependency), source, impact, and effort estimate.

```
/vsdd-factory:track-debt add "Missing retry logic in API client — deferred from STORY-005"
```

### `/vsdd-factory:track-debt list`

Display all active technical debt items in table format with ID, severity, category, description, effort, and source.

```
/vsdd-factory:track-debt list
```

### `/vsdd-factory:track-debt resolve`

Mark a debt item as resolved. Adds a resolution date and the PR or story that fixed it, then moves the item to the Resolved section.

```
/vsdd-factory:track-debt resolve TD-003
```

---

## Worktree Management

### `/vsdd-factory:worktree-manage create`

Create a new story worktree in `.worktrees/`. Validates the story exists in `.factory/stories/`, creates the worktree branching from `develop`, and names the branch `feature/STORY-NNN-<description>`.

```
/vsdd-factory:worktree-manage create STORY-007
```

### `/vsdd-factory:worktree-manage list`

List all active worktrees with their branch, status (clean/modified), and the factory worktree. Cross-references with `.worktrees/` directory contents.

```
/vsdd-factory:worktree-manage list
```

### `/vsdd-factory:worktree-manage cleanup`

Remove a story worktree after merge. Checks for uncommitted changes (aborts if dirty), verifies the branch is merged to develop, removes the worktree, and deletes the feature branch.

```
/vsdd-factory:worktree-manage cleanup STORY-007
```

---

## Multi-Repo

### `/vsdd-factory:multi-repo-health`

Scan `.worktrees/` for a multi-repo project layout and report detected repositories with their manifests (Cargo.toml, package.json, pyproject.toml, go.mod). Reports each repo's name, path, current branch, and git status. Cross-checks against `.factory/stories/` to warn about repos with no stories or stories targeting undetected repos. Read-only -- does not modify any repository.

```
/vsdd-factory:multi-repo-health
```

---

## Research

### `/vsdd-factory:research`

Conduct external research via the research-agent using Perplexity, Context7, and Tavily MCP servers. Supports domain research (problem space analysis) and general research (technology evaluation, library comparison, security advisory lookup). Always cites sources and verifies library versions against registries.

```
/vsdd-factory:research domain "CQRS event sourcing patterns for Rust"
/vsdd-factory:research general "cargo-kani vs proptest for bounded model checking"
```

Produces `.factory/specs/vsdd-factory:research/domain-<topic>-<date>.md` or `.factory/specs/vsdd-factory:research/general-<topic>-<date>.md`.

---

## Visual Tooling

### `/vsdd-factory:visual-companion`

Browser-based visual companion for showing mockups, diagrams, and interactive A/B choices during brainstorming, brief creation, and architecture design. Runs a local Node.js server that watches for HTML files and serves them with live reload and WebSocket-based interaction tracking.

```
/vsdd-factory:visual-companion
```

**Optional** — requires Node.js. The early-phase skills (brainstorming, guided-brief-creation, create-architecture) automatically detect available visual tools using a tiered strategy:

| Tier | Tool | Requirement |
|------|------|-------------|
| 1 | `/vsdd-factory:visual-companion` | Node.js + user consent |
| 1 | `/vsdd-factory:visual-companion` (excalidraw) | Setup completed |
| 2 | `/vsdd-factory:create-excalidraw` | Always available |
| 3 | Mermaid code blocks | Always available |
| 4 | ASCII/text | Always available |

If the visual companion isn't available, skills fall back automatically. No hard dependency.

---

## Debugging

### `/vsdd-factory:systematic-debugging`

4-phase root cause investigation for any bug, test failure, or unexpected behavior. Enforces investigation before fixes.

```
/vsdd-factory:systematic-debugging
```

**The four phases:**

1. **Root Cause Investigation** — read errors, reproduce, check changes, trace data flow
2. **Pattern Analysis** — find working examples, compare, identify differences
3. **Hypothesis Testing** — form single hypothesis, test minimally, one variable at a time
4. **Implementation** — write failing test first, implement single fix, verify

**The 3-fix rule:** If 3+ fixes fail, this is an architectural problem, not a failed hypothesis. The skill escalates rather than continuing to guess.

BC-aware: when a bug violates a behavioral contract, the skill traces from the violated clause back to the implementation.

---

## Wave Status

### `/vsdd-factory:wave-status`

Report current wave readiness from `.factory/stories/sprint-state.yaml`. Shows which stories are ready, in-progress, or blocked for the current wave.

```
/vsdd-factory:wave-status
```

---

## Plugin Development

### `/vsdd-factory:writing-skills`

TDD methodology for creating and maintaining plugin skills. Applies RED-GREEN-REFACTOR to process documentation: write pressure scenarios first, verify agents fail without the skill, then write the skill.

```
/vsdd-factory:writing-skills
```

Use when creating new skills for the vsdd-factory plugin or editing existing ones. The skill covers:
- When to create a skill vs adding to an existing agent
- SKILL.md structure and frontmatter conventions
- CSO (Claude Search Optimization) for skill discovery
- Pressure scenario testing methodology
- Rationalization resistance patterns (Red Flags tables, anti-pattern lists)
