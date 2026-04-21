#!/bin/bash
# validate-changelog-monotonicity.sh — PostToolUse hook for changelog ordering
#
# Validates that ## Changelog table rows have strictly decreasing versions
# (newest at top) and non-increasing dates. Also cross-checks frontmatter
# version against the top changelog row.
#
# Trigger: PostToolUse on Write/Edit to .factory/**/*.md.
# Exit 0 on pass (or if file has no changelog section).
# Exit 2 on ordering violation with diagnostic on stderr.
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

# Only trigger for .factory/ markdown files
case "$FILE_PATH" in
  *.factory/*.md) ;;
  *) exit 0 ;;
esac

# Skip files unlikely to have changelogs
case "$FILE_PATH" in
  *STATE.md|*INDEX.md|*burst-log*|*convergence-trajectory*|*session-checkpoint*|*lessons*) exit 0 ;;
esac

# Check if file has a Changelog section
if ! grep -q "^## Changelog" "$FILE_PATH"; then
  exit 0
fi

# Extract frontmatter version
FM_VERSION=$(awk '
  /^---$/{ fm++; next }
  fm==1 && /^version:/ {
    sub(/^version:[ \t]*/, "")
    gsub(/["'"'"']/, "")
    gsub(/^[ \t]+|[ \t]+$/, "")
    print
    exit
  }
' "$FILE_PATH")

ERRORS=""
PREV_VERSION=""
PREV_DATE=""
FIRST_VERSION=""
IN_CHANGELOG=false
IN_TABLE=false
LINE_NUM=0

while IFS= read -r line; do
  LINE_NUM=$((LINE_NUM + 1))

  # Enter changelog section
  if echo "$line" | grep -q "^## Changelog"; then
    IN_CHANGELOG=true
    continue
  fi

  # Exit on next H2
  if [[ "$IN_CHANGELOG" == true ]] && echo "$line" | grep -q "^## " && ! echo "$line" | grep -q "^## Changelog"; then
    break
  fi

  if [[ "$IN_CHANGELOG" != true ]]; then
    continue
  fi

  # Skip separator lines
  if echo "$line" | grep -qE '^\|[-: |]+\|$'; then
    IN_TABLE=true
    continue
  fi

  # Skip header row (contains "Version")
  if echo "$line" | grep -qi "Version.*|.*Date\|Version.*|.*Burst\|Version.*|.*Change"; then
    IN_TABLE=true
    continue
  fi

  # Parse data rows
  if [[ "$IN_TABLE" == true ]] && echo "$line" | grep -q '^|'; then
    # Extract version (first data column after leading |)
    VERSION=$(echo "$line" | awk -F'|' '{gsub(/^[ \t]+|[ \t]+$/, "", $2); print $2}')

    # Extract date — try column 3 (5-col: Version|Burst|Date|Author|Change)
    # or column 2 (4-col: Version|Date|Change|...)
    DATE=$(echo "$line" | awk -F'|' '{
      for(i=2; i<=NF; i++) {
        gsub(/^[ \t]+|[ \t]+$/, "", $i)
        if ($i ~ /^[0-9]{4}-[0-9]{2}-[0-9]{2}$/) { print $i; exit }
      }
    }')

    # Skip empty version rows
    if [[ -z "$VERSION" ]]; then
      continue
    fi

    # Track first (highest) version
    if [[ -z "$FIRST_VERSION" ]]; then
      FIRST_VERSION="$VERSION"
    fi

    # Check version ordering (must be strictly decreasing)
    if [[ -n "$PREV_VERSION" ]] && [[ "$PREV_VERSION" == "$VERSION" ]]; then
      ERRORS="${ERRORS:+$ERRORS\n}Line $LINE_NUM: duplicate version '$VERSION' (also at prior row)"
    fi

    # Check date ordering (must be non-increasing — newer at top)
    if [[ -n "$PREV_DATE" ]] && [[ -n "$DATE" ]]; then
      if [[ "$DATE" > "$PREV_DATE" ]]; then
        ERRORS="${ERRORS:+$ERRORS\n}Line $LINE_NUM: date '$DATE' is newer than prior row '$PREV_DATE' — changelog should be newest-first"
      fi
    fi

    PREV_VERSION="$VERSION"
    if [[ -n "$DATE" ]]; then
      PREV_DATE="$DATE"
    fi
  fi
done < "$FILE_PATH"

# Cross-check frontmatter version vs top changelog row
if [[ -n "$FM_VERSION" ]] && [[ -n "$FIRST_VERSION" ]]; then
  if [[ "$FM_VERSION" != "$FIRST_VERSION" ]]; then
    ERRORS="${ERRORS:+$ERRORS\n}Frontmatter version '$FM_VERSION' != top changelog version '$FIRST_VERSION'"
  fi
fi

if [[ -n "$ERRORS" ]]; then
  echo "CHANGELOG MONOTONICITY VIOLATION in $(basename "$FILE_PATH"):" >&2
  echo -e "$ERRORS" | while IFS= read -r line; do
    echo "  - $line" >&2
  done
  exit 2
fi

exit 0
