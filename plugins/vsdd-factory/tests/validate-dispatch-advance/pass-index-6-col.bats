#!/usr/bin/env bats
# pass-index-6-col.bats — AC-13: hook emits Continue when all INDEX.md adversary-pass rows are 6 columns
#
# Traces to:
#   BC-5.39.006 postcondition 8; EC-015
#
# Fixture: INDEX.md where all adversary-pass rows have exactly 6 columns (8 pipe chars per row).
# Expected: hook exits 0 (no block).
#
# RED GATE PHASE: test skips if validate-dispatch-advance.wasm is not yet compiled.
# Note: As with pass-all-valid-state.bats, when WASM is built stubs crash => fail-open => exit 0.
# The load-bearing Red Gate distinction is that integration-production-registry.bats Scenario B
# must also pass for the right reason (genuine block, not fail-open).

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-dispatch-advance.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-dispatch-advance/pass-index-6-col"
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
name = "validate-dispatch-advance"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-dispatch-advance.wasm"
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [
  ".factory",
]
TOML
}

_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built -- run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WASM_PLUGIN" ]; then
    skip "validate-dispatch-advance.wasm not built -- implement T-5 through T-7 of S-15.14"
  fi
}

_index_md_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-index-6-col","tool_input":{"file_path":".factory/cycles/v1.0-brownfield-backfill/INDEX.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-13: valid 6-column INDEX.md => hook exits 0 (Continue, not Block)
# Traces to BC-5.39.006 postcondition 8; EC-015
# ---------------------------------------------------------------------------

@test "AC-13 PASS: hook emits Continue when all INDEX.md adversary-pass rows are exactly 6 columns" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_index_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: no block signal emitted
  [ "$status" -eq 0 ]

  # No blocking_plugins= for a clean pass
  [[ "$output" != *"blocking_plugins=validate-dispatch-advance"* ]]
}
