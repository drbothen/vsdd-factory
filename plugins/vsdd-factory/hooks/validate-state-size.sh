#!/bin/bash
# validate-state-size.sh — PostToolUse hook for STATE.md size enforcement
#
# Checks line count of STATE.md after every Write/Edit.
# WARN at 200 lines, BLOCK at 500 lines (unless the write reduced size).
#
# Trigger: PostToolUse on Write/Edit to STATE.md.
# Exit 0 on pass (or if file is not STATE.md, or if write reduced size).
# Exit 2 on bloat detected (>500 lines AND file grew) with diagnostic on stderr.
#
# Deterministic, <100ms, no LLM.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

if [[ -z "$FILE_PATH" ]]; then
  exit 0
fi

# Only trigger for STATE.md files in .factory/
case "$FILE_PATH" in
  */.factory/STATE.md|*.factory/STATE.md) ;;
  *) exit 0 ;;
esac

if [[ ! -f "$FILE_PATH" ]]; then
  exit 0
fi

LINE_COUNT=$(wc -l < "$FILE_PATH" | tr -d ' ')

# Check if this write reduced the file size (compaction).
# Use git to compare against the committed version.
PARENT_DIR=$(dirname "$FILE_PATH")
PRIOR_COUNT=0
if command -v git &>/dev/null; then
  PRIOR_COUNT=$(git -C "$PARENT_DIR" show HEAD:STATE.md 2>/dev/null | wc -l | tr -d ' ' || echo 0)
fi

# If the write REDUCED lines, always allow (compaction in progress)
if [[ "$LINE_COUNT" -lt "$PRIOR_COUNT" ]]; then
  exit 0
fi

if [[ "$LINE_COUNT" -gt 500 ]]; then
  echo "STATE.md BLOAT — BLOCKED:" >&2
  echo "  STATE.md has $LINE_COUNT lines (limit: 500)." >&2
  echo "  STATE.md should be a quick status check, not a history log." >&2
  echo "  Run /vsdd-factory:compact-state to extract historical content" >&2
  echo "  to cycle files (burst logs, adversary passes, session checkpoints)." >&2
  echo "  See state-manager agent 'Content Routing Rules' for what belongs where." >&2
  exit 2
elif [[ "$LINE_COUNT" -gt 200 ]]; then
  echo "STATE.md SIZE WARNING:" >&2
  echo "  STATE.md has $LINE_COUNT lines (recommended: <200, limit: 500)." >&2
  echo "  Consider running /vsdd-factory:compact-state to extract" >&2
  echo "  historical content to cycle files before it grows further." >&2
  exit 0
fi

exit 0
