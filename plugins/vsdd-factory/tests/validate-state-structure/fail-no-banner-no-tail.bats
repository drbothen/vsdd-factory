#!/usr/bin/env bats
# fail-no-banner-no-tail.bats — F-P1-002: EC-014 — truly empty STATE.md fires BOTH
#                                banner-wc AND trajectory-tail violations.
#
# Traces to:
#   BC-5.39.005 EC-014: empty STATE.md must produce both banner and tail violations
#   F-P1-002: fire banner violation on absent banner
#   BC-5.39.005 postcondition 4: absent trajectory tail is a violation
#
# Fixture: near-empty STATE.md with no HTML comment and no trajectory tail.
# Expected: hook exits 2 (block) enumerating both violations.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-state-structure.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-state-structure/fail-no-banner-no-tail"
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
    skip "validate-state-structure.wasm not built"
  fi
}

_state_md_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-no-banner-no-tail","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# EC-014: empty STATE.md fires both banner and tail violations
# ---------------------------------------------------------------------------

@test "F-P1-002 / EC-014 FAIL: empty STATE.md blocks with both banner-wc and tail violations" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal emitted
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook
  [[ "$output" == *"blocking_plugins=validate-state-structure"* ]]
}

@test "F-P1-002 / EC-014 FAIL: block message enumerates both banner-wc and trajectory-tail violations" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]

  # Must mention both banner absence and trajectory tail absence
  [[ "$output" == *"SIZE BUDGET"* ]] || [[ "$output" == *"no SIZE BUDGET"* ]]
  [[ "$output" == *"trajectory"* ]]

  # Exact violation count: banner-wc absent + dual-margin absent + tail absent = 3
  [[ "$output" == *"3 violation(s)"* ]] || fail "expected 3 violations (banner-wc absent + dual-margin absent + tail absent); got: $output"
}
