#!/usr/bin/env bats
# pass-all-valid-state.bats — AC-9: hook emits Continue when all STATE.md validations pass
#
# Traces to:
#   BC-5.39.006 postcondition 1; EC-011
#
# Fixture: STATE.md with current_step: satisfying all conditions:
#   - No forbidden meta-commentary pattern
#   - All 4 index cites: BC-INDEX v2.30 / VP-INDEX v1.97 / STORY-INDEX v3.31 / ARCH-INDEX v2.18
#   - Tail →9→9→9→9 (LENGTH=4)
#   - D-382..D-477 with D-477 as highest in Decisions Log
# Expected: hook exits 0 (no block).
#
# RED GATE PHASE: test skips if validate-dispatch-advance.wasm is not yet compiled.
# When WASM is built, the todo!() bodies panic => hook crashes => dispatcher fail-opens => exit 0.
# This test will PASS against stubs (exit 0) but for the WRONG reason (crash/fail-open vs. real Continue).
# Red Gate note: this test is load-bearing for the implementer — it must pass for the RIGHT reason
# after implementation (genuine Continue, not fail-open from crash). The implementer MUST verify
# that Scenario B of integration-production-registry.bats (which CAN distinguish) also passes.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-dispatch-advance.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-dispatch-advance/pass-all-valid-state"
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-all-valid-state","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-9: valid STATE.md => hook exits 0 (Continue, not Block)
# Traces to BC-5.39.006 postcondition 1; EC-011
# ---------------------------------------------------------------------------

@test "AC-9 PASS: hook emits Continue when current_step: has no violations (all conditions satisfied)" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: no block signal emitted
  [ "$status" -eq 0 ]

  # No blocking_plugins= for a clean pass
  [[ "$output" != *"blocking_plugins=validate-dispatch-advance"* ]]
}
