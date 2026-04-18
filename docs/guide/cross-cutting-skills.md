# Cross-Cutting Skills

These skills are available at any point in the pipeline. They handle session management, navigation, quality validation, debt tracking, worktree management, and multi-repo coordination.

---

## Session Management

### `/vsdd-factory:activate`

Opt in to the VSDD factory persona for this project. Writes `.claude/settings.local.json` to set the orchestrator as the default main-thread agent. Reversible via `/vsdd-factory:deactivate`.

```
/vsdd-factory:activate
```

### `/vsdd-factory:deactivate`

Reverse `/vsdd-factory:activate` -- remove the orchestrator default agent from `.claude/settings.local.json`. Leaves the plugin enabled; only the default persona changes.

```
/vsdd-factory:deactivate
```

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

### `/vsdd-factory:artifact-detection`

Scans the project for existing planning artifacts (brief, PRD, architecture, UX spec, stories), validates their quality, identifies gaps, and routes to the correct pipeline entry point. This is the universal front-end for the VSDD pipeline -- it replaces the assumption that the human always arrives with a finished product brief.

```
/vsdd-factory:artifact-detection
```

### `/vsdd-factory:run-phase`

Execute a VSDD phase by reading its Lobster workflow file and spawning the declared sub-agents in dependency order. Use when you want to run one phase without activating the full orchestrator persona.

```
/vsdd-factory:run-phase <phase-id>
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

### `/vsdd-factory:validate-brief`

Validates a product brief against the required structure, quality, and context engineering criteria. Checks for both gaps (missing substance) and bloat (over-detail that wastes agent context budget). Produces a validation report with specific remediation guidance.

```
/vsdd-factory:validate-brief
```

### `/vsdd-factory:validate-workflow`

Schema-check a Lobster workflow file. Confirms required fields, `depends_on` resolution, and agent/skill references are valid. Run before committing workflow changes or when debugging a pipeline that fails to drive correctly.

```
/vsdd-factory:validate-workflow <file>
```

### `/vsdd-factory:implementation-readiness`

Validates that the complete spec package (PRD + architecture + stories) is internally consistent and ready for implementation. This is the gate between planning and building. Runs 6 validation dimensions as parallel checks.

```
/vsdd-factory:implementation-readiness
```

### `/vsdd-factory:consistency-validation`

Cross-document validation skill. Checks alignment between PRD, architecture, UX specs, stories, and implementation artifacts. Validates index files for structural completeness before loading detail files.

```
/vsdd-factory:consistency-validation
```

---

## Visual Tooling

### `/vsdd-factory:visual-companion`

Browser-based visual companion for showing mockups, diagrams, and interactive A/B choices during brainstorming, brief creation, and architecture design. Runs a local Node.js server that watches for HTML files and serves them with live reload and WebSocket-based interaction tracking.

```
/vsdd-factory:visual-companion
```

**Optional** -- requires Node.js. The early-phase skills (brainstorming, guided-brief-creation, create-architecture) automatically detect available visual tools using a tiered strategy:

| Tier | Tool | Requirement |
|------|------|-------------|
| 1 | `/vsdd-factory:visual-companion` | Node.js + user consent |
| 1 | `/vsdd-factory:visual-companion` (excalidraw) | Setup completed |
| 2 | `/vsdd-factory:create-excalidraw` | Always available |
| 3 | Mermaid code blocks | Always available |
| 4 | ASCII/text | Always available |

If the visual companion isn't available, skills fall back automatically. No hard dependency.

### `/vsdd-factory:create-excalidraw`

Generate `.excalidraw` JSON files for architecture diagrams, entity relationships, and flow charts. Files can be opened in excalidraw.com, VS Code (with the excalidraw extension), or rendered interactively in the visual companion browser.

```
/vsdd-factory:create-excalidraw
```

Includes element type reference (rectangle, ellipse, diamond, arrow, text), styling guide, layout helpers, and arrow binding documentation. Output to `.factory/diagrams/`.

### `/vsdd-factory:excalidraw-export`

Batch-render `.excalidraw` wireframe diagrams to pixel-perfect PNG using headless Firefox via Playwright.

Reference documentation, not directly invokable.

---

## Debugging

### `/vsdd-factory:systematic-debugging`

4-phase root cause investigation for any bug, test failure, or unexpected behavior. Enforces investigation before fixes.

```
/vsdd-factory:systematic-debugging
```

**The four phases:**

1. **Root Cause Investigation** -- read errors, reproduce, check changes, trace data flow
2. **Pattern Analysis** -- find working examples, compare, identify differences
3. **Hypothesis Testing** -- form single hypothesis, test minimally, one variable at a time
4. **Implementation** -- write failing test first, implement single fix, verify

**The 3-fix rule:** If 3+ fixes fail, this is an architectural problem, not a failed hypothesis. The skill escalates rather than continuing to guess.

BC-aware: when a bug violates a behavioral contract, the skill traces from the violated clause back to the implementation.

---

## Ideation & Planning

### `/vsdd-factory:brainstorming`

Guided brainstorming session that helps the human explore and refine product ideas. Uses structured techniques (SCAMPER, reverse brainstorming, mind mapping, constraint removal) to generate and evaluate options. Produces a brainstorming report that feeds into brief creation.

```
/vsdd-factory:brainstorming
```

### `/vsdd-factory:guided-brief-creation`

Interactive, facilitated workflow that guides the human from raw ideas to a structured product brief. Uses staged elicitation -- understand intent first, then fill sections through conversation, then draft and review.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:guided-brief-creation
```

### `/vsdd-factory:planning-research`

Conducts market, domain, and technical research to validate assumptions and fill knowledge gaps before brief or PRD creation. Uses Perplexity for web research and Context7 for library/framework documentation. Can run domain, market, or technical research independently or combined.

```
/vsdd-factory:planning-research
```

### `/vsdd-factory:market-intelligence-assessment`

Mandatory market intelligence assessment that runs before any spec work begins. Researches competitive landscape, validates customer pain, assesses market size, identifies differentiation opportunities, and flags risk signals. Produces a GO / CAUTION / STOP recommendation for human review.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:market-intelligence-assessment
```

---

## Spec Creation

### `/vsdd-factory:create-brief`

Create a product brief through guided discovery. Asks questions to understand the product vision, users, constraints, and success criteria. Writes to `.factory/specs/product-brief.md`.

```
/vsdd-factory:create-brief
```

### `/vsdd-factory:create-domain-spec`

Create a sharded L2 domain specification from the product brief. Models the problem domain -- entities, relationships, processes, invariants, and ubiquitous language. Writes to `.factory/specs/domain-spec/`.

```
/vsdd-factory:create-domain-spec
```

### `/vsdd-factory:create-prd`

Create a PRD with behavioral contracts from the product brief and domain spec. Elaborates requirements into testable contracts with error taxonomy and edge cases. Writes to `.factory/specs/prd.md` and supplements.

```
/vsdd-factory:create-prd
```

### `/vsdd-factory:create-architecture`

Create sharded architecture documents from PRD and behavioral contracts. Designs system architecture with ADR-style decisions, component diagrams, and purity boundaries. Writes to `.factory/specs/architecture/`.

```
/vsdd-factory:create-architecture
```

### `/vsdd-factory:phase-1-prd-revision`

Product Owner revises PRD v1 based on Architect's feasibility review. Incorporates architectural feedback, updates subsystem grouping if justified. Max 3 iterations before escalation to human.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:phase-1-prd-revision
```

### `/vsdd-factory:phase-1d-adversarial-spec-review`

Present complete spec package to the Adversary (different model family, fresh context) for adversarial review. Runs after all Phase 1 specs are complete.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:phase-1d-adversarial-spec-review
```

---

## Story Management

### `/vsdd-factory:decompose-stories`

Decompose PRD and architecture into epics, stories, dependency graph, and wave schedule. Creates sprint-ready story files in `.factory/stories/`. Requires completed Phase 1 specs.

```
/vsdd-factory:decompose-stories
```

### `/vsdd-factory:create-story`

Create or refine a single story spec with full acceptance criteria, tasks, and implementation details. Takes a story ID and produces a sprint-ready story file.

```
/vsdd-factory:create-story STORY-NNN
```

### `/vsdd-factory:wave-scheduling`

Computes wave-based implementation order from story dependencies. Groups stories into waves for parallel execution within each wave, then sub-partitions waves into parallel groups based on the one-story-per-agent rule.

```
/vsdd-factory:wave-scheduling
```

### `/vsdd-factory:wave-status`

Report current wave readiness from `.factory/stories/sprint-state.yaml`. Shows which stories are ready, in-progress, or blocked for the current wave.

```
/vsdd-factory:wave-status
```

### `/vsdd-factory:wave-gate`

Run the post-wave integration gate -- full test suite on develop, adversarial review of wave diff, holdout evaluation, demo evidence validation, and DTU validation for critical modules. Blocks next wave until all checks pass.

```
/vsdd-factory:wave-gate wave-N
```

---

## Implementation

### `/vsdd-factory:deliver-story`

Dispatches fresh specialist subagents (test-writer, implementer, demo-recorder, pr-manager, devops-engineer) via the per-story-delivery orchestrator workflow. Each step runs in isolated context to preserve reasoning quality.

```
/vsdd-factory:deliver-story STORY-NNN
```

### `/vsdd-factory:code-delivery`

Post-convergence code delivery workflow. Pushes verified code to remote, creates PRs with structured evidence, waits for CI, and executes merge based on autonomy level. Handles greenfield, brownfield, feature, and maintenance mode PRs.

```
/vsdd-factory:code-delivery
```

### `/vsdd-factory:demo-recording`

Records visual demonstrations of the target project for human review evidence, PR documentation, and regression baseline. Supports CLI (VHS), web (Playwright), API (cURL), and library (test harness) demos.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:demo-recording
```

### `/vsdd-factory:record-demo`

Record visual demo evidence for story acceptance criteria using Playwright. Captures screenshots or screen recordings of each acceptance criterion being satisfied.

```
/vsdd-factory:record-demo STORY-NNN
```

### `/vsdd-factory:holdout-eval`

Run holdout evaluation against merged wave code. Spawns the holdout-evaluator agent with strict information asymmetry -- cannot see specs, source internals, or prior reviews. Returns satisfaction scores per hidden scenario.

```
/vsdd-factory:holdout-eval wave-N
```

---

## Review

### `/vsdd-factory:adversarial-review`

Launch a fresh-context adversarial review of specs or implementation. Uses the adversary agent with information asymmetry to find gaps, contradictions, and missing edge cases. Minimum 2 passes to convergence.

```
/vsdd-factory:adversarial-review [specs|implementation]
```

### `/vsdd-factory:convergence-check`

Run 7-dimension convergence validation -- spec, tests, implementation, verification, visual, performance, documentation. Determines if the project is ready for release.

```
/vsdd-factory:convergence-check
```

### `/vsdd-factory:convergence-tracking`

Computes quantitative convergence metrics across adversarial passes, mutation testing runs, and formal verification results. Produces a CONVERGED or NOT_CONVERGED assessment with supporting metrics.

```
/vsdd-factory:convergence-tracking
```

---

## DTU (Digital Twin Universe)

### `/vsdd-factory:dtu-creation`

Creates behavioral clones of third-party services for the Digital Twin Universe. Agents build clones from API documentation, OpenAPI specs, and recorded traffic. Clones are packaged as Docker containers for use in testing and holdout evaluation.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:dtu-creation
```

### `/vsdd-factory:dtu-validate`

Digital Twin Universe validation -- create and maintain behavioral clones of critical subsystems for regression detection. DTU clones run in parallel with implementation to catch behavioral divergence early.

```
/vsdd-factory:dtu-validate
```

---

## Formal Verification

### `/vsdd-factory:formal-verify`

Run formal hardening -- Kani proofs for pure core functions, fuzz testing with cargo-fuzz, mutation testing with cargo-mutants, and security scanning with semgrep. Phase 5 quality gate.

```
/vsdd-factory:formal-verify
```

### `/vsdd-factory:perf-check`

Run performance validation -- benchmark regression checks, resource profiling, and budget compliance. Ensures no performance regression between implementations.

```
/vsdd-factory:perf-check
```

---

## Release

### `/vsdd-factory:release`

Config-driven release pipeline: version bump, CHANGELOG update, git tagging, GitHub Release, registry publishing. Reads `.factory/release-config.yaml`. Supports bootstrap (`init`), release, and dry-run modes.

```
/vsdd-factory:release [init | <version> | --dry-run]
```

---

## Debt & Tracking

### `/vsdd-factory:track-debt add`

Add a new item to the technical debt register at `.factory/tech-debt-register.md`. Assigns a `TD-NNN` identifier and records severity (critical/high/medium/low), category (design/performance/security/testing/documentation/dependency), source, impact, and effort estimate.

```
/vsdd-factory:track-debt add "Missing retry logic in API client -- deferred from STORY-005"
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

### `/vsdd-factory:spec-versioning`

Manages spec evolution using semantic versioning. Determines version bumps (MAJOR/MINOR/PATCH), maintains changelog, and detects drift between code and spec versions.

```
/vsdd-factory:spec-versioning
```

### `/vsdd-factory:traceability-extension`

Rules for extending the VSDD traceability chain when adding features incrementally. New links are appended, never replaced. Ensures every line of code traces through the 4-level specification hierarchy.

Reference documentation, not directly invokable.

---

## Research

### `/vsdd-factory:research`

Conduct external research via the research-agent using Perplexity, Context7, and Tavily MCP servers. Supports domain research (problem space analysis) and general research (technology evaluation, library comparison, security advisory lookup). Always cites sources and verifies library versions against registries.

```
/vsdd-factory:research domain "CQRS event sourcing patterns for Rust"
/vsdd-factory:research general "cargo-kani vs proptest for bounded model checking"
```

Produces `.factory/specs/vsdd-factory:research/domain-<topic>-<date>.md` or `.factory/specs/vsdd-factory:research/general-<topic>-<date>.md`.

### `/vsdd-factory:research-cache-ops`

Operate the research-cache for Perplexity/Context7 query results. Check, inspect, and clear cached research to avoid re-running expensive queries.

```
/vsdd-factory:research-cache-ops
```

---

## Multi-Repo

### `/vsdd-factory:multi-repo-health`

Scan `.worktrees/` for a multi-repo project layout and report detected repositories with their manifests (Cargo.toml, package.json, pyproject.toml, go.mod). Reports each repo's name, path, current branch, and git status. Cross-checks against `.factory/stories/` to warn about repos with no stories or stories targeting undetected repos. Read-only -- does not modify any repository.

```
/vsdd-factory:multi-repo-health
```

### `/vsdd-factory:multi-repo-phase-0-synthesis`

Synthesizes per-repo codebase ingestion outputs into a unified project-level context. Runs after all individual repo ingestions complete. Produces cross-repo dependency graph, unified architecture, convention reconciliation, unified security posture, cross-repo holdout scenarios, and unified project context.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:multi-repo-phase-0-synthesis
```

---

## Brownfield

### `/vsdd-factory:brownfield-ingest`

Analyze an existing codebase using the broad-then-converge analysis protocol. 6 broad passes, then iterative deepening on every pass until novelty decays to LOW. Produces a complete semantic understanding that feeds into spec crystallization.

```
/vsdd-factory:brownfield-ingest [codebase-path] [--resume]
```

### `/vsdd-factory:semport-analyze`

Semantic code porting -- scan a reference codebase, extract behavioral intent, and design a translation strategy to the target language. Supports full ingestion and incremental change-level analysis.

```
/vsdd-factory:semport-analyze [source-path] [target-language] [--incremental module-name]
```

---

## Feature Mode

### `/vsdd-factory:phase-f1-delta-analysis`

Feature Mode Phase F1: Analyze a feature request against existing artifacts to determine impact boundary, affected specs/stories/tests, and regression risk.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:phase-f1-delta-analysis
```

### `/vsdd-factory:phase-f2-spec-evolution`

Feature Mode Phase F2: Update specs incrementally -- PRD, architecture, and verification properties. Delta only, not full rewrite.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:phase-f2-spec-evolution
```

### `/vsdd-factory:phase-f3-incremental-stories`

Feature Mode Phase F3: Create new stories for the feature and integrate them into the existing dependency graph without cycles.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:phase-f3-incremental-stories
```

### `/vsdd-factory:phase-f4-delta-implementation`

Feature Mode Phase F4: TDD implementation scoped to new stories only, with full regression suite as safety net.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:phase-f4-delta-implementation
```

### `/vsdd-factory:phase-f5-scoped-adversarial`

Feature Mode Phase F5: Adversarial review scoped to changed/new code only. Fresh context, different model family.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:phase-f5-scoped-adversarial
```

### `/vsdd-factory:phase-f6-targeted-hardening`

Feature Mode Phase F6: Formal verification, fuzz testing, and mutation testing scoped to the delta. Full regression and security scans on full tree.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:phase-f6-targeted-hardening
```

### `/vsdd-factory:phase-f7-delta-convergence`

Feature Mode Phase F7: Five-dimensional convergence check on the delta plus regression validation on the full codebase. Final human gate.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:phase-f7-delta-convergence
```

### `/vsdd-factory:feature-mode-scoping-rules`

Reference document defining how scope is determined in Feature Mode. Defines what "the delta" is and how it stays fixed through all F1-F7 phases.

Reference documentation, not directly invokable.

### `/vsdd-factory:post-feature-validation`

After a feature ships, monitors feedback channels and analytics for signals about the feature's reception. Runs at configured intervals (7, 30, 90 days post-ship). Produces a feature impact report.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:post-feature-validation
```

---

## Design & UX

### `/vsdd-factory:design-system-bootstrap`

Bootstraps a design system for UI products. In greenfield: creates from product brief + brand guidelines. In brownfield: extracts from existing codebase. Produces design tokens, component registry, and constraints in `.factory/design-system/`.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:design-system-bootstrap
```

### `/vsdd-factory:multi-variant-design`

Generates 2-3 design variants for complex screens. Each variant scored on 6 dimensions by different agents. Top variant + runner-up presented to human for selection or synthesis.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:multi-variant-design
```

### `/vsdd-factory:design-drift-detection`

Detects design system drift during maintenance sweeps. Scans for token overrides, component misuse, pattern violations, and emergent patterns. Runs as Sweep 10 in maintenance (UI products only).

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:design-drift-detection
```

### `/vsdd-factory:responsive-validation`

Automated responsive testing at 4+ breakpoints for every screen. Captures screenshots, validates breakpoint-specific rules, and stores evidence in `.factory/ui-evidence/`.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:responsive-validation
```

### `/vsdd-factory:ui-completeness-check`

Validates UI completeness via traceability matrix. Tracks every UI element from UX spec through story, component, state, test, and visual evidence. Zero gaps required before convergence.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:ui-completeness-check
```

### `/vsdd-factory:ui-quality-gate`

Comprehensive UI quality gate that validates all dimensions: design system compliance, completeness, heuristics, accessibility, responsive, performance, visual regression, and state coverage. Strictness scales by pipeline point.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:ui-quality-gate
```

### `/vsdd-factory:ux-heuristic-evaluation`

Automated usability evaluation against Nielsen's 10 heuristics. Runs on UX specs (given UX spec document) and implemented UI (given running application after holdout evaluation). Includes cognitive walkthrough for key user tasks.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:ux-heuristic-evaluation
```

### `/vsdd-factory:storybook-mcp-integration`

Integrates Storybook MCP (@storybook/addon-mcp) as the UI validation backbone. Provides 6 tools to factory agents for component documentation, preview, and self-healing test loops. Installed during toolchain preflight for UI products.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:storybook-mcp-integration
```

---

## Discovery

### `/vsdd-factory:discovery-engine`

Autonomous discovery engine that continuously researches opportunities for both new features (existing products) and new product concepts. Evaluates ideas against structured criteria, facilitates planning document creation, and routes approved ideas to the appropriate development pipeline.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:discovery-engine
```

### `/vsdd-factory:competitive-monitoring`

Monitors competitor activity: new releases, feature announcements, pricing changes, funding rounds, acquisitions. Produces a competitive update report that feeds into the synthesis layer.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:competitive-monitoring
```

### `/vsdd-factory:customer-feedback-ingestion`

Ingests customer feedback from configured channels (GitHub issues, support tickets, app reviews, Slack/Discord). Categorizes, deduplicates, and produces a structured feedback digest.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:customer-feedback-ingestion
```

### `/vsdd-factory:analytics-integration`

Reads product analytics data (if available) to identify feature adoption, error patterns, and usage signals. Optional -- only runs if analytics sources are configured.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:analytics-integration
```

### `/vsdd-factory:intelligence-synthesis`

Correlates signals across market research, customer feedback, and usage analytics to produce scored insights. Clusters related signals into themes and calculates evidence strength.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:intelligence-synthesis
```

---

## Infrastructure

### `/vsdd-factory:repo-initialization`

Interactive repository creation flow. Orchestrator gathers requirements from the human, devops-engineer creates repo with standard configuration.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:repo-initialization
```

### `/vsdd-factory:toolchain-provisioning`

Dynamically installs the verification toolchain based on the target project's language(s). Reads `config/verification-toolchains.yaml` for the tool manifest and the architecture doc for language selection.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:toolchain-provisioning
```

### `/vsdd-factory:sdk-generation`

Generates production-ready SDKs in multiple languages from API contracts (OpenAPI, protobuf, or GraphQL schemas). Produces idiomatic code following each language's conventions.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:sdk-generation
```

### `/vsdd-factory:worktree-manage`

Create, list, or cleanup story worktrees in `.worktrees/`. Use when starting a new story, checking active worktrees, or cleaning up after merge.

```
/vsdd-factory:worktree-manage create STORY-NNN
/vsdd-factory:worktree-manage list
/vsdd-factory:worktree-manage cleanup STORY-NNN
```

---

## PR Management

### `/vsdd-factory:pr-create`

Create a pull request with story context, mermaid diagrams, and behavioral contract traceability. Generates a structured PR targeting develop.

```
/vsdd-factory:pr-create STORY-NNN
```

### `/vsdd-factory:fix-pr-delivery`

Streamlined delivery flow for fix PRs created during adversarial refinement, formal hardening, and convergence. Same rigor as story PRs (worktree, AI review, security review) but skips stubs, Red Gate, and wave integration gates.

```
/vsdd-factory:fix-pr-delivery
```

### `/vsdd-factory:pr-review-triage`

Finding classification and dispatch for PR review findings. Used by pr-manager to triage pr-reviewer comments into fix routes -- classifies each finding and routes it to the appropriate agent for resolution.

Orchestrator-managed -- run `/vsdd-factory:activate` first.

```
/vsdd-factory:pr-review-triage
```

---

## Documentation

### `/vsdd-factory:generate-pdf`

Generate a professional 1898 & Co. branded PDF from a markdown research document. Validates frontmatter, uses branded template, and reports results. Requires pandoc and weasyprint.

```
/vsdd-factory:generate-pdf
```

### `/vsdd-factory:agent-file-review`

Reviews AGENTS.md files for compliance with Dark Factory agent design principles. Checks token budget, contradictions, negative examples, FACTORY.md duplication, tool profile mismatches, and structural compliance with the canonical template.

```
/vsdd-factory:agent-file-review
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

---

## Reference

### `/vsdd-factory:jira`

Reference documentation for the ankitpokhrel jira-cli tool used for Jira integration in factory workflows.

Reference documentation, not directly invokable.

### `/vsdd-factory:model-routing`

Reference skill for LiteLLM model routing strategy. Documents which models serve which agents and how fallback chains work.

Reference documentation, not directly invokable.

### `/vsdd-factory:disposition-pass`

Re-examine ingested reference repos through the project vision lens to decide what to Model, Reimplement, Enhance, or Leave Behind. Produces per-repo disposition docs and a master rollup. Run after brownfield ingest and before spec crystallization.

```
/vsdd-factory:disposition-pass [<repo>|--all] [--rollup] [--update-vision]
```
