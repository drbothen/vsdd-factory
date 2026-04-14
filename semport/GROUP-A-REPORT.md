# Group A Remediation Report

## Summary

Total files modified: **46**

- 33 root agents (`agents/*.md`)
- 9 orchestrator includes (`agents/orchestrator/*.md`, excluding `orchestrator.md`)
- 2 reference-only skills (excalidraw-export, jira)
- 2 skill frontmatter tweaks (state-update, plus A7 verified as no-op)

## Per-task summary

### A1 — Stub description rewrites (22 agents)

Rewrote boilerplate `VSDD factory agent: <name>` or truncated wrapped descriptions into single-sentence "Use when..." descriptions drawn from each agent body:

architect, business-analyst, code-reviewer, data-engineer, demo-recorder, devops-engineer, dtu-validator, dx-engineer, formal-verifier, github-ops, pr-manager, pr-reviewer, product-owner, security-reviewer, spec-reviewer, spec-steward, state-manager, story-writer, technical-writer, test-writer (20 pure stubs).

Plus these which had wrapped/truncated multi-line stubs rewritten as single sentences: accessibility-auditor, consistency-validator, e2e-tester, implementer, performance-engineer, visual-reviewer (6). ux-designer and session-review already had complete descriptions and were left intact (only model/color added).

Total rewritten: **26** (slightly more than the 22 projected because wrapped-description stubs also needed one-sentence rewrites).

### A2 — Add `model:` field (33 agents)

Added `model: sonnet` to 27 agents that lacked it. Added `model: opus` to formal-verifier, pr-reviewer, and spec-reviewer.

Left alone (already had model): adversary (sonnet), holdout-evaluator (sonnet), codebase-analyzer (opus), research-agent (opus), validate-extraction (sonnet). Note: adversary and holdout-evaluator were listed as opus exceptions in the spec but the instruction "If model: already exists, leave it" took precedence, so their existing `sonnet` value was preserved.

### A3 — Add `color:` field (33 agents)

Added per functional group:
- **red** (reviewers, 13): adversary, holdout-evaluator, spec-reviewer, pr-reviewer, code-reviewer, security-reviewer, visual-reviewer, consistency-validator, dtu-validator, validate-extraction, accessibility-auditor, formal-verifier, session-review
- **green** (builders, 6): implementer, test-writer, story-writer, architect, data-engineer, codebase-analyzer
- **blue** (planners, 7): product-owner, business-analyst, demo-recorder, technical-writer, ux-designer, dx-engineer, e2e-tester
- **yellow** (ops, 6): devops-engineer, github-ops, pr-manager, state-manager, spec-steward, performance-engineer
- **purple** (research, 1): research-agent

### A4 — implementer.md truncated description

Rewrote `Strict TDD implementation agent. Picks next failing test, writes minimum code` (truncated) into a single complete "Use when..." sentence.

### A5 — Orchestrator include frontmatter (9 files)

Added YAML frontmatter with `name: orchestrator-<base>`, a one-sentence description, and `disable-model-invocation: true` to:

brownfield-sequence, discovery-sequence, feature-sequence, greenfield-sequence, HEARTBEAT, maintenance-sequence, multi-repo, per-story-delivery, steady-state.

`orchestrator.md` not touched (already has frontmatter).

### A6 — Reference-only skill frontmatter (2 files)

Added full frontmatter (`name`, `description`, `disable-model-invocation: true`) to `skills/excalidraw-export/SKILL.md` and `skills/jira/SKILL.md`.

### A7 — Empty `allowed-tools` (NO-OP)

None of the four listed skills actually have an empty `allowed-tools:` field. Confirmed via grep:
- `generate-pdf`: uses `tools: Read, Bash` (not `allowed-tools`) — populated.
- `holdout-eval`, `research`, `spec-drift`: no `allowed-tools` field at all (agent-delegating fork-context skills).

No changes made. If the intent was to populate `tools:`/add `allowed-tools:` to these, clarification is needed — they are currently spawning agents via `agent:` frontmatter and don't need file-tools for the dispatch step itself.

### A8 — state-update disable-model-invocation

Added `disable-model-invocation: true` to `skills/state-update/SKILL.md` frontmatter (already had `user-invocable: false`).

## Surprises / notes

1. **Agents with already-good descriptions** (left as-is on description, only added model/color): adversary, codebase-analyzer, holdout-evaluator, research-agent, validate-extraction, ux-designer, session-review.

2. **Wrapped-description stubs**: 6 agents (accessibility-auditor, consistency-validator, e2e-tester, implementer, performance-engineer, visual-reviewer) had descriptions that wrapped onto a second line, then got cut off by the frontmatter `---`. These were all rewritten to single sentences.

3. **Adversary/holdout-evaluator model conflict**: The spec called these out as opus exceptions, but the "leave existing model alone" rule took precedence. If they should actually be opus, a follow-up edit is needed.

4. **A7 no-op**: The 4 named skills don't have empty allowed-tools. Nothing to fix.

5. **generate-pdf** uses the legacy `tools:` key instead of `allowed-tools:`. Not touched per scope.
