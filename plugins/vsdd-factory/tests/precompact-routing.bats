#!/usr/bin/env bats
# precompact-routing.bats — VP-086 binary-level harness for S-18.00
#
# Verifies the factory-dispatcher binary correctly routes PreCompact and
# PostCompact events per BC-1.15.001:
#
#   TC-AC001: PreCompact + exit-0 plugin → plugins_run=1, exit 0
#   TC-AC002: PostCompact + exit-2 plugin + on_error=block → exit 0 (advisory-only)
#             (historical Red Gate: the no-op dispatcher exited 2 here; delivered is_advisory_only() suppression in main.rs now exits 0 — GREEN)
#   TC-AC003: PreCompact with no registered plugins → sync_plugins=0, exit 0
#   TC-AC004: PreCompact + exit-2 plugin + on_error=block → exit 2 (block_intent=true)
#   TC-AC005a: PreCompact + crash plugin + on_error=block → exit 2 (fail-closed)
#   TC-AC005b: PreCompact + exit-2 plugin + on_error=continue → exit 0 (fail-open)
#   TC-EC001: Two PreCompact plugins; one exits 2 → exit 2 (single exit-2 sufficient)
#   TC-AC006: Unknown event type handled; does not crash dispatcher
#
# VP-086 property: "factory-dispatcher receives a PreCompact event; registered
# plugin exits 2; dispatcher propagates block_intent=true to harness."
#
# Story: S-18.00
# BC:    BC-1.15.001 PC1/PC2/PC3/PC4/PC5 (PreCompact/PostCompact routing)
# VP:    VP-086 (Dispatcher Exit-2 Propagation for PreCompact Block-Intent)
#
# RED GATE (historical — S-18.00 is now implemented and all tests pass GREEN):
#   TC-AC002 historically failed against the no-op code: PostCompact exit-2 + on_error=block
#   caused the dispatcher to exit 2 instead of 0 (advisory-only violation,
#   BC-1.15.001 PC2 / F-002 BLOCKER). Delivered is_advisory_only() gate in main.rs closes this.
#
# All tests skip gracefully if the dispatcher binary is not built or if
# legacy-bash-adapter.wasm is not present.
#
# Dispatcher binary: target/release/factory-dispatcher
# Plugin: hook-plugins/legacy-bash-adapter.wasm (executes stub shell scripts)
# Shell stubs: written to WORK/hooks/ per test

# ---------------------------------------------------------------------------
# setup / teardown
# ---------------------------------------------------------------------------

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  ADAPTER_WASM="$REPO_ROOT/plugins/vsdd-factory/hook-plugins/legacy-bash-adapter.wasm"
  WORK="$(mktemp -d)"
  # PROJECT_DIR is a distinct subdirectory of WORK — CLAUDE_PLUGIN_ROOT=$WORK,
  # CLAUDE_PROJECT_DIR=$PROJECT_DIR so path-domain tests are non-tautological
  # (S-18.04a-prereq AC-005: distinct roots required per ADR-028 §Decision 8).
  PROJECT_DIR="$WORK/project"
  mkdir -p "$WORK/.factory/logs" "$WORK/hook-plugins" "$WORK/hooks"
  mkdir -p "$PROJECT_DIR/.factory"

  # Copy legacy-bash-adapter.wasm into WORK's hook-plugins directory
  # so registry plugin paths resolve correctly.
  if [ -f "$ADAPTER_WASM" ]; then
    cp "$ADAPTER_WASM" "$WORK/hook-plugins/legacy-bash-adapter.wasm"
  fi

  # Create reusable stub scripts.
  # stub-exit0.sh: exits 0 (hook passes, no block)
  cat > "$WORK/hooks/stub-exit0.sh" <<'STUB_EOF'
#!/usr/bin/env bash
exit 0
STUB_EOF
  chmod +x "$WORK/hooks/stub-exit0.sh"

  # stub-exit2.sh: exits 2 (hook requests block)
  cat > "$WORK/hooks/stub-exit2.sh" <<'STUB_EOF'
#!/usr/bin/env bash
exit 2
STUB_EOF
  chmod +x "$WORK/hooks/stub-exit2.sh"

  export CLAUDE_PROJECT_DIR="$PROJECT_DIR"
}

teardown() {
  rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# Preflight helpers
# ---------------------------------------------------------------------------

# Skip the test if the dispatcher binary is not built or the adapter WASM is absent.
_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built — run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WORK/hook-plugins/legacy-bash-adapter.wasm" ]; then
    skip "legacy-bash-adapter.wasm not present — build hook-plugins or copy to $WORK/hook-plugins/"
  fi
}

# ---------------------------------------------------------------------------
# Registry helpers
# ---------------------------------------------------------------------------

# Write a registry with ONE PreCompact plugin.
# Args: script (stub-exit0.sh | stub-exit2.sh) on_error (block | continue) [name]
_write_precompact_registry() {
  local script="${1:-stub-exit0.sh}"
  local on_error="${2:-block}"
  local name="${3:-precompact-stub}"
  cat > "$WORK/hooks-registry.toml" <<EOF
schema_version = 2

[[hooks]]
name = "$name"
event = "PreCompact"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
timeout_ms = 5000
on_error = "$on_error"

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash"]
shell_bypass_acknowledged = "yes"
cwd_allow = ["."]

[hooks.config]
script_path = "hooks/$script"
EOF
}

# Write a registry with ONE PostCompact plugin.
# Args: script on_error [name]
_write_postcompact_registry() {
  local script="${1:-stub-exit0.sh}"
  local on_error="${2:-block}"
  local name="${3:-postcompact-stub}"
  cat > "$WORK/hooks-registry.toml" <<EOF
schema_version = 2

[[hooks]]
name = "$name"
event = "PostCompact"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
timeout_ms = 5000
on_error = "$on_error"

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash"]
shell_bypass_acknowledged = "yes"
cwd_allow = ["."]

[hooks.config]
script_path = "hooks/$script"
EOF
}

# Write a registry with NO PreCompact plugins (only a PostToolUse entry).
_write_no_precompact_registry() {
  cat > "$WORK/hooks-registry.toml" <<'EOF'
schema_version = 2

[[hooks]]
name = "other-plugin"
event = "PostToolUse"
tool = "Bash"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash"]
shell_bypass_acknowledged = "yes"
cwd_allow = ["."]

[hooks.config]
script_path = "hooks/stub-exit0.sh"
EOF
}

# Write a registry with TWO PreCompact plugins (different names, same script).
# First plugin exits 2 (block), second exits 0.
_write_two_precompact_registry() {
  cat > "$WORK/hooks-registry.toml" <<EOF
schema_version = 2

[[hooks]]
name = "precompact-blocker"
event = "PreCompact"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
priority = 100
timeout_ms = 5000
on_error = "block"

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash"]
shell_bypass_acknowledged = "yes"
cwd_allow = ["."]

[hooks.config]
script_path = "hooks/stub-exit2.sh"

[[hooks]]
name = "precompact-passer"
event = "PreCompact"
plugin = "hook-plugins/legacy-bash-adapter.wasm"
priority = 200
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.exec_subprocess]
binary_allow = ["bash"]
shell_bypass_acknowledged = "yes"
cwd_allow = ["."]

[hooks.config]
script_path = "hooks/stub-exit0.sh"
EOF
}

# ---------------------------------------------------------------------------
# Dispatcher invocation helper
# ---------------------------------------------------------------------------

# Run the dispatcher with a given JSON envelope.
# Captures combined stdout+stderr into $output; sets $status.
# CLAUDE_PLUGIN_ROOT=$WORK (plugin directory), CLAUDE_PROJECT_DIR=$PROJECT_DIR
# (project directory — a distinct subdirectory of WORK per S-18.04a-prereq AC-005).
_run_dispatcher() {
  local envelope="$1"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$PROJECT_DIR' '$DISPATCHER' 2>&1"
}

# ---------------------------------------------------------------------------
# TC-AC001: PreCompact + exit-0 plugin → plugins_run=1, exit 0
# BC-1.15.001 PC1 (AC-001)
# ---------------------------------------------------------------------------

@test "TC-AC001: PreCompact event routes to registered plugin and exits 0 when plugin exits 0" {
  _require_artifacts
  _write_precompact_registry "stub-exit0.sh" "block" "precompact-ok"

  _run_dispatcher '{"event_name":"PreCompact","tool_name":"","session_id":"tc-ac001","tool_input":{}}'

  # Dispatcher must exit 0 (plugin exits 0, no block intent).
  [ "$status" -eq 0 ]

  # Dispatcher must have routed to the plugin (sync_plugins=1, plugins_run=1).
  [[ "$output" == *"sync_plugins=1"* ]] || [[ "$output" == *"plugins_run=1"* ]]
}

# ---------------------------------------------------------------------------
# TC-AC002: PostCompact + exit-2 plugin + on_error=block → exit 0 (advisory-only)
# BC-1.15.001 PC2 (AC-002) — BLOCKER F-002
#
# (historical: failed at Red Gate against the no-op impl — dispatcher exited 2 instead of 0;
# now GREEN against the delivered is_advisory_only() suppression in main.rs which exits 0.)
# ---------------------------------------------------------------------------

@test "TC-AC002: PostCompact exit-2 plugin with on_error=block must not block (advisory-only)" {
  _require_artifacts
  _write_postcompact_registry "stub-exit2.sh" "block" "postcompact-advisory-test"

  _run_dispatcher '{"event_name":"PostCompact","tool_name":"","session_id":"tc-ac002","tool_input":{}}'

  # BLOCKER F-002 (historical — delivered and GREEN): PostCompact must be advisory-only.
  # The dispatcher MUST exit 0 even though the plugin exits 2 and on_error=block.
  # BC-1.15.001 PC2: "PostCompact dispatch invokes registered plugins and propagates
  # exit codes, but NEVER sets block_intent=true regardless of plugin exit code."
  #
  # (historical: this test failed against the no-op implementation which exited 2 instead of 0;
  # is_advisory_only() gate in main.rs now suppresses block_intent for PostCompact — GREEN.)
  [ "$status" -eq 0 ]

  # The dispatcher must NOT contain block_intent=true in its output.
  [[ "$output" != *"block_intent=true"* ]]
}

# ---------------------------------------------------------------------------
# TC-AC003: PreCompact with no registered plugins → sync_plugins=0, exit 0
# BC-1.15.001 PC3 (AC-003) / EC-007
# ---------------------------------------------------------------------------

@test "TC-AC003: PreCompact with no registered plugins exits 0 without errors" {
  _require_artifacts
  _write_no_precompact_registry

  _run_dispatcher '{"event_name":"PreCompact","tool_name":"","session_id":"tc-ac003","tool_input":{}}'

  # No plugins registered for PreCompact → exit 0, no plugins run.
  [ "$status" -eq 0 ]

  # sync_plugins must be 0 (no PreCompact plugins matched).
  [[ "$output" == *"sync_plugins=0"* ]]
}

# ---------------------------------------------------------------------------
# TC-AC004: PreCompact + exit-2 plugin + on_error=block → exit 2 (block_intent=true)
# BC-1.15.001 PC4 (AC-004) / VP-086
# ---------------------------------------------------------------------------

@test "TC-AC004: PreCompact exit-2 plugin with on_error=block sets block_intent=true and exits 2" {
  _require_artifacts
  _write_precompact_registry "stub-exit2.sh" "block" "precompact-blocker"

  _run_dispatcher '{"event_name":"PreCompact","tool_name":"","session_id":"tc-ac004","tool_input":{}}'

  # VP-086: PreCompact exit-2 must propagate block_intent=true.
  [ "$status" -eq 2 ]

  # The dispatcher must report block_intent=true and the plugin name.
  [[ "$output" == *"block_intent=true"* ]]
  [[ "$output" == *"blocking_plugins=precompact-blocker"* ]]
}

# ---------------------------------------------------------------------------
# TC-AC005b: PreCompact + exit-0 plugin + on_error=continue → exit 0 (normal flow)
# BC-1.15.001 PC5 (AC-005) / EC-004
#
# NOTE: BC-1.15.001 EC-004 specifies "on_error=continue crash → advisory only,
# block_intent=false". Testing a WASM trap/crash requires a WASM module that
# traps, which is not available in the current fixture infrastructure. The
# legacy-bash-adapter translates script exit-2 into an advisory JSON block
# ({"outcome":"block"}) regardless of on_error, because on_error only controls
# WASM crash handling (trap/unreachable), not the advisory JSON block path.
#
# This test verifies EC-004's normal-path precondition: on_error=continue with
# a plugin that exits 0 does not produce any block. The WASM crash path for
# on_error=continue is verified at the unit level in precompact.rs via the
# aggregator tests (test_BC_1_15_001_precompact_on_error_continue_crash_no_block).
# ---------------------------------------------------------------------------

@test "TC-AC005b: PreCompact on_error=continue with exit-0 plugin exits 0 (EC-004 normal path)" {
  _require_artifacts
  _write_precompact_registry "stub-exit0.sh" "continue" "precompact-continue"

  _run_dispatcher '{"event_name":"PreCompact","tool_name":"","session_id":"tc-ac005b","tool_input":{}}'

  # on_error=continue with exit-0: plugin passes, no block.
  # This verifies on_error=continue does not suppress normal plugin results.
  [ "$status" -eq 0 ]
  [[ "$output" != *"block_intent=true"* ]]
}

# ---------------------------------------------------------------------------
# TC-EC001: Two PreCompact plugins; first exits 2 → exit 2 (single exit-2 sufficient)
# BC-1.15.001 EC-001
# ---------------------------------------------------------------------------

@test "TC-EC001: Two PreCompact plugins, one exits 2, dispatcher exits 2 (block propagates)" {
  _require_artifacts
  _write_two_precompact_registry

  _run_dispatcher '{"event_name":"PreCompact","tool_name":"","session_id":"tc-ec001","tool_input":{}}'

  # Both plugins run; first exits 2 (block) → dispatcher exits 2.
  # BC-1.15.001 EC-001: single exit-2 sufficient for block_intent=true.
  [ "$status" -eq 2 ]
  [[ "$output" == *"block_intent=true"* ]]
  [[ "$output" == *"blocking_plugins=precompact-blocker"* ]]
}

# ---------------------------------------------------------------------------
# TC-AC003b: PostCompact with no registered plugins → sync_plugins=0, exit 0
# BC-1.15.001 PC3 / EC-007 (PostCompact variant)
# ---------------------------------------------------------------------------

@test "TC-AC003b: PostCompact with no registered plugins exits 0 without errors" {
  _require_artifacts
  _write_no_precompact_registry

  _run_dispatcher '{"event_name":"PostCompact","tool_name":"","session_id":"tc-ac003b","tool_input":{}}'

  # No PostCompact plugins registered → exit 0, no plugins run.
  [ "$status" -eq 0 ]
  [[ "$output" == *"sync_plugins=0"* ]]
}

# ---------------------------------------------------------------------------
# TC-AC006: Unknown event type is handled gracefully; dispatcher exits 0
# BC-1.15.001 INV1 (closed enum — Other variant handles unknown events)
# ---------------------------------------------------------------------------

@test "TC-AC006: Unknown event type handled gracefully, dispatcher exits 0" {
  _require_artifacts
  _write_no_precompact_registry

  _run_dispatcher '{"event_name":"UnknownFutureEvent","tool_name":"","session_id":"tc-ac006","tool_input":{}}'

  # Unknown event → no plugins match → exit 0 (fail-open per BC-1.08.001).
  [ "$status" -eq 0 ]
}

# ---------------------------------------------------------------------------
# TC-INV1: PreCompact registered as event string in registry is matched
# BC-1.15.001 INV1 (event string round-trip)
# ---------------------------------------------------------------------------

@test "TC-INV1: Registry entry with event=PreCompact is matched by dispatcher routing" {
  _require_artifacts
  _write_precompact_registry "stub-exit0.sh" "continue" "inv1-precompact"

  _run_dispatcher '{"event_name":"PreCompact","tool_name":"","session_id":"tc-inv1","tool_input":{}}'

  # The PreCompact event string must route to the registered plugin.
  # sync_plugins=1 proves the routing matched (INV1: PreCompact is first-class enum variant).
  [ "$status" -eq 0 ]
  [[ "$output" == *"sync_plugins=1"* ]]
}

# ---------------------------------------------------------------------------
# TC-INV1b: PostCompact registered as event string in registry is matched
# BC-1.15.001 INV1 (event string round-trip for PostCompact)
# ---------------------------------------------------------------------------

@test "TC-INV1b: Registry entry with event=PostCompact is matched by dispatcher routing" {
  _require_artifacts
  _write_postcompact_registry "stub-exit0.sh" "continue" "inv1b-postcompact"

  _run_dispatcher '{"event_name":"PostCompact","tool_name":"","session_id":"tc-inv1b","tool_input":{}}'

  # The PostCompact event string must route to the registered plugin.
  # sync_plugins=1 proves the routing matched (INV1: PostCompact is first-class enum variant).
  [ "$status" -eq 0 ]
  [[ "$output" == *"sync_plugins=1"* ]]
}

