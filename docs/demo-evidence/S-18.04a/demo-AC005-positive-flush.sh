#!/usr/bin/env bash
# demo-AC005-positive-flush.sh
# Demonstrates: AC-004/AC-005 positive flush — factory-artifacts HEAD advances.
# Shows git log BEFORE flush, runs dispatcher, shows git log AFTER flush.
set -euo pipefail
source "$(dirname "$0")/demo-setup.sh"

# Stage a pending change so flush has work to do
echo "# pending change $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$FACTORY_ARTS/STATE.md"

echo "=== factory-artifacts git log BEFORE flush ==="
git -C "$FACTORY_ARTS" log --oneline -3
echo ""

HEAD_BEFORE=$(git -C "$FACTORY_ARTS" rev-parse HEAD)
echo "HEAD before: $HEAD_BEFORE"
echo ""

echo "=== Sending PreCompact event to dispatcher ==="
ENVELOPE='{"event_name":"PreCompact","tool_name":"","session_id":"demo-flush-001","dispatcher_trace_id":"demo-trace-flush-001"}'
_run_dispatcher "$ENVELOPE"
echo ""

HEAD_AFTER=$(git -C "$FACTORY_ARTS" rev-parse HEAD)
echo "=== factory-artifacts git log AFTER flush ==="
git -C "$FACTORY_ARTS" log --oneline -3
echo ""
echo "HEAD before: $HEAD_BEFORE"
echo "HEAD after:  $HEAD_AFTER"

if [ "$HEAD_AFTER" != "$HEAD_BEFORE" ]; then
    echo "PASS: HEAD advanced — flush commit landed on factory-artifacts"
else
    echo "FAIL: HEAD did not advance"
    exit 1
fi
