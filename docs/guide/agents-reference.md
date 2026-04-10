# Agents Reference

The vsdd-factory plugin ships 34 agent definitions: 24 specialist agents and 10 orchestrator workflow files. Agents are spawned by the orchestrator or by skills that need specialist behavior.

---

## Agent Summary

| Agent | Model | Color | Description |
|-------|-------|-------|-------------|
| `accessibility-auditor` | sonnet | red | WCAG AA/AAA accessibility compliance auditing |
| `adversary` | opus | red | Fresh-context adversarial reviewer for specs and implementation |
| `architect` | sonnet | green | System architecture design, ADRs, subsystem decomposition |
| `business-analyst` | sonnet | blue | L2 Domain Specification synthesis from product brief |
| `code-reviewer` | sonnet | red | Constructive code review with cognitive diversity |
| `codebase-analyzer` | opus | green | Deep codebase scanner for semantic analysis and pattern extraction |
| `consistency-validator` | sonnet | red | Cross-document consistency validation across all artifacts |
| `data-engineer` | sonnet | green | Schema design, migrations, data-layer integrity |
| `demo-recorder` | sonnet | blue | Visual evidence capture via VHS or Playwright |
| `devops-engineer` | sonnet | yellow | CI/CD pipelines, containers, deployment, monitoring |
| `dtu-validator` | sonnet | red | DTU clone fidelity validation against real services |
| `dx-engineer` | sonnet | blue | Development environment preparation and pre-flight checks |
| `e2e-tester` | sonnet | blue | End-to-end user journey tests and Playwright suites |
| `formal-verifier` | opus | red | Mathematical proofs, fuzzing, mutation testing, security scanning |
| `github-ops` | sonnet | yellow | Execute gh CLI commands for agents without shell access |
| `holdout-evaluator` | opus | red | Hidden acceptance scenario evaluation with information asymmetry |
| `implementer` | sonnet | green | Strict TDD implementation -- pick failing test, write minimum code |
| `performance-engineer` | sonnet | yellow | Benchmarks, regression detection, Core Web Vitals |
| `pr-manager` | sonnet | yellow | Full PR lifecycle -- creation, review dispatch, merge |
| `pr-reviewer` | opus | red | Fresh-eyes PR review seeing only diff and test evidence |
| `product-owner` | sonnet | blue | L2 Domain Spec to L3 PRD with BC-S.SS.NNN contracts |
| `research-agent` | opus | purple | External research via Perplexity, Context7, Tavily |
| `security-reviewer` | sonnet | red | Application security review with CWE/CVE classification |
| `session-review` | sonnet | red | Post-session lessons, decisions, and follow-ups |
| `spec-reviewer` | opus | red | Constructive second-opinion review of Phase 1/2 specs |
| `spec-steward` | sonnet | yellow | Spec versioning, traceability, and governance enforcement |
| `state-manager` | sonnet | yellow | STATE.md updates and .factory/ directory maintenance |
| `story-writer` | sonnet | green | Spec decomposition into per-story files with BC traceability |
| `technical-writer` | sonnet | blue | Documentation generation from code and specs |
| `test-writer` | sonnet | green | TDD test suites from behavioral contracts |
| `ux-designer` | sonnet | blue | UX specifications, wireframes, interaction design |
| `validate-extraction` | sonnet | red | Verify extracted artifacts against actual code (max 3 iterations) |
| `visual-reviewer` | sonnet | red | Visual verification of demos and UI fidelity |

---

## Agents by Function

### Builders (green)

**architect** -- Spawned during Phase 1b to design system architecture from domain specs and PRDs. Produces ADR-style decisions, sharded ARCH-NN sections, component diagrams, and verification-ready design artifacts.

**codebase-analyzer** -- Deep analysis agent used by `/vsdd-factory:brownfield-ingest` and `/vsdd-factory:semport-analyze`. Runs a structured 6-pass protocol with file prioritization, test-as-spec extraction, and state checkpointing. Has broad tool access including Bash for running analysis commands in `.reference/` directories.

**data-engineer** -- Designs schemas and writes reversible migrations while preserving the pure-core / effectful-I/O boundary. Spawned during implementation when data layer work is needed.

**implementer** -- The TDD workhorse. Picks the next failing test, writes the minimum code to make it pass, and micro-commits each step. Spawned by `/vsdd-factory:deliver-story` after the test-writer has written failing tests.

**story-writer** -- Decomposes validated specs into per-story files. Every acceptance criterion traces back to a BC-S.SS.NNN behavioral contract. Spawned by `/vsdd-factory:decompose-stories`.

**test-writer** -- Generates TDD test suites from behavioral contracts. Covers unit tests, edge case tests, integration tests, and property-based tests. For UI products, includes full state coverage for component contracts. Spawned by `/vsdd-factory:deliver-story` as the first implementation step.

### Reviewers (red)

**adversary** -- The cornerstone of VSDD quality. Fresh-context adversarial reviewer that cannot see prior review passes. Uses a different model family for genuine perspective diversity. Spawned by `/vsdd-factory:adversarial-review` with strict information asymmetry via `context: fork`.

**code-reviewer** -- Constructive code review with cognitive diversity from a different model family. Produces classified findings without modifying code. Spawned during the PR review cycle.

**consistency-validator** -- Validates cross-document consistency across specs, stories, design systems, and UI artifacts. Catches broken references and drift. Spawned by `/vsdd-factory:validate-consistency` and `/vsdd-factory:maintenance-sweep`.

**formal-verifier** -- Runs the Phase 5 hardening gauntlet: Kani proofs, cargo-fuzz, cargo-mutants, and semgrep. Spawned by `/vsdd-factory:formal-verify`.

**holdout-evaluator** -- Evaluates implementation against hidden acceptance scenarios. Operates under strict information asymmetry: cannot see source code internals, specs, implementation notes, or prior reviews. Only sees the public API surface and holdout scenarios. Spawned by `/vsdd-factory:holdout-eval`.

**pr-reviewer** -- Fresh-eyes PR review seeing only the diff, PR description, and test evidence -- not the full codebase. Spawned during the merge gate.

**security-reviewer** -- Manual application security review and automated scan triage. Cites CWE/CVE numbers and classifies severity for every finding.

**spec-reviewer** -- Constructive second-opinion review of Phase 1 specs or Phase 2 story decomposition with cognitive diversity.

**validate-extraction** -- AgenticAKM-style validator for codebase analysis output. Verifies extracted behavioral contracts, domain models, and architecture docs against actual code. Catches hallucinated dependencies, phantom modules, and inaccurate contracts. Maximum 3 refinement iterations.

**visual-reviewer** -- Visual verification of demos, screenshots, and UI fidelity with multimodal comparison for regression detection.

**accessibility-auditor** -- Audits specs, designs, or implementations for WCAG AA/AAA compliance at any pipeline checkpoint.

**dtu-validator** -- Validates DTU behavioral clones against real third-party services, producing fidelity scores.

**session-review** -- Post-pipeline analysis of the complete factory run across 8 dimensions. Produces improvement proposals.

### Ops (yellow)

**devops-engineer** -- CI/CD pipelines, container configurations, deployment scripts, and monitoring. Handles worktree health checks and release CI monitoring.

**github-ops** -- Executes `gh` CLI commands on behalf of agents that lack shell access. Returns raw results without making decisions.

**pr-manager** -- Coordinates the full PR lifecycle: creation, review dispatch, finding triage, fix delegation, convergence tracking, and merge.

**spec-steward** -- Enforces spec versioning, traceability, and governance. Ensures every spec change is versioned, traced, and auditable without modifying spec content.

**state-manager** -- Updates STATE.md with phase transitions and maintains the `.factory/` directory structure.

### Analysts (blue)

**business-analyst** -- Synthesizes the L2 Domain Specification from product brief and domain research. Produces sharded capability, entity, and invariant docs.

**demo-recorder** -- Captures visual evidence via VHS terminal recordings or Playwright browser sessions for PRs, READMEs, and convergence reports.

**dx-engineer** -- Prepares the development environment: installing tools, configuring direnv, validating API keys, and running pre-flight checks.

**e2e-tester** -- Executes end-to-end user journey tests, visual validation, and Playwright/Cypress browser suites against a running application.

**performance-engineer** -- Runs benchmarks, detects performance regressions, and enforces Core Web Vitals or other performance budgets.

**product-owner** -- Transforms an L2 Domain Specification into a structured L3 PRD with BC-S.SS.NNN behavioral contracts.

**technical-writer** -- Generates documentation from code and specs. Strictly describes current behavior, never aspirational plans.

**ux-designer** -- Creates UX specifications, wireframes, and interaction designs for UI products from a product brief.

### Research (purple)

**research-agent** -- Conducts external research using Perplexity, Context7, and Tavily MCP servers. Covers technology evaluations, library comparisons, security advisory lookups, architecture pattern research, and domain research. Always cites sources and verifies library versions against registries. Flags inconclusive findings.

---

## Orchestrator Workflows

The orchestrator agent reads workflow data from `.lobster` files and spawns sub-agents in dependency order. It has 10 workflow reference files that it loads based on the current pipeline mode and phase:

| Workflow File | Purpose |
|---------------|---------|
| `orchestrator.md` | Core orchestrator -- reads workflow data, spawns agents across phases and modes |
| `greenfield-sequence.md` | Full greenfield pipeline from brief through release |
| `brownfield-sequence.md` | Existing codebase ingestion plus greenfield overlay |
| `feature-sequence.md` | Post-v1 feature additions (F1-F7 delta phases) |
| `maintenance-sequence.md` | Dependency bumps, doc sweeps, reactive fixes |
| `discovery-sequence.md` | Autonomous discovery engine (Path 8) |
| `multi-repo.md` | Cross-repository pipeline coordination |
| `per-story-delivery.md` | Per-story TDD delivery cycle (red-green-refactor, PR, review, merge) |
| `steady-state.md` | Post-v1.0 ongoing maintenance and feature work |
| `HEARTBEAT.md` | Periodic health check between delegations -- detects stale pipelines |

These workflow files are internal orchestrator references. They are not invoked directly by users.

---

## Model Assignment

Agents use two model tiers:

- **opus** -- Used for high-stakes tasks requiring deep reasoning: adversary, codebase-analyzer, formal-verifier, holdout-evaluator, pr-reviewer, research-agent, spec-reviewer
- **sonnet** -- Used for execution tasks: implementation, testing, writing, ops, validation

The adversary intentionally uses a different model to provide genuine cognitive diversity and prevent blind spots from model-family homogeneity.

## Color Coding

Agent colors indicate their function in the pipeline:
- **green** -- builders (create artifacts)
- **red** -- reviewers (find problems)
- **yellow** -- ops (manage infrastructure)
- **blue** -- analysts (gather information)
- **purple** -- research (external knowledge)
