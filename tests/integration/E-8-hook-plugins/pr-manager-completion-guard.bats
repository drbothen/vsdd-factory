#!/usr/bin/env bats
# tests/integration/E-8-hook-plugins/pr-manager-completion-guard.bats
#
# AC-006 bats parity tests for pr-manager-completion-guard native WASM plugin.
#
# Tests behavioral parity of the native WASM crate against the S-8.02 BCs:
#   BC-7.03.045 (identity & registry binding)
#   BC-7.03.046 (counts STEP_COMPLETE; passes if >= 8)
#   BC-7.03.047 (BLOCKED status is legitimate early exit)
#   BC-7.03.048 (blocks with step-specific continuation hint)
#   BC-2.02.012 (typed projection fallback chains)
#
# Invocation pattern: pipe JSON to factory-dispatcher with CLAUDE_PLUGIN_ROOT
# pointing at the worktree plugin directory. The dispatcher provides all WASM
# host functions (host::emit_event, block-mode envelope, stderr capture).
#
# Verification: The dispatcher captures plugin stderr in the internal events log.
# The hook.block event appears as a separate log line. The dispatcher exits 2
# when a hook returns Block with on_error=block behavior.
#
# Story: S-8.02 — Native port: pr-manager-completion-guard (SubagentStop)
# AC:    AC-006 (bats parity tests; all 9 step positions + edge cases)

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

  if [ ! -f "${PLUGIN_ROOT}/hook-plugins/pr-manager-completion-guard.wasm" ]; then
    skip "pr-manager-completion-guard.wasm not built: run 'cargo build --target wasm32-wasip1 --release -p pr-manager-completion-guard' and copy artifact"
  fi
}

# Helper: invoke dispatcher with a JSON envelope and capture results.
# After this call: $status = dispatcher exit code, $output = dispatcher stderr.
invoke_hook() {
  local json="$1"
  local json_file="${WORK}/payload.json"
  printf '%s' "$json" > "${json_file}"

  run env \
    CLAUDE_PLUGIN_ROOT="${PLUGIN_ROOT}" \
    CLAUDE_PROJECT_DIR="${WORK}" \
    bash -c "'${DISPATCHER}' < '${json_file}'"
}

# Helper: build a SubagentStop JSON envelope with given fields.
# Usage: make_payload '"agent_type":"pr-manager","last_assistant_message":"..."'
make_payload() {
  local extra="$1"
  printf '{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t",%s}' "$extra"
}

# Helper: assert dispatcher-internal log contains a hook.block entry for
# pr-manager-completion-guard with the given reason field value.
assert_hook_block_reason() {
  local reason="$1"
  local log="${WORK}/.factory/logs/dispatcher-internal-$(date +%Y-%m-%d).jsonl"
  if [ ! -f "${log}" ]; then
    echo "Dispatcher-internal log not found: ${log}"
    return 1
  fi
  if ! grep -q "\"reason\":\"${reason}\"" "${log}"; then
    echo "Expected hook.block reason='${reason}' not found in ${log}"
    echo "pr-manager-completion-guard entries:"
    grep '"plugin_name":"pr-manager-completion-guard"' "${log}" || echo "(no entries)"
    return 1
  fi
}

# Helper: assert dispatcher-internal log does NOT contain hook.block from
# pr-manager-completion-guard.
assert_no_hook_block() {
  local log="${WORK}/.factory/logs/dispatcher-internal-$(date +%Y-%m-%d).jsonl"
  if [ ! -f "${log}" ]; then
    return 0  # no log = no events = no block
  fi
  if grep '"type":"hook.block"' "${log}" 2>/dev/null | grep -q '"plugin_name":"pr-manager-completion-guard"'; then
    echo "Unexpected hook.block from pr-manager-completion-guard found in ${log}"
    grep '"pr-manager-completion-guard"' "${log}"
    return 1
  fi
}

# Helper: assert dispatcher-internal log contains a plugin.completed entry for
# pr-manager-completion-guard with the given string in the stderr field.
assert_plugin_stderr_contains() {
  local needle="$1"
  local log="${WORK}/.factory/logs/dispatcher-internal-$(date +%Y-%m-%d).jsonl"
  if [ ! -f "${log}" ]; then
    echo "Dispatcher-internal log not found: ${log}"
    return 1
  fi
  if ! grep '"plugin_name":"pr-manager-completion-guard"' "${log}" | grep -q "${needle}"; then
    echo "Expected stderr to contain '${needle}' in plugin.completed for pr-manager-completion-guard"
    echo "Log contents for pr-manager-completion-guard:"
    grep '"plugin_name":"pr-manager-completion-guard"' "${log}" || echo "(no entries)"
    return 1
  fi
}

# ---------------------------------------------------------------------------
# AC (a): < 8 STEP_COMPLETE lines → block emitted with hint
# BC-7.03.048 postcondition 1
# ---------------------------------------------------------------------------

@test "AC-006(a): 0 steps → exit 2, hook.block emitted, NEXT_STEP=1 hint" {
  local json
  json="$(make_payload '"agent_type":"pr-manager","last_assistant_message":"no steps yet"')"
  invoke_hook "$json"
  # Dispatcher exits 2 when Block hook fires with on_error=block
  [ "$status" -eq 2 ]
  assert_hook_block_reason "pr_manager_incomplete_lifecycle"
  assert_plugin_stderr_contains "CONTINUE TO STEP 1 NOW: populate PR description from template"
}

@test "AC-006(step1): STEP_COMPLETE for step=1 only → NEXT_STEP=2 hint" {
  local msg="STEP_COMPLETE: step=1 name=populate status=ok note="
  local json
  json="$(make_payload "\"agent_type\":\"pr-manager\",\"last_assistant_message\":\"${msg}\"")"
  invoke_hook "$json"
  [ "$status" -eq 2 ]
  assert_plugin_stderr_contains "CONTINUE TO STEP 2 NOW: verify demo evidence"
}

@test "AC-006(step2): STEP_COMPLETE for steps 1-2 → NEXT_STEP=3 hint" {
  local msg
  msg="$(printf 'STEP_COMPLETE: step=1\nSTEP_COMPLETE: step=2')"
  local json_file="${WORK}/payload.json"
  printf '{"event_name":"SubagentStop","session_id":"s","dispatcher_trace_id":"t","agent_type":"pr-manager","last_assistant_message":"%s"}' \
    "$(printf '%s' "$msg" | sed 's/\\/\\\\/g; s/"/\\"/g; s/$/\\n/g' | tr -d '\n' | sed 's/\\n$//')" \
    > "${json_file}"
  run env CLAUDE_PLUGIN_ROOT="${PLUGIN_ROOT}" CLAUDE_PROJECT_DIR="${WORK}" \
    bash -c "'${DISPATCHER}' < '${json_file}'"
  [ "$status" -eq 2 ]
  assert_plugin_stderr_contains "CONTINUE TO STEP 3 NOW: create PR via github-ops"
}

@test "AC-006(step3): STEP_COMPLETE for steps 1-3 → NEXT_STEP=4 hint" {
  local json_file="${WORK}/payload.json"
  python3 -c "
import json
msg = '\n'.join(['STEP_COMPLETE: step=%d' % i for i in range(1,4)])
payload = {'event_name':'SubagentStop','session_id':'s','dispatcher_trace_id':'t','agent_type':'pr-manager','last_assistant_message':msg}
print(json.dumps(payload))
" > "${json_file}"
  run env CLAUDE_PLUGIN_ROOT="${PLUGIN_ROOT}" CLAUDE_PROJECT_DIR="${WORK}" \
    bash -c "'${DISPATCHER}' < '${json_file}'"
  [ "$status" -eq 2 ]
  assert_plugin_stderr_contains "CONTINUE TO STEP 4 NOW: spawn security-reviewer via Agent tool"
}

@test "AC-006(step4): STEP_COMPLETE for steps 1-4 → NEXT_STEP=5 hint" {
  local json_file="${WORK}/payload.json"
  python3 -c "
import json
msg = '\n'.join(['STEP_COMPLETE: step=%d' % i for i in range(1,5)])
payload = {'event_name':'SubagentStop','session_id':'s','dispatcher_trace_id':'t','agent_type':'pr-manager','last_assistant_message':msg}
print(json.dumps(payload))
" > "${json_file}"
  run env CLAUDE_PLUGIN_ROOT="${PLUGIN_ROOT}" CLAUDE_PROJECT_DIR="${WORK}" \
    bash -c "'${DISPATCHER}' < '${json_file}'"
  [ "$status" -eq 2 ]
  assert_plugin_stderr_contains "CONTINUE TO STEP 5 NOW: spawn pr-reviewer/pr-review-triage via Agent tool"
}

@test "AC-006(step5): STEP_COMPLETE for steps 1-5 → NEXT_STEP=6 hint" {
  local json_file="${WORK}/payload.json"
  python3 -c "
import json
msg = '\n'.join(['STEP_COMPLETE: step=%d' % i for i in range(1,6)])
payload = {'event_name':'SubagentStop','session_id':'s','dispatcher_trace_id':'t','agent_type':'pr-manager','last_assistant_message':msg}
print(json.dumps(payload))
" > "${json_file}"
  run env CLAUDE_PLUGIN_ROOT="${PLUGIN_ROOT}" CLAUDE_PROJECT_DIR="${WORK}" \
    bash -c "'${DISPATCHER}' < '${json_file}'"
  [ "$status" -eq 2 ]
  assert_plugin_stderr_contains "CONTINUE TO STEP 6 NOW: spawn github-ops: gh pr checks --watch"
}

@test "AC-006(step6): STEP_COMPLETE for steps 1-6 → NEXT_STEP=7 hint" {
  local json_file="${WORK}/payload.json"
  python3 -c "
import json
msg = '\n'.join(['STEP_COMPLETE: step=%d' % i for i in range(1,7)])
payload = {'event_name':'SubagentStop','session_id':'s','dispatcher_trace_id':'t','agent_type':'pr-manager','last_assistant_message':msg}
print(json.dumps(payload))
" > "${json_file}"
  run env CLAUDE_PLUGIN_ROOT="${PLUGIN_ROOT}" CLAUDE_PROJECT_DIR="${WORK}" \
    bash -c "'${DISPATCHER}' < '${json_file}'"
  [ "$status" -eq 2 ]
  assert_plugin_stderr_contains "CONTINUE TO STEP 7 NOW: verify all dependency PRs merged"
}

@test "AC-006(step7 / EC-002): STEP_COMPLETE for steps 1-7 → NEXT_STEP=8 hint" {
  local json_file="${WORK}/payload.json"
  python3 -c "
import json
msg = '\n'.join(['STEP_COMPLETE: step=%d' % i for i in range(1,8)])
payload = {'event_name':'SubagentStop','session_id':'s','dispatcher_trace_id':'t','agent_type':'pr-manager','last_assistant_message':msg}
print(json.dumps(payload))
" > "${json_file}"
  run env CLAUDE_PLUGIN_ROOT="${PLUGIN_ROOT}" CLAUDE_PROJECT_DIR="${WORK}" \
    bash -c "'${DISPATCHER}' < '${json_file}'"
  [ "$status" -eq 2 ]
  assert_plugin_stderr_contains "CONTINUE TO STEP 8 NOW: spawn github-ops: gh pr merge --squash --delete-branch"
}

@test "AC-006(step8 only, last_step=8): 1 step but step=8 → NEXT_STEP=9 hint" {
  local json_file="${WORK}/payload.json"
  python3 -c "
import json
msg = 'STEP_COMPLETE: step=8 name=merge status=ok note='
payload = {'event_name':'SubagentStop','session_id':'s','dispatcher_trace_id':'t','agent_type':'pr-manager','last_assistant_message':msg}
print(json.dumps(payload))
" > "${json_file}"
  run env CLAUDE_PLUGIN_ROOT="${PLUGIN_ROOT}" CLAUDE_PROJECT_DIR="${WORK}" \
    bash -c "'${DISPATCHER}' < '${json_file}'"
  [ "$status" -eq 2 ]
  assert_plugin_stderr_contains "CONTINUE TO STEP 9 NOW: confirm branch deletion"
}

# ---------------------------------------------------------------------------
# AC (b): >= 8 STEP_COMPLETE lines → continue silently (exit 0)
# BC-7.03.046 postcondition 1
# ---------------------------------------------------------------------------

@test "AC-006(b): 9 STEP_COMPLETE lines → exit 0, no hook.block" {
  local json_file="${WORK}/payload.json"
  python3 -c "
import json
msg = '\n'.join(['STEP_COMPLETE: step=%d' % i for i in range(1,10)])
payload = {'event_name':'SubagentStop','session_id':'s','dispatcher_trace_id':'t','agent_type':'pr-manager','last_assistant_message':msg}
print(json.dumps(payload))
" > "${json_file}"
  run env CLAUDE_PLUGIN_ROOT="${PLUGIN_ROOT}" CLAUDE_PROJECT_DIR="${WORK}" \
    bash -c "'${DISPATCHER}' < '${json_file}'"
  [ "$status" -eq 0 ]
  assert_no_hook_block
}

@test "AC-006(b EC-005): exactly 8 STEP_COMPLETE lines → exit 0, no hook.block" {
  local json_file="${WORK}/payload.json"
  python3 -c "
import json
msg = '\n'.join(['STEP_COMPLETE: step=%d' % i for i in range(1,9)])
payload = {'event_name':'SubagentStop','session_id':'s','dispatcher_trace_id':'t','agent_type':'pr-manager','last_assistant_message':msg}
print(json.dumps(payload))
" > "${json_file}"
  run env CLAUDE_PLUGIN_ROOT="${PLUGIN_ROOT}" CLAUDE_PROJECT_DIR="${WORK}" \
    bash -c "'${DISPATCHER}' < '${json_file}'"
  [ "$status" -eq 0 ]
  assert_no_hook_block
}

# ---------------------------------------------------------------------------
# AC (c): BLOCKED status → continue silently (legitimate early exit)
# BC-7.03.047 postcondition 1
# ---------------------------------------------------------------------------

@test "AC-006(c): 'Status: BLOCKED' result → exit 0, no hook.block" {
  local json_file="${WORK}/payload.json"
  python3 -c "
import json
msg = 'Status: BLOCKED\nDependency PR not yet merged.'
payload = {'event_name':'SubagentStop','session_id':'s','dispatcher_trace_id':'t','agent_type':'pr-manager','last_assistant_message':msg}
print(json.dumps(payload))
" > "${json_file}"
  run env CLAUDE_PLUGIN_ROOT="${PLUGIN_ROOT}" CLAUDE_PROJECT_DIR="${WORK}" \
    bash -c "'${DISPATCHER}' < '${json_file}'"
  [ "$status" -eq 0 ]
  assert_no_hook_block
}

@test "AC-006(c): bare 'BLOCKED' at line start → exit 0, no hook.block" {
  local json_file="${WORK}/payload.json"
  python3 -c "
import json
msg = 'BLOCKED: waiting for security reviewer approval'
payload = {'event_name':'SubagentStop','session_id':'s','dispatcher_trace_id':'t','agent_type':'pr-manager','last_assistant_message':msg}
print(json.dumps(payload))
" > "${json_file}"
  run env CLAUDE_PLUGIN_ROOT="${PLUGIN_ROOT}" CLAUDE_PROJECT_DIR="${WORK}" \
    bash -c "'${DISPATCHER}' < '${json_file}'"
  [ "$status" -eq 0 ]
  assert_no_hook_block
}

# ---------------------------------------------------------------------------
# AC (d): malformed JSON → continue silently (graceful degradation)
# BC-7.03.045 invariant 2 / AC-008
# ---------------------------------------------------------------------------

@test "AC-006(d): malformed JSON → exit 0, no panic" {
  invoke_hook "not valid json {{{###"
  [ "$status" -eq 0 ]
  [[ "$output" != *"panicked"* ]] || { echo "Malformed JSON must not panic; got: $output"; false; }
}

# ---------------------------------------------------------------------------
# Non-pr-manager agent → exit 0, no block (AC-006 scope check)
# BC-7.03.045: non-pr-manager passes through
# ---------------------------------------------------------------------------

@test "AC-006: non-pr-manager agent → exit 0, no hook.block" {
  local json
  json="$(make_payload '"agent_type":"product-owner","last_assistant_message":"no steps"')"
  invoke_hook "$json"
  [ "$status" -eq 0 ]
  assert_no_hook_block
}

# ---------------------------------------------------------------------------
# Wildcard arm: NEXT_STEP=10 (F-S802-P2-001 coverage)
# LAST_STEP=9, STEP_COUNT=7 due to duplicate step numbers
# BC-7.03.048 EC-006 / AC-006 wildcard arm coverage
# ---------------------------------------------------------------------------

@test "AC-006(wildcard NEXT_STEP=10): 7 lines with max step=9 → wildcard hint" {
  # 7 STEP_COMPLETE lines (count < 8) but highest step number is 9
  # → LAST_STEP=9, NEXT_STEP=10 → wildcard arm fires
  local json_file="${WORK}/payload.json"
  python3 -c "
import json
# 7 lines: steps 3,4,5,6,7,8,9 (step=9 is highest; count=7 < 8)
lines = ['STEP_COMPLETE: step=%d' % i for i in [3,4,5,6,7,8,9]]
msg = '\n'.join(lines)
payload = {'event_name':'SubagentStop','session_id':'s','dispatcher_trace_id':'t','agent_type':'pr-manager','last_assistant_message':msg}
print(json.dumps(payload))
" > "${json_file}"
  run env CLAUDE_PLUGIN_ROOT="${PLUGIN_ROOT}" CLAUDE_PROJECT_DIR="${WORK}" \
    bash -c "'${DISPATCHER}' < '${json_file}'"
  [ "$status" -eq 2 ]
  assert_plugin_stderr_contains "CONTINUE TO STEP 10 NOW: continue the 9-step lifecycle"
}

# ---------------------------------------------------------------------------
# Wildcard arm: NEXT_STEP=99 (F-S802-P2-001 coverage)
# LAST_STEP=98, STEP_COUNT=3
# BC-7.03.048 AC-006 wildcard arm coverage
# ---------------------------------------------------------------------------

@test "AC-006(wildcard NEXT_STEP=99): 3 lines with max step=98 → wildcard hint" {
  # 3 STEP_COMPLETE lines (count < 8); highest step=98
  # → LAST_STEP=98, NEXT_STEP=99 → wildcard arm fires
  local json_file="${WORK}/payload.json"
  python3 -c "
import json
lines = ['STEP_COMPLETE: step=%d' % i for i in [96,97,98]]
msg = '\n'.join(lines)
payload = {'event_name':'SubagentStop','session_id':'s','dispatcher_trace_id':'t','agent_type':'pr-manager','last_assistant_message':msg}
print(json.dumps(payload))
" > "${json_file}"
  run env CLAUDE_PLUGIN_ROOT="${PLUGIN_ROOT}" CLAUDE_PROJECT_DIR="${WORK}" \
    bash -c "'${DISPATCHER}' < '${json_file}'"
  [ "$status" -eq 2 ]
  assert_plugin_stderr_contains "CONTINUE TO STEP 99 NOW: continue the 9-step lifecycle"
}

# ---------------------------------------------------------------------------
# EC-001: underscore variant pr_manager → scoped (hook applies)
# BC-7.03.045 EC-001
# ---------------------------------------------------------------------------

@test "AC-006(EC-001): agent_type with pr_manager underscore → hook applies, blocks" {
  local json
  json="$(make_payload '"agent_type":"pr_manager_agent","last_assistant_message":"no steps"')"
  invoke_hook "$json"
  [ "$status" -eq 2 ]
  assert_hook_block_reason "pr_manager_incomplete_lifecycle"
}

# ---------------------------------------------------------------------------
# BC-2.02.012 Postcondition 5: subagent_name fallback
# ---------------------------------------------------------------------------

@test "AC-006(BC-2.02.012): agent_type absent, subagent_name=pr-manager-fallback → scoped" {
  local json
  json="$(make_payload '"subagent_name":"pr-manager-fallback","last_assistant_message":"no steps"')"
  invoke_hook "$json"
  [ "$status" -eq 2 ]
  assert_hook_block_reason "pr_manager_incomplete_lifecycle"
}
