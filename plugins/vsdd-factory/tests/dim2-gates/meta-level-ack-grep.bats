#!/usr/bin/env bats
# meta-level-ack-grep.bats — bats tests for meta-level-ack-grep.sh
#
# Traces to: AC-011 (S-15.08), D-451(a)
# Gate: runs "grep -c 'META-LEVEL-<N> CANDIDATE CONFIRMED'" against each of 4 files
# and prints literal counts per file (D-449(a) compliant).
# Exits 0 if total count across all 4 files >= 1; exits 1 if total == 0.
#
# RED GATE PHASE: all tests currently FAIL because the script does not yet exist.
# Implementer (phase 3/6) must write the script to make these tests green.
#
# PASS fixture: burst-log.md contains "META-LEVEL-24 CANDIDATE CONFIRMED"; other 3 files do not.
# FAIL fixture: none of the 4 files contain the acknowledgment string.

setup() {
  PLUGIN_ROOT="$(cd "${BATS_TEST_DIRNAME}/../.." && pwd)"
  SCRIPT="$PLUGIN_ROOT/hooks/dim2-gates/meta-level-ack-grep.sh"
  FIX_PASS="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/meta-level-ack-grep-pass"
  FIX_FAIL="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/meta-level-ack-grep-fail"
}

@test "PASS: meta-level-ack-grep exits 0 when acknowledgment present in at least 1 of 4 files" {
  run "$SCRIPT" 24 \
    "$FIX_PASS/burst-log.md" \
    "$FIX_PASS/lessons.md" \
    "$FIX_PASS/decision-log.md" \
    "$FIX_PASS/state.md"
  [ "$status" -eq 0 ]
  [[ "$output" == *"PASS"* ]] || [[ "$output" == *"total"* ]]
}

@test "FAIL: meta-level-ack-grep exits 1 when acknowledgment absent from all 4 files" {
  run "$SCRIPT" 24 \
    "$FIX_FAIL/burst-log.md" \
    "$FIX_FAIL/lessons.md" \
    "$FIX_FAIL/decision-log.md" \
    "$FIX_FAIL/state.md"
  [ "$status" -eq 1 ]
  [[ "$output" == *"FAIL"* ]] || [[ "$output" == *"total: 0"* ]] || [[ "$output" == *"absent"* ]]
}
