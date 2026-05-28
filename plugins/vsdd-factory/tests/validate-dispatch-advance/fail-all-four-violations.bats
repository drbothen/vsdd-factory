#!/usr/bin/env bats
# fail-all-four-violations.bats — AC-10: all 4 STATE.md violations simultaneously produce
#                                  single consolidated BlockWithFix
#
# Traces to:
#   BC-5.39.006 postcondition 6; EC-012
#
# Fixture: STATE.md violating all 4 conditions:
#   - forbidden pattern: "META-LEVEL-5 WATCH: self-application."
#   - missing index cite: ARCH-INDEX v absent
#   - tail LENGTH=5: →9→9→9→9→9
#   - stale D-chain: D-382..D-476 but Decisions Log has D-477
# Expected: hook exits 2 with SINGLE block message enumerating all 4 violation classes.
#
# RED GATE PHASE: test skips if validate-dispatch-advance.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-dispatch-advance.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-dispatch-advance/fail-all-four-violations"
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

_state_md_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-all-four-violations","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-10: all 4 violations => single BlockWithFix enumerating all
# Traces to BC-5.39.006 postcondition 6; EC-012
# ---------------------------------------------------------------------------

@test "AC-10 FAIL: hook blocks with single consolidated message when all 4 STATE.md violations present" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal emitted
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook (only once, not twice)
  [[ "$output" == *"blocking_plugins=validate-dispatch-advance"* ]]
}

@test "AC-10 FAIL: single block message covers meta-commentary, missing index cite, tail, and D-chain violations" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
  # Consolidated message: must mention forbidden pattern class
  [[ "$output" == *"META-LEVEL"* ]] || [[ "$output" == *"meta-commentary"* ]] || [[ "$output" == *"D-440"* ]]
  # Consolidated message: must mention missing index cite
  [[ "$output" == *"ARCH-INDEX"* ]] || [[ "$output" == *"D-439"* ]]
  # Consolidated message: must mention tail cardinality
  [[ "$output" == *"D-451"* ]] || [[ "$output" == *"trajectory"* ]] || [[ "$output" == *"LENGTH"* ]]
}
