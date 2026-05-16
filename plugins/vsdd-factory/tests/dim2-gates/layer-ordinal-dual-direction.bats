#!/usr/bin/env bats
# layer-ordinal-dual-direction.bats — bats tests for layer-ordinal-dual-direction.sh
#
# Traces to: AC-010 (S-15.08), D-452(b)
# Gate: dual-direction layer-N sweep:
#   (a) positive form: scans for "<N>th-layer" occurrences (informational)
#   (b) negative drift: scans for "<N-1>th-layer" and "<N+1>th-layer" (failure on any match)
#
# Exits 0 if no drift-class occurrences found; exits 1 if any drift-class match.
#
# RED GATE PHASE: all tests currently FAIL because the script does not yet exist.
# Implementer (phase 3/6) must write the script to make these tests green.
#
# PASS fixture: lessons.md contains "42nd-layer" (positive) but no "41st-layer" or "43rd-layer".
# FAIL fixture: lessons.md contains "42nd-layer" AND "43rd-layer" (N+1 drift injected).

setup() {
  PLUGIN_ROOT="$(cd "${BATS_TEST_DIRNAME}/../.." && pwd)"
  SCRIPT="$PLUGIN_ROOT/hooks/dim2-gates/layer-ordinal-dual-direction.sh"
  FIX_PASS="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/layer-ordinal-dual-direction-pass"
  FIX_FAIL="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/layer-ordinal-dual-direction-fail"
}

@test "PASS: layer-ordinal-dual-direction exits 0 when no N+1/N-1 drift occurrences found" {
  run "$SCRIPT" 42 "$FIX_PASS/lessons.md"
  [ "$status" -eq 0 ]
  [[ "$output" == *"PASS"* ]] || [[ "$output" == *"no drift"* ]] || [[ "$output" == *"clean"* ]]
}

@test "FAIL: layer-ordinal-dual-direction exits 1 when N+1 drift layer ordinal detected" {
  run "$SCRIPT" 42 "$FIX_FAIL/lessons.md"
  [ "$status" -eq 1 ]
  [[ "$output" == *"FAIL"* ]] || [[ "$output" == *"drift"* ]] || [[ "$output" == *"43rd-layer"* ]]
}
