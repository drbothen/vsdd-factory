#!/usr/bin/env bats
# fail-dim1-cardinality.bats — AC-2: hook blocks when Dim-1 headline count does not match enumerated list
#
# Traces to:
#   BC-5.39.004 postcondition 4 (Dim-1 mismatch => HookResult::BlockWithFix naming both counts)
#   BC-5.39.004 EC-005 (headline "5 unique files" but list has 7 items)
#   BC-5.39.004 Canonical Test Vectors: "Dim-1 mismatch" row
# D-NNN closure: D-432(e) + D-448(d)(i)
#
# Fixture: burst-log.md with correct h2 and all 9 blocks; Dim-1 headline says "5 unique files"
#          but the enumerated list contains 7 file entries.
# Expected: hook exits 2 (block) and block_reason cites both counts (5 and 7).
#
# RED GATE PHASE: test skips because validate-burst-log.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-burst-log.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-burst-log/fail-dim1-cardinality"
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
  ".factory/cycles/**",
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-dim1-cardinality","tool_input":{"file_path":".factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-2: Dim-1 headline 5 vs list 7 => dispatcher exits 2 (block)
# Traces to BC-5.39.004 postcondition 4
# ---------------------------------------------------------------------------

@test "AC-2 FAIL: hook blocks when Dim-1 headline states 5 but enumerated list has 7 files" {
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
# AC-2 block message: names both the headline count (5) and the actual list count (7)
# Traces to BC-5.39.004 postcondition 4 actionable message requirement
# ---------------------------------------------------------------------------

@test "AC-2 FAIL: block message cites headline count 5 and actual list count 7" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_burst_log_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal
  [ "$status" -eq 2 ]

  # block_reason must name both the headline count and the actual list count (D-432(e))
  [[ "$output" == *"5"* ]]
  [[ "$output" == *"7"* ]]
  # Confirm this is a Dim-1 cardinality violation message
  [[ "$output" == *"Dim-1"* ]]
}
