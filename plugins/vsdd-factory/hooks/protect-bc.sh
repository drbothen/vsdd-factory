#!/bin/bash
# protect-bc.sh — PreToolUse hook that blocks edits to green Behavioral Contracts
#
# Companion to protect-vp.sh. BC files in
# .factory/specs/behavioral-contracts/BC-*.md that have reached
# "Status: green" are immutable per spec-format rules. Non-green contracts
# are freely editable.
#
# Exits 2 (block) only when the file matches a BC path AND already exists on
# disk AND contains a `Status: green` line. Otherwise exits 0.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  echo "protect-bc.sh: jq is required but not found" >&2
  exit 1
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

if [[ ! "$FILE_PATH" =~ \.factory/specs/behavioral-contracts/BC-.*\.md$ ]]; then
  exit 0
fi

if [[ ! -f "$FILE_PATH" ]]; then
  exit 0
fi

if grep -q "^Status: green" "$FILE_PATH"; then
  echo "Blocked: $FILE_PATH has Status: green and is immutable." >&2
  echo "To change a green BC, create a new BC that supersedes it per spec-format.md." >&2
  exit 2
fi

exit 0
