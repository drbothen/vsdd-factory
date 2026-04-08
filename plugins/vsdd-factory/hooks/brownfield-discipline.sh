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

if [[ -z "$FILE_PATH" ]]; then
  exit 0
fi

if [[ "$FILE_PATH" =~ (^|/)\.reference/ ]]; then
  echo "Blocked: $FILE_PATH is inside .reference/ which is read-only." >&2
  echo "Reference codebases are used for brownfield analysis only — edits poison the extraction." >&2
  echo "If you need to change reference material, update the source repo upstream and re-clone." >&2
  exit 2
fi

exit 0
