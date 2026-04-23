#!/bin/bash
# validate-finding-format.sh — PostToolUse hook for finding ID format
#
# Validates that adversarial findings and fix files use current ID formats:
# - Adversarial findings: ADV-<CYCLE>-P[N]-[SEV]-NNN (not legacy ADV-NNN or ADV-P[N]-NNN)
# - Fix files: FIX-P[N]-NNN (not STORY-NNN-FIX-NNN)
#
# Trigger: PostToolUse on Write to adversarial review or fix files in .factory/.
# Exit 0 on pass (or if file is not a finding/fix file).
# Exit 2 on legacy format detected with diagnostic on stderr.
#
# Deterministic, <100ms, no LLM.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

_emit() {
  if [ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -x "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" ]; then
    "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" "$@" 2>/dev/null || true
  fi
  return 0
}

if [[ -z "$FILE_PATH" ]] || [[ ! -f "$FILE_PATH" ]]; then
  exit 0
fi

# Only trigger for adversarial review and fix files in .factory/
case "$FILE_PATH" in
  *.factory/*adversarial-review*|*.factory/*pass-*|*.factory/*FIX-*|*.factory/*fix-*) ;;
  *) exit 0 ;;
esac

ERRORS=""

# --- Check adversarial findings for legacy format ---
# Current format: ADV-<CYCLE>-P[N]-[SEV]-NNN (e.g., ADV-P1CONV-P03-CRIT-001)
# Legacy formats: ADV-NNN (no pass), ADV-P[N]-NNN (no cycle/severity)

# Extract all finding IDs from the file
FINDING_IDS=$(grep -oE 'ADV-[A-Z0-9_-]+' "$FILE_PATH" | sort -u || true)

for fid in $FINDING_IDS; do
  # Skip if it matches current format (has at least cycle + pass + severity segments)
  if echo "$fid" | grep -qE '^ADV-[A-Z0-9]+-P[0-9]+-[A-Z]+-[0-9]+$'; then
    continue
  fi
  # Legacy: ADV-NNN (just a number)
  if echo "$fid" | grep -qE '^ADV-[0-9]+$'; then
    ERRORS="${ERRORS:+$ERRORS\n}Legacy finding ID \"$fid\" — use ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ> format"
    continue
  fi
  # Legacy: ADV-P[N]-NNN (pass but no cycle/severity)
  if echo "$fid" | grep -qE '^ADV-P[0-9]+-[0-9]+$'; then
    ERRORS="${ERRORS:+$ERRORS\n}Legacy finding ID \"$fid\" — use ADV-<CYCLE>-P<PASS>-<SEV>-<SEQ> format"
    continue
  fi
done

# --- Check fix files for legacy format ---
# Current format: FIX-P[N]-NNN (e.g., FIX-P4-001)
# Legacy format: STORY-NNN-FIX-NNN

FIX_IDS=$(grep -oE 'STORY-[0-9]+-FIX-[0-9]+' "$FILE_PATH" || true)
if [[ -n "$FIX_IDS" ]]; then
  while IFS= read -r fid; do
    ERRORS="${ERRORS:+$ERRORS\n}Legacy fix ID \"$fid\" — use FIX-P<PHASE>-<SEQ> format (e.g., FIX-P4-001)"
  done <<< "$FIX_IDS"
fi

# --- Report ---
if [[ -n "$ERRORS" ]]; then
  _emit type=hook.block hook=validate-finding-format matcher=PostToolUse \
        reason=finding_id_legacy_format file_path="$FILE_PATH"
  echo "ID FORMAT VIOLATION:" >&2
  echo -e "$ERRORS" | while IFS= read -r line; do
    echo "  - $line" >&2
  done
  echo "  See FACTORY.md ID Format Reference for current formats." >&2
  exit 2
fi

exit 0
