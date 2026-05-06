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

# Source canonical block-message helper (provides block_pre).
if [ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -f "${CLAUDE_PLUGIN_ROOT}/hooks/lib/block.sh" ]; then
  # shellcheck source=lib/block.sh
  source "${CLAUDE_PLUGIN_ROOT}/hooks/lib/block.sh"
fi

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

LINE_COUNT=$(wc -l < "$FILE_PATH" | tr -d ' \t\n')

# Check if this write reduced the file size (compaction).
# Use git to compare against the committed version.
PARENT_DIR=$(dirname "$FILE_PATH")
PRIOR_COUNT=0
if command -v git &>/dev/null; then
  _git_out=$(git -C "$PARENT_DIR" show HEAD:STATE.md 2>/dev/null | wc -l | tr -d ' ' 2>/dev/null) || _git_out=0
  # Ensure PRIOR_COUNT is a clean integer (strip trailing newlines/spaces)
  PRIOR_COUNT="${_git_out//[[:space:]]/}"
  PRIOR_COUNT="${PRIOR_COUNT:-0}"
  # If still not numeric, default to 0
  [[ "$PRIOR_COUNT" =~ ^[0-9]+$ ]] || PRIOR_COUNT=0
fi

# If the write REDUCED lines, always allow (compaction in progress)
if [[ "$LINE_COUNT" -lt "$PRIOR_COUNT" ]]; then
  exit 0
fi

if [[ "$LINE_COUNT" -gt 500 ]]; then
  block_pre "validate-state-size" \
    "STATE.md exceeds 500-line limit ($LINE_COUNT lines). STATE.md should be a quick status check, not a history log" \
    "Run /vsdd-factory:compact-state to extract historical content to cycle files" \
    "state_md_bloat"
elif [[ "$LINE_COUNT" -gt 200 ]]; then
  echo "STATE.md SIZE WARNING:" >&2
  echo "  STATE.md has $LINE_COUNT lines (recommended: <200, limit: 500)." >&2
  echo "  Consider running /vsdd-factory:compact-state to extract" >&2
  echo "  historical content to cycle files before it grows further." >&2
  exit 0
fi

exit 0
