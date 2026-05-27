#!/usr/bin/env bats
# fail-advisory-rustsec-high.bats — AC-17 (Part C): dispatch package file with HIGH RUSTSEC advisory
#                                    in cache produces block citing RUSTSEC ID and crate name
#
# Traces to:
#   BC-5.39.008 postcondition 13 (HIGH advisory => BlockWithFix citing RUSTSEC ID + crate name)
#   BC-5.39.008 EC-017 (td-99-dispatch.md with serde_yaml = "0.9.34"; cache has RUSTSEC-2025-0068 HIGH)
#
# Fixture: td-99-dispatch.md recommending serde_yaml = "0.9.34" +
#          cargo-audit-cache.json with RUSTSEC-2025-0068 severity HIGH for serde_yaml.
# Expected: hook exits 2 (block); block_reason cites RUSTSEC-2025-0068.
#
# RED GATE PHASE: test skips because validate-policies-schema.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-policies-schema.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-policies-schema/fail-advisory-rustsec-high"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
  mkdir -p "$WORK/.factory/hooks"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  # Copy dispatch package to the expected td-*-dispatch.md path
  cp "$FIXTURE_SRC/td-99-dispatch.md" "$WORK/td-99-dispatch.md"
  # Copy cargo-audit-cache.json to .factory/hooks/
  cp -r "$FIXTURE_SRC/factory/." "$WORK/.factory/"
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
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"fail-advisory-rustsec-high","tool_input":{"file_path":".factory/cycles/v1.0-brownfield-backfill/td-99-dispatch.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-17: Part C HIGH advisory => dispatcher exits 2 (block)
# Traces to BC-5.39.008 postcondition 13
# ---------------------------------------------------------------------------

@test "test_BC_5_39_008_rejects_high_rustsec_advisory_exits_2" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"
  # Place the dispatch package at the path referenced in the envelope
  mkdir -p "$WORK/.factory/cycles/v1.0-brownfield-backfill"
  cp "$FIXTURE_SRC/td-99-dispatch.md" "$WORK/.factory/cycles/v1.0-brownfield-backfill/td-99-dispatch.md"

  local envelope
  envelope="$(_dispatch_package_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
  [[ "$output" == *"blocking_plugins=validate-policies-schema"* ]]
}

@test "test_BC_5_39_008_rejects_high_rustsec_advisory_block_reason_cites_rustsec_id" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"
  mkdir -p "$WORK/.factory/cycles/v1.0-brownfield-backfill"
  cp "$FIXTURE_SRC/td-99-dispatch.md" "$WORK/.factory/cycles/v1.0-brownfield-backfill/td-99-dispatch.md"

  local envelope
  envelope="$(_dispatch_package_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
  [[ "$output" == *"RUSTSEC-2025-0068"* ]]
}

@test "test_BC_5_39_008_rejects_high_rustsec_advisory_block_reason_names_crate" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"
  mkdir -p "$WORK/.factory/cycles/v1.0-brownfield-backfill"
  cp "$FIXTURE_SRC/td-99-dispatch.md" "$WORK/.factory/cycles/v1.0-brownfield-backfill/td-99-dispatch.md"

  local envelope
  envelope="$(_dispatch_package_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  [ "$status" -eq 2 ]
  [[ "$output" == *"serde_yaml"* ]]
}
