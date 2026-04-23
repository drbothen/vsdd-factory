#!/bin/bash
# brownfield-discipline.sh — PreToolUse hook
#
# Enforces that `.reference/**` directories are read-only. The brownfield
# ingest pipeline clones reference codebases into `.reference/` for analysis;
# editing them corrupts the extraction and poisons downstream spec work.
#
# Blocks Edit|Write on any path containing `/.reference/` or starting with
# `.reference/`. Exits 0 for every other path.
#
# Ports dark-factory's brownfield-discipline.ts runtime extension.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  echo "brownfield-discipline.sh: jq is required but not found" >&2
  exit 1
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')
TOOL_NAME=$(echo "$INPUT" | jq -r '.tool_name // "Edit|Write"')

if [[ -z "$FILE_PATH" ]]; then
  exit 0
fi

_emit() {
  if [ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -x "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" ]; then
    "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" "$@" 2>/dev/null || true
  fi
  return 0
}

if [[ "$FILE_PATH" =~ (^|/)\.reference/ ]]; then
  _emit type=hook.block hook=brownfield-discipline matcher="$TOOL_NAME" reason=reference_readonly file_path="$FILE_PATH"
  echo "Blocked: $FILE_PATH is inside .reference/ which is read-only." >&2
  echo "Reference codebases are used for brownfield analysis only — edits poison the extraction." >&2
  echo "If you need to change reference material, update the source repo upstream and re-clone." >&2
  exit 2
fi

exit 0
