#!/bin/bash
# validate-template-compliance.sh (bin/) — VP multi-BC source_bc convention check
#
# Extends template compliance validation with VP-specific enforcement:
# When a VP file has a `bcs:` array with >1 entry, the file MUST also have
# a non-empty `source_bc:` field that names the primary BC being exercised.
#
# Rule: IF len(bcs) > 1 THEN source_bc MUST be non-empty AND source_bc MUST appear in bcs[].
#
# Scope: VP-*.md files or files with document_type: verification-property.
# Other file types are passed through without checking.
#
# Exit 0 on pass or if not a VP file.
# Exit 2 on violation with structured warning to stderr.
#
# S-7.02 / BC-7.05.003
# Multi-BC source_bc convention established in commit 7765573.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

if [[ -z "$FILE_PATH" ]] || [[ ! -f "$FILE_PATH" ]]; then
  exit 0
fi

# Scope: VP-*.md files or document_type: verification-property
BASENAME=$(basename "$FILE_PATH")
IS_VP=0
case "$BASENAME" in
  VP-*.md) IS_VP=1 ;;
esac

if [[ "$IS_VP" -eq 0 ]]; then
  # Check document_type frontmatter
  DOC_TYPE=$(awk '
    /^---$/{ fm++; next }
    fm==1 && /^document_type:/ {
      sub(/^document_type:[ \t]*/, "")
      gsub(/["'"'"']/, "")
      gsub(/^[ \t]+|[ \t]+$/, "")
      print
      exit
    }
  ' "$FILE_PATH")
  if [[ "$DOC_TYPE" == "verification-property" ]]; then
    IS_VP=1
  fi
fi

if [[ "$IS_VP" -eq 0 ]]; then
  exit 0
fi

# Extract VP ID for error messages
VP_ID="$BASENAME"
if [[ "$BASENAME" =~ (VP-[0-9]+) ]]; then
  VP_ID="${BASH_REMATCH[1]}"
fi

# Extract `bcs:` array entries from frontmatter (YAML list format)
# Supports both: bcs: [BC-A, BC-B] and multi-line list
BCS_COUNT=0
BCS_LIST=()
IN_BCS=0

while IFS= read -r line; do
  # Detect end of frontmatter
  if [[ "$IN_BCS" -eq -1 ]]; then
    break
  fi

  # Single-line array: bcs: [BC-5.01.001, BC-5.01.002]
  if [[ "$line" =~ ^bcs:[[:space:]]*\[(.+)\] ]]; then
    IFS=',' read -ra entries <<< "${BASH_REMATCH[1]}"
    for entry in "${entries[@]}"; do
      entry=$(echo "$entry" | tr -d '[:space:]"'"'"'')
      [[ -n "$entry" ]] && BCS_LIST+=("$entry") && ((BCS_COUNT++))
    done
    break
  fi

  # Multi-line list start: bcs:
  if [[ "$line" =~ ^bcs:[[:space:]]*$ ]]; then
    IN_BCS=1
    continue
  fi

  # Multi-line list items: "  - BC-5.01.001"
  if [[ "$IN_BCS" -eq 1 ]]; then
    if [[ "$line" =~ ^[[:space:]]*-[[:space:]]*(BC-[0-9.]+) ]]; then
      BCS_LIST+=("${BASH_REMATCH[1]}")
      ((BCS_COUNT++))
    elif [[ "$line" =~ ^[a-z] ]]; then
      IN_BCS=0  # Another frontmatter key — end of bcs list
    fi
  fi
done < <(awk '/^---$/{fm++; if(fm==2) exit} fm==1{print}' "$FILE_PATH")

# If bcs list has 0 or 1 entries, multi-BC rule does not apply — pass
if [[ "$BCS_COUNT" -le 1 ]]; then
  exit 0
fi

# Extract source_bc field from frontmatter
SOURCE_BC=$(awk '
  /^---$/{ fm++; next }
  fm==1 && /^source_bc:/ {
    sub(/^source_bc:[ \t]*/, "")
    gsub(/["'"'"']/, "")
    gsub(/^[ \t]+|[ \t]+$/, "")
    print
    exit
  }
' "$FILE_PATH")

# Validate: source_bc must be non-empty
if [[ -z "$SOURCE_BC" ]]; then
  echo "TEMPLATE COMPLIANCE WARNING: ${VP_ID} has multiple bcs[] (${BCS_COUNT} entries) but missing source_bc field." >&2
  echo "  Convention: when len(bcs) > 1, source_bc must name the primary BC being exercised." >&2
  echo "  Ref: commit 7765573 (VP multi-BC source_bc convention)." >&2
  exit 2
fi

# Validate: source_bc must appear in bcs[] list
FOUND=0
for bc in "${BCS_LIST[@]}"; do
  if [[ "$bc" == "$SOURCE_BC" ]]; then
    FOUND=1
    break
  fi
done

if [[ "$FOUND" -eq 0 ]]; then
  echo "TEMPLATE COMPLIANCE WARNING: ${VP_ID} source_bc='${SOURCE_BC}' is not in bcs[] list." >&2
  echo "  source_bc must be one of: ${BCS_LIST[*]}" >&2
  exit 2
fi

exit 0
