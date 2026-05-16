#!/usr/bin/env bats
# dim1-file-count-arithmetic.bats — bats tests for dim1-file-count-arithmetic.sh
#
# Traces to: AC-007 (S-15.08), D-450(c)
# Gate: Dim-1 headline cardinality vs enumerated list count arithmetic.
# Matches headlines of form: "**Files touched (Dim-1): N unique files**"
# and counts comma-delimited filenames on the following list line,
# asserting the headline integer equals the list count.
#
# RED GATE PHASE: all tests currently FAIL because the script does not yet exist.
# Implementer (phase 3/6) must write the script to make these tests green.
#
# PASS fixture: headline says "3 unique files"; comma-list has 3 files.
# FAIL fixture: headline says "5 unique files"; comma-list has 3 files.

setup() {
  PLUGIN_ROOT="$(cd "${BATS_TEST_DIRNAME}/../.." && pwd)"
  SCRIPT="$PLUGIN_ROOT/hooks/dim2-gates/dim1-file-count-arithmetic.sh"
  FIX_PASS="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/dim1-file-count-arithmetic-pass"
  FIX_FAIL="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/dim1-file-count-arithmetic-fail"
}

@test "PASS: dim1-file-count-arithmetic exits 0 when headline count matches comma-list length" {
  run "$SCRIPT" "$FIX_PASS/burst-log.md"
  [ "$status" -eq 0 ]
  [[ "$output" == *"PASS"* ]]
}

@test "FAIL: dim1-file-count-arithmetic exits 1 when headline count differs from comma-list length" {
  run "$SCRIPT" "$FIX_FAIL/burst-log.md"
  [ "$status" -eq 1 ]
  [[ "$output" == *"FAIL:"* ]]
}
