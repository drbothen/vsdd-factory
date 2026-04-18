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

# Extract the title for this BC from BC-INDEX
# Table format: | BC-S.SS.NNN | Title | ... |
INDEX_TITLE=$(awk -F'|' -v bc="$BC_ID" '
  {
    gsub(/^[ \t]+|[ \t]+$/, "", $2)
    if ($2 == bc) {
      gsub(/^[ \t]+|[ \t]+$/, "", $3)
      print $3
      exit
    }
  }
' "$BC_INDEX")

if [[ -z "$INDEX_TITLE" ]]; then
  # BC exists in file but not in BC-INDEX — that's a different issue (criterion 23)
  exit 0
fi

# Compare titles
if [[ "$H1_TITLE" != "$INDEX_TITLE" ]]; then
  echo "POLICY 7 VIOLATION (bc_h1_is_title_source_of_truth):" >&2
  echo "  - BC file H1 title: \"$H1_TITLE\"" >&2
  echo "  - BC-INDEX title:   \"$INDEX_TITLE\"" >&2
  echo "  The H1 heading is authoritative. Update BC-INDEX to match," >&2
  echo "  or if the H1 is wrong, fix the H1 first." >&2
  exit 2
fi

exit 0
