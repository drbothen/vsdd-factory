#!/bin/bash
# session-learning.sh — Stop hook
#
# At session end, append a learning stub to `.factory/sidecar-learning.md`.
# The actual synthesis is done by the session-review skill; this hook just
# ensures a marker exists so nothing gets silently lost when sessions end
# abruptly.
#
# Non-blocking. No tool calls. Safe to fail.
#
# Ports dark-factory's sidecar-learning.ts runtime extension (partial —
# the full version captures more context, but needs API-level integration).

set -euo pipefail

STATE_DIR=".factory"
[[ ! -d "$STATE_DIR" ]] && exit 0

LEARNING_FILE="$STATE_DIR/sidecar-learning.md"
TS=$(date -u +"%Y-%m-%dT%H:%M:%SZ")

if [[ ! -f "$LEARNING_FILE" ]]; then
  {
    echo "# Sidecar Learning"
    echo ""
    echo "Session-end markers for the VSDD factory. Run /session-review to synthesize."
    echo ""
  } > "$LEARNING_FILE"
fi

echo "- Session ended at $TS (awaiting /session-review)" >> "$LEARNING_FILE"

exit 0
