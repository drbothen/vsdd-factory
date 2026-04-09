#!/bin/bash
# protect-vp.sh — PreToolUse hook that blocks edits to green Verification Properties
#
# Per SOUL.md #4 and the spec-format rules, VP files in
# .factory/specs/verification-properties/ that have reached "Status: green"
# are immutable. To change a green VP, create a new VP that supersedes it.
#
# Emits a PreToolUse JSON envelope with permissionDecision — "deny" for a
# protected green VP, "allow" otherwise. The envelope carries a reason the
# agent can surface to the user, which is richer than a bare exit code.
#
# Deterministic, <100ms, no LLM.

set -euo pipefail

# Read PreToolUse JSON from stdin
INPUT=$(cat)

# Extract file_path (Edit and Write both put it in tool_input.file_path)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

emit_allow() {
  printf '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow"}}\n'
  exit 0
}

emit_deny() {
  local reason="$1"
  # Escape quotes in reason for JSON safety via jq
  jq -nc --arg reason "$reason" '{
    hookSpecificOutput: {
      hookEventName: "PreToolUse",
      permissionDecision: "deny",
      permissionDecisionReason: $reason
    }
  }'
  exit 0
}

# Fast path: not a VP file → allow immediately
if [[ ! "$FILE_PATH" =~ \.factory/specs/verification-properties/VP-.*\.md$ ]]; then
  emit_allow
fi

# VP path matched. Check if the existing file has Status: green.
# New files (not yet on disk) are allowed — only existing green VPs are immutable.
if [[ ! -f "$FILE_PATH" ]]; then
  emit_allow
fi

if grep -q "^Status: green" "$FILE_PATH"; then
  emit_deny "Blocked: $FILE_PATH has Status: green and is immutable per SOUL.md #4. To change a green VP, create a new VP that supersedes it (see .claude/rules/spec-format.md). The old VP stays on disk unchanged; reference it from the new VP's 'Supersedes:' field."
fi

# VP file exists but is not green — allow the edit.
emit_allow
