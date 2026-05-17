#!/usr/bin/env bats
# fail-malformed-h2.bats — AC-4: hook blocks when h2 heading does not match canonical format
#
# Traces to:
#   BC-5.39.004 postcondition 2 (malformed h2 => HookResult::BlockWithFix naming required format)
#   BC-5.39.004 EC-002 (h2 is "## Fix Burst: description" — wrong prefix, no parenthesized date)
#   BC-5.39.004 Canonical Test Vectors: "Malformed h2" row
# D-NNN closure: D-421(e) + D-438(d) + D-439(a)
#
# Fixture: burst-log.md with `## Fix Burst: Pass-44 description without parenthesized date`
#          (wrong prefix AND no YYYY-MM-DD in parentheses); all 9 blocks otherwise present.
# Expected: hook exits 2 (block) and block_reason names the required format.
#
# RED GATE PHASE: test skips because validate-burst-log.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-burst-log.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-burst-log/fail-malformed-h2"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  cp -r "$FIXTURE_SRC/factory/." "$WORK/.factory/"
}

_write_registry() {
  cat > "$WORK/hooks-registry.toml" << 'TOML'
schema_version = 2

[[hooks]]
name = "validate-burst-log"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-burst-log.wasm"
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [
  ".factory/cycles/",
]
TOML
}

_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built -- run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WASM_PLUGIN" ]; then
    skip "validate-burst-log.wasm not built -- implement T-4 through T-7 of S-15.11"
  fi
}

_burst_log_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-malformed-h2","tool_input":{"file_path":".factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-4: malformed h2 => dispatcher exits 2 (block)
# Traces to BC-5.39.004 postcondition 2
# ---------------------------------------------------------------------------

@test "AC-4 FAIL: hook blocks when h2 heading uses wrong prefix and lacks parenthesized date" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_burst_log_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal emitted
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook
  [[ "$output" == *"blocking_plugins=validate-burst-log"* ]]
}

# ---------------------------------------------------------------------------
# AC-4: block message names the required canonical h2 format
# Traces to BC-5.39.004 postcondition 2 actionable message requirement
# ---------------------------------------------------------------------------

@test "AC-4 FAIL: block message cites required format '## Burst: <desc> (YYYY-MM-DD)'" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_burst_log_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal
  [ "$status" -eq 2 ]

  # block_reason must reference the canonical heading format (D-421(e) anchor)
  [[ "$output" == *"## Burst:"* ]]
  # Must name the malformed heading or the requirement for a parenthesized date
  [[ "$output" == *"YYYY-MM-DD"* ]] || [[ "$output" == *"h2"* ]] || [[ "$output" == *"heading"* ]]
}
