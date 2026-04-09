#!/bin/bash
# protect-bc.sh — PreToolUse hook that blocks edits to green Behavioral Contracts
#
# Companion to protect-vp.sh. BC files in
# .factory/specs/behavioral-contracts/BC-*.md that have reached
# "Status: green" are immutable per spec-format rules. Non-green contracts
# are freely editable.
#
# Emits a PreToolUse JSON envelope with permissionDecision — "deny" for a
# protected green BC, "allow" otherwise.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  echo "protect-bc.sh: jq is required but not found" >&2
  exit 1
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

emit_allow() {
  printf '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow"}}\n'
  exit 0
}

emit_deny() {
  local reason="$1"
  jq -nc --arg reason "$reason" '{
    hookSpecificOutput: {
      hookEventName: "PreToolUse",
      permissionDecision: "deny",
      permissionDecisionReason: $reason
    }
  }'
  exit 0
}

if [[ ! "$FILE_PATH" =~ \.factory/specs/behavioral-contracts/BC-.*\.md$ ]]; then
  emit_allow
fi

if [[ ! -f "$FILE_PATH" ]]; then
  emit_allow
fi

if grep -q "^Status: green" "$FILE_PATH"; then
  emit_deny "Blocked: $FILE_PATH has Status: green and is immutable per spec-format.md. To change a green BC, create a new BC that supersedes it. The old BC stays on disk; reference it from the new BC's 'Supersedes:' field."
fi

emit_allow
