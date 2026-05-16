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
  # added in S-15.08 fix-burst-1 for F-S15.08-LOCAL-P1-001
  FIX_PASS_3BURST="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/dim7-dispatched-count-sweep-pass-3burst"
  FIX_FAIL_3BURST="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/dim7-dispatched-count-sweep-fail-3burst"
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

# added in S-15.08 fix-burst-1 for F-S15.08-LOCAL-P1-001
# 3-burst fixtures verify the line-mapping formula is correct for the 3rd burst section:
# the anachronism in the 3rd burst's Dim-7 must be detected regardless of where those
# lines appear in absolute file numbering.

@test "PASS: dim7-dispatched-count-sweep exits 0 for 3-burst log with no anachronisms" {
  run "$SCRIPT" "$FIX_PASS_3BURST/burst-log.md"
  [ "$status" -eq 0 ]
  [[ "$output" == *"PASS"* ]] || [[ "$output" == *"no anachronism"* ]]
}

@test "FAIL: dim7-dispatched-count-sweep exits 1 when 3rd burst Dim-7 has forward pass reference" {
  run "$SCRIPT" "$FIX_FAIL_3BURST/burst-log.md"
  [ "$status" -eq 1 ]
  [[ "$output" == *"FAIL"* ]] || [[ "$output" == *"anachronism"* ]] || [[ "$output" == *"pass-76"* ]]
}
