#!/bin/bash
# validate-anchor-capabilities-union.sh — PostToolUse hook for anchor_capabilities invariant
#
# Validates that a story's anchor_capabilities: frontmatter equals the sorted
# union of capability: fields across all referenced behavioral contracts.
#
# Example: Story anchors BC-2.04.007..012 (all capability CAP-006) but
# anchor_capabilities: [CAP-005] → block.
#
# Trigger: PostToolUse on Write/Edit to story files (stories/S-*.md or STORY-*.md).
# Exit 0 on pass (or skip conditions: no anchor_bcs, no anchor_capabilities).
# Exit 2 on mismatch (block) with diagnostic showing expected vs actual.
#
# Deterministic, <500ms (reads ~10 BC files per story), no LLM.
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

# Only trigger for story files under .factory/
case "$FILE_PATH" in
  *.factory/stories/S-*.md|*.factory/stories/STORY-*.md) ;;
  *) exit 0 ;;
esac

# Skip index files
case "$FILE_PATH" in
  *INDEX*) exit 0 ;;
esac

# --- Extract anchor_bcs (or behavioral_contracts) from frontmatter ---
ANCHOR_BCS=$(awk '
  /^---$/{ fm++; next }
  fm==1 && /^(anchor_bcs|behavioral_contracts):/ {
    if (/\[/) {
      gsub(/.*\[/, "")
      gsub(/\].*/, "")
      gsub(/,/, "\n")
      gsub(/[ \t"'"'"']/, "")
      print
    }
    in_arr=1
    next
  }
  fm==1 && in_arr && /^  *- / {
    sub(/^  *- */, "")
    gsub(/[ \t"'"'"']/, "")
    print
  }
  fm==1 && in_arr && /^[^ -]/ { exit }
' "$FILE_PATH" | grep -E '^BC-' | sort -u || true)

# If no anchor BCs, skip — no derivation possible
if [[ -z "$ANCHOR_BCS" ]]; then
  exit 0
fi

# --- Extract anchor_capabilities from frontmatter ---
ANCHOR_CAPS=$(awk '
  /^---$/{ fm++; next }
  fm==1 && /^anchor_capabilities:/ {
    if (/\[/) {
      gsub(/.*\[/, "")
      gsub(/\].*/, "")
      gsub(/,/, "\n")
      gsub(/[ \t"'"'"']/, "")
      print
    }
    in_arr=1
    next
  }
  fm==1 && in_arr && /^  *- / {
    sub(/^  *- */, "")
    gsub(/[ \t"'"'"']/, "")
    print
  }
  fm==1 && in_arr && /^[^ -]/ { exit }
' "$FILE_PATH" | grep -E '^CAP-' | sort -u || true)

# If no anchor_capabilities field, skip
if [[ -z "$ANCHOR_CAPS" ]]; then
  exit 0
fi

# --- Locate .factory/specs/behavioral-contracts/ directory ---
FACTORY_DIR="${FILE_PATH%%/stories/*}"
BC_DIR="$FACTORY_DIR/specs/behavioral-contracts"

if [[ ! -d "$BC_DIR" ]]; then
  exit 0  # No BC directory — can't resolve
fi

# --- Resolve each BC and collect capabilities ---
ALL_CAPS=""
MISSING_BCS=""
BC_CAP_MAP=""  # "BC-ID:CAP-NNN" lines for diagnostics

for bc_id in $ANCHOR_BCS; do
  # Find BC file: BC_DIR/BC-ID-*.md or BC_DIR/BC-ID.md
  BC_FILE=""
  for candidate in "$BC_DIR/${bc_id}"*.md "$BC_DIR/${bc_id}.md"; do
    if [[ -f "$candidate" ]]; then
      BC_FILE="$candidate"
      break
    fi
  done

  if [[ -z "$BC_FILE" ]]; then
    MISSING_BCS="${MISSING_BCS:+$MISSING_BCS, }$bc_id"
    continue
  fi

  # Extract capability: field from BC frontmatter
  # Handles single value: capability: CAP-006
  # Handles CSV: capability: "CAP-029, CAP-030"
  BC_CAPS=$(awk '
    /^---$/{ fm++; next }
    fm==1 && /^capability:/ {
      sub(/^capability:[ \t]*/, "")
      gsub(/["'"'"']/, "")
      gsub(/,/, "\n")
      gsub(/[ \t]/, "")
      print
      exit
    }
  ' "$BC_FILE" | grep -E '^CAP-' || true)

  if [[ -n "$BC_CAPS" ]]; then
    ALL_CAPS="${ALL_CAPS:+$ALL_CAPS
}$BC_CAPS"
    while IFS= read -r cap; do
      [[ -n "$cap" ]] && BC_CAP_MAP="${BC_CAP_MAP:+$BC_CAP_MAP
}$bc_id:$cap"
    done <<< "$BC_CAPS"
  fi
done

# Warn about missing BCs but don't block (may be new BCs not yet created)
if [[ -n "$MISSING_BCS" ]]; then
  echo "ANCHOR CAPABILITIES NOTE: BC files not found for: $MISSING_BCS (may be new)" >&2
fi

# If no capabilities found at all (all BCs missing or none have capability field), skip
if [[ -z "$ALL_CAPS" ]]; then
  exit 0
fi

# --- Compute expected vs actual ---
EXPECTED=$(echo "$ALL_CAPS" | sort -u | tr '\n' ',' | sed 's/,$//')
ACTUAL=$(echo "$ANCHOR_CAPS" | sort -u | tr '\n' ',' | sed 's/,$//')

if [[ "$EXPECTED" != "$ACTUAL" ]]; then
  echo "ANCHOR CAPABILITIES UNION VIOLATION in $(basename "$FILE_PATH"):" >&2
  echo "  Expected (from BCs): [$EXPECTED]" >&2
  echo "  Actual (frontmatter): [$ACTUAL]" >&2
  echo "  BC → CAP mapping:" >&2
  echo "$BC_CAP_MAP" | sort -u | while IFS= read -r mapping; do
    [[ -n "$mapping" ]] && echo "    $mapping" >&2
  done
  echo "  Fix: set anchor_capabilities: [$EXPECTED] in story frontmatter." >&2
  exit 2
fi

exit 0
