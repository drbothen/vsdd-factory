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
echo "== Hook tests =="
bats tests/hooks.bats

echo
echo "== Bin tests =="
bats tests/bin.bats

echo
echo "== Skill structure tests =="
bats tests/skills.bats

echo
echo "== Visual companion tests =="
bats tests/visual-companion.bats

echo
echo "== Permission model tests =="
bats tests/permissions.bats

echo
echo "== Policy 9 VP-consistency tests =="
bats tests/policy9.bats

echo
echo "== Destructive command guard tests =="
bats tests/destructive-guard.bats

echo
echo "== Protect secrets hook tests =="
bats tests/protect-secrets.bats

echo
echo "== Emit-event helper tests =="
bats tests/emit-event.bats

echo
echo "== Policy enforcement hook tests =="
bats tests/policy-enforcement.bats

echo
echo "== Hook robustness tests =="
bats tests/hook-robustness.bats

echo
echo "== Template compliance hook tests =="
bats tests/template-compliance.bats

echo
echo "== Finding format hook tests =="
bats tests/finding-format.bats

echo
echo "== Input hash tests =="
bats tests/input-hash.bats

echo
echo "All tests passed."
