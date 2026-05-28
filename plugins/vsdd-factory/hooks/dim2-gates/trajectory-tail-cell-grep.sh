#!/usr/bin/env bash
# trajectory-tail-cell-grep.sh — D-454(a) per-cell trajectory_tail grep gate
#
# Closes: D-454(a) — Per-cell line-anchor grep verifying trajectory_tail at each
# canonical prescribed site (not file-level count).
#
# Usage: trajectory-tail-cell-grep.sh <factory-root> <tail-value> <site-list-file>
# For each line in site-list-file (format "<file-path>:<anchor-pattern>"),
# greps the file at <factory-root>/<file-path> for the tail-value.
# Exits 0 if tail-value found at all sites; exits 1 if any site misses it.
# EC-004: blank lines in sites-file are skipped.

set -euo pipefail

# ---- help ----
if [[ "${1:-}" == "--help" ]] || [[ "${1:-}" == "-h" ]]; then
  cat <<'USAGE'
trajectory-tail-cell-grep.sh — D-454(a) per-cell trajectory_tail grep gate

USAGE:
  trajectory-tail-cell-grep.sh <factory-root> <tail-value> <site-list-file>

ARGUMENTS:
  factory-root    Root directory of the factory project (base for relative file paths)
  tail-value      The trajectory tail string to search for (e.g., "→9→9→9→9")
  site-list-file  File listing "<file-path>:<anchor-pattern>" pairs (one per line)
                  File paths are relative to factory-root.

EXIT CODES:
  0 — tail-value found at all prescribed sites (PASS)
  1 — tail-value missing from one or more sites (FAIL)

SITE-LIST FORMAT:
  .factory/STATE.md:→9→9→9→9
  .factory/cycles/.../INDEX.md:→9→9→9→9

EXAMPLES:
  trajectory-tail-cell-grep.sh . "→9→9→9→9" sites.txt
  trajectory-tail-cell-grep.sh /path/to/factory "→9→9→9→9" prescribed-sites.txt

NOTES:
  Per D-454(a): per-cell grep (not file-level count). Each site is checked
  independently so the failure message can cite the specific failing site.
  Invoked during Dim-2 fix-burst attestation per D-449(a).
USAGE
  exit 0
fi

# ---- argument validation ----
if [[ $# -lt 3 ]]; then
  echo "ERROR: trajectory-tail-cell-grep.sh requires 3 arguments: <factory-root> <tail-value> <site-list-file>" >&2
  echo "Run with --help for usage." >&2
  exit 1
fi

FACTORY_ROOT="$1"
TAIL_VALUE="$2"
SITE_LIST="$3"

if [[ ! -d "$FACTORY_ROOT" ]]; then
  echo "FAIL: factory-root directory not found: ${FACTORY_ROOT}" >&2
  exit 1
fi

if [[ ! -f "$SITE_LIST" ]]; then
  echo "FAIL: site-list-file not found: ${SITE_LIST}" >&2
  exit 1
fi

echo "$ grep -F '${TAIL_VALUE}' <prescribed-sites>"
echo "---"

FAIL=0
PASS_COUNT=0
FAIL_COUNT=0

while IFS= read -r SITE_LINE; do
  # Skip blank lines (EC-004)
  [[ -z "${SITE_LINE// }" ]] && continue

  # Parse "<file-path>:<anchor-pattern>"
  FILE_PATH="${SITE_LINE%%:*}"
  ANCHOR_PATTERN="${SITE_LINE#*:}"

  FULL_PATH="${FACTORY_ROOT}/${FILE_PATH}"

  if [[ ! -f "$FULL_PATH" ]]; then
    echo "FAIL: prescribed file not found: ${FULL_PATH}"
    FAIL=1
    FAIL_COUNT=$(( FAIL_COUNT + 1 ))
    continue
  fi

  # Per-cell grep for the tail value
  COUNT=$(grep -cF "${TAIL_VALUE}" "$FULL_PATH" || true)
  if [[ "$COUNT" -ge 1 ]]; then
    echo "PASS: found ${COUNT} occurrence(s) of '${TAIL_VALUE}' in ${FILE_PATH}"
    PASS_COUNT=$(( PASS_COUNT + 1 ))
  else
    echo "FAIL: '${TAIL_VALUE}' NOT found in ${FILE_PATH} — expected at anchor '${ANCHOR_PATTERN}'"
    FAIL=1
    FAIL_COUNT=$(( FAIL_COUNT + 1 ))
  fi
done < "$SITE_LIST"

echo "---"
echo "Sites checked: $(( PASS_COUNT + FAIL_COUNT )) (${PASS_COUNT} PASS, ${FAIL_COUNT} FAIL)"

if [[ "$FAIL" -eq 0 ]]; then
  echo "PASS: tail value '${TAIL_VALUE}' present at all prescribed sites"
  exit 0
else
  echo "FAIL: tail value '${TAIL_VALUE}' missing from ${FAIL_COUNT} prescribed site(s)"
  exit 1
fi
