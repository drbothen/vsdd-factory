#!/usr/bin/env bash
# meta-level-ack-grep.sh — D-451(a) META-LEVEL-N acknowledgment literal-shell grep
#
# Closes: D-451(a) — META-LEVEL-N acknowledgment grep with captured cardinality
# across 4 canonical documents (burst-log, lessons-md, decision-log, state-md).
#
# Usage: meta-level-ack-grep.sh <meta-level-n> <burst-log> <lessons-md> <decision-log> <state-md>
# Exits 0 if total grep -c count across all 4 files >= 1.
# Exits 1 if total == 0 (acknowledgment absent from all 4 files).
#
# Per D-449(a): outputs literal command + exit code + raw stdout for Dim-2 attestation.

set -euo pipefail

# ---- help ----
if [[ "${1:-}" == "--help" ]] || [[ "${1:-}" == "-h" ]]; then
  cat <<'USAGE'
meta-level-ack-grep.sh — D-451(a) META-LEVEL-N acknowledgment literal-shell grep

USAGE:
  meta-level-ack-grep.sh <meta-level-n> <burst-log> <lessons-md> <decision-log> <state-md>

ARGUMENTS:
  meta-level-n   Integer N for which to search "META-LEVEL-<N> CANDIDATE CONFIRMED"
  burst-log      Path to burst-log.md
  lessons-md     Path to lessons.md
  decision-log   Path to decision-log.md
  state-md       Path to state.md (or STATE.md)

EXIT CODES:
  0 — total grep -c count across all 4 files >= 1 (acknowledgment present)
  1 — total == 0 (acknowledgment absent from all 4 files)

EXAMPLES:
  meta-level-ack-grep.sh 24 burst-log.md lessons.md decision-log.md state.md
  meta-level-ack-grep.sh 29 .factory/cycles/.../burst-log.md .factory/cycles/.../lessons.md \
    .factory/cycles/.../decision-log.md .factory/STATE.md

NOTES:
  Invoked during Dim-2 fix-burst attestation per D-449(a). Paste literal output
  (command + exit code + stdout) into burst-log Dim-2 block.
USAGE
  exit 0
fi

# ---- argument validation ----
if [[ $# -lt 5 ]]; then
  echo "ERROR: meta-level-ack-grep.sh requires 5 arguments: <meta-level-n> <burst-log> <lessons-md> <decision-log> <state-md>" >&2
  echo "Run with --help for usage." >&2
  exit 1
fi

META_LEVEL_N="$1"
BURST_LOG="$2"
LESSONS_MD="$3"
DECISION_LOG="$4"
STATE_MD="$5"

# Validate N is a positive integer
if ! [[ "$META_LEVEL_N" =~ ^[0-9]+$ ]] || [[ "$META_LEVEL_N" -lt 1 ]]; then
  echo "FAIL: META-LEVEL-N must be >= 1; got: ${META_LEVEL_N}" >&2
  exit 1
fi

SEARCH_STRING="META-LEVEL-${META_LEVEL_N} CANDIDATE CONFIRMED"
TOTAL=0

echo "$ grep -c \"${SEARCH_STRING}\" <4 files>"
echo "---"

for FILE in "$BURST_LOG" "$LESSONS_MD" "$DECISION_LOG" "$STATE_MD"; do
  if [[ ! -f "$FILE" ]]; then
    echo "ERROR: file not found: ${FILE}" >&2
    exit 1
  fi
  # grep -c returns exit 1 if no matches; capture count without failing
  COUNT=$(grep -c "${SEARCH_STRING}" "$FILE" || true)
  echo "${FILE}: ${COUNT}"
  TOTAL=$(( TOTAL + COUNT ))
done

echo "---"
echo "total: ${TOTAL}"
echo "exit code: 0"
echo "---"

if [[ "$TOTAL" -ge 1 ]]; then
  echo "PASS: META-LEVEL-${META_LEVEL_N} CANDIDATE CONFIRMED found — total count: ${TOTAL}"
  exit 0
else
  echo "FAIL: META-LEVEL-${META_LEVEL_N} CANDIDATE CONFIRMED is absent from all 4 files — total: 0"
  exit 1
fi
