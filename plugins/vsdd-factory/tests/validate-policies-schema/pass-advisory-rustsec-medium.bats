#!/usr/bin/env bats
# pass-advisory-rustsec-medium.bats — AC-18 (Part C): dispatch package with MEDIUM advisory in cache
#                                      produces Continue + log_warn (not a block)
#
# Traces to:
#   BC-5.39.008 postcondition 13 (MEDIUM advisory => host::log_warn + Continue; no block)
#   BC-5.39.008 EC-021 (two MEDIUM advisories; each gets log_warn; Continue)
#
# Fixture: td-99-dispatch.md with old_crate = "1.2.3" +
#          cargo-audit-cache.json with MEDIUM advisory for old_crate.
# Expected: hook exits 0 (Continue); MEDIUM advisory does not block.
#
# RED GATE PHASE: test skips because validate-policies-schema.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-policies-schema.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-policies-schema/pass-advisory-rustsec-medium"
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"pass-advisory-rustsec-medium","tool_input":{"file_path":".factory/cycles/v1.0-brownfield-backfill/td-99-dispatch.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-18: Part C MEDIUM advisory => Continue (not block)
# Traces to BC-5.39.008 postcondition 13
# ---------------------------------------------------------------------------

@test "test_BC_5_39_008_accepts_medium_rustsec_advisory_exits_0" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_dispatch_package_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 0 ]
}

@test "test_BC_5_39_008_accepts_medium_rustsec_advisory_no_block" {
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
