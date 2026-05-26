#!/usr/bin/env bats
# pass-tally-sync.bats — AC-5: hook passes when STATE.md tally values agree with INDEX.md
#
# Traces to:
#   BC-5.39.005 postcondition 1 (all properties hold => HookResult::Continue)
#   D-432(a) — tally-sync satisfied (both show pass count: 72)
#
# Fixture pair:
#   STATE.md — Convergence Status "pass count: 72"
#   INDEX.md — Convergence Status "pass count: 72" (agrees with STATE.md)
# Expected: hook exits 0 (no block).
#
# RED GATE PHASE: Phase 2 implementation does not exist yet.
# All tests in this file skip pending Phase 2 implementation (T-5 through T-8 of S-15.10).
#
# RED GATE NOTE: This PASS test may exit 0 for the wrong reason before Phase 2 is implemented
# (Phase 1 does not check tally sync). Implementer must verify genuine Continue after T-8.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-state-structure.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-state-structure/pass-tally-sync"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  # Copies both STATE.md and cycles/v1.0-brownfield-backfill/INDEX.md
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-tally-sync","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-5: STATE.md and INDEX.md both show pass count 72 => Continue (exit 0)
# Traces to BC-5.39.005 postcondition 1 / D-432(a) satisfied
# ---------------------------------------------------------------------------

@test "AC-5 PASS: hook emits Continue when STATE.md and INDEX.md tally values agree (pass count: 72)" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: no block signal emitted (tally values agree)
  [ "$status" -eq 0 ]

  # No blocking_plugins= for a clean pass
  [[ "$output" != *"blocking_plugins="* ]]
}
