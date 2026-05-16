#!/usr/bin/env bats
# propagation-completeness.bats — bats tests for propagation-completeness.sh
#
# Traces to: AC-005 (S-15.08), D-452(a)
# Gate: post-derivation propagation-completeness — derived value must appear at
# ALL prescribed sites, not just the primary site. Sites listed in a file as
# "<file-path>:<grep-pattern>" pairs (one per line).
#
# RED GATE PHASE: all tests currently FAIL because the script does not yet exist.
# Implementer (phase 3/6) must write the script to make these tests green.

setup() {
  PLUGIN_ROOT="$(cd "${BATS_TEST_DIRNAME}/../.." && pwd)"
  SCRIPT="$PLUGIN_ROOT/hooks/dim2-gates/propagation-completeness.sh"
  FIX_PASS="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/propagation-completeness-pass"
  FIX_FAIL="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/propagation-completeness-fail"
}

@test "PASS: propagation-completeness exits 0 when derived value present at all prescribed sites" {
  # Sites file uses paths relative to the fixture dir; run from fixture dir
  pushd "$FIX_PASS" >/dev/null
  run "$SCRIPT" "D-453" "$FIX_PASS/sites.txt"
  popd >/dev/null
  [ "$status" -eq 0 ]
  [[ "$output" == *"PASS"* ]]
}

@test "FAIL: propagation-completeness exits 1 when derived value missing from one prescribed site" {
  pushd "$FIX_FAIL" >/dev/null
  run "$SCRIPT" "D-453" "$FIX_FAIL/sites.txt"
  popd >/dev/null
  [ "$status" -eq 1 ]
  [[ "$output" == *"FAIL"* ]] || [[ "$output" == *"missing"* ]] || [[ "$output" == *"INDEX"* ]]
}
