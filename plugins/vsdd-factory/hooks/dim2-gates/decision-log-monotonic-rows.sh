#!/usr/bin/env bash
# decision-log-monotonic-rows.sh — D-450(e) decision-log D-NNN ascending-order enforcement
#
# Closes: D-450(e) — Decision-log D-NNN ascending-order enforcement; regex
# "^\| D-[0-9]+[( ]" covers both paren and space variants per D-451(b).
#
# Usage: decision-log-monotonic-rows.sh <decision-log-path>
# Exits 0 if all D-NNN rows are in strictly ascending (non-decreasing) order.
# Exits 1 with the first inversion pair on FAIL.
# Exits 0 with "no D-NNN rows found" if the file has no matching rows (EC-003).

set -euo pipefail

# ---- help ----
if [[ "${1:-}" == "--help" ]] || [[ "${1:-}" == "-h" ]]; then
  cat <<'USAGE'
decision-log-monotonic-rows.sh — D-450(e) decision-log D-NNN ascending-order enforcement

USAGE:
  decision-log-monotonic-rows.sh <decision-log-path>

ARGUMENTS:
  decision-log-path   Path to decision-log.md (or STATE.md decisions table)

EXIT CODES:
  0 — all D-NNN rows in strictly ascending (non-decreasing) order, or no rows found
  1 — inversion detected (D-M appears after D-N where M < N)

REGEX:
  Matches lines starting with "| D-NNN" (paren or space variant): ^\| D-[0-9]+[( ]

EXAMPLES:
  decision-log-monotonic-rows.sh decision-log.md
  decision-log-monotonic-rows.sh .factory/cycles/.../decision-log.md

NOTES:
  Multiple rows with the same D-NNN base integer (e.g., D-452(a) and D-452(b))
  are allowed — non-strict within the same D-NNN family; strict ascending applies
  across families. Invoked during Dim-2 attestation per D-449(a).
USAGE
  exit 0
fi

# ---- argument validation ----
if [[ $# -lt 1 ]]; then
  echo "ERROR: decision-log-monotonic-rows.sh requires 1 argument: <decision-log-path>" >&2
  echo "Run with --help for usage." >&2
  exit 1
fi

DECISION_LOG="$1"

if [[ ! -f "$DECISION_LOG" ]]; then
  echo "FAIL: decision-log file not found: ${DECISION_LOG}" >&2
  exit 1
fi

echo "$ grep -E '^\| D-[0-9]+[( ]' ${DECISION_LOG}"

# Extract D-NNN rows and pull out the numeric part
ROWS=$(grep -E '^\| D-[0-9]+[( ]' "$DECISION_LOG" || true)

if [[ -z "$ROWS" ]]; then
  echo "no D-NNN rows found — vacuously monotonic"
  echo "PASS: no D-NNN rows found in ${DECISION_LOG} — vacuously ascending"
  exit 0
fi

echo "$ROWS"
echo "---"

# Extract numeric values from D-NNN (ignoring sub-clauses like (a), (b))
NUMS=()
LABELS=()
while IFS= read -r LINE; do
  # Extract the number after "D-"
  NUM=$(echo "$LINE" | grep -oE 'D-[0-9]+' | head -1 | grep -oE '[0-9]+')
  if [[ -n "$NUM" ]]; then
    NUMS+=("$NUM")
    LABELS+=("$LINE")
  fi
done <<< "$ROWS"

if [[ ${#NUMS[@]} -eq 0 ]]; then
  echo "PASS: no D-NNN rows found — vacuously ascending"
  exit 0
fi

# Check strict non-decreasing order
PREV="${NUMS[0]}"
PREV_LABEL="${LABELS[0]}"
INVERSION_FOUND=0

for i in $(seq 1 $(( ${#NUMS[@]} - 1 ))); do
  CURR="${NUMS[$i]}"
  CURR_LABEL="${LABELS[$i]}"
  if [[ "$CURR" -lt "$PREV" ]]; then
    echo "FAIL: inversion detected — D-${PREV} (prev) appears before D-${CURR} (curr)"
    echo "  prev row: ${PREV_LABEL}"
    echo "  curr row: ${CURR_LABEL}"
    INVERSION_FOUND=1
    break
  fi
  PREV="$CURR"
  PREV_LABEL="$CURR_LABEL"
done

if [[ "$INVERSION_FOUND" -eq 0 ]]; then
  echo "PASS: all ${#NUMS[@]} D-NNN rows are monotonically ascending in ${DECISION_LOG}"
  exit 0
else
  exit 1
fi
