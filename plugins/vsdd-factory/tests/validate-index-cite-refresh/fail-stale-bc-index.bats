#!/usr/bin/env bats
# fail-stale-bc-index.bats — AC-1 + AC-10: hook blocks when ARCH-INDEX cites stale BC-INDEX version
#
# Traces to:
#   BC-5.39.003 postcondition 2 (stale ARCH-INDEX cite => BlockWithFix)
#   BC-5.39.003 EC-005 (BC-INDEX v1.05 cited, live is v2.24)
# Canonical test vector: "Stale BC-INDEX cite" row in BC-5.39.003 Canonical Test Vectors
# D-NNN closure: D-405(c)
#
# RED GATE PHASE: test skips because validate-index-cite-refresh.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-index-cite-refresh.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-index-cite-refresh/fail-stale-bc-index"
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
name = "validate-index-cite-refresh"
event = "PostToolUse"
tool = "Write|Edit"
plugin = "hook-plugins/validate-index-cite-refresh.wasm"
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [
  ".factory/specs/behavioral-contracts/BC-INDEX.md",
  ".factory/specs/verification-properties/VP-INDEX.md",
  ".factory/stories/STORY-INDEX.md",
  ".factory/specs/architecture/ARCH-INDEX.md",
  ".factory/STATE.md",
  ".factory/cycles/v1.0-brownfield-backfill/INDEX.md",
]
TOML
}

_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built -- run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WASM_PLUGIN" ]; then
    skip "validate-index-cite-refresh.wasm not built -- implement T-4 through T-7 of S-15.07"
  fi
}

_arch_index_envelope() {
  printf '{"event_name":"PostToolUse","tool_name":"Write","session_id":"fail-bc-index","tool_input":{"file_path":".factory/specs/architecture/ARCH-INDEX.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-1: stale BC-INDEX cite => dispatcher exits 2 (block)
# ---------------------------------------------------------------------------

@test "AC-1 FAIL: hook blocks when ARCH-INDEX body cites stale BC-INDEX version (v1.05 vs live v2.24)" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_arch_index_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal emitted
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook
  [[ "$output" == *"blocking_plugins=validate-index-cite-refresh"* ]]
}

# ---------------------------------------------------------------------------
# AC-10: block message names the stale cite, cited version, and live version
# ---------------------------------------------------------------------------

@test "AC-10 FAIL: block message names BC-INDEX, stale cite v1.05, and live version v2.24" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_arch_index_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal
  [ "$status" -eq 2 ]

  # block_reason must name BC-INDEX and the two versions (TD #71 actionable message requirement).
  # Post-F-P2-001: fixture cites BC-INDEX v1.05; hook preserves body-literal form => "v1.05".
  # Live side uses integer-rendered canonical form => "v2.24".
  [[ "$output" == *"BC-INDEX"* ]]
  [[ "$output" == *"v1.05"* ]]
  [[ "$output" == *"v2.24"* ]]
}
