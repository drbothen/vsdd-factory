# After: Visible-Block (S-15.01)

## Context

After S-15.01, all hooks.json.* files have `"async": true` removed from every
event entry (AC-010, BC-9.01.006). The dispatcher now runs synchronously at the
envelope — Claude Code waits for the dispatcher exit code before proceeding.

## Behavior

With the new synchronous envelope (no `async: true` on PostToolUse):
```
echo '{"hook_event_name":"PostToolUse","tool_name":"Write","session_id":"test","tool_input":{"path":"CHANGELOG.md"}}' \
  | CLAUDE_PLUGIN_ROOT=plugins/vsdd-factory factory-dispatcher
# Dispatcher exits 2 (block intent from validate-template-compliance)
# Claude Code SEES the exit 2 and surfaces "Tool call blocked" to the user
```

## Block Message to User

The user now sees:
```
BLOCKED by validate-template-compliance: CHANGELOG.md violates template constraints.
Fix: Ensure the changelog entry follows the required format.
Code: E-TMPL-001.
```

## Async Partition

Telemetry-only plugins (capture-commit-activity, session-end-telemetry, etc.) are
now classified `async = true` in hooks-registry.toml. They run fire-and-forget after
the sync_group completes, draining within ASYNC_DRAIN_WINDOW_MS (DI-019 = 100ms).
Their verdicts never reach Claude Code — the fix for the silent-block-bleed bug is
structural, not behavioral for telemetry.
