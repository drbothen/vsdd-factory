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

  # F-003 EOL fragility guard: assert fixture is exactly 23 lines before invoking the script.
  # If an editor save / git autocrlf rule / .gitattributes misconfiguration inserts or strips a
  # trailing newline, this assertion fires at fixture-prep time with a diagnostic message rather
  # than producing a confusing banner-mismatch at gate time.
  # The fixture is protected by tests/fixtures/dim2-gates/.gitattributes (banner-wc-l-pass/STATE.md -text).
  # Closes: F-S15.08-LOCAL-P1-003
  local actual_lines
  actual_lines="$(wc -l < "${FIX_PASS}/STATE.md")"
  if [[ "$actual_lines" -ne 23 ]]; then
    echo "FIXTURE INTEGRITY ERROR: banner-wc-l-pass/STATE.md must be exactly 23 lines" \
         "(wc -l returned ${actual_lines}). Check for trailing-newline regression or" \
         "git autocrlf/eol normalization. See tests/fixtures/dim2-gates/.gitattributes." >&2
    return 1
  fi
}

@test "PASS: banner-wc-l exits 0 when banner line count matches wc -l and margin arithmetic correct" {
  run "$SCRIPT" "$FIX_PASS/STATE.md"
  [ "$status" -eq 0 ]
  [[ "$output" == *"PASS"* ]]
}

@test "FAIL: banner-wc-l exits 1 when banner line count does not match wc -l" {
  run "$SCRIPT" "$FIX_FAIL/STATE.md"
  [ "$status" -eq 1 ]
  [[ "$output" == *"FAIL:"* ]]
}
