#!/usr/bin/env bats
# fail-no-h2.bats — AC-6: hook blocks when no h2 heading is present at all
#
# Traces to:
#   BC-5.39.004 postcondition 2 (no canonical h2 found => HookResult::BlockWithFix)
#   BC-5.39.004 EC-010 (no h2 heading at all in burst-log.md)
#   BC-5.39.004 Canonical Test Vectors: "No h2 present" row
# D-NNN closure: D-443(e)(ii) (own-burst h2 must be present at Commit A in real-time)
#
# Fixture: burst-log.md with all 9 block-type bold headers present but no `## Burst:` h2 heading.
#          Simulates the case where a state-manager writes the body before the h2 is added.
# Expected: hook exits 2 (block) and block_reason reports that no canonical h2 was found.
#
# RED GATE PHASE: test skips because validate-burst-log.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-burst-log.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-burst-log/fail-no-h2"
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-no-h2","tool_input":{"file_path":".factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-6: no h2 heading => dispatcher exits 2 (block)
# Traces to BC-5.39.004 postcondition 2 + EC-010
# ---------------------------------------------------------------------------

@test "AC-6 FAIL: hook blocks when burst-log.md contains no '## Burst:' h2 heading" {
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
# AC-6: block message reports absence of canonical h2 (D-443(e)(ii) enforcement)
# ---------------------------------------------------------------------------

@test "AC-6 FAIL: block message reports that no canonical h2 heading was found" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_burst_log_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal
  [ "$status" -eq 2 ]

  # block_reason must indicate the h2 heading is missing or not found
  [[ "$output" == *"## Burst:"* ]] || [[ "$output" == *"h2"* ]] || [[ "$output" == *"heading"* ]]
  # Must not contain blocking_plugins= from a different hook (verify correct hook fired)
  [[ "$output" == *"validate-burst-log"* ]]
}
