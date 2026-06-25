#!/usr/bin/env bash
# check-bats-orphans.sh — detect orphan hooks/<name>.sh references
# in bats test files that no longer have a corresponding hook on disk.
#
# Catches all common path forms:
#   $HOOKS_DIR/<name>.sh
#   $PLUGIN_ROOT/hooks/<name>.sh
#   ${BATS_TEST_DIRNAME}/../hooks/<name>.sh
#   ${CLAUDE_PLUGIN_ROOT}/hooks/<name>.sh
#   any other "/hooks/<name>.sh" form
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

# Hook names that are test-only fixtures (live in tests/fixtures/, not hooks/).
# Bats tests reference them as `hooks/<name>.sh` because they are created
# dynamically inside $WORK/hooks/ at test runtime — the real files are NOT
# in hooks/ and must NOT be treated as orphan references.
TEST_FIXTURE_NAMES=(
  stub-exit0
  stub-exit2
  capture  # VP-093 git_context capture fixture — created at runtime by vp093-git-context-injection.bats
)

is_test_fixture() {
  local name="$1"
  local f
  for f in "${TEST_FIXTURE_NAMES[@]}"; do
    [[ "$name" == "$f.sh" ]] && return 0
  done
  return 1
}

orphans_found=0

while IFS= read -r bats_file; do
  # Extract all hooks/<name>.sh references from this bats file.
  # The broad pattern matches any path that contains hooks/<name>.sh,
  # regardless of how the prefix is expressed ($HOOKS_DIR, $PLUGIN_ROOT/hooks,
  # ${BATS_TEST_DIRNAME}/../hooks, etc).
  while IFS=: read -r lineno match; do
    # Extract just the <name>.sh portion from the match
    hook_name="$(printf '%s' "$match" | grep -oE 'hooks/[A-Za-z0-9_-]+\.sh' | sed 's|hooks/||' | head -1)"
    [ -z "$hook_name" ] && continue

    # Skip known test-fixture hooks that live in tests/fixtures/ rather than
    # hooks/ — they are created dynamically in $WORK/hooks/ at test runtime.
    is_test_fixture "$hook_name" && continue

    hook_path="$HOOKS_DIR/$hook_name"
    if [ ! -f "$hook_path" ]; then
      printf 'ORPHAN: %s:%s: references hooks/%s (not found at %s)\n' \
        "$bats_file" "$lineno" "$hook_name" "$hook_path"
      orphans_found=$((orphans_found + 1))
    fi
  # Exclude lines where the path is the simulated command input (e.g.
  # _run_hook "chmod +x .../hooks/foo.sh") rather than a hook invocation.
  done < <(grep -nE 'hooks/[A-Za-z0-9_-]+\.sh' "$bats_file" 2>/dev/null | grep -v '_run_hook' || true)
done < <(find "$BATS_DIR" -maxdepth 1 -name '*.bats' -type f | sort)

if [ "$orphans_found" -gt 0 ]; then
  printf '\n%d orphan hook reference(s) found. Delete the test cases or restore the hook files.\n' "$orphans_found"
  exit 1
fi

echo "check-bats-orphans: all hook references resolve ok"
exit 0
