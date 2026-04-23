#!/usr/bin/env bats
# agent-guards-emission.bats — emission tests for the 2 PreToolUse Agent guards
# instrumented in observability phase 2c (v0.59.0).
#
# For each hook, we verify:
#   1. The block path writes a hook.block event with the correct reason code.
#   2. The hook still blocks correctly when emit-event is unreachable.
#
# Existing behavior tests live in wave-gate-hooks.bats and pr-lifecycle-hooks.bats.

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

# ---------- validate-wave-gate-prerequisite.sh ----------

_wave_state_with_pending_gate() {
  # Writes a wave-state.yaml to .factory/wave-state.yaml under $SCRATCH that
  # has wave-1 with gate_status=pending and wave-2 containing our target story.
  mkdir -p "$SCRATCH/.factory"
  cat > "$SCRATCH/.factory/wave-state.yaml" <<EOF
waves:
  wave-1:
    gate_status: pending
    stories: [S-1.01, S-1.02]
  wave-2:
    gate_status: unknown
    stories: [S-2.01]
EOF
}

@test "wave-gate-prerequisite: emits wave_gate_prerequisite_not_passed" {
  _wave_state_with_pending_gate
  local input
  input=$(jq -nc '{
    tool_name: "Agent",
    tool_input: {
      subagent_type: "test-writer",
      prompt: "Please implement S-2.01 for wave-2."
    }
  }')
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "cd '$SCRATCH' && echo '$input' | '$HOOKS_DIR/validate-wave-gate-prerequisite.sh' 2>&1"
  [ "$status" -eq 2 ]
  local f
  f=$(_logfile)
  [ -n "$f" ]
  [ "$(jq -r '.type' < "$f")" = "hook.block" ]
  [ "$(jq -r '.hook' < "$f")" = "validate-wave-gate-prerequisite" ]
  [ "$(jq -r '.reason' < "$f")" = "wave_gate_prerequisite_not_passed" ]
  [ "$(jq -r '.matcher' < "$f")" = "Agent" ]
  [ "$(jq -r '.story_id' < "$f")" = "S-2.01" ]
  [ "$(jq -r '.target_wave' < "$f")" = "wave-2" ]
  [ "$(jq -r '.blocking_wave' < "$f")" = "wave-1" ]
  [ "$(jq -r '.blocking_status' < "$f")" = "pending" ]
  [ "$(jq -r '.subagent' < "$f")" = "test-writer" ]
}

@test "wave-gate-prerequisite: still blocks when CLAUDE_PLUGIN_ROOT unset" {
  _wave_state_with_pending_gate
  local input
  input=$(jq -nc '{
    tool_name: "Agent",
    tool_input: {
      subagent_type: "test-writer",
      prompt: "Implement S-2.01"
    }
  }')
  run bash -c "unset CLAUDE_PLUGIN_ROOT; cd '$SCRATCH' && echo '$input' | '$HOOKS_DIR/validate-wave-gate-prerequisite.sh' 2>&1"
  [ "$status" -eq 2 ]
}

@test "wave-gate-prerequisite: still blocks when emit-event path broken" {
  _wave_state_with_pending_gate
  local input
  input=$(jq -nc '{
    tool_name: "Agent",
    tool_input: {
      subagent_type: "test-writer",
      prompt: "Implement S-2.01"
    }
  }')
  run bash -c "CLAUDE_PLUGIN_ROOT=/nonexistent cd '$SCRATCH' && echo '$input' | '$HOOKS_DIR/validate-wave-gate-prerequisite.sh' 2>&1"
  [ "$status" -eq 2 ]
}

@test "wave-gate-prerequisite: passed gate emits no event" {
  mkdir -p "$SCRATCH/.factory"
  cat > "$SCRATCH/.factory/wave-state.yaml" <<EOF
waves:
  wave-1:
    gate_status: passed
    stories: [S-1.01]
  wave-2:
    gate_status: unknown
    stories: [S-2.01]
EOF
  local input
  input=$(jq -nc '{
    tool_name: "Agent",
    tool_input: {
      subagent_type: "test-writer",
      prompt: "Implement S-2.01"
    }
  }')
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "cd '$SCRATCH' && echo '$input' | '$HOOKS_DIR/validate-wave-gate-prerequisite.sh' 2>&1"
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

@test "wave-gate-prerequisite: non-worker subagent emits no event" {
  _wave_state_with_pending_gate
  local input
  input=$(jq -nc '{
    tool_name: "Agent",
    tool_input: {
      subagent_type: "orchestrator",
      prompt: "S-2.01"
    }
  }')
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "cd '$SCRATCH' && echo '$input' | '$HOOKS_DIR/validate-wave-gate-prerequisite.sh' 2>&1"
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

# ---------- validate-pr-merge-prerequisites.sh ----------

_delivery_dir_empty() {
  # Creates .factory/code-delivery/STORY-042/ with nothing in it.
  mkdir -p "$SCRATCH/.factory/code-delivery/STORY-042"
}

_delivery_dir_complete() {
  mkdir -p "$SCRATCH/.factory/code-delivery/STORY-042"
  echo "desc" > "$SCRATCH/.factory/code-delivery/STORY-042/pr-description.md"
  echo "review" > "$SCRATCH/.factory/code-delivery/STORY-042/pr-review.md"
  echo "security" > "$SCRATCH/.factory/code-delivery/STORY-042/security-review.md"
}

@test "pr-merge-prerequisites: emits pr_merge_evidence_missing when files absent" {
  _delivery_dir_empty
  local input
  input=$(jq -nc --arg p "$SCRATCH" '{
    tool_name: "Agent",
    tool_input: {
      subagent_type: "github-ops",
      prompt: ("cd " + $p + " && gh pr merge for STORY-042")
    }
  }')
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$input' | '$HOOKS_DIR/validate-pr-merge-prerequisites.sh' 2>&1"
  [ "$status" -eq 2 ]
  local f
  f=$(_logfile)
  [ -n "$f" ]
  [ "$(jq -r '.type' < "$f")" = "hook.block" ]
  [ "$(jq -r '.hook' < "$f")" = "validate-pr-merge-prerequisites" ]
  [ "$(jq -r '.reason' < "$f")" = "pr_merge_evidence_missing" ]
  [ "$(jq -r '.matcher' < "$f")" = "Agent" ]
  [ "$(jq -r '.story_id' < "$f")" = "STORY-042" ]
  # All three files missing
  local missing
  missing=$(jq -r '.missing' < "$f")
  [[ "$missing" == *"pr-description.md"* ]]
  [[ "$missing" == *"pr-review.md"* ]]
  [[ "$missing" == *"security-review.md"* ]]
}

@test "pr-merge-prerequisites: still blocks when CLAUDE_PLUGIN_ROOT unset" {
  _delivery_dir_empty
  local input
  input=$(jq -nc --arg p "$SCRATCH" '{
    tool_name: "Agent",
    tool_input: {
      subagent_type: "github-ops",
      prompt: ("cd " + $p + " && gh pr merge for STORY-042")
    }
  }')
  run bash -c "unset CLAUDE_PLUGIN_ROOT; echo '$input' | '$HOOKS_DIR/validate-pr-merge-prerequisites.sh' 2>&1"
  [ "$status" -eq 2 ]
}

@test "pr-merge-prerequisites: complete delivery emits no event" {
  _delivery_dir_complete
  local input
  input=$(jq -nc --arg p "$SCRATCH" '{
    tool_name: "Agent",
    tool_input: {
      subagent_type: "github-ops",
      prompt: ("cd " + $p + " && gh pr merge for STORY-042")
    }
  }')
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$input' | '$HOOKS_DIR/validate-pr-merge-prerequisites.sh' 2>&1"
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

@test "pr-merge-prerequisites: non-merge github-ops call emits no event" {
  _delivery_dir_empty
  local input
  input=$(jq -nc --arg p "$SCRATCH" '{
    tool_name: "Agent",
    tool_input: {
      subagent_type: "github-ops",
      prompt: ("cd " + $p + " && gh pr view 42")
    }
  }')
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  run bash -c "echo '$input' | '$HOOKS_DIR/validate-pr-merge-prerequisites.sh' 2>&1"
  [ "$status" -eq 0 ]
  [ -z "$(_logfile)" ]
}

# ---------- VSDD_TELEMETRY=off across both ----------

@test "VSDD_TELEMETRY=off: wave-gate-prerequisite still blocks, no event" {
  _wave_state_with_pending_gate
  local input
  input=$(jq -nc '{
    tool_name: "Agent",
    tool_input: {
      subagent_type: "test-writer",
      prompt: "Implement S-2.01"
    }
  }')
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  export VSDD_TELEMETRY=off
  run bash -c "cd '$SCRATCH' && echo '$input' | '$HOOKS_DIR/validate-wave-gate-prerequisite.sh' 2>&1"
  [ "$status" -eq 2 ]
  [ -z "$(_logfile)" ]
}

@test "VSDD_TELEMETRY=off: pr-merge-prerequisites still blocks, no event" {
  _delivery_dir_empty
  local input
  input=$(jq -nc --arg p "$SCRATCH" '{
    tool_name: "Agent",
    tool_input: {
      subagent_type: "github-ops",
      prompt: ("cd " + $p + " && gh pr merge for STORY-042")
    }
  }')
  export CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT"
  export VSDD_TELEMETRY=off
  run bash -c "echo '$input' | '$HOOKS_DIR/validate-pr-merge-prerequisites.sh' 2>&1"
  [ "$status" -eq 2 ]
  [ -z "$(_logfile)" ]
}
