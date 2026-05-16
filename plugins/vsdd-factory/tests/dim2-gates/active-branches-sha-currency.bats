#!/usr/bin/env bats
# active-branches-sha-currency.bats — bats tests for active-branches-sha-currency.sh
#
# Traces to: AC-008 (S-15.08), D-450(d) (SHA currency sub-clause)
# Gate: for each branch in STATE.md Active Branches table, runs
# "git rev-parse origin/<branch>" and compares to the SHA cell.
#
# Env-var override pattern (per story-writer note 1):
# Script supports GIT_TEST_SHA_OVERRIDE_<branch>=<sha> to avoid requiring
# a real git remote during testing. Branch hyphens become underscores:
#   factory-artifacts -> GIT_TEST_SHA_OVERRIDE_factory_artifacts
#
# RED GATE PHASE: all tests currently FAIL because the script does not yet exist.
# Implementer (phase 3/6) must write the script to make these tests green.
#
# PASS fixture: STATE.md SHAs match GIT_TEST_SHA_OVERRIDE_* values.
# FAIL fixture: STATE.md has stale SHA for develop (deadbeef...).

setup() {
  PLUGIN_ROOT="$(cd "${BATS_TEST_DIRNAME}/../.." && pwd)"
  SCRIPT="$PLUGIN_ROOT/hooks/dim2-gates/active-branches-sha-currency.sh"
  FIX_PASS="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/active-branches-sha-currency-pass"
  FIX_FAIL="${BATS_TEST_DIRNAME}/../fixtures/dim2-gates/active-branches-sha-currency-fail"

  # SHA values matching the PASS fixture STATE.md
  SHA_MAIN="666d689f1234567890abcdef1234567890abcdef"
  SHA_DEVELOP="224fa18421214b30dacf1cdd606152294cd33bd6"
  SHA_FACTORY="0b4972a6abcdef1234567890abcdef1234567890"
}

@test "PASS: active-branches-sha-currency exits 0 when all branch SHAs match" {
  run env \
    GIT_TEST_SHA_OVERRIDE_main="$SHA_MAIN" \
    GIT_TEST_SHA_OVERRIDE_develop="$SHA_DEVELOP" \
    GIT_TEST_SHA_OVERRIDE_factory_artifacts="$SHA_FACTORY" \
    "$SCRIPT" "$FIX_PASS" "$FIX_PASS/STATE.md"
  [ "$status" -eq 0 ]
  [[ "$output" == *"PASS"* ]]
}

@test "FAIL: active-branches-sha-currency exits 1 when develop SHA is stale in STATE.md" {
  run env \
    GIT_TEST_SHA_OVERRIDE_main="$SHA_MAIN" \
    GIT_TEST_SHA_OVERRIDE_develop="$SHA_DEVELOP" \
    GIT_TEST_SHA_OVERRIDE_factory_artifacts="$SHA_FACTORY" \
    "$SCRIPT" "$FIX_FAIL" "$FIX_FAIL/STATE.md"
  [ "$status" -eq 1 ]
  [[ "$output" == *"FAIL"* ]] || [[ "$output" == *"stale"* ]] || [[ "$output" == *"develop"* ]]
}
