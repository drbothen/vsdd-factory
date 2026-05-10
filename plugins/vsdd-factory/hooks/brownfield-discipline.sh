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

# Source canonical block-message helper (provides block_pre).
_SELF_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
_BLOCK_SH="${CLAUDE_PLUGIN_ROOT:+${CLAUDE_PLUGIN_ROOT}/hooks/lib/block.sh}"
_BLOCK_SH="${_BLOCK_SH:-${_SELF_DIR}/lib/block.sh}"
# shellcheck source=lib/block.sh disable=SC1091
if [ -f "$_BLOCK_SH" ]; then source "$_BLOCK_SH"; fi

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
  block_pre "brownfield-discipline" \
    ".reference/ is read-only — edits poison brownfield extraction. Reference codebases are for analysis only" \
    "Update the source repo upstream and re-clone" \
    "reference_readonly"
fi

exit 0
