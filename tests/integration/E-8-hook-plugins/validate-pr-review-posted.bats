#!/usr/bin/env bats
# tests/integration/E-8-hook-plugins/validate-pr-review-posted.bats
#
# AC-007 bats parity tests for validate-pr-review-posted native WASM plugin.
#
# Tests behavioral parity of the native WASM crate against the 7 canonical
# test cases from AC-007 (BC-7.04.040..044 + BC-2.02.012 postconditions 5-6).
#
# Invocation pattern: pipe JSON to factory-dispatcher with CLAUDE_PLUGIN_ROOT
# pointing at the worktree plugin directory. The dispatcher provides all WASM
# host functions (host::emit_event, block-mode envelope, stderr capture).
# Bare wasmtime does NOT provide these host fns — always use the dispatcher.
#
# Advisory block-mode: the hook emits hook.block + writes stderr, then exits 0.
# The dispatcher writes hook events to:
#   $WORK/.factory/logs/dispatcher-internal-YYYY-MM-DD.jsonl
# We grep the dispatcher-internal log to assert expected behavior.
#
# Cases covered (AC-007):
#   (a) all three checks pass → exit 0, no block event
#   (b) pr-review.md not written → exit 0, check1 error in stderr
#   (c) gh pr comment used → exit 0, check2 error in stderr
#   (d) no formal review → exit 0, check3a error in stderr
#   (e) review posted, no verdict → exit 0, check3b error in stderr
#   (f) multiple checks fail → exit 0, all accumulated errors in stderr
#   (g) non-pr-reviewer agent → exit 0, no block event
#   (g.1) agent_type present, subagent_name absent — primary arm of BC-2.02.012 chain
#   (g.2) agent_type absent, subagent_name present — fallback arm exercised
#
# BC traces:
#   BC-7.04.040 postcondition 1 (identity & registry binding)
#   BC-7.04.041 postcondition 1 (agent scope — cases g, g.1, g.2)
#   BC-7.04.042 postcondition 1 (check 1 — case b)
#   BC-7.04.043 postcondition 1 (check 2 — case c)
#   BC-7.04.044 postcondition 1 (check 3a/3b — cases d, e)
#   BC-2.02.012 postconditions 5-6 (fallback chains — cases g.1, g.2)
#
# Story: S-8.05 — Native port: validate-pr-review-posted (SubagentStop)
# AC:    AC-007 (T-7 parity tests)

# ---------------------------------------------------------------------------
# Setup
# ---------------------------------------------------------------------------

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  DISPATCHER="${REPO_ROOT}/target/release/factory-dispatcher"
  PLUGIN_ROOT="${REPO_ROOT}/plugins/vsdd-factory"

  # Fall back to debug build if release not present
  if [ ! -x "${DISPATCHER}" ]; then
    DISPATCHER="${REPO_ROOT}/target/debug/factory-dispatcher"
  fi

  # Per test work dir to avoid log file collisions
  WORK="${BATS_TEST_TMPDIR}/proj"
  mkdir -p "${WORK}/.factory/logs"

  if [ ! -x "${DISPATCHER}" ]; then
    skip "factory-dispatcher not built: run 'cargo build --release -p factory-dispatcher' first"
  fi

  # Verify the native WASM artifact is present
  if [ ! -f "${PLUGIN_ROOT}/hook-plugins/validate-pr-review-posted.wasm" ]; then
    skip "validate-pr-review-posted.wasm not built — run: cargo build --target wasm32-wasip1 --release -p validate-pr-review-posted && cp target/wasm32-wasip1/release/validate-pr-review-posted.wasm plugins/vsdd-factory/hook-plugins/"
  fi
}

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

# Invoke dispatcher with a JSON envelope.
# After this call: $status = dispatcher exit code, $output = dispatcher output.
invoke_hook() {
  local json="$1"
  local json_file="${WORK}/payload.json"
  printf '%s' "$json" > "${json_file}"
  run env \
    CLAUDE_PLUGIN_ROOT="${PLUGIN_ROOT}" \
    CLAUDE_PROJECT_DIR="${WORK}" \
    bash -c "'${DISPATCHER}' < '${json_file}'"
}

# Assert dispatcher-internal log contains a hook.block entry with pr_review_not_posted reason.
assert_hook_block_pr_review() {
  local log="${WORK}/.factory/logs/dispatcher-internal-$(date +%Y-%m-%d).jsonl"
  if [ ! -f "${log}" ]; then
    echo "Dispatcher-internal log not found: ${log}"
    return 1
  fi
  if ! grep -q '"reason":"pr_review_not_posted"' "${log}"; then
    echo "Expected hook.block reason='pr_review_not_posted' not found in ${log}"
    echo "validate-pr-review-posted entries:"
    grep '"plugin_name":"validate-pr-review-posted"' "${log}" || echo "(no entries)"
    return 1
  fi
}

# Assert no hook.block from validate-pr-review-posted in the log.
assert_no_hook_block() {
  local log="${WORK}/.factory/logs/dispatcher-internal-$(date +%Y-%m-%d).jsonl"
  if [ ! -f "${log}" ]; then
    return 0
  fi
  if grep '"type":"hook.block"' "${log}" 2>/dev/null | grep -q '"plugin_name":"validate-pr-review-posted"'; then
    echo "Unexpected hook.block from validate-pr-review-posted found in ${log}"
    grep '"validate-pr-review-posted"' "${log}"
    return 1
  fi
}

# Assert plugin.completed stderr field contains the given needle.
assert_stderr_contains() {
  local needle="$1"
  local log="${WORK}/.factory/logs/dispatcher-internal-$(date +%Y-%m-%d).jsonl"
  if [ ! -f "${log}" ]; then
    echo "Dispatcher-internal log not found: ${log}"
    return 1
  fi
  if ! grep '"plugin_name":"validate-pr-review-posted"' "${log}" | grep -q "${needle}"; then
    echo "Expected stderr to contain '${needle}'"
    echo "Log entries for validate-pr-review-posted:"
    grep '"plugin_name":"validate-pr-review-posted"' "${log}" || echo "(no entries)"
    return 1
  fi
}

# Assert plugin.completed stderr does NOT contain the given needle.
assert_stderr_not_contains() {
  local needle="$1"
  local log="${WORK}/.factory/logs/dispatcher-internal-$(date +%Y-%m-%d).jsonl"
  if [ ! -f "${log}" ]; then
    return 0
  fi
  if grep '"plugin_name":"validate-pr-review-posted"' "${log}" | grep -q "${needle}"; then
    echo "Unexpected stderr content '${needle}' in validate-pr-review-posted entry"
    return 1
  fi
}

# ---------------------------------------------------------------------------
# AC-001: hooks-registry.toml references native WASM (not legacy-bash-adapter)
# ---------------------------------------------------------------------------

@test "AC-001: hooks-registry.toml validate-pr-review-posted references native WASM" {
  local stanza
  stanza="$(grep -A 6 'name = "validate-pr-review-posted"' "${PLUGIN_ROOT}/hooks-registry.toml")"
  echo "$stanza" | grep -q 'validate-pr-review-posted.wasm'
  ! echo "$stanza" | grep -q 'legacy-bash-adapter'
}

@test "AC-001: hooks-registry.toml validate-pr-review-posted has no script_path" {
  local stanza
  stanza="$(awk '/name = "validate-pr-review-posted"/,/^\[\[hooks\]\]/' "${PLUGIN_ROOT}/hooks-registry.toml" | head -20)"
  ! echo "$stanza" | grep -q 'script_path'
}

@test "AC-001: hooks-registry.toml validate-pr-review-posted has no exec_subprocess block" {
  local stanza
  stanza="$(awk '/name = "validate-pr-review-posted"/,/^\[\[hooks\]\]/' "${PLUGIN_ROOT}/hooks-registry.toml" | head -20)"
  ! echo "$stanza" | grep -q 'exec_subprocess'
}

# ---------------------------------------------------------------------------
# AC-002: hooks.json entry absent + .sh deleted
# ---------------------------------------------------------------------------

@test "AC-002: hooks.json does not contain validate-pr-review-posted command entry" {
  ! grep -q "validate-pr-review-posted" "${PLUGIN_ROOT}/hooks/hooks.json"
}

@test "AC-002: validate-pr-review-posted.sh is deleted" {
  [ ! -f "${PLUGIN_ROOT}/hooks/validate-pr-review-posted.sh" ]
}

# ---------------------------------------------------------------------------
# Case (a): all checks pass → exit 0, no hook.block event
# BC-7.04.040/042/043/044 postcondition 1
# ---------------------------------------------------------------------------

@test "AC-007(a): all checks pass → exit 0, no hook.block event" {
  # Case (a) all-pass concrete input from story spec AC-007
  local json
  json='{"event_name":"SubagentStop","session_id":"test-sess","dispatcher_trace_id":"test-trace","agent_type":"pr-reviewer","last_assistant_message":"wrote pr-review.md and posted gh pr review --approve"}'
  invoke_hook "$json"
  [ "$status" -eq 0 ]
  assert_no_hook_block
}

# ---------------------------------------------------------------------------
# Case (b): pr-review.md not written → exit 0 + check1 error
# BC-7.04.042 postcondition 1
# ---------------------------------------------------------------------------

@test "AC-007(b): pr-review.md not written → exit 0, check1 error in stderr" {
  local json
  json='{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","agent_type":"pr-reviewer","last_assistant_message":"ran gh pr review --approve posted"}'
  invoke_hook "$json"
  [ "$status" -eq 0 ]
  assert_hook_block_pr_review
  assert_stderr_contains "pr-review.md"
}

# ---------------------------------------------------------------------------
# Case (c): gh pr comment used → exit 0 + check2 error
# BC-7.04.043 postcondition 1
# ---------------------------------------------------------------------------

@test "AC-007(c): gh pr comment used → exit 0, check2 error in stderr" {
  local json
  json='{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","agent_type":"pr-reviewer","last_assistant_message":"wrote pr-review.md and ran gh pr comment --body findings"}'
  invoke_hook "$json"
  [ "$status" -eq 0 ]
  assert_hook_block_pr_review
  assert_stderr_contains "gh pr comment"
}

# ---------------------------------------------------------------------------
# Case (d): no formal review posted → exit 0 + check3a error
# BC-7.04.044 postcondition 1 (check 3a)
# ---------------------------------------------------------------------------

@test "AC-007(d): no formal review posted → exit 0, check3a error in stderr" {
  local json
  json='{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","agent_type":"pr-reviewer","last_assistant_message":"wrote pr-review.md and submitted my findings"}'
  invoke_hook "$json"
  [ "$status" -eq 0 ]
  assert_hook_block_pr_review
  assert_stderr_contains "formal GitHub review"
}

# ---------------------------------------------------------------------------
# Case (e): review posted, no verdict → exit 0 + check3b error only
# BC-7.04.044 postcondition 1 (check 3b)
# Case (e) concrete input from story spec AC-007
# ---------------------------------------------------------------------------

@test "AC-007(e): gh pr review with no verdict → exit 0, check3b error in stderr" {
  # gh pr review --no-body: matches gh pr review (check 3a passes, no error)
  # but no approve/request-changes → check 3b fires
  local json
  json='{"event_name":"SubagentStop","session_id":"test-sess","dispatcher_trace_id":"test-trace","agent_type":"pr-reviewer","last_assistant_message":"ran gh pr review --no-body"}'
  invoke_hook "$json"
  [ "$status" -eq 0 ]
  assert_hook_block_pr_review
  assert_stderr_contains "no verdict"
  # Check 3a must NOT have fired (gh pr review was present)
  assert_stderr_not_contains "formal GitHub review"
}

# ---------------------------------------------------------------------------
# Case (f): multiple checks fail → exit 0 + all accumulated errors
# BC-7.04.042/043/044 postcondition 1 (EC-002)
# ---------------------------------------------------------------------------

@test "AC-007(f): multiple checks fail → exit 0, all errors accumulated in stderr" {
  # No pr-review.md + uses gh pr comment + no formal review
  local json
  json='{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","agent_type":"pr-reviewer","last_assistant_message":"ran gh pr comment --body my findings"}'
  invoke_hook "$json"
  [ "$status" -eq 0 ]
  assert_hook_block_pr_review
  assert_stderr_contains "pr-review.md"
  assert_stderr_contains "gh pr comment"
  assert_stderr_contains "formal GitHub review"
}

# ---------------------------------------------------------------------------
# Case (g): non-pr-reviewer agent → exit 0, no block event
# BC-7.04.041 postcondition 1
# ---------------------------------------------------------------------------

@test "AC-007(g): non-pr-reviewer agent → exit 0, no hook.block event" {
  local json
  json='{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","agent_type":"code-reviewer","last_assistant_message":"done nothing"}'
  invoke_hook "$json"
  [ "$status" -eq 0 ]
  assert_no_hook_block
}

# ---------------------------------------------------------------------------
# Case (g.1): agent_type present, subagent_name absent — primary BC-2.02.012 arm
# BC-2.02.012 Postcondition 5
# ---------------------------------------------------------------------------

@test "AC-007(g.1): agent_type=pr-reviewer present (primary chain arm) → checks applied" {
  local json
  json='{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","agent_type":"pr-reviewer","last_assistant_message":"done nothing"}'
  invoke_hook "$json"
  [ "$status" -eq 0 ]
  # Checks applied → should emit block (no pr-review.md, no formal review)
  assert_hook_block_pr_review
}

# ---------------------------------------------------------------------------
# Case (g.2): agent_type absent, subagent_name=pr-reviewer — fallback BC-2.02.012 arm
# BC-2.02.012 Postcondition 5
# ---------------------------------------------------------------------------

@test "AC-007(g.2): subagent_name=pr-reviewer (fallback chain arm) → checks applied" {
  local json
  json='{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","subagent_name":"pr-reviewer","last_assistant_message":"done nothing"}'
  invoke_hook "$json"
  [ "$status" -eq 0 ]
  # subagent_name fallback resolves to pr-reviewer → checks applied → block
  assert_hook_block_pr_review
}
