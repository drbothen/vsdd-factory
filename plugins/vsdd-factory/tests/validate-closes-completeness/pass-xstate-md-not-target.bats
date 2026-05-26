#!/usr/bin/env bats
# pass-xstate-md-not-target.bats — AC-17: path /some/dir/xSTATE.md does NOT trigger
#                                    STATE.md validation (path-component-strict guard)
#
# Traces to:
#   BC-5.39.007 invariant 3 (path-component-strict guard: file_name() == "STATE.md" only)
#   BC-5.39.007 EC-015 (xSTATE.md path does not match STATE.md guard => Continue)
#
# Scenario: envelope file_path is ".factory/xSTATE.md" (not ".factory/STATE.md").
#           The path-component-strict guard must NOT match xSTATE.md, so umbrella-flag
#           checks must NOT run even though the content contains a bare D-389..D-480 range.
# Expected: hook exits 0 (no block — xSTATE.md is not a target file).
#
# RED GATE PHASE: test skips because validate-closes-completeness.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-closes-completeness.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-closes-completeness/pass-xstate-md-not-target"
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

_xstate_md_envelope() {
  # file_path is xSTATE.md — must NOT trigger STATE.md validation arm
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-xstate-md-not-target","tool_input":{"file_path":".factory/xSTATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-17: xSTATE.md path does NOT trigger STATE.md validation => exits 0
# Traces to BC-5.39.007 invariant 3 / EC-015
# ---------------------------------------------------------------------------

@test "AC-17 PASS: hook emits Continue for xSTATE.md path (path-component-strict guard must not match)" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_xstate_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: no block signal emitted — xSTATE.md is not a target
  [ "$status" -eq 0 ]

  # No blocking_plugins= for a clean pass
  [[ "$output" != *"blocking_plugins="* ]]
}
