#!/usr/bin/env bats
# stop-hooks-emission.bats — emission tests for the 5 SubagentStop + Stop hooks
# instrumented in observability phase 2e (v0.63.0).
#
# Introduces the hook.action event type (update-wave-state-on-merge) for
# passive state-change signals, alongside hook.block with severity=warn for
# advisory-only hooks.

setup() {
  HOOKS_DIR="${BATS_TEST_DIRNAME}/../hooks"
  PLUGIN_ROOT="${BATS_TEST_DIRNAME}/.."
  EMIT_TMPDIR="$(mktemp -d)"
  SCRATCH="$(mktemp -d)"
  export VSDD_LOG_DIR="$EMIT_TMPDIR"
  unset VSDD_TELEMETRY
}

teardown() {
  rm -rf "$EMIT_TMPDIR" "$SCRATCH"
  unset VSDD_LOG_DIR CLAUDE_PLUGIN_ROOT VSDD_TELEMETRY
}

_logfile() {
  ls "$EMIT_TMPDIR"/events-*.jsonl 2>/dev/null | head -1
}

_subagent_input() {
  # $1 = agent_type, $2 = last_assistant_message
  jq -nc --arg a "$1" --arg m "$2" '{agent_type: $a, last_assistant_message: $m}'
}

# ---------- handoff-validator.sh (SubagentStop, severity=warn) ----------

@test "handoff-validator: emits subagent_empty_result on empty output" {
  local input
  input=$(_subagent_input "test-writer" "")
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$input' | '$HOOKS_DIR/handoff-validator.sh' 2>&1"
  [ "$status" -eq 0 ]
  local lf
  lf=$(_logfile)
  [ -n "$lf" ]
  [ "$(jq -r '.reason' < "$lf")" = "subagent_empty_result" ]
  [ "$(jq -r '.severity' < "$lf")" = "warn" ]
  [ "$(jq -r '.subagent' < "$lf")" = "test-writer" ]
}

@test "handoff-validator: emits subagent_truncated_result on short output" {
  local input
  input=$(_subagent_input "test-writer" "ok")
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$input' | '$HOOKS_DIR/handoff-validator.sh' 2>&1"
  [ "$status" -eq 0 ]
  local lf
  lf=$(_logfile)
  [ -n "$lf" ]
  [ "$(jq -r '.reason' < "$lf")" = "subagent_truncated_result" ]
  [ "$(jq -r '.severity' < "$lf")" = "warn" ]
}

@test "handoff-validator: full output emits no event" {
  local input
  input=$(_subagent_input "test-writer" "This is a sufficiently long response from the subagent that exceeds the 40-character threshold.")
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$input' | '$HOOKS_DIR/handoff-validator.sh' 2>&1"
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

# ---------- pr-manager-completion-guard.sh (SubagentStop) ----------

@test "pr-manager-completion-guard: emits pr_manager_incomplete_lifecycle" {
  # Only 3 STEP_COMPLETE emissions (below the 8 threshold)
  local msg="STEP_COMPLETE: step=1 name=description status=ok
STEP_COMPLETE: step=2 name=demo status=na
STEP_COMPLETE: step=3 name=create-pr status=ok"
  local input
  input=$(_subagent_input "pr-manager" "$msg")
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$input' | '$HOOKS_DIR/pr-manager-completion-guard.sh' 2>&1"
  [ "$status" -eq 2 ]
  local lf
  lf=$(_logfile)
  [ -n "$lf" ]
  [ "$(jq -r '.reason' < "$lf")" = "pr_manager_incomplete_lifecycle" ]
  [ "$(jq -r '.step_count' < "$lf")" = "3" ]
  [ "$(jq -r '.last_step' < "$lf")" = "3" ]
  [ "$(jq -r '.next_step' < "$lf")" = "4" ]
}

@test "pr-manager-completion-guard: 8+ steps emits no event" {
  local msg=""
  for i in 1 2 3 4 5 6 7 8; do
    msg+="STEP_COMPLETE: step=$i name=x status=ok"$'\n'
  done
  local input
  input=$(_subagent_input "pr-manager" "$msg")
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$input' | '$HOOKS_DIR/pr-manager-completion-guard.sh' 2>&1"
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

@test "pr-manager-completion-guard: still blocks when CLAUDE_PLUGIN_ROOT unset" {
  local msg="STEP_COMPLETE: step=1 name=x status=ok"
  local input
  input=$(_subagent_input "pr-manager" "$msg")
  run bash -c "unset CLAUDE_PLUGIN_ROOT; echo '$input' | '$HOOKS_DIR/pr-manager-completion-guard.sh' 2>&1"
  [ "$status" -eq 2 ]
}

# ---------- validate-pr-review-posted.sh (SubagentStop) ----------

@test "validate-pr-review-posted: emits pr_review_not_posted when comment used" {
  local msg="Ran gh pr comment 42 --body 'Findings: ...'"
  local input
  input=$(_subagent_input "pr-reviewer" "$msg")
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$input' | '$HOOKS_DIR/validate-pr-review-posted.sh' 2>&1"
  [ "$status" -eq 2 ]
  local lf
  lf=$(_logfile)
  [ -n "$lf" ]
  [ "$(jq -r '.reason' < "$lf")" = "pr_review_not_posted" ]
}

@test "validate-pr-review-posted: proper review emits no event" {
  local msg="wrote pr-review.md to .factory/code-delivery/STORY-042/pr-review.md. Ran gh pr review --approve --body-file pr-review.md"
  local input
  input=$(_subagent_input "pr-reviewer" "$msg")
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$input' | '$HOOKS_DIR/validate-pr-review-posted.sh' 2>&1"
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

# ---------- update-wave-state-on-merge.sh (hook.action type) ----------

@test "update-wave-state-on-merge: emits hook.action with reason=wave_merge_recorded" {
  if ! command -v python3 >/dev/null 2>&1; then
    skip "python3 unavailable"
  fi
  mkdir -p "$SCRATCH/.factory"
  cat > "$SCRATCH/.factory/wave-state.yaml" <<EOF
waves:
  wave-1:
    stories: [S-1.01, S-1.02]
    stories_merged: [S-1.01]
    gate_status: not_started
EOF
  local msg="STEP_COMPLETE: step=8 name=merge status=ok. merged S-1.02"
  local input
  input=$(_subagent_input "pr-manager" "$msg")
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "cd '$SCRATCH' && echo '$input' | '$HOOKS_DIR/update-wave-state-on-merge.sh' 2>&1"
  [ "$status" -eq 0 ]
  local lf
  lf=$(_logfile)
  [ -n "$lf" ]
  [ "$(jq -r '.type' < "$lf")" = "hook.action" ]
  [ "$(jq -r '.reason' < "$lf")" = "wave_merge_recorded" ]
  [ "$(jq -r '.story_id' < "$lf")" = "S-1.02" ]
  [ "$(jq -r '.wave' < "$lf")" = "wave-1" ]
  [ "$(jq -r '.gate_transitioned' < "$lf")" = "True" ]
}

# ---------- warn-pending-wave-gate.sh (Stop, severity=warn) ----------

@test "warn-pending-wave-gate: emits pending_wave_gate_at_session_end" {
  if ! command -v python3 >/dev/null 2>&1; then
    skip "python3 unavailable"
  fi
  mkdir -p "$SCRATCH/.factory"
  cat > "$SCRATCH/.factory/wave-state.yaml" <<EOF
waves:
  wave-1:
    gate_status: pending
  wave-2:
    gate_status: not_started
EOF
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "cd '$SCRATCH' && echo '{}' | '$HOOKS_DIR/warn-pending-wave-gate.sh' 2>&1"
  [ "$status" -eq 0 ]
  local lf
  lf=$(_logfile)
  [ -n "$lf" ]
  [ "$(jq -r '.type' < "$lf")" = "hook.block" ]
  [ "$(jq -r '.reason' < "$lf")" = "pending_wave_gate_at_session_end" ]
  [ "$(jq -r '.severity' < "$lf")" = "warn" ]
  [ "$(jq -r '.pending_waves' < "$lf")" = "wave-1" ]
}

@test "warn-pending-wave-gate: no pending gates emits no event" {
  if ! command -v python3 >/dev/null 2>&1; then
    skip "python3 unavailable"
  fi
  mkdir -p "$SCRATCH/.factory"
  cat > "$SCRATCH/.factory/wave-state.yaml" <<EOF
waves:
  wave-1:
    gate_status: passed
EOF
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "cd '$SCRATCH' && echo '{}' | '$HOOKS_DIR/warn-pending-wave-gate.sh' 2>&1"
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

# ---------- VSDD_TELEMETRY=off ----------

@test "VSDD_TELEMETRY=off: pr-manager-completion-guard still blocks, no event" {
  local msg="STEP_COMPLETE: step=1 name=x status=ok"
  local input
  input=$(_subagent_input "pr-manager" "$msg")
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  export VSDD_TELEMETRY=off
  run bash -c "echo '$input' | '$HOOKS_DIR/pr-manager-completion-guard.sh' 2>&1"
  [ "$status" -eq 2 ]
  [ -z "$(_logfile)" ]
}
