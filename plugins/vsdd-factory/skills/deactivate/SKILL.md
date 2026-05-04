---
name: deactivate
description: Reverse `/vsdd-factory:activate` — remove the orchestrator default agent, the platform activation state, and the generated `hooks.json`. Leaves the plugin enabled; only the default persona and per-machine hooks config are cleared.
disable-model-invocation: true
---

# Deactivate VSDD Factory

Reverses everything `/vsdd-factory:activate` does: clears the
orchestrator default-agent override in `.claude/settings.local.json`,
removes the platform activation block, and deletes the
per-machine `hooks/hooks.json` (which was a copy of the platform
variant). The plugin itself stays enabled — agents, skills, and the
underlying `hooks.json.<platform>` files remain available for explicit
invocation or future re-activation.

## Procedure

1. **Read `.claude/settings.local.json`.** If it doesn't exist, the
   factory was never activated here — say so and stop.

2. **Sanity-check the agent default.** If the existing `agent` value
   does not point at a `vsdd-factory:` agent, stop and warn — don't
   clobber unrelated config.

3. **Remove the keys.** Use `jq 'del(.agent) | del(.["vsdd-factory"])'`
   and write the file back. If the resulting object is empty, either
   delete the file or leave it as `{}` — ask the user which.

4. **Remove the per-machine hooks.json.** The activation step copied
   `hooks/hooks.json.<platform>` to `hooks/hooks.json`. Delete that
   copy: `rm -f "${CLAUDE_PLUGIN_ROOT}/hooks/hooks.json"`. The
   gitignore'd file leaves no git side effect; the per-platform
   variants remain in place for the next activation.

5. **Confirm.** Print:
   - The remaining `settings.local.json` contents
   - That the plugin is still enabled (you can still invoke
     individual skills/agents explicitly)
   - That `/vsdd-factory:activate` is the inverse and is required
     before the dispatcher fires again

## Notes

Step 4 deletes a per-machine artifact — `hooks/hooks.json` is generated
at activation time by copying the right `hooks.json.<platform>` variant
into place. Removing it has no git side effect (the file is gitignored)
and the per-platform variants remain in place for the next activation
to copy from.

## See also

- `/vsdd-factory:activate` — the inverse
