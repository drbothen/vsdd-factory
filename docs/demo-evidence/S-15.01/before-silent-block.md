# Before: Silent-Block Scenario (S-15.01)

## Context

Prior to S-15.01, the `hooks.json.*` envelope had `"async": true` on PostToolUse,
Stop, SubagentStop, SessionStart, SessionEnd, WorktreeCreate, WorktreeRemove, and
PostToolUseFailure events. This meant the dispatcher ran fire-and-forget: Claude Code
did not wait for the dispatcher exit code before proceeding.

## Prism Audit Finding (2026-05-07)

55 block decisions were silently discarded during the prism audit. A PostToolUse
dispatcher invocation with a blocking plugin (e.g. `validate-template-compliance`)
would exit 2, but since the hook was declared `async: true` at the envelope, Claude
Code never saw the block signal.

## Reproduction

With the old envelope (`async: true` on PostToolUse):
```
echo '{"hook_event_name":"PostToolUse","tool_name":"Write","session_id":"test","tool_input":{"path":"CHANGELOG.md"}}' \
  | CLAUDE_PLUGIN_ROOT=plugins/vsdd-factory factory-dispatcher
# Dispatcher exits 2 (block intent from validate-template-compliance)
# But Claude Code ignores it (async envelope = no verdict gate)
```

The 55 block decisions refer to audit log entries where `block_intent=true` in the
dispatcher internal log but no corresponding blocking behavior was observed by the
operator in the Claude Code session.
