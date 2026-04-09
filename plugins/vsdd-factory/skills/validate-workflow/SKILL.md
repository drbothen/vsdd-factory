---
name: validate-workflow
description: Schema-check a Lobster workflow file. Confirms required fields, depends_on resolution, and agent/skill references are valid. Invoke as `/vsdd-factory:validate-workflow <file>`.
---

# Validate Workflow

Static checker for `.lobster` files. Run before committing workflow changes or when debugging a pipeline that fails to drive correctly.

## Arguments

`$ARGUMENTS` — path to a `.lobster` file, relative to the plugin root or absolute.

## Checks

For each step in the workflow:

1. **Required fields.** `name`, `type` (one of `agent`/`skill`/`command`), and `task` must be present.
2. **Agent existence.** If `type: agent`, confirm `${CLAUDE_PLUGIN_ROOT}/agents/<agent>.md` or `${CLAUDE_PLUGIN_ROOT}/agents/<agent>/<agent>.md` exists.
3. **Skill existence.** If `type: skill`, confirm `${CLAUDE_PLUGIN_ROOT}/skills/<skill>/SKILL.md` exists.
4. **Dependency graph.** Every `depends_on` entry must name an earlier step. No cycles, no forward references, no dangling references.
5. **Duplicate step names.** Fail if two steps share the same `name`.
6. **Top-level fields.** `workflow.name`, `workflow.version`, `workflow.steps` must exist.

## Procedure

1. Parse the file:
   ```bash
   ${CLAUDE_PLUGIN_ROOT}/bin/lobster-parse <file> '.'
   ```
   If parsing fails, report the yq error and stop.

2. Walk the step list, collecting errors. Do not bail on the first error — report all of them.

3. Build a dependency graph and run a topological sort. Report any cycles.

4. For each agent/skill reference, check existence with `ls` against `${CLAUDE_PLUGIN_ROOT}`.

5. **Report.** Compact table: step name, check, status (ok/fail), detail. End with a summary line: `<n> checks, <passes> passed, <fails> failed`.

6. Exit 0 if all checks pass, non-zero otherwise.

## Non-goals

- Do not fix the file. Report problems; the user fixes them.
- Do not execute any step. This is purely static.
