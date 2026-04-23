#!/bin/bash
# track-agent-start.sh — PreToolUse hook on Agent dispatches.
#
# Emits an `agent.start` telemetry event for every subagent dispatch. Pairs
# with track-agent-stop.sh (SubagentStop) to enable duration measurement
# via factory-sla.
#
# This hook NEVER blocks. It exits 0 on every path, emitting a best-effort
# event. Missing jq, unreachable emit-event, or malformed input all result
# in silent no-ops.
#
# Trigger: PreToolUse on Agent.
# Deterministic, <50ms, no LLM.

set +e

if ! command -v jq >/dev/null 2>&1; then
  exit 0
fi

INPUT=$(cat)
TOOL=$(echo "$INPUT" | jq -r '.tool_name // ""')
if [[ "$TOOL" != "Agent" ]]; then
  exit 0
fi

SUBAGENT=$(echo "$INPUT" | jq -r '.tool_input.subagent_type // "unknown"')
PROMPT=$(echo "$INPUT" | jq -r '.tool_input.prompt // ""')

# Best-effort story_id extraction (S-N.NN or STORY-NNN) for correlation.
STORY_ID=$(echo "$PROMPT" | grep -oE 'S-[0-9]+\.[0-9]+' | head -1 || true)
if [ -z "$STORY_ID" ]; then
  STORY_ID=$(echo "$PROMPT" | grep -oE 'STORY-[0-9]+' | head -1 || true)
fi

_emit() {
  if [ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -x "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" ]; then
    "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" "$@" 2>/dev/null || true
  fi
  return 0
}

_emit type=agent.start hook=track-agent-start matcher=Agent \
      subagent="$SUBAGENT" ${STORY_ID:+story_id="$STORY_ID"}

exit 0
