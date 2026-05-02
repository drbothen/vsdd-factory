---
name: run-phase
description: Execute a VSDD phase by reading its Lobster workflow file and spawning the declared sub-agents in dependency order. Invoke as `/vsdd-factory:run-phase <phase-id>`.
argument-hint: "<phase-id|mode-name>"
---

# Run Phase

Drives a single phase from its `.lobster` workflow file. This is the executable counterpart to the orchestrator agent — use it when you want to run one phase without activating the full orchestrator persona.

## Arguments

`$ARGUMENTS` — the phase identifier. One of:

- `phase-0-codebase-ingestion`
- `phase-1-spec-crystallization`
- `phase-2-story-decomposition`
- `phase-3-tdd-implementation`
- `phase-4-holdout-evaluation`
- `phase-5-adversarial-refinement`
- `phase-6-formal-hardening`
- `phase-7-convergence`

Or a mode workflow: `greenfield`, `brownfield`, `feature`, `maintenance`, `discovery`, `planning`, `multi-repo`, `code-delivery`.

## Procedure

1. **Resolve the workflow file.** If `$ARGUMENTS` starts with `phase-`, it lives in `${CLAUDE_PLUGIN_ROOT}/workflows/phases/$ARGUMENTS.lobster`. Otherwise it lives in `${CLAUDE_PLUGIN_ROOT}/workflows/$ARGUMENTS.lobster`. Fail with an actionable message if the file doesn't exist.

2. **Validate the workflow.** Run:
   ```bash
   ${CLAUDE_PLUGIN_ROOT}/bin/lobster-parse <file> '.workflow | {name, version, steps: (.steps | length)}'
   ```
   Confirm the result has a `name` and at least one step.

3. **Enumerate steps in dependency order.** Extract the step list:
   ```bash
   ${CLAUDE_PLUGIN_ROOT}/bin/lobster-parse <file> '.workflow.steps[] | {name, type, agent, depends_on, task}'
   ```
   For each step, resolve `depends_on` and execute topologically.

4. **Execute each step.**
   - If `type: agent`, spawn the declared `agent` via the Task tool with the step's `task` as the prompt.
   - If `type: skill`, invoke the named skill via the Skill tool.
   - If `type: command`, run the declared command via Bash.
   - On failure, honor `on_failure` (`escalate` | `retry` | `skip`) and `max_retries`.

5. **Update `.factory/STATE.md`** after each step. Append a line with timestamp, phase, step name, and outcome. If `STATE.md` doesn't exist, stop and tell the user to run `/vsdd-factory:factory-health` first.

6. **Report at the end.** Print a compact summary: phase name, step count, passes, failures, elapsed wall time.

## Non-goals

- Do not edit the workflow file itself. It's data, not a working document.
- Do not invent steps. If a step's `task` is ambiguous, stop and ask.
- Do not skip `depends_on` — that ordering exists for correctness, not performance.

## See also

- Orchestrator agent: `${CLAUDE_PLUGIN_ROOT}/agents/orchestrator/orchestrator.md`
- Lobster helper: `${CLAUDE_PLUGIN_ROOT}/bin/lobster-parse`
- Next-step helper: `/vsdd-factory:next-step`
