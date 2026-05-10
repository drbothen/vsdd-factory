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

# Source canonical block-message helper (provides block_pre_json).
_SELF_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
_BLOCK_SH="${CLAUDE_PLUGIN_ROOT:+${CLAUDE_PLUGIN_ROOT}/hooks/lib/block.sh}"
_BLOCK_SH="${_BLOCK_SH:-${_SELF_DIR}/lib/block.sh}"
# shellcheck source=lib/block.sh disable=SC1091
if [ -f "$_BLOCK_SH" ]; then source "$_BLOCK_SH"; fi

# Read PreToolUse JSON from stdin
INPUT=$(cat)

# Extract file_path (Edit and Write both put it in tool_input.file_path)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

emit_allow() {
  printf '{"hookSpecificOutput":{"hookEventName":"PreToolUse","permissionDecision":"allow"}}\n'
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
  block_pre_json "protect-vp" \
    "$FILE_PATH has Status: green and is immutable per SOUL.md #4 and spec-format rules" \
    "Create a new VP that supersedes it; cite the old VP via the new VP's 'Supersedes:' field" \
    "vp_green_immutable"
fi

# VP file exists but is not green — allow the edit.
emit_allow
