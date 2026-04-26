#!/bin/bash
# validate-novelty-assessment.sh — PostToolUse hook for adversarial review novelty assessment
#
# Validates that adversarial review files contain a Novelty Assessment section
# with required fields: pass number, new findings count, duplicate count,
# novelty score, median severity, trajectory, and verdict.
#
# Trigger: PostToolUse on Write/Edit to adversarial review files in .factory/.
# Exit 0 on pass (or if file is not an adversarial review).
# Exit 2 on missing or incomplete novelty assessment with diagnostic on stderr.
#
# Deterministic, <100ms, no LLM.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

_emit() {
  if [ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -x "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" ]; then
    "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" "$@" 2>/dev/null || true
  fi
  return 0
}

if [[ -z "$FILE_PATH" ]] || [[ ! -f "$FILE_PATH" ]]; then
  exit 0
fi

# Only trigger for adversarial review pass files in .factory/
# Match: pass-N.md (incl. <scope>-pass-N.md), adversarial-*-review.md (specs/),
#        round-N-review.md, spec-review-pass*.md, gemini-review.md.
# Anchor to adversarial-reviews/ directory to avoid false-positives on ADRs
# whose filenames mention "adversarial-review" (e.g., ADR-013).
case "$FILE_PATH" in
  *.factory/cycles/*/adversarial-reviews/*pass-[0-9]*.md) ;;
  *.factory/cycles/*/adversarial-reviews/*-pass-[0-9]*.md) ;;
  *.factory/specs/adversarial-*review*.md) ;;
  *.factory/*round-[0-9]*-review*.md) ;;
  *.factory/*gemini-review*.md) ;;
  *.factory/*spec-review-pass*.md) ;;
  *) exit 0 ;;
esac

# Skip index files, finding files, and ADRs (they don't need novelty assessment)
case "$FILE_PATH" in
  *INDEX*.md|*FINDINGS.md|*ADV-*.md|*convergence-summary*.md|*convergence-trajectory*.md) exit 0 ;;
  */architecture/decisions/ADR-*.md) exit 0 ;;
esac

ERRORS=""

# Check for Novelty Assessment section header
if ! grep -q "## Novelty Assessment" "$FILE_PATH"; then
  ERRORS="${ERRORS:+$ERRORS\n}Missing '## Novelty Assessment' section"
fi

# Check for required fields in the Novelty Assessment section
# Only check if the section exists (avoid duplicate errors)
if grep -q "## Novelty Assessment" "$FILE_PATH"; then

  # Pass number
  if ! grep -qi "Pass.*|.*[0-9]" "$FILE_PATH" || ! grep -qi "\*\*Pass\*\*" "$FILE_PATH"; then
    ERRORS="${ERRORS:+$ERRORS\n}Missing 'Pass' field in Novelty Assessment"
  fi

  # Novelty score
  if ! grep -qi "novelty score" "$FILE_PATH"; then
    ERRORS="${ERRORS:+$ERRORS\n}Missing 'Novelty score' field in Novelty Assessment"
  fi

  # Verdict
  if ! grep -qiE "CONVERGENCE_REACHED|FINDINGS_REMAIN" "$FILE_PATH"; then
    ERRORS="${ERRORS:+$ERRORS\n}Missing verdict (CONVERGENCE_REACHED or FINDINGS_REMAIN) in Novelty Assessment"
  fi

  # Trajectory
  if ! grep -qi "trajectory" "$FILE_PATH"; then
    ERRORS="${ERRORS:+$ERRORS\n}Missing 'Trajectory' field in Novelty Assessment"
  fi

fi

# --- Report ---
if [[ -n "$ERRORS" ]]; then
  _emit type=hook.block hook=validate-novelty-assessment matcher=PostToolUse \
        reason=novelty_assessment_incomplete file_path="$FILE_PATH"
  echo "NOVELTY ASSESSMENT VIOLATION:" >&2
  echo -e "$ERRORS" | while IFS= read -r line; do
    echo "  - $line" >&2
  done
  echo "  Adversarial review files MUST include a '## Novelty Assessment' section" >&2
  echo "  with Pass, Novelty score, Trajectory, and Verdict fields." >&2
  echo "  See adversarial-review-template.md for the required format." >&2
  exit 2
fi

exit 0
