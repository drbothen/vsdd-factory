# Commands Reference

The vsdd-factory plugin ships 47 slash commands. Each command dispatches to a corresponding skill. Run any command by typing its name in Claude Code.

All commands are prefixed with `/` when invoked. If you have multiple plugins installed, use the fully qualified form `/vsdd-factory:<command-name>` to avoid ambiguity.

Commands are organized by pipeline phase below. Cross-cutting commands are available at any phase.

---

## Phase 0: Brownfield Ingest

| Command | Arguments | Description |
|---------|-----------|-------------|
| `/vsdd-factory:brownfield-ingest` | `[codebase-path] [--resume]` | Analyze an existing codebase using the broad-then-converge protocol; produces complete semantic understanding feeding into spec crystallization. |
| `/vsdd-factory:disposition-pass` | `[<repo>\|--all] [--rollup] [--update-vision]` | Re-examine ingested reference repos through the vision lens to decide what to Model, Reimplement, Enhance, or Leave Behind. |
| `/vsdd-factory:semport-analyze` | `[source-path] [target-language] [--incremental module-name]` | Semantic code porting -- scan a reference codebase, extract behavioral intent, and design a translation strategy to the target language. |

### Usage Notes

**`/vsdd-factory:brownfield-ingest`** -- Pass the path to the codebase you want to analyze. Use `--resume` to continue an interrupted ingestion. Runs 6 broad passes (inventory, architecture, domain model, behavioral contracts, NFRs, conventions) then iterative deepening until novelty decays. Produces `.factory/semport/<project>/` artifacts.

**`/vsdd-factory:disposition-pass`** -- Run after all reference repos are ingested. Pass `--all` to process every repo, or name a specific repo. `--rollup` generates a master disposition summary across all repos. `--update-vision` propagates decisions back into the vision document. Each repo gets one of four dispositions: Model (extract the pattern), Reimplement (same behavior, new code), Enhance (improve on the original), or Leave Behind (do not carry forward).

**`/vsdd-factory:semport-analyze`** -- Supports full ingestion and incremental change-level analysis. For porting existing implementations, pass the source path and target language (e.g., `rust`). Use `--incremental module-name` for module-level re-analysis after upstream changes. Produces `.factory/semport/<module>-semantic-analysis.md` and `.factory/semport/<module>-target-design.md`. Stories that use these artifacts set `implementation_strategy: gene-transfusion`.

---

## Phase 1: Spec Crystallization

| Command | Arguments | Description |
|---------|-----------|-------------|
| `/vsdd-factory:research` | `[domain\|general] <topic>` | Conduct external research via Perplexity, Context7, and Tavily. |
| `/vsdd-factory:create-brief` | | Create a product brief through guided discovery. |
| `/vsdd-factory:guided-brief-creation` | | Interactive facilitated workflow from raw ideas to structured product brief. |
| `/vsdd-factory:validate-brief` | | Validate a product brief against required structure and quality criteria. |
| `/vsdd-factory:create-domain-spec` | | Create a sharded L2 domain specification from the product brief. |
| `/vsdd-factory:create-prd` | | Create a PRD with behavioral contracts from brief and domain spec. |
| `/vsdd-factory:create-architecture` | | Create sharded architecture documents with ADR-style decisions. |
| `/vsdd-factory:adversarial-review` | `[specs\|implementation]` | Launch a fresh-context adversarial review. Minimum 2 passes. |
| `/vsdd-factory:design-system-bootstrap` | | Bootstrap a design system for UI products -- tokens, components, constraints. |

### Usage Notes

**`/vsdd-factory:research`** -- Run as many research sessions as needed before creating specs. Use `domain` for problem-space analysis and `general` for technology evaluation. Each session produces a dated report in `.factory/specs/vsdd-factory:research/`.

**`/vsdd-factory:create-brief`** -- Guided Q&A session that produces `.factory/specs/product-brief.md`. Cover vision, target users, scope boundaries, and success criteria.

**`/vsdd-factory:guided-brief-creation`** -- Extended version of `/vsdd-factory:create-brief` with staged elicitation. Use when starting from raw, unstructured ideas.

**`/vsdd-factory:validate-brief`** -- Run after creating or updating a brief to check structure, quality, and context-engineering criteria. Reports gaps and bloat with remediation guidance.

**`/vsdd-factory:create-domain-spec`** -- Reads the product brief and produces sharded L2 domain spec sections (entities, relationships, processes, invariants, ubiquitous language) in `.factory/specs/domain-spec/`.

**`/vsdd-factory:create-prd`** -- Reads brief and domain spec, produces `.factory/specs/prd.md`, behavioral contracts in `.factory/specs/behavioral-contracts/`, and supplements (error taxonomy, interface definitions, NFR catalog, test vectors) in `.factory/specs/prd-supplements/`.

**`/vsdd-factory:create-architecture`** -- Reads PRD and BCs, produces sharded architecture sections (ARCH-00 through ARCH-NN) in `.factory/specs/architecture/` with verification properties in `.factory/specs/verification-properties/`.

**`/vsdd-factory:adversarial-review specs`** -- Phase 1 spec review. Spawns a fresh-context adversary that cannot see prior passes. Minimum 2 passes until novelty decays. Findings go to `.factory/cycles/<current>/vsdd-factory:adversarial-reviews/`.

**`/vsdd-factory:design-system-bootstrap`** -- For UI products only. Creates design tokens (colors, typography, spacing, elevation, motion), component registry, and pattern definitions in the design system template format.

---

## Phase 2: Story Decomposition

| Command | Arguments | Description |
|---------|-----------|-------------|
| `/vsdd-factory:decompose-stories` | | Decompose PRD and architecture into epics, stories, dependency graph, and wave schedule. |
| `/vsdd-factory:create-story` | `[STORY-NNN]` | Create or refine a single story spec with full acceptance criteria and tasks. |
| `/vsdd-factory:wave-scheduling` | | Compute wave-based implementation order from story dependencies. |

### Usage Notes

**`/vsdd-factory:decompose-stories`** -- Requires completed Phase 1 specs. Produces `.factory/stories/STORY-*.md`, `STORY-INDEX.md`, `epics.md`, `dependency-graph.md`, `sprint-state.yaml`, and holdout scenarios in `.factory/holdout-scenarios/`.

**`/vsdd-factory:create-story`** -- Fleshes out a single story with detailed acceptance criteria, tasks, dev notes, and file lists. Runs the story completeness checklist before marking as ready.

**`/vsdd-factory:wave-scheduling`** -- Groups stories into waves for parallel execution based on dependency analysis.

---

## Phase 3: Test-First Implementation

| Command | Arguments | Description |
|---------|-----------|-------------|
| `/vsdd-factory:deliver-story` | `[STORY-NNN]` | Deliver a story through the full TDD pipeline: test-writer, implementer, demo-recorder, pr-manager, devops-engineer. |
| `/vsdd-factory:pr-create` | `[STORY-NNN]` | Create a pull request with story context, mermaid diagrams, and BC traceability. Targets develop. |
| `/vsdd-factory:record-demo` | `[STORY-NNN]` | Record visual demo evidence for story acceptance criteria using Playwright. |
| `/vsdd-factory:wave-gate` | `[wave-N]` | Run the post-wave integration gate -- test suite, adversarial review, holdout evaluation, demo evidence, DTU validation. |
| `/vsdd-factory:wave-status` | | Report current wave readiness from sprint-state.yaml. |
| `/vsdd-factory:holdout-eval` | `[wave-N]` | Run holdout evaluation with strict information asymmetry. |
| `/fix-pr-delivery` | | Streamlined delivery for fix PRs from adversarial refinement, hardening, and convergence. Skips stubs and Red Gate. |

### Usage Notes

**`/vsdd-factory:deliver-story`** -- The full TDD delivery cycle. Creates a worktree via `/vsdd-factory:worktree-manage create`, generates compilable stubs, spawns the test-writer to write failing tests (Red), verifies the Red Gate (tests fail for the right reasons, not build errors), spawns the implementer for minimum code (Green), refactors, records demos, creates a PR, and dispatches reviewers. Each test pass produces a micro-commit for full TDD history.

**`/vsdd-factory:pr-create`** -- Creates a GitHub PR targeting develop with mermaid architecture diagrams, BC traceability (which behavioral contracts this story implements), test evidence, and a structured description following the PR template.

**`/vsdd-factory:record-demo`** -- Uses Playwright for browser-based products or VHS for CLI products to capture acceptance criterion evidence. Each AC gets a visual proof. Writes `.factory/demo-evidence/STORY-NNN-demo-report.md`.

**`/vsdd-factory:wave-gate`** -- Run after all stories in a wave are merged to develop. Executes 6 gates in order: (1) full test suite on develop, (2) DTU validation if critical modules were touched, (3) adversarial review of the wave diff, (4) demo evidence completeness check, (5) holdout evaluation via `/vsdd-factory:holdout-eval`, and (6) state update. All 6 must pass.

**`/vsdd-factory:wave-status`** -- Quick read-only check of sprint-state.yaml showing which stories in the current wave are ready, in-progress, blocked, or merged.

**`/vsdd-factory:holdout-eval`** -- Spawns the holdout-evaluator agent with strict information asymmetry. The evaluator sees only the product brief, public API, and hidden scenarios from `.factory/holdout-scenarios/wave-scenarios/<wave>/`. It cannot see specs, source code, or prior reviews. Gate: mean satisfaction at least 0.85, every critical scenario at least 0.60.

**`/fix-pr-delivery`** -- Streamlined delivery for fix PRs generated by adversarial refinement, formal hardening, or convergence checks. Skips stubs, Red Gate enforcement, and wave gates since those apply to new feature work. Preserves testing and review.

---

## Phase 4: Adversarial Refinement

| Command | Arguments | Description |
|---------|-----------|-------------|
| `/vsdd-factory:adversarial-review` | `[specs\|implementation]` | Launch a fresh-context adversarial review. Also used in Phase 1 for specs. |

### Usage Notes

**`/vsdd-factory:adversarial-review implementation`** -- Reviews the full codebase against specs. The adversary reads specs first, then source code. Findings are classified by severity and routed to fix PRs or tech debt.

---

## Phase 5: Formal Hardening

| Command | Arguments | Description |
|---------|-----------|-------------|
| `/vsdd-factory:formal-verify` | | Run Kani proofs, fuzzing, mutation testing, and security scanning. |
| `/vsdd-factory:perf-check` | | Run performance validation -- benchmarks, binary size, startup time, memory profiling. |
| `/vsdd-factory:dtu-validate` | | DTU validation -- run behavioral clones against implementation for regression detection. |
| `/vsdd-factory:dtu-creation` | | Create behavioral clones of third-party services as Docker containers. |

### Usage Notes

**`/vsdd-factory:formal-verify`** -- Runs four verification tracks: Kani proofs (pure core functions), cargo-fuzz (parsers, deserializers, state machines), cargo-mutants (mutation kill rate target 90%), and semgrep security scanning. Reports missing tools and skips those tracks rather than failing silently. Produces `.factory/cycles/<current>/formal-verification-report.md` with a per-track verdict and overall gate.

**`/vsdd-factory:perf-check`** -- Validates 6 metrics: benchmark suite (criterion), binary size, startup time (hyperfine), memory profiling, compile time, and test suite time. Compares against budgets in `.factory/specs/prd-supplements/performance-budgets.md`. If no benchmarks exist, recommends creating them. Produces `.factory/cycles/<current>/performance-report.md`.

**`/vsdd-factory:dtu-validate`** -- Runs DTU comparison harnesses that exercise both the real implementation and the DTU clone with identical inputs. Uses proptest for property-based input generation. A divergence in a CRITICAL module is a blocking finding.

**`/vsdd-factory:dtu-creation`** -- Creates behavioral clones of third-party services at four fidelity levels: L1 (API shape -- read-only integrations), L2 (stateful -- CRUD operations), L3 (behavioral -- complex workflows like OAuth), L4 (adversarial -- reliability-critical like payment processing). Packages each clone as a Docker container with a docker-compose.yml and environment variable overrides for pointing the SUT to the clones.

---

## Phase 6: Convergence and Release

| Command | Arguments | Description |
|---------|-----------|-------------|
| `/vsdd-factory:convergence-check` | | Run 7-dimension convergence validation. |
| `/vsdd-factory:release` | | Release pipeline -- semver, CHANGELOG, tagging, GitHub Release, registry publishing. |

### Usage Notes

**`/vsdd-factory:convergence-check`** -- Checks all 7 dimensions: spec (adversary novelty LOW), tests (mutation kill rate 90%+, coverage 85%+), implementation (no drift, no TODOs), verification (all proofs pass, no fuzz crashes), visual (demo evidence for all stories), performance (budgets met), and documentation (CLAUDE.md, README, API docs current). Produces `.factory/cycles/<current>/convergence-report.md` with a per-dimension status table. Reports CONVERGED when all 7 pass, or lists remaining items with severity and estimated effort.

**`/vsdd-factory:release`** -- Requires human approval after convergence. The full sequence: (1) determine semver from story types (feat = MINOR, fix = PATCH, breaking = MAJOR), (2) generate CHANGELOG.md grouped by commit type with a quality evidence section, (3) create annotated git tag, (4) push tag to trigger release CI, (5) wait for release CI to pass, (6) create GitHub Release with binaries and demo evidence, (7) update README version badge and install instructions, (8) handle registry publishing (cargo publish, npm publish, etc. via CI). If release CI fails, the skill diagnoses from logs and guides recovery.

---

## Cross-Cutting

| Command | Arguments | Description |
|---------|-----------|-------------|
| `/vsdd-factory:factory-health` | | Validate and auto-repair the .factory/ worktree. |
| `/vsdd-factory:factory-worktree-health` | | Extended worktree validation with remote sync and multi-repo support. |
| `/vsdd-factory:setup-env` | | Validate development environment -- tools, versions, MCP servers. |
| `/vsdd-factory:next-step` | | Propose the next pipeline step from STATE.md and workflow data. |
| `/vsdd-factory:state-update` | | Update STATE.md with phase transitions (internal, not user-invoked). |
| `/vsdd-factory:validate-consistency` | | Cross-file consistency validation for all planning artifacts. |
| `/vsdd-factory:spec-drift` | | Compare implementation against specs to detect drift. |
| `/vsdd-factory:track-debt` | `[add\|list\|resolve] [description]` | Manage the technical debt register. |
| `/vsdd-factory:worktree-manage` | `[create\|list\|cleanup] [STORY-NNN]` | Manage per-story git worktrees. |
| `/vsdd-factory:session-review` | | Post-pipeline analysis with improvement proposals. |
| `/vsdd-factory:maintenance-sweep` | | Periodic quality sweep -- 9 parallel audit tracks. |
| `/vsdd-factory:multi-repo-health` | | Scan for multi-repo layout and report detected repos. |
| `/vsdd-factory:mode-decision-guide` | | Choose between Greenfield, Brownfield, and Feature modes. |
| `/vsdd-factory:quick-dev-routing` | | Route trivially-scoped changes through compressed pipeline. |

### Usage Notes

**`/vsdd-factory:factory-health`** -- Run at the start of every session. Checks that the `factory-artifacts` orphan branch exists, worktree is mounted correctly, STATE.md is present, and directory structure is intact. Auto-repairs common issues like missing directories or unmounted worktrees.

**`/vsdd-factory:factory-worktree-health`** -- More thorough than `/vsdd-factory:factory-health`. Includes a workspace isolation guard that prevents accidental operation in the engine repository. For multi-repo projects with `project.yaml`, also checks `.factory-project/` on `factory-project-artifacts`. Runs as a blocking precondition on every pipeline start.

**`/vsdd-factory:setup-env`** -- Checks 8 required tools and 10 optional tools with version requirements. Reports MCP server health for Perplexity, Tavily, Playwright, and Tally. Also runs `/vsdd-factory:factory-health` internally.

**`/vsdd-factory:next-step`** -- Read-only navigation aid. Reads STATE.md and the active Lobster workflow to find the first uncompleted step whose dependencies are satisfied. Does not execute anything -- only proposes.

**`/vsdd-factory:state-update`** -- Internal skill, not user-invoked. Other skills call this at phase boundaries to update the YAML frontmatter in STATE.md and append to the phase history table.

**`/vsdd-factory:validate-consistency`** -- Runs 7 check categories: BC ID integrity, VP ID integrity, story traceability, architecture cross-references, count consistency, status consistency, and naming consistency. Run after creating or modifying any spec artifacts.

**`/vsdd-factory:spec-drift`** -- Runs in a forked context for objectivity. Compares every BC against its implementing code. Checks architecture decision compliance. Identifies orphaned code and unimplemented specs. Produces a prioritized drift report.

**`/vsdd-factory:track-debt add "description"`** -- Assigns a TD-NNN ID and records severity, category, source, impact, and effort. Common use: logging deferred adversarial findings or known shortcuts.

**`/vsdd-factory:worktree-manage create STORY-NNN`** -- Creates `.worktrees/STORY-NNN/` branching from develop. Branch name follows `feature/STORY-NNN-<description>` pattern.

**`/vsdd-factory:worktree-manage cleanup STORY-NNN`** -- Refuses to remove worktrees with uncommitted changes. Warns if the branch is not merged to develop.

**`/vsdd-factory:session-review`** -- Analyzes 8 dimensions (cost, timing, convergence, agent behavior, gate outcomes, wall integrity, quality signals, patterns). Produces improvement proposals routed to the human for approval. Auto-defers after 72h if no response.

**`/vsdd-factory:maintenance-sweep`** -- Runs 9 parallel sweep tracks: dependency audit, doc drift, pattern consistency, holdout freshness, performance baselines, DTU fidelity, spec coherence, tech debt review, and accessibility regression. Can be scheduled via GitHub Actions cron.

**`/vsdd-factory:multi-repo-health`** -- Reports each detected repo's manifest type, current branch, and git status. Cross-checks against stories. Read-only.

**`/vsdd-factory:mode-decision-guide`** -- Presents criteria for choosing Greenfield (new project), Brownfield (existing codebase), or Feature (post-v1 incremental) mode.

**`/vsdd-factory:quick-dev-routing`** -- Skips stubs, Red Gate, and wave gates. Preserves regression testing and adversarial review. Use for bug fixes with verified zero blast radius.

---

## UI and Design

| Command | Arguments | Description |
|---------|-----------|-------------|
| `/vsdd-factory:design-drift-detection` | | Detect design system drift -- token overrides, component misuse, pattern violations. |
| `/vsdd-factory:ui-completeness-check` | | Validate UI completeness via traceability matrix from UX spec through implementation. |
| `/vsdd-factory:ui-quality-gate` | | Comprehensive UI quality gate -- design system, completeness, heuristics, accessibility, responsive, performance, visual regression. |
| `/vsdd-factory:ux-heuristic-evaluation` | | Automated usability evaluation against Nielsen's 10 heuristics. |
| `/vsdd-factory:responsive-validation` | | Automated responsive testing at 4+ breakpoints with screenshot evidence. |

### Usage Notes

**`/vsdd-factory:design-drift-detection`** -- Detects token overrides (hardcoded colors instead of tokens), component misuse (wrong component for the pattern), pattern violations (non-standard layouts), and emergent patterns (repeated custom solutions that should become components). Use during maintenance to keep UI consistent.

**`/vsdd-factory:ui-completeness-check`** -- Builds a traceability matrix from UX spec through story, component, state, test, and visual evidence. Flags missing links in the chain.

**`/vsdd-factory:ui-quality-gate`** -- Comprehensive gate that runs design system compliance, completeness, heuristics, accessibility, responsive, performance, and visual regression checks as a single pass.

**`/vsdd-factory:ux-heuristic-evaluation`** -- Evaluates against Nielsen's 10 usability heuristics: visibility of system status, match between system and real world, user control and freedom, consistency and standards, error prevention, recognition rather than recall, flexibility and efficiency, aesthetic and minimalist design, error recovery, and help and documentation.

**`/vsdd-factory:responsive-validation`** -- Tests at 4+ breakpoints (mobile, tablet, desktop, wide) and captures screenshots as evidence. Reports layout breaks and overflow issues per breakpoint.
