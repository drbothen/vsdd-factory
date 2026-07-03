#!/usr/bin/env bats
# integration-production-registry.bats — AC-19: production registry capability shape regression test
#
# Traces to:
#   BC-5.39.006 invariant 9 (fail-open on host::read_file error must NOT be triggered
#                             by a production registry misconfiguration)
#   S-15.11 F-P2-001 lesson (preemptive application — included from day 1, not added in fix-burst)
#   Dispatch package §Hard Constraint 1 (bare paths, no "**" glob in path_allow)
#
# AC-18 note: This test also validates AC-18 (hooks-registry.toml entry present with correct
# tool = "Edit|Write" and no file_pattern field) as part of the production registry extraction.
# AC-19 is covered by the full scenario test below.
#
# Two scenarios:
#   A. Production-shape registry + valid STATE.md => exit 0 (Continue, not fail-open crash)
#      Proves host::read_file succeeds (capability grants access).
#   B. Production-shape registry + invalid STATE.md (forbidden meta-commentary) => exit 2 (Block)
#      Proves hook actually evaluates content (distinguishes real Continue from fail-open Continue).
#
# If path_allow uses "**" glob (regression to S-15.07/S-15.11 bug): canonicalize() fails =>
# path_allowed() returns false => host::read_file returns CapabilityDenied => hook fail-opens =>
# Scenario B exits 0 instead of 2. Test fails.
#
# RED GATE PHASE: test skips if validate-dispatch-advance.wasm is not yet compiled.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  WASM_PLUGIN="$PLUGIN_ROOT/hook-plugins/validate-dispatch-advance.wasm"
  PRODUCTION_REGISTRY="$REPO_ROOT/plugins/vsdd-factory/hooks-registry.toml"
  FIXTURE_VALID="${BATS_TEST_DIRNAME}/../fixtures/validate-dispatch-advance/pass-all-valid-state"
  FIXTURE_INVALID="${BATS_TEST_DIRNAME}/../fixtures/validate-dispatch-advance/fail-meta-commentary-watch"
  WORK="$(mktemp -d)"
  mkdir -p "$WORK/hook-plugins"
  mkdir -p "$WORK/.factory/logs"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && find "$WORK" -type f -delete && find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && rmdir "$WORK" 2>/dev/null || true
}

_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "dispatcher binary not built -- run: cargo build --release -p factory-dispatcher"
  fi
  if [ ! -f "$WASM_PLUGIN" ]; then
    skip "validate-dispatch-advance.wasm not built -- implement T-5 through T-7 of S-15.14"
  fi
}

# Extract the production path_allow for validate-dispatch-advance from hooks-registry.toml.
# Also validates AC-19: confirms tool = "Edit|Write" (not "Write|Edit") and no file_pattern.
_write_production_registry() {
  # AC-19 check 1: hook entry must exist with correct tool form
  if ! grep -q 'name = "validate-dispatch-advance"' "$PRODUCTION_REGISTRY"; then
    echo "FAIL: validate-dispatch-advance entry not found in production registry at $PRODUCTION_REGISTRY" >&2
    return 1
  fi

  # AC-19 check 2: tool = "Edit|Write" (canonical form per Q5/Q6 — NOT "Write|Edit")
  local tool_line
  tool_line=$(awk '/^name = "validate-dispatch-advance"$/{found=1} found && /^tool =/{print; exit}' "$PRODUCTION_REGISTRY")
  if ! echo "$tool_line" | grep -q 'tool = "Edit|Write"'; then
    echo "FAIL: production registry uses wrong tool form: $tool_line" >&2
    echo "FAIL: must be tool = \"Edit|Write\" (canonical Q5 form)" >&2
    return 1
  fi

  # AC-19 check 3: no file_pattern field (path-component-strict is in-plugin, not registry)
  if awk '/^name = "validate-dispatch-advance"$/{found=1} found && /^\[\[hooks\]\]/{found=0} found && /^file_pattern/{print; exit}' "$PRODUCTION_REGISTRY" | grep -q 'file_pattern'; then
    echo "FAIL: production registry has file_pattern field — must use in-plugin path-component-strict guards" >&2
    return 1
  fi

  # Extract path_allow from production registry for validate-dispatch-advance entry
  local prod_path_allow
  prod_path_allow=$(awk '
    /^name = "validate-dispatch-advance"$/ { in_hook=1 }
    in_hook && /^path_allow = \[/ { in_pa=1; next }
    in_pa && /^\]/ { in_pa=0; in_hook=0 }
    in_pa { gsub(/^[[:space:]]+/, ""); gsub(/,$/, ""); print }
  ' "$PRODUCTION_REGISTRY")

  if [ -z "$prod_path_allow" ]; then
    echo "FAIL: could not extract path_allow from production registry" >&2
    return 1
  fi

  # Confirm production path_allow does NOT use ** glob (S-15.11 F-P2-001 lesson preemptive application)
  if echo "$prod_path_allow" | grep -q '\*\*'; then
    echo "FAIL: production path_allow contains '**' glob — canonicalize() will fail => fail-open" >&2
    echo "FAIL: use bare directory path (e.g., \".factory\") not glob" >&2
    return 1
  fi

  cat > "$WORK/hooks-registry.toml" << TOML
schema_version = 2

[[hooks]]
name = "validate-dispatch-advance"
event = "PostToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/validate-dispatch-advance.wasm"
timeout_ms = 5000
on_error = "continue"

[hooks.capabilities.read_file]
path_allow = [
  ${prod_path_allow}
]
TOML
}

_state_md_envelope() {
  local session_id="${1:-prod-registry-test}"
  printf '{"event_name":"PostToolUse","tool_name":"Edit","session_id":"%s","tool_input":{"file_path":".factory/STATE.md","content":""},"tool_response":{"exit_code":0,"stdout":"","stderr":""}}' \
    "$session_id"
}

# ---------------------------------------------------------------------------
# AC-19 / Scenario A: production-shape registry + valid STATE.md => exit 0 (Continue)
# Proves host::read_file succeeds with the production path_allow.
# ---------------------------------------------------------------------------

@test "PROD-REGISTRY: hook emits Continue for valid STATE.md using production path_allow entry" {
  _require_artifacts
  cp -r "$FIXTURE_VALID/factory/." "$WORK/.factory/"
  _write_production_registry || {
    echo "production registry validation failed — test cannot proceed" >&2
    return 1
  }
  grep -q 'path_allow = \[' "$WORK/hooks-registry.toml" || {
    echo "FAIL: synthesized registry missing path_allow block" >&2
    return 1
  }
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope "prod-valid")"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 0: valid STATE.md => Continue
  [ "$status" -eq 0 ]

  # No blocking_plugins for a clean pass
  [[ "$output" != *"blocking_plugins=validate-dispatch-advance"* ]]
}

# ---------------------------------------------------------------------------
# AC-19 / Scenario B: production-shape registry + invalid STATE.md => exit 2 (Block)
#
# LOAD-BEARING for S-15.11 F-P2-001 preemptive application:
#   - If path_allow = ".factory/**": canonicalize() fails => path_allowed() false =>
#     host::read_file returns CapabilityDenied => hook fail-opens => exit 0.
#     TEST FAILS (expected 2, got 0).
#   - If path_allow = ".factory": canonicalize() succeeds => read_file returns content =>
#     hook evaluates => blocks => exit 2. TEST PASSES.
# ---------------------------------------------------------------------------

@test "PROD-REGISTRY: hook blocks for invalid STATE.md using production path_allow entry (not fail-open)" {
  _require_artifacts
  cp -r "$FIXTURE_INVALID/factory/." "$WORK/.factory/"
  _write_production_registry || {
    echo "production registry validation failed — test cannot proceed" >&2
    return 1
  }
  grep -q 'path_allow = \[' "$WORK/hooks-registry.toml" || {
    echo "FAIL: synthesized registry missing path_allow block" >&2
    return 1
  }
  cp "$WASM_PLUGIN" "$WORK/hook-plugins/"

  local envelope
  envelope="$(_state_md_envelope "prod-invalid")"
  run bash -c "printf '%s' '$envelope' | CLAUDE_PLUGIN_ROOT='$WORK' CLAUDE_PROJECT_DIR='$WORK' '$DISPATCHER' 2>&1 >/dev/null"

  # Exit 2: invalid STATE.md => Block. Exit 0 here means hook silently fail-opened
  # due to CapabilityDenied from misconfigured path_allow — the preemptive S-15.11 regression.
  [ "$status" -eq 2 ]

  # blocking_plugins= names this hook (not fail-open)
  [[ "$output" == *"blocking_plugins=validate-dispatch-advance"* ]]
}
