#!/usr/bin/env bats
# block-label-canonical-form.bats — bats tests for block-label-canonical-form.sh
#
# Traces to: AC-003 (S-15.08), D-454(d)
# Gate: verifies all 9 D-444(c) canonical block labels present in a burst-log file.
# Required labels: Parent-commit, Adversary verdict, Files touched, Codifications,
# Dim-2, Dim-5, Dim-6, Dim-7, Closes.
#
# RED GATE PHASE: all tests currently FAIL because the script does not yet exist.
# Implementer (phase 3/6) must write the script to make these tests green.

setup() {
  PLUGIN_ROOT="$(cd "${BATS_TEST_DIRNAME}/../.." && pwd)"
  SCRIPT="$PLUGIN_ROOT/hooks/dim2-gates/block-label-canonical-form.sh"
  FIX_PASS="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/block-label-canonical-form-pass"
  FIX_FAIL="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/block-label-canonical-form-fail"
}

@test "PASS: block-label-canonical-form exits 0 when all 9 D-444(c) labels present" {
  run "$SCRIPT" "$FIX_PASS/burst-log.md"
  [ "$status" -eq 0 ]
  [[ "$output" == *"PASS"* ]]
}

@test "FAIL: block-label-canonical-form exits 1 when a canonical block label is missing" {
  run "$SCRIPT" "$FIX_FAIL/burst-log.md"
  [ "$status" -eq 1 ]
  [[ "$output" == *"FAIL"* ]] || [[ "$output" == *"missing"* ]] || [[ "$output" == *"Dim-7"* ]]
}
