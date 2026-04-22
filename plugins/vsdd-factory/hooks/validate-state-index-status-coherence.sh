#!/bin/bash
# validate-state-index-status-coherence.sh — PostToolUse hook for STATE/INDEX status sync
#
# When STATE.md or a cycles/*/INDEX.md file is edited, verifies that the
# convergence_status in STATE.md frontmatter is coherent with the **Status:**
# line in each cycle INDEX.md. Catches the recurring drift where state-manager
# updates STATE.md but the INDEX.md Status header lags behind (or vice versa).
#
# Trigger: PostToolUse on Write/Edit to .factory/STATE.md or .factory/cycles/*/INDEX.md.
# Exit 0 on pass (match, or files not found — new project).
# Exit 1 on mismatch (warning only — state transitions land at different commits).
#
# Deterministic, <500ms, no LLM.
# Compatible with bash 3.2+ (no associative arrays).

set -euo pipefail

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

if [[ -z "$FILE_PATH" ]] || [[ ! -f "$FILE_PATH" ]]; then
  exit 0
fi

# Only trigger for STATE.md or cycles/*/INDEX.md under .factory/
case "$FILE_PATH" in
  *.factory/STATE.md|*.factory/cycles/*/INDEX.md) ;;
  *) exit 0 ;;
esac

# Locate .factory/ root and STATE.md
case "$FILE_PATH" in
  *.factory/STATE.md)
    FACTORY_DIR=$(dirname "$FILE_PATH")
    STATE_FILE="$FILE_PATH"
    ;;
  *.factory/cycles/*/INDEX.md)
    FACTORY_DIR="${FILE_PATH%%/cycles/*}"
    STATE_FILE="$FACTORY_DIR/STATE.md"
    ;;
esac

if [[ ! -f "$STATE_FILE" ]]; then
  exit 0  # No STATE.md — nothing to compare
fi

# Extract convergence_status from STATE.md frontmatter
STATE_STATUS=$(awk '
  /^---$/{ fm++; next }
  fm==1 && /^convergence_status:/ {
    sub(/^convergence_status:[ \t]*/, "")
    gsub(/["'"'"']/, "")
    gsub(/^[ \t]+|[ \t]+$/, "")
    print
    exit
  }
' "$STATE_FILE")

if [[ -z "$STATE_STATUS" ]]; then
  exit 0  # No convergence_status field — skip
fi

# Normalize STATE status: replace _ with - for comparison
STATE_NORMALIZED=$(echo "$STATE_STATUS" | tr '_' '-' | tr '[:upper:]' '[:lower:]')

ERRORS=""

# Find all cycle INDEX.md files
CYCLES_DIR="$FACTORY_DIR/cycles"
if [[ ! -d "$CYCLES_DIR" ]]; then
  exit 0  # No cycles directory yet
fi

for index_file in "$CYCLES_DIR"/*/INDEX.md; do
  [[ -f "$index_file" ]] || continue

  # Extract Status line: **Status:** VALUE
  INDEX_STATUS=$(awk '/^\*\*Status:\*\*/ {
    sub(/^\*\*Status:\*\*[ \t]*/, "")
    print
    exit
  }' "$index_file" || true)

  if [[ -z "$INDEX_STATUS" ]]; then
    continue  # No Status line in this INDEX — skip
  fi

  # Trim trailing " — <description>" or " - <description>"
  # Strip from em-dash, en-dash, or hyphen separator onwards
  INDEX_PREFIX="$INDEX_STATUS"
  INDEX_PREFIX="${INDEX_PREFIX%%—*}"
  INDEX_PREFIX="${INDEX_PREFIX%%–*}"
  INDEX_PREFIX="${INDEX_PREFIX%% - *}"
  # Trim trailing whitespace
  INDEX_PREFIX="${INDEX_PREFIX%"${INDEX_PREFIX##*[![:space:]]}"}"

  # Normalize: lowercase, underscores to hyphens, trim whitespace
  INDEX_NORMALIZED=$(echo "$INDEX_PREFIX" | tr '_' '-' | tr '[:upper:]' '[:lower:]')
  INDEX_NORMALIZED="${INDEX_NORMALIZED#"${INDEX_NORMALIZED%%[![:space:]]*}"}"
  INDEX_NORMALIZED="${INDEX_NORMALIZED%"${INDEX_NORMALIZED##*[![:space:]]}"}"

  if [[ "$STATE_NORMALIZED" != "$INDEX_NORMALIZED" ]]; then
    CYCLE_NAME=$(basename "$(dirname "$index_file")")
    ERRORS="${ERRORS:+$ERRORS\n}cycles/$CYCLE_NAME/INDEX.md Status: '$INDEX_PREFIX' ≠ STATE.md convergence_status: '$STATE_STATUS'"
  fi
done

if [[ -n "$ERRORS" ]]; then
  echo "STATE/INDEX STATUS COHERENCE WARNING:" >&2
  echo -e "$ERRORS" | while IFS= read -r line; do
    echo "  - $line" >&2
  done
  echo "  State-manager often lands STATE.md update before INDEX.md in the same burst." >&2
  echo "  If this persists after the burst completes, update the lagging file." >&2
  exit 1
fi

exit 0
