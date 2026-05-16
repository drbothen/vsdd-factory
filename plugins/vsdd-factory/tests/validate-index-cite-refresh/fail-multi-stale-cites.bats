#!/usr/bin/env bats
# fail-multi-stale-cites.bats — EC-008: hook accumulates all violations into a single BlockWithFix
#
# Traces to:
#   BC-5.39.003 postcondition 2 (stale ARCH-INDEX cites => BlockWithFix)
#   BC-5.39.003 Canonical Test Vectors: "Multiple stale cites" row
#   BC-5.39.003 invariant 1 (single HookResult::BlockWithFix even for multiple violations)
#   Story S-15.07 §Edge Cases EC-008
# Closes: F-S15.07-LOCAL-P1-004 (adversary pass-1 finding)
#
# Fixture: ARCH-INDEX body cites BC-INDEX v1.05 (stale; live v2.24)
#          AND STORY-INDEX v3.28 (stale; live v3.32) simultaneously.
# Expected: hook exits 2 (block) and block_reason names BOTH stale indexes
#           plus a violation count of 2.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-index-cite-refresh.wasm"
  FIXTURE_SRC="${BATS_TEST_DIRNAME}/../fixtures/validate-index-cite-refresh/fail-multi-stale-cites"
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
  printf '{"event_name":"PostToolUse","tool_name":"Write","session_id":"fail-multi-stale-cites","tool_input":{"file_path":".factory/specs/architecture/ARCH-INDEX.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}'
}

# ---------------------------------------------------------------------------
# EC-008: multi-stale-cites => dispatcher exits 2 AND names both indexes
# Traces to BC-5.39.003 Canonical Test Vector "Multiple stale cites"
# ---------------------------------------------------------------------------

@test "EC-008: hook blocks when ARCH-INDEX body cites both BC-INDEX and STORY-INDEX stale" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_arch_index_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal emitted for multi-violation input
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook
  [[ "$output" == *"blocking_plugins=validate-index-cite-refresh"* ]]
}

# ---------------------------------------------------------------------------
# EC-008: block message names BOTH stale indexes and BOTH cited versions
# Asserts on substring containment only (stable across F-001 format fix)
# ---------------------------------------------------------------------------

@test "EC-008: block message names BC-INDEX with stale cite v1.05 and STORY-INDEX with stale cite v3.28" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_arch_index_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal
  [ "$status" -eq 2 ]

  # Both stale index names must appear in block_reason
  [[ "$output" == *"BC-INDEX"* ]]
  [[ "$output" == *"STORY-INDEX"* ]]

  # Both cited (stale) versions must appear in body-literal form.
  # Post-F-P2-001: BC-INDEX cite is "v1.05" in the fixture; hook preserves body-literal form => "v1.05".
  # Post-F-P2-001: STORY-INDEX cite is "v3.28" in the fixture; preserves body-literal form => "v3.28".
  [[ "$output" == *"BC-INDEX v1.05"* ]]
  [[ "$output" == *"v3.28"* ]]
}

# ---------------------------------------------------------------------------
# EC-008: block message includes violation count >= 2
# BC-5.39.003 invariant 1: single BlockWithFix enumerates all violations
# ---------------------------------------------------------------------------

@test "EC-008: block message reports violation count of 2 stale cites" {
  _require_artifacts
  _setup_fixture
  _write_registry
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_arch_index_envelope)"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: block signal
  [ "$status" -eq 2 ]

  # Count message must name "2 stale cite" (tolerant of "2 stale cite(s) found" or "2 stale cites found")
  [[ "$output" == *"2 stale cite"* ]]
}
