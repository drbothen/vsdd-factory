#!/usr/bin/env bats
# fail-stale-story-index.bats — AC-3: hook blocks when ARCH-INDEX cites stale STORY-INDEX version
#
# Traces to:
#   BC-5.39.003 postcondition 2 (stale ARCH-INDEX STORY-INDEX cite => BlockWithFix)
# Canonical test vector: "Stale STORY-INDEX cite" row in BC-5.39.003 Canonical Test Vectors
#   Fixture: ARCH-INDEX body cites STORY-INDEX v3.28; live STORY-INDEX frontmatter version is v3.31
# D-NNN closure: D-405(c)
#
# RED GATE PHASE: test skips because validate-index-cite-refresh.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-index-cite-refresh.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-index-cite-refresh/fail-stale-story-index"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_setup_fixture() {
  cp -r "$FIXTURE_SRC/factory" "$WORK/.factory"
}

_write_registry() {
  cat > "$WORK/hooks-registry.toml" << 'TOML'
schema_version = 2

[[hooks]]
name = "validate-index-cite-refresh"
event = "PostToolUse"
tool = "Write|Edit"
file_pattern = "ARCH-INDEX.md"
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
  printf '{"event_name":"PostToolUse","tool_name":"Write","session_id":"fail-story-index","tool_input":{"file_path":".factory/specs/architecture/ARCH-INDEX.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-3: stale STORY-INDEX cite => dispatcher exits 2 (block)
# ---------------------------------------------------------------------------

@test "AC-3 FAIL: hook blocks when ARCH-INDEX body cites stale STORY-INDEX version (v3.28 vs live v3.31)" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_arch_index_envelope)"
  run bash -c "printf '%s' \"$envelope\" | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal emitted
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook
  [[ "$output" == *"blocking_plugins=validate-index-cite-refresh"* ]]

  # block message must name STORY-INDEX
  [[ "$output" == *"STORY-INDEX"* ]]
}
