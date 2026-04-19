---
name: phase-0-codebase-ingestion
description: Phase 0 entry point — analyze existing codebase using brownfield-ingest broad-then-converge protocol. Delegates to phase sub-workflow.
---

# Phase 0: Codebase Ingestion

Phase entry point for brownfield projects. Analyzes an existing codebase to produce a complete semantic understanding that feeds into spec crystallization.

## Sub-Workflow

Execute the steps defined in:
```
${CLAUDE_PLUGIN_ROOT}/workflows/phases/phase-0-codebase-ingestion.lobster
```

## Steps

| Step | File | What It Does |
|------|------|-------------|
| A | `brownfield-ingest/steps/step-a-source-acquisition.md` | Clone/copy codebase to `.reference/` |
| B | `brownfield-ingest/steps/step-b-broad-sweep.md` | 7 broad analysis passes |
| C | `brownfield-ingest/steps/step-c-convergence-deepening.md` | Iterative deepening until NITPICK |
| D | `brownfield-ingest/steps/step-d-coverage-audit.md` | Grep-driven coverage verification |
| E | `brownfield-ingest/steps/step-e-extraction-validation.md` | Behavioral + metric accuracy check |
| F | `brownfield-ingest/steps/step-f-final-synthesis.md` | Definitive synthesis with lessons |

## Work Skill

Direct command: `/vsdd-factory:brownfield-ingest <path>`

## Prerequisites

- Target codebase accessible (local path or Git URL)
- `.factory/` worktree mounted on `factory-artifacts` branch

## Gate Criteria

- Project context document exists
- Module criticality classification exists
- Behavioral contracts extracted with `origin: recovered`
- Coverage audit PASS
- Extraction validation PASS
- Input-hash drift check clean
- Human approval
