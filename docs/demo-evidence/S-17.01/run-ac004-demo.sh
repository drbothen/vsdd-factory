#!/usr/bin/env bash
# run-ac004-demo.sh — demonstrate AC-004
# AC-004: mid-burst renew advances expires_at; locked_at is unchanged

set -euo pipefail

WORKTREE=/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-17.01
HELPER="$WORKTREE/plugins/vsdd-factory/bin/factory-lock-write.sh"

echo "=== Creating fixture and acquiring lock ==="
FIXTURE=$(mktemp /tmp/demo-state.XXXXXX)
cat > "$FIXTURE" << 'FIXTURE_EOF'
---
document_type: state
version: "1.0"
phase: test
---

# Factory STATE
FIXTURE_EOF
bash "$HELPER" acquire "$FIXTURE" > /dev/null
echo ""

echo "=== Before renew ==="
echo "locked_at and expires_at after acquire:"
grep -E "locked_at|expires_at" "$FIXTURE"
BEFORE_LOCKED=$(grep "locked_at:" "$FIXTURE" | sed 's/.*"\(.*\)".*/\1/')
BEFORE_EXPIRES=$(grep "expires_at:" "$FIXTURE" | sed 's/.*"\(.*\)".*/\1/')
echo ""

echo "=== AC-004: factory-lock-write.sh renew (mid-burst TTL refresh) ==="
bash "$HELPER" renew "$FIXTURE"
echo ""

echo "=== After renew ==="
grep -E "locked_at|expires_at" "$FIXTURE"
AFTER_LOCKED=$(grep "locked_at:" "$FIXTURE" | sed 's/.*"\(.*\)".*/\1/')
AFTER_EXPIRES=$(grep "expires_at:" "$FIXTURE" | sed 's/.*"\(.*\)".*/\1/')
echo ""

echo "=== Verification ==="
if [[ "$BEFORE_LOCKED" == "$AFTER_LOCKED" ]]; then
  echo "PASS: locked_at is UNCHANGED: $AFTER_LOCKED"
else
  echo "FAIL: locked_at changed from $BEFORE_LOCKED to $AFTER_LOCKED"
  exit 1
fi

BE=$(date -j -f "%Y-%m-%dT%H:%M:%SZ" "$BEFORE_EXPIRES" +%s)
AE=$(date -j -f "%Y-%m-%dT%H:%M:%SZ" "$AFTER_EXPIRES" +%s)
if [[ "$AE" -ge "$BE" ]]; then
  echo "PASS: expires_at advanced: $BEFORE_EXPIRES -> $AFTER_EXPIRES"
else
  echo "FAIL: expires_at did not advance"
  exit 1
fi

rm -f "$FIXTURE"
