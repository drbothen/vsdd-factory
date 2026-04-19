#!/bin/bash
# validate-story-bc-sync.sh — PostToolUse hook for Policy 8
#
# Validates bidirectional BC completeness in story files:
# - Every BC in frontmatter behavioral_contracts: array appears in body BC table
# - Every BC in frontmatter behavioral_contracts: array has at least one AC trace
# - Every BC in body BC table appears in frontmatter behavioral_contracts: array
#
# Trigger: PostToolUse on Edit|Write to STORY-*.md files.
# Exit 0 on pass (or if story has no behavioral_contracts: field yet).
# Exit 2 on mismatch with diagnostic on stderr.
#
# Deterministic, <200ms, no LLM.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

if [[ -z "$FILE_PATH" ]] || [[ ! -f "$FILE_PATH" ]]; then
  exit 0
fi

# Only trigger for story files
case "$FILE_PATH" in
  *stories/STORY-*.md|*STORY-*-*.md) ;;
  *) exit 0 ;;
esac

# Skip index files
if [[ "$FILE_PATH" == *"STORY-INDEX"* ]]; then
  exit 0
fi

# Extract BCs from frontmatter — accepts both 'behavioral_contracts:' (canonical)
# and 'bcs:' (legacy). Template compliance hook flags wrong field name separately.
FRONTMATTER_BCS=$(awk '
  /^---$/{ fm++; next }
  fm==1 && /^(bcs|behavioral_contracts):/ {
    # Inline array: behavioral_contracts: [BC-2.01.001, BC-2.01.003]
    if (/\[/) {
      gsub(/.*\[/, "")
      gsub(/\].*/, "")
      gsub(/,/, "\n")
      gsub(/[ \t"'\''"]/, "")
      print
    }
    in_arr=1
    next
  }
  fm==1 && in_arr && /^  *- / {
    sub(/^  *- */, "")
    gsub(/[ \t"'\''"]/, "")
    print
  }
  fm==1 && in_arr && /^[^ -]/ { exit }
' "$FILE_PATH" | grep -E '^BC-' | sort -u || true)

# If no BCs in frontmatter, skip (story may be in early creation)
if [[ -z "$FRONTMATTER_BCS" ]]; then
  exit 0
fi

# Extract BCs from body BC table (look for BC-S.SS.NNN in table rows after frontmatter)
BODY_BCS=$(awk '
  /^---$/{ fm++; next }
  fm>=2 { print }
' "$FILE_PATH" | grep -oE 'BC-[0-9]+\.[0-9]+\.[0-9]+' | sort -u || true)

# Extract BCs from AC trace annotations (traces to BC-S.SS.NNN)
AC_BCS=$(awk '
  /^---$/{ fm++; next }
  fm>=2 && /traces to BC-/ { print }
' "$FILE_PATH" | grep -oE 'BC-[0-9]+\.[0-9]+\.[0-9]+' | sort -u || true)

ERRFILE=$(mktemp)
trap 'rm -f "$ERRFILE"' EXIT

# Check 1: Every BC in frontmatter appears in body BC table
for bc in $FRONTMATTER_BCS; do
  if [[ -n "$BODY_BCS" ]] && ! echo "$BODY_BCS" | grep -qxF "$bc"; then
    echo "$bc is in frontmatter behavioral_contracts: but missing from body Behavioral Contracts table" >> "$ERRFILE"
  fi
done

# Check 2: Every BC in frontmatter has at least one AC trace
for bc in $FRONTMATTER_BCS; do
  if [[ -n "$AC_BCS" ]] && ! echo "$AC_BCS" | grep -qxF "$bc"; then
    echo "$bc is in frontmatter behavioral_contracts: but has no AC trace annotation (traces to $bc)" >> "$ERRFILE"
  elif [[ -z "$AC_BCS" ]] && [[ -n "$BODY_BCS" ]]; then
    # Body exists (has BC table) but no AC traces at all
    echo "$bc is in frontmatter behavioral_contracts: but has no AC trace annotation (traces to $bc)" >> "$ERRFILE"
  fi
done

# Check 3: Every BC in body BC table appears in frontmatter
if [[ -n "$BODY_BCS" ]]; then
  for bc in $BODY_BCS; do
    if ! echo "$FRONTMATTER_BCS" | grep -qxF "$bc"; then
      echo "$bc is in body Behavioral Contracts table but missing from frontmatter behavioral_contracts: array" >> "$ERRFILE"
    fi
  done
fi

if [[ -s "$ERRFILE" ]]; then
  echo "POLICY 8 VIOLATION (bc_array_changes_propagate_to_body_and_acs):" >&2
  while IFS= read -r line; do
    echo "  - $line" >&2
  done < "$ERRFILE"
  echo "Fix: ensure frontmatter behavioral_contracts: array, body BC table, and AC traces all reference the same set of BCs." >&2
  exit 2
fi

exit 0
