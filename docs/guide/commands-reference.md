# Commands Reference

The vsdd-factory plugin ships 103 slash commands. Each command dispatches to a corresponding skill. Run any command by typing its name in Claude Code.

All commands are prefixed with `/` when invoked. If you have multiple plugins installed, use the fully qualified form `/vsdd-factory:<command-name>` to avoid ambiguity.

Commands are organized by category below.

---

## Session Management

| Command | Description |
|---------|-------------|
| `/vsdd-factory:activate` | Opt in to the VSDD factory persona for this project. |
| `/vsdd-factory:deactivate` | Remove the orchestrator default agent from settings. |
| `/vsdd-factory:factory-health` | Validate and auto-repair the .factory/ worktree. |
| `/vsdd-factory:factory-worktree-health` | Extended worktree validation with remote sync and multi-repo support. |
| `/vsdd-factory:setup-env` | Validate development environment -- tools, versions, MCP servers. |
| `/vsdd-factory:scaffold-claude-md` | Auto-detect project context and generate a CLAUDE.md. |
| `/vsdd-factory:session-review` | Post-pipeline analysis with improvement proposals. |
| `/vsdd-factory:repo-initialization` | Initialize repository structure for VSDD workflow. (orchestrator-managed) |
| `/vsdd-factory:toolchain-provisioning` | Install verification toolchain based on project language. |

---

## Navigation

| Command | Description |
|---------|-------------|
| `/vsdd-factory:next-step` | Propose the next pipeline step from STATE.md and workflow data. |
| `/vsdd-factory:state-update` | Update STATE.md with phase transitions (internal, not user-invoked). |
| `/vsdd-factory:mode-decision-guide` | Choose between Greenfield, Brownfield, and Feature modes. |
| `/vsdd-factory:run-phase` | Execute a VSDD phase from its Lobster workflow file. |
| `/vsdd-factory:validate-workflow` | Schema-check a Lobster workflow file. |
| `/vsdd-factory:artifact-detection` | Scan project for existing planning artifacts and route to correct pipeline entry. |
| `/vsdd-factory:quick-dev-routing` | Route trivially-scoped changes through compressed pipeline. |

---

## Quality

| Command | Description |
|---------|-------------|
| `/vsdd-factory:validate-consistency` | Cross-file consistency validation for all planning artifacts. |
| `/vsdd-factory:consistency-validation` | Cross-document validation between PRD, architecture, stories, and implementation. |
| `/vsdd-factory:spec-drift` | Compare implementation against specs to detect drift. |
| `/vsdd-factory:spec-versioning` | Manage spec evolution using semantic versioning. |
| `/vsdd-factory:maintenance-sweep` | Periodic quality sweep -- 9 parallel audit tracks. |
| `/vsdd-factory:implementation-readiness` | Validate spec package is ready for implementation. |
| `/vsdd-factory:traceability-extension` | Rules for extending the VSDD traceability chain. |
| `/vsdd-factory:convergence-tracking` | Compute quantitative convergence metrics across adversarial and verification passes. |

---

## Visual Tooling

| Command | Description |
|---------|-------------|
| `/vsdd-factory:visual-companion` | Browser-based visual companion for mockups and diagrams. Requires Node.js. |
| `/vsdd-factory:create-excalidraw` | Generate .excalidraw JSON files for architecture diagrams and flow charts. |
| `/vsdd-factory:excalidraw-export` | Batch-render .excalidraw diagrams to PNG. Reference-only. |
| `/vsdd-factory:generate-pdf` | Generate a branded PDF from a markdown research document. |
| `/vsdd-factory:demo-recording` | Record visual demonstrations for review evidence and PR documentation. |
| `/vsdd-factory:record-demo` | Record visual demo evidence for story acceptance criteria using Playwright. |

---

## Debugging

| Command | Description |
|---------|-------------|
| `/vsdd-factory:systematic-debugging` | 4-phase root cause investigation for bugs and test failures. |

---

## Ideation & Planning

| Command | Description |
|---------|-------------|
| `/vsdd-factory:brainstorming` | Guided brainstorming session using structured ideation techniques. |
| `/vsdd-factory:planning-research` | Conduct market, domain, and technical research before brief/PRD creation. |

---

## Spec Creation

| Command | Description |
|---------|-------------|
| `/vsdd-factory:research` | Conduct external research via Perplexity, Context7, and Tavily. |
| `/vsdd-factory:create-brief` | Create a product brief through guided discovery. |
| `/vsdd-factory:guided-brief-creation` | Interactive facilitated workflow from raw ideas to structured product brief. |
| `/vsdd-factory:validate-brief` | Validate a product brief against required structure and quality criteria. |
| `/vsdd-factory:create-domain-spec` | Create a sharded L2 domain specification from the product brief. |
| `/vsdd-factory:create-prd` | Create a PRD with behavioral contracts from brief and domain spec. |
| `/vsdd-factory:phase-1-prd-revision` | Revise PRD based on architect feasibility review. |
| `/vsdd-factory:create-architecture` | Create sharded architecture documents with ADR-style decisions. |
| `/vsdd-factory:adversarial-review` | Launch a fresh-context adversarial review. Minimum 2 passes. |
| `/vsdd-factory:phase-1d-adversarial-spec-review` | Adversarial review of the full spec package. (orchestrator-managed) |
| `/vsdd-factory:design-system-bootstrap` | Bootstrap a design system for UI products -- tokens, components, constraints. |

---

## Story Management

| Command | Description |
|---------|-------------|
| `/vsdd-factory:decompose-stories` | Decompose PRD and architecture into epics, stories, dependency graph, and wave schedule. |
| `/vsdd-factory:create-story` | Create or refine a single story spec with full acceptance criteria and tasks. |
| `/vsdd-factory:wave-scheduling` | Compute wave-based implementation order from story dependencies. |
| `/vsdd-factory:wave-status` | Report current wave readiness from sprint-state.yaml. |
| `/vsdd-factory:wave-gate` | Run the post-wave integration gate -- test suite, adversarial review, holdout evaluation, demo evidence, DTU validation. |

---

## Implementation

| Command | Description |
|---------|-------------|
| `/vsdd-factory:deliver-story` | Deliver a story through the full TDD pipeline: test-writer, implementer, demo-recorder, pr-manager, devops-engineer. |
| `/vsdd-factory:code-delivery` | Post-convergence code delivery -- push, PR, review, merge. |
| `/vsdd-factory:worktree-manage` | Manage per-story git worktrees (create, list, cleanup). |
| `/vsdd-factory:sdk-generation` | Generate production-ready SDKs from API contracts. |
| `/vsdd-factory:model-routing` | Reference for LiteLLM model routing strategy. |
| `/vsdd-factory:writing-skills` | TDD methodology for creating and maintaining plugin skills. |

---

## Review & Convergence

| Command | Description |
|---------|-------------|
| `/vsdd-factory:convergence-check` | Run 7-dimension convergence validation. |
| `/vsdd-factory:holdout-eval` | Run holdout evaluation with strict information asymmetry. |
| `/vsdd-factory:agent-file-review` | Review agent files for compliance with Dark Factory design principles. |

---

## DTU

| Command | Description |
|---------|-------------|
| `/vsdd-factory:dtu-creation` | Create behavioral clones of third-party services as Docker containers. |
| `/vsdd-factory:dtu-validate` | DTU validation -- run behavioral clones against implementation for regression detection. |
| `/vsdd-factory:storybook-mcp-integration` | Integrate Storybook MCP for UI validation. |

---

## Formal Verification & Performance

| Command | Description |
|---------|-------------|
| `/vsdd-factory:formal-verify` | Run Kani proofs, fuzzing, mutation testing, and security scanning. |
| `/vsdd-factory:perf-check` | Run performance validation -- benchmarks, binary size, startup time, memory profiling. |

---

## Release

| Command | Description |
|---------|-------------|
| `/vsdd-factory:release` | Release pipeline -- semver, CHANGELOG, tagging, GitHub Release, registry publishing. |

---

## Debt & Tracking

| Command | Description |
|---------|-------------|
| `/vsdd-factory:track-debt` | Manage the technical debt register (add, list, resolve). |
| `/vsdd-factory:analytics-integration` | Ingest product analytics data to identify adoption and error patterns. |

---

## Governance & Policy

| Command | Description |
|---------|-------------|
| `/vsdd-factory:policy-registry` | View, validate, and manage the project's governance policy registry. |
| `/vsdd-factory:policy-add` | Register a new governance policy in the project's policy registry. |
| `/vsdd-factory:validate-template-compliance` | Audit whether artifact files conform to their corresponding templates. |
| `/vsdd-factory:conform-to-template` | Fix structural gaps in an artifact file by adding missing template structure. |
| `/vsdd-factory:register-artifact` | Register a newly created artifact in its INDEX file. |
| `/vsdd-factory:recover-state` | Reconstruct STATE.md from artifacts on disk when corrupted or missing. |
| `/vsdd-factory:factory-cycles-bootstrap` | Migrate from flat adversarial-review layout to cycle-keyed directory structure. |

---

## Research

| Command | Description |
|---------|-------------|
| `/vsdd-factory:research-cache-ops` | Operate the research cache for Perplexity/Context7 query results. |

---

## Multi-Repo

| Command | Description |
|---------|-------------|
| `/vsdd-factory:multi-repo-health` | Scan for multi-repo layout and report detected repos. |
| `/vsdd-factory:multi-repo-phase-0-synthesis` | Synthesize per-repo ingestion outputs into unified project context. |

---

## Brownfield

| Command | Description |
|---------|-------------|
| `/vsdd-factory:brownfield-ingest` | Analyze an existing codebase using the broad-then-converge protocol. |
| `/vsdd-factory:disposition-pass` | Re-examine ingested reference repos through the vision lens to decide what to Model, Reimplement, Enhance, or Leave Behind. |
| `/vsdd-factory:semport-analyze` | Semantic code porting -- scan a reference codebase, extract behavioral intent, and design a translation strategy. |

---

## Feature Mode

| Command | Description |
|---------|-------------|
| `/vsdd-factory:feature-mode-scoping-rules` | Reference for scope determination in Feature Mode. |
| `/vsdd-factory:phase-f1-delta-analysis` | Delta analysis for feature-mode iteration. (orchestrator-managed) |
| `/vsdd-factory:phase-f2-spec-evolution` | Evolve specs based on delta analysis findings. (orchestrator-managed) |
| `/vsdd-factory:phase-f3-incremental-stories` | Generate incremental stories from evolved specs. (orchestrator-managed) |
| `/vsdd-factory:phase-f4-delta-implementation` | Implement delta changes from incremental stories. (orchestrator-managed) |
| `/vsdd-factory:phase-f5-scoped-adversarial` | Scoped adversarial review of delta changes. (orchestrator-managed) |
| `/vsdd-factory:phase-f6-targeted-hardening` | Targeted hardening based on scoped adversarial findings. (orchestrator-managed) |
| `/vsdd-factory:phase-f7-delta-convergence` | Convergence verification for feature-mode delta cycle. (orchestrator-managed) |
| `/vsdd-factory:post-feature-validation` | Monitor feedback and analytics after feature ships. |

---

## Design & UX

| Command | Description |
|---------|-------------|
| `/vsdd-factory:design-drift-detection` | Detect design system drift -- token overrides, component misuse, pattern violations. |
| `/vsdd-factory:ui-completeness-check` | Validate UI completeness via traceability matrix from UX spec through implementation. |
| `/vsdd-factory:ui-quality-gate` | Comprehensive UI quality gate -- design system, completeness, heuristics, accessibility, responsive, performance, visual regression. |
| `/vsdd-factory:ux-heuristic-evaluation` | Automated usability evaluation against Nielsen's 10 heuristics. |
| `/vsdd-factory:responsive-validation` | Automated responsive testing at 4+ breakpoints with screenshot evidence. |
| `/vsdd-factory:multi-variant-design` | Generate 2-3 design variants for complex screens with scoring. |

---

## Discovery & Intelligence

| Command | Description |
|---------|-------------|
| `/vsdd-factory:discovery-engine` | Automated codebase discovery and context extraction. (orchestrator-managed) |
| `/vsdd-factory:market-intelligence-assessment` | Mandatory market intelligence assessment before spec work. |
| `/vsdd-factory:competitive-monitoring` | Monitor competitor activity and produce competitive update report. |
| `/vsdd-factory:customer-feedback-ingestion` | Ingest customer feedback from configured channels. |
| `/vsdd-factory:intelligence-synthesis` | Correlate signals across market research, feedback, and analytics. |

---

## Infrastructure

| Command | Description |
|---------|-------------|
| `/vsdd-factory:jira` | Reference documentation for jira-cli tool. Reference-only. |

---

## PR Management

| Command | Description |
|---------|-------------|
| `/vsdd-factory:pr-create` | Create a pull request with story context, mermaid diagrams, and BC traceability. |
| `/vsdd-factory:pr-review-triage` | Classify and dispatch PR review findings. |
| `/vsdd-factory:fix-pr-delivery` | Streamlined delivery for fix PRs from adversarial refinement, hardening, and convergence. Skips stubs and Red Gate. |

