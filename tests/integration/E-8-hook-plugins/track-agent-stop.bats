#!/usr/bin/env bats
# track-agent-stop.bats — AC-004 + AC-007 parity tests for track-agent-stop native WASM port
#
# Invokes via factory-dispatcher (not bare wasmtime) following the pattern from
# tests/integration/E-8-hook-plugins/track-agent-start.bats.
#
# What this suite covers:
#   AC-004(a): empty last_assistant_message → EXIT_CLASS=empty, result_len=0, exit 0
#   AC-004(b): result with "Status: BLOCKED" → EXIT_CLASS=blocked, exit 0
#   AC-004(c): non-empty non-BLOCKED result → EXIT_CLASS=ok, result_len=N, exit 0
#   AC-004(d): missing last_assistant_message, result present → fallback to result (BC-2.02.012 PC-6)
#   AC-004(e): both last_assistant_message and result absent → EXIT_CLASS=empty, result_len=0
#   AC-004(f): agent_type absent but subagent_name present → subagent=subagent_name (BC-2.02.012 PC-5)
#   AC-007:    malformed JSON stdin → exit 0, no panic
#   EC-006:    multibyte UTF-8 (emoji) → result_len=byte count (not char count)
#   EC-007:    BLOCKED on non-first line (multiline (?m)) → EXIT_CLASS=blocked
#   registry:  hooks-registry.toml entry references native WASM (AC-001)
#   file:      track-agent-stop.sh is deleted (AC-002)
#
# Note on SINK_FILE: The dispatcher writes ALL plugin-domain events to the sink,
# not just those from track-agent-stop (e.g., handoff-validator may also emit
# hook.block events). Tests extract agent.stop events using:
#   jq -r 'select(.type == "agent.stop") | .FIELD' "$SINK_FILE"
#
# Story: S-8.03
# BCs: BC-7.03.081, BC-7.03.082, BC-2.02.012
# See: .factory/stories/S-8.03-native-port-track-agent-stop.md

# ---------------------------------------------------------------------------
# Setup: locate dispatcher binary and configure a file-sink event sink
# ---------------------------------------------------------------------------

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  DISPATCHER="${REPO_ROOT}/target/release/factory-dispatcher"
  PLUGIN_ROOT="${REPO_ROOT}/plugins/vsdd-factory"

  # Fall back to debug build if release not present
  if [ ! -x "$DISPATCHER" ]; then
    DISPATCHER="${REPO_ROOT}/target/debug/factory-dispatcher"
  fi

  # WASM artifact path (release preferred, debug fallback)
  WASM_ARTIFACT="${REPO_ROOT}/target/wasm32-wasip1/release/track-agent-stop.wasm"
  if [ ! -f "$WASM_ARTIFACT" ]; then
    WASM_ARTIFACT="${REPO_ROOT}/target/wasm32-wasip1/debug/track-agent-stop.wasm"
  fi

  # Per regression-v1.0.bats pattern: project dir with .factory/logs subdirectory
  WORK="$BATS_TEST_TMPDIR/proj"
  mkdir -p "$WORK/.factory/logs"

  # Temporary directory for sink output
  SINK_DIR="$(mktemp -d)"
  SINK_FILE="${SINK_DIR}/events.jsonl"

  REGISTRY="${PLUGIN_ROOT}/hooks-registry.toml"
}

teardown() {
  if [ -n "${SINK_DIR:-}" ] && [ -d "$SINK_DIR" ]; then
    rm -rf "$SINK_DIR"
  fi
}

# ---------------------------------------------------------------------------
# Helper: check that the native WASM artifact and dispatcher exist
# ---------------------------------------------------------------------------

_require_wasm_artifact() {
  if [ ! -f "$WASM_ARTIFACT" ]; then
    skip "track-agent-stop.wasm not found at $WASM_ARTIFACT — run: cargo build --target wasm32-wasip1 -p track-agent-stop"
  fi
}

_require_dispatcher() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "factory-dispatcher binary not found — run: cargo build --release -p factory-dispatcher"
  fi
}

# Helper: invoke dispatcher and write payload JSON to a temp file.
# After call: dispatcher exit code in $status, stderr in $output.
_invoke() {
  local json_file="${WORK}/payload.json"
  printf '%s' "$1" > "$json_file"
  run env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" VSDD_SINK_FILE="$SINK_FILE" \
    bash -c "'$DISPATCHER' < '$json_file'"
}

# Helper: extract a field from the agent.stop event in the sink file.
# Uses `jq` with select(.type == "agent.stop") to skip other plugin events.
_agent_stop_field() {
  local field="$1"
  jq -r "select(.type == \"agent.stop\") | .${field}" "$SINK_FILE" 2>/dev/null | head -1
}

# ---------------------------------------------------------------------------
# Pre-condition: registry + file assertions (AC-001, AC-002)
# ---------------------------------------------------------------------------

@test "AC-001: hooks-registry.toml track-agent-stop entry references native WASM (not legacy-bash-adapter)" {
  run grep -A 5 'name = "track-agent-stop"' "$REGISTRY"
  [ "$status" -eq 0 ]
  echo "$output" | grep -q 'plugin'
  ! echo "$output" | grep -q 'legacy-bash-adapter'
  echo "$output" | grep -q 'track-agent-stop.wasm'
}

@test "AC-001: hooks-registry.toml track-agent-stop has no script_path (legacy-bash-adapter artifact removed)" {
  local stanza
  stanza="$(awk '/name = "track-agent-stop"/,/^\[\[hooks\]\]/' "$REGISTRY" | head -30)"
  ! echo "$stanza" | grep -q 'script_path'
}

@test "AC-001: hooks-registry.toml track-agent-stop has no exec_subprocess block" {
  local stanza
  stanza="$(awk '/name = "track-agent-stop"/,/^\[\[hooks\]\]/' "$REGISTRY" | head -30)"
  ! echo "$stanza" | grep -q 'exec_subprocess'
}

@test "AC-001: hooks-registry.toml track-agent-stop has no [hooks.capabilities] section" {
  local stanza
  stanza="$(awk '/name = "track-agent-stop"/,/^\[\[hooks\]\]/' "$REGISTRY" | head -30)"
  ! echo "$stanza" | grep -q '\[hooks.capabilities\]'
}

@test "AC-001 invariant: track-agent-stop WASM artifact exists at wasm32-wasip1 target" {
  [ -f "${REPO_ROOT}/target/wasm32-wasip1/release/track-agent-stop.wasm" ] \
    || [ -f "${REPO_ROOT}/target/wasm32-wasip1/debug/track-agent-stop.wasm" ]
}

@test "AC-002: plugins/vsdd-factory/hooks/track-agent-stop.sh is deleted" {
  [ ! -f "${REPO_ROOT}/plugins/vsdd-factory/hooks/track-agent-stop.sh" ]
}

# ---------------------------------------------------------------------------
# AC-004(a): empty last_assistant_message → EXIT_CLASS=empty, result_len=0
# ---------------------------------------------------------------------------

@test "AC-004(a): empty last_assistant_message => agent.stop with exit_class=empty result_len=0 exit 0" {
  _require_wasm_artifact
  _require_dispatcher

  _invoke '{"event_name":"SubagentStop","session_id":"bats-test-session","dispatcher_trace_id":"bats-trace-001","agent_type":"implementer","last_assistant_message":""}'
  [ "$status" -eq 0 ]

  [ -f "$SINK_FILE" ]

  # Verify agent.stop event was written (at least one line with type=agent.stop)
  run grep -c '"agent.stop"' "$SINK_FILE"
  [ "$output" -ge 1 ]

  val="$(_agent_stop_field exit_class)"
  [ "$val" = "empty" ]

  val="$(_agent_stop_field result_len)"
  [ "$val" = "0" ]

  val="$(_agent_stop_field subagent)"
  [ "$val" = "implementer" ]
}

# ---------------------------------------------------------------------------
# AC-004(b): result with "Status: BLOCKED" → EXIT_CLASS=blocked
# ---------------------------------------------------------------------------

@test "AC-004(b): Status: BLOCKED in last_assistant_message => exit_class=blocked exit 0" {
  _require_wasm_artifact
  _require_dispatcher

  _invoke '{"event_name":"SubagentStop","session_id":"bats-test-session","dispatcher_trace_id":"bats-trace-002","agent_type":"story-writer","last_assistant_message":"Status: BLOCKED — missing context"}'
  [ "$status" -eq 0 ]

  [ -f "$SINK_FILE" ]

  val="$(_agent_stop_field exit_class)"
  [ "$val" = "blocked" ]

  val="$(_agent_stop_field subagent)"
  [ "$val" = "story-writer" ]
}

# ---------------------------------------------------------------------------
# AC-004(c): non-empty non-BLOCKED result → EXIT_CLASS=ok, result_len=N
# ---------------------------------------------------------------------------

@test "AC-004(c): non-empty non-BLOCKED last_assistant_message => exit_class=ok result_len=4 exit 0" {
  _require_wasm_artifact
  _require_dispatcher

  # "DONE" = 4 non-whitespace bytes
  _invoke '{"event_name":"SubagentStop","session_id":"bats-test-session","dispatcher_trace_id":"bats-trace-003","agent_type":"pr-reviewer","last_assistant_message":"DONE"}'
  [ "$status" -eq 0 ]

  [ -f "$SINK_FILE" ]

  val="$(_agent_stop_field exit_class)"
  [ "$val" = "ok" ]

  val="$(_agent_stop_field result_len)"
  [ "$val" = "4" ]

  val="$(_agent_stop_field subagent)"
  [ "$val" = "pr-reviewer" ]
}

# ---------------------------------------------------------------------------
# AC-004(d): missing last_assistant_message, result present → fallback (BC-2.02.012 PC-6)
# ---------------------------------------------------------------------------

@test "AC-004(d): last_assistant_message absent, result present => fallback to result field exit 0" {
  _require_wasm_artifact
  _require_dispatcher

  # No last_assistant_message field; result="DONE via result" = 13 non-ws bytes
  _invoke '{"event_name":"SubagentStop","session_id":"bats-test-session","dispatcher_trace_id":"bats-trace-004","agent_type":"implementer","result":"DONE via result"}'
  [ "$status" -eq 0 ]

  [ -f "$SINK_FILE" ]

  val="$(_agent_stop_field exit_class)"
  [ "$val" = "ok" ]

  # "DONEviaresult" = 13 non-whitespace bytes
  val="$(_agent_stop_field result_len)"
  [ "$val" = "13" ]
}

# ---------------------------------------------------------------------------
# AC-004(e): both last_assistant_message and result absent → EXIT_CLASS=empty
# ---------------------------------------------------------------------------

@test "AC-004(e): both last_assistant_message and result absent => exit_class=empty result_len=0 exit 0" {
  _require_wasm_artifact
  _require_dispatcher

  # Neither field present
  _invoke '{"event_name":"SubagentStop","session_id":"bats-test-session","dispatcher_trace_id":"bats-trace-005","agent_type":"reviewer"}'
  [ "$status" -eq 0 ]

  [ -f "$SINK_FILE" ]

  val="$(_agent_stop_field exit_class)"
  [ "$val" = "empty" ]

  val="$(_agent_stop_field result_len)"
  [ "$val" = "0" ]
}

# ---------------------------------------------------------------------------
# AC-004(f): agent_type absent but subagent_name present → BC-2.02.012 PC-5 fallback
# ---------------------------------------------------------------------------

@test "AC-004(f): agent_type absent, subagent_name present => subagent=subagent_name (BC-2.02.012 PC-5)" {
  _require_wasm_artifact
  _require_dispatcher

  # No agent_type; subagent_name = "state-manager"
  _invoke '{"event_name":"SubagentStop","session_id":"bats-test-session","dispatcher_trace_id":"bats-trace-006","subagent_name":"state-manager","last_assistant_message":"DONE"}'
  [ "$status" -eq 0 ]

  [ -f "$SINK_FILE" ]

  val="$(_agent_stop_field subagent)"
  [ "$val" = "state-manager" ]

  val="$(_agent_stop_field exit_class)"
  [ "$val" = "ok" ]
}

# ---------------------------------------------------------------------------
# AC-007: malformed JSON stdin → exit 0, no panic
# ---------------------------------------------------------------------------

@test "AC-007: malformed JSON stdin => exit 0 and no panic (best-effort)" {
  _require_wasm_artifact
  _require_dispatcher

  run env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" VSDD_SINK_FILE="$SINK_FILE" \
    bash -c "printf '%s' 'not valid json at all {{{}}}' | '$DISPATCHER'"

  # Must exit 0 (best-effort — BC-7.03.081 invariant 1)
  [ "$status" -eq 0 ]

  # No stderr panic/backtrace
  ! echo "$output" | grep -qi "panic"
  ! echo "$output" | grep -qi "backtrace"

  # No agent.stop event in sink (non-SubagentStop envelopes → track-agent-stop not triggered)
  if [ -f "$SINK_FILE" ]; then
    run grep -c '"agent.stop"' "$SINK_FILE"
    [ "$output" = "0" ]
  fi
}

# ---------------------------------------------------------------------------
# EC-006: multibyte UTF-8 (emoji) → result_len=byte count (not char count)
# ---------------------------------------------------------------------------

@test "EC-006: U+1F600 emoji in result => result_len=4 (byte count parity with wc -c)" {
  _require_wasm_artifact
  _require_dispatcher

  # U+1F600 = 4 bytes in UTF-8; result_len must be 4 (byte count, not char count)
  # Use Python to write valid JSON with emoji (avoids shell UTF-8 escaping issues)
  python3 -c 'import json,sys; sys.stdout.write(json.dumps({"event_name":"SubagentStop","session_id":"bats-test-session","dispatcher_trace_id":"bats-trace-007","agent_type":"implementer","last_assistant_message":"\U0001F600"}))' \
    > "${WORK}/payload.json"

  run env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" VSDD_SINK_FILE="$SINK_FILE" \
    bash -c "'$DISPATCHER' < '${WORK}/payload.json'"
  [ "$status" -eq 0 ]

  [ -f "$SINK_FILE" ]

  val="$(_agent_stop_field result_len)"
  [ "$val" = "4" ]

  val="$(_agent_stop_field exit_class)"
  [ "$val" = "ok" ]
}

# ---------------------------------------------------------------------------
# EC-007: BLOCKED on non-first line (multiline (?m) regex) → EXIT_CLASS=blocked
# ---------------------------------------------------------------------------

@test "EC-007: BLOCKED on non-first line => exit_class=blocked (multiline regex parity)" {
  _require_wasm_artifact
  _require_dispatcher

  # BLOCKED appears on second line; (?m) flag ensures ^ matches start-of-line
  python3 -c 'import json,sys; sys.stdout.write(json.dumps({"event_name":"SubagentStop","session_id":"bats-test-session","dispatcher_trace_id":"bats-trace-008","agent_type":"pr-manager","last_assistant_message":"first line of output\nBLOCKED\nsome more"}))' \
    > "${WORK}/payload.json"

  run env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" VSDD_SINK_FILE="$SINK_FILE" \
    bash -c "'$DISPATCHER' < '${WORK}/payload.json'"
  [ "$status" -eq 0 ]

  [ -f "$SINK_FILE" ]

  val="$(_agent_stop_field exit_class)"
  [ "$val" = "blocked" ]
}

# ---------------------------------------------------------------------------
# Parity audit: exact field set check
# ---------------------------------------------------------------------------

@test "parity audit: agent.stop event contains hook=track-agent-stop, matcher=SubagentStop, correct subagent/exit_class/result_len" {
  _require_wasm_artifact
  _require_dispatcher

  _invoke '{"event_name":"SubagentStop","session_id":"bats-parity-session","dispatcher_trace_id":"bats-parity-trace","agent_type":"pr-manager","last_assistant_message":"DONE"}'
  [ "$status" -eq 0 ]
  [ -f "$SINK_FILE" ]

  val="$(_agent_stop_field hook)"
  [ "$val" = "track-agent-stop" ]

  val="$(_agent_stop_field matcher)"
  [ "$val" = "SubagentStop" ]

  val="$(_agent_stop_field subagent)"
  [ "$val" = "pr-manager" ]

  val="$(_agent_stop_field exit_class)"
  [ "$val" = "ok" ]

  # result_len = "DONE" = 4 non-ws bytes
  val="$(_agent_stop_field result_len)"
  [ "$val" = "4" ]

  # Forbidden reserved fields must not be set by the plugin
  run jq 'select(.type == "agent.stop") | has("agent_id")' "$SINK_FILE"
  [ "$output" = "false" ]
}

# ---------------------------------------------------------------------------
# EC-004b: both agent_type and subagent_name absent → subagent="unknown"
# ---------------------------------------------------------------------------

@test "EC-004b: both agent_type and subagent_name absent => subagent=unknown" {
  _require_wasm_artifact
  _require_dispatcher

  _invoke '{"event_name":"SubagentStop","session_id":"bats-ec004b-session","dispatcher_trace_id":"bats-ec004b-trace","last_assistant_message":"DONE"}'
  [ "$status" -eq 0 ]

  [ -f "$SINK_FILE" ]

  val="$(_agent_stop_field subagent)"
  [ "$val" = "unknown" ]
}
