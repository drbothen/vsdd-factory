#!/usr/bin/env bats
# fail-open-index-unreadable.bats — AC-6: hook is fail-open when INDEX.md is unreadable;
#                                    emits Continue + log_warn (not Block)
#
# Traces to:
#   BC-5.39.005 invariant 7 (all host::read_file calls are fail-open)
#   P2-PC-1 — tally check is fail-open if INDEX.md is unreadable
#   EC-007 — host::read_file returns HostError::CapabilityDenied => Continue + log_warn
#
# Scenario: the hook is invoked with STATE.md but no INDEX.md exists in the WORK sandbox.
#           host::read_file for INDEX.md returns a file-not-found error.
#           The tally check MUST be skipped (fail-open). Other Phase 2 checks proceed normally.
#           The fixture STATE.md has all other Phase 1 + Phase 2 properties valid, so the
#           overall result is Continue.
#
# Note: this test does NOT arrange an INDEX.md file. The fixture directory has STATE.md only.
#       The WASM hook attempts to read .factory/cycles/v1.0-brownfield-backfill/INDEX.md
#       and receives a read error, triggering fail-open for the tally check only.
#
# RED GATE PHASE: Phase 2 implementation does not exist yet.
# All tests in this file skip pending Phase 2 implementation (T-5 through T-8 of S-15.10).
#
# RED GATE NOTE: Before Phase 2 is implemented, Phase 1 already passes this fixture (no
# banner/margin/tail violation). The test will pass for the right reason only after Phase 2
# implements the fail-open tally check path in run_hook_phase2.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-state-structure.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-state-structure/fail-open-index-unreadable"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  # Copies STATE.md only — no INDEX.md present (absent INDEX.md triggers fail-open)
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-open-index-unreadable","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-6: INDEX.md absent => tally check is fail-open => hook emits Continue (exit 0)
# Traces to BC-5.39.005 invariant 7; P2-PC-1 fail-open; EC-007
# ---------------------------------------------------------------------------

@test "AC-6 PASS (fail-open): hook emits Continue when INDEX.md is unreadable (tally check skipped)" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  # Confirm INDEX.md is absent from the WORK sandbox (prerequisite for this test)
  [ ! -f "$WORK/.factory/cycles/v1.0-brownfield-backfill/INDEX.md" ]

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: fail-open means no block even though INDEX.md is unreadable
  [ "$status" -eq 0 ]

  # No blocking_plugins= for a fail-open pass
  [[ "$output" != *"blocking_plugins="* ]]
}
