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

ERRFILE=$(mktemp)
# Preserve exit code through EXIT trap (Defect 2 fix)
# shellcheck disable=SC2154  # rc is assigned inside the trap via $?
trap 'rc=$?; rm -f "$ERRFILE" "${ERRFILE}.tools" "${ERRFILE}.declared" 2>/dev/null; exit $rc' EXIT

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

# --- Extract VP IDs from VP-INDEX table rows ---
VP_IDS=$(grep -oE 'VP-[0-9]+' "$VP_INDEX" | sort -u || true)

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
# Count VPs per tool from the main VP table (rows with VP-NNN).
# Normalize tool labels to snake_case so "Integration test" → "integration_test"
# and both awk blocks produce identical label shapes. (Defect 1 fix)
awk -F'|' '
  /VP-[0-9]+/ {
    gsub(/^[ \t]+|[ \t]+$/, "", $5)
    tool = tolower($5)
    gsub(/[^a-z0-9]+/, "_", tool)
    sub(/^_|_$/, "", tool)
    if (tool != "") counts[tool]++
  }
  END {
    for (t in counts) print t, counts[t]
  }
' "$VP_INDEX" > "${ERRFILE}.tools"

# Extract declared summary totals from the Summary section.
# Normalize labels identically to the above. (Defect 1 fix)
awk -F'|' '
  /[Ss]ummary/,0 {
    gsub(/^[ \t]+|[ \t]+$/, "", $2)
    gsub(/^[ \t]+|[ \t]+$/, "", $3)
    gsub(/\*/, "", $2)
    gsub(/\*/, "", $3)
    label = tolower($2)
    gsub(/[^a-z0-9]+/, "_", label)
    sub(/^_|_$/, "", label)
    count = $3 + 0
    if (label ~ /kani|proptest|fuzz|mutation|integration/ && count > 0) {
      print label, count
    }
  }
' "$VP_INDEX" > "${ERRFILE}.declared"

# Compare actual vs declared (Defect 1 + P0.2 defense-in-depth)
while read -r tool declared_count; do
  actual_count=$(awk -v t="$tool" '$1 == t { print $2 }' "${ERRFILE}.tools")
  actual_count=${actual_count:-0}
  # Guard: skip non-integer declared_count (defense-in-depth for malformed Summary)
  if ! [[ "${declared_count:-}" =~ ^[0-9]+$ ]]; then
    continue
  fi
  if [[ "$actual_count" -ne "$declared_count" ]]; then
    echo "VP-INDEX declares $tool total = $declared_count but row count = $actual_count" >> "$ERRFILE"
  fi
done < "${ERRFILE}.declared"

rm -f "${ERRFILE}.tools" "${ERRFILE}.declared"

# --- Check (d): Coverage matrix per-method column sums match VP-INDEX Summary totals ---
#
# Generalized approach: discover numeric columns from the Coverage by Module header,
# sum each column across data rows, then compare against VP-INDEX Summary per-method
# totals. Works for any verification tool names (Kani, CBMC, Hypothesis, etc.).
#
# Step 1: Extract column-name → column-sum pairs from Coverage by Module section.
# Step 2: For each column, look up the matching VP-INDEX Summary entry.
# Step 3: Compare sums.

awk -F'|' '
  # Detect the Coverage by Module header row (contains "Module" + at least one other column)
  !found_header && /Module/ && /\|.*\|.*\|/ {
    for (i=1; i<=NF; i++) {
      h = $i; gsub(/^[ \t]+|[ \t]+$/, "", h)
      # Skip non-method columns: empty, Module, Criticality, Coverage, VPs, separators
      lh = tolower(h)
      if (h == "" || lh == "module" || lh ~ /criticality/ || lh ~ /coverage/ || lh ~ /^vps?$/) continue
      # Remaining columns are verification method columns — record position and name
      col_name[i] = h
      col_count++
    }
    if (col_count > 0) found_header = 1
    next
  }
  # Skip separator rows
  found_header && /---/ { next }
  # Stop at next ## heading (exit Coverage by Module section)
  found_header && /^##/ { exit }
  # Data rows: sum each method column
  found_header && /^\|/ {
    m = $2; gsub(/^[ \t]+|[ \t]+$/, "", m)
    if (m == "" || m ~ /^\*/ || m ~ /[Tt]otal/) next
    for (i in col_name) {
      gsub(/^[ \t]+|[ \t]+$/, "", $i)
      if ($i ~ /^[0-9]+$/) sums[i] += $i
    }
  }
  END {
    for (i in col_name) {
      # Normalize column name to snake_case for matching against VP-INDEX Summary
      label = tolower(col_name[i])
      gsub(/[^a-z0-9]+/, "_", label)
      sub(/^_|_$/, "", label)
      print label, sums[i]+0
    }
  }
' "$COVERAGE_MATRIX" > "${ERRFILE}.matrix_sums"

# Helper: extract a per-method total from VP-INDEX Summary section
get_summary_total() {
  local key="$1"
  awk -F'|' -v k="$key" '
    /[Ss]ummary/,0 {
      gsub(/^[ \t]+|[ \t]+$/, "", $2); gsub(/\*/, "", $2)
      gsub(/^[ \t]+|[ \t]+$/, "", $3); gsub(/\*/, "", $3)
      label = tolower($2)
      gsub(/[^a-z0-9]+/, "_", label); sub(/^_|_$/, "", label)
      if (label == k) { print $3+0; exit }
    }
  ' "$VP_INDEX"
}

# Compare each matrix column sum against VP-INDEX Summary.
# Uses get_summary_total for each method, with partial matching for label variants
# (e.g., matrix header "Kani Proofs" → snake_case "kani_proofs", VP-INDEX Summary
# label "Kani" → snake_case "kani"). Partial match: either contains the other.
while read -r method_label matrix_sum; do
  if [[ -z "$method_label" ]]; then continue; fi
  # Try exact match first
  index_total=$(get_summary_total "$method_label")
  # If no exact match, try partial: strip trailing _proofs, _targets, _properties, etc.
  if [[ -z "$index_total" ]]; then
    short_label=$(echo "$method_label" | sed 's/_proofs$//;s/_targets$//;s/_properties$//;s/_test$//;s/_vps$//')
    index_total=$(get_summary_total "$short_label")
  fi
  if [[ -n "$index_total" ]] && [[ "$index_total" =~ ^[0-9]+$ ]] && [[ "$matrix_sum" -ne "$index_total" ]]; then
    echo "coverage-matrix: $method_label column sum ($matrix_sum) != VP-INDEX total ($index_total)" >> "$ERRFILE"
  fi
done < "${ERRFILE}.matrix_sums"

rm -f "${ERRFILE}.matrix_sums"

# --- Check (e): VPs in coverage matrix exist in VP-INDEX ---
MATRIX_VPS=$(grep -oE 'VP-[0-9]+' "$COVERAGE_MATRIX" | sort -u || true)
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
