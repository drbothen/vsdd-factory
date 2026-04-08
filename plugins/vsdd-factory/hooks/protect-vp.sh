#!/bin/bash
# protect-vp.sh — PreToolUse hook that blocks edits to green Verification Properties
#
# Per SOUL.md #4 and the spec-format rules, VP files in
# .factory/specs/verification-properties/ that have reached "Status: green"
# are immutable. To change a green VP, create a new VP that supersedes it.
#
# This hook reads PreToolUse JSON from stdin, extracts the file_path,
# and exits 2 (block) only if BOTH:
#   1. The file path matches .factory/specs/verification-properties/VP-*.md
#   2. The file currently exists AND contains a line starting with "Status: green"
#
# Otherwise it exits 0 (allow). Deterministic, <100ms, no LLM.
#
# Replaces the previous prompt-based hook that used haiku and timed out
# intermittently, blocking unrelated writes.

set -euo pipefail

# Read PreToolUse JSON from stdin
INPUT=$(cat)

# Extract file_path (Edit and Write both put it in tool_input.file_path)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

# Fast path: not a VP file → allow immediately
if [[ ! "$FILE_PATH" =~ \.factory/specs/verification-properties/VP-.*\.md$ ]]; then
  exit 0
fi

# VP path matched. Check if the existing file has Status: green.
# New files (not yet on disk) are allowed — only existing green VPs are immutable.
if [[ ! -f "$FILE_PATH" ]]; then
  exit 0
fi

if grep -q "^Status: green" "$FILE_PATH"; then
  echo "Blocked: $FILE_PATH has Status: green and is immutable per SOUL.md #4." >&2
  echo "To change a green VP, create a new VP that supersedes it (see .claude/rules/spec-format.md)." >&2
  exit 2
fi

# VP file exists but is not green — allow the edit.
exit 0
