#!/usr/bin/env bats
# pass-valid-decisions-log.bats — AC-3: hook passes when STATE.md Decisions Log has unique
#                                  D-NNN rows in ascending order (D-490, D-491, D-492)
#
# Traces to:
#   BC-5.39.005 postcondition 1 (all properties hold => HookResult::Continue)
#   D-431(b) — Decisions Log monotonic-row enforcement (satisfied)
#   D-446(e) — single-row per D-NNN (satisfied)
#
# Fixture: STATE.md with D-490, D-491, D-492 ascending unique rows; all Phase 1 properties valid.
# Expected: hook exits 0 (no block).
#
# RED GATE PHASE: Phase 2 implementation does not exist yet.
# All tests in this file skip pending Phase 2 implementation (T-5 through T-8 of S-15.10).
#
# RED GATE NOTE: This PASS test may exit 0 for the wrong reason before Phase 2 is implemented
# (Phase 1 does not check Decisions Log, so it passes already). The implementer MUST verify
# the pass is genuine (not fail-open) by confirming Phase 1 + Phase 2 both return Continue.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-state-structure.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-state-structure/pass-valid-decisions-log"
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
name = "validate-state-structure"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-state-structure.wasm"
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
    skip "validate-state-structure.wasm not built -- implement T-5 through T-8 of S-15.10"
  fi
}

_state_md_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-valid-decisions-log","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-3: Decisions Log D-490, D-491, D-492 ascending unique => Continue (exit 0)
# Traces to BC-5.39.005 postcondition 1 / D-431(b) + D-446(e)
# ---------------------------------------------------------------------------

@test "AC-3 PASS: hook emits Continue when Decisions Log has D-490, D-491, D-492 in ascending order" {
  skip "pending Phase 2 implementation (S-15.10 T-5 through T-8)"
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: no block signal emitted (valid ascending unique Decisions Log)
  [ "$status" -eq 0 ]

  # No blocking_plugins= for a clean pass
  [[ "$output" != *"blocking_plugins="* ]]
}
