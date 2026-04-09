---
name: activate
description: Opt in to the VSDD factory persona for this project. Writes `.claude/settings.local.json` to set the orchestrator as the default main-thread agent. Reversible via `/vsdd-factory:deactivate`.
---

# Activate VSDD Factory

Per-project opt-in. Enabling the plugin alone does not change your default Claude persona — it only makes the factory's agents, skills, and hooks available. Running this skill flips the default agent to `orchestrator` so that a plain session becomes the VSDD pipeline driver.

## Procedure

1. **Confirm the user is inside a project that wants VSDD.** Check for `.factory/` and `.factory/STATE.md`. If missing, ask whether to continue anyway (you can activate before initializing).

2. **Read existing `.claude/settings.local.json`.** If it doesn't exist, create an empty `{}`. If it does, parse it with `jq`.

3. **Merge the agent default.** Write back the file with:
   ```json
   { "agent": "vsdd-factory:orchestrator" }
   ```
   merged into the existing contents. Preserve all other keys.

4. **Confirm activation.** Print:
   - File written
   - New default agent
   - How to deactivate (`/vsdd-factory:deactivate`)
   - Reminder that this only affects the current project (`.claude/settings.local.json` is per-project)

## Notes

- This is intentionally local, not shared. `settings.local.json` is typically gitignored, so teammates opt in individually.
- Plugin-level `settings.json` (ships with a `{"agent": ...}` default) is the alternative "hijack-on-enable" approach — we deliberately did not choose that. Activation is always an explicit user action.

## See also

- `/vsdd-factory:deactivate` — reverse this
- Orchestrator agent: `${CLAUDE_PLUGIN_ROOT}/agents/orchestrator/orchestrator.md`
