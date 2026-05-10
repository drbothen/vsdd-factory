#!/bin/bash
# validate-factory-path-root.sh — PostToolUse hook for Edit|Write
#
# Catches agents writing .factory/ artifacts to relative paths inside
# story worktrees instead of the project root. When an agent runs
# inside .worktrees/STORY-NNN/ and writes to ".factory/foo.md", the
# resolved path becomes .worktrees/STORY-NNN/.factory/foo.md — a
# nonexistent directory that silently creates artifacts in the wrong
# place (or falls back to docs/ paths).
#
# The correct behavior: .factory/ writes always use absolute paths
# to the project root's .factory/ worktree on factory-artifacts.
#
# Trigger: PostToolUse on Write/Edit.
# Exit 0 on pass (path is at project root, or not a .factory/ write).
# Exit 2 blocks with instructions to use absolute path.
#
# Deterministic, <100ms, no LLM.

set -euo pipefail

# Source canonical block-message helper (provides block_pre).
_SELF_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
_BLOCK_SH="${CLAUDE_PLUGIN_ROOT:+${CLAUDE_PLUGIN_ROOT}/hooks/lib/block.sh}"
_BLOCK_SH="${_BLOCK_SH:-${_SELF_DIR}/lib/block.sh}"
# shellcheck source=lib/block.sh disable=SC1091
if [ -f "$_BLOCK_SH" ]; then source "$_BLOCK_SH"; fi

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

if [[ -z "$FILE_PATH" ]]; then
  exit 0
fi

# Only check .factory/ paths
case "$FILE_PATH" in
  *.factory/*) ;;
  *) exit 0 ;;
esac

# Block if the path goes through .worktrees/ — agent is writing to
# a relative .factory/ from inside a story worktree
if [[ "$FILE_PATH" == *".worktrees/"*"/.factory/"* ]]; then
  # Extract the story worktree name for the diagnostic
  WORKTREE=$(echo "$FILE_PATH" | grep -oE '\.worktrees/[^/]+' | head -1 || true)
  RELATIVE="${FILE_PATH##*/.factory/}"

  block_pre "validate-factory-path-root" \
    "Worktree-relative .factory/ path used: $FILE_PATH (inside $WORKTREE). .factory/ artifacts MUST use absolute project root path" \
    "Use absolute <project-root>/.factory/$RELATIVE from your dispatch prompt" \
    "factory_path_relative"
fi

exit 0
