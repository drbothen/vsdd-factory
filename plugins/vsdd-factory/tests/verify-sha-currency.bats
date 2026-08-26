#!/usr/bin/env bats
# verify-sha-currency.bats — regression suite for the develop_head cite
# extraction + prefix comparison in templates/verify-sha-currency.sh.
#
# Pins the #629 fix: a correct 7-char short-SHA cite must PASS (the prior
# {8,40} + cut -c1-8 form reported NOT_FOUND and silently skipped the check),
# a stale short cite must FAIL, and a declared-but-unextractable develop_head
# must WARN instead of silently skipping.
#
# Fixture: throwaway git repo with a develop branch and a plain .factory/
# holding STATE.md; the script is invoked with --project-root so no live
# factory state is touched.

setup() {
  PLUGIN_ROOT="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
  SCRIPT="$PLUGIN_ROOT/templates/verify-sha-currency.sh"
  WORK="$(mktemp -d)"
  cd "$WORK"
  git init --quiet -b develop
  git config user.email "test@test.com"
  git config user.name "test"
  git commit --allow-empty -qm "chore: fixture root"
  DEV_FULL="$(git rev-parse develop)"
  mkdir -p .factory
}

teardown() {
  cd /
  rm -rf "$WORK"
}

_state_with_cite() {  # $1 = cite value
  printf 'develop_head: "%s"\n' "$1" > .factory/STATE.md
}

@test "verify-sha-currency: correct 7-char cite passes (#629 primary case)" {
  _state_with_cite "${DEV_FULL:0:7}"
  run bash "$SCRIPT" --project-root "$WORK"
  [ "$status" -eq 0 ]
  [[ "$output" != *"NOT_FOUND"* ]]
  [[ "$output" != *"FAIL"* ]]
}

@test "verify-sha-currency: correct 8-char cite passes (no regression)" {
  _state_with_cite "${DEV_FULL:0:8}"
  run bash "$SCRIPT" --project-root "$WORK"
  [ "$status" -eq 0 ]
  [[ "$output" != *"FAIL"* ]]
}

@test "verify-sha-currency: correct 40-char cite passes" {
  _state_with_cite "$DEV_FULL"
  run bash "$SCRIPT" --project-root "$WORK"
  [ "$status" -eq 0 ]
  [[ "$output" != *"FAIL"* ]]
}

@test "verify-sha-currency: stale 7-char cite fails with FAIL line (#629 counterpart)" {
  # A hex string of the right shape that cannot prefix-match the actual SHA.
  local stale="abcdef1"
  [ "${DEV_FULL:0:7}" = "$stale" ] && stale="1234567"
  _state_with_cite "$stale"
  run bash "$SCRIPT" --project-root "$WORK"
  [ "$status" -eq 1 ]
  [[ "$output" == *"FAIL: develop SHA in STATE.md is stale"* ]]
  # The message shows the actual at the cited length, so cited/actual are
  # visually comparable (review F2).
  [[ "$output" == *"actual=${DEV_FULL:0:7}"* ]]
}

@test "verify-sha-currency: develop_head present but unextractable warns, exits 0" {
  printf 'develop_head: "TBD"\n' > .factory/STATE.md
  run bash "$SCRIPT" --project-root "$WORK"
  [ "$status" -eq 0 ]
  [[ "$output" == *"WARN: STATE.md declares develop_head but no 7-40 char hex SHA"* ]]
}
