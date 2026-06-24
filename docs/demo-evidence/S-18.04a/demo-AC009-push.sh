#!/usr/bin/env bash
# demo-AC009-push.sh
# Demonstrates: AC-009 — push to (local bare) remote succeeds; remote ref advances.
set -euo pipefail
source "$(dirname "$0")/demo-setup.sh"

# Stage a pending change
echo "# AC-009 push demo $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$FACTORY_ARTS/STATE.md"

REMOTE_BEFORE=$(git -C "$BARE_REMOTE" rev-parse refs/heads/factory-artifacts)
echo "=== Remote factory-artifacts ref BEFORE flush ==="
echo "remote HEAD: $REMOTE_BEFORE"
echo ""

echo "=== Running PreCompact flush ==="
ENVELOPE='{"event_name":"PreCompact","tool_name":"","session_id":"demo-push-001","dispatcher_trace_id":"demo-trace-push-001"}'
_run_dispatcher "$ENVELOPE"
echo ""

HEAD_AFTER=$(git -C "$FACTORY_ARTS" rev-parse HEAD)
REMOTE_AFTER=$(git -C "$BARE_REMOTE" rev-parse refs/heads/factory-artifacts)

echo "=== Remote factory-artifacts ref AFTER flush ==="
echo "remote HEAD: $REMOTE_AFTER"
echo ""

if [ "$REMOTE_AFTER" = "$HEAD_AFTER" ]; then
    echo "PASS: remote ref advanced to match local HEAD $HEAD_AFTER"
else
    echo "FAIL: remote $REMOTE_AFTER != local $HEAD_AFTER"
    exit 1
fi
