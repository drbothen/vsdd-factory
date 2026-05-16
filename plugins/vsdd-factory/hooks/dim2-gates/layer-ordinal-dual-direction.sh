#!/usr/bin/env bash
# layer-ordinal-dual-direction.sh — D-452(b) layer-N dual-direction sweep
#
# Closes: D-452(b) — Layer-N dual-direction sweep: positive Nth-layer confirmation
# + negative (N-1)th/(N+1)th drift detection.
#
# Usage: layer-ordinal-dual-direction.sh <layer-n> <file1> [file2 ...]
# For the given N:
#   (a) Scans for positive form "<N>th-layer" (informational — does not affect exit code)
#   (b) Scans for drift classes "<N-1>th-layer" and "<N+1>th-layer" (failure on any match)
# Exits 0 if no drift-class occurrences found.
# Exits 1 if any drift-class occurrence is found.
#
# EC-007: N=1 — only checks 2nd-layer as +1 drift; skips 0th-layer check (not a valid ordinal).

set -euo pipefail

# ---- help ----
if [[ "${1:-}" == "--help" ]] || [[ "${1:-}" == "-h" ]]; then
  cat <<'USAGE'
layer-ordinal-dual-direction.sh — D-452(b) layer-N dual-direction sweep

USAGE:
  layer-ordinal-dual-direction.sh <layer-n> <file1> [file2 ...]

ARGUMENTS:
  layer-n   Integer N — the layer ordinal being validated (must be >= 1)
  file1+    One or more files to scan

EXIT CODES:
  0 — no drift-class (N-1 or N+1) occurrences found (PASS)
  1 — at least one drift-class occurrence found (FAIL)

EXAMPLES:
  layer-ordinal-dual-direction.sh 42 lessons.md
  layer-ordinal-dual-direction.sh 29 burst-log.md lessons.md decision-log.md

NOTES:
  Positive occurrences of Nth-layer are informational only (reported but do not
  affect exit code). Only N-1 or N+1 drift classes cause failure.
  When N=1, the 0th-layer check is skipped (not a valid ordinal form); only
  2nd-layer is checked as the +1 drift class.
  Invoked during Dim-2 fix-burst attestation per D-449(a).
USAGE
  exit 0
fi

# ---- argument validation ----
if [[ $# -lt 2 ]]; then
  echo "ERROR: layer-ordinal-dual-direction.sh requires at least 2 arguments: <layer-n> <file1> [file2 ...]" >&2
  echo "Run with --help for usage." >&2
  exit 1
fi

LAYER_N="$1"
shift
FILES=("$@")

# Validate N is a positive integer >= 1
if ! [[ "$LAYER_N" =~ ^[0-9]+$ ]] || [[ "$LAYER_N" -lt 1 ]]; then
  echo "FAIL: <layer-n> must be a positive integer >= 1; got: ${LAYER_N}" >&2
  exit 1
fi

LAYER_PREV=$(( LAYER_N - 1 ))
LAYER_NEXT=$(( LAYER_N + 1 ))

# Ordinal suffix helper
ordinal_suffix() {
  local n="$1"
  local mod=$(( n % 100 ))
  if [[ $mod -ge 11 ]] && [[ $mod -le 13 ]]; then
    echo "${n}th"
  else
    case $(( n % 10 )) in
      1) echo "${n}st" ;;
      2) echo "${n}nd" ;;
      3) echo "${n}rd" ;;
      *) echo "${n}th" ;;
    esac
  fi
}

POSITIVE_FORM="$(ordinal_suffix "$LAYER_N")-layer"
DRIFT_PREV_FORM=""
if [[ "$LAYER_N" -gt 1 ]]; then
  DRIFT_PREV_FORM="$(ordinal_suffix "$LAYER_PREV")-layer"
fi
DRIFT_NEXT_FORM="$(ordinal_suffix "$LAYER_NEXT")-layer"

echo "$ grep -n '${POSITIVE_FORM}' <files> (positive sweep — informational)"
echo "$ grep -n '${DRIFT_NEXT_FORM}' <files> (drift +1 sweep — failure)"
if [[ -n "$DRIFT_PREV_FORM" ]]; then
  echo "$ grep -n '${DRIFT_PREV_FORM}' <files> (drift -1 sweep — failure)"
fi
echo "---"

DRIFT_FOUND=0
DRIFT_LINES=()

for FILE in "${FILES[@]}"; do
  if [[ ! -f "$FILE" ]]; then
    echo "ERROR: file not found: ${FILE}" >&2
    exit 1
  fi

  # Positive sweep (informational only)
  POS_COUNT=$(grep -c "$POSITIVE_FORM" "$FILE" || true)
  echo "positive (${POSITIVE_FORM}) in ${FILE}: ${POS_COUNT} occurrence(s)"
  if [[ "$POS_COUNT" -gt 0 ]]; then
    grep -n "$POSITIVE_FORM" "$FILE" | while IFS= read -r LINE; do
      echo "  [info] ${FILE}:${LINE}"
    done || true
  fi

  # Drift +1 sweep (failure trigger)
  NEXT_COUNT=$(grep -c "$DRIFT_NEXT_FORM" "$FILE" || true)
  if [[ "$NEXT_COUNT" -gt 0 ]]; then
    echo "DRIFT DETECTED (${DRIFT_NEXT_FORM}) in ${FILE}: ${NEXT_COUNT} occurrence(s)"
    while IFS= read -r DRIFT_LINE; do
      DRIFT_LINES+=("[drift+1] ${FILE}:${DRIFT_LINE}")
    done < <(grep -n "$DRIFT_NEXT_FORM" "$FILE" || true)
    DRIFT_FOUND=1
  fi

  # Drift -1 sweep (failure trigger, only if N > 1)
  if [[ -n "$DRIFT_PREV_FORM" ]]; then
    PREV_COUNT=$(grep -c "$DRIFT_PREV_FORM" "$FILE" || true)
    if [[ "$PREV_COUNT" -gt 0 ]]; then
      echo "DRIFT DETECTED (${DRIFT_PREV_FORM}) in ${FILE}: ${PREV_COUNT} occurrence(s)"
      while IFS= read -r DRIFT_LINE; do
        DRIFT_LINES+=("[drift-1] ${FILE}:${DRIFT_LINE}")
      done < <(grep -n "$DRIFT_PREV_FORM" "$FILE" || true)
      DRIFT_FOUND=1
    fi
  fi
done

echo "---"

if [[ "$DRIFT_FOUND" -eq 0 ]]; then
  echo "PASS: no drift-class occurrences found — layer-${LAYER_N} sweep is clean"
  exit 0
else
  echo "Offending drift lines:"
  for LINE in "${DRIFT_LINES[@]}"; do
    echo "  $LINE"
  done
  echo "FAIL: drift-class occurrences detected in layer-${LAYER_N} sweep"
  exit 1
fi
