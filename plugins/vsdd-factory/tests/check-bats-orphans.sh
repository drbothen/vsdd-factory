#!/usr/bin/env bash
# check-bats-orphans.sh — detect orphan $HOOKS_DIR/<name>.sh references
# in bats test files that no longer have a corresponding hook on disk.
#
# Usage:
#   bash plugins/vsdd-factory/tests/check-bats-orphans.sh
#
# Exit codes:
#   0  all references resolve to existing hook files
#   1  one or more orphan references found (details printed to stdout)

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
BATS_DIR="$SCRIPT_DIR"
HOOKS_DIR="$(cd "$SCRIPT_DIR/../hooks" && pwd)"

orphans_found=0

while IFS= read -r bats_file; do
  # Extract all $HOOKS_DIR/<name>.sh references from this bats file.
  # The regex matches the pattern literally used in the test files.
  while IFS=: read -r lineno match; do
    # Extract just the <name>.sh portion from the match
    hook_name="$(printf '%s' "$match" | grep -oE '\$HOOKS_DIR/[A-Za-z0-9_-]+\.sh' | sed 's|\$HOOKS_DIR/||' | head -1)"
    [ -z "$hook_name" ] && continue

    hook_path="$HOOKS_DIR/$hook_name"
    if [ ! -f "$hook_path" ]; then
      printf 'ORPHAN: %s:%s: references %s (not found at %s)\n' \
        "$bats_file" "$lineno" "\$HOOKS_DIR/$hook_name" "$hook_path"
      orphans_found=$((orphans_found + 1))
    fi
  done < <(grep -nE '\$HOOKS_DIR/[A-Za-z0-9_-]+\.sh' "$bats_file" 2>/dev/null || true)
done < <(find "$BATS_DIR" -maxdepth 1 -name '*.bats' -type f | sort)

if [ "$orphans_found" -gt 0 ]; then
  printf '\n%d orphan hook reference(s) found. Delete the test cases or restore the hook files.\n' "$orphans_found"
  exit 1
fi

echo "check-bats-orphans: all hook references resolve ok"
exit 0
