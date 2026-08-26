#!/bin/bash
# validate-bc-title.sh — PostToolUse hook for Policy 7
#
# Validates that a BC file's H1 heading matches its title in BC-INDEX.md.
# The H1 is the authoritative title — BC-INDEX must match, not the other
# way around. This hook catches drift after edits.
#
# Trigger: PostToolUse on Edit|Write to BC-*.md files.
# Exit 0 on pass (or if BC-INDEX doesn't exist yet).
# Exit 2 on mismatch with diagnostic on stderr.
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

# Only trigger for BC files (not BC-INDEX itself)
case "$FILE_PATH" in
  *behavioral-contracts/BC-*.md) ;;
  *) exit 0 ;;
esac

# Skip BC-INDEX.md
if [[ "$FILE_PATH" == *"BC-INDEX.md" ]]; then
  exit 0
fi

# Extract BC ID from filename (e.g., BC-2.01.001 from BC-2.01.001.md or BC-2.01.001-slug.md)
FILENAME=$(basename "$FILE_PATH" .md)
BC_ID=$(echo "$FILENAME" | grep -oE 'BC-[0-9]+\.[0-9]+\.[0-9]+' || true)

if [[ -z "$BC_ID" ]]; then
  exit 0
fi

# Extract H1 title from the BC file
# H1 format: # BC-S.SS.NNN: <title>
H1_TITLE=$(grep -m1 "^# ${BC_ID}:" "$FILE_PATH" | sed "s/^# ${BC_ID}: *//" || true)

if [[ -z "$H1_TITLE" ]]; then
  exit 0  # No H1 found — may be in progress
fi

# Find BC-INDEX.md
BC_DIR=$(dirname "$FILE_PATH")
BC_INDEX="$BC_DIR/BC-INDEX.md"

if [[ ! -f "$BC_INDEX" ]]; then
  exit 0  # BC-INDEX doesn't exist yet
fi

# Extract the title for this BC from BC-INDEX.
# Table format: | BC-S.SS.NNN | Title | ... |
#
# The BC id can legitimately appear in BC-INDEX more than once — e.g. in a
# capability-satisfaction table (| BC | Satisfies | ... |) that precedes the
# §2 navigation table. Matching the first occurrence anywhere grabbed the
# satisfaction cell (e.g. "CAP-001") and reported it as the indexed title,
# firing a false bc_h1_index_drift (#566). Scope the lookup to the table
# whose header row carries a "Title" column; only that table's title cell is
# authoritative. Fall back to the first occurrence when no Title-headed table
# is present (headerless single-row indexes), preserving prior behavior.
INDEX_TITLE=$(awk -F'|' -v bc="$BC_ID" '
  function trim(s) { gsub(/^[ \t]+|[ \t]+$/, "", s); return s }
  {
    # Non-table line ends any table scope.
    if ($0 !~ /^[ \t]*\|/) { in_title_table = 0; next }
    c2 = trim($2); c3 = trim($3)
    # A header row with "Title" in the 2nd data column opens a scoped table.
    if (tolower(c3) == "title") { in_title_table = 1; next }
    if (c2 == bc) {
      if (in_title_table && scoped == "") scoped = c3
      if (fallback == "") fallback = c3
    }
  }
  END { if (scoped != "") print scoped; else print fallback }
' "$BC_INDEX")

if [[ -z "$INDEX_TITLE" ]]; then
  # BC exists in file but not in BC-INDEX — that's a different issue (criterion 23)
  exit 0
fi

# Compare titles
if [[ "$H1_TITLE" != "$INDEX_TITLE" ]]; then
  block_pre "validate-bc-title" \
    "BC H1 title \"$H1_TITLE\" != BC-INDEX title \"$INDEX_TITLE\" (POLICY 7: bc_h1_is_title_source_of_truth)" \
    "Update BC-INDEX title to match H1, or fix H1 if it is wrong" \
    "bc_h1_index_drift"
fi

exit 0
