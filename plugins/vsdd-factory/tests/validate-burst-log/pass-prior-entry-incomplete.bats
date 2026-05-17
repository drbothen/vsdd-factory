#!/usr/bin/env bats
# pass-prior-entry-incomplete.bats — AC-5: hook validates latest h2 only;
#                                    prior incomplete entries do not cause block
#
# Traces to:
#   BC-5.39.004 postcondition 1 (all properties hold for latest entry => Continue)
#   BC-5.39.004 invariant 4 (only latest burst entry is validated; prior entries ignored)
#   BC-5.39.004 EC-008 (old entry incomplete, latest entry complete => pass)
#   BC-5.39.004 Canonical Test Vectors: "Prior entry incomplete, current complete" row
# D-NNN closure: D-443(e)(ii) correctness boundary
#
# Fixture: burst-log.md with an old incomplete h2+entry (only 3 blocks) followed by
#          `## Burst: Pass-40 current fix burst (2026-05-16)` with all 9 blocks.
# Expected: hook exits 0 (pass) because only the latest h2 entry is validated.
#
# RED GATE PHASE: test skips because validate-burst-log.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-burst-log.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-burst-log/pass-prior-entry-incomplete"
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-prior-incomplete","tool_input":{"file_path":".factory/cycles/v1.0-feature-engine-discipline-pass-1/burst-log.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-5: old incomplete entry + complete current entry => hook passes (no retroactive sweep)
# Traces to BC-5.39.004 invariant 4 + D-443(e)(ii) scoping policy
# ---------------------------------------------------------------------------

@test "AC-5 PASS: hook emits Continue when latest entry is complete despite prior incomplete entry" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_burst_log_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: validator scopes to latest h2 only; prior incomplete entry is not re-validated
  [ "$status" -eq 0 ]

  # No blocking_plugins= for a correct scoping pass
  [[ "$output" != *"blocking_plugins="* ]]
}
