---
name: orchestrator-brownfield-sequence
description: Orchestrator workflow reference for the brownfield ingestion and analysis pipeline. Loaded by the orchestrator agent during the corresponding phase. Not directly invokable.
disable-model-invocation: true
---

> **Global Operating Rules:** Read `../../docs/FACTORY.md` and `../../docs/VSDD.md` for factory-wide constraints.


# Brownfield Delegation Sequence

Reference file for the orchestrator. Load when running a brownfield pipeline.

## Overview

Brownfield adds Phase 0 (Codebase Ingestion) before the standard greenfield
pipeline. Phase 0 analyzes the existing code, recovers architecture, extracts
conventions, and builds a project context that scopes all subsequent phases
to NEW features only.

## Environment Setup

1. Spawn dx-engineer: "Install tools, LLM health check (all 3 model families),
   MCP preflight, supply chain audit. Block on any security finding."

## Repo Verification (NOT creation — repo already exists)

2. Spawn devops-engineer: "Verify existing repo infrastructure:
   - Verify git remote exists and is accessible
   - Verify default branch is set
   - Check/add CI/CD workflows if missing
   - Check/add branch protection if missing
   - Create .factory/ worktree on factory-artifacts branch
   - Enable git rerere, add merge-config.yaml if missing
   IMPORTANT: Preserve all existing repo configuration. No destructive changes."

## Factory Worktree Health Check (BLOCKING)

3. Spawn devops-engineer with factory-worktree-health skill
4. Do NOT proceed until worktree health passes

## Phase 0: Codebase Ingestion

Run via phase-0-codebase-ingestion skill. The 12 substeps:

| Step | Agent | Task | Backup |
|------|-------|------|--------|
| 0a | codebase-analyzer | Project discovery: languages, frameworks, deps, entry points | Commit |
| 0b | architect | Architecture recovery: module boundaries, dependency graph, patterns | Commit |
| 0c | codebase-analyzer | Convention extraction: naming, error handling, testing patterns | Commit |
| 0d | product-owner | Spec reverse-engineering: extract behavioral contracts (BC-S.SS.NNN, origin: recovered) | Commit |
| 0d+ | spec-steward | Review recovered BCs for completeness | Commit |
| 0e | formal-verifier | Verification gap analysis: what's tested, what's not, coverage map | Commit |
| 0e-sec | security-reviewer | Security & dependency audit (BLOCKING — critical findings halt pipeline) | Commit |
| 0e.5 | architect | Module criticality classification | Commit |
| 0f-pre | product-owner | Holdout scenario seeding from recovered BCs | Commit |
| 0f | business-analyst | Project context synthesis: combine all Phase 0 outputs | Commit |
| 0f-adv | adversary | Adversarial review of Phase 0 artifacts (information wall applies) | Commit |
| 0f-post | consistency-validator | Consistency validation + fix loop | Commit |

Each step commits artifacts to factory-artifacts branch before proceeding.

## Phase 0 Gate

Verify all Phase 0 artifacts exist:
- `.factory/phase-0-ingestion/project-context.md`
- `.factory/phase-0-ingestion/recovered-architecture.md`
- `.factory/phase-0-ingestion/conventions.md`
- `.factory/specs/behavioral-contracts/` (BC-S.SS.NNN with origin: recovered)
- `.factory/phase-0-ingestion/verification-gap-analysis.md`
- `.factory/phase-0-ingestion/security-audit.md`
- `.factory/specs/module-criticality.md`
- `.factory/phase-0-ingestion/adversarial-review-0.md`
- `.factory/phase-0-ingestion/validation-report.md`
- `.factory/holdout-scenarios/` (seeded from recovered BCs)

HUMAN APPROVAL required.

## Post-Phase 0 Routing

Ask the human:
1. "I have a feature in mind" -> continue to market intelligence + Phase 1
2. "Help me figure out what to build" -> route to planning.lobster (Path 5)
3. "Nothing right now" -> park (STATE.md: phase: 0-complete, awaiting: feature-request)

When human returns later with a feature request, orchestrator detects
STATE.md has phase: 0-complete and routes to Phase 1 with Phase 0 context.

## Market Intelligence (if feature chosen)

Spawn business-analyst + research-agent: "Market intelligence for proposed feature,
scoped to existing product context from Phase 0."
HUMAN reviews GO/CAUTION/STOP.

## Cross-Language Porting Detection

Runs after Phase 0 gate passes, before Phase 1 starts.

Spawn architect: "Determine if human requested language migration. Check for
explicit porting signals in the feature request (e.g., 'rewrite in Rust',
'port the API from Python', 'migrate to Go')."

If porting detected:
1. Spawn codebase-analyzer: "Semport Phase 1 — semantic analysis of source modules
   targeted for porting. Write to <project-path>/.factory/semport/"
2. Spawn architect: "Semport Phase 2 — target language design, purity boundary
   mapping, idiomatic construct translation plan"
3. Spawn implementer: "Semport Phase 3 — initial translation following target design"
4. Spawn implementer: "Semport Phase 4 — syntax fix and idiomatic refinement"
5. Spawn test-writer: "Semport Phase 5 — behavioral equivalence tests comparing
   source and target implementations"
6. HUMAN reviews Semport output before proceeding

Semport artifacts in `.factory/semport/` become reference context for Phase 1 —
the architect uses the translated code when designing the target architecture,
and the product-owner references it when writing behavioral contracts.

See `workflows/skills/semport/SKILL.md` for the full Semport pipeline.

If no porting detected: skip this section entirely.

## Design System Extraction (UI products only)

Spawn codebase-analyzer + ux-designer + accessibility-auditor:
"Extract design system from existing codebase — tokens, contracts, constraints."
HUMAN approves design system.

## Transition to Greenfield Phases 1-6

Spawn state-manager: "Prepare Phase 1 context from Phase 0 output. Ensure all
Phase 0 artifacts are committed and accessible."

Then run the standard greenfield pipeline (Phase 1-6) from `greenfield-sequence.md`,
with Phase 0 context available to all agents.
