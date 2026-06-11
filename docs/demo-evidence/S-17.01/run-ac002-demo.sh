#!/usr/bin/env bash
# run-ac002-demo.sh — demonstrate AC-002
# AC-002: clear removes factory_lock key entirely (not null) from frontmatter

set -euo pipefail

WORKTREE=/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-17.01
HELPER="$WORKTREE/plugins/vsdd-factory/bin/factory-lock-write.sh"

echo "=== Creating fixture with existing factory_lock block ==="
FIXTURE=$(mktemp /tmp/demo-state.XXXXXX)
cat > "$FIXTURE" << 'FIXTURE_EOF'
---
document_type: state
version: "1.0"
factory_lock:
  holder: "user@example.com"
  locked_at: "2026-06-11T04:00:00Z"
  expires_at: "2026-06-11T04:45:00Z"
---

# Factory STATE
FIXTURE_EOF

echo "=== Before clear: factory_lock is present ==="
grep "factory_lock" "$FIXTURE" && echo "(key present in frontmatter)"
echo ""

echo "=== AC-002: factory-lock-write.sh clear ==="
bash "$HELPER" clear "$FIXTURE"
echo ""

echo "=== After clear: factory_lock MUST be absent (not nulled) ==="
if grep -q "factory_lock" "$FIXTURE"; then
  echo "FAIL: factory_lock key still present after clear"
  exit 1
else
  echo "PASS: factory_lock key is absent from frontmatter"
fi
echo ""

echo "=== Full file after clear (no factory_lock, no null placeholder) ==="
cat "$FIXTURE"

rm -f "$FIXTURE"
