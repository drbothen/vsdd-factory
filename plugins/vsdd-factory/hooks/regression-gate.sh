#!/bin/bash
# regression-gate.sh — PostToolUse hook that records test-run outcomes
#
# Watches Bash commands that run tests (cargo test, pytest, npm test, go test,
# just test, just ci). After each run, records pass/fail to
# `.factory/regression-state.json`. If the suite transitions from pass → fail,
# emits a warning to stderr so the next Edit/Write can be informed.
#
# This is a telemetry hook — it does not block. The red-gate hook consumes the
# state file if strict mode is active.
#
# Ports dark-factory's regression-gate.ts runtime extension.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
TOOL=$(echo "$INPUT" | jq -r '.tool_name // empty')
[[ "$TOOL" != "Bash" ]] && exit 0

_emit() {
  if [ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -x "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" ]; then
    "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" "$@" 2>/dev/null || true
  fi
  return 0
}

CMD=$(echo "$INPUT" | jq -r '.tool_input.command // empty')
# Claude Code's `tool_response` for Bash does NOT include `exit_code`;
# it sends `interrupted`, `stdout`, `stderr`, `isImage`, `noOutputExpected`.
# Prefer `exit_code` / `returncode` when the host provides them (back-compat),
# fall back to the `interrupted` signal under Claude Code.
EXIT=$(echo "$INPUT" | jq -r '.tool_response.exit_code // .tool_response.returncode // empty')
INTERRUPTED=$(echo "$INPUT" | jq -r '.tool_response.interrupted // empty')

# Only care about test-running commands
case "$CMD" in
  *"cargo test"*|*"cargo nextest"*|*"pytest"*|*"pnpm test"*|*"npm test"*|*"go test"*|*"just test"*|*"just ci"*|*"yarn test"*) ;;
  *) exit 0 ;;
esac

STATE_DIR=".factory"
STATE_FILE="$STATE_DIR/regression-state.json"
[[ ! -d "$STATE_DIR" ]] && exit 0

# Determine pass/fail
# Priority: exit_code (when host sends it) > interrupted (Claude Code shape).
# When neither is available we silently skip — better to record nothing than
# to record wrong state.
STATUS="unknown"
if [[ "$EXIT" == "0" ]]; then
  STATUS="pass"
elif [[ -n "$EXIT" && "$EXIT" != "null" ]]; then
  STATUS="fail"
elif [[ "$INTERRUPTED" == "true" ]]; then
  STATUS="fail"
elif [[ "$INTERRUPTED" == "false" ]]; then
  STATUS="pass"
fi

[[ "$STATUS" == "unknown" ]] && exit 0

# Read prior status
PRIOR="unknown"
if [[ -f "$STATE_FILE" ]]; then
  PRIOR=$(jq -r '.status // "unknown"' "$STATE_FILE" 2>/dev/null || echo "unknown")  # STDERR-EXEMPT: jq failure falls back to unknown
fi

# Write new state
TS=$(date -u +"%Y-%m-%dT%H:%M:%SZ")
jq -n --arg s "$STATUS" --arg t "$TS" --arg c "$CMD" \
  '{status: $s, timestamp: $t, command: $c}' > "$STATE_FILE"

# Warn on regression
if [[ "$PRIOR" == "pass" && "$STATUS" == "fail" ]]; then
  _emit type=hook.block hook=regression-gate matcher=Bash \
        reason=regression_gate_pass_to_fail severity=warn command="$CMD"
  echo "regression-gate: suite transitioned pass → fail." >&2
  echo "  command: $CMD" >&2
  echo "  recorded: $STATE_FILE" >&2
fi

exit 0
