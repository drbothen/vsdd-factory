---
name: deactivate
description: Reverse `/vsdd-factory:activate` — remove the orchestrator default agent from `.claude/settings.local.json`. Leaves the plugin enabled; only the default persona changes.
---

# Deactivate VSDD Factory

Removes the per-project default-agent override written by `/vsdd-factory:activate`. The plugin itself stays enabled — agents, skills, and hooks remain available for explicit invocation.

## Procedure

1. **Read `.claude/settings.local.json`.** If it doesn't exist, say so and stop.

2. **Check whether the agent default is set to an `vsdd-factory:` agent.** If it points at something else, stop and warn — don't clobber unrelated config.

3. **Remove the key.** Use `jq 'del(.agent)'` and write the file back. If the resulting object is empty, either delete the file or leave it as `{}` — ask the user which.

4. **Confirm.** Print the remaining contents of `settings.local.json` and a reminder that the plugin is still enabled.

## See also

- `/vsdd-factory:activate` — the inverse
