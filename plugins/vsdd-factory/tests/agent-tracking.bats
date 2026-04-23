#!/usr/bin/env bats
# agent-tracking.bats — tests for track-agent-start.sh + track-agent-stop.sh,
# the PreToolUse Agent + SubagentStop telemetry hooks introduced in v0.68.

setup() {
  HOOKS_DIR="${BATS_TEST_DIRNAME}/../hooks"
  PLUGIN_ROOT="${BATS_TEST_DIRNAME}/.."
  EMIT_TMPDIR="$(mktemp -d)"
  export VSDD_LOG_DIR="$EMIT_TMPDIR"
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  unset VSDD_TELEMETRY
}

teardown() {
  rm -rf "$EMIT_TMPDIR"
  unset VSDD_LOG_DIR CLAUDE_PLUGIN_ROOT VSDD_TELEMETRY
}

_logfile() {
  ls "$EMIT_TMPDIR"/events-*.jsonl 2>/dev/null | head -1
}

# ---------- track-agent-start.sh ----------

@test "track-agent-start: exists and executable" {
  [ -x "$HOOKS_DIR/track-agent-start.sh" ]
}

@test "track-agent-start: passes syntax check" {
  bash -n "$HOOKS_DIR/track-agent-start.sh"
}

@test "track-agent-start: emits agent.start event on Agent dispatch" {
  local input
  input=$(jq -nc '{
    tool_name: "Agent",
    tool_input: {subagent_type: "pr-manager", prompt: "dispatching for S-1.01"}
  }')
  run bash -c "echo '$input' | '$HOOKS_DIR/track-agent-start.sh'"
  [ "$status" -eq 0 ]
  local lf
  lf=$(_logfile)
  [ -n "$lf" ]
  [ "$(jq -r '.type' < "$lf")" = "agent.start" ]
  [ "$(jq -r '.subagent' < "$lf")" = "pr-manager" ]
  [ "$(jq -r '.story_id' < "$lf")" = "S-1.01" ]
  [ "$(jq -r '.matcher' < "$lf")" = "Agent" ]
}

@test "track-agent-start: no story_id when prompt lacks one" {
  local input
  input=$(jq -nc '{
    tool_name: "Agent",
    tool_input: {subagent_type: "test-writer", prompt: "write some tests"}
  }')
  run bash -c "echo '$input' | '$HOOKS_DIR/track-agent-start.sh'"
  [ "$status" -eq 0 ]
  local lf
  lf=$(_logfile)
  # story_id should be absent (jq returns null)
  [ "$(jq -r '.story_id // "ABSENT"' < "$lf")" = "ABSENT" ]
}

@test "track-agent-start: non-Agent tool is a no-op" {
  local input
  input=$(jq -nc '{tool_name: "Bash", tool_input: {command: "ls"}}')
  run bash -c "echo '$input' | '$HOOKS_DIR/track-agent-start.sh'"
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

@test "track-agent-start: exits 0 even when CLAUDE_PLUGIN_ROOT is broken" {
  local input
  input=$(jq -nc '{tool_name: "Agent", tool_input: {subagent_type: "x"}}')
  run bash -c "CLAUDE_PLUGIN_ROOT=/nowhere echo '$input' | '$HOOKS_DIR/track-agent-start.sh'"
  [ "$status" -eq 0 ]
}

# ---------- track-agent-stop.sh ----------

@test "track-agent-stop: exists and executable" {
  [ -x "$HOOKS_DIR/track-agent-stop.sh" ]
}

@test "track-agent-stop: passes syntax check" {
  bash -n "$HOOKS_DIR/track-agent-stop.sh"
}

@test "track-agent-stop: emits agent.stop event" {
  local input
  input=$(jq -nc '{
    agent_type: "pr-manager",
    last_assistant_message: "STEP_COMPLETE for every step. Ran clean, no issues."
  }')
  run bash -c "echo '$input' | '$HOOKS_DIR/track-agent-stop.sh'"
  [ "$status" -eq 0 ]
  local lf
  lf=$(_logfile)
  [ -n "$lf" ]
  [ "$(jq -r '.type' < "$lf")" = "agent.stop" ]
  [ "$(jq -r '.subagent' < "$lf")" = "pr-manager" ]
  [ "$(jq -r '.exit_class' < "$lf")" = "ok" ]
  # result_len should be > 0
  local rlen
  rlen=$(jq -r '.result_len' < "$lf")
  [ "$rlen" -gt 0 ]
}

@test "track-agent-stop: empty result marks exit_class=empty" {
  local input
  input=$(jq -nc '{agent_type: "pr-manager", last_assistant_message: ""}')
  run bash -c "echo '$input' | '$HOOKS_DIR/track-agent-stop.sh'"
  [ "$status" -eq 0 ]
  local lf
  lf=$(_logfile)
  [ "$(jq -r '.exit_class' < "$lf")" = "empty" ]
}

@test "track-agent-stop: BLOCKED status marks exit_class=blocked" {
  local input
  input=$(jq -nc '{agent_type: "pr-manager", last_assistant_message: "BLOCKED: dependency PR not merged"}')
  run bash -c "echo '$input' | '$HOOKS_DIR/track-agent-stop.sh'"
  [ "$status" -eq 0 ]
  local lf
  lf=$(_logfile)
  [ "$(jq -r '.exit_class' < "$lf")" = "blocked" ]
}

@test "track-agent-stop: exits 0 even when CLAUDE_PLUGIN_ROOT is broken" {
  local input
  input=$(jq -nc '{agent_type: "pr-manager", last_assistant_message: "ok"}')
  run bash -c "CLAUDE_PLUGIN_ROOT=/nowhere echo '$input' | '$HOOKS_DIR/track-agent-stop.sh'"
  [ "$status" -eq 0 ]
}

# ---------- hooks.json wiring ----------

@test "hooks.json: track-agent-start wired under PreToolUse Agent matcher" {
  jq -e '
    .hooks.PreToolUse
    | map(select(.matcher == "Agent"))[0].hooks
    | map(.command)
    | any(endswith("/track-agent-start.sh"))
  ' "$HOOKS_DIR/hooks.json"
}

@test "hooks.json: track-agent-stop wired under SubagentStop" {
  jq -e '
    .hooks.SubagentStop[0].hooks
    | map(.command)
    | any(endswith("/track-agent-stop.sh"))
  ' "$HOOKS_DIR/hooks.json"
}
