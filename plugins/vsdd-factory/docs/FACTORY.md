# FACTORY.md — Dark Factory Project Constitution

This file is loaded into every agent's context via OpenClaw's global_instructions.
It defines the factory's operating rules.

## What This Is

The Dark Factory is an autonomous multi-agent software development system implementing
Verified Spec-Driven Development (VSDD). It transforms product briefs into production-ready
software through six phases: Spec Crystallization → Story Decomposition → Test-First
Implementation → Adversarial Refinement → Formal Hardening → Convergence.

Orchestration is handled by **NemoClaw/OpenClaw** (not Claude Code). Agents are defined
in `openclaw.json` with personality prompts loaded from workspace directories. Each agent's
workspace contains `AGENTS.md` (operating instructions), `SOUL.md` (persona), and
`IDENTITY.md` (name/emoji). Workflows are defined as Lobster workflows (`.lobster`) for
orchestration and skills (`SKILL.md`) for phase instructions in `workflows/`. Plugins in
`plugins/src/` enforce TDD discipline, quality gates, and purity boundaries.

See `docs/OPERATIONS.md` for real-world deployment notes, troubleshooting, and validated
architecture assumptions.

## Engine vs. Target Project

The Dark Factory is a **tool** (the engine), not a project template. It is installed
once and pointed at one or more **target projects** — the codebases it builds or extends.

```
$DARK_FACTORY_HOME/              ← Engine (installed once, never forked)
├── agents/                      ← Agent personality prompts (static)
├── workflows/                   ← Pipeline orchestration (static)
├── plugins/                     ← Enforcement hooks (static)
├── templates/                   ← Output templates (static)
├── config/                      ← Model routing, sandbox policies (static)
└── openclaw.json                ← Agent fleet configuration (static)

~/projects/my-product/           ← Target project (where agents work)
├── src/                         ← Source code (written by agents)
├── tests/                       ← Test suite (written by agents)
├── .factory/                    ← Pipeline runtime state (gitignored)
│   ├── specs/                   ← LIVING — always current truth (DF-030)
│   │   ├── product-brief.md     ← L1 product brief
│   │   ├── domain-spec-L2.md    ← L2 domain specification
│   │   ├── prd.md               ← L3 core PRD (index document)
│   │   ├── prd-supplements/     ← L3 supplements (interface defs, error taxonomy, etc.)
│   │   ├── behavioral-contracts/← BC-INDEX.md + per-BC files (accumulate)
│   │   ├── verification-properties/ ← VP-INDEX.md + per-VP files (accumulate)
│   │   ├── architecture/        ← ARCH-INDEX.md + 7 section files (DF-021)
│   │   ├── ux-spec.md           ← UX spec (if UI product)
│   │   ├── module-criticality.md ← Module criticality classification
│   │   ├── dtu-assessment.md    ← DTU clone assessment
│   │   └── gene-transfusion-assessment.md
│   │
│   ├── stories/                 ← LIVING — accumulate across cycles (DF-030)
│   │   ├── STORY-INDEX.md       ← Complete story registry (all cycles)
│   │   ├── STORY-NNN.md         ← Individual stories
│   │   ├── epics.md             ← Accumulated epics
│   │   ├── dependency-graph.md  ← Current full graph
│   │   └── sprint-state.yaml    ← Current story states
│   │
│   ├── cycles/                  ← CYCLE-SCOPED — per pipeline run (DF-030)
│   │   ├── v1.0.0-greenfield/   ← First release cycle
│   │   │   ├── cycle-manifest.md
│   │   │   ├── adversarial-reviews/
│   │   │   ├── convergence-report.md
│   │   │   ├── traceability-matrix.md
│   │   │   ├── wave-schedule.md
│   │   │   ├── cost-summary.md
│   │   │   └── release-notes.md
│   │   └── v1.1.0-feature-auth/ ← Feature cycle
│   │       ├── cycle-manifest.md
│   │       ├── delta-analysis.md
│   │       └── spec-evolution.md
│   │
│   ├── holdout-scenarios/       ← LIVING — accumulate, some retired (DF-030)
│   │   ├── HS-INDEX.md
│   │   ├── wave-scenarios/
│   │   └── evaluations/         ← EVAL-INDEX.md + pass-N/ per-scenario files
│   │
│   ├── dtu-clones/              ← LIVING — clones evolve
│   ├── semport/                 ← LIVING — translation artifacts
│   ├── code-delivery/           ← Per-story delivery (accumulates)
│   ├── demo-evidence/           ← Visual review tracking (per-story reports)
│   │
│   ├── STATE.md                 ← Pipeline progress tracker + product backlog
│   ├── cost-summary.md          ← Cumulative cost across ALL cycles
│   ├── tech-debt-register.md    ← Technical debt tracking (DF-030)
│   ├── merge-config.yaml        ← Autonomy, notifications, budget
│   └── autonomy-config.yaml     ← Budget thresholds, protected agents
└── product-brief.md             ← Human's input (or via DF-016 guided creation)
```

> **Migration note (DF-030):** The phase-based directories (`phase-1-spec/`,
> `phase-2-stories/`, `phase-3-implementation/`, `phase-5-adversarial/`,
> `phase-5-hardening/`, `phase-7-convergence/`) are replaced by the lifecycle-aware
> structure above. Living specs move to `specs/`, stories to `stories/`, and
> operational artifacts to `cycles/vX.Y.Z-name/`. See the Artifact Path Migration
> table in DF-030 for the complete mapping.

### Key Separation Rules

- **Engine files are read-only** during pipeline execution. Agents read agent prompts,
  skill instructions, and templates from the engine but never modify them.
- **`.factory/` lives in the TARGET project**, not in the engine. Each target project
  has its own independent pipeline state.
- **The engine is reusable.** Run it on Project A, then Project B, then Project C.
  Each gets its own `.factory/` directory. No forking required.
- **Multi-repo projects** use a `project.yaml` manifest that sits above the individual
  repos and coordinates them (see DF-012). Each repo gets its own `.factory/`.

### Pointing the Factory at a Target

```bash
# Single-repo project
nemoclaw dark-factory connect --workspace ~/projects/my-product

# Or via OpenClaw (no sandbox)
openclaw start --workspace ~/projects/my-product

# Multi-repo project (project.yaml sits above repos)
nemoclaw dark-factory connect --workspace ~/projects/acme-platform
# project.yaml defines: ./api-server, ./frontend, ./sdk-typescript, ./sdk-python
```

The `--workspace` flag sets the target project path. All agents operate on this
path. The `.factory/` directory is created here automatically on first pipeline run.

### Environment Variable

Set `DARK_FACTORY_HOME` to the engine installation path:
```bash
export DARK_FACTORY_HOME=/opt/dark-factory    # or wherever you cloned it
```

Agents resolve engine paths (templates, skills) relative to `$DARK_FACTORY_HOME`.
Target project paths are resolved relative to the workspace.

### Project-Specific Instructions (CLAUDE.md)

The vsdd-factory plugin provides methodology, principles, rules, and agent instructions automatically. Project-specific context — build commands, git workflow, toolchain, and reference links — lives in a `CLAUDE.md` at the project root.

Run `/vsdd-factory:scaffold-claude-md` to auto-detect and generate this file. It inspects your project for language markers, task runners, CI configs, git branch strategy, and documentation, then presents a draft for your approval.

This file is maintained by the project owner and is not managed by the plugin.

### Visual Companion (optional)

`/vsdd-factory:visual-companion` provides a browser-based tool for showing mockups, diagrams, and interactive choices during brainstorming, brief creation, and architecture design. Requires Node.js. Early-phase skills automatically detect availability and fall back to Mermaid code blocks, excalidraw-export, or ASCII text when the visual companion isn't available.

## Operating Rules

### Pipeline Discipline
- No phase may begin until the previous phase's quality gate passes
- The Orchestrator is the only agent that transitions between phases
- All living spec artifacts are written to `<target-project>/.factory/specs/`
- Cycle-scoped artifacts are written to `<target-project>/.factory/cycles/vX.Y.Z-name/`
- Stories accumulate in `<target-project>/.factory/stories/`
- `<target-project>/.factory/STATE.md` is the single source of truth for pipeline progress

### Sub-Agent Delegation Rule (CRITICAL)

**Every `sessions_spawn` call MUST include `runtime: "subagent"`, `agentId`, and `cwd`.**
This is non-negotiable. All three fields are required to delegate to a specialist agent.
Omitting `runtime` or `agentId` causes the spawn to create a copy of the calling agent.

**The `cwd` parameter alone is NOT reliable.** Agents default to their `workspace` dir
(inside dark-factory). The `write` tool resolves relative paths from the workspace, not
from `cwd`. To guarantee correct file operations:

1. Prepend `cd <project-path> &&` in the task text
2. Specify ALL file paths as **absolute paths** in the task description

```
sessions_spawn({ runtime: "subagent", agentId: "business-analyst", cwd: "<resolved-project-path>", task: "cd <resolved-project-path> && Write analysis to <resolved-project-path>/.factory/planning/market-context.md" })
```

Call `agents_list` on startup to discover valid agent IDs. The orchestrator
MUST NOT use `agentId: "orchestrator"` — it delegates, never to itself.

### Workspace Isolation Rule (CRITICAL)

**Agents MUST NEVER create git worktrees, branches, or repos inside the dark-factory
engine directory.** All git operations target the product repository (resolved dynamically by the orchestrator at session start).

Before any `git worktree add`, `git checkout --orphan`, or `git init`:
1. Verify `pwd` does NOT contain `dark-factory`
2. Verify `git remote get-url origin` does NOT point to the dark-factory repo
3. After worktree creation, verify `.git` file does NOT reference dark-factory

If any check fails, STOP immediately and report the error. Do not attempt to fix it.

### VSDD Constraints
- **Spec Supremacy:** The spec is the highest authority below the human. Tests serve the spec. Code serves the tests.
- **Red Before Green:** No implementation code is written until a failing test demands it
- **Adversary Independence:** The Adversary MUST use a different model family than the Builder
- **Context Reset:** The Adversary gets a fresh context window on every review pass
- **Five-Dimensional Convergence:** Not done until specs, tests, implementation, AND formal proofs all survive adversarial review

### Single Source of Truth Rule

Every metric (BC count, story count, wave summary, VP count) has ONE authoritative source. All other documents cite the authoritative source — they do not re-derive the value.

| Metric | Authoritative Source | Cites (do not re-derive) |
|--------|---------------------|--------------------------|
| BC count | BC-INDEX.md | PRD, STORY-INDEX, STATE.md |
| Story count | STORY-INDEX.md | STATE.md Wave Summary |
| VP count | VP-INDEX.md | ARCH-INDEX, STATE.md |
| Wave assignment | sprint-state.yaml | STORY-INDEX, STATE.md |

When updating a count, update the authoritative source FIRST, then propagate to citing documents. State-manager should auto-generate denormalized copies from the authoritative source where possible.

### Governance Policies

Policy flags are top-level integrity rules that prevent specific classes of drift. Each policy is enforced by multiple agents and validated by consistency-validator criteria.

#### bc_array_changes_propagate_to_body_and_acs

- **Value:** `true` (always enforced)
- **Severity floor:** HIGH (blocking)
- **Symmetric with:** `bc_h1_is_title_source_of_truth`, `architecture_is_subsystem_name_source_of_truth`
- **Enforces:**
  - Story frontmatter `bcs:` array ↔ body Behavioral Contracts table: bidirectional completeness
  - Story frontmatter `bcs:` array ↔ AC trace annotations: bidirectional completeness
  - Story body Token Budget "N BCs" count == `len(bcs)`
- **Review axis:** story-frontmatter-body coherence
- **Validation criteria:** Consistency-validator criteria 67, 68, 69
- **Enforcing agents:** story-writer (pre-commit verification), adversary (5+ story sampling per pass), consistency-validator (criteria 67-69), product-owner (handoff to story-writer)
- **Orchestrator rule:** When a burst involves BC un-retirement, re-anchoring, or new BC creation affecting stories, dispatch story-writer AFTER product-owner completes (not in parallel)

**Generalization:** Whenever a list of IDs is maintained in two representations (machine-readable frontmatter and human-readable body) within the same artifact, edits to one MUST propagate to the other in the same atomic commit.

#### append_only_numbering

- **Value:** `true` (always enforced)
- **Severity floor:** HIGH (blocking)
- **Enforces:**
  - All VSDD identifiers (BC, CAP, VP, EC, DI, ASM, R, FM, STORY, HS) are never renumbered
  - Retired/removed IDs stay in indexes with `status: retired/removed` — never reused
  - Filename slugs are immutable — even when titles change, filenames keep original slugs
  - Retirement requires `replaced_by:` / `replaces:` traceability
- **Validation criteria:** Consistency-validator criterion 32 (cross-cycle conflicts), criterion 77 (ID reuse)
- **Enforcing agents:** product-owner (slug protection rule), spec-steward (append-only governance), consistency-validator (criteria 32, 77)

#### lift_invariants_to_bcs

- **Value:** `true` (always enforced)
- **Severity floor:** MEDIUM (blocking at 3+ orphans)
- **Enforces:**
  - Every DI-NNN in `domain-spec/invariants.md` cited by at least one BC's Traceability L2 Invariants field
  - Bidirectional: invariant Scope/enforcer column names BCs that cite it back
  - Orphan invariants (declared but no BC enforces) are drift findings
- **Validation criteria:** Consistency-validator criterion 74
- **Enforcing agents:** product-owner (invariant lifting obligation), adversary (orphan detection review axis), consistency-validator (criterion 74)

#### semantic_anchoring_integrity

- **Value:** `true` (always enforced)
- **Severity floor:** MEDIUM (CRITICAL when mis-anchor misleads implementer)
- **Enforces:**
  - Every anchor claim (BC→CAP, BC→Subsystem, VP→anchor_story, traceability descriptions) must be semantically correct, not just syntactically valid
  - Mis-anchoring is NEVER an "Observation" or "deferred post-v1" — it always blocks convergence
- **Validation criteria:** Consistency-validator criteria 70, 71, 72, 73
- **Enforcing agents:** adversary (semantic anchoring audit), consistency-validator (criteria 70-73), orchestrator (human review gate question)

#### creators_justify_anchors

- **Value:** `true` (always enforced)
- **Severity floor:** MEDIUM (blocking)
- **Enforces:**
  - Agents creating anchors must justify each choice against the source-of-truth artifact
  - Mechanical citation without body substantiation is a finding
  - "Stop and ask" clause: if justification cannot be written, agent must stop rather than guess
- **Enforcing agents:** product-owner (BC→CAP justification), architect (ADR/subsystem/crate justification), story-writer (SS-ID/dependency/VP justification), business-analyst (CAP-NNN justification)

#### bc_h1_is_title_source_of_truth

- **Value:** `true` (always enforced)
- **Severity floor:** HIGH (blocking)
- **Enforces:**
  - BC file H1 heading is the authoritative title — all downstream references must match verbatim
  - Title enrichment must be moved INTO the H1, not left as index-only context
  - H1 and postconditions must be internally consistent
- **Validation criteria:** Consistency-validator criterion 75
- **Enforcing agents:** product-owner (H1 authority rule, enrichment-into-H1), adversary (title sync review axis), consistency-validator (criterion 75)

#### architecture_is_subsystem_name_source_of_truth

- **Value:** `true` (always enforced)
- **Severity floor:** HIGH (blocking)
- **Enforces:**
  - ARCH-INDEX Subsystem Registry is the authoritative source for subsystem SS-NN IDs
  - All references (BC frontmatter `subsystem:`, story frontmatter `subsystems:`) must use SS-NN IDs from the registry
- **Validation criteria:** Consistency-validator criterion 76
- **Enforcing agents:** product-owner (BC subsystem validation), architect (subsystem scope verification), story-writer (SS-ID justification), adversary (subsystem label sync review axis), consistency-validator (criterion 76)

#### vp_index_is_vp_catalog_source_of_truth

- **Value:** `true` (always enforced)
- **Severity floor:** HIGH (blocking)
- **Symmetric with:** `architecture_is_subsystem_name_source_of_truth` (subsystem names), `bc_h1_is_title_source_of_truth` (BC titles), `bc_array_changes_propagate_to_body_and_acs` (story frontmatter)
- **Enforces:**
  - VP-INDEX.md is the authoritative VP enumeration — changes must propagate to architecture anchor docs in the same burst
  - `verification-architecture.md` Provable Properties Catalog must match VP-INDEX (VP rows, P0/P1 lists, totals)
  - `verification-coverage-matrix.md` VP-to-Module table and totals must match VP-INDEX
  - VP-INDEX arithmetic self-consistency (total = sum of per-tool counts = row count)
- **Validation criteria:** Consistency-validator criteria 78, 79, 80
- **Enforcing agents:** architect (propagation obligation), adversary (VP-INDEX ↔ arch-doc review axis), product-owner (VP citation change handoff), consistency-validator (criteria 78-80)
- **Orchestrator rule:** When a burst involves VP additions, retirements, module reassignments, or tool changes, architect must update both architecture anchor docs in the same burst

### Hierarchical Specification

All specs follow a 4-level hierarchy:

| Level | Name | Purpose | Consumer | ID Format |
|-------|------|---------|----------|-----------|
| L1 | Product Brief | Capture user intent | Humans, business-analyst | -- (natural language) |
| L2 | Domain Spec | Domain model independent of implementation | Architect, product-owner | CAP-NNN, DI-NNN, DEC-NNN, ASM-NNN, R-NNN, FM-NNN |
| L3 | Behavioral Contracts | Formal behavioral specification per subsystem | Test-writer, implementer, adversary | BC-S.SS.NNN, NFR-NNN, E-xxx-NNN |
| L4 | Verification Properties | Machine-verifiable properties | Formal-verifier, Kani, proptest | VP-NNN |

Each level traces to the one above. The consistency-validator verifies
the chain L1->L2->L3->L4 is unbroken at every phase gate.

### Canonical Frontmatter Standard

Every VSDD artifact MUST include this frontmatter block:

```yaml
---
document_type: [type]               # required -- what kind of document
level: [L1|L2|L3|L4|ops]           # required -- spec hierarchy level
version: "1.0"                      # required -- semver
status: draft|revised|approved      # required -- document status
producer: [agent-id]                # required -- which agent wrote this
timestamp: YYYY-MM-DDTHH:MM:SS     # required -- when produced
phase: [1a|1b|2|3|4|5|6]           # required -- VSDD phase
inputs: []                          # required -- list of input files consumed
input-hash: [md5]                   # optional -- hash of inputs for change detection
traces_to: [file]                   # required for L2+ -- parent spec this traces to
pass: [integer]                     # required for iterative artifacts -- iteration number (1-based)
previous_review: [file path]        # required for pass >= 2 -- link to prior iteration
---
```

Rules:
- Every template MUST include this frontmatter block
- Agents MUST populate all required fields when producing artifacts
- The consistency-validator checks that frontmatter is complete at every gate
- `pass` and `previous_review` only required for iterative artifacts (adversarial
  reviews, holdout evaluations, code reviews)
- `traces_to` not required for L1 (Product Brief is the root)

### ID Format Reference

| Level | ID Format | Meaning | Example | Scope |
|-------|-----------|---------|---------|-------|
| L2 | CAP-NNN | Domain capability | CAP-001: Parse markdown files | Lifecycle |
| L2 | DI-NNN | Domain invariant | DI-001: URL extraction preserves position | Lifecycle |
| L2 | DEC-NNN | Domain edge case | DEC-001: Empty .md file (0 bytes) | Lifecycle |
| L2 | ASM-NNN | Assumption requiring validation | ASM-001: pulldown-cmark byte offsets accurate | Lifecycle |
| L2 | R-NNN | Risk | R-001: GitHub slug algorithm changes | Lifecycle |
| L2 | FM-NNN | Failure mode | FM-001: DNS resolution failure | Lifecycle |
| L3 | SS-NN | Architecture subsystem | SS-01: Sensor Adapters | Lifecycle |
| L3 | BC-S.SS.NNN | Behavioral contract (hierarchical) | BC-2.3.045: Notification timeout handling | Lifecycle |
| L3 | NFR-NNN | Non-functional requirement | NFR-001: Parse 1000 files in <= 2s | Lifecycle |
| L3 | E-xxx-NNN | Error taxonomy | E-NET-001: DNS lookup failed | Lifecycle |
| L3 | EC-NNN | Edge case (within BC) | EC-001: Empty input | Local to BC |
| L4 | VP-NNN | Verification property | VP-001: Slug GitHub-equivalence | Lifecycle |
| Stories | STORY-NNN | Story | STORY-001: Implement input validation | Lifecycle |
| Stories | EPIC-NNN | Epic | EPIC-001: Core Engine | Lifecycle |
| Stories | AC-NNN | Acceptance criterion (traces to BC) | AC-001 (traces to BC-2.1.001) | Local to story |
| Stories | EAC-NNN | Epic acceptance criterion | EAC-001: All core APIs operational | Local to epic |
| Stories | GAP-NNN | Gap register entry (deferred requirement) | GAP-001: External API not available in v1 | Lifecycle |
| Holdout | HS-NNN | Holdout scenario (lifecycle-tracked) | HS-001: Malformed input injection | Lifecycle |
| Holdout | WHS-W[N]-NNN | Wave holdout scenario (wave-scoped) | WHS-W2-001: Wave 2 integration check | Cycle (resets) |
| Reviews | ADV-<CYCLE>-P[N]-[SEV]-NNN | Adversarial finding (cycle-prefixed) | ADV-P1CONV-P03-CRIT-001 | Cycle |
| Reviews | CR-NNN | Code review finding | CR-001: Column diagnostic wrong | Cycle |
| Reviews | SEC-NNN | Security finding | SEC-001: SSRF guard missing | Cycle |
| Reviews | FIX-P[N]-NNN | Fix PR from phase N review | FIX-P4-001: Fix BC timeout handling | Lifecycle |
| Ops | TD-NNN | Tech debt register entry | TD-001: Refactor auth middleware | Lifecycle |
| Ops | EVAL-NNN | Holdout evaluation result | EVAL-HS-001-P1: HS-001 pass 1 result | Cycle |
| UX | SCR-NNN | UX screen specification | SCR-001: Dashboard overview | Lifecycle |
| UX | CMP-NNN | UI component (within screen) | CMP-001: AlertCard | Local to SCR |
| UX | ELM-NNN | UI element (within screen) | ELM-001: severity-badge | Local to SCR |
| UX | INT-NNN | UI interaction (within screen) | INT-001: click-to-expand | Local to SCR |
| Architecture | AD-NNN | Architecture decision | AD-001: Use Kani for model checking | Lifecycle |

### ID Scope Definitions

| Scope | Meaning | Numbering Rule |
|-------|---------|---------------|
| **Lifecycle** | Continuous across all cycles. Never renumbered, never reused (Policy 1). Retired IDs stay in indexes. | Append-only sequential |
| **Cycle** | Resets per convergence cycle. Prefix identifies the cycle (e.g., `ADV-P1CONV-P03-CRIT-001`). | Sequential within cycle |
| **Local** | Scoped to a parent artifact. EC-NNN is local to its BC file. AC-NNN is local to its story. CMP/ELM/INT are local to their SCR. | Sequential within parent |

### Priority Scale

| Priority | Meaning |
|----------|---------|
| P0 | Must-have for release. Blocking. |
| P1 | Should-have for release. Significant user value. |
| P2 | Nice-to-have. Can defer without major impact. |

### Model Routing
- Model selection is managed by LiteLLM proxy at localhost:4000
- Each agent has a designated model tier in openclaw.json — do not override without Orchestrator approval
- The Adversary uses GPT-5.4 (primary) or DeepSeek-V3.2 (fallback) — NEVER Claude
- Three-vendor strategy: Claude (builders), OpenAI (adversary + PR review), Gemini (review/secondary adversary), DeepSeek (local fallback)

### Model Tier Mapping

| LiteLLM Tier | Actual Model | Cost/M (in/out) | Used By |
|-------------|--------------|-----------------|---------|
| `judgment/primary` | Claude Opus 4.6 | $5/$25 | Orchestrator, Product Owner, Architect, Security Reviewer |
| `implementation/primary` | Claude Sonnet 4.6 | $3/$15 | 12 builder agents |
| `validation/primary` | Claude Haiku 4.5 | $1/$5 | Consistency Validator, Technical Writer, Spec Steward |
| `adversary/primary` | GPT-5.4 | $2.50/$15 | Adversary, Code Reviewer, Holdout Evaluator, PR Reviewer |
| `review/primary` | Gemini 3.1 Pro | $2/$12 | Secondary adversary pass, rotating code review, security review second opinion |
| `fallback/fast` | Codestral 22B | $0 (self-hosted) / ~$0.15/M (OpenRouter) | Fast code gen fallback for implementation + validation tiers |
| `fallback/standard` | DeepSeek-V3 0324 | $0 (self-hosted) / ~$0.60/M (OpenRouter) | Standard fallback for all tiers — tool calling, reasoning, code gen |
| `fallback/reasoning` | Qwen3-235B-A22B (thinking) | ~$2.80/M (OpenRouter) | Reasoning fallback for adversary + judgment tiers — adversarial review, complex debugging |

### Fallback Architecture (Three-Tier)

Fallback models are used when primary cloud APIs are down, rate-limited, or budget-exceeded.
The factory uses a **three-tier fallback chain** optimized for different task profiles:

| Fallback Tier | Model | Strengths | Weaknesses | Hosting |
|---------------|-------|-----------|------------|---------|
| **Fast** | Codestral 22B | Low latency, cheap, fits on DGX (22GB INT8) | No reasoning, weak adversarial | Self-hosted via vLLM + OpenRouter |
| **Standard** | DeepSeek-V3 0324 | Strong tool calling (strict mode), 128K context, $0.60/M | Needs INT4 quantization for self-hosting (350-400GB full) | OpenRouter primary, self-hosted (quantized) on DGX |
| **Reasoning** | Qwen3-235B-A22B (thinking mode) | 131K context, unified thinking/non-thinking mode, strong adversarial | Higher latency in thinking mode, ~$2.80/M | OpenRouter only (230GB INT8 exceeds 128GB DGX) |

**Per-tier fallback chains:**
- Judgment: Opus → GPT-5.4 → `fallback/reasoning` (Qwen3 thinking)
- Implementation: Sonnet → `fallback/fast` (Codestral) → `fallback/standard` (DeepSeek-V3)
- Adversary: GPT-5.4 → `fallback/standard` (DeepSeek-V3) → `fallback/reasoning` (Qwen3 thinking)
- Review: Gemini 3.1 Pro → `fallback/standard` (DeepSeek-V3)
- Validation: Haiku → `fallback/fast` (Codestral)

**Safety note:** Small models (Codestral, Qwen2.5-Coder-32B) have inverted safety profiles — planning-level defenses that work in frontier models fail in smaller models. Do NOT use `fallback/fast` for adversarial validation or security review paths. Use `fallback/standard` or `fallback/reasoning` instead.

All agents use LiteLLM tier names in openclaw.json (e.g., `"model": {"primary": "adversary/primary"}`).
LiteLLM resolves these to the actual upstream provider and model.

### Information Asymmetry Walls

Seven agents have explicit information asymmetry walls -- enforced context
exclusions that prevent them from seeing artifacts that would compromise their
independent judgment. Walls are enforced via Lobster workflow `context.exclude`
blocks and documented in each agent's `AGENTS.md`.

| Agent | Phase | What is Excluded | Why |
|-------|-------|------------------|-----|
| holdout-evaluator | 3.5, 4b | Source code, specs, stories, implementation | ML-style train/test separation |
| pr-reviewer | 3b | All `.factory/` artifacts | Reviews like a human -- diff only |
| security-reviewer (PR) | 3b | Implementer notes/session logs | Independent security assessment |
| security-reviewer (wave) | 3c | Per-story PR review comments | Fresh cross-story analysis |
| wave adversary | 3c | Per-story PR review comments | Fresh integration analysis |
| Phase 4 adversary | 4 | TDD logs, wave schedule, own prior passes | Fresh attacker perspective |
| code-reviewer (Gemini) | 4 | Adversary findings | Independent secondary review |
| formal-verifier | 5 | Adversary findings | Specification-driven verification |

**Convention:** Each walled agent's `AGENTS.md` includes an "Information Asymmetry
Wall" section documenting excluded paths and rationale. The wall is enforced in
`workflows/greenfield.lobster` via `context.exclude` blocks on the relevant steps.

Soft instructions alone are insufficient for wall enforcement (same lesson as the
orchestrator non-coding rule from DF-023). Walls must be structurally enforced
through context exclusion.

### Agent Permission Model

Agents are assigned tool profiles based on their role, not their convenience. The principle: separate "who writes files" from "who commits them."

| Role | Agents | Profile | Shell access | Commits |
|------|--------|---------|-------------|---------|
| **Spec producers** | product-owner, story-writer, architect | `coding` | No | State-manager commits `.factory/` artifacts |
| **Code producers** | implementer, test-writer | `full` | Yes — compile, test, commit in worktrees | Direct commits in feature branch worktrees |
| **Coordinators** | orchestrator, pr-manager | Restricted | No — delegate everything | pr-manager spawns github-ops for gh operations |
| **Infrastructure** | devops-engineer, state-manager | `full` | Yes — git, gh, tooling | devops-engineer owns repo/CI/CD; state-manager owns `.factory/` branch |
| **Reviewers** | adversary, code-reviewer, pr-reviewer, spec-reviewer, consistency-validator, holdout-evaluator | `coding` or `read-only` | Read artifacts, write findings | No commits — findings go to `.factory/` via state-manager |
| **Tool-based reviewers** | accessibility-auditor | `full` | Yes — runs axe-core, lighthouse, pa11y, eslint jsx-a11y | No commits — findings go to `.factory/` via state-manager |

**Why spec producers don't get shell access:**

1. **They write markdown, not code.** They never need to compile, run tests, or execute build tools.
2. **Centralized commits prevent version-race regressions.** If product-owner, story-writer, and state-manager all commit independently, citation version mismatches occur (e.g., state-manager writes STORY-INDEX citations BEFORE story-writer bumps the version).
3. **State-manager runs LAST in every burst** (Lesson 1). It sees all written files and commits atomically.
4. **Audit trail is cleaner.** All `.factory/` commits come from state-manager — one committer, predictable history.

**Why code producers get shell access:**

1. **They must compile and run tests.** Implementer runs `cargo test` after each TDD step. Test-writer runs `cargo check` to verify stubs compile.
2. **They work in isolated worktrees** on feature branches — different git lifecycle than `.factory/`.
3. **Their commits are per-story**, not per-burst — different granularity than spec artifacts.

**Why coordinators don't execute:**

1. **Orchestrator is a scheduler, not a doer.** It reads workflow data and dispatches agents. If it could write files, it would bypass specialist boundaries.
2. **pr-manager is a project manager for PRs.** It triages findings and delegates fixes. It spawns github-ops for all `gh` and `git` commands — same principle as orchestrator spawning specialists.

### File Conventions
- All specs use Markdown with YAML frontmatter
- All artifacts include producer, timestamp, and input-hash metadata
- Tally findings track the full VSDD traceability chain

### Specification Artifact File Conventions

- L1 artifacts: `product-brief.md` (project root), `.factory/specs/product-brief.md`
- L2 artifacts: `.factory/specs/domain-spec-L2.md`
- L2 support: `.factory/specs/domain-research.md`, `requirements-analysis.md`
- L3 artifacts: `.factory/specs/prd.md` (index),
  `.factory/specs/behavioral-contracts/BC-S.SS.NNN.md` (per-file)
- L4 artifacts: `.factory/specs/verification-properties/VP-NNN.md` (per-file)
- Stories: `.factory/stories/STORY-NNN.md` (per-file)
- Holdout scenarios: `.factory/holdout-scenarios/HS-NNN.md` (per-file)
- Auto-generated indexes: `BC-INDEX.md`, `VP-INDEX.md`, `STORY-INDEX.md`, `HS-INDEX.md`

### Iterative Artifact Naming

Artifacts that go through multiple passes (adversarial reviews, holdout
evaluations, code reviews) MUST follow this naming convention:

- `[artifact-type]-pass[N].md` -- Nth iteration (pass1, pass2, pass3)
- `[artifact-type]-final.md` -- Canonical final version
- `[artifact-type]-work/pass[N]/` -- Working data for each pass

No ad-hoc suffixes (-clean, -rerun, -regression, -phase4-final). If the
artifact is iterative, use pass numbering. If it's the final version, use -final.

Frontmatter for iterative artifacts MUST include:
- `pass: [integer]` -- iteration number (1-based)
- `previous_review: [file path]` -- link to prior iteration (null for pass 1)

### Artifact Sharding Convention (DF-021)

Monolithic specification documents are decomposed into an **index + detail file** pattern
for optimal agent consumption. Research shows 800-1,200 token detail files outperform
monolithic documents (28.6% faster, 16.6% fewer tokens) due to "lost in the middle"
degradation in long-context models.

**Universal rules:**
- Every sharded directory has an INDEX file (e.g., ARCH-INDEX.md, ADV-P[N]-INDEX.md, EVAL-INDEX.md)
- Every detail file has `traces_to:` in frontmatter pointing at its index
- The index file has `traces_to:` pointing at its parent artifact
- Agents load ONLY the detail files relevant to their task, NOT entire directories
- The orchestrator reads index files for gate decisions, NOT all detail files

**Sharded artifact types:**

| Artifact Type | Directory | Index File | Detail File Pattern |
|--------------|-----------|------------|-------------------|
| Architecture | `architecture/` | `ARCH-INDEX.md` | `[section-name].md` |
| Behavioral Contracts | `behavioral-contracts/` | `BC-INDEX.md` | `BC-S.SS.NNN.md` |
| Verification Properties | `verification-properties/` | `VP-INDEX.md` | `VP-NNN.md` |
| Stories | `stories/` | `STORY-INDEX.md` | `STORY-NNN.md` |
| Holdout Scenarios | `holdout-scenarios/` | `HS-INDEX.md` | `HS-NNN.md` |
| Holdout Evaluations | `evaluations/pass-N/` | `EVAL-INDEX.md` | `EVAL-HS-NNN-P[N].md` |
| Adversarial Reviews | `adversarial-reviews/` | `ADV-P[N]-INDEX.md` | `ADV-P[N]-NNN.md` |
| PRD Supplements | `prd-supplements/` | (referenced from prd.md) | `[section-type].md` |

**Artifacts that remain monolithic** (under optimal size or consumed as a whole):
STATE.md, module-criticality.md, convergence-report.md, consistency-report.md,
domain-research.md, fuzz-report.md, security-scan-report.md, code-review.md,
security-review.md, red-gate-log.md.

**Context discipline per agent:**
- **Implementer:** module-decomposition + dependency-graph + api-surface + relevant BCs
- **Formal-verifier:** verification-architecture + purity-boundary-map + tooling-selection + relevant VPs
- **Story-writer:** module-decomposition + dependency-graph
- **Test-writer:** api-surface + test-vectors + relevant BCs
- **Consistency-validator:** ARCH-INDEX + verification-coverage-matrix + BC-INDEX + VP-INDEX
- **Orchestrator:** Index files only for gate decisions; dependency DAG for parallel triage

### Security Review Touchpoints

Proactive and reactive security review occurs at four points in the pipeline:

| # | Where | Trigger | Wall | What It Reviews |
|---|-------|---------|------|----------------|
| 1 | Per-story PR | Story touches CRIT/HIGH module | No impl reasoning | PR diff for CWE/OWASP |
| 2 | Wave integration | Wave has CRIT/HIGH stories | No per-story reviews | Combined wave diff for cross-story attack surfaces |
| 3 | Phase 4 | Adversary finds security findings | No impl reasoning | Deep CWE/OWASP analysis |
| 4 | Phase 5 | Security scan finds HIGH/CRIT | -- | Triage scan findings |

Module criticality is classified in `.factory/specs/module-criticality.md` during Phase 1.

### Artifact Lifecycle Status Model (DF-030)

Every living artifact (BCs, VPs, holdout scenarios, stories) has lifecycle status
tracked in its frontmatter:

```yaml
lifecycle_status: active | deprecated | retired | removed
introduced: v1.0.0         # cycle that created this artifact
modified: [v1.1.0, v1.3.0] # cycles that modified it
deprecated: null            # cycle that deprecated (null if active)
deprecated_by: null         # which cycle deprecated it
replacement: null           # replacement artifact (if any)
retired: null               # cycle that retired (null if active)
removed: null               # cycle that removed (null if not removed)
```

**Lifecycle states:**

```
BC/Story: ACTIVE --> DEPRECATED --> RETIRED --> REMOVED
VP:       ACTIVE --> DEPRECATED/WITHDRAWN --> RETIRED --> REMOVED
Holdout:  ACTIVE --> STALE --> RETIRED
```

- **ACTIVE:** Normal operating state. Tests run. Agents reference.
- **DEPRECATED:** Still in specs/ but marked. Tests updated to skip. Sunset date set.
- **RETIRED:** Tests no longer run. Kept in specs/ with retired status.
- **REMOVED:** Deleted from specs/ (preserved in git history and cycle archives).
- **WITHDRAWN (VP only):** Property no longer applies; withdrawal document created.
- **STALE (Holdout only):** Scenario references features that no longer exist.

### Spec Versioning Strategy (DF-030)

Living specs in `specs/` don't have version numbers in their filenames. Version
history is tracked through:

1. **Git history** on factory-artifacts branch (every phase gate commit)
2. **Git tags** at release boundaries (v1.0.0, v1.1.0, etc.)
3. **Frontmatter `version` field** -- semantic version of the spec itself

Spec version bumps:

| Change Type | Spec Version Bump | Triggered By |
|------------|------------------|-------------|
| New BC/VP added | MINOR | Feature cycle (F2 spec evolution) |
| BC/VP modified | MINOR | Feature cycle or bug fix |
| BC/VP deprecated | MINOR | Deprecation cycle |
| Architecture restructured | MAJOR | Major refactor or greenfield re-run |
| Typo/formatting fix | PATCH | Maintenance or manual |

Retrieving historical specs:
```bash
git show v1.0.0:specs/prd.md              # PRD at v1.0.0
git show v1.1.0:specs/behavioral-contracts/BC-INDEX.md  # BCs at v1.1.0
git diff v1.0.0..v1.1.0 -- specs/         # Diff between releases
```

### Continuous Numbering Convention (DF-030)

All numbered artifacts continue incrementing across cycles. No resets.

| Artifact | Greenfield | Feature A | Feature B |
|----------|-----------|-----------|-----------|
| Stories | STORY-001 to STORY-040 | STORY-041 to STORY-055 | STORY-056+ |
| BCs | BC-1.01.001 to BC-4.03.012 | BC-5.01.001+ | BC-6.01.001+ |
| VPs | VP-001 to VP-020 | VP-021+ | VP-026+ |
| Holdout | HS-001 to HS-025 | HS-026+ | HS-033+ |
| Fix PRs | FIX-P4-001+ | FIX-P4-009+ | FIX-P5-010+ |

Cycle-scoped numbering (resets per cycle):

| Artifact | Pattern | Resets? |
|----------|---------|--------|
| Adversarial findings | ADV-P[N]-NNN | YES -- per cycle |
| Wave schedules | Wave 1, 2, 3 | YES -- per cycle |
| Wave holdout scenarios | WHS-W[N]-NNN | YES -- per cycle |
| Cost summary | Per cycle | YES -- cumulative also tracked |

### Artifact Path Migration (DF-030)

| Old Path | New Path |
|----------|----------|
| `.factory/phase-1-spec/prd.md` | `.factory/specs/prd.md` |
| `.factory/phase-1-spec/domain-spec-L2.md` | `.factory/specs/domain-spec-L2.md` |
| `.factory/phase-1-spec/prd-supplements/` | `.factory/specs/prd-supplements/` |
| `.factory/phase-1-spec/behavioral-contracts/` | `.factory/specs/behavioral-contracts/` |
| `.factory/phase-1-spec/verification-properties/` | `.factory/specs/verification-properties/` |
| `.factory/phase-1-spec/architecture/` | `.factory/specs/architecture/` |
| `.factory/phase-1-spec/ux-spec.md` | `.factory/specs/ux-spec.md` |
| `.factory/phase-1-spec/dtu-assessment.md` | `.factory/specs/dtu-assessment.md` |
| `.factory/phase-1-spec/gene-transfusion-assessment.md` | `.factory/specs/gene-transfusion-assessment.md` |
| `.factory/phase-1-spec/adversarial-reviews/` | `.factory/cycles/vX.Y.Z/adversarial-reviews/` |
| `.factory/phase-2-stories/stories/` | `.factory/stories/` |
| `.factory/phase-2-stories/epics.md` | `.factory/stories/epics.md` |
| `.factory/phase-2-stories/dependency-graph.md` | `.factory/stories/dependency-graph.md` |
| `.factory/phase-2-stories/sprint-state.yaml` | `.factory/stories/sprint-state.yaml` |
| `.factory/phase-2-stories/consistency-report.md` | `.factory/cycles/vX.Y.Z/consistency-report.md` |
| `.factory/phase-3-implementation/` | `.factory/cycles/vX.Y.Z/implementation/` |
| `.factory/phase-5-adversarial/` | `.factory/cycles/vX.Y.Z/adversarial-reviews/` |
| `.factory/phase-5-hardening/` | `.factory/cycles/vX.Y.Z/hardening/` |
| `.factory/phase-7-convergence/` | `.factory/cycles/vX.Y.Z/convergence/` |
| `.factory/module-criticality.md` | `.factory/specs/module-criticality.md` |

Paths unchanged: `.factory/STATE.md`, `.factory/cost-summary.md`,
`.factory/holdout-scenarios/` (already living).

### Release Process

After Phase 6 convergence and human approval, the devops-engineer executes the
release pipeline:

1. Determine semver version from story types (feat -> MINOR, fix -> PATCH, breaking -> MAJOR)
2. Generate CHANGELOG.md with quality evidence (convergence summary, test stats)
3. Create annotated git tag (`vX.Y.Z`)
4. Push tag to trigger `release.yml` CI workflow
5. Create GitHub Release with binaries, demo GIF, and convergence summary
6. Update README.md version badge and installation instructions

Release notes follow `templates/release-notes-template.md`. See
`workflows/skills/release/SKILL.md` for the complete protocol.

### Human-in-the-Loop Gates
| Gate | When | Human Action Required |
|------|------|----------------------|
| Spec Approval | After Phase 1d (adversarial spec review) | Review and approve final spec |
| Architecture Sign-off | After Phase 1b (verification architecture) | Approve purity boundaries and tech choices |
| Story Approval | After Phase 2 (story decomposition) | Review and approve story breakdown |
| Production Deploy | After Phase 6 (convergence) | Authorize deployment |

### Interactive Work vs. Shift Work

The factory operates in two distinct modes, separated by Phase 2's human approval gate:

**Interactive phases (Phases 1-2):** Human intent is incomplete. Back-and-forth is expected.
Agents present options, ask clarifying questions, and iterate on specs and stories. Human
approval gates are mandatory. This is collaborative work — the human and agents are forming
intent together.

**Shift work phases (Phases 3-6):** The spec is fully specified. Agents run end-to-end without
human intervention until a quality gate is reached. Quality is enforced by automated gates
(Red Gate, holdout evaluation, convergence metrics), not by human monitoring. The factory
should be able to run overnight without human presence during these phases.

The Phase 6 human approval gate is a **quality gate**, not an intent checkpoint. The human
reviews the convergence report and authorizes deployment — they are not shaping what gets
built, only confirming it meets the bar.

Feature Mode follows the same pattern: Phases F1-F3 are interactive (scoping), F4-F7 are
shift work (execution).

### Multi-Repo Projects

The Dark Factory supports projects spanning multiple repositories and languages. A Rust API
server in one repo, a Next.js frontend in another, and TypeScript/Python SDKs in additional
repos can all be orchestrated as a single project.

Multi-repo support uses OpenClaw's per-agent workspace configuration — each agent's
`workspace` field in `openclaw.json` points to the repo it operates on. Cross-repo
coordination uses Lobster workflows with per-step `cwd` parameters, and a project
manifest (`project.yaml`) describes the inter-repo dependency graph.

See **DF-012** for the complete multi-repo orchestration architecture, **DF-013** for
contract-first development and SDK generation across repos, and **DF-014** for semantic
code porting between languages.

## Sub-Agent Execution Policy

### Timeout Configuration

| Agent Tier | Default Timeout | Max Timeout | Rationale |
|-----------|----------------|-------------|-----------|
| Tier 1 (read-only) | 600s (10 min) | 900s | Coordination/research — should be fast |
| Tier 2 (spec-writers) | 900s (15 min) | 1800s | Document generation — moderate |
| Tier 3 (coding) | 1800s (30 min) | 3600s | Compilation + testing — needs headroom |

### One-Story-Per-Agent Rule

For Phase 3 (test-first implementation):
- **S/M complexity (1-5 points):** Max 2 stories per test-writer, max 2 per implementer
- **L/XL complexity (8-13 points):** Exactly 1 story per test-writer, 1 per implementer
- **Never** ask the same agent to both implement AND run the full test suite
  as a verification step — spawn a separate verification agent

### Pre-Compilation Requirement

Before spawning implementers for a new wave:
1. Run `cargo check` on the current workspace
2. If compilation fails from prior wave's incomplete work, fix first
3. Only then spawn the new wave's implementers

This prevents implementers from wasting timeout budget on others'
compilation errors.

## Build & Test (for target projects)

Commands depend on the target project's stack. The Orchestrator reads the target project's
configuration files for stack-specific commands. Common patterns:

```bash
# Rust
cargo test                    # Run tests
cargo clippy -- -D warnings   # Lint
cargo +nightly fmt --check    # Format check

# TypeScript
npm test                      # Run tests
npm run lint                  # Lint
npm run typecheck             # Type check

# Python
pytest                        # Run tests
ruff check .                  # Lint
mypy .                        # Type check
```

## Git Lifecycle & Code Delivery (DF-024)

### Repository Initialization

New projects start with agent-driven repo creation:
1. Orchestrator asks human for org, name, visibility, default branch
2. DevOps-engineer creates repo via `gh repo create`, configures branch protection
3. Git rerere enabled for automatic conflict resolution reuse
4. `.factory/merge-config.yaml` created from template
5. **factory-artifacts orphan branch created + `.factory/` mounted as worktree (BLOCKING)**

### Greenfield Initialization Order (Dependency Chain)

The following order is **mandatory** — each step requires the previous:

| Order | Agent | Task | Dependency |
|-------|-------|------|------------|
| 1 | devops-engineer | Repo creation + orphan branch + `.factory/` worktree mount | None (first step) |
| 2 | state-manager | `.factory/` directory contents + `STATE.md` | **REQUIRES** worktree from step 1 |
| 3 | business-analyst | Market intelligence + domain research | PARALLEL with step 2 |

**Critical rule:** State-manager MUST NOT run until devops-engineer confirms
`.factory/` is a valid git worktree on the `factory-artifacts` branch. Creating
`.factory/` as a regular directory breaks artifact backup and the entire
factory-artifacts branch lifecycle.

### Branch Naming Convention

- Default branch: `develop` (VSDD standard, not `main`)
- Feature branches: `feature/STORY-NNN` (one per story)
- Fix branches (wave integration): `feature/STORY-NNN-FIX-001`
- Fix branches (Phase 4/5/6): `fix/FIX-P[phase]-NNN` (e.g., `fix/FIX-P4-001`, `fix/FIX-P5-003`)
- Branch prefix configurable in `.factory/merge-config.yaml`

### Fix PR Naming Convention (Phases 4-6)

When Phases 4, 5, or 6 find issues on merged develop, fix tasks use the naming
pattern `FIX-P[phase]-NNN`:
- **FIX-P4-NNN:** Fixes from Phase 4 adversarial review findings
- **FIX-P5-NNN:** Fixes from Phase 5 formal hardening findings
- **FIX-P6-NNN:** Fixes from Phase 6 convergence failure routing

Fix PRs follow the same delivery rigor as story PRs (worktree, AI review,
security review if applicable) but skip stubs, Red Gate, and wave integration
gates. See `workflows/skills/fix-pr-delivery/SKILL.md`.

### Worktree Convention

Each story gets its own git worktree for isolation:
```
<project-root>/.worktrees/
  ├── STORY-001/    (feature/STORY-001 branch)
  ├── STORY-002/    (feature/STORY-002 branch)
  └── STORY-003/    (feature/STORY-003 branch)
```

- `.worktrees/` is in `.gitignore` -- never committed
- Worktrees created per-wave by devops-engineer
- All Phase 3 agents work inside the worktree
- Worktree removed after story PR merges

### Per-Story Delivery Flow

Each story follows the complete delivery lifecycle in its worktree:
```
stub -> test -> implement -> demo -> push -> PR -> AI review -> merge
```

1. Test-writer creates stubs and tests in worktree (Red Gate)
2. Implementer implements via TDD in worktree
3. Demo-recorder records per-AC demos in worktree:
   - CLI products: VHS `.tape` → `.gif` + `.webm`
   - Web products: Playwright `.spec.ts` → `.webm` + `.png`
   - Output: `docs/demo-evidence/` (committed to feature branch, visible in PR diff)
   - Both success AND error paths for each AC
4. Implementer pushes with `--force-with-lease`
5. PR-manager runs full 9-step process:
   a. Populate PR description from template (embeds demo .gif thumbnails)
   b. Verify demo evidence gate (`docs/demo-evidence/evidence-report.md` exists)
   c. Create PR via github-ops
   d. Security review (security-reviewer posts formal review)
   e. PR review convergence loop (pr-reviewer posts formal review, max 10 cycles)
   f. Wait for CI
   g. Dependency check
   h. Merge
   i. Cleanup
6. Worktree cleanup (devops-engineer)

### Wave Integration Gate

After all stories in a wave merge to develop:
1. Full test suite on merged develop
2. Adversarial review of combined wave diff
3. Holdout regression on affected scenarios
4. Wave-level demo (cross-story integration flows)
5. Fix loop if issues found (max 10 cycles)
6. Wave gate passes before next wave starts

### PR Review Agents

| Agent | Role | Model | Tier |
|-------|------|-------|------|
| pr-reviewer | Reviews PR diff with cognitive diversity | adversary model | T2 |
| pr-manager | Coordinates PR lifecycle, triages findings | Haiku | T2 |
| github-ops | Executes `gh` CLI commands for T2 agents | Sonnet | T3 |

### Merge Configuration

`.factory/merge-config.yaml` controls merge autonomy:
- **Level 3:** All PRs require human review
- **Level 3.5:** Low-risk auto-merge; medium/high-risk human review
- **Level 4:** Full auto-merge (AI review only)

## On-Demand References

| Topic | Location |
|-------|----------|
| Factory Values | `SOUL.md` |
| VSDD Methodology | `VSDD.md` |
| Coding Standards | `AGENTS.md` |
| Convergence Criteria | `CONVERGENCE.md` |
| Model Routing Config | `config/litellm-config.yaml` |
| Agent Configurations | `openclaw.json` |
| Agent Personality Prompts | `agents/<name>/AGENTS.md` |
| Pipeline Orchestration | `workflows/*.lobster` (Lobster YAML) |
| Phase Skills | `workflows/skills/*/SKILL.md` (Markdown instructions) |
| Enforcement Plugins | `plugins/src/` |
| Document Templates | `templates/` |
| Specification Hierarchy | `templates/L2-domain-spec-template.md`, `templates/prd-template.md` (L3), `templates/L4-verification-property-template.md` |
| Traceability Chain | `CONVERGENCE.md` section "VSDD Contract Chain" |

## Sandbox Alignment Notes (DF-023)

The following agents have `workspaceAccess: "ro"` sandbox annotations that are
currently unenforced (sandbox mode is off for local macOS deployment). When
NemoClaw/OpenShell deployment resumes, these need updating:

| Agent | Current Sandbox | Tool Tier | Issue | Required Change |
|-------|----------------|-----------|-------|----------------|
| adversary | ro | T2 | Must write adversarial-reviews/ | Change to rw |
| code-reviewer | ro | T2 | Must write code-review files | Change to rw |
| holdout-evaluator | ro | T3 | Must write eval reports + exec | Change to rw |
| consistency-validator | ro | T2 | Must write consistency-report | Change to rw |

Agents correctly marked `ro` (no changes needed):
- security-reviewer (T2, ro aligns -- writes only review reports)
- codebase-analyzer (T1, ro aligns -- read-only)
- visual-reviewer (T1, ro aligns -- read-only)
- accessibility-auditor (T2, ro aligns -- writes only audit reports)

New agents added by DF-024 (sandbox mode off, no changes needed now):
- pr-reviewer (T2, no sandbox annotation -- reads diffs, writes review findings)
- pr-manager (T2, no sandbox annotation -- reads/writes PR lifecycle artifacts)
- github-ops (T3, no sandbox annotation -- needs exec for `gh` CLI)

New agents added by DF-027 (sandbox mode off, no changes needed now):
- dx-engineer (T3, needs exec for tool installation, version checks, key validation)
-  (T3, needs exec for `mcporter call` commands)

## Operational Infrastructure (DF-027)

### Secrets & Environment Management

Product repos use `.env` for runtime keys (Stripe, Okta, etc.) and `.envrc`
for direnv auto-loading. Factory LLM keys live in the dark-factory repo, NOT
in product repos.

- `.env.example` -- committed template (key names only, no values)
- `.env` -- gitignored (human populates with actual values)
- `.envrc` -- committed (`dotenv .env` for direnv)

Environment validation checks key NAMES only, never prints values:
`[ -n "${!key}" ]` -- reports "set" or "MISSING". Missing required keys
trigger a BLOCKING human notification.

### Pipeline Resume / Recovery

The orchestrator reconstructs pipeline state from existing artifacts on restart:
STATE.md (current phase), sprint-state.yaml (per-story state), .worktrees/
(git status), GitHub PRs (gh pr list). Presents deduced state to human for
confirmation before resuming.

Implementers use **micro-commits** during TDD (one commit per passing test,
`wip(STORY-NNN):` prefix), squashed before PR. Worst-case loss on crash:
~5 minutes of one agent's work.

### Cost Monitoring & Budget Guards

The cost-tracker plugin (plugins/src/cost-tracker.ts) logs per-call costs
with phase/wave/story metadata and maintains `.factory/cost-summary.md`
with phase aggregation, projections, and top-5 expensive stories.

**Tiered budget response:**
- 0-70%: No action. Log costs.
- 70-85%: WARN notification. No model changes.
- 85-95%: ALERT. Downgrade NON-CRITICAL agents only.
- 95-100%: PAUSE pipeline. BLOCKING notification.
- >100%: HARD STOP. No agent spawns.

**Protected agents** (never downgraded): adversary, holdout-evaluator,
formal-verifier, pr-reviewer, security-reviewer. Configured in
`.factory/autonomy-config.yaml`.

### Toolchain Validation

dx-engineer runs full toolchain preflight before Phase 3: checks all required
tools, validates versions, installs missing tools after security-reviewer audit
and human approval. SHA pinning via `--locked`, `.tool-versions`, Docker SHA
digests, GitHub Actions commit SHAs.

**Any tool security finding -- regardless of severity -- BLOCKS installation
until human approves.**

### Human Notification System

Configurable notification channel in `.factory/merge-config.yaml`:
github-issues (default), slack, discord, email, terminal. Available to
orchestrator, pr-manager, dx-engineer, state-manager.

### Model Availability

All 3 model families (Claude, GPT-5.4, Gemini) are REQUIRED. dx-engineer
checks LiteLLM /health before pipeline starts. No silent model fallback ever --
human approval required for any model substitution.

### Artifact Backup

`.factory/` is a git worktree on a dedicated `factory-artifacts` orphan branch.
state-manager commits and pushes artifacts at every phase gate. Recovery:
`git worktree add .factory factory-artifacts` restores all artifacts.

### Rate Limits

LiteLLM config includes verified RPM/TPM per model tier:
- Anthropic Tier 2: 1,000 RPM, 450K ITPM (cached tokens don't count)
- OpenAI Tier 2: 5,000 RPM, 1M TPM
- Gemini: conservative 1,000 RPM, 1M TPM (not publicly published)

### MCP Access for All Agents

MCP servers (Perplexity, Context7, Tally, Playwright) are configured in
`openclaw.json` under `mcp.servers`. All agents — including sub-agents
spawned via `sessions_spawn` — have direct access to MCP tools. No
mcporter or  middleman is needed.

Available MCP tools:
- `perplexity_search`, `perplexity_ask`, `perplexity_research`, `perplexity_reason`
- `resolve-library-id`, `query-docs` (Context7)
- `browser_*` (Playwright)
- Tally finding tracker tools

## L2 Domain Spec and UX Spec Sharding (DF-021 Extension)

### L2 Domain Spec
```
.factory/specs/domain-spec/
├── L2-INDEX.md          ← Document Map, Cross-References, ID Registry
├── capabilities.md      ← CAP-NNN (consumed by PO, architect, story-writer)
├── entities.md          ← Domain entities (architect, ux-designer)
├── invariants.md        ← DI-NNN business rules (PO, architect)
├── events.md            ← Domain events (architect)
├── edge-cases.md        ← DEC-NNN (story-writer, test-writer)
├── assumptions.md       ← ASM-NNN (PO, test-writer)
├── risks.md             ← R-NNN (PO, architect)
├── failure-modes.md     ← FM-NNN (architect, test-writer)
├── differentiators.md   ← Competitive mapping (PO)
└── event-flow.md        ← Optional causality chains (human reference)
```

### UX Spec (UI products only)
```
.factory/specs/ux-spec/
├── UX-INDEX.md          ← Screen/Flow inventory + global settings
├── screens/
│   ├── SCR-001-[name].md
│   └── ...
└── flows/
    ├── FLOW-001-[name].md
    └── ...
```

### Sharded Artifact Summary

| Artifact | Directory | Index | Detail Pattern | Producer |
|----------|-----------|-------|---------------|----------|
| Architecture | architecture/ | ARCH-INDEX.md | [section].md | architect |
| L2 Domain Spec | domain-spec/ | L2-INDEX.md | [section].md | business-analyst |
| UX Spec | ux-spec/ | UX-INDEX.md | screens/SCR-NNN.md, flows/FLOW-NNN.md | ux-designer |
| Adversarial Reviews | adversarial-reviews/ | ADV-P[N]-INDEX.md | ADV-P[N]-NNN.md | adversary |
| Holdout Evaluations | evaluations/ | EVAL-INDEX.md | EVAL-HS-NNN-P[N].md | holdout-evaluator |
| Stories | stories/ | STORY-INDEX.md | STORY-NNN.md | story-writer |
| Behavioral Contracts | behavioral-contracts/ | BC-INDEX.md | BC-S.SS.NNN.md | product-owner |
| Verification Properties | verification-properties/ | VP-INDEX.md | VP-NNN.md | architect |
| Holdout Scenarios | holdout-scenarios/ | HS-INDEX.md | HS-NNN.md | product-owner |
