#!/bin/bash
# handoff-validator.sh — SubagentStop hook
#
# Validates that subagent output is non-empty and structurally plausible.
# An empty or truncated subagent result is a common silent failure (SOUL.md #4)
# — the orchestrator ends up acting on nothing and the pipeline drifts.
#
# This hook reads the SubagentStop JSON from stdin, extracts the result text,
# and warns to stderr if:
#   - result is empty or whitespace only
#   - result is under 40 characters (suspiciously short for any factory agent)
#
# Non-blocking — the orchestrator still receives the (empty) result and can
# decide what to do. The warning surfaces the problem for human review.
#
# Ports dark-factory's handoff-validator.ts runtime extension.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
# SubagentStop sends: agent_type, last_assistant_message
# Fallback fields for compatibility with potential format variations
AGENT=$(echo "$INPUT" | jq -r '.agent_type // .subagent_name // "unknown"')
RESULT=$(echo "$INPUT" | jq -r '.last_assistant_message // .result // .output // empty')

TRIMMED=$(echo -n "$RESULT" | tr -d '[:space:]')
LEN=${#TRIMMED}

if (( LEN == 0 )); then
  echo "handoff-validator: subagent '$AGENT' returned an empty result." >&2
  echo "  This is a silent-failure risk — verify before continuing." >&2
  exit 0
fi

if (( LEN < 40 )); then
  echo "handoff-validator: subagent '$AGENT' returned only $LEN non-whitespace characters." >&2
  echo "  Suspiciously short — verify the subagent completed its task." >&2
fi

exit 0
