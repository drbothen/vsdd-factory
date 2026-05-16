#!/usr/bin/env bash
# banner-wc-l.sh — D-450(d) STATE.md banner wc-l arithmetic + dual-margin gate
#
# Closes: D-450(d) (wc-l sub-clause) — STATE.md banner "actual N lines" vs wc -l
# arithmetic gate with dual-margin verification (500 - N = margin).
#
# Usage: banner-wc-l.sh <state-md-path>
# Exits 0 if banner line count matches wc -l and margin arithmetic is correct.
# Exits 1 if either check fails.
#
# Banner format expected: "> **Factory pipeline state — actual N lines** (500 - N = M margin)"
# EC-002: exits 1 with clear message if no banner line matching the expected pattern is found.

set -euo pipefail

# ---- help ----
if [[ "${1:-}" == "--help" ]] || [[ "${1:-}" == "-h" ]]; then
  cat <<'USAGE'
banner-wc-l.sh — D-450(d) STATE.md banner wc-l arithmetic + dual-margin gate

USAGE:
  banner-wc-l.sh <state-md-path>

ARGUMENTS:
  state-md-path   Path to STATE.md file to inspect

EXIT CODES:
  0 — banner N matches wc -l count AND 500 - N equals banner margin (PASS)
  1 — mismatch detected or banner line not found (FAIL)

BANNER FORMAT:
  > **Factory pipeline state — actual N lines** (500 - N = M margin)

EXAMPLES:
  banner-wc-l.sh .factory/STATE.md
  banner-wc-l.sh fixtures/STATE.md

NOTES:
  Runs wc -l on the file and compares to the "actual N lines" figure in the banner.
  Also verifies 500 - N = M (dual-margin arithmetic).
  Invoked during Dim-2 fix-burst attestation per D-449(a).
USAGE
  exit 0
fi

# ---- argument validation ----
if [[ $# -lt 1 ]]; then
  echo "ERROR: banner-wc-l.sh requires 1 argument: <state-md-path>" >&2
  echo "Run with --help for usage." >&2
  exit 1
fi

STATE_MD="$1"

if [[ ! -f "$STATE_MD" ]]; then
  echo "FAIL: STATE.md file not found: ${STATE_MD}" >&2
  exit 1
fi

# Run wc -l and capture output
echo "$ wc -l ${STATE_MD}"
WC_OUTPUT=$(wc -l < "$STATE_MD")
WC_COUNT=$(echo "$WC_OUTPUT" | tr -d ' ')
echo "  ${WC_COUNT}"
echo "exit code: 0"
echo "---"

# Extract banner line: pattern "> **Factory pipeline state — actual N lines**"
# Also accept variants without the blockquote prefix
BANNER_LINE=$(grep -E 'actual [0-9]+ lines' "$STATE_MD" | head -1 || true)

if [[ -z "$BANNER_LINE" ]]; then
  echo "FAIL: banner wc-l line not found — expected pattern 'actual N lines' in ${STATE_MD}"
  exit 1
fi

echo "Banner line found: ${BANNER_LINE}"

# Extract N (the banner's claimed count)
BANNER_N=$(echo "$BANNER_LINE" | grep -oE 'actual [0-9]+ lines' | grep -oE '[0-9]+' | head -1)

if [[ -z "$BANNER_N" ]]; then
  echo "FAIL: could not parse N from banner line: ${BANNER_LINE}"
  exit 1
fi

# Extract margin (500 - N = M)
BANNER_MARGIN=$(echo "$BANNER_LINE" | grep -oE '500 - [0-9]+ = [0-9]+' | grep -oE '= [0-9]+' | grep -oE '[0-9]+' | head -1)

echo "Banner claims: actual ${BANNER_N} lines"
echo "wc -l reports: ${WC_COUNT} lines"

# Check 1: banner N == wc -l count
FAIL=0
if [[ "$BANNER_N" -ne "$WC_COUNT" ]]; then
  echo "FAIL: banner line count mismatch — banner says ${BANNER_N}, wc -l returns ${WC_COUNT}"
  FAIL=1
fi

# Check 2: dual-margin arithmetic (500 - N = margin), only if margin is present
if [[ -n "$BANNER_MARGIN" ]]; then
  EXPECTED_MARGIN=$(( 500 - BANNER_N ))
  echo "Dual-margin check: 500 - ${BANNER_N} = ${EXPECTED_MARGIN} (banner claims ${BANNER_MARGIN})"
  if [[ "$EXPECTED_MARGIN" -ne "$BANNER_MARGIN" ]]; then
    echo "FAIL: dual-margin arithmetic error — 500 - ${BANNER_N} = ${EXPECTED_MARGIN}, but banner claims ${BANNER_MARGIN}"
    FAIL=1
  fi
fi

if [[ "$FAIL" -eq 0 ]]; then
  echo "PASS: banner line count (${BANNER_N}) matches wc -l (${WC_COUNT}) and margin arithmetic correct"
  exit 0
else
  exit 1
fi
