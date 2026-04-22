#!/bin/bash
# warn-pending-wave-gate.sh — Stop hook (session-end safety net)
#
# At session end, checks if any wave has gate_status: pending in
# wave-state.yaml. Warns (never blocks) via stderr so the user
# knows to run the gate before starting the next session.
#
# Trigger: Stop event.
# Exit 0 always (advisory only — never blocks session end).
#
# Requires: python3 (for YAML parsing).
# Deterministic, <500ms, no LLM.

set -euo pipefail

if ! command -v python3 &>/dev/null; then
  exit 0
fi

WAVE_STATE=".factory/wave-state.yaml"
if [[ ! -f "$WAVE_STATE" ]]; then
  exit 0
fi

PENDING=$(python3 -c "
import yaml, sys

with open('$WAVE_STATE') as f:
    state = yaml.safe_load(f)

if not state or 'waves' not in state:
    sys.exit(0)

pending = []
for name, data in state['waves'].items():
    if data.get('gate_status') == 'pending':
        pending.append(name)

if pending:
    print(','.join(pending))
" 2>/dev/null || true)

if [[ -n "$PENDING" ]]; then
  echo "" >&2
  echo "WAVE GATE REMINDER:" >&2
  IFS=',' read -ra WAVES <<< "$PENDING"
  for w in "${WAVES[@]}"; do
    echo "  - $w gate is pending. Run the gate before starting the next wave." >&2
  done
  echo "" >&2
  echo "  Invoke /vsdd-factory:wave-gate or update .factory/wave-state.yaml" >&2
  echo "  with gate_status: passed (after running checks) or deferred (with rationale)." >&2
fi

exit 0
