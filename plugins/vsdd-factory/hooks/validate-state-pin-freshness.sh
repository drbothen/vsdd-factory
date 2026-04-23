#!/bin/bash
# validate-state-pin-freshness.sh — PostToolUse hook for STATE.md version pin freshness
#
# When STATE.md is edited, verifies that version pins in frontmatter
# match the actual artifact file versions. Catches stale pins from agents
# dispatched out of order or citing versions before state-manager runs last.
#
# Trigger: PostToolUse on Write/Edit to .factory/STATE.md.
# Exit 0 on pass (or if file is not STATE.md).
# Exit 2 on version pin mismatch with diagnostic on stderr.
#
# Deterministic, <500ms (reads artifact files), no LLM.
# Compatible with bash 3.2+ (no associative arrays).

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

# Only trigger for STATE.md
case "$FILE_PATH" in
  */.factory/STATE.md|*.factory/STATE.md) ;;
  *) exit 0 ;;
esac

FACTORY_DIR=$(dirname "$FILE_PATH")
ERRORS=""

# Check each known version pin field against its artifact
# Format: "field_name:relative/path/to/artifact.md"
VERSION_PAIRS="
bc_index_version:specs/behavioral-contracts/BC-INDEX.md
story_index_version:stories/STORY-INDEX.md
vp_index_version:specs/verification-properties/VP-INDEX.md
test_vectors_version:specs/prd-supplements/test-vectors.md
arch_index_version:specs/architecture/ARCH-INDEX.md
"

for pair in $VERSION_PAIRS; do
  field="${pair%%:*}"
  relpath="${pair#*:}"

  # Extract cited version from STATE.md frontmatter
  CITED=$(awk -v f="$field" '
    /^---$/{ fm++; next }
    fm==1 && $0 ~ "^"f":" {
      sub(/^[^:]+:[ \t]*/, "")
      gsub(/["'"'"']/, "")
      gsub(/^[ \t]+|[ \t]+$/, "")
      gsub(/^v/, "")
      print
      exit
    }
  ' "$FILE_PATH")

  if [[ -z "$CITED" ]]; then
    continue
  fi

  ARTIFACT="$FACTORY_DIR/$relpath"
  if [[ ! -f "$ARTIFACT" ]]; then
    continue
  fi

  ACTUAL=$(awk '
    /^---$/{ fm++; next }
    fm==1 && /^version:/ {
      sub(/^version:[ \t]*/, "")
      gsub(/["'"'"']/, "")
      gsub(/^[ \t]+|[ \t]+$/, "")
      gsub(/^v/, "")
      print
      exit
    }
  ' "$ARTIFACT")

  if [[ -n "$ACTUAL" ]] && [[ "$CITED" != "$ACTUAL" ]]; then
    ERRORS="${ERRORS:+$ERRORS\n}$field: STATE.md cites '$CITED' but $(basename "$ARTIFACT") has version '$ACTUAL'"
  fi
done

if [[ -n "$ERRORS" ]]; then
  _emit type=hook.block hook=validate-state-pin-freshness matcher=PostToolUse \
        reason=state_version_pin_drift file_path="$FILE_PATH"
  echo "STATE.md VERSION PIN DRIFT:" >&2
  echo -e "$ERRORS" | while IFS= read -r line; do
    echo "  - $line" >&2
  done
  echo "  Ensure state-manager runs LAST in every burst to capture final versions." >&2
  exit 2
fi

exit 0
