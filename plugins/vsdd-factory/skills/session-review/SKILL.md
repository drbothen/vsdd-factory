---
name: session-review
description: >
  Post-pipeline analysis that reviews the complete factory run and produces
  improvement proposals. Runs after every pipeline completion (any path).
  Analyzes cost, timing, convergence, agent behavior, gate outcomes, wall
  integrity, and produces actionable improvement recommendations.
agents:
  primary: session-review
  supporting: [state-manager]
inputs:
  - .factory/STATE.md (complete run history)
  - .factory/cost-summary.md
  - All phase artifacts
  - All convergence logs
  - All adversarial findings
  - All PR review histories
  - All holdout results
outputs:
  - .factory/session-reviews/review-YYYY-MM-DD-[run-id].md
  - .factory/session-reviews/improvement-proposals-[run-id].md
---

# Session Review Skill

## Purpose

Post-pipeline analysis that reviews the complete factory run and produces
improvement proposals for the factory itself. This skill runs after every
pipeline completion (any path) and is the mechanism by which the factory
learns from every run.

## Trigger

This skill is invoked as the final step in every top-level Lobster workflow,
after all pipeline work is complete (including release), but before the
pipeline is marked as DONE in STATE.md.

## Procedure

```
PIPELINE COMPLETES (any path)
  |
  v
session-review agent (T1, adversary model):
  -> read STATE.md (complete run history)
  -> read cost-summary.md
  -> read all phase artifacts, convergence logs, adversary findings
  -> read PR review histories, holdout results
  -> read pattern-database.yaml (prior patterns)
  -> read benchmarks.yaml (running averages)
  -> analyze 8 dimensions
  -> produce review report + improvement proposals
  -> return findings to orchestrator
  |
  v
state-manager (T2):
  -> write review report to .factory/session-reviews/
  -> write improvement proposals
  -> commit to factory-artifacts
  |
  v
HUMAN REVIEW (72h timeout)
  -> approve / defer / reject each proposal
  |
  v
state-manager (T2):
  -> process decisions
  -> update improvement-backlog.md
  -> update pattern-database.yaml
  -> update benchmarks.yaml
  -> mark STATE.md as COMPLETE
```

## Analysis Dimensions

The session-review agent analyzes 8 dimensions:

1. **Cost Analysis** — total vs budget, per phase/wave/story, agent costs, efficiency
2. **Timing Analysis** — wall clock, bottlenecks, timeouts, parallelization, human wait
3. **Convergence Analysis** — adversary rounds, PR rounds, hardening, trends
4. **Agent Behavior Analysis** — tier compliance, template adherence, scope creep
5. **Gate Outcome Analysis** — first-try pass rate, human overrides, phase skips
6. **Wall Integrity Analysis** — information asymmetry enforcement, leak detection
7. **Quality Signal Analysis** — holdout scores, mutation rates, fuzz crashes, security
8. **Pattern Detection** — cross-run comparison, recurring issues, trends

## Proposal Categories

| Category | Examples | Affects |
|----------|---------|---------|
| **cost** | "Hardening used Opus where Sonnet would suffice for 3 checks" | Model config, agent assignments |
| **timing** | "Story decomposition human approval took 18h — consider async notification" | Notification config, gate timeouts |
| **convergence** | "Adversary produced 12 findings in round 1, 11 cosmetic — tune prompt" | Adversary prompt, convergence thresholds |
| **agent** | "Orchestrator attempted Write 3 times — T1 enforcement caught it" | Agent AGENTS.md, OpenClaw tool config |
| **gate** | "Build verification gate failed 4 times due to flaky test" | Test suite, gate criteria |
| **wall** | "PR reviewer referenced .factory/adversary findings — wall leak" | Lobster context.exclude config |
| **quality** | "Mutation kill rate 87% on module X — below 90% target" | Test coverage targets, test generation prompts |
| **pattern** | "Last 3 runs all had >5 adversary rounds in adversarial refinement" | Adversary system prompt, convergence criteria |
| **workflow** | "Steps X and Y could run in parallel — no dependency" | Lobster workflow file |
| **template** | "Agent consistently deviated from template section 3" | Template file |

## Proposal Lifecycle

```
session-review produces proposal
  -> state-manager writes to .factory/session-reviews/improvement-proposals-[run-id].md
  -> human reviews:
    APPROVE -> create story/task for implementation
    DEFER -> add to .factory/session-reviews/improvement-backlog.md with rationale
    REJECT -> log rejection reason for future pattern detection
```

## Proposal Routing (Approved)

| Proposal Category | Routes To |
|-------------------|-----------|
| **workflow** | Story in dark-factory repo (factory improves itself) |
| **agent** | Agent prompt update task |
| **template** | Template update task |
| **cost** | Config change (model assignments, timeouts) |
| **wall** | Lobster context.exclude update |
| **gate** | Gate threshold update |
| **convergence** | Convergence criteria update |
| **quality** | Test target adjustment |
| **pattern** | May spawn research task to investigate root cause |

## Non-Blocking Completion

If the human does not respond to the session review within 72h, all proposals
auto-defer to the improvement backlog. The pipeline is marked COMPLETE regardless.
Session review should never block pipeline completion indefinitely.

## Cross-Run Pattern Database

The skill maintains a pattern database in `.factory/session-reviews/` that
accumulates across runs:

```
.factory/session-reviews/
  review-YYYY-MM-DD-[run-id].md          # Per-run review
  improvement-proposals-[run-id].md       # Per-run proposals
  improvement-backlog.md                  # Deferred proposals
  pattern-database.yaml                   # Cross-run pattern tracking
  benchmarks.yaml                         # Running averages for comparison
```

## Self-Cost Awareness

Track session review cost separately in cost-summary.md. If session review
cost exceeds 5% of the pipeline run cost, flag for optimization.

## Failure Modes

- If session logs are incomplete (missing phases, truncated STATE.md): analyze what is available and document explicit gaps in the review report
- If no cost data is available (cost-summary.md missing or empty): skip the cost analysis dimension and note the gap
- If pattern-database.yaml is corrupted or missing: start fresh pattern tracking for this run and flag for recovery
