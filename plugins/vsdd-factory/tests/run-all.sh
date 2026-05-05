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

# SKIP_SUITES — bats suites excluded from the release-validation gate.
# Currently empty after the TD-020 sweep (2026-05-04) resolved the four
# previously-skipped suites (codify-lessons, generate-registry,
# novelty-assessment, state-health). See CHANGELOG entry "TD-020 sweep —
# bats SKIP_SUITES cleanup" for per-suite outcomes.
#
# Do NOT add new entries without an accompanying tech-debt-register
# ticket and an inline rationale.
SKIP_SUITES=()

is_skipped() {
  local target="$1"
  local s
  # Guard for empty array under `set -u` on older bash (3.2 on macOS).
  [ "${#SKIP_SUITES[@]}" -eq 0 ] && return 1
  for s in "${SKIP_SUITES[@]}"; do
    [ "$s" = "$target" ] && return 0
  done
  return 1
}

shopt -s nullglob
set +e   # allow individual bats suites to fail without aborting the loop
fail_count=0
failed_suites=()
skipped_suites=()
for f in tests/*.bats; do
  name=$(basename "$f" .bats)
  if is_skipped "$name"; then
    skipped_suites+=("$name")
    continue
  fi
  echo
  echo "-- $name --"
  if ! bats "$f"; then
    fail_count=$((fail_count + 1))
    failed_suites+=("$name")
  fi
done
set -e

if [ "${#skipped_suites[@]}" -gt 0 ]; then
  echo
  echo "== Skipped suites (TD-020 — pre-existing failures) =="
  for name in "${skipped_suites[@]}"; do
    echo "  - $name"
  done
fi

echo
if [ "$fail_count" -gt 0 ]; then
  echo "FAIL: $fail_count suite(s) had failures:"
  for name in "${failed_suites[@]}"; do
    echo "  - $name"
  done
  exit 1
fi
echo "All tests passed."
