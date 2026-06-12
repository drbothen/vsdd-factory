#!/usr/bin/env bats
# verify-state-timestamp-refresh.bats — D17 bats integration tests for S-17.04.
#
# Covers the 3 canonical bats integration cases from ADR-025 §12.6 / D17 spec:
#
#   T-1 (AC-003): Timestamps advanced → guard allows (exit 0)
#   T-2 (AC-005): Stale timestamp (unchanged) → guard blocks (exit 2, TimestampStale)
#   T-3 (AC-007): Non-STATE.md path → guard allows (exit 0) immediately
#
# Story: S-17.04 (verify-state-timestamp-refresh WASM guard + factory-lock-parse crate)
# BC gate: BC-5.40.001 PC4 (mid-burst renewal enforcement)
# ADR: ADR-025 v1.6 Decision 12
#
# RED GATE strategy:
#   All tests require two artifacts that the implementer produces in T-3/T-4 (D16):
#     1. plugins/vsdd-factory/hook-plugins/verify-state-timestamp-refresh.wasm (compiled WASM)
#     2. Registry entry in plugins/vsdd-factory/hooks-registry.toml (T-4 / D16 registry)
#   Until those artifacts exist, ALL tests skip with an actionable "not built yet"
#   message — the tests are correctly RED (skip != pass) at Red Gate time.
#
#   After D15/D16 implementation and wasm build:
#     T-1 (AC-003): should become GREEN (timestamps advanced → exit 0)
#     T-2 (AC-005): should become GREEN (stale timestamp → exit 2, TimestampStale)
#     T-3 (AC-007): should become GREEN (non-STATE.md → exit 0)
#
# Dispatcher invocation pattern mirrors verify-factory-lock.bats:
#   printf '%s' "$envelope" | CLAUDE_PLUGIN_ROOT="$WORK" CLAUDE_PROJECT_DIR="$WORK" \
#     "$DISPATCHER" 2>&1 >/dev/null
#
# Exit codes:
#   0 = Continue (allow)
#   2 = Block (block with reason)
#   1 = Error (plugin failed)
#
# Run:
#   bats plugins/vsdd-factory/tests/verify-state-timestamp-refresh/verify-state-timestamp-refresh.bats

# ---------------------------------------------------------------------------
# setup / teardown
# ---------------------------------------------------------------------------

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  GUARD_WASM="$PLUGIN_ROOT/hook-plugins/verify-state-timestamp-refresh.wasm"

  WORK="$(mktemp -d)"
  mkdir -p "$WORK/.factory/logs"
  mkdir -p "$WORK/hook-plugins"

  # Copy the guard WASM into the synthetic plugin root if it exists.
  if [ -f "$GUARD_WASM" ]; then
    cp "$GUARD_WASM" "$WORK/hook-plugins/verify-state-timestamp-refresh.wasm"
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
# This is the RED GATE skip — both artifacts are produced by the implementer (T-3/T-4 D16).
_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    skip "factory-dispatcher binary not built — run: cargo build --release -p factory-dispatcher (S-17.04 implementer task T-3)"
  fi
  if [ ! -f "$WORK/hook-plugins/verify-state-timestamp-refresh.wasm" ]; then
    skip "verify-state-timestamp-refresh.wasm not present — run: cargo build --target wasm32-wasip1 -p verify-state-timestamp-refresh (S-17.04 implementer task T-3)"
  fi
}

# ---------------------------------------------------------------------------
# Registry writer
# ---------------------------------------------------------------------------

# Write the canonical verify-state-timestamp-refresh registry entry.
# Per ADR-025 Decision 12 §12.5 + S-17.04 AC-010 / D16 spec.
# async = false REQUIRED (ADR-019 correctness requirement).
# [hooks.capabilities.read_file] block MANDATORY (deny-by-default footgun prevention).
# No exec_subprocess capability (guard reads file only; no subprocess per §12.5).
_write_full_registry() {
  cat > "$WORK/hooks-registry.toml" <<'EOF'
schema_version = 2

# ---------- verify-state-timestamp-refresh (PreToolUse, Edit|Write) ----------
# S-17.04: PreToolUse guard — blocks Edit/Write to .factory/STATE.md when proposed
# content does not advance timestamp: frontmatter (every write) or
# factory_lock.expires_at (when lock held). Decision 12 / BC-5.40.001 PC4.
# Trigger: file_path == ".factory/STATE.md" — structurally bypass-proof.
# async = false REQUIRED — see ADR-019 + ADR-025 Decision 12

[[hooks]]
name = "verify-state-timestamp-refresh"
event = "PreToolUse"
tool = "Edit|Write"
plugin = "hook-plugins/verify-state-timestamp-refresh.wasm"
priority = 142
timeout_ms = 5000
on_error = "continue"
async = false

[hooks.capabilities.read_file]
path_allow = [".factory/STATE.md"]
EOF
}

# ---------------------------------------------------------------------------
# STATE.md fixture writers
# ---------------------------------------------------------------------------

# Write STATE.md with a given timestamp and NO factory_lock block.
_write_state_no_lock_with_ts() {
  local timestamp="${1:-2026-06-11T10:00:00Z}"
  mkdir -p "$WORK/.factory"
  cat > "$WORK/.factory/STATE.md" <<EOF
---
document_type: state
version: "0.0.1-bats-test"
timestamp: "${timestamp}"
phase: test
current_step: "bats-test"
---

# STATE (bats fixture — no lock, ts=${timestamp})
EOF
}

# Write STATE.md with a given timestamp AND a lock block with given expires_at.
_write_state_with_lock() {
  local timestamp="${1:-2026-06-11T10:00:00Z}"
  local expires_at="${2:-2026-06-11T10:45:00Z}"
  mkdir -p "$WORK/.factory"
  cat > "$WORK/.factory/STATE.md" <<EOF
---
document_type: state
version: "0.0.1-bats-test"
timestamp: "${timestamp}"
phase: test
current_step: "bats-test"
factory_lock:
  holder: "dev@example.com"
  locked_at: "2026-06-11T10:00:00Z"
  expires_at: "${expires_at}"
---

# STATE (bats fixture — lock held, ts=${timestamp}, exp=${expires_at})
EOF
}

# ---------------------------------------------------------------------------
# Dispatcher invocation helper
# ---------------------------------------------------------------------------

# Invoke the dispatcher with the given JSON envelope and proposed content.
# Sets $status and $output (combined stdout+stderr).
_run_dispatcher_with_content() {
  local envelope="$1"
  run bash -c "printf '%s' '$envelope' | \
    CLAUDE_PLUGIN_ROOT='$WORK' \
    CLAUDE_PROJECT_DIR='$WORK' \
    '$DISPATCHER' 2>&1"
}

# ---------------------------------------------------------------------------
# T-1 (AC-003): Timestamps advanced → guard allows (exit 0)
#
# On-disk STATE.md: timestamp = TS_OLD
# Proposed content (in new_content): timestamp = TS_NEW (advanced)
# Expected: exit 0 (Continue)
#
# RED GATE: WASM absent → skip (the correct Red Gate result).
# ---------------------------------------------------------------------------

@test "T-1 test_verify_state_timestamp_refresh_continues_when_timestamps_advanced" {
  _require_artifacts
  _write_full_registry

  local ts_old="2026-06-11T10:00:00Z"
  local ts_new="2026-06-11T11:00:00Z"

  # On-disk STATE.md: old timestamp.
  _write_state_no_lock_with_ts "$ts_old"

  # Proposed content: timestamp advanced to ts_new.
  # Note: YAML values are unquoted to avoid double-quote escaping issues when
  # embedding in the JSON envelope via shell variable interpolation.
  local proposed_content
  proposed_content="---\ndocument_type: state\nversion: 0.0.1-bats-test\ntimestamp: ${ts_new}\nphase: test\ncurrent_step: bats-test\n---\n\n# STATE (bats fixture - advanced)\n"

  local envelope
  # new_content carries the proposed write payload.
  envelope="{\"event_name\":\"PreToolUse\",\"tool_name\":\"Edit\",\"session_id\":\"t1\",\"dispatcher_trace_id\":\"t1-trace\",\"tool_input\":{\"file_path\":\".factory/STATE.md\",\"new_content\":\"${proposed_content}\"}}"

  _run_dispatcher_with_content "$envelope"

  # Must exit 0 (Continue) — timestamps advanced, guard passes.
  [ "$status" -eq 0 ]
}

# ---------------------------------------------------------------------------
# T-2 (AC-005): Stale timestamp (unchanged) → guard blocks (exit 2, TimestampStale)
#
# On-disk STATE.md: timestamp = TS_OLD
# Proposed content: timestamp = TS_OLD (NOT advanced — stale)
# Expected: exit 2 (Block) with "TimestampStale" in output.
#
# RED GATE: WASM absent → skip. If WASM is present but stub (returns Continue
# unconditionally), exit-2 assertion fails — correct Red Gate failure for the
# compiled stub.
# ---------------------------------------------------------------------------

@test "T-2 test_verify_state_timestamp_refresh_stale_timestamp_blocks" {
  _require_artifacts
  _write_full_registry

  local ts_old="2026-06-11T10:00:00Z"

  # On-disk STATE.md: old timestamp.
  _write_state_no_lock_with_ts "$ts_old"

  # Proposed content: timestamp NOT advanced (same as on-disk → stale).
  # Note: YAML values are unquoted to avoid double-quote escaping issues when
  # embedding in the JSON envelope via shell variable interpolation.
  local proposed_content
  proposed_content="---\ndocument_type: state\nversion: 0.0.1-bats-test\ntimestamp: ${ts_old}\nphase: test\ncurrent_step: bats-test\n---\n\n# STATE (bats fixture - stale)\n"

  local envelope
  envelope="{\"event_name\":\"PreToolUse\",\"tool_name\":\"Edit\",\"session_id\":\"t2\",\"dispatcher_trace_id\":\"t2-trace\",\"tool_input\":{\"file_path\":\".factory/STATE.md\",\"new_content\":\"${proposed_content}\"}}"

  _run_dispatcher_with_content "$envelope"

  # Must exit 2 (Block) — stale timestamp detected.
  [ "$status" -eq 2 ]

  # Block reason must contain "TimestampStale" (canonical error code per AC-005).
  [[ "$output" == *"TimestampStale"* ]] || {
    echo "FAIL: expected 'TimestampStale' in output. Got: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-3 (AC-007): Non-STATE.md path → guard allows (exit 0) immediately
#
# file_path is NOT .factory/STATE.md → guard returns Continue without
# reading the STATE.md file (zero overhead path).
# Expected: exit 0 (Continue).
#
# RED GATE: WASM absent → skip.
# ---------------------------------------------------------------------------

@test "T-3 test_verify_state_timestamp_refresh_continues_for_non_state_md" {
  _require_artifacts
  _write_full_registry

  # No need to write STATE.md — the guard must not read it for non-STATE.md paths.
  # (The absence of the file is itself a test: if the guard tries to read it and
  # fails, it should fail-open. But the correct behavior is to not call read_file at all.)

  local envelope
  envelope="{\"event_name\":\"PreToolUse\",\"tool_name\":\"Edit\",\"session_id\":\"t3\",\"dispatcher_trace_id\":\"t3-trace\",\"tool_input\":{\"file_path\":\".factory/specs/some-spec.md\",\"new_content\":\"# Some spec content\n\"}}"

  _run_dispatcher_with_content "$envelope"

  # Must exit 0 (Continue) — non-STATE.md path returns Continue immediately (AC-007).
  [ "$status" -eq 0 ]
}

# ---------------------------------------------------------------------------
# Registry assertion test (no WASM required — inspects production registry file).
#
# Asserts that the production hooks-registry.toml has the verify-state-timestamp-refresh
# entry with the mandatory [hooks.capabilities.read_file] block (AC-010).
# Omitting the block causes CapabilityDenied → silent fail-open → guard is a no-op.
#
# RED GATE: The entry does not exist yet (implementer adds it in T-4 / D16).
# This test FAILS until the registry entry is added.
# ---------------------------------------------------------------------------

@test "test_verify_state_timestamp_refresh_registry_entry_has_capability_block" {
  local registry="$REPO_ROOT/plugins/vsdd-factory/hooks-registry.toml"

  # Confirm the production registry file exists and is readable.
  [ -f "$registry" ]

  # Check that verify-state-timestamp-refresh is registered.
  # This will FAIL if the entry does not yet exist (Red Gate for registry task T-4).
  grep -q 'name = "verify-state-timestamp-refresh"' "$registry" || {
    echo "FAIL: verify-state-timestamp-refresh entry not found in production hooks-registry.toml."
    echo "Implementer: add the [[hooks]] entry per ADR-025 Decision 12 §12.5 / S-17.04 AC-010."
    return 1
  }

  # Use awk to extract the verify-state-timestamp-refresh section and verify:
  #   1. async = false (correctness requirement per ADR-019 + ADR-025 Decision 12)
  #   2. [hooks.capabilities.read_file] block present
  #   3. path_allow includes ".factory/STATE.md"
  #   4. max_bytes = 65536

  local score
  score=$(awk '
    /name = "verify-state-timestamp-refresh"/ { in_section=1; has_async_false=0; has_read_file=0; has_path_allow=0 }
    /^\[\[hooks\]\]/ && in_section && !/name = "verify-state-timestamp-refresh"/ {
      # End of section — tally.
      in_section = 0
    }
    in_section && /^async = false/ { has_async_false = 1 }
    in_section && /\[hooks\.capabilities\.read_file\]/ { has_read_file = 1 }
    in_section && has_read_file && /\.factory\/STATE\.md/ { has_path_allow = 1 }
    END {
      # Flush last section.
      total = has_async_false + has_read_file + has_path_allow
      print total
    }
  ' "$registry")

  # Must score 3: async=false + read_file block + path_allow=.factory/STATE.md.
  # Note: max_bytes and timeout_ms are not valid ReadFileCaps fields in the registry
  # schema (ReadFileCaps uses deny_unknown_fields; only path_allow is supported).
  # The registry entry must use only schema-valid fields to avoid TOML parse failure.
  [ "$score" -eq 3 ] || {
    echo "FAIL: verify-state-timestamp-refresh registry entry is incomplete (score=$score/3)."
    echo "Required: async=false + [hooks.capabilities.read_file] + path_allow=.factory/STATE.md"
    echo "See ADR-025 Decision 12 §12.5 / S-17.04 AC-010."
    return 1
  }
}
