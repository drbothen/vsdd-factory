#!/bin/bash
# validate-table-cell-count.sh — PostToolUse hook for markdown table integrity
#
# Validates that markdown table rows have the same pipe count as their header.
# Catches unescaped pipes inside cells that break table rendering.
#
# Trigger: PostToolUse on Write/Edit to .factory/**/*.md.
# Exit 0 on pass (or if file has no tables).
# Exit 2 on pipe count mismatch with diagnostic on stderr.
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

ERRORS=""
IN_TABLE=false
HEADER_PIPES=0
TABLE_START=0
LINE_NUM=0

while IFS= read -r line; do
  LINE_NUM=$((LINE_NUM + 1))

  # Count unescaped pipes: remove escaped pipes (\|) then count |
  clean="${line//\\|/}"
  pipe_count=$(echo "$clean" | tr -cd '|' | wc -c | tr -d ' ')

  # Detect table header (line with pipes, followed by separator)
  if [[ "$IN_TABLE" == false ]] && [[ "$pipe_count" -ge 2 ]] && echo "$line" | grep -q '^|'; then
    # Peek: is this a potential header? Check if it starts with |
    HEADER_PIPES=$pipe_count
    TABLE_START=$LINE_NUM
    IN_TABLE="maybe"
    continue
  fi

  # Separator line confirms we're in a table
  if [[ "$IN_TABLE" == "maybe" ]]; then
    if echo "$line" | grep -qE '^\|[-: |]+\|$'; then
      IN_TABLE=true
      continue
    else
      IN_TABLE=false
      continue
    fi
  fi

  # In a table — validate data rows
  if [[ "$IN_TABLE" == true ]]; then
    if [[ "$pipe_count" -lt 2 ]] || ! echo "$line" | grep -q '^|'; then
      # Table ended
      IN_TABLE=false
      continue
    fi
    if [[ "$pipe_count" -ne "$HEADER_PIPES" ]]; then
      ERRORS="${ERRORS:+$ERRORS\n}Line $LINE_NUM: row has $pipe_count pipes vs header $HEADER_PIPES (table starts line $TABLE_START) — possible unescaped | inside cell"
    fi
  fi
done < "$FILE_PATH"

if [[ -n "$ERRORS" ]]; then
  echo "TABLE CELL COUNT VIOLATION in $(basename "$FILE_PATH"):" >&2
  echo -e "$ERRORS" | while IFS= read -r line; do
    echo "  - $line" >&2
  done
  echo "  Escape pipes inside cells with \\| or restructure the cell content." >&2
  exit 2
fi

exit 0
