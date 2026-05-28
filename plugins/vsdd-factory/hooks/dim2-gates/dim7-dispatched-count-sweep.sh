#!/usr/bin/env bash
# dim7-dispatched-count-sweep.sh — D-450(b) Dim-7 sibling-sweep for anachronism pattern
#
# Closes: D-450(b) — Dim-7 sibling-sweep across ALL prior burst-log entries for
# anachronism pattern (Dim-7 cell references a pass number inconsistent with the burst).
#
# Usage: dim7-dispatched-count-sweep.sh <burst-log-path>
# Sweeps all "## Burst: F5 pass-N" sections for Dim-7 cells that reference a
# pass number GREATER than the burst's own pass-N (forward reference = anachronism).
# Exits 0 if no anachronism lines found; exits 1 with offending lines on FAIL.
# EC-009: exits 0 with message if no Burst sections found.

set -euo pipefail

# ---- help ----
if [[ "${1:-}" == "--help" ]] || [[ "${1:-}" == "-h" ]]; then
  cat <<'USAGE'
dim7-dispatched-count-sweep.sh — D-450(b) Dim-7 sibling-sweep for anachronism detection

USAGE:
  dim7-dispatched-count-sweep.sh <burst-log-path>

ARGUMENTS:
  burst-log-path   Path to burst-log.md file to sweep

EXIT CODES:
  0 — no Dim-7 anachronisms found (PASS), or no Burst sections in file
  1 — at least one Dim-7 cell contains a forward pass reference (FAIL)

ANACHRONISM PATTERN:
  A "## Burst: F5 pass-N" section's Dim-7 block contains a reference to
  "pass-M" where M > N (forward reference to a pass not yet reached at time N).

EXAMPLES:
  dim7-dispatched-count-sweep.sh burst-log.md
  dim7-dispatched-count-sweep.sh .factory/cycles/.../burst-log.md

NOTES:
  Sweeps ALL burst entries in the file (sibling-sweep per D-450(b)).
  Only forward references (M > N) are anachronisms. Back-references (M <= N)
  are legitimate (a burst may reference counts from prior bursts).
  Invoked during Dim-2 fix-burst attestation per D-449(a).
USAGE
  exit 0
fi

# ---- argument validation ----
if [[ $# -lt 1 ]]; then
  echo "ERROR: dim7-dispatched-count-sweep.sh requires 1 argument: <burst-log-path>" >&2
  echo "Run with --help for usage." >&2
  exit 1
fi

BURST_LOG="$1"

if [[ ! -f "$BURST_LOG" ]]; then
  echo "FAIL: burst-log file not found: ${BURST_LOG}" >&2
  exit 1
fi

echo "$ grep -n '## Burst: F5 pass-' ${BURST_LOG}"

# Find all burst section headings and their line numbers
BURST_HEADINGS=$(grep -n "^## Burst: F5 pass-" "$BURST_LOG" || true)

if [[ -z "$BURST_HEADINGS" ]]; then
  echo "no Burst sections found — no Dim-7 anachronisms to check"
  echo "PASS: no Burst sections found in ${BURST_LOG} — vacuously clean"
  exit 0
fi

echo "$BURST_HEADINGS"
echo "---"

TOTAL_LINES=$(wc -l < "$BURST_LOG")
FAIL=0
ANACHRONISM_LINES=()

# Process each burst section
while IFS= read -r HEADING_LINE; do
  # Extract line number and pass number
  LINE_NUM="${HEADING_LINE%%:*}"
  HEADING_TEXT="${HEADING_LINE#*:}"

  # Extract pass number from heading: "## Burst: F5 pass-N"
  BURST_PASS=$(echo "$HEADING_TEXT" | grep -oE 'pass-[0-9]+' | grep -oE '[0-9]+' | head -1)

  if [[ -z "$BURST_PASS" ]]; then
    continue
  fi

  # Find the next burst section heading (or end of file) to bound this section
  NEXT_LINE_NUM=$(echo "$BURST_HEADINGS" | awk -F: -v curr="$LINE_NUM" '$1 > curr {print $1; exit}')
  if [[ -z "$NEXT_LINE_NUM" ]]; then
    NEXT_LINE_NUM="$TOTAL_LINES"
  fi

  # Within this burst section, find Dim-7 content and scan for pass references
  # Extract lines from LINE_NUM+1 to NEXT_LINE_NUM-1
  SECTION_START=$(( LINE_NUM + 1 ))
  SECTION_END=$(( NEXT_LINE_NUM - 1 ))

  if [[ "$SECTION_START" -gt "$SECTION_END" ]]; then
    continue
  fi

  # Check for pass-M references in Dim-7 blocks within this section.
  # Extract ONLY the section's actual lines using sed, then iterate with
  # correct absolute line numbers (SECTION_START + 0-based offset).
  IN_DIM7=0
  LINE_OFFSET=0

  while IFS= read -r FILE_LINE; do
    ACTUAL_LINE_NUM=$(( SECTION_START + LINE_OFFSET ))
    LINE_OFFSET=$(( LINE_OFFSET + 1 ))

    # Track if we're in a Dim-7 block
    if echo "$FILE_LINE" | grep -qE '^\*\*Dim-7'; then
      IN_DIM7=1
    elif echo "$FILE_LINE" | grep -qE '^\*\*Dim-[^7]|\*\*Closes|\*\*Parent|\*\*Adversary|\*\*Files|\*\*Codifications'; then
      # Entering another block — exit Dim-7 context
      IN_DIM7=0
    elif echo "$FILE_LINE" | grep -qE '^## Burst'; then
      IN_DIM7=0
    fi

    # Within Dim-7 context, scan for pass-M references where M > BURST_PASS
    if [[ "$IN_DIM7" -eq 1 ]]; then
      # Find all pass-NNN references in this line
      REFS=$(echo "$FILE_LINE" | grep -oE 'pass-[0-9]+' | grep -oE '[0-9]+' || true)
      for REF in $REFS; do
        if [[ "$REF" -gt "$BURST_PASS" ]]; then
          ANACHRONISM_LINES+=("Line ${ACTUAL_LINE_NUM} (in Burst: pass-${BURST_PASS} Dim-7): ${FILE_LINE}")
          FAIL=1
        fi
      done
    fi
  done < <(sed -n "${SECTION_START},${SECTION_END}p" "$BURST_LOG")

done <<< "$BURST_HEADINGS"

echo "Burst sections swept: $(echo "$BURST_HEADINGS" | wc -l | tr -d ' ')"

if [[ "$FAIL" -eq 0 ]]; then
  echo "PASS: no Dim-7 anachronisms found in ${BURST_LOG}"
  exit 0
else
  echo ""
  echo "Anachronism lines found (${#ANACHRONISM_LINES[@]}):"
  for LINE in "${ANACHRONISM_LINES[@]}"; do
    echo "  $LINE"
  done
  echo "FAIL: Dim-7 anachronism pattern detected — forward pass references in ${BURST_LOG}"
  exit 1
fi
