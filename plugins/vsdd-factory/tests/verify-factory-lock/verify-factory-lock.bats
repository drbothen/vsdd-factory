#!/usr/bin/env bats
# verify-factory-lock.bats — D9 bats integration tests for BC-4.13.001.
#
# Covers the 9 canonical test vectors from BC-4.13.001 §Canonical Test Vectors
# (T-1..T-9; T-10 belongs to S-17.03 skill tests):
#
#   T-1: factory_lock absent → Edit → Continue
#   T-2: foreign unexpired lock → Edit → Block with 5-field message
#   T-3: foreign expired lock → Edit → Continue
#   T-4: self-held lock → Edit → Continue
#   T-5: foreign lock → Read → not triggered (Continue)
#   T-6: foreign lock → Bash git push origin factory-artifacts → Block
#   T-7: foreign lock → Bash cat .factory/STATE.md → Continue (non-push)
#   T-8: capability-omitted registry entry → Edit → Continue (graceful-degrade)
#   T-9: malformed expires_at → Edit → Continue (malformed fail-open)
#
# Story: S-17.02 (verify-factory-lock WASM guard crate + registry entries)
# BC gate: BC-4.13.001 (v1.0)
#
# RED GATE strategy:
#   All tests require two artifacts that the implementer produces in T-5/T-6:
#     1. plugins/vsdd-factory/hook-plugins/verify-factory-lock.wasm (compiled WASM)
#     2. Registry entries in plugins/vsdd-factory/hooks-registry.toml (T-5)
#   Until those artifacts exist, ALL tests skip with an actionable "not built yet"
#   message — the tests are correctly RED (skip != pass) at Red Gate time.
#   After implementation (T-5/T-6), the tests should become GREEN on the full suite.
#
# Dispatcher invocation pattern mirrors td-71-stderr-block-reason.bats:
#   printf '%s' "$envelope" | CLAUDE_PLUGIN_ROOT="$WORK" CLAUDE_PROJECT_DIR="$WORK" \
#     "$DISPATCHER" 2>&1 >/dev/null
#
# Exit codes:
#   0 = Continue (allow)
#   2 = Block (block with reason)
#   1 = Error (plugin failed)
#
# Run:
#   bats plugins/vsdd-factory/tests/verify-factory-lock/verify-factory-lock.bats

# ---------------------------------------------------------------------------
# setup / teardown
# ---------------------------------------------------------------------------

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  GUARD_WASM="$PLUGIN_ROOT/hook-plugins/verify-factory-lock.wasm"

  WORK="$(mktemp -d)"
  mkdir -p "$WORK/.factory/logs"
  mkdir -p "$WORK/hook-plugins"

  # Copy the guard WASM into the synthetic plugin root if it exists.
  if [ -f "$GUARD_WASM" ]; then
    cp "$GUARD_WASM" "$WORK/hook-plugins/verify-factory-lock.wasm"
  fi

  export CLAUDE_PROJECT_DIR="$WORK"
  export CLAUDE_PLUGIN_ROOT="$WORK"
}

teardown() {
  rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# Preflight helpers
# ---------------------------------------------------------------------------

# Skip if dispatcher binary or guard WASM is not present.
# This is the RED GATE skip — both artifacts are produced by the implementer (T-5/T-6).
_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "factory-dispatcher binary not built — run: cargo build --release -p factory-dispatcher (S-17.02 implementer task T-6)"
  fi
  if [ ! -f "$WORK/hook-plugins/verify-factory-lock.wasm" ]; then
    skip "verify-factory-lock.wasm not present — run: cargo build --target wasm32-wasip1 -p verify-factory-lock (S-17.02 implementer task T-6)"
  fi
}

# ---------------------------------------------------------------------------
# Registry writers
# ---------------------------------------------------------------------------

# Write the standard two-entry verify-factory-lock registry (AC-009 canonical form).
# Both entries have BOTH capability blocks (the production-grade default).
# async = false on both (ADR-019 correctness requirement).
_write_full_registry() {
  cat > "$WORK/hooks-registry.toml" <<'EOF'
schema_version = 2

# async = false REQUIRED — see ADR-019 + ADR-025
[[hooks]]
name = "verify-factory-lock"
plugin = "hook-plugins/verify-factory-lock.wasm"
event = "PreToolUse"
tool = "Edit|Write|Agent"
async = false
on_error = "continue"
timeout_ms = 5000

[hooks.capabilities.read_file]
path_allow = [".factory/STATE.md"]

[hooks.capabilities.exec_subprocess]
binary_allow = ["git"]
env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]

# async = false REQUIRED — see ADR-019 + ADR-025
[[hooks]]
name = "verify-factory-lock-bash"
plugin = "hook-plugins/verify-factory-lock.wasm"
event = "PreToolUse"
tool = "Bash"
async = false
on_error = "continue"
timeout_ms = 5000

[hooks.capabilities.read_file]
path_allow = [".factory/STATE.md"]

[hooks.capabilities.exec_subprocess]
binary_allow = ["git"]
env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]
EOF
}

# Write a registry with the Bash entry only — no read_file capability block.
# This simulates EC-007 (capability-omitted footgun for T-8).
_write_capability_omitted_registry() {
  cat > "$WORK/hooks-registry.toml" <<'EOF'
schema_version = 2

# Intentionally missing [hooks.capabilities.read_file] — tests graceful degrade
[[hooks]]
name = "verify-factory-lock"
plugin = "hook-plugins/verify-factory-lock.wasm"
event = "PreToolUse"
tool = "Edit|Write|Agent"
async = false
on_error = "continue"
timeout_ms = 5000

[hooks.capabilities.exec_subprocess]
binary_allow = ["git"]
EOF
}

# ---------------------------------------------------------------------------
# STATE.md fixture writers (written to WORK/.factory/STATE.md)
# ---------------------------------------------------------------------------

# Write STATE.md with NO factory_lock block (unlocked baseline, EC-001 path).
_write_state_no_lock() {
  mkdir -p "$WORK/.factory"
  cat > "$WORK/.factory/STATE.md" <<'EOF'
---
document_type: state
version: "0.0.1-bats-test"
phase: test
current_step: "bats-test"
---

# STATE (bats fixture — no lock)
EOF
}

# Write STATE.md with a foreign UNEXPIRED factory_lock block.
# holder = other@example.com; expires_at = 2099-01-01T00:00:00Z (far future).
_write_state_foreign_unexpired_lock() {
  local holder="${1:-other@example.com}"
  local locked_at="${2:-2026-06-10T14:00:00Z}"
  local expires_at="${3:-2099-01-01T00:00:00Z}"
  mkdir -p "$WORK/.factory"
  cat > "$WORK/.factory/STATE.md" <<EOF
---
document_type: state
version: "0.0.1-bats-test"
phase: test
current_step: "bats-test"
factory_lock:
  holder: "${holder}"
  locked_at: "${locked_at}"
  expires_at: "${expires_at}"
---

# STATE (bats fixture — foreign unexpired lock)
EOF
}

# Write STATE.md with a foreign EXPIRED factory_lock block.
# expires_at is well in the past.
_write_state_foreign_expired_lock() {
  mkdir -p "$WORK/.factory"
  cat > "$WORK/.factory/STATE.md" <<'EOF'
---
document_type: state
version: "0.0.1-bats-test"
phase: test
current_step: "bats-test"
factory_lock:
  holder: "other@example.com"
  locked_at: "2020-01-01T00:00:00Z"
  expires_at: "2020-01-01T00:45:00Z"
---

# STATE (bats fixture — foreign expired lock)
EOF
}

# Write STATE.md with a self-held UNEXPIRED factory_lock block.
# holder matches git config user.email at test time.
_write_state_self_held_lock() {
  local self_email
  self_email="$(git config user.email 2>/dev/null | tr -d '\n')"
  if [ -z "$self_email" ]; then
    self_email="bats-test-self@example.com"
  fi
  mkdir -p "$WORK/.factory"
  cat > "$WORK/.factory/STATE.md" <<EOF
---
document_type: state
version: "0.0.1-bats-test"
phase: test
current_step: "bats-test"
factory_lock:
  holder: "${self_email}"
  locked_at: "2026-06-10T14:00:00Z"
  expires_at: "2099-01-01T00:00:00Z"
---

# STATE (bats fixture — self-held lock)
EOF
}

# Write STATE.md with a malformed factory_lock block: expires_at is not ISO-8601.
_write_state_malformed_expires_at() {
  mkdir -p "$WORK/.factory"
  cat > "$WORK/.factory/STATE.md" <<'EOF'
---
document_type: state
version: "0.0.1-bats-test"
phase: test
current_step: "bats-test"
factory_lock:
  holder: "other@example.com"
  locked_at: "2026-06-10T14:00:00Z"
  expires_at: "not-a-valid-iso8601-timestamp"
---

# STATE (bats fixture — malformed expires_at)
EOF
}

# ---------------------------------------------------------------------------
# Dispatcher invocation helper
# ---------------------------------------------------------------------------

# Invoke the dispatcher with the given JSON envelope.
# Sets $status and $output (combined stdout+stderr per td-71 pattern).
_run_dispatcher() {
  local envelope="$1"
  run bash -c "printf '%s' '$envelope' | \
    CLAUDE_PLUGIN_ROOT='$WORK' \
    CLAUDE_PROJECT_DIR='$WORK' \
    '$DISPATCHER' 2>&1 >/dev/null"
}

# ---------------------------------------------------------------------------
# T-1: factory_lock absent → Edit → Continue (unlocked path, EC-001)
# ---------------------------------------------------------------------------

@test "T-1 test_BC_4_13_001_absent_lock_edit_returns_continue" {
  _require_artifacts
  _write_full_registry
  _write_state_no_lock

  local envelope
  envelope='{"event_name":"PreToolUse","tool_name":"Edit","session_id":"t1","dispatcher_trace_id":"t1-trace","tool_input":{"file_path":".factory/STATE.md"}}'

  _run_dispatcher "$envelope"

  # Must exit 0 (Continue) — no lock held.
  [ "$status" -eq 0 ]
}

# ---------------------------------------------------------------------------
# T-2: foreign unexpired lock → Edit → Block with 5-field message
# ---------------------------------------------------------------------------

@test "T-2 test_BC_4_13_001_foreign_unexpired_lock_edit_blocks_with_five_fields" {
  _require_artifacts
  _write_full_registry
  _write_state_foreign_unexpired_lock "other@example.com" "2026-06-10T14:00:00Z" "2099-01-01T00:00:00Z"

  local envelope
  envelope='{"event_name":"PreToolUse","tool_name":"Edit","session_id":"t2","dispatcher_trace_id":"t2-trace","tool_input":{"file_path":".factory/STATE.md"}}'

  # Capture both stdout AND stderr so we can inspect the block message.
  run bash -c "printf '%s' '$envelope' | \
    CLAUDE_PLUGIN_ROOT='$WORK' \
    CLAUDE_PROJECT_DIR='$WORK' \
    '$DISPATCHER' 2>&1"

  # Must exit 2 (Block).
  [ "$status" -eq 2 ]

  # All 5 required fields per BC-4.13.001 PC1 / AC-001:
  # 1. holder email
  [[ "$output" == *"other@example.com"* ]]
  # 2. locked_at timestamp
  [[ "$output" == *"2026-06-10T14:00:00Z"* ]]
  # 3. expires_at timestamp
  [[ "$output" == *"2099-01-01T00:00:00Z"* ]]
  # 4. time_remaining human-readable ("N min remaining")
  [[ "$output" == *"min remaining"* ]]
  # 5. break-glass command (exact string)
  [[ "$output" == *"/factory-unlock --force"* ]]
}

# ---------------------------------------------------------------------------
# T-3: foreign expired lock → Edit → Continue (TTL expired, PC2 LockExpired path)
# ---------------------------------------------------------------------------

@test "T-3 test_BC_4_13_001_foreign_expired_lock_edit_returns_continue" {
  _require_artifacts
  _write_full_registry
  _write_state_foreign_expired_lock

  local envelope
  envelope='{"event_name":"PreToolUse","tool_name":"Edit","session_id":"t3","dispatcher_trace_id":"t3-trace","tool_input":{"file_path":".factory/STATE.md"}}'

  _run_dispatcher "$envelope"

  # Must exit 0 (Continue) — lock is expired.
  [ "$status" -eq 0 ]
}

# ---------------------------------------------------------------------------
# T-4: self-held lock → Edit → Continue (PC3 self-held, developer never blocked)
# ---------------------------------------------------------------------------

@test "T-4 test_BC_4_13_001_self_held_lock_edit_returns_continue" {
  _require_artifacts
  _write_full_registry
  _write_state_self_held_lock

  local envelope
  envelope='{"event_name":"PreToolUse","tool_name":"Edit","session_id":"t4","dispatcher_trace_id":"t4-trace","tool_input":{"file_path":".factory/STATE.md"}}'

  _run_dispatcher "$envelope"

  # Must exit 0 (Continue) — holder == caller.
  [ "$status" -eq 0 ]
}

# ---------------------------------------------------------------------------
# T-5: foreign unexpired lock → Read → not triggered (Continue, PC5)
#
# Read is NOT in the registry's tool regex (Edit|Write|Agent) so the plugin
# is never invoked. The dispatcher exits 0 immediately.
# ---------------------------------------------------------------------------

@test "T-5 test_BC_4_13_001_foreign_lock_read_not_triggered_returns_continue" {
  _require_artifacts
  _write_full_registry
  _write_state_foreign_unexpired_lock "other@example.com" "2026-06-10T14:00:00Z" "2099-01-01T00:00:00Z"

  local envelope
  envelope='{"event_name":"PreToolUse","tool_name":"Read","session_id":"t5","dispatcher_trace_id":"t5-trace","tool_input":{"file_path":".factory/STATE.md"}}'

  _run_dispatcher "$envelope"

  # Must exit 0 (Continue) — Read is not in scope (PC5: not triggered).
  [ "$status" -eq 0 ]
}

# ---------------------------------------------------------------------------
# T-6: foreign unexpired lock → Bash git push origin factory-artifacts → Block
#
# The Bash arm's internal push-regex matches and the guard blocks.
# ---------------------------------------------------------------------------

@test "T-6 test_BC_4_13_001_bash_push_factory_artifacts_foreign_lock_blocks" {
  _require_artifacts
  _write_full_registry
  _write_state_foreign_unexpired_lock "other@example.com" "2026-06-10T14:00:00Z" "2099-01-01T00:00:00Z"

  local envelope
  envelope='{"event_name":"PreToolUse","tool_name":"Bash","session_id":"t6","dispatcher_trace_id":"t6-trace","tool_input":{"command":"git push origin factory-artifacts"}}'

  _run_dispatcher "$envelope"

  # Must exit 2 (Block) — push arm intercepted by internal regex.
  [ "$status" -eq 2 ]
}

# ---------------------------------------------------------------------------
# T-7: foreign unexpired lock → Bash cat .factory/STATE.md → Continue (EC-011)
#
# Non-push Bash command: internal push-regex does NOT match → Continue immediately
# without reading STATE.md.
# ---------------------------------------------------------------------------

@test "T-7 test_BC_4_13_001_bash_non_push_command_foreign_lock_returns_continue" {
  _require_artifacts
  _write_full_registry
  _write_state_foreign_unexpired_lock "other@example.com" "2026-06-10T14:00:00Z" "2099-01-01T00:00:00Z"

  local envelope
  envelope='{"event_name":"PreToolUse","tool_name":"Bash","session_id":"t7","dispatcher_trace_id":"t7-trace","tool_input":{"command":"cat .factory/STATE.md"}}'

  _run_dispatcher "$envelope"

  # Must exit 0 (Continue) — non-push Bash short-circuits immediately.
  [ "$status" -eq 0 ]
}

# ---------------------------------------------------------------------------
# T-8: capability-omitted registry entry → Edit → Continue (graceful-degrade, EC-007)
#
# When [hooks.capabilities.read_file] is absent from the registry entry,
# host::read_file returns CapabilityDenied → the plugin graceful-degrades to
# HookResult::Continue (never blocks) per BC-4.13.001 Invariant 6 + AC-010.
# ---------------------------------------------------------------------------

@test "T-8 test_BC_4_13_001_capability_omitted_registry_gracefully_degrades_to_continue" {
  _require_artifacts
  _write_capability_omitted_registry
  # Write a foreign unexpired lock — the guard SHOULD block if working correctly,
  # but with capability omitted it must graceful-degrade to Continue.
  _write_state_foreign_unexpired_lock "other@example.com" "2026-06-10T14:00:00Z" "2099-01-01T00:00:00Z"

  local envelope
  envelope='{"event_name":"PreToolUse","tool_name":"Edit","session_id":"t8","dispatcher_trace_id":"t8-trace","tool_input":{"file_path":".factory/STATE.md"}}'

  _run_dispatcher "$envelope"

  # Must exit 0 (Continue) — CapabilityDenied graceful-degrade.
  # The guard is silently inert when the capability block is omitted (the
  # documented footgun per ADR-025 D2 Rationale; see AC-010 + AC-009).
  [ "$status" -eq 0 ]
}

# ---------------------------------------------------------------------------
# T-9: malformed expires_at → Edit → Continue (EC-005 malformed fail-open)
#
# When factory_lock.expires_at is not a valid ISO-8601 datetime, the guard
# treats it as MalformedLockBlock and returns Continue (PC4 fail-open).
# ---------------------------------------------------------------------------

@test "T-9 test_BC_4_13_001_malformed_expires_at_edit_returns_continue" {
  _require_artifacts
  _write_full_registry
  _write_state_malformed_expires_at

  local envelope
  envelope='{"event_name":"PreToolUse","tool_name":"Edit","session_id":"t9","dispatcher_trace_id":"t9-trace","tool_input":{"file_path":".factory/STATE.md"}}'

  _run_dispatcher "$envelope"

  # Must exit 0 (Continue) — malformed expires_at → MalformedLockBlock → fail-open.
  [ "$status" -eq 0 ]
}

# ---------------------------------------------------------------------------
# AC-016: env_allow-present assertion — production registry guard
#
# The production hooks-registry.toml MUST have env_allow containing HOME on
# BOTH verify-factory-lock exec_subprocess blocks. Without this, git config
# user.email runs in a clean environment → IdentityResolutionFailed → the
# guard falls back to fail-open (never blocks), breaking T-2 and T-6.
#
# Literal-shell grep asserting exactly 2 occurrences within the verify-factory-lock
# section of the production registry. Guards the env_allow footgun in CI.
# ---------------------------------------------------------------------------

@test "test_BC_4_13_001_registry_exec_subprocess_has_env_allow_home" {
  local registry="$REPO_ROOT/plugins/vsdd-factory/hooks-registry.toml"

  # Confirm the production registry file exists and is readable.
  [ -f "$registry" ]

  # Extract only the verify-factory-lock section(s) from the registry and count
  # occurrences of env_allow lines containing HOME.
  #
  # Strategy: use awk to collect lines between a verify-factory-lock plugin
  # declaration and the next [[hooks]] section (or EOF), then grep for env_allow
  # lines containing HOME.
  local count
  count=$(awk '
    /name = "verify-factory-lock/ { in_section=1 }
    /^\[\[hooks\]\]/ && in_section { in_section=0 }
    in_section && /env_allow/ && /HOME/ { count++ }
    END { print count+0 }
  ' "$registry")

  # Must be exactly 2 — one for verify-factory-lock (Edit|Write|Agent) and one
  # for verify-factory-lock-bash (Bash). Any fewer means a block is missing and
  # the guard is broken for that tool class.
  [ "$count" -eq 2 ]
}
