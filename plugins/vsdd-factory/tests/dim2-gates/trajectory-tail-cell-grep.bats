#!/usr/bin/env bats
# trajectory-tail-cell-grep.bats — bats tests for trajectory-tail-cell-grep.sh
#
# Traces to: AC-001 (S-15.08), D-454(a)
# Gate: per-cell line-anchor grep for trajectory_tail at each canonical prescribed site.
#
# RED GATE PHASE: all tests currently FAIL because the script does not yet exist.
# Implementer (phase 3/6) must write the script to make these tests green.

setup() {
  PLUGIN_ROOT="$(cd "${BATS_TEST_DIRNAME}/../.." && pwd)"
  SCRIPT="$PLUGIN_ROOT/hooks/dim2-gates/trajectory-tail-cell-grep.sh"
  FIX_PASS="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/trajectory-tail-cell-grep-pass"
  FIX_FAIL="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/trajectory-tail-cell-grep-fail"
}

@test "PASS: trajectory-tail-cell-grep exits 0 when tail value present at all prescribed sites" {
  run "$SCRIPT" "$FIX_PASS" "→9→9→9→9" "$FIX_PASS/sites.txt"
  [ "$status" -eq 0 ]
  [[ "$output" == *"PASS"* ]]
}

@test "FAIL: trajectory-tail-cell-grep exits 1 when tail value missing from a prescribed site" {
  run "$SCRIPT" "$FIX_FAIL" "→9→9→9→9" "$FIX_FAIL/sites.txt"
  [ "$status" -eq 1 ]
  [[ "$output" == *"FAIL"* ]]
}
