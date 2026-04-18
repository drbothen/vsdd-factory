#!/usr/bin/env bats
# policy9.bats — tests for Policy 9 validate-vp-consistency hook
#
# Tests the VP-INDEX ↔ verification-architecture ↔ coverage-matrix consistency
# hook against three fixture sets: green (all consistent), canary (fuzz column
# drift replicating a real adversarial finding), and missing-vp (VP added to
# VP-INDEX without arch-doc propagation).

setup() {
  PLUGIN_ROOT="$(cd "$BATS_TEST_DIRNAME/.." && pwd)"
  HOOK="$PLUGIN_ROOT/hooks/validate-vp-consistency.sh"
  FIXTURES="$PLUGIN_ROOT/tests/fixtures"
}

# ---------- Green fixture: all consistent ----------

@test "policy-9: green fixture passes (all consistent)" {
  VP_INDEX="$FIXTURES/policy-9-green/specs/verification-properties/VP-INDEX.md"
  INPUT=$(jq -nc --arg fp "$VP_INDEX" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOK'"
  [ "$status" -eq 0 ]
}

@test "policy-9: green fixture produces no stderr output" {
  VP_INDEX="$FIXTURES/policy-9-green/specs/verification-properties/VP-INDEX.md"
  INPUT=$(jq -nc --arg fp "$VP_INDEX" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOK' 2>&1"
  [ "$status" -eq 0 ]
  [[ -z "$output" ]]
}

# ---------- Canary fixture: fuzz column drift ----------

@test "policy-9: canary fixture fails (fuzz column drift)" {
  VP_INDEX="$FIXTURES/policy-9-canary/specs/verification-properties/VP-INDEX.md"
  INPUT=$(jq -nc --arg fp "$VP_INDEX" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOK' 2>&1"
  [ "$status" -eq 2 ]
}

@test "policy-9: canary reports POLICY 9 VIOLATION" {
  VP_INDEX="$FIXTURES/policy-9-canary/specs/verification-properties/VP-INDEX.md"
  INPUT=$(jq -nc --arg fp "$VP_INDEX" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOK' 2>&1"
  [[ "$output" == *"POLICY 9 VIOLATION"* ]]
}

@test "policy-9: canary detects missing VP-009 or VP-010 in coverage matrix VPs column" {
  # security-module lists only VP-038 but VP-009 and VP-010 also belong there
  # The hook should detect these VPs are missing from coverage matrix
  VP_INDEX="$FIXTURES/policy-9-canary/specs/verification-properties/VP-INDEX.md"
  INPUT=$(jq -nc --arg fp "$VP_INDEX" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOK' 2>&1"
  [ "$status" -eq 2 ]
}

# ---------- Missing-VP fixture: VP-039 not in arch docs ----------

@test "policy-9: missing-VP fixture fails" {
  VP_INDEX="$FIXTURES/policy-9-missing-vp/specs/verification-properties/VP-INDEX.md"
  INPUT=$(jq -nc --arg fp "$VP_INDEX" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOK' 2>&1"
  [ "$status" -eq 2 ]
}

@test "policy-9: missing-VP reports VP-039" {
  VP_INDEX="$FIXTURES/policy-9-missing-vp/specs/verification-properties/VP-INDEX.md"
  INPUT=$(jq -nc --arg fp "$VP_INDEX" '{tool_input: {file_path: $fp}}')
  run bash -c "echo '$INPUT' | '$HOOK' 2>&1"
  [[ "$output" == *"VP-039"* ]]
}

# ---------- Non-VP file: hook is a no-op ----------

@test "policy-9: non-VP file passes through silently" {
  INPUT='{"tool_input":{"file_path":"src/main.rs"}}'
  run bash -c "echo '$INPUT' | '$HOOK'"
  [ "$status" -eq 0 ]
}

# ---------- Structural checks ----------

@test "policy-9: hook file exists and is executable" {
  [ -x "$HOOK" ]
}

@test "policy-9: hook passes syntax check" {
  bash -n "$HOOK"
}

@test "policy-9: hooks.json wires validate-vp-consistency" {
  jq -e '.hooks.PostToolUse[0].hooks[] | select(.command | contains("validate-vp-consistency"))' "$PLUGIN_ROOT/hooks/hooks.json" >/dev/null
}
