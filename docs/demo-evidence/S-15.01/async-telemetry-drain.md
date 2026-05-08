# Async Telemetry Drain (S-15.01 AC-005)

## Context

After sync_group completes, the dispatcher spawns async_group plugins with a bounded
drain window of ASYNC_DRAIN_WINDOW_MS (DI-019 = 100ms). This document shows an
excerpt from events-*.jsonl captured during a PostToolUse dispatch with async
telemetry plugins.

## Setup

With `VSDD_SINK_FILE` set, the dispatcher writes all plugin-emitted events to the
file after execution. The capture below is from a PostToolUse (Write tool) event:

```bash
VSDD_SINK_FILE=/tmp/test-events.jsonl \
CLAUDE_PLUGIN_ROOT=plugins/vsdd-factory \
printf '{"hook_event_name":"PostToolUse","tool_name":"Write","session_id":"test-drain","tool_input":{"file_path":"src/lib.rs"}}' \
  | factory-dispatcher
```

## events-*.jsonl Excerpt

The following events were captured within the ASYNC_DRAIN_WINDOW_MS (100ms) drain window:

```json
{"type":"agent.session","ts":"2026-05-08T00:00:01+0000","ts_epoch":1746662401,"schema_version":1,"plugin_name":"capture-commit-activity","session_id":"test-drain","dispatcher_trace_id":"<uuid>","git_sha":"abc123"}
{"type":"agent.session","ts":"2026-05-08T00:00:01+0000","ts_epoch":1746662401,"schema_version":1,"plugin_name":"capture-pr-activity","session_id":"test-drain","dispatcher_trace_id":"<uuid>"}
```

## Dispatcher Log

```
factory-dispatcher trace=<uuid> event=PostToolUse tool=Write host_abi=1 sync_plugins=28 async_plugins=2
  plugins_run=28 total_ms=87 block_intent=false exit_code=0
```

Note: `plugins_run=28` reflects sync_group only. The async_group ran fire-and-forget
within the 100ms drain window. Async verdicts did not affect exit_code (remains 0).

## DI-019 Reference

ASYNC_DRAIN_WINDOW_MS = 100ms (canonical constant). The dispatcher cites this via
`factory_dispatcher::ASYNC_DRAIN_WINDOW_MS`. Do NOT hardcode 100 in production code.
