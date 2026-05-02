#!/usr/bin/env bats
# tests/integration/E-8-hook-plugins/regression-gate.bats
#
# AC-007 bats parity tests for regression-gate native WASM plugin.
#
# Covers all 9 scenarios from S-8.09 AC-007:
#   (a) cargo test pass, no prior state file
#   (b) cargo test pass via interrupted=false, no prior state
#   (c) cargo test fail exit_code=1, prior state=pass → regression warning
#   (d) cargo test interrupted=true, prior state=pass → regression warning
#   (e) cargo test, no exit_code + no interrupted → unknown → no state write
#   (f) non-test command (git commit) → exit 0, no state write
#   (g) .factory/ absent → exit 0, no state write
#   (h) cargo test fail, prior state=fail → no regression warning
#   (i) pytest pass → pattern coverage for non-cargo runner
#
# Plus AC-001 registry + AC-002 file deletion checks.
#
# Invocation pattern: pipe JSON to factory-dispatcher with CLAUDE_PLUGIN_ROOT
# pointing at the worktree plugin directory. The dispatcher provides all WASM
# host functions (host::read_file, host::write_file, host::emit_event).
#
# BC traces:
#   BC-7.03.071 postcondition 1 (exit 0 always; AC-001 registry binding)
#   BC-7.03.072 postcondition 1 (AC-003: Bash guard + 9-pattern match)
#   BC-7.03.073 postcondition 1 (AC-004: status derivation cascade)
#   BC-7.03.074 postcondition 1 (AC-005: state file write + timestamp)
#   BC-7.03.075 postcondition 1 (AC-006: pass-to-fail warning)
#
# Story: S-8.09
# BCs: BC-7.03.071, BC-7.03.072, BC-7.03.073, BC-7.03.074, BC-7.03.075

# ---------------------------------------------------------------------------
# Setup: locate dispatcher binary and configure project dir
# ---------------------------------------------------------------------------

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  DISPATCHER="${REPO_ROOT}/target/release/factory-dispatcher"
  PLUGIN_ROOT="${REPO_ROOT}/plugins/vsdd-factory"
  REGISTRY="${PLUGIN_ROOT}/hooks-registry.toml"

  # Fall back to debug build if release not present
  if [ ! -x "$DISPATCHER" ]; then
    DISPATCHER="${REPO_ROOT}/target/debug/factory-dispatcher"
  fi

  # WASM artifact path (release preferred, debug fallback)
  WASM_ARTIFACT="${REPO_ROOT}/target/wasm32-wasip1/release/regression-gate.wasm"
  if [ ! -f "$WASM_ARTIFACT" ]; then
    WASM_ARTIFACT="${REPO_ROOT}/target/wasm32-wasip1/debug/regression-gate.wasm"
  fi

  # Per dispatcher-runner pattern: project dir with .factory/logs subdirectory
  WORK="$BATS_TEST_TMPDIR/proj"
  mkdir -p "$WORK/.factory/logs"

  # Temporary directory for sink output
  SINK_DIR="$(mktemp -d)"
  SINK_FILE="${SINK_DIR}/events.jsonl"
}

teardown() {
  if [ -n "${SINK_DIR:-}" ] && [ -d "$SINK_DIR" ]; then
    rm -rf "$SINK_DIR"
  fi
}

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

_require_wasm_artifact() {
  if [ ! -f "$WASM_ARTIFACT" ]; then
    skip "regression-gate.wasm not found at $WASM_ARTIFACT — run: cargo build --target wasm32-wasip1 -p regression-gate"
  fi
}

_require_dispatcher() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "factory-dispatcher binary not found — run: cargo build --release -p factory-dispatcher"
  fi
}

# Invoke dispatcher with a JSON payload file. After call:
#   $status = dispatcher exit code, $output = dispatcher stderr.
_invoke() {
  local json_file="${WORK}/payload.json"
  printf '%s' "$1" > "$json_file"
  run env \
    CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" \
    CLAUDE_PROJECT_DIR="$WORK" \
    VSDD_SINK_FILE="$SINK_FILE" \
    bash -c "'$DISPATCHER' < '$json_file'"
}

# Invoke dispatcher with a project dir that has NO .factory/ directory.
_invoke_no_factory() {
  local work_no_factory="$BATS_TEST_TMPDIR/proj_no_factory"
  mkdir -p "$work_no_factory"
  local json_file="${work_no_factory}/payload.json"
  printf '%s' "$1" > "$json_file"
  run env \
    CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" \
    CLAUDE_PROJECT_DIR="$work_no_factory" \
    VSDD_SINK_FILE="$SINK_FILE" \
    bash -c "'$DISPATCHER' < '$json_file'"
}

# Build a PostToolUse JSON payload for Bash tool.
# $1 = command string
# $2 = tool_response JSON (pass "" to omit tool_response entirely)
_bash_payload() {
  local cmd="$1"
  local resp="$2"
  if [ -z "$resp" ]; then
    printf '{"event_name":"PostToolUse","session_id":"bats-s","dispatcher_trace_id":"bats-t","tool_name":"Bash","tool_input":{"command":"%s"}}' \
      "$(printf '%s' "$cmd" | sed 's/"/\\"/g')"
  else
    printf '{"event_name":"PostToolUse","session_id":"bats-s","dispatcher_trace_id":"bats-t","tool_name":"Bash","tool_input":{"command":"%s"},"tool_response":%s}' \
      "$(printf '%s' "$cmd" | sed 's/"/\\"/g')" "$resp"
  fi
}

# Assert the state file was written and contains expected status.
_assert_state_file_status() {
  local expected_status="$1"
  local state_file="$WORK/.factory/regression-state.json"
  if [ ! -f "$state_file" ]; then
    echo "State file not found: $state_file"
    return 1
  fi
  local actual
  actual="$(jq -r '.status' "$state_file" 2>/dev/null)"
  if [ "$actual" != "$expected_status" ]; then
    echo "Expected status='$expected_status' got '$actual' in $state_file"
    cat "$state_file"
    return 1
  fi
}

# Assert the state file does NOT exist.
_assert_no_state_file() {
  local state_file="$WORK/.factory/regression-state.json"
  if [ -f "$state_file" ]; then
    echo "State file should not exist: $state_file"
    cat "$state_file"
    return 1
  fi
}

# Assert the state file does NOT exist for a specific work dir.
_assert_no_state_file_in() {
  local dir="$1"
  local state_file="$dir/.factory/regression-state.json"
  if [ -f "$state_file" ]; then
    echo "State file should not exist: $state_file"
    cat "$state_file"
    return 1
  fi
}

# Assert the sink events file contains a hook.block event for regression-gate
# with the expected severity.
_assert_hook_block_warn_event() {
  if [ ! -f "$SINK_FILE" ]; then
    echo "Sink file not found: $SINK_FILE"
    return 1
  fi
  if ! jq -e 'select(.type == "hook.block") | select(.severity == "warn") | select(.hook == "regression-gate")' \
      "$SINK_FILE" > /dev/null 2>&1; then
    echo "Expected hook.block severity=warn hook=regression-gate not found in $SINK_FILE"
    cat "$SINK_FILE"
    return 1
  fi
}

# Assert the sink events file does NOT contain a hook.block event for regression-gate.
_assert_no_hook_block_warn() {
  if [ ! -f "$SINK_FILE" ]; then
    return 0  # no events = no block
  fi
  if jq -e 'select(.type == "hook.block") | select(.hook == "regression-gate")' \
      "$SINK_FILE" > /dev/null 2>&1; then
    echo "Unexpected hook.block event from regression-gate found in $SINK_FILE"
    cat "$SINK_FILE"
    return 1
  fi
}

# ---------------------------------------------------------------------------
# AC-001 / AC-002: registry + file deletion assertions
# ---------------------------------------------------------------------------

@test "AC-001: hooks-registry.toml regression-gate entry references native WASM (not legacy-bash-adapter)" {
  run grep -A 5 'name = "regression-gate"' "$REGISTRY"
  [ "$status" -eq 0 ]
  echo "$output" | grep -q 'plugin'
  ! echo "$output" | grep -q 'legacy-bash-adapter'
  echo "$output" | grep -q 'regression-gate.wasm'
}

@test "AC-001: hooks-registry.toml regression-gate has no script_path" {
  local stanza
  stanza="$(awk '/name = "regression-gate"/,/^\[\[hooks\]\]/' "$REGISTRY" | head -30)"
  ! echo "$stanza" | grep -q 'script_path'
}

@test "AC-001: hooks-registry.toml regression-gate has no exec_subprocess block" {
  local stanza
  stanza="$(awk '/name = "regression-gate"/,/^\[\[hooks\]\]/' "$REGISTRY" | head -30)"
  ! echo "$stanza" | grep -q 'exec_subprocess'
}

@test "AC-001: hooks-registry.toml regression-gate has no shell_bypass_acknowledged" {
  local stanza
  stanza="$(awk '/name = "regression-gate"/,/^\[\[hooks\]\]/' "$REGISTRY" | head -30)"
  ! echo "$stanza" | grep -q 'shell_bypass_acknowledged'
}

@test "AC-001: hooks-registry.toml regression-gate has read_file capability" {
  local stanza
  stanza="$(awk '/name = "regression-gate"/,/^\[\[hooks\]\]/' "$REGISTRY" | head -30)"
  echo "$stanza" | grep -q 'read_file'
}

@test "AC-001: hooks-registry.toml regression-gate has write_file capability" {
  local stanza
  stanza="$(awk '/name = "regression-gate"/,/^\[\[hooks\]\]/' "$REGISTRY" | head -30)"
  echo "$stanza" | grep -q 'write_file'
}

@test "AC-001 invariant: regression-gate WASM artifact exists at wasm32-wasip1 target" {
  [ -f "${REPO_ROOT}/target/wasm32-wasip1/release/regression-gate.wasm" ] \
    || [ -f "${REPO_ROOT}/target/wasm32-wasip1/debug/regression-gate.wasm" ]
}

@test "AC-002: plugins/vsdd-factory/hooks/regression-gate.sh is deleted" {
  [ ! -f "${REPO_ROOT}/plugins/vsdd-factory/hooks/regression-gate.sh" ]
}

@test "AC-002: hooks.json does not contain regression-gate.sh entry" {
  ! grep -r "regression-gate" "${REPO_ROOT}/plugins/vsdd-factory/hooks/" --include="hooks.json*" > /dev/null 2>&1
}

@test "AC-011: zero Tier 1 hooks reference legacy-bash-adapter in hooks-registry.toml" {
  local tier1_hooks=(
    "handoff-validator"
    "pr-manager-completion-guard"
    "track-agent-stop"
    "update-wave-state-on-merge"
    "validate-pr-review-posted"
    "session-learning"
    "warn-pending-wave-gate"
    "track-agent-start"
    "regression-gate"
  )
  for hook in "${tier1_hooks[@]}"; do
    local plugin_line
    plugin_line="$(grep -A3 "name = \"$hook\"" "$REGISTRY" | grep 'plugin =')"
    if echo "$plugin_line" | grep -q 'legacy-bash-adapter'; then
      echo "Tier 1 hook '$hook' still references legacy-bash-adapter: $plugin_line"
      return 1
    fi
  done
}

@test "AC-012: bin/emit-event is still present (D-10 deferral)" {
  test -f "${REPO_ROOT}/plugins/vsdd-factory/bin/emit-event"
}

# ---------------------------------------------------------------------------
# Scenario (a): cargo test pass, no prior state → state file written status=pass
# BC-7.03.073 postcondition 1 (exit_code=0 → pass)
# BC-7.03.074 postcondition 1 (state file written)
# BC-7.03.075 postcondition 1 (no warning — no prior state)
# ---------------------------------------------------------------------------

@test "regression_gate_cargo_test_pass_no_prior: exit_code=0 writes pass state, no warning" {
  _require_wasm_artifact
  _require_dispatcher

  local payload
  payload="$(_bash_payload "cargo test" '{"exit_code":0}')"
  _invoke "$payload"

  [ "$status" -eq 0 ]
  _assert_state_file_status "pass"
  _assert_no_hook_block_warn
}

# ---------------------------------------------------------------------------
# Scenario (b): cargo test pass via interrupted=false, no prior state
# BC-7.03.073 postcondition 1 (interrupted=false → pass)
# ---------------------------------------------------------------------------

@test "regression_gate_cargo_test_pass_interrupted_false: interrupted=false writes pass, no warning" {
  _require_wasm_artifact
  _require_dispatcher

  local payload
  payload="$(_bash_payload "cargo test" '{"interrupted":false}')"
  _invoke "$payload"

  [ "$status" -eq 0 ]
  _assert_state_file_status "pass"
  _assert_no_hook_block_warn
}

# ---------------------------------------------------------------------------
# Scenario (c): cargo test fail exit_code=1, prior state=pass → regression warning
# BC-7.03.075 postcondition 1 (pass-to-fail warning)
# ---------------------------------------------------------------------------

@test "regression_gate_cargo_test_fail_regression_warning: exit_code=1 + prior=pass emits hook.block warn" {
  _require_wasm_artifact
  _require_dispatcher

  # Write prior state as "pass"
  printf '{"status":"pass","timestamp":"2026-05-01T00:00:00Z","command":"cargo test"}' \
    > "$WORK/.factory/regression-state.json"

  local payload
  payload="$(_bash_payload "cargo test" '{"exit_code":1}')"
  _invoke "$payload"

  [ "$status" -eq 0 ]
  _assert_state_file_status "fail"
  _assert_hook_block_warn_event
}

# ---------------------------------------------------------------------------
# Scenario (d): cargo test interrupted=true, prior state=pass → regression warning
# BC-7.03.073 postcondition 1 (interrupted=true → fail)
# BC-7.03.075 postcondition 1 (pass-to-fail warning)
# ---------------------------------------------------------------------------

@test "regression_gate_cargo_test_interrupted_regression_warning: interrupted=true + prior=pass emits warn" {
  _require_wasm_artifact
  _require_dispatcher

  # Write prior state as "pass"
  printf '{"status":"pass","timestamp":"2026-05-01T00:00:00Z","command":"cargo test"}' \
    > "$WORK/.factory/regression-state.json"

  local payload
  payload="$(_bash_payload "cargo test" '{"interrupted":true}')"
  _invoke "$payload"

  [ "$status" -eq 0 ]
  _assert_state_file_status "fail"
  _assert_hook_block_warn_event
}

# ---------------------------------------------------------------------------
# Scenario (e): cargo test, no exit_code + no interrupted → unknown → no state write
# BC-7.03.073 postcondition 1 (step 5: unknown → skip)
# ---------------------------------------------------------------------------

@test "regression_gate_status_unknown_no_state_write: no exit_code + no interrupted → skip" {
  _require_wasm_artifact
  _require_dispatcher

  # tool_response has neither exit_code nor interrupted
  local payload
  payload="$(_bash_payload "cargo test" '{"stdout":"test output","noOutputExpected":false}')"
  _invoke "$payload"

  [ "$status" -eq 0 ]
  _assert_no_state_file
  _assert_no_hook_block_warn
}

# ---------------------------------------------------------------------------
# Scenario (f): non-test command → exit 0, no state write
# BC-7.03.072 postcondition 1 (9-pattern match; non-test exits 0)
# ---------------------------------------------------------------------------

@test "regression_gate_non_test_command_exits_0: git commit -m msg → exit 0, no state write" {
  _require_wasm_artifact
  _require_dispatcher

  local payload
  payload="$(_bash_payload "git commit -m 'chore: update'" '{"exit_code":0}')"
  _invoke "$payload"

  [ "$status" -eq 0 ]
  _assert_no_state_file
  _assert_no_hook_block_warn
}

# ---------------------------------------------------------------------------
# Scenario (g): .factory/ directory absent → exit 0, no state write
# BC-7.03.074 postcondition 1 (.factory/ guard)
#
# NOTE: The dispatcher always creates $CLAUDE_PROJECT_DIR/.factory/logs/
# before invoking plugins (dispatcher internal log setup). Therefore,
# it is not possible to test the .factory/-absent guard via the dispatcher
# integration path — the guard will never trigger because .factory/ is
# always present by the time the hook runs.
#
# This AC-007 scenario (g) is verified at the UNIT TEST level:
#   test_BC_7_03_074_factory_dir_absent_exits_0_no_state_write in lib.rs
# passes `factory_dir_exists = false` directly to regression_gate_logic.
#
# The integration test below verifies the complementary property: hook runs
# correctly in a project dir where .factory/ exists but has no prior state.
# ---------------------------------------------------------------------------

@test "regression_gate_factory_dir_absent_exits_0: dispatcher always creates .factory/; hook runs cleanly on first invocation" {
  _require_wasm_artifact
  _require_dispatcher

  # The dispatcher creates .factory/logs/ before running hooks.
  # Verify that a fresh project dir (no pre-existing state file) still works:
  # the hook should write the state file (pass state) and exit 0.
  local payload
  payload="$(_bash_payload "cargo test" '{"exit_code":0}')"

  # Use a fresh work dir with no state file (only the standard .factory/logs setup)
  local work_fresh="$BATS_TEST_TMPDIR/proj_fresh"
  mkdir -p "$work_fresh/.factory/logs"
  local json_file="${work_fresh}/payload.json"
  printf '%s' "$payload" > "$json_file"

  run env \
    CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" \
    CLAUDE_PROJECT_DIR="$work_fresh" \
    VSDD_SINK_FILE="$SINK_FILE" \
    bash -c "'$DISPATCHER' < '$json_file'"

  [ "$status" -eq 0 ]
  # State file is created on first run (no prior state = no regression warning)
  [ -f "$work_fresh/.factory/regression-state.json" ]
  local status_val
  status_val="$(jq -r '.status' "$work_fresh/.factory/regression-state.json" 2>/dev/null)"
  [ "$status_val" = "pass" ]
}

# ---------------------------------------------------------------------------
# Scenario (h): fail-to-fail → state written but no regression warning
# BC-7.03.075 postcondition 1 (warning ONLY on pass-to-fail, not fail-to-fail)
# ---------------------------------------------------------------------------

@test "regression_gate_fail_to_fail_no_warning: exit_code=1 + prior=fail → state updated, no warn" {
  _require_wasm_artifact
  _require_dispatcher

  # Write prior state as "fail"
  printf '{"status":"fail","timestamp":"2026-05-01T00:00:00Z","command":"cargo test"}' \
    > "$WORK/.factory/regression-state.json"

  local payload
  payload="$(_bash_payload "cargo test" '{"exit_code":1}')"
  _invoke "$payload"

  [ "$status" -eq 0 ]
  _assert_state_file_status "fail"
  _assert_no_hook_block_warn
}

# ---------------------------------------------------------------------------
# Scenario (i): pytest pass → verifies non-cargo pattern coverage
# BC-7.03.072 postcondition 1 (pytest pattern matched)
# ---------------------------------------------------------------------------

@test "regression_gate_pytest_pass: pytest tests/ pass → state written status=pass, no warning" {
  _require_wasm_artifact
  _require_dispatcher

  local payload
  payload="$(_bash_payload "pytest tests/" '{"exit_code":0}')"
  _invoke "$payload"

  [ "$status" -eq 0 ]
  _assert_state_file_status "pass"

  # Verify command is stored in state file
  local stored_cmd
  stored_cmd="$(jq -r '.command' "$WORK/.factory/regression-state.json" 2>/dev/null)"
  [ "$stored_cmd" = "pytest tests/" ]

  _assert_no_hook_block_warn
}

# ---------------------------------------------------------------------------
# Additional: non-Bash tool → exit 0, no state write (EC-001)
# BC-7.03.072 postcondition 1 (Bash guard)
# ---------------------------------------------------------------------------

@test "AC-003: non-Bash tool (Read) → exit 0, no state write" {
  _require_wasm_artifact
  _require_dispatcher

  # Construct a PostToolUse for a non-Bash tool
  local json
  json='{"event_name":"PostToolUse","session_id":"bats-s","dispatcher_trace_id":"bats-t","tool_name":"Read","tool_input":{"file_path":"src/lib.rs"},"tool_response":{"content":"code"}}'
  local json_file="${WORK}/payload.json"
  printf '%s' "$json" > "$json_file"
  run env \
    CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" \
    CLAUDE_PROJECT_DIR="$WORK" \
    VSDD_SINK_FILE="$SINK_FILE" \
    bash -c "'$DISPATCHER' < '$json_file'"

  [ "$status" -eq 0 ]
  _assert_no_state_file
  _assert_no_hook_block_warn
}

# ---------------------------------------------------------------------------
# State file format: timestamp is ISO-8601 UTC (AC-005)
# BC-7.03.074 postcondition 1
# ---------------------------------------------------------------------------

@test "AC-005: state file timestamp is ISO-8601 UTC format" {
  _require_wasm_artifact
  _require_dispatcher

  local payload
  payload="$(_bash_payload "cargo test" '{"exit_code":0}')"
  _invoke "$payload"

  [ "$status" -eq 0 ]
  [ -f "$WORK/.factory/regression-state.json" ]

  local ts
  ts="$(jq -r '.timestamp' "$WORK/.factory/regression-state.json" 2>/dev/null)"
  # Must match YYYY-MM-DDTHH:MM:SSZ pattern
  echo "$ts" | grep -qE '^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$'
}
