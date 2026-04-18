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
# (Defect 3 fix — replaced generic $3..$5 with per-method named column extraction)
#
# Coverage matrix header: | Module | Criticality | Kani Proofs | Proptest | Fuzz Targets | Coverage Target | VPs |
# With -F'|' and leading pipe: $2=Module, $3=Criticality, $4=Kani, $5=Proptest, $6=Fuzz
#
# Data rows: any row where $2 is a module name (not header/separator/summary).
# We identify data rows by: starts with |, not a separator (---), not a header
# (Module/Method/Total/Count/Gap/Invariant/BC), and $2 is non-empty after trim.

# Detect column positions for Kani/Proptest/Fuzz from the header row.
# Handles both 5-column (| Module | Kani | Proptest | Fuzz | VPs |) and
# 7-column (| Module | Criticality | Kani Proofs | Proptest | Fuzz Targets | Coverage | VPs |) formats.
SUMS=$(awk -F'|' '
  # Find the header row containing Kani to determine column positions
  !found_header && /[Kk]ani/ {
    for (i=1; i<=NF; i++) {
      h = $i; gsub(/^[ \t]+|[ \t]+$/, "", h); h = tolower(h)
      if (h ~ /kani/) kani_col = i
      if (h ~ /proptest/) proptest_col = i
      if (h ~ /fuzz/) fuzz_col = i
    }
    found_header = 1
    next
  }
  # Skip separator rows
  found_header && /^[|].*---/ { next }
  # Stop at next ## heading
  found_header && /^##/ { exit }
  # Data rows
  found_header && /^\|/ {
    m = $2; gsub(/^[ \t]+|[ \t]+$/, "", m)
    if (m == "" || m ~ /^\*/ || m ~ /[Tt]otal/) next
    if (kani_col) { gsub(/^[ \t]+|[ \t]+$/, "", $kani_col); if ($kani_col ~ /^[0-9]+$/) kani += $kani_col }
    if (proptest_col) { gsub(/^[ \t]+|[ \t]+$/, "", $proptest_col); if ($proptest_col ~ /^[0-9]+$/) proptest += $proptest_col }
    if (fuzz_col) { gsub(/^[ \t]+|[ \t]+$/, "", $fuzz_col); if ($fuzz_col ~ /^[0-9]+$/) fuzz += $fuzz_col }
  }
  END { print kani+0, proptest+0, fuzz+0 }
' "$COVERAGE_MATRIX")

read -r MATRIX_KANI MATRIX_PROPTEST MATRIX_FUZZ <<< "$SUMS"

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

INDEX_KANI=$(get_summary_total kani)
INDEX_PROPTEST=$(get_summary_total proptest)
INDEX_FUZZ=$(get_summary_total fuzz)

if [[ -n "$INDEX_KANI" && "$MATRIX_KANI" -ne "$INDEX_KANI" ]]; then
  echo "coverage-matrix: Kani column sum ($MATRIX_KANI) != VP-INDEX Kani total ($INDEX_KANI)" >> "$ERRFILE"
fi
if [[ -n "$INDEX_PROPTEST" && "$MATRIX_PROPTEST" -ne "$INDEX_PROPTEST" ]]; then
  echo "coverage-matrix: Proptest column sum ($MATRIX_PROPTEST) != VP-INDEX Proptest total ($INDEX_PROPTEST)" >> "$ERRFILE"
fi
if [[ -n "$INDEX_FUZZ" && "$MATRIX_FUZZ" -ne "$INDEX_FUZZ" ]]; then
  echo "coverage-matrix: Fuzz column sum ($MATRIX_FUZZ) != VP-INDEX Fuzz total ($INDEX_FUZZ)" >> "$ERRFILE"
fi

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
