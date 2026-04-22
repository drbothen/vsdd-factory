#!/bin/bash
# validate-wave-gate-completeness.sh — PostToolUse hook on Write to wave-state.yaml
#
# When gate_status is changed to "passed" in wave-state.yaml, verifies that
# a gate report exists AND contains evidence of all required gate checks.
# Prevents marking a gate as passed without actually running all 6 gates.
#
# Required gate checks (from wave-gate skill):
#   Gate 1: Test Suite (cargo test / test suite)
#   Gate 2: DTU Validation (may be SKIP if no critical modules)
#   Gate 3: Adversarial Review
#   Gate 4: Demo Evidence
#   Gate 5: Holdout Evaluation
#   Gate 6: State Update
#
# Also validates GATE_CHECK telemetry lines if present.
#
# Trigger: PostToolUse on Write/Edit to .factory/wave-state.yaml.
# Exit 0 on pass (all gates evidenced, or not wave-state.yaml).
# Exit 2 blocks if gate_status: passed but evidence incomplete.
#
# Deterministic, <500ms, no LLM.
# Requires: python3 (for YAML parsing).

set -euo pipefail

if ! command -v jq &>/dev/null; then
  exit 0
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

if [[ -z "$FILE_PATH" ]] || [[ ! -f "$FILE_PATH" ]]; then
  exit 0
fi

# Only trigger for wave-state.yaml
case "$FILE_PATH" in
  *wave-state.yaml) ;;
  *) exit 0 ;;
esac

if ! command -v python3 &>/dev/null; then
  exit 0
fi

# Check if any wave was just marked as "passed"
NEWLY_PASSED=$(python3 -c "
import yaml, sys

with open('$FILE_PATH') as f:
    state = yaml.safe_load(f)

if not state or 'waves' not in state:
    sys.exit(0)

for name, data in state['waves'].items():
    if data.get('gate_status') == 'passed':
        report = data.get('gate_report')
        if not report:
            print(f'{name}:no_report')
        else:
            print(f'{name}:{report}')
" 2>/dev/null || true)

if [[ -z "$NEWLY_PASSED" ]]; then
  exit 0  # no waves with passed status
fi

ERRORS=""
FACTORY_DIR=$(dirname "$FILE_PATH")

while IFS= read -r entry; do
  [[ -z "$entry" ]] && continue
  WAVE_NAME="${entry%%:*}"
  REPORT_PATH="${entry#*:}"

  if [[ "$REPORT_PATH" == "no_report" ]]; then
    ERRORS="${ERRORS:+$ERRORS\n}$WAVE_NAME: gate_status=passed but no gate_report path specified"
    continue
  fi

  # Resolve report path relative to .factory/
  FULL_REPORT=""
  if [[ -f "$REPORT_PATH" ]]; then
    FULL_REPORT="$REPORT_PATH"
  elif [[ -f "$FACTORY_DIR/$REPORT_PATH" ]]; then
    FULL_REPORT="$FACTORY_DIR/$REPORT_PATH"
  fi

  if [[ -z "$FULL_REPORT" ]]; then
    ERRORS="${ERRORS:+$ERRORS\n}$WAVE_NAME: gate_report '$REPORT_PATH' not found"
    continue
  fi

  # Check that the gate report contains evidence of all 6 gates
  # Accept: "Gate N", "Gate N:", "GATE_CHECK: gate=N", "✅", "❌", "PASS", "SKIP"
  MISSING_GATES=""
  for gate_num in 1 2 3 4 5 6; do
    if ! grep -qiE "Gate ${gate_num}[^0-9]|GATE_CHECK:.*gate=${gate_num}" "$FULL_REPORT"; then
      case $gate_num in
        1) GATE_NAME="Test Suite" ;;
        2) GATE_NAME="DTU Validation" ;;
        3) GATE_NAME="Adversarial Review" ;;
        4) GATE_NAME="Demo Evidence" ;;
        5) GATE_NAME="Holdout Evaluation" ;;
        6) GATE_NAME="State Update" ;;
      esac
      MISSING_GATES="${MISSING_GATES:+$MISSING_GATES, }Gate $gate_num ($GATE_NAME)"
    fi
  done

  if [[ -n "$MISSING_GATES" ]]; then
    ERRORS="${ERRORS:+$ERRORS\n}$WAVE_NAME: gate report missing evidence for: $MISSING_GATES"
  fi
done <<< "$NEWLY_PASSED"

if [[ -n "$ERRORS" ]]; then
  echo "WAVE GATE COMPLETENESS VIOLATION:" >&2
  echo -e "$ERRORS" | while IFS= read -r line; do
    echo "  - $line" >&2
  done
  echo "" >&2
  echo "  All 6 gates must be evidenced in the gate report before marking passed." >&2
  echo "  Run /vsdd-factory:wave-gate to complete all gates, or set gate_status: deferred" >&2
  echo "  with a rationale if gates are intentionally skipped." >&2
  exit 2
fi

exit 0
