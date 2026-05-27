#!/usr/bin/env bats
# pass-cache-invalid-json.bats — AC-20 (Part C): cargo-audit-cache.json present but invalid JSON
#                                 produces Continue + log_warn; never blocks (fail-open)
#
# Traces to:
#   BC-5.39.008 postcondition 12 (cache present but invalid JSON => Continue + parse-error log_warn)
#   BC-5.39.008 invariant 9(c) (fail-open: cache parse error => Continue)
#   BC-5.39.008 EC-019 (cargo-audit-cache.json invalid JSON)
#
# Fixture: td-99-dispatch.md + cargo-audit-cache.json containing malformed JSON ("{ this is not valid").
# Expected: hook exits 0 (Continue); invalid JSON does not block.
#
# RED GATE PHASE: test skips because validate-policies-schema.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-policies-schema.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-policies-schema/pass-cache-invalid-json"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
  mkdir -p "$WORK/.factory/hooks"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  cp -r "$FIXTURE_SRC/factory/." "$WORK/.factory/"
  mkdir -p "$WORK/.factory/cycles/v1.0-brownfield-backfill"
  cp "$FIXTURE_SRC/td-99-dispatch.md" "$WORK/.factory/cycles/v1.0-brownfield-backfill/td-99-dispatch.md"
}

_write_registry() {
  cat > "$WORK/hooks-registry.toml" << 'TOML'
schema_version = 2

[[hooks]]
name = "validate-policies-schema"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-policies-schema.wasm"
priority = 157
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [".factory", "plugins/vsdd-factory"]
TOML
}

_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built -- run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WASM_PLUGIN" ]; then
    skip "validate-policies-schema.wasm not built -- implement T-6 through T-11 of S-15.15"
  fi
}

_dispatch_package_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-cache-invalid-json","tool_input":{"file_path":".factory/cycles/v1.0-brownfield-backfill/td-99-dispatch.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-20: Part C invalid JSON cache => Continue (fail-open)
# Traces to BC-5.39.008 postcondition 12
# ---------------------------------------------------------------------------

@test "test_BC_5_39_008_accepts_invalid_json_cache_exits_0" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_dispatch_package_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
}

@test "test_BC_5_39_008_accepts_invalid_json_cache_no_block" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_dispatch_package_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
  [[ "$output" != *"blocking_plugins=validate-policies-schema"* ]]
}
