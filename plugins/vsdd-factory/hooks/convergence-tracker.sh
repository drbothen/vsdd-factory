#!/bin/bash
# convergence-tracker.sh — PostToolUse hook for lightweight convergence rule enforcement
#
# Parses the guaranteed Novelty Assessment fields from adversarial review files
# and enforces convergence rules:
#
#   1. Trajectory monotonicity — finding count must not increase pass-over-pass
#   2. Minimum 3 clean passes — CONVERGENCE_REACHED requires 3 consecutive
#      passes with 0 CRIT and 0 HIGH findings
#   3. Novelty score vs verdict — CONVERGENCE_REACHED requires novelty ≤ 0.15
#   4. Zero-findings first pass — 0 findings on pass 1 is suspicious (warn)
#
# Trigger: PostToolUse on Write/Edit to adversarial review files in .factory/.
# Exit 0 on pass or warn.
# Exit 2 on convergence rule violation with diagnostic on stderr.
#
# Deterministic, <500ms (reads sibling pass files), no LLM.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

if [[ -z "$FILE_PATH" ]] || [[ ! -f "$FILE_PATH" ]]; then
  exit 0
fi

# Only trigger for adversarial review pass files
case "$FILE_PATH" in
  *.factory/*pass-[0-9]*.md) ;;
  *.factory/*adversarial-*review*.md) ;;
  *.factory/*round-[0-9]*-review*.md) ;;
  *.factory/*gemini-review*.md) ;;
  *.factory/*spec-review-pass*.md) ;;
  *) exit 0 ;;
esac

# Skip index, finding, summary, and trajectory files
case "$FILE_PATH" in
  *INDEX*.md|*FINDINGS.md|*ADV-*.md|*convergence-summary*.md|*convergence-trajectory*.md) exit 0 ;;
esac

# Must have Novelty Assessment section (validate-novelty-assessment.sh checks this)
if ! grep -q "## Novelty Assessment" "$FILE_PATH"; then
  exit 0  # let the format hook handle missing section
fi

ERRORS=""
WARNINGS=""

# --- Extract fields from the Novelty Assessment table ---

# Extract value from "| **Field** | value |" table format
_extract_field() {
  local field="$1"
  grep -i "\\*\\*${field}\\*\\*" "$FILE_PATH" | head -1 | sed 's/.*| *\*\*[^*]*\*\* *| *//' | sed 's/ *|.*//' | tr -d ' '
}

PASS=$(_extract_field "Pass")
NOVELTY_SCORE=$(_extract_field "Novelty score")
TRAJECTORY=$(_extract_field "Trajectory")
VERDICT=$(_extract_field "Verdict")
NEW_FINDINGS=$(_extract_field "New findings")
DUPLICATE_FINDINGS=$(_extract_field "Duplicate/variant findings")

# --- Extract severity counts from Summary table ---
# Look for "| CRITICAL | N |" pattern
CRIT_COUNT=$(grep -i "| *CRITICAL *|" "$FILE_PATH" | head -1 | sed 's/.*| *CRITICAL *| *//' | sed 's/ *|.*//' | tr -d ' ')
HIGH_COUNT=$(grep -i "| *HIGH *|" "$FILE_PATH" | head -1 | sed 's/.*| *HIGH *| *//' | sed 's/ *|.*//' | tr -d ' ')

# Default to empty if not found
CRIT_COUNT="${CRIT_COUNT:-}"
HIGH_COUNT="${HIGH_COUNT:-}"

# --- Rule 1: Zero-findings first pass (WARN only) ---

if [[ "$PASS" == "1" ]]; then
  TOTAL_FINDINGS=""
  if [[ -n "$NEW_FINDINGS" ]] && [[ -n "$DUPLICATE_FINDINGS" ]]; then
    TOTAL_FINDINGS=$((NEW_FINDINGS + DUPLICATE_FINDINGS))
  fi
  if [[ -n "$TOTAL_FINDINGS" ]] && [[ "$TOTAL_FINDINGS" -eq 0 ]]; then
    WARNINGS="${WARNINGS:+$WARNINGS\n}Zero findings on first adversary pass — suspicious. Re-dispatch with explicit justification requirement per CONVERGENCE.md Zero-Findings Halt protocol."
  fi
fi

# --- Rule 2: Trajectory monotonicity ---

if [[ -n "$TRAJECTORY" ]] && [[ "$TRAJECTORY" == *"→"* ]]; then
  # Parse trajectory: "29→24→21→7→4→3" → array of numbers
  IFS='→' read -ra TRAJ_NUMS <<< "$TRAJECTORY"
  PREV=""
  REGRESSION_FOUND=false
  for num in "${TRAJ_NUMS[@]}"; do
    num=$(echo "$num" | tr -d ' ')
    # Skip non-numeric entries (e.g., "CLEAN", "**3**")
    clean_num=$(echo "$num" | tr -d '*')
    if [[ "$clean_num" =~ ^[0-9]+$ ]]; then
      if [[ -n "$PREV" ]] && [[ "$clean_num" -gt "$PREV" ]]; then
        REGRESSION_FOUND=true
      fi
      PREV="$clean_num"
    fi
  done
  if [[ "$REGRESSION_FOUND" == "true" ]]; then
    WARNINGS="${WARNINGS:+$WARNINGS\n}Trajectory monotonicity violation — finding count increased in trajectory: $TRAJECTORY. Investigate root cause before continuing (CONVERGENCE.md: new scope without pre-validation? fix introduced new defect? adversary perimeter expanded?)."
  fi
fi

# --- Rule 3: Novelty score vs verdict consistency ---

if [[ "$VERDICT" == "CONVERGENCE_REACHED" ]]; then
  # Check novelty score ≤ 0.15
  if [[ -n "$NOVELTY_SCORE" ]]; then
    # Compare as integers (multiply by 100 to avoid float issues)
    SCORE_INT=$(echo "$NOVELTY_SCORE" | awk '{printf "%d", $1 * 100}')
    if [[ "$SCORE_INT" -gt 15 ]]; then
      ERRORS="${ERRORS:+$ERRORS\n}CONVERGENCE_REACHED declared but novelty score is $NOVELTY_SCORE (must be ≤ 0.15). Either the verdict is premature or the score is miscalculated."
    fi
  fi

  # Check no CRIT/HIGH findings in this pass
  if [[ -n "$CRIT_COUNT" ]] && [[ "$CRIT_COUNT" -gt 0 ]]; then
    ERRORS="${ERRORS:+$ERRORS\n}CONVERGENCE_REACHED declared but this pass has $CRIT_COUNT CRITICAL findings. Cannot converge with unresolved CRITICAL findings."
  fi
  if [[ -n "$HIGH_COUNT" ]] && [[ "$HIGH_COUNT" -gt 0 ]]; then
    ERRORS="${ERRORS:+$ERRORS\n}CONVERGENCE_REACHED declared but this pass has $HIGH_COUNT HIGH findings. Cannot converge with unresolved HIGH findings."
  fi

  # Rule 4: Minimum 3 clean passes
  # Look at sibling pass files to count consecutive clean passes
  REVIEW_DIR=$(dirname "$FILE_PATH")
  CLEAN_STREAK=0

  # Find all pass files, sorted by pass number descending
  PASS_FILES=$(find "$REVIEW_DIR" -maxdepth 1 -name 'pass-*.md' 2>/dev/null | sort -t'-' -k2 -n -r || true)

  if [[ -n "$PASS_FILES" ]]; then
    for pf in $PASS_FILES; do
      # Check if this pass has 0 CRIT and 0 HIGH
      PF_CRIT=$(grep -i "| *CRITICAL *|" "$pf" 2>/dev/null | head -1 | sed 's/.*| *CRITICAL *| *//' | sed 's/ *|.*//' | tr -d ' ')
      PF_HIGH=$(grep -i "| *HIGH *|" "$pf" 2>/dev/null | head -1 | sed 's/.*| *HIGH *| *//' | sed 's/ *|.*//' | tr -d ' ')
      PF_CRIT="${PF_CRIT:-0}"
      PF_HIGH="${PF_HIGH:-0}"

      if [[ "$PF_CRIT" -eq 0 ]] && [[ "$PF_HIGH" -eq 0 ]]; then
        CLEAN_STREAK=$((CLEAN_STREAK + 1))
      else
        break  # streak broken
      fi
    done

    if [[ "$CLEAN_STREAK" -lt 3 ]]; then
      ERRORS="${ERRORS:+$ERRORS\n}CONVERGENCE_REACHED declared but only $CLEAN_STREAK consecutive clean passes (0 CRIT, 0 HIGH). Minimum is 3. Continue iterating."
    fi
  fi
fi

# --- Report ---
if [[ -n "$WARNINGS" ]]; then
  echo "CONVERGENCE TRACKER WARNING:" >&2
  echo -e "$WARNINGS" | while IFS= read -r line; do
    echo "  ⚠ $line" >&2
  done
fi

if [[ -n "$ERRORS" ]]; then
  echo "CONVERGENCE RULE VIOLATION — BLOCKED:" >&2
  echo -e "$ERRORS" | while IFS= read -r line; do
    echo "  ✗ $line" >&2
  done
  echo "  See CONVERGENCE.md for the full quantitative criteria." >&2
  exit 2
fi

exit 0
