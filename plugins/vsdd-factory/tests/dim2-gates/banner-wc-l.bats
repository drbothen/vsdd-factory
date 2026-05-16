#!/usr/bin/env bats
# banner-wc-l.bats — bats tests for banner-wc-l.sh
#
# Traces to: AC-004 (S-15.08), D-450(d) (banner wc-l sub-clause)
# Gate: STATE.md banner "actual N lines" vs wc -l arithmetic + dual-margin verification.
#
# RED GATE PHASE: all tests currently FAIL because the script does not yet exist.
# Implementer (phase 3/6) must write the script to make these tests green.
#
# PASS fixture: STATE.md has banner "actual 23 lines" (500 - 23 = 477 margin);
#               wc -l returns 23 — arithmetic matches.
# FAIL fixture: STATE.md has banner "actual 50 lines" (500 - 50 = 450 margin);
#               wc -l returns 23 — mismatch is the injected defect.

setup() {
  PLUGIN_ROOT="$(cd "${BATS_TEST_DIRNAME}/../.." && pwd)"
  SCRIPT="$PLUGIN_ROOT/hooks/dim2-gates/banner-wc-l.sh"
  FIX_PASS="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/banner-wc-l-pass"
  FIX_FAIL="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/banner-wc-l-fail"
}

@test "PASS: banner-wc-l exits 0 when banner line count matches wc -l and margin arithmetic correct" {
  run "$SCRIPT" "$FIX_PASS/STATE.md"
  [ "$status" -eq 0 ]
  [[ "$output" == *"PASS"* ]]
}

@test "FAIL: banner-wc-l exits 1 when banner line count does not match wc -l" {
  run "$SCRIPT" "$FIX_FAIL/STATE.md"
  [ "$status" -eq 1 ]
  [[ "$output" == *"FAIL"* ]] || [[ "$output" == *"expected"* ]] || [[ "$output" == *"actual"* ]]
}
