#!/usr/bin/env bash
# block-label-canonical-form.sh — D-454(d) tri-way canonical block label form alignment
#
# Closes: D-454(d) — Tri-way canonical form alignment: codification text == regex ==
# document headers (using D-444(c) canonical block types).
#
# Usage: block-label-canonical-form.sh <burst-log-path>
# Exits 0 if all 9 D-444(c) canonical block labels are present.
# Exits 1 with list of missing labels if any are absent.
#
# The 9 canonical D-444(c) block labels:
#   Parent-commit, Adversary verdict, Files touched, Codifications,
#   Dim-2, Dim-5, Dim-6, Dim-7, Closes

set -euo pipefail

# ---- help ----
if [[ "${1:-}" == "--help" ]] || [[ "${1:-}" == "-h" ]]; then
  cat <<'USAGE'
block-label-canonical-form.sh — D-454(d) canonical block label presence gate

USAGE:
  block-label-canonical-form.sh <burst-log-path>

ARGUMENTS:
  burst-log-path   Path to burst-log.md file to inspect

EXIT CODES:
  0 — all 9 D-444(c) canonical block labels present (PASS)
  1 — one or more labels missing (FAIL)

CANONICAL LABELS (D-444(c)):
  1. Parent-commit
  2. Adversary verdict
  3. Files touched
  4. Codifications
  5. Dim-2
  6. Dim-5
  7. Dim-6
  8. Dim-7
  9. Closes

EXAMPLES:
  block-label-canonical-form.sh burst-log.md
  block-label-canonical-form.sh .factory/cycles/.../burst-log.md

NOTES:
  Each label is matched as a bold markdown header (**Label...**).
  Invoked during Dim-2 fix-burst attestation per D-449(a).
USAGE
  exit 0
fi

# ---- argument validation ----
if [[ $# -lt 1 ]]; then
  echo "ERROR: block-label-canonical-form.sh requires 1 argument: <burst-log-path>" >&2
  echo "Run with --help for usage." >&2
  exit 1
fi

BURST_LOG="$1"

if [[ ! -f "$BURST_LOG" ]]; then
  echo "FAIL: burst-log file not found: ${BURST_LOG}" >&2
  exit 1
fi

# D-444(c) canonical block labels (9 total)
LABELS=(
  "Parent-commit"
  "Adversary verdict"
  "Files touched"
  "Codifications"
  "Dim-2"
  "Dim-5"
  "Dim-6"
  "Dim-7"
  "Closes"
)

MISSING=()
FOUND=()

for LABEL in "${LABELS[@]}"; do
  if grep -q "^\*\*${LABEL}" "$BURST_LOG"; then
    FOUND+=("$LABEL")
  else
    MISSING+=("$LABEL")
  fi
done

echo "$ grep -E '^\*\*(Parent-commit|Adversary verdict|Files touched|Codifications|Dim-2|Dim-5|Dim-6|Dim-7|Closes)' ${BURST_LOG}"
echo "Found labels (${#FOUND[@]} of 9):"
for L in "${FOUND[@]}"; do
  echo "  FOUND: $L"
done

if [[ ${#MISSING[@]} -eq 0 ]]; then
  echo "PASS: all 9 D-444(c) canonical block labels present in ${BURST_LOG}"
  exit 0
else
  echo ""
  echo "Missing labels (${#MISSING[@]}):"
  for L in "${MISSING[@]}"; do
    echo "  MISSING: $L"
  done
  echo "FAIL: ${#MISSING[@]} canonical block label(s) missing from ${BURST_LOG}"
  exit 1
fi
