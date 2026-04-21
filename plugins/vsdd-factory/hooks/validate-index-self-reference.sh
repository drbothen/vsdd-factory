#!/bin/bash
# validate-index-self-reference.sh — PostToolUse hook for INDEX/burst-log self-reference
#
# When INDEX.md or burst-log.md in a cycle directory is edited, verifies that
# the current pass/burst (from STATE.md current_step) is referenced in the file.
# Catches the recurring defect where an agent edits the index but omits the
# row for the current burst — a self-referential gap.
#
# Trigger: PostToolUse on Write/Edit to .factory/cycles/*/INDEX.md or burst-log.md.
# Exit 0 on pass (or if file is not an index/burst-log, or STATE.md unavailable).
# Emit warning on stderr if self-reference gap detected (exit 0 — advisory).
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

# Only trigger for INDEX.md or burst-log.md under .factory/cycles/
case "$FILE_PATH" in
  *.factory/cycles/*/INDEX.md|*.factory/cycles/*/burst-log.md) ;;
  *) exit 0 ;;
esac

# Find STATE.md — walk up from the cycle dir
FACTORY_DIR=$(echo "$FILE_PATH" | sed 's|/cycles/.*||')
STATE_FILE="$FACTORY_DIR/STATE.md"

if [[ ! -f "$STATE_FILE" ]]; then
  exit 0  # No STATE.md — can't determine current step
fi

# Extract current_step from STATE.md frontmatter
CURRENT_STEP=$(awk '
  /^---$/{ fm++; next }
  fm==1 && /^current_step:/ {
    sub(/^current_step:[ \t]*/, "")
    gsub(/["'"'"']/, "")
    gsub(/^[ \t]+|[ \t]+$/, "")
    print
    exit
  }
' "$STATE_FILE")

if [[ -z "$CURRENT_STEP" ]]; then
  exit 0  # No current_step — can't check
fi

# Extract pass/burst numbers from current_step
# Patterns: "pass-72", "Pass 72", "Burst 39", "burst-39"
PASS_NUM=$(echo "$CURRENT_STEP" | grep -oiE 'pass[- ]?[0-9]+' | grep -oE '[0-9]+' | head -1 || true)
BURST_NUM=$(echo "$CURRENT_STEP" | grep -oiE 'burst[- ]?[0-9]+' | grep -oE '[0-9]+' | head -1 || true)

# Check if the edited file references the current pass/burst
FOUND=false

if [[ -n "$PASS_NUM" ]]; then
  if grep -qiE "pass[- ]?${PASS_NUM}[^0-9]|pass[- ]?${PASS_NUM}$" "$FILE_PATH"; then
    FOUND=true
  fi
fi

if [[ -n "$BURST_NUM" ]] && [[ "$FOUND" == false ]]; then
  if grep -qiE "burst[- ]?${BURST_NUM}[^0-9]|burst[- ]?${BURST_NUM}$" "$FILE_PATH"; then
    FOUND=true
  fi
fi

if [[ "$FOUND" == false ]] && [[ -n "$PASS_NUM$BURST_NUM" ]]; then
  LABEL=""
  [[ -n "$PASS_NUM" ]] && LABEL="pass-$PASS_NUM"
  [[ -n "$BURST_NUM" ]] && LABEL="${LABEL:+$LABEL / }burst-$BURST_NUM"
  echo "INDEX SELF-REFERENCE WARNING:" >&2
  echo "  $(basename "$FILE_PATH") was edited but does not reference the current step ($LABEL)." >&2
  echo "  Current step from STATE.md: '$CURRENT_STEP'" >&2
  echo "  Ensure a row/entry exists for the current pass/burst — this gap recurs across 3+ passes." >&2
  # Advisory only — don't block (the edit may be for a different purpose)
  exit 0
fi

exit 0
