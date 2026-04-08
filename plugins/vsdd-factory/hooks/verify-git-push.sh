#!/bin/bash
# PreToolUse hook: verify before git push
# Exit 0 = allow, Exit 2 = block

INPUT=$(cat)
COMMAND=$(echo "$INPUT" | jq -r '.tool_input.command // empty')

# Only process git push commands
if [[ "$COMMAND" != *"git push"* ]]; then
  exit 0
fi

# Block force push
if [[ "$COMMAND" == *"--force"* ]] || [[ "$COMMAND" == *"-f "* ]]; then
  echo "Force push detected. Verify this is intentional." >&2
  exit 2
fi

# Remind about verification
echo '{"additionalContext": "Push requested. Ensure cargo test, clippy, and fmt are clean before pushing."}'
exit 0
