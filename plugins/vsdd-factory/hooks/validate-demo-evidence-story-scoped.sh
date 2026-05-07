#!/bin/bash
# validate-demo-evidence-story-scoped.sh — PostToolUse hook for POL-010
#
# Enforces that all demo evidence files live under docs/demo-evidence/<STORY-ID>/
# and never directly at docs/demo-evidence/*.md. Prevents evidence-report.md
# collisions when multiple stories' evidence lands in the same target repo.
#
# Trigger: PostToolUse on Write/Edit to files under docs/demo-evidence/.
# Exit 0 on pass (file is in a story subdirectory, or not a demo-evidence file).
# Exit 2 on flat-level file (blocks commit).
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

if [[ -z "$FILE_PATH" ]] || [[ ! -f "$FILE_PATH" ]]; then
  exit 0
fi

# Only trigger for files under docs/demo-evidence/
case "$FILE_PATH" in
  */docs/demo-evidence/*) ;;
  *) exit 0 ;;
esac

# Extract the path after docs/demo-evidence/
RELATIVE="${FILE_PATH##*/docs/demo-evidence/}"

# Check if the file is directly at the top level (no / in relative path)
# Valid: S-0.02/evidence-report.md (has a /)
# Invalid: evidence-report.md (no /)
if [[ "$RELATIVE" != */* ]]; then
  block_pre "validate-demo-evidence-story-scoped" \
    "Demo evidence not under <STORY-ID>/ (POL-010): $FILE_PATH is at the top level of docs/demo-evidence/" \
    "Move to docs/demo-evidence/<STORY-ID>/$(basename "$FILE_PATH") where <STORY-ID> matches the story's frontmatter story_id field" \
    "pol_010_violation"
fi

exit 0
