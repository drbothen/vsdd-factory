#!/bin/bash
# validate-vp-consistency.sh — PostToolUse hook for Policy 9
#
# Validates VP-INDEX.md ↔ verification-architecture.md ↔
# verification-coverage-matrix.md consistency after any edit to these files.
#
# Language choice: bash + awk + grep. Avoids bash 4+ features (associative
# arrays) for macOS compatibility. Table parsing delegated to awk.
#
# Trigger: PostToolUse on Write or Edit to any of the three VP source files.
# Exit 0 on pass (or if file is not a VP source file).
# Exit 2 on mismatch with diagnostic on stderr.
#
# Deterministic, <200ms, no LLM.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  echo "validate-vp-consistency.sh: jq is required but not found" >&2
  exit 0
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

if [[ -z "$FILE_PATH" ]]; then
  exit 0
fi

# Only trigger for the three VP source files
case "$FILE_PATH" in
  *VP-INDEX.md|*verification-architecture.md|*verification-coverage-matrix.md) ;;
  *) exit 0 ;;
esac

# Derive the base path from the edited file
FACTORY_BASE=""
case "$FILE_PATH" in
  *specs/verification-properties/VP-INDEX.md)
    FACTORY_BASE="${FILE_PATH%/specs/verification-properties/VP-INDEX.md}" ;;
  *specs/architecture/verification-architecture.md)
    FACTORY_BASE="${FILE_PATH%/specs/architecture/verification-architecture.md}" ;;
  *specs/architecture/verification-coverage-matrix.md)
    FACTORY_BASE="${FILE_PATH%/specs/architecture/verification-coverage-matrix.md}" ;;
esac

if [[ -z "$FACTORY_BASE" ]]; then
  exit 0
fi

VP_INDEX="$FACTORY_BASE/specs/verification-properties/VP-INDEX.md"
VERIF_ARCH="$FACTORY_BASE/specs/architecture/verification-architecture.md"
COVERAGE_MATRIX="$FACTORY_BASE/specs/architecture/verification-coverage-matrix.md"

# All three files must exist for validation
for f in "$VP_INDEX" "$VERIF_ARCH" "$COVERAGE_MATRIX"; do
  if [[ ! -f "$f" ]]; then
    exit 0
  fi
done

ERRFILE=$(mktemp)
trap 'rm -f "$ERRFILE"' EXIT

# --- Extract VP IDs from VP-INDEX table rows ---
VP_IDS=$(grep -oE 'VP-[0-9]+' "$VP_INDEX" | sort -u)

if [[ -z "$VP_IDS" ]]; then
  exit 0
fi

# --- Check (a): Every VP in VP-INDEX appears in verification-architecture.md ---
for vp in $VP_IDS; do
  if ! grep -q "$vp" "$VERIF_ARCH"; then
    echo "VP $vp is in VP-INDEX but missing from verification-architecture.md" >> "$ERRFILE"
  fi
done

# --- Check (b): Every VP in VP-INDEX appears in verification-coverage-matrix.md ---
for vp in $VP_IDS; do
  if ! grep -q "$vp" "$COVERAGE_MATRIX"; then
    echo "VP $vp is in VP-INDEX but missing from verification-coverage-matrix.md" >> "$ERRFILE"
  fi
done

# --- Check (c): VP-INDEX per-tool summary totals match VP table row counts ---
# Count VPs per tool from the main VP table (rows with VP-NNN)
awk -F'|' '
  /VP-[0-9]+/ {
    gsub(/^[ \t]+|[ \t]+$/, "", $5)
    tool = tolower($5)
    if (tool != "") counts[tool]++
  }
  END {
    for (t in counts) print t, counts[t]
  }
' "$VP_INDEX" > "${ERRFILE}.tools"

# Extract declared summary totals from the Summary section
awk -F'|' '
  /[Ss]ummary/,0 {
    gsub(/^[ \t]+|[ \t]+$/, "", $2)
    gsub(/^[ \t]+|[ \t]+$/, "", $3)
    gsub(/\*/, "", $2)
    gsub(/\*/, "", $3)
    label = tolower($2)
    count = $3 + 0
    if (label ~ /kani|proptest|fuzz|mutation|integration/ && count > 0) {
      print label, count
    }
  }
' "$VP_INDEX" > "${ERRFILE}.declared"

# Compare actual vs declared
while read -r tool declared_count; do
  actual_count=$(awk -v t="$tool" '$1 == t { print $2 }' "${ERRFILE}.tools")
  actual_count=${actual_count:-0}
  if [[ "$actual_count" -ne "$declared_count" ]]; then
    echo "VP-INDEX declares $tool total = $declared_count but row count = $actual_count" >> "$ERRFILE"
  fi
done < "${ERRFILE}.declared"

rm -f "${ERRFILE}.tools" "${ERRFILE}.declared"

# --- Check (d): Coverage matrix Totals row arithmetic ---
# Sum numeric values from data rows (not header, not separator, not Totals)
ROW_SUM=$(awk -F'|' '
  /^\s*\|/ && !/---/ && !/Module/ && !/[Tt]otal/ {
    for (i=3; i<=5; i++) {
      gsub(/^[ \t]+|[ \t]+$/, "", $i)
      if ($i ~ /^[0-9]+$/) sum += $i
    }
  }
  END { print sum+0 }
' "$COVERAGE_MATRIX")

# Extract Totals row sum (numeric columns only, skip the bold formatting)
TOTALS_SUM=$(awk -F'|' '
  /[Tt]otal/ {
    for (i=3; i<=5; i++) {
      gsub(/^[ \t]+|[ \t]+$/, "", $i)
      gsub(/\*/, "", $i)
      if ($i ~ /^[0-9]+$/) sum += $i
    }
  }
  END { print sum+0 }
' "$COVERAGE_MATRIX")

if [[ "$ROW_SUM" -ne "$TOTALS_SUM" && "$TOTALS_SUM" -gt 0 ]]; then
  echo "verification-coverage-matrix.md: data row sum ($ROW_SUM) != Totals row ($TOTALS_SUM)" >> "$ERRFILE"
fi

# --- Check (e): VPs in coverage matrix exist in VP-INDEX ---
MATRIX_VPS=$(grep -oE 'VP-[0-9]+' "$COVERAGE_MATRIX" | sort -u)
for mvp in $MATRIX_VPS; do
  if ! echo "$VP_IDS" | grep -q "^${mvp}$"; then
    echo "VP $mvp referenced in verification-coverage-matrix.md but not in VP-INDEX" >> "$ERRFILE"
  fi
done

# --- Report ---
if [[ -s "$ERRFILE" ]]; then
  echo "POLICY 9 VIOLATION (vp_index_is_vp_catalog_source_of_truth):" >&2
  while IFS= read -r line; do
    echo "  - $line" >&2
  done < "$ERRFILE"
  echo "Fix: ensure VP-INDEX.md, verification-architecture.md, and verification-coverage-matrix.md are consistent." >&2
  exit 2
fi

exit 0
