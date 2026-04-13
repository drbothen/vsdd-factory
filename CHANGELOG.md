# Changelog

## 0.11.0 — CLAUDE.md scaffolding skill

### Added
- **scaffold-claude-md skill** (`/vsdd-factory:scaffold-claude-md`) — auto-detects project language, build/test/lint commands, git workflow, and project references to generate a project-specific `CLAUDE.md`
- Activate skill now suggests `scaffold-claude-md` when no `CLAUDE.md` exists
- Optional `scaffold-claude-md` step in greenfield and brownfield workflows

### Changed
- FACTORY.md documents CLAUDE.md scaffolding in project setup section
- VSDD.md references the new skill in Tooling section

## 0.10.3 — Release infrastructure and CI/CD

### Added
- **Release workflow** (`.github/workflows/release.yml`) — tag-triggered validation + GitHub Release with CHANGELOG excerpt
- **Release config** (`.factory/release-config.yaml`) — declarative release manifest on factory-artifacts branch
- **Release skill rewrite** — config-driven, 3 modes (init/release/dry-run), quality gate spectrum
- Retroactive git tags and GitHub Releases for all 12 prior versions (v0.1.0 through v0.10.2)
- Version field in marketplace.json for release validation
- Factory-artifacts mount step in CI and release workflows

### Changed
- CI workflow renamed from `plugin-validation.yml` to `ci.yml` for cross-repo consistency
- Bump `actions/checkout` from v4 to v6 (Node.js 20 deprecation)

## 0.10.2 — Template path portability fix

Closes a portability hole that would have broken clean installs.

### The bug

Skills and agents referenced templates as `.claude/templates/<name>.md` — a path that only exists inside corverax, where the plugin was originally developed and `.claude/templates/` is pre-populated. A clean install of vsdd-factory into any other project would ship the templates at `plugins/vsdd-factory/templates/` (where they actually live) but every skill referencing `.claude/templates/...` would fail the lookup.

59 references across 24 files were affected:

- 20 skills: `research`, `semport-analyze`, `brownfield-ingest`, `create-brief`, `create-story`, `create-domain-spec`, `create-architecture`, `create-prd`, `adversarial-review`, `holdout-eval`, `state-update`, `record-demo`, `pr-create`, `decompose-stories`, `track-debt`, `convergence-check`, `validate-consistency`, `deliver-story`, `dtu-validate`, `formal-verify`
- 4 agents: `validate-extraction`, `research-agent`, `adversary`, `holdout-evaluator`

### The fix

All 59 references rewritten from `.claude/templates/<name>` to `${CLAUDE_PLUGIN_ROOT}/templates/<name>` — the Claude Code canonical environment variable for the plugin root directory. Agents shell-expand the variable when reading via bash, and the path resolves to the real template location that ships with the plugin regardless of install target.

### Regression guards (3 new tests)

`tests/skills.bats` grew a "Template path portability" section with three tests:

- `no skill references the non-portable .claude/templates/ path` — grep-based regression guard
- `no agent references the non-portable .claude/templates/ path` — same
- `every referenced template actually exists in plugin templates/` — extracts every `${CLAUDE_PLUGIN_ROOT}/templates/<file>` reference from skills and agents, strips the prefix, and asserts the file exists at `plugins/vsdd-factory/templates/<file>`. Caught zero dangling references on first run.

Test suite now **62 tests**, all pass.

### Note for future skill authors

When citing a template in a new skill or agent, use:

```
- `${CLAUDE_PLUGIN_ROOT}/templates/<name>.md` — <description>
```

The `.claude/templates/` prefix is never portable and is now a test failure.

## 0.10.1 — Step-file content fill

Closes the last deferred item from 0.9.0: empty `steps/` placeholder stubs in three skills now carry real per-step playbooks.

### 17 step files expanded (1566 LOC)

The three facilitation / inspection skills (`brainstorming`, `artifact-detection`, `guided-brief-creation`) had `## Step-File Decomposition` tables referencing per-step files that were 3-6 line placeholders. The parent SKILL.md carried the high-level flow; the step files existed only as stubs.

Each step file is now a 58-130 line self-contained playbook the orchestrator can load on demand when executing that specific step. Structure per file:

- **Inputs** — what previous steps produced, files to read, expected state
- **Outputs** — exact artifact paths and formats
- **Procedure** — specific moves, exact elicitation questions (for facilitation skills), exact commands and glob patterns (for inspection skills)
- **Decision points** — branches with criteria, where applicable
- **Failure modes** — step-level failures (distinct from whole-skill failures in parent SKILL.md)
- **Quality gate** — short checklist before advancing
- **Hand-off** — what to pass to the next step

**brainstorming (6 files, 487 LOC):** session setup, technique selection, facilitated ideation, synthesis, direction selection, report writing. Includes exact opening questions, transition phrases, SCAMPER/reverse-brainstorming/mind-mapping/constraint-removal scripts, and the verbatim markdown template for `brainstorming-report.md`.

**artifact-detection (5 files, 510 LOC):** scan, classify, validate, gap analysis, route decision. Includes exact glob patterns per artifact type, explicit validation checklists (rewritten from the SKILL.md prose as iterable rules), DF-020/DF-021 format-migration handling, and verbatim templates for `artifact-inventory.md`, `gap-analysis.md`, and `routing-decision.md`.

**guided-brief-creation (6 files, 569 LOC):** understand intent, contextual discovery, guided elicitation, draft review, adversarial review, finalize. Includes exact section-by-section elicitation questions, research-agent / adversary dispatch criteria, market-intel integration points, and verbatim structures for `product-brief.md` and `elicitation-notes.md`.

### Cross-step dependencies surfaced

Step files make explicit several dependencies that were implicit in the prose:

- **artifact-detection format flags propagate** — format detection in step 1b (FR-NNN vs BC-S.SS.NNN, old vs DF-021-sharded architecture) flows into step 3 validation rules and step 5 routing decisions
- **guided-brief-creation market-intel reference** — `market-intel.md` is read in step 3 and again in step 5 adversarial review for differentiation and risk signals
- **guided-brief-creation adversarial loopback** — step 5 feedback can send the agent back to step 3 (re-elicitation) or step 4 (redraft)

### Meta

- No SKILL.md files modified. Step-file decomposition tables unchanged.
- All 59 tests still pass. No new tests added for step-file content (content is prose, not behavior).
- Full analysis report at `.factory/semport/STEPS-REPORT.md`.

## 0.10.0 — Deferred remediation: commands, hook envelopes, structural tests

Closes out the remaining P1/P2 items deferred from 0.9.0.

### Commands directory (47 files)

Prior versions exposed skills only — many with `disable-model-invocation: true`, which meant users had no slash-command entry point for phase transitions, health checks, or delivery. This release ships `plugins/vsdd-factory/commands/` with **47 thin slash-command wrappers**, one per user-facing skill.

Each command is 15-30 lines: frontmatter (description + optional `argument-hint` mirrored from the skill) and a body that delegates via `Use the <skill-name> skill via the Skill tool`. Commands are entry points; skills remain the source of truth.

Coverage: all Phase 0-6 lifecycle skills (brownfield-ingest, semport-analyze, create-brief through release), cross-cutting ops (factory-health, track-debt, worktree-manage), and UI/design skills (design-system-bootstrap, ui-quality-gate, etc.).

### Hook upgrade: permissionDecision envelopes (POC on spec-steward)

`hooks/protect-vp.sh` and `hooks/protect-bc.sh` now emit `PreToolUse` JSON envelopes with `permissionDecision` + `permissionDecisionReason` instead of bare exit codes. The denial reasons are richer and instruct the agent to create a superseding artifact rather than just blocking the edit.

This is a POC on the two spec-steward hooks. The other hooks (`brownfield-discipline`, `red-gate`, `purity-check`, etc.) still use exit codes. Upgrading them requires per-hook design — deferred until a specific need motivates each one.

Tests updated: the two "blocks edit to green X" tests now assert `status -eq 0` with `permissionDecision:deny` in stdout, replacing the old `status -eq 2` stderr check.

### Structural tests for Iron Laws and Red Flags (18 new tests)

New `tests/skills.bats` enforces that the four discipline skills carry their behavior-shaping scaffolding. A discipline skill missing its Iron Law, "Announce at start" line, or Red Flags table is now a test failure — empirically load-bearing content cannot silently rot.

Test coverage per skill:
- `deliver-story`, `brownfield-ingest`, `adversarial-review`, `wave-gate`: Iron Law token + `## The Iron Law` section + `## Announce at Start` section + `## Red Flags` table with ≥8 rows
- `brownfield-ingest` specifically: Honest Convergence clause, Known Round-1 Hallucination Classes, Subagent Delivery Protocol (`=== FILE:` delimiter), Behavioral vs Metric split, Priority-ordered Lessons mandate
- `validate-extraction` agent: Behavioral vs Metric operating mode with Phase 1 / Phase 2 sections

Total suite: **59 tests** (41 pre-existing + 18 new). All pass.

### Name collision fix

`agents/session-review.md` renamed to `agents/session-reviewer.md` to disambiguate from the `skills/session-review/` directory. Non-breaking — no referring files use the old basename (verified via grep).

### Deferred

Placeholder `steps/` stubs in `brainstorming`, `artifact-detection`, and `guided-brief-creation` skills are still empty. These need real content (not a mechanical fix); tracked for a scoped content PR.

Non-spec-steward hooks remain on exit-code semantics until a per-hook motivation exists for the envelope upgrade.

## 0.9.0 — Self-ingest remediation: apply lessons from claude-code + superpowers

Applies the P0/P1 lessons from running the plugin's own `brownfield-ingest` protocol against `anthropics/claude-code` and `obra/superpowers` in the `.factory/semport/` analysis. The ingest caught 3 round-1 hallucinations via strict-binary novelty, which validated both the protocol and specific gaps in the plugin itself.

### Agent frontmatter remediation (Group A — 46 files)

- **26 agent descriptions rewritten** from the boilerplate stub `VSDD factory agent: <name>` to one-sentence "Use when..." triggers drawn from each agent's body, following superpowers' CSO rule (third-person, when-not-what, <1024 chars).
- **`model:` field added to 28 agents.** Defaults to `sonnet`. Exceptions on `opus`: `adversary`, `holdout-evaluator`, `formal-verifier`, `pr-reviewer`, `spec-reviewer` — terminal reviewers where reasoning quality dominates call volume.
- **`color:` field added to all 33 root agents**, grouped by function: reviewers=red, builders=green, planners=blue, ops=yellow, research=purple.
- **`implementer.md` description** fixed (was truncated mid-sentence).
- **9 `agents/orchestrator/` include files** gained YAML frontmatter with `disable-model-invocation: true` so strict loaders no longer trip on them.
- **`excalidraw-export` and `jira` SKILLs** gained frontmatter (reference-only, `disable-model-invocation: true`).
- **`state-update` skill** marked `disable-model-invocation: true` (internal).

### deliver-story dispatch rewrite (Group B)

`skills/deliver-story/SKILL.md` was a single-context script that quietly drifted from the `agents/orchestrator/per-story-delivery.md` workflow it was supposed to use. Rewritten as a thin dispatcher:

- Declares itself a dispatcher, not an implementer, via `EXTREMELY-IMPORTANT` block.
- Iron Law: `NO IMPLEMENTATION WITHOUT RED GATE VERIFICATION FIRST`.
- Prerequisites check that STOPs on failure (no silent bypass).
- 9-step dispatch sequence: devops-engineer → test-writer (stubs) → test-writer (tests) → **independent Red Gate verification** → implementer → demo-recorder → implementer (push) → pr-manager → devops-engineer (cleanup) → state update.
- Context discipline table naming which files each specialist receives (prevents topic drift from passing whole-story context to every agent).
- Story split recovery flow for oversized PRs.
- 10-row Red Flags table targeting the rationalizations that lead back to single-context execution.
- `agents/orchestrator/per-story-delivery.md` header marked as canonical source.

### brownfield-ingest self-improvements (Group C)

Codifies the 5 lessons the ingest protocol taught itself when applied to real reference repos:

- **Honest Convergence clause** — mandatory verbatim text in every round prompt: "<3 substantive → declare converged, emit no file." Stops agents from fabricating findings under pressure to produce SUBSTANTIVE output.
- **Known Round-1 Hallucination Classes** — 5 named failure modes (over-extrapolated token lists, miscounted enumerations, named pattern conflation, same-basename artifact conflation, inflated/deflated metrics) with verbatim examples from superpowers round 1 (persuasion matrix, Pressure Taxonomy, writing-plans forbidden tokens). Round 2+ prompts must audit round 1 against these classes.
- **Subagent Delivery Protocol (inline-by-default)** — `=== FILE: <name> ===` delimiter pattern that works around sandbox Write denials. Explicit override of subagent default system prompts that forbid "inline fallback."
- **Behavioral vs Metric split** in Phase B.6 — mandatory two-phase validation: Phase 1 samples contracts/entities for CONFIRMED/INACCURATE/HALLUCINATED (judgment); Phase 2 independently recounts every numeric claim via `find` + `wc -l` (arithmetic, not judgment). Empirical anchor: superpowers Pass 0 round 1 claimed 32 files / 5279 LOC; recount showed 23 files / 3859 LOC.
- **Priority-ordered Lessons mandate** in Phase C — synthesis MUST include a `## Lessons for <target-project>` section with P0/P1/P2/P3 buckets, each lesson naming (a) what target does today, (b) what reference does, (c) gap, (d) specific action items with file paths. Makes the synthesis a directly actionable backlog.
- **`agents/validate-extraction.md`** updated with matching operating-mode split and two-table output format.

### Iron Laws and Red Flags rollout (Group D)

Applies superpowers' empirically-anchored behavior-shaping scaffolding to the 4 highest-stakes discipline skills. Iron Laws follow the canonical form `NO <verb> <scope> WITHOUT <prerequisite> FIRST`. Each skill gained an "Announce at Start" verbatim line and a Red Flags table enumerating the rationalizations observed during pressure testing.

- **`deliver-story`** — `NO IMPLEMENTATION WITHOUT RED GATE VERIFICATION FIRST` (+ 10 Red Flags, included in Group B rewrite)
- **`brownfield-ingest`** — `NO ROUND COMPLETION WITHOUT HONEST CONVERGENCE CHECK FIRST` (+ 10 Red Flags)
- **`adversarial-review`** — `NO APPROVAL WITHOUT FRESH-CONTEXT REVIEW FIRST` (+ 8 Red Flags targeting information-asymmetry violations)
- **`wave-gate`** — `NO WAVE ADVANCE WITHOUT ALL SIX GATES PASSING FIRST` (+ 8 Red Flags targeting threshold rounding, gate skipping, flake handling)

### AGENT-SOUL pragmatism footnote (Group E)

`docs/AGENT-SOUL.md` §8 "Pragmatism Over Ceremony" gained a footnote distinguishing **principled pragmatism** (design-time, human-in-loop, ROI-reasoning, documented) from **rationalization** (execution-time, bypass-a-rule). References superpowers' Pressure Taxonomy and the Meincke 2025 empirical anchor (N=28000, compliance 33%→72% under persuasion pressure) — which names "I'm just being pragmatic" as a first-class attack vector on discipline skills. This is the principle most easily weaponized to justify skipping Iron Laws; the footnote exists to stop that.

### Meta

- Reference analysis artifacts live in `.factory/semport/claude-code/` and `.factory/semport/superpowers/` (Phase A + B + C complete, validated).
- `TAKEAWAYS.md` and `PLUGIN-INVENTORY.md` in `.factory/semport/` document the analysis → remediation trace.
- No behavior changes to hooks, workflows, or bin helpers.
- No new tests yet — Group F bookkeeping only. Test coverage for the new Iron Law / Red Flags content is deferred.

## 0.8.0 — Wave 7: Validation infrastructure

Ships the test harness that validates the enforcement layer actually works. Previously, Wave 4's hooks and Wave 5/6's bin helpers had only smoke tests ("does it run without crashing"). Wave 7 adds allow/block path coverage.

- **41 TAP tests** across two bats files:
  - `tests/hooks.bats` (28 tests) — allow and block paths for every hook: brownfield-discipline (4), protect-vp (4), protect-bc (3), red-gate (6), purity-check (3), handoff-validator (3), regression-gate (3), session-learning (2)
  - `tests/bin.bats` (13 tests) — lobster-parse (3, including all 15 workflow files parse), research-cache (4, round-trip + determinism + normalization), multi-repo-scan (3), wave-state (3)
- **Smoke fixture** `fixtures/smoke-project/` — minimal Rust crate with one passing test, for future hook integration tests
- **Test runner** `tests/run-all.sh` — syntax checks + hook tests + bin tests; tool-guarded per `bash.md`
- **GitHub Actions CI** `.github/workflows/plugin-validation.yml` — runs on push/PR to main: installs bats/jq/yq, syntax-checks shell scripts, runs both test suites, validates all JSON manifests, parses every workflow file

All 41 tests pass on first run locally.

## 0.7.0 — Wave 6: Runtime helpers and not-portable documentation

Finishes the runtime-extension port. Ships bin helpers for the extensions that map to bash+jq+yq, wraps them in skills, and documents the four that cannot be ported.

**New bin helpers** (`plugins/vsdd-factory/bin/`):

- `research-cache` — SHA-keyed disk cache for Perplexity/Context7 query results at `.factory/research-cache/`. Subcommands: `get`, `put`, `has`, `key`, `clear`, `stats`. Ports `research-cache.ts`.
- `wave-state` — read-only query of `.factory/stories/sprint-state.yaml`. Subcommands: `current`, `stories`, `ready`, `summary`. Read-only slice of `wave-orchestrator.ts`.
- `multi-repo-scan` — detects multi-repo layouts from `.worktrees/`, reports repos with manifest types. Read-only slice of `multi-repo-orchestrator.ts`.

**New skill wrappers**:

- `research-cache-ops` — operates the research cache from within a session
- `wave-status` — reports wave readiness with recommendations
- `multi-repo-health` — detects multi-repo layouts and cross-checks against `.factory/stories/`

**Not-portable documentation** (`docs/not-portable.md`):

Documents why four dark-factory extensions cannot port to Claude Code's plugin primitives:

- `cost-tracker.ts` — no `PreModelCall` hook
- `attention-heatmap.ts` — no read-event hooks
- `tiered-context.ts` — Claude Code manages context natively
- `sidecar-learning.ts` (full synthesis) — `Stop` hook has no transcript access; partial marker-only port shipped in Wave 4

All bin helpers follow `bash.md`: `set -euo pipefail`, stderr guards, STDERR-EXEMPT tags, tool availability checks. Pass `bash -n` syntax checks and basic smoke tests.

Total skills: 91. Total bin helpers: 4.

## 0.6.0 — Wave 5: Orchestrator + workflow data (Lobster replacement)

Replaces dark-factory's Lobster workflow DSL with "Lobster-as-data" driven by the orchestrator agent and a bash helper.

- **Workflow corpus** — shipped all 15 `.lobster` files as data under `plugins/vsdd-factory/workflows/`:
  - Mode workflows: greenfield, brownfield, feature, maintenance, discovery, planning, multi-repo, code-delivery
  - Phase sub-workflows: phase-0-codebase-ingestion, phase-1-spec-crystallization, phase-3-test-first-implementation, phase-3.5-holdout-evaluation, phase-4-adversarial-refinement, phase-5-formal-hardening, phase-6-convergence
- **`bin/lobster-parse`** — bash helper wrapping `yq` + `jq` that emits workflow files as JSON with optional jq expressions. Lobster files parse cleanly as YAML.
- **Orchestrator agent updated** — added a Workflow Data section that points at the `workflows/` corpus and documents the lobster-parse helper with worked examples.
- **Five new skills** in `skills/`:
  - `run-phase` — execute a phase by reading its Lobster file and spawning declared sub-agents in dependency order
  - `next-step` — read `.factory/STATE.md` + active workflow, propose next action (does not execute)
  - `validate-workflow` — static schema check: required fields, agent/skill existence, depends_on graph, cycles, duplicate step names
  - `activate` — per-project opt-in that writes `{"agent": "vsdd-factory:orchestrator"}` to `.claude/settings.local.json`
  - `deactivate` — removes the agent override; leaves plugin enabled

Opt-in design (vs hijacking default persona on plugin enable) chosen per earlier decision — activation is always an explicit user action, per-project.

Total skills: 88.

## 0.5.0 — Wave 4: Enforcement layer (hooks)

Ported dark-factory's OpenClaw runtime extensions to Claude Code hooks. This is the "make the wrong thing impossible" wave — recovering the enforcement discipline that was missing from the initial extract.

**New hooks** (in `plugins/vsdd-factory/hooks/`):

- `brownfield-discipline.sh` (PreToolUse) — blocks edits to `.reference/**`
- `protect-bc.sh` (PreToolUse) — blocks edits to green Behavioral Contracts
- `red-gate.sh` (PreToolUse) — enforces TDD red-before-green when `.factory/red-gate-state.json` declares strict mode; opt-in per project
- `purity-check.sh` (PostToolUse, warn) — flags side-effect patterns in pure-core paths (`*/pure/**`, `*/core/**`, `*_pure.rs`, `*.pure.ts`, `*/kernel/*`)
- `regression-gate.sh` (PostToolUse) — records cargo/pytest/npm/go test outcomes to `.factory/regression-state.json`, warns on pass→fail transitions
- `handoff-validator.sh` (SubagentStop) — warns on empty/truncated subagent output
- `session-learning.sh` (Stop) — appends session-end markers to `.factory/sidecar-learning.md`

**Wired existing hooks**:

- `protect-vp.sh` (PreToolUse, Edit|Write) — already shipped, now registered
- `verify-git-push.sh` (PreToolUse, Bash) — registered
- `check-factory-commit.sh` (PreToolUse, Bash) — registered

All hooks follow `.claude/rules/bash.md`: `set -euo pipefail`, jq-based JSON parsing with stderr guards, no `eval`, tool availability checks, STDERR-EXEMPT tags where stderr suppression is intentional. All 10 hooks pass `bash -n` syntax checks and basic smoke tests.

**Not portable** (needs API-level integration Claude Code doesn't expose):

- Cost tracker, attention heatmap, tiered-context enforcement, full sidecar-learning synthesis — will ship as doc stubs in Wave 6.

## 0.4.0 — Wave 3: Design system, UX, and market intelligence

- Ported 13 skills for UI-heavy projects and product-intelligence workflows
- **Design & UX:** `design-drift-detection`, `design-system-bootstrap`, `multi-variant-design`, `storybook-mcp-integration`, `responsive-validation`, `ui-completeness-check`, `ui-quality-gate`, `ux-heuristic-evaluation`
- **Market & customer:** `competitive-monitoring`, `customer-feedback-ingestion`, `intelligence-synthesis`, `market-intelligence-assessment`, `analytics-integration`
- `templates/design-system/` already present from initial extraction

Total skills: 83.

## 0.3.0 — Wave 2: Skill coverage (feature-mode + maintenance)

- Ported 39 skills from dark-factory workflow catalogue
- **Feature-mode (F1–F7):** `phase-f1-delta-analysis`, `phase-f2-spec-evolution`, `phase-f3-incremental-stories`, `phase-f4-delta-implementation`, `phase-f5-scoped-adversarial`, `phase-f6-targeted-hardening`, `phase-f7-delta-convergence`
- **Maintenance & discovery:** `maintenance-sweep`, `discovery-engine`, `planning-research`, `post-feature-validation`, `pr-review-triage`, `fix-pr-delivery`
- **Mode routing:** `mode-decision-guide`, `quick-dev-routing`, `feature-mode-scoping-rules`, `implementation-readiness`, `validate-brief`
- **Infrastructure:** `model-routing`, `repo-initialization`, `toolchain-provisioning`, `wave-scheduling`, `spec-versioning`, `traceability-extension`, `sdk-generation`
- **Session & consistency:** `consistency-validation`, `convergence-tracking`, `artifact-detection`, `phase-1-prd-revision`, `phase-1d-adversarial-spec-review`, `multi-repo-phase-0-synthesis`, `factory-worktree-health`, `dtu-creation`, `brainstorming`, `agent-file-review`, `code-delivery`, `demo-recording`, `session-review`, `guided-brief-creation`
- Replaced Corverax's `release` skill with dark-factory's authoritative version per merge rules

Total skills: 70 (was 31).

## 0.2.0 — Wave 1: Foundation

- Shipped `docs/VSDD.md`, `docs/FACTORY.md`, `docs/CONVERGENCE.md` — the methodology documents
- Shipped `docs/AGENT-SOUL.md` — shared engine-wide agent principles
- Ported 28 dark-factory agents into single-file triune format (`## Identity` + `## Operating Procedure`) with synthesized frontmatter:
  `accessibility-auditor, architect, business-analyst, code-reviewer, consistency-validator, data-engineer, demo-recorder, devops-engineer, dtu-validator, dx-engineer, e2e-tester, formal-verifier, github-ops, implementer, performance-engineer, pr-manager, pr-reviewer, product-owner, security-reviewer, session-review, spec-reviewer, spec-steward, state-manager, story-writer, technical-writer, test-writer, ux-designer, visual-reviewer`
- Shipped `agents/orchestrator/` as a directory containing `orchestrator.md` plus 9 mode-sequence sub-files (greenfield, brownfield, feature, maintenance, discovery, multi-repo, per-story-delivery, steady-state, heartbeat)
- Kept Corverax's enhanced versions of the 5 overlapping agents (adversary, codebase-analyzer, holdout-evaluator, research-agent, validate-extraction) unchanged

Total agents: 34 (33 dark-factory + 1 Corverax addition).

## 0.1.0 — Initial marketplace

- Extracted Corverax `.claude/` VSDD pipeline into a shareable plugin marketplace
- 5 agents, 31 skills, 3 hooks, rules, templates
