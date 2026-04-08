#!/bin/bash
# PostToolUse hook: remind about STATE.md after .factory/ commits
# Exit 0 always (non-blocking, advisory only)

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# Only process git commit commands
if [[ "$COMMAND" != *"git commit"* ]]; then
  exit 0
fi

# Check if this commit was in .factory/
if [[ "$COMMAND" == *".factory"* ]] || [[ "$COMMAND" == *"cd .factory"* ]]; then
  # Check if STATE.md was part of the commit
  if [[ "$COMMAND" != *"STATE.md"* ]]; then
    echo '{"additionalContext": "Factory artifact committed but STATE.md was not updated. If this is a phase transition or milestone, update STATE.md using the state-update skill."}'
    exit 0
  fi
fi

echo '{}'
exit 0
