#!/usr/bin/env bats
# fail-open-missing-index.bats — AC-7: hook emits Continue (fail-open) when BC-INDEX.md is absent
#
# Traces to:
#   BC-5.39.003 postcondition 4 (host::read_file error => Continue + log_warn, not Block)
#   BC-5.39.003 invariant 5 (all read failures are fail-open)
#   BC-5.39.003 EC-004 (any index file unreadable => Continue)
# Canonical test vector: "BC-INDEX unreadable" row in BC-5.39.003 Canonical Test Vectors
#   Fixture: BC-INDEX.md is absent from fixture tree; ARCH-INDEX body cites BC-INDEX v1.05
#            Hook must emit Continue (not BlockWithFix) because the live version is unreadable
#
# This test validates the fail-open invariant: read failures must NOT cause blocking.
# A hook that blocks on read failure would be unsafe in production (network partitions, etc.)
#
# RED GATE PHASE: test skips because validate-index-cite-refresh.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-index-cite-refresh.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-index-cite-refresh/fail-open-missing-index"
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
tool = "Edit|Write"
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
  printf '{"event_name":"PostToolUse","tool_name":"Write","session_id":"fail-open","tool_input":{"file_path":".factory/specs/architecture/ARCH-INDEX.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# AC-7: BC-INDEX.md absent => hook emits Continue (fail-open), not Block
# ---------------------------------------------------------------------------

@test "AC-7 PASS (fail-open): hook emits Continue when BC-INDEX.md is absent from project tree" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  # Verify BC-INDEX.md is truly absent from the fixture (defensive assertion)
  [ ! -f "$WORK/.factory/specs/behavioral-contracts/BC-INDEX.md" ]

  local envelope
  envelope="$(_arch_index_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: fail-open means no block even though BC-INDEX is unreadable
  [ "$status" -eq 0 ]

  # No blocking_plugins= for a fail-open pass
  [[ "$output" != *"blocking_plugins="* ]]
}
