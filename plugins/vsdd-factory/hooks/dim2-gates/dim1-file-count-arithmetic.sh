#!/usr/bin/env bash
# dim1-file-count-arithmetic.sh — D-450(c) Dim-1 headline cardinality vs list count arithmetic
#
# Closes: D-450(c) — Dim-1 headline cardinality vs enumerated list count arithmetic gate.
#
# Usage: dim1-file-count-arithmetic.sh <burst-log-path>
# For each headline matching "**Files touched (Dim-1): N unique files**",
# counts comma-delimited filenames on the following list line and asserts
# that the headline integer equals the list count.
# Exits 0 if all headlines match; exits 1 with per-headline expected vs actual on FAIL.

set -euo pipefail

# ---- help ----
if [[ "${1:-}" == "--help" ]] || [[ "${1:-}" == "-h" ]]; then
  cat <<'USAGE'
dim1-file-count-arithmetic.sh — D-450(c) Dim-1 headline cardinality vs list count gate

USAGE:
  dim1-file-count-arithmetic.sh <burst-log-path>

ARGUMENTS:
  burst-log-path   Path to burst-log.md file to inspect

EXIT CODES:
  0 — all Dim-1 headlines match the comma-delimited list counts (PASS)
  1 — at least one headline count differs from its list count (FAIL)

HEADLINE PATTERN:
  **Files touched (Dim-1): N unique files**

LIST FORMAT:
  Comma-delimited filenames on the line immediately following the headline.

EXAMPLES:
  dim1-file-count-arithmetic.sh burst-log.md
  dim1-file-count-arithmetic.sh .factory/cycles/.../burst-log.md

NOTES:
  Invoked during Dim-2 fix-burst attestation per D-449(a).
USAGE
  exit 0
fi

# ---- argument validation ----
if [[ $# -lt 1 ]]; then
  echo "ERROR: dim1-file-count-arithmetic.sh requires 1 argument: <burst-log-path>" >&2
  echo "Run with --help for usage." >&2
  exit 1
fi

BURST_LOG="$1"

if [[ ! -f "$BURST_LOG" ]]; then
  echo "FAIL: burst-log file not found: ${BURST_LOG}" >&2
  exit 1
fi

echo "$ grep -n 'Files touched (Dim-1)' ${BURST_LOG}"

FAIL=0
HEADLINE_COUNT=0

# Read file line by line, tracking line numbers
LINE_NUM=0
TOTAL_LINES=$(wc -l < "$BURST_LOG")

while IFS= read -r LINE; do
  LINE_NUM=$(( LINE_NUM + 1 ))

  # Match headline: **Files touched (Dim-1): N unique files**
  if echo "$LINE" | grep -qE '^\*\*Files touched \(Dim-1\): [0-9]+ unique files\*\*'; then
    HEADLINE_COUNT=$(( HEADLINE_COUNT + 1 ))

    # Extract the claimed N from the headline — specifically the N in "): N unique files"
    CLAIMED_N=$(echo "$LINE" | grep -oE '\): [0-9]+ unique' | grep -oE '[0-9]+')

    # Read the next line (the comma-delimited list)
    NEXT_LINE_NUM=$(( LINE_NUM + 1 ))
    if [[ "$NEXT_LINE_NUM" -le "$TOTAL_LINES" ]]; then
      NEXT_LINE=$(sed -n "${NEXT_LINE_NUM}p" "$BURST_LOG")
    else
      NEXT_LINE=""
    fi

    # Count comma-delimited entries in the next line
    if [[ -z "$NEXT_LINE" ]]; then
      ACTUAL_COUNT=0
    else
      # Count by splitting on commas — trim whitespace and count non-empty entries
      ACTUAL_COUNT=$(echo "$NEXT_LINE" | tr ',' '\n' | grep -cE '\S' || true)
    fi

    echo "Line ${LINE_NUM}: claimed ${CLAIMED_N} unique files"
    echo "  List line: ${NEXT_LINE}"
    echo "  Comma count: ${ACTUAL_COUNT}"

    if [[ "$CLAIMED_N" -ne "$ACTUAL_COUNT" ]]; then
      echo "  FAIL: expected ${CLAIMED_N} files in list, found ${ACTUAL_COUNT}"
      FAIL=1
    else
      echo "  PASS: headline count (${CLAIMED_N}) matches list count (${ACTUAL_COUNT})"
    fi
  fi
done < "$BURST_LOG"

echo "---"
echo "Dim-1 headlines checked: ${HEADLINE_COUNT}"

if [[ "$HEADLINE_COUNT" -eq 0 ]]; then
  echo "PASS: no Dim-1 headlines found — vacuously consistent"
  exit 0
fi

if [[ "$FAIL" -eq 0 ]]; then
  echo "PASS: all ${HEADLINE_COUNT} Dim-1 headline(s) match their comma-delimited list counts"
  exit 0
else
  echo "FAIL: one or more Dim-1 headline counts do not match their comma-delimited list lengths"
  exit 1
fi
