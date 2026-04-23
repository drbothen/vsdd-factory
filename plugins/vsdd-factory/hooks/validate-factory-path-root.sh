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

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

_emit() {
  if [ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -x "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" ]; then
    "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" "$@" 2>/dev/null || true
  fi
  return 0
}

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

  _emit type=hook.block hook=validate-factory-path-root matcher=PostToolUse \
        reason=factory_path_worktree_relative file_path="$FILE_PATH" worktree="$WORKTREE"
  echo "FACTORY PATH ERROR — writing to worktree instead of project root:" >&2
  echo "  Got:      $FILE_PATH" >&2
  echo "  Expected: <project-root>/.factory/$RELATIVE" >&2
  echo "" >&2
  echo "  You are inside $WORKTREE/ and used a relative .factory/ path." >&2
  echo "  .factory/ artifacts MUST use the absolute project root path," >&2
  echo "  not a path relative to the worktree working directory." >&2
  echo "" >&2
  echo "  Fix: use the resolved project path from your dispatch prompt" >&2
  echo "  (e.g., /Users/.../project/.factory/$RELATIVE)." >&2
  exit 2
fi

exit 0
