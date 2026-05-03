#!/bin/bash
# run-all.sh — run the full vsdd-factory plugin test suite.
#
# Requirements: bats-core, jq, yq.

set -euo pipefail

PLUGIN_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$PLUGIN_ROOT"

die() { echo "run-all: $*" >&2; exit 1; }

for tool in bats jq yq; do
  command -v "$tool" &>/dev/null || die "$tool is required. Install: brew install $tool"
done

echo "== Syntax checks =="
fail=0
for f in hooks/*.sh bin/*; do
  if ! bash -n "$f" 2>&1; then
    echo "FAIL: $f"
    fail=$((fail+1))
  fi
done
[ "$fail" -eq 0 ] || die "$fail syntax errors"
echo "all scripts ok"

echo
echo "== Running all bats test suites =="
shopt -s nullglob
set +e   # allow individual bats suites to fail without aborting the loop
fail_count=0
failed_suites=()
for f in tests/*.bats; do
  name=$(basename "$f" .bats)
  echo
  echo "-- $name --"
  if ! bats "$f"; then
    fail_count=$((fail_count + 1))
    failed_suites+=("$name")
  fi
done
set -e

echo
if [ "$fail_count" -gt 0 ]; then
  echo "FAIL: $fail_count suite(s) had failures:"
  for name in "${failed_suites[@]}"; do
    echo "  - $name"
  done
  exit 1
fi
echo "All tests passed."
