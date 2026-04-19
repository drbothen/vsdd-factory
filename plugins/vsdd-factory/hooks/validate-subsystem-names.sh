#!/bin/bash
# validate-subsystem-names.sh — PostToolUse hook for Policy 6
#
# Validates that subsystem: fields in BC files and subsystems: fields in
# story files match canonical SS-NN IDs from ARCH-INDEX.md Subsystem Registry.
#
# Trigger: PostToolUse on Edit|Write to BC-*.md or STORY-*.md files.
# Exit 0 on pass (or if ARCH-INDEX doesn't exist yet).
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

# Only trigger for BC files and story files
case "$FILE_PATH" in
  *behavioral-contracts/BC-*.md|*stories/STORY-*.md) ;;
  *) exit 0 ;;
esac

# Find ARCH-INDEX.md relative to the file
FACTORY_BASE=""
if [[ "$FILE_PATH" == *"behavioral-contracts/"* ]]; then
  FACTORY_BASE="${FILE_PATH%/specs/behavioral-contracts/*}"
elif [[ "$FILE_PATH" == *"stories/"* ]]; then
  FACTORY_BASE="${FILE_PATH%/stories/*}"
fi

if [[ -z "$FACTORY_BASE" ]]; then
  exit 0
fi

ARCH_INDEX="$FACTORY_BASE/specs/architecture/ARCH-INDEX.md"

# Skip if ARCH-INDEX doesn't exist yet (architecture not produced)
if [[ ! -f "$ARCH_INDEX" ]]; then
  exit 0
fi

# Extract canonical SS-NN IDs from ARCH-INDEX Subsystem Registry
# Table format: | SS ID | Name | Architecture Doc | Implementing Modules | Phase |
# Column 2 (after leading pipe) = SS ID
CANONICAL_IDS=$(awk '
  /[Ss]ubsystem.*[Rr]egistry/ { found=1; next }
  found && /^#[^|]/ { exit }
  found && /^\|/ && !/---/ && !/SS.*ID/ {
    split($0, cols, "|")
    gsub(/^[ \t]+|[ \t]+$/, "", cols[2])
    if (cols[2] ~ /^SS-[0-9]+$/) print cols[2]
  }
' "$ARCH_INDEX")

# Also build an ID→Name map for error messages
CANONICAL_MAP=$(awk '
  /[Ss]ubsystem.*[Rr]egistry/ { found=1; next }
  found && /^#[^|]/ { exit }
  found && /^\|/ && !/---/ && !/SS.*ID/ {
    split($0, cols, "|")
    gsub(/^[ \t]+|[ \t]+$/, "", cols[2])
    gsub(/^[ \t]+|[ \t]+$/, "", cols[3])
    if (cols[2] ~ /^SS-[0-9]+$/) print cols[2] " (" cols[3] ")"
  }
' "$ARCH_INDEX")

# If no canonical IDs found, skip (ARCH-INDEX may not have registry yet)
if [[ -z "$CANONICAL_IDS" ]]; then
  exit 0
fi

ERRORS=""

# For BC files: check subsystem: frontmatter field
if [[ "$FILE_PATH" == *"BC-"* ]]; then
  # Extract subsystem from YAML frontmatter
  SUBSYSTEM=$(awk '
    /^---$/{ fm++; next }
    fm==1 && /^subsystem:/ {
      sub(/^subsystem:[ \t]*/, "")
      gsub(/^["'\'']|["'\'']$/, "")
      print
      exit
    }
  ' "$FILE_PATH")

  if [[ -n "$SUBSYSTEM" ]]; then
    if ! echo "$CANONICAL_IDS" | grep -qxF "$SUBSYSTEM"; then
      ERRORS="BC file subsystem: \"$SUBSYSTEM\" does not match any SS-ID in ARCH-INDEX Subsystem Registry."
    fi
  fi
fi

# For story files: check subsystems: frontmatter field (YAML array)
if [[ "$FILE_PATH" == *"STORY-"* ]]; then
  # Extract subsystems from YAML frontmatter (handles both inline and multi-line arrays)
  STORY_SUBSYSTEMS=$(awk '
    /^---$/{ fm++; next }
    fm==1 && /^subsystems:/ {
      # Inline array: subsystems: [Name1, Name2]
      if (/\[/) {
        gsub(/.*\[/, "")
        gsub(/\].*/, "")
        gsub(/,/, "\n")
        gsub(/["'\'']/, "")
        print
      }
      # Multi-line array starts on next line
      in_arr=1
      next
    }
    fm==1 && in_arr && /^  *- / {
      sub(/^  *- */, "")
      gsub(/["'\'']/, "")
      print
    }
    fm==1 && in_arr && /^[^ -]/ { exit }
  ' "$FILE_PATH" | sed 's/^[ \t]*//;s/[ \t]*$//' | grep -v '^$')

  if [[ -n "$STORY_SUBSYSTEMS" ]]; then
    while IFS= read -r ss; do
      if [[ -n "$ss" ]] && ! echo "$CANONICAL_IDS" | grep -qxF "$ss"; then
        ERRORS="${ERRORS:+$ERRORS\n}Story subsystems: \"$ss\" does not match any SS-ID in ARCH-INDEX Subsystem Registry."
      fi
    done <<< "$STORY_SUBSYSTEMS"
  fi
fi

if [[ -n "$ERRORS" ]]; then
  echo "POLICY 6 VIOLATION (architecture_is_subsystem_name_source_of_truth):" >&2
  echo -e "$ERRORS" | while IFS= read -r line; do
    echo "  - $line" >&2
  done
  echo "Fix: use SS-NN IDs from ARCH-INDEX.md Subsystem Registry. Available:" >&2
  echo "$CANONICAL_MAP" | while IFS= read -r entry; do
    echo "    - $entry" >&2
  done
  exit 2
fi

exit 0
