#!/usr/bin/env bash
# run-ac003adj-demo.sh — demonstrate AC-001 error path (SchemaViolation on malformed file)
# Shows: acquire on a file with no frontmatter fences exits non-zero + SchemaViolation message

set -uo pipefail

WORKTREE=/Users/zious/Documents/GITHUB/vsdd-factory/.worktrees/S-17.01
HELPER="$WORKTREE/plugins/vsdd-factory/bin/factory-lock-write.sh"

echo "=== Creating malformed file (no --- frontmatter fences) ==="
MALFORMED=$(mktemp /tmp/malformed.XXXXXX)
echo "# Just a body — no YAML frontmatter fences" > "$MALFORMED"
echo ""
echo "File contents:"
cat "$MALFORMED"
echo ""

echo "=== Running factory-lock-write.sh acquire on malformed file ==="
bash "$HELPER" acquire "$MALFORMED" 2>&1
EXIT_CODE=$?
echo ""
echo "Exit code: $EXIT_CODE (expected: 1)"
if [[ "$EXIT_CODE" -ne 0 ]]; then
  echo "PASS: non-zero exit on SchemaViolation"
else
  echo "FAIL: expected non-zero exit"
fi

rm -f "$MALFORMED"
