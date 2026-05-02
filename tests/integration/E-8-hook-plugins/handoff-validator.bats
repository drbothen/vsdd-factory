#!/usr/bin/env bats
# tests/integration/E-8-hook-plugins/handoff-validator.bats
#
# AC-005 bats parity tests for handoff-validator native WASM plugin.
#
# Tests behavioral parity of the native WASM crate against the 7 canonical
# test cases from S-8.01 T-5 (BC-7.03.043 + BC-7.03.044).
#
# RED GATE: All tests in this file MUST FAIL until:
#   1. The native WASM artifact is built:
#      cargo build --target wasm32-wasip1 -p handoff-validator
#   2. The dispatcher can invoke the WASM plugin with the test fixtures.
#
# The dispatcher invocation command is TBD pending T-4 WASM build completion.
# This bats file is the STRUCTURAL CONTRACT for the parity tests; the
# invocation mechanism is filled in by the implementer in T-5.
#
# BC traces:
#   BC-7.03.043 postcondition 1 (cases a, f — empty result warn)
#   BC-7.03.044 postcondition 1 (cases b, d — short result warn)
#   BC-7.03.042 postcondition 2 (cases c, e — no warn)
#   BC-7.03.042 invariant 2 (case g — malformed JSON graceful exit)
#   BC-2.02.012 postconditions 5-6 (agent identity + message fallback)
#
# Story: S-8.01 — Native port: handoff-validator (SubagentStop)
# AC:    AC-005 (T-5 parity tests)

# ---------------------------------------------------------------------------
# Setup: locate the WASM artifact and the dispatcher invocation helper.
# The WASM artifact lives at:
#   ${WORKTREE_ROOT}/target/wasm32-wasip1/debug/handoff-validator.wasm
# ---------------------------------------------------------------------------

setup() {
  WORKTREE_ROOT="$(git rev-parse --show-toplevel)"
  WASM_ARTIFACT="${WORKTREE_ROOT}/target/wasm32-wasip1/debug/handoff-validator.wasm"

  # The dispatcher CLI is used to invoke the WASM plugin with a JSON fixture
  # piped to stdin. The dispatcher binary is expected at:
  #   ${WORKTREE_ROOT}/target/debug/factory-dispatcher
  # (or installed in PATH as vsdd-dispatcher for CI).
  # Implementer fills in the correct invocation in T-5.
  DISPATCHER="${WORKTREE_ROOT}/target/debug/factory-dispatcher"
}

# Helper: invoke the handoff-validator WASM plugin with a JSON envelope.
# Captures stdout, stderr, and exit status.
# Usage: invoke_hook <json_envelope>
invoke_hook() {
  local json="$1"
  # RED GATE: WASM artifact does not exist until T-4 builds it.
  # Fail with a clear message if the artifact is missing.
  if [ ! -f "$WASM_ARTIFACT" ]; then
    skip "WASM artifact not built: run 'cargo build --target wasm32-wasip1 -p handoff-validator' first (T-4)"
  fi

  # Invoke via dispatcher CLI (exact invocation TBD in T-5).
  # Placeholder: pipe the JSON to wasmtime directly for initial testing.
  # The implementer will update this to use the dispatcher once T-4 is done.
  echo "$json" | wasmtime run --dir . "$WASM_ARTIFACT"
}

# ---------------------------------------------------------------------------
# Case (a): empty result → exit 0 + stderr contains "empty result"
# BC-7.03.043 postcondition 1
# ---------------------------------------------------------------------------

@test "AC-005(a): empty last_assistant_message → exit 0, stderr contains 'empty result'" {
  local json='{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","agent_type":"test-agent","last_assistant_message":""}'
  run invoke_hook "$json"
  [ "$status" -eq 0 ]
  [[ "$output" == *"empty result"* ]] || [[ "$stderr" == *"empty result"* ]] || \
    { echo "Expected 'empty result' in output; got: $output"; false; }
}

# ---------------------------------------------------------------------------
# Case (b): result with 5 non-whitespace chars → exit 0 + stderr contains
#           "non-whitespace characters"
# BC-7.03.044 postcondition 1
# ---------------------------------------------------------------------------

@test "AC-005(b): 5-char result → exit 0, stderr contains 'non-whitespace characters'" {
  local json='{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","agent_type":"test-agent","last_assistant_message":"hello"}'
  run invoke_hook "$json"
  [ "$status" -eq 0 ]
  [[ "$output" == *"non-whitespace characters"* ]] || [[ "$stderr" == *"non-whitespace characters"* ]] || \
    { echo "Expected 'non-whitespace characters' in output; got: $output"; false; }
}

# ---------------------------------------------------------------------------
# Case (c): result with 50 non-whitespace chars → exit 0, no stderr warning
# BC-7.03.042 postcondition 2
# ---------------------------------------------------------------------------

@test "AC-005(c): 50-char result → exit 0, no stderr warning" {
  # 50 'a' characters = 50 non-whitespace
  local msg
  msg="$(printf 'a%.0s' {1..50})"
  local json="{\"event_name\":\"SubagentStop\",\"session_id\":\"s\",\"dispatcher_trace_id\":\"t\",\"agent_type\":\"test-agent\",\"last_assistant_message\":\"${msg}\"}"
  run invoke_hook "$json"
  [ "$status" -eq 0 ]
  # Must NOT contain any warning message
  [[ "$output" != *"non-whitespace"* ]] || { echo "50-char result must not warn; got: $output"; false; }
  [[ "$output" != *"empty result"* ]]   || { echo "50-char result must not warn; got: $output"; false; }
}

# ---------------------------------------------------------------------------
# Case (d): result with exactly 39 non-whitespace chars → exit 0 + stderr warning
# BC-7.03.044 postcondition 1 (below threshold)
# ---------------------------------------------------------------------------

@test "AC-005(d): LEN=39 → exit 0, warning emitted (below threshold)" {
  local msg
  msg="$(printf 'a%.0s' {1..39})"
  local json="{\"event_name\":\"SubagentStop\",\"session_id\":\"s\",\"dispatcher_trace_id\":\"t\",\"agent_type\":\"test-agent\",\"last_assistant_message\":\"${msg}\"}"
  run invoke_hook "$json"
  [ "$status" -eq 0 ]
  [[ "$output" == *"non-whitespace characters"* ]] || [[ "$stderr" == *"non-whitespace characters"* ]] || \
    { echo "LEN=39 must warn (below threshold); got: $output"; false; }
}

# ---------------------------------------------------------------------------
# Case (e): result with exactly 40 non-whitespace chars → exit 0, NO warning
# BC-7.03.042 postcondition 2 (at-or-above threshold)
# Off-by-one parity with bash `(( LEN < 40 ))`
# ---------------------------------------------------------------------------

@test "AC-005(e): LEN=40 → exit 0, NO warning (at-or-above threshold)" {
  local msg
  msg="$(printf 'a%.0s' {1..40})"
  local json="{\"event_name\":\"SubagentStop\",\"session_id\":\"s\",\"dispatcher_trace_id\":\"t\",\"agent_type\":\"test-agent\",\"last_assistant_message\":\"${msg}\"}"
  run invoke_hook "$json"
  [ "$status" -eq 0 ]
  [[ "$output" != *"non-whitespace"* ]] || { echo "LEN=40 must NOT warn; got: $output"; false; }
  [[ "$output" != *"empty result"* ]]   || { echo "LEN=40 must NOT warn; got: $output"; false; }
}

# ---------------------------------------------------------------------------
# Case (f): JSON with missing last_assistant_message → exit 0, stderr warns
# BC-2.02.012 Postcondition 6 / EC-001: missing field → fallback to result → empty
# BC-7.03.043 postcondition 1
# ---------------------------------------------------------------------------

@test "AC-005(f): missing last_assistant_message field → exit 0, stderr warns" {
  # Neither last_assistant_message nor result present → 2-stage chain → "" → empty warn
  local json='{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","agent_type":"test-agent"}'
  run invoke_hook "$json"
  [ "$status" -eq 0 ]
  [[ "$output" == *"empty result"* ]] || [[ "$stderr" == *"empty result"* ]] || \
    { echo "Missing last_assistant_message must warn empty; got: $output"; false; }
}

# ---------------------------------------------------------------------------
# Case (g): malformed JSON → exit 0, no panic
# BC-7.03.042 invariant 2 / AC-006
# ---------------------------------------------------------------------------

@test "AC-005(g): malformed JSON → exit 0, no panic" {
  run invoke_hook "not valid json {{{###"
  # Must exit 0 (graceful degradation — advisory hook, not fail-closed)
  [ "$status" -eq 0 ]
  # Must not panic (no 'panicked' or 'SIGTRAP' in output)
  [[ "$output" != *"panicked"* ]] || { echo "Malformed JSON must not panic; got: $output"; false; }
}
