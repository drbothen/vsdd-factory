#!/usr/bin/env bats
# dim7-dispatched-count-sweep.bats — bats tests for dim7-dispatched-count-sweep.sh
#
# Traces to: AC-006 (S-15.08), D-450(b)
# Gate: Dim-7 sibling-sweep across ALL prior burst-log entries for anachronism
# pattern (Dim-7 cell references a pass count inconsistent with the burst's pass number).
#
# RED GATE PHASE: all tests currently FAIL because the script does not yet exist.
# Implementer (phase 3/6) must write the script to make these tests green.
#
# PASS fixture: burst-log.md with clean Dim-7 cells (no anachronism).
# FAIL fixture: burst-log.md with one injected Dim-7 anachronism
#               (pass-72 burst's Dim-7 cell says "pass-74" count — forward reference).

setup() {
  PLUGIN_ROOT="$(cd "${BATS_TEST_DIRNAME}/../.." && pwd)"
  SCRIPT="$PLUGIN_ROOT/hooks/dim2-gates/dim7-dispatched-count-sweep.sh"
  FIX_PASS="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/dim7-dispatched-count-sweep-pass"
  FIX_FAIL="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/dim7-dispatched-count-sweep-fail"
}

@test "PASS: dim7-dispatched-count-sweep exits 0 for burst-log with no Dim-7 anachronisms" {
  run "$SCRIPT" "$FIX_PASS/burst-log.md"
  [ "$status" -eq 0 ]
  [[ "$output" == *"PASS"* ]] || [[ "$output" == *"no anachronism"* ]]
}

@test "FAIL: dim7-dispatched-count-sweep exits 1 when Dim-7 anachronism line detected" {
  run "$SCRIPT" "$FIX_FAIL/burst-log.md"
  [ "$status" -eq 1 ]
  [[ "$output" == *"FAIL"* ]] || [[ "$output" == *"anachronism"* ]] || [[ "$output" == *"pass-74"* ]]
}
