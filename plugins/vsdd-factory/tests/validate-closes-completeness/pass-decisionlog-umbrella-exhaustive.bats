#!/usr/bin/env bats
# pass-decisionlog-umbrella-exhaustive.bats — AC-11: hook passes when decision-log.md
#                                              umbrella cite has "(exhaustive)" flag
#
# Traces to:
#   BC-5.39.007 postcondition 1 (all properties hold => HookResult::Continue)
#   BC-5.39.007 EC-008 (umbrella cite with (exhaustive) flag => Continue)
#   BC-5.39.007 invariant 3 (path-component-strict guard)
# D-NNN closure: D-441(c)+D-442(c) satisfied
#
# Fixture: decision-log.md with "D-401..D-454 (exhaustive)" cite.
# Expected: hook exits 0 (no block).
#
# RED GATE PHASE: test skips because validate-closes-completeness.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-closes-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-closes-completeness/pass-decisionlog-umbrella-exhaustive"
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
name = "validate-closes-completeness"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-closes-completeness.wasm"
priority = 156
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [
  ".factory/cycles",
  ".factory/STATE.md",
  ".factory/cycles/v1.0-brownfield-backfill/INDEX.md",
  ".factory/cycles/v1.0-brownfield-backfill/lessons.md",
]
TOML
}

_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built -- run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WASM_PLUGIN" ]; then
    skip "validate-closes-completeness.wasm not built -- implement T-4 through T-7 of S-15.12"
  fi
}

_decision_log_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-decisionlog-umbrella-exhaustive","tool_input":{"file_path":".factory/cycles/v1.0-brownfield-backfill/decision-log.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-11: decision-log.md umbrella cite with (exhaustive) flag => exits 0 (no block)
# Traces to BC-5.39.007 postcondition 1 / EC-008
# ---------------------------------------------------------------------------

@test "AC-11 PASS: hook emits Continue when decision-log.md umbrella cite has (exhaustive) flag" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_decision_log_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: no block signal emitted
  [ "$status" -eq 0 ]

  # No blocking_plugins= for a clean pass
  [[ "$output" != *"blocking_plugins="* ]]
}
