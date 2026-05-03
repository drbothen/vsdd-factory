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
for f in tests/*.bats; do
  name=$(basename "$f" .bats)
  echo
  echo "-- $name --"
  bats "$f"
done
echo
echo "All tests passed."
