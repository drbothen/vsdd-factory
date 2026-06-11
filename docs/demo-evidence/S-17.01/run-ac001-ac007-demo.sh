#!/usr/bin/env bash
# run-ac001-ac007-demo.sh — demonstrate AC-001 and AC-007
# AC-001: factory_lock block written with all 3 fields on acquire
# AC-007: expires_at - locked_at == 2700 seconds exactly

set -euo pipefail

WORKTREE=/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-17.01
HELPER="$WORKTREE/plugins/vsdd-factory/bin/factory-lock-write.sh"

echo "=== Creating fixture STATE.md ==="
FIXTURE=$(mktemp /tmp/demo-state.XXXXXX)
cat > "$FIXTURE" << 'FIXTURE_EOF'
---
document_type: state
version: "1.0"
phase: test
---

# Factory STATE
FIXTURE_EOF
echo "Fixture: $FIXTURE"
echo ""

echo "=== AC-001/AC-007: factory-lock-write.sh acquire ==="
bash "$HELPER" acquire "$FIXTURE"
echo ""

echo "=== Resulting factory_lock block in frontmatter ==="
grep -A5 "factory_lock" "$FIXTURE"
echo ""

echo "=== Verify expires_at - locked_at == 2700s (Invariant 2) ==="
LOCKED_AT=$(grep "locked_at:" "$FIXTURE" | sed 's/.*"\(.*\)".*/\1/')
EXPIRES_AT=$(grep "expires_at:" "$FIXTURE" | sed 's/.*"\(.*\)".*/\1/')
echo "locked_at:  $LOCKED_AT"
echo "expires_at: $EXPIRES_AT"
LE=$(date -j -f "%Y-%m-%dT%H:%M:%SZ" "$LOCKED_AT" +%s)
EE=$(date -j -f "%Y-%m-%dT%H:%M:%SZ" "$EXPIRES_AT" +%s)
DIFF=$(( EE - LE ))
echo "TTL = ${DIFF}s (expected: 2700)"
if [[ "$DIFF" -eq 2700 ]]; then
  echo "PASS: TTL is exactly 2700 seconds"
else
  echo "FAIL: TTL mismatch"
  exit 1
fi

rm -f "$FIXTURE"
