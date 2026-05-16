#!/usr/bin/env bats
# freshness-literal-stdout.bats — bats tests for freshness-literal-stdout.sh
#
# Traces to: AC-002 (S-15.08), D-454(b)
# Gate: freshness re-execution with literal stdout capture — command + exit code + raw output.
#
# RED GATE PHASE: all tests currently FAIL because the script does not yet exist.
# Implementer (phase 3/6) must write the script to make these tests green.

setup() {
  PLUGIN_ROOT="$(cd "${BATS_TEST_DIRNAME}/../.." && pwd)"
  SCRIPT="$PLUGIN_ROOT/hooks/dim2-gates/freshness-literal-stdout.sh"
}

@test "PASS: freshness-literal-stdout exits 0 when re-run command exits 0 and captures stdout" {
  run "$SCRIPT" "echo hello"
  [ "$status" -eq 0 ]
  [[ "$output" == *"hello"* ]]
}

@test "FAIL: freshness-literal-stdout exits 1 when re-run command exits non-zero" {
  run "$SCRIPT" "false"
  [ "$status" -eq 1 ]
  [[ "$output" == *"FAIL"* ]] || [[ "$output" == *"exit"* ]] || [ "$status" -ne 0 ]
}
