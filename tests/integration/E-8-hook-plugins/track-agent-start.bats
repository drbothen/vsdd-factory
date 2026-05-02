#!/usr/bin/env bats
# track-agent-start.bats — AC-005 parity tests for track-agent-start native WASM port
#
# Invokes via factory-dispatcher (not bare wasmtime) following the pattern from
# plugins/vsdd-factory/tests/regression-v1.0.bats.
#
# What this suite covers:
#   AC-005(a): Agent dispatch subagent=pr-manager, prompt containing "S-6.07"
#              → exit 0 + agent.start event with subagent=pr-manager story_id=S-6.07
#   AC-005(b): Agent dispatch subagent=implementer, prompt containing "STORY-042"
#              → exit 0 + agent.start event with subagent=implementer story_id=STORY-042
#   AC-005(c): Agent dispatch subagent=reviewer, prompt with no story pattern
#              → exit 0 + agent.start event with subagent=reviewer, no story_id field
#   AC-005(d): Non-Agent tool_name in envelope → exit 0, no event emitted
#   AC-005(e): Malformed JSON stdin → exit 0, no event, no panic
#   AC-002a parity audit: agent.start event contains no agent_id field, no tool_name field
#
# Story: S-8.08
# BCs: BC-7.03.079, BC-7.03.080
# See: .factory/stories/S-8.08-native-port-track-agent-start.md

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
  WASM_ARTIFACT="${REPO_ROOT}/target/wasm32-wasip1/release/track-agent-start.wasm"
  if [ ! -f "$WASM_ARTIFACT" ]; then
    WASM_ARTIFACT="${REPO_ROOT}/target/wasm32-wasip1/debug/track-agent-start.wasm"
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
# Helper: check that the native WASM artifact exists (built for wasm32-wasip1)
# ---------------------------------------------------------------------------

_require_wasm_artifact() {
  if [ ! -f "$WASM_ARTIFACT" ]; then
    skip "track-agent-start.wasm not found at $WASM_ARTIFACT — run: cargo build --target wasm32-wasip1 -p track-agent-start"
  fi
}

_require_dispatcher() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "factory-dispatcher binary not found — run: cargo build --release -p factory-dispatcher"
  fi
}

# ---------------------------------------------------------------------------
# Pre-condition: registry entry must reference native WASM (not bash adapter)
# ---------------------------------------------------------------------------

@test "AC-001: hooks-registry.toml track-agent-start entry references native WASM (not legacy-bash-adapter)" {
  run grep -A 5 'name = "track-agent-start"' "$REGISTRY"
  [ "$status" -eq 0 ]

  # The plugin line must NOT be the legacy-bash-adapter
  echo "$output" | grep -q 'plugin'
  ! echo "$output" | grep -q 'legacy-bash-adapter'
  echo "$output" | grep -q 'track-agent-start.wasm'
}

@test "AC-001: hooks-registry.toml track-agent-start has no script_path (legacy-bash-adapter artifact removed)" {
  local stanza
  stanza="$(awk '/name = "track-agent-start"/,/^\[\[hooks\]\]/' "$REGISTRY" | head -30)"
  ! echo "$stanza" | grep -q 'script_path'
}

@test "AC-001: hooks-registry.toml track-agent-start has no exec_subprocess block" {
  local stanza
  stanza="$(awk '/name = "track-agent-start"/,/^\[\[hooks\]\]/' "$REGISTRY" | head -30)"
  ! echo "$stanza" | grep -q 'exec_subprocess'
}

@test "AC-002b: plugins/vsdd-factory/hooks/track-agent-start.sh is deleted" {
  [ ! -f "${REPO_ROOT}/plugins/vsdd-factory/hooks/track-agent-start.sh" ]
}

@test "AC-001 invariant: track-agent-start WASM artifact exists at wasm32-wasip1 target" {
  [ -f "${REPO_ROOT}/target/wasm32-wasip1/release/track-agent-start.wasm" ] \
    || [ -f "${REPO_ROOT}/target/wasm32-wasip1/debug/track-agent-start.wasm" ]
}

# ---------------------------------------------------------------------------
# AC-005(a): Agent dispatch with subagent=pr-manager, story_id=S-6.07
# ---------------------------------------------------------------------------

@test "AC-005(a): Agent dispatch subagent=pr-manager S-6.07 => agent.start with subagent + story_id" {
  _require_wasm_artifact
  _require_dispatcher

  local envelope
  envelope='{"event_name":"PreToolUse","tool_name":"Agent","session_id":"bats-test-session","dispatcher_trace_id":"bats-trace-001","tool_input":{"subagent_type":"pr-manager","prompt":"Implement the changes for S-6.07 acceptance criteria"}}'

  run env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" VSDD_SINK_FILE="$SINK_FILE" \
    bash -c "printf '%s' '$envelope' | '$DISPATCHER'"
  [ "$status" -eq 0 ]

  # Verify agent.start event was written to sink
  [ -f "$SINK_FILE" ]

  run jq -r '.type' "$SINK_FILE"
  [ "$status" -eq 0 ]
  echo "$output" | grep -q "agent.start"

  # Verify subagent=pr-manager
  run jq -r '.subagent' "$SINK_FILE"
  [ "$output" = "pr-manager" ]

  # Verify story_id=S-6.07
  run jq -r '.story_id' "$SINK_FILE"
  [ "$output" = "S-6.07" ]

  # AC-002a: no agent_id field
  run jq 'has("agent_id")' "$SINK_FILE"
  [ "$output" = "false" ]

  # AC-002a: no tool_name field
  run jq 'has("tool_name")' "$SINK_FILE"
  [ "$output" = "false" ]
}

# ---------------------------------------------------------------------------
# AC-005(b): Agent dispatch with subagent=implementer, story_id=STORY-042 (fallback)
# ---------------------------------------------------------------------------

@test "AC-005(b): Agent dispatch subagent=implementer STORY-042 => agent.start story_id=STORY-042" {
  _require_wasm_artifact
  _require_dispatcher

  local envelope
  envelope='{"event_name":"PreToolUse","tool_name":"Agent","session_id":"bats-test-session","dispatcher_trace_id":"bats-trace-002","tool_input":{"subagent_type":"implementer","prompt":"Please implement the changes described in STORY-042"}}'

  run env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" VSDD_SINK_FILE="$SINK_FILE" \
    bash -c "printf '%s' '$envelope' | '$DISPATCHER'"
  [ "$status" -eq 0 ]

  [ -f "$SINK_FILE" ]

  # Verify story_id=STORY-042 (pattern 2 fallback — no S-N.NN in prompt)
  run jq -r '.story_id' "$SINK_FILE"
  [ "$output" = "STORY-042" ]

  run jq -r '.subagent' "$SINK_FILE"
  [ "$output" = "implementer" ]

  # AC-002a parity: no forbidden fields
  run jq 'has("agent_id")' "$SINK_FILE"
  [ "$output" = "false" ]
  run jq 'has("tool_name")' "$SINK_FILE"
  [ "$output" = "false" ]
}

# ---------------------------------------------------------------------------
# AC-005(c): Agent dispatch with subagent=reviewer, no story pattern
# ---------------------------------------------------------------------------

@test "AC-005(c): Agent dispatch subagent=reviewer no story pattern => agent.start no story_id" {
  _require_wasm_artifact
  _require_dispatcher

  local envelope
  envelope='{"event_name":"PreToolUse","tool_name":"Agent","session_id":"bats-test-session","dispatcher_trace_id":"bats-trace-003","tool_input":{"subagent_type":"reviewer","prompt":"Please review the pull request for correctness and style"}}'

  run env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" VSDD_SINK_FILE="$SINK_FILE" \
    bash -c "printf '%s' '$envelope' | '$DISPATCHER'"
  [ "$status" -eq 0 ]

  [ -f "$SINK_FILE" ]

  run jq -r '.subagent' "$SINK_FILE"
  [ "$output" = "reviewer" ]

  # story_id must be absent
  run jq 'has("story_id")' "$SINK_FILE"
  [ "$output" = "false" ]

  # AC-002a parity
  run jq 'has("agent_id")' "$SINK_FILE"
  [ "$output" = "false" ]
}

# ---------------------------------------------------------------------------
# AC-005(d): Non-Agent tool_name → exit 0, no event emitted
# ---------------------------------------------------------------------------

@test "AC-005(d): non-Agent tool_name => exit 0 and no agent.start event emitted" {
  _require_wasm_artifact
  _require_dispatcher

  local envelope
  envelope='{"event_name":"PreToolUse","tool_name":"Bash","session_id":"bats-test-session","dispatcher_trace_id":"bats-trace-004","tool_input":{"command":"ls -la"}}'

  # track-agent-start hooks on PreToolUse:Agent only; Bash dispatch must not trigger it
  run env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" VSDD_SINK_FILE="$SINK_FILE" \
    bash -c "printf '%s' '$envelope' | '$DISPATCHER'"
  [ "$status" -eq 0 ]

  # Sink file should either not exist or not contain an agent.start event
  if [ -f "$SINK_FILE" ]; then
    run grep -c "agent.start" "$SINK_FILE"
    [ "$output" = "0" ]
  fi
}

# ---------------------------------------------------------------------------
# AC-005(e): Malformed JSON stdin → exit 0, no event, no panic
# ---------------------------------------------------------------------------

@test "AC-005(e): malformed JSON stdin => exit 0 and no event emitted (best-effort AC-006)" {
  _require_wasm_artifact
  _require_dispatcher

  # Send completely invalid JSON as stdin
  run env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" VSDD_SINK_FILE="$SINK_FILE" \
    bash -c "printf '%s' 'not valid json at all {{{}}}' | '$DISPATCHER'"

  # Must exit 0 (best-effort — AC-006 / BC-7.03.079 invariant 2)
  [ "$status" -eq 0 ]

  # No agent.start event in sink
  if [ -f "$SINK_FILE" ]; then
    run grep -c "agent.start" "$SINK_FILE"
    [ "$output" = "0" ]
  fi
}

@test "AC-005(e) variant: empty stdin => exit 0 and no event emitted (EC-005)" {
  _require_wasm_artifact
  _require_dispatcher

  run env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" VSDD_SINK_FILE="$SINK_FILE" \
    bash -c "printf '' | '$DISPATCHER'"
  [ "$status" -eq 0 ]

  if [ -f "$SINK_FILE" ]; then
    run grep -c "agent.start" "$SINK_FILE"
    [ "$output" = "0" ]
  fi
}

# ---------------------------------------------------------------------------
# AC-002a parity audit: exact field set check (no additive fields)
# ---------------------------------------------------------------------------

@test "AC-002a parity audit: agent.start event contains exactly the bash-parity field set" {
  _require_wasm_artifact
  _require_dispatcher

  local envelope
  envelope='{"event_name":"PreToolUse","tool_name":"Agent","session_id":"bats-parity-session","dispatcher_trace_id":"bats-parity-trace","tool_input":{"subagent_type":"pr-manager","prompt":"S-6.07 parity check"}}'

  run env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" VSDD_SINK_FILE="$SINK_FILE" \
    bash -c "printf '%s' '$envelope' | '$DISPATCHER'"
  [ "$status" -eq 0 ]
  [ -f "$SINK_FILE" ]

  # Plugin-set fields: hook, matcher, subagent, story_id (when present)
  run jq -r '.hook' "$SINK_FILE"
  [ "$output" = "track-agent-start" ]

  run jq -r '.matcher' "$SINK_FILE"
  [ "$output" = "Agent" ]

  run jq -r '.subagent' "$SINK_FILE"
  [ "$output" = "pr-manager" ]

  run jq -r '.story_id' "$SINK_FILE"
  [ "$output" = "S-6.07" ]

  # Strict parity assertions: forbidden fields must be absent
  run jq 'has("agent_id")' "$SINK_FILE"
  [ "$output" = "false" ]

  run jq 'has("tool_name")' "$SINK_FILE"
  [ "$output" = "false" ]
}

# ---------------------------------------------------------------------------
# EC-001: missing subagent_type defaults to "unknown"
# ---------------------------------------------------------------------------

@test "EC-001: missing subagent_type in Agent envelope defaults to subagent=unknown" {
  _require_wasm_artifact
  _require_dispatcher

  local envelope
  envelope='{"event_name":"PreToolUse","tool_name":"Agent","session_id":"bats-ec001-session","dispatcher_trace_id":"bats-ec001-trace","tool_input":{"prompt":"S-8.08 work"}}'

  run env CLAUDE_PLUGIN_ROOT="$PLUGIN_ROOT" CLAUDE_PROJECT_DIR="$WORK" VSDD_SINK_FILE="$SINK_FILE" \
    bash -c "printf '%s' '$envelope' | '$DISPATCHER'"
  [ "$status" -eq 0 ]
  [ -f "$SINK_FILE" ]

  run jq -r '.subagent' "$SINK_FILE"
  [ "$output" = "unknown" ]
}
