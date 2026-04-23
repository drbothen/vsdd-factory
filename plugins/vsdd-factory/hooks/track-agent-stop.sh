#!/bin/bash
# track-agent-stop.sh — SubagentStop hook.
#
# Emits an `agent.stop` telemetry event for every subagent completion. Pairs
# with track-agent-start.sh (PreToolUse Agent) to enable duration measurement
# via factory-sla.
#
# This hook NEVER blocks. It exits 0 on every path, emitting a best-effort
# event. The other SubagentStop hooks (handoff-validator, pr-manager-
# completion-guard, etc.) run independently and may still emit block events.
#
# Trigger: SubagentStop (all subagents).
# Deterministic, <50ms, no LLM.

set +e

if ! command -v jq >/dev/null 2>&1; then
  exit 0
fi

INPUT=$(cat)
AGENT=$(echo "$INPUT" | jq -r '.agent_type // .subagent_name // "unknown"')
RESULT=$(echo "$INPUT" | jq -r '.last_assistant_message // .result // ""')

# Rough classification of the exit condition. Matches the BLOCKED detection
# used by pr-manager-completion-guard for consistency.
RESULT_LEN=$(printf '%s' "$RESULT" | tr -d '[:space:]' | wc -c | tr -d ' ')
EXIT_CLASS=ok
if [ "$RESULT_LEN" -eq 0 ]; then
  EXIT_CLASS=empty
elif echo "$RESULT" | grep -qE '^(Status:|##?\s*)?\s*BLOCKED'; then
  EXIT_CLASS=blocked
fi

_emit() {
  if [ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -x "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" ]; then
    "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" "$@" 2>/dev/null || true
  fi
  return 0
}

_emit type=agent.stop hook=track-agent-stop matcher=SubagentStop \
      subagent="$AGENT" exit_class="$EXIT_CLASS" result_len="$RESULT_LEN"

exit 0
