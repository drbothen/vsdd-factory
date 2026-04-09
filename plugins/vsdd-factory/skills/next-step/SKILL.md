---
name: next-step
description: Read `.factory/STATE.md` and the active workflow, then propose the next Lobster step to execute. Use when resuming work on an in-flight pipeline.
---

# Next Step

Answers the question "what should I do now?" by cross-referencing pipeline state with workflow data.

## Procedure

1. **Check pipeline state.** Read `.factory/STATE.md`. If it doesn't exist, tell the user to run `/vsdd-factory:factory-health` and stop.

2. **Determine current mode and phase.** STATE.md should declare `mode:` (greenfield/brownfield/feature/maintenance) and `phase:` (0–6 or F1–F7). If either is missing, ask the user.

3. **Resolve the workflow file.**
   - Mode-level next step: `${CLAUDE_PLUGIN_ROOT}/workflows/<mode>.lobster`
   - Phase-level next step: `${CLAUDE_PLUGIN_ROOT}/workflows/phases/<phase>.lobster`

4. **Enumerate completed steps from STATE.md.** Collect step names already marked done.

5. **Find the first uncompleted step whose `depends_on` are all satisfied.** Use:
   ```bash
   ${CLAUDE_PLUGIN_ROOT}/bin/lobster-parse <file> '.workflow.steps[] | {name, agent, depends_on, task}'
   ```

6. **Report the proposal.** Output:
   - Workflow file
   - Next step name
   - Declared agent (or skill/command)
   - The step's `task` text
   - Any warnings about unsatisfied dependencies or ambiguous state

7. **Do not execute.** This skill only proposes. The user decides whether to run `/vsdd-factory:run-phase` or invoke the step manually.

## See also

- `/vsdd-factory:run-phase` — actually executes a phase
- `/vsdd-factory:factory-health` — diagnoses STATE.md and worktree issues
