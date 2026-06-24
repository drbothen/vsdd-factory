#!/usr/bin/env bash
# demo-AC007-log-entry.sh
# Demonstrates: AC-007 — precompact-flush-log gets a newline-terminated
# "<ts> <SHA_B> <cycle>/<step> commit" line appended after flush.
set -euo pipefail
source "$(dirname "$0")/demo-setup.sh"

# Stage a pending change
echo "# AC-007 demo change $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$FACTORY_ARTS/STATE.md"

LOG_PATH="$FACTORY_ARTS/hooks/precompact-flush-log"
echo "=== precompact-flush-log BEFORE flush ==="
if [ -f "$LOG_PATH" ]; then cat "$LOG_PATH"; else echo "(file does not exist yet)"; fi
echo ""

echo "=== Running PreCompact flush ==="
ENVELOPE='{"event_name":"PreCompact","tool_name":"","session_id":"demo-log-001","dispatcher_trace_id":"demo-trace-log-001"}'
_run_dispatcher "$ENVELOPE"
echo ""

echo "=== precompact-flush-log AFTER flush ==="
cat "$LOG_PATH"
echo ""

HEAD_AFTER=$(git -C "$FACTORY_ARTS" rev-parse HEAD)
echo "=== Verifying log entry contains flush SHA ==="
if grep -q "$HEAD_AFTER" "$LOG_PATH"; then
    echo "PASS: log entry contains SHA $HEAD_AFTER"
    echo "PASS: format: <ISO-ts> <SHA> <cycle>/<step> commit"
else
    echo "FAIL: SHA $HEAD_AFTER not found in log"
    exit 1
fi
