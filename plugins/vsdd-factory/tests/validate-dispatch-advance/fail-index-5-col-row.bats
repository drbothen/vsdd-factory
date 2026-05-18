#!/usr/bin/env bats
# fail-index-5-col-row.bats — AC-11: hook blocks when INDEX.md ## Adversarial Reviews row has 4 columns
#
# BC-5.39.006 v1.1 update (F-P1-002): canonical schema is 5-col (6 pipes). A 4-col row (5 pipes)
# in a 5-col-header ## Adversarial Reviews section violates EC-013.
#
# Traces to:
#   BC-5.39.006 v1.1 postcondition 9; D-441(b)/D-442(b); EC-013
#
# Fixture: INDEX.md with ## Adversarial Reviews (5-col header) and one 4-column data row (5 pipe chars).
# Expected: hook exits 2; block message names row, actual=4, required=5, cites D-441(b).
#
# RED GATE PHASE: test skips if validate-dispatch-advance.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-dispatch-advance.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-dispatch-advance/fail-index-5-col-row"
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-index-5-col-row","tool_input":{"file_path":".factory/cycles/v1.0-brownfield-backfill/INDEX.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-11: INDEX.md ## Adversarial Reviews with 4-col row => hook blocks (exit 2)
# Traces to BC-5.39.006 v1.1 postcondition 9; D-441(b)/D-442(b); EC-013
# ---------------------------------------------------------------------------

@test "AC-11 FAIL: hook blocks when INDEX.md adversary-pass row has 4 columns (5 pipe chars)" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_index_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal emitted for non-5-column row in 5-col-header section
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook
  [[ "$output" == *"blocking_plugins=validate-dispatch-advance"* ]]
}

@test "AC-11 FAIL: block message names actual=4 required=5 and cites D-441" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_index_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
  # Block message must name actual column count 4 and required 5
  [[ "$output" == *"4"* ]]
  [[ "$output" == *"5"* ]]
  # Must cite D-441(b)
  [[ "$output" == *"D-441"* ]]
}
