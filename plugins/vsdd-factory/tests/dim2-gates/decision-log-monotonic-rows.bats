#!/usr/bin/env bats
# decision-log-monotonic-rows.bats — bats tests for decision-log-monotonic-rows.sh
#
# Traces to: AC-009 (S-15.08), D-450(e)
# Gate: extracts all "| D-NNN" rows using regex "^\| D-[0-9]+[\( ]",
# checks that extracted D-NNN integers are strictly ascending,
# reports any inversion.
#
# RED GATE PHASE: all tests currently FAIL because the script does not yet exist.
# Implementer (phase 3/6) must write the script to make these tests green.
#
# PASS fixture: rows in order D-450, D-451, D-452, D-452, D-453, D-454 (ascending).
# FAIL fixture: rows with inversion D-450, D-453, D-452, D-454 (D-453 before D-452).

setup() {
  PLUGIN_ROOT="$(cd "${BATS_TEST_DIRNAME}/../.." && pwd)"
  SCRIPT="$PLUGIN_ROOT/hooks/dim2-gates/decision-log-monotonic-rows.sh"
  FIX_PASS="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/decision-log-monotonic-rows-pass"
  FIX_FAIL="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/decision-log-monotonic-rows-fail"
}

@test "PASS: decision-log-monotonic-rows exits 0 for strictly ascending D-NNN rows" {
  run "$SCRIPT" "$FIX_PASS/decision-log.md"
  [ "$status" -eq 0 ]
  [[ "$output" == *"PASS"* ]] || [[ "$output" == *"monotonic"* ]] || [[ "$output" == *"ascending"* ]]
}

@test "FAIL: decision-log-monotonic-rows exits 1 when D-NNN rows contain an inversion" {
  run "$SCRIPT" "$FIX_FAIL/decision-log.md"
  [ "$status" -eq 1 ]
  [[ "$output" == *"FAIL:"* ]]
}
