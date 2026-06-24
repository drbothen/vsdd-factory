#!/usr/bin/env bash
# demo-AC014-hermetic-git-only.sh
# Demonstrates: AC-014/AC-001 — hermetic: only git is invoked; no bash subprocess.
# Shows registry binary_allow=["git"] and confirms no bash in dispatcher output.
set -euo pipefail
source "$(dirname "$0")/demo-setup.sh"

# Stage a pending change
echo "# AC-014 hermetic demo $(date -u +%Y-%m-%dT%H:%M:%SZ)" >> "$FACTORY_ARTS/STATE.md"

echo "=== Registry: binary_allow for precompact-flush ==="
grep -A 20 'name = "precompact-flush"' "$WORK/hooks-registry.toml" | grep 'binary_allow'
echo ""

echo "=== Running PreCompact flush ==="
ENVELOPE='{"event_name":"PreCompact","tool_name":"","session_id":"demo-hermetic-001","dispatcher_trace_id":"demo-trace-hermetic-001"}'
OUTPUT=$(_run_dispatcher "$ENVELOPE")
echo "$OUTPUT"
echo ""

echo "=== Checking: sync_plugins=1 (plugin was invoked) ==="
if echo "$OUTPUT" | grep -q "sync_plugins=1"; then
    echo "PASS: precompact-flush WASM was invoked by dispatcher"
else
    echo "FAIL: plugin not invoked"
    exit 1
fi

echo "=== Checking: no bash subprocess in dispatcher output ==="
if echo "$OUTPUT" | grep -q '"binary":"bash"'; then
    echo "FAIL: bash subprocess detected — AC-014 violated"
    exit 1
else
    echo "PASS: no bash subprocess (binary_allow=[\"git\"] only)"
fi

echo "=== Checking: registry has git in binary_allow, not bash ==="
BINARY_ALLOW=$(grep -A 20 'name = "precompact-flush"' "$WORK/hooks-registry.toml" | grep 'binary_allow')
if echo "$BINARY_ALLOW" | grep -q '"git"' && ! echo "$BINARY_ALLOW" | grep -q '"bash"'; then
    echo "PASS: binary_allow=[\"git\"] — bash excluded per ADR-028 Decision 2"
else
    echo "FAIL: binary_allow does not match expected"
    exit 1
fi
