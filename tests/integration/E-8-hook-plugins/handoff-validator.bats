#!/usr/bin/env bats
# tests/integration/E-8-hook-plugins/handoff-validator.bats
#
# AC-005 bats parity tests for handoff-validator native WASM plugin.
#
# Tests behavioral parity of the native WASM crate against the 7 canonical
# test cases from S-8.01 T-5 (BC-7.03.043 + BC-7.03.044).
#
# Invocation pattern: pipe JSON to factory-dispatcher with CLAUDE_PLUGIN_ROOT
# pointing at the worktree plugin directory. The dispatcher provides all WASM
# host functions (host::emit_event, block-mode envelope, stderr capture).
# Bare wasmtime does NOT provide these host fns — always use the dispatcher.
#
# Verification: The dispatcher captures plugin stderr in the events log at
#   $WORK/.factory/logs/events-YYYY-MM-DD.jsonl
# as the "stderr" field of plugin.completed events. The hook.block event
# also appears as a separate log line. We grep the events log to assert
# the expected behavior.
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
# Setup
# ---------------------------------------------------------------------------

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  DISPATCHER="${REPO_ROOT}/target/release/factory-dispatcher"
  PLUGIN_ROOT="${REPO_ROOT}/plugins/vsdd-factory"

  # Unique work dir per test to avoid log file collisions
  WORK="${BATS_TEST_TMPDIR}/proj"
  mkdir -p "${WORK}/.factory/logs"

  if [ ! -x "${DISPATCHER}" ]; then
    skip "dispatcher not built: run 'cargo build --release -p factory-dispatcher' first"
  fi

  # Verify the native WASM artifact is in the plugin dir
  if [ ! -f "${PLUGIN_ROOT}/hook-plugins/handoff-validator.wasm" ]; then
    skip "handoff-validator.wasm not built: run 'cargo build --target wasm32-wasip1 --release -p handoff-validator' and copy artifact"
  fi
}

# Helper: invoke dispatcher with a JSON envelope and capture results.
# After this call: $status = dispatcher exit code, $output = dispatcher stderr.
# The dispatcher writes hook events to:
#   $WORK/.factory/logs/dispatcher-internal-YYYY-MM-DD.jsonl
invoke_hook() {
  local json="$1"
  # Write JSON to a temp file to avoid shell quoting issues when passing to
  # the dispatcher via stdin through bash -c.
  local json_file="${WORK}/payload.json"
  printf '%s' "$json" > "${json_file}"

  run env \
    CLAUDE_PLUGIN_ROOT="${PLUGIN_ROOT}" \
    CLAUDE_PROJECT_DIR="${WORK}" \
    bash -c "'${DISPATCHER}' < '${json_file}'"
}

# Helper: assert dispatcher-internal log contains a hook.block entry for
# handoff-validator with the given reason field value.
# The dispatcher writes plugin.invoked, hook.block, and plugin.completed events
# to dispatcher-internal-YYYY-MM-DD.jsonl (not events-YYYY-MM-DD.jsonl).
assert_hook_block_reason() {
  local reason="$1"
  local log="${WORK}/.factory/logs/dispatcher-internal-$(date +%Y-%m-%d).jsonl"
  if [ ! -f "${log}" ]; then
    echo "Dispatcher-internal log not found: ${log}"
    return 1
  fi
  if ! grep -q "\"reason\":\"${reason}\"" "${log}"; then
    echo "Expected hook.block reason='${reason}' not found in ${log}"
    echo "handoff-validator entries:"
    grep '"plugin_name":"handoff-validator"' "${log}" || echo "(no entries)"
    return 1
  fi
}

# Helper: assert dispatcher-internal log does NOT contain any hook.block from handoff-validator.
assert_no_hook_block() {
  local log="${WORK}/.factory/logs/dispatcher-internal-$(date +%Y-%m-%d).jsonl"
  if [ ! -f "${log}" ]; then
    # No log = no events = no block. Pass.
    return 0
  fi
  # Check there is no hook.block from handoff-validator
  if grep '"type":"hook.block"' "${log}" 2>/dev/null | grep -q '"plugin_name":"handoff-validator"'; then
    echo "Unexpected hook.block from handoff-validator found in ${log}"
    grep '"handoff-validator"' "${log}"
    return 1
  fi
}

# Helper: assert dispatcher-internal log contains a plugin.completed entry for
# handoff-validator with the given string in the stderr field.
# The dispatcher captures plugin stderr and stores it as "stderr":"..." in
# plugin.completed log entries.
assert_plugin_stderr_contains() {
  local needle="$1"
  local log="${WORK}/.factory/logs/dispatcher-internal-$(date +%Y-%m-%d).jsonl"
  if [ ! -f "${log}" ]; then
    echo "Dispatcher-internal log not found: ${log}"
    return 1
  fi
  # The stderr field is JSON-encoded: newlines become \n, etc.
  # We grep for the needle as a literal substring in the JSON line.
  if ! grep '"plugin_name":"handoff-validator"' "${log}" | grep -q "${needle}"; then
    echo "Expected stderr to contain '${needle}' in plugin.completed for handoff-validator"
    echo "Log contents for handoff-validator:"
    grep '"plugin_name":"handoff-validator"' "${log}" || echo "(no entries)"
    return 1
  fi
}

# Helper: assert dispatcher-internal log does NOT have the given string in
# handoff-validator stderr.
assert_plugin_stderr_not_contains() {
  local needle="$1"
  local log="${WORK}/.factory/logs/dispatcher-internal-$(date +%Y-%m-%d).jsonl"
  if [ ! -f "${log}" ]; then
    return 0  # no log = no stderr = assertion trivially passes
  fi
  if grep '"plugin_name":"handoff-validator"' "${log}" | grep -q "${needle}"; then
    echo "Unexpected stderr content '${needle}' found in plugin.completed for handoff-validator"
    return 1
  fi
}

# ---------------------------------------------------------------------------
# Case (a): empty result → exit 0 + stderr contains "empty result"
# BC-7.03.043 postcondition 1
# ---------------------------------------------------------------------------

@test "AC-005(a): empty last_assistant_message → exit 0, stderr contains 'empty result'" {
  local json='{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","agent_type":"test-agent","last_assistant_message":""}'
  invoke_hook "$json"
  [ "$status" -eq 0 ]
  assert_hook_block_reason "subagent_empty_result"
  assert_plugin_stderr_contains "empty result"
}

# ---------------------------------------------------------------------------
# Case (b): result with 5 non-whitespace chars → exit 0 + stderr contains
#           "non-whitespace characters"
# BC-7.03.044 postcondition 1
# ---------------------------------------------------------------------------

@test "AC-005(b): 5-char result → exit 0, stderr contains 'non-whitespace characters'" {
  local json='{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","agent_type":"test-agent","last_assistant_message":"hello"}'
  invoke_hook "$json"
  [ "$status" -eq 0 ]
  assert_hook_block_reason "subagent_truncated_result"
  assert_plugin_stderr_contains "non-whitespace characters"
}

# ---------------------------------------------------------------------------
# Case (c): result with 50 non-whitespace chars → exit 0, no stderr warning
# BC-7.03.042 postcondition 2
# ---------------------------------------------------------------------------

@test "AC-005(c): 50-char result → exit 0, no stderr warning" {
  local msg
  msg="$(printf 'a%.0s' {1..50})"
  local json="{\"event_name\":\"SubagentStop\",\"session_id\":\"s\",\"dispatcher_trace_id\":\"t\",\"agent_type\":\"test-agent\",\"last_assistant_message\":\"${msg}\"}"
  invoke_hook "$json"
  [ "$status" -eq 0 ]
  assert_no_hook_block
  assert_plugin_stderr_not_contains "non-whitespace"
  assert_plugin_stderr_not_contains "empty result"
}

# ---------------------------------------------------------------------------
# Case (d): result with exactly 39 non-whitespace chars → exit 0 + stderr warning
# BC-7.03.044 postcondition 1 (below threshold)
# ---------------------------------------------------------------------------

@test "AC-005(d): LEN=39 → exit 0, warning emitted (below threshold)" {
  local msg
  msg="$(printf 'a%.0s' {1..39})"
  local json="{\"event_name\":\"SubagentStop\",\"session_id\":\"s\",\"dispatcher_trace_id\":\"t\",\"agent_type\":\"test-agent\",\"last_assistant_message\":\"${msg}\"}"
  invoke_hook "$json"
  [ "$status" -eq 0 ]
  assert_hook_block_reason "subagent_truncated_result"
  assert_plugin_stderr_contains "non-whitespace characters"
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
  invoke_hook "$json"
  [ "$status" -eq 0 ]
  assert_no_hook_block
  assert_plugin_stderr_not_contains "non-whitespace"
  assert_plugin_stderr_not_contains "empty result"
}

# ---------------------------------------------------------------------------
# Case (f): JSON with missing last_assistant_message → exit 0, stderr warns
# BC-2.02.012 Postcondition 6 / EC-001: missing field → fallback to result → empty
# BC-7.03.043 postcondition 1
# ---------------------------------------------------------------------------

@test "AC-005(f): missing last_assistant_message field → exit 0, stderr warns" {
  # Neither last_assistant_message nor result present → 2-stage chain → "" → empty warn
  local json='{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","agent_type":"test-agent"}'
  invoke_hook "$json"
  [ "$status" -eq 0 ]
  assert_hook_block_reason "subagent_empty_result"
  assert_plugin_stderr_contains "empty result"
}

# ---------------------------------------------------------------------------
# Case (g): malformed JSON → exit 0, no panic
# BC-7.03.042 invariant 2 / AC-006
# ---------------------------------------------------------------------------

@test "AC-005(g): malformed JSON → exit 0, no panic" {
  invoke_hook "not valid json {{{###"
  # Dispatcher exits 0 (graceful degradation — dispatcher itself handles malformed JSON)
  [ "$status" -eq 0 ]
  # Must not panic (no 'panicked' in output)
  [[ "$output" != *"panicked"* ]] || { echo "Malformed JSON must not panic; got: $output"; false; }
}
