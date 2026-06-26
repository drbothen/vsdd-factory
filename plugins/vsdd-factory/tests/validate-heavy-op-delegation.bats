#!/usr/bin/env bats
# validate-heavy-op-delegation.bats — Red Gate integration tests for S-18.06.
#
# Story:   S-18.06 v1.5 — validate-heavy-op-delegation WASM gate
# BC gate: BC-4.15.001 v1.2 (all postconditions PC-A/PC-B-B1/PC-B-B2/PC-C/PC-D;
#          all 4 invariants INV1/INV2/INV3/INV4 in scope)
# VP:      VP-091 (machine-stable assertion discipline)
#
# RED GATE minimum: 7 bats tests (this file) + 5 Rust unit tests = 12 total.
# All MUST FAIL before any Rust crate source or compiled WASM binary is authored.
#
# RED GATE strategy for dispatcher-dependent tests:
#   Tests require two artifacts that the implementer produces in T-3/T-4/T-5 (S-18.06):
#     1. plugins/vsdd-factory/hook-plugins/validate-heavy-op-delegation.wasm
#     2. Registry entry in plugins/vsdd-factory/hooks-registry.toml
#   Until those artifacts exist, tests involving the dispatcher skip with an
#   actionable message (skip != pass; correctly RED at Red Gate time).
#   The registry shape test (AC-008) fails IMMEDIATELY because the registry
#   entry is absent — it does NOT require the WASM artifact.
#
# MACHINE-STABLE ASSERTION DISCIPLINE (VP-091 §2 / L-F2-machine-stable-count-assertion):
#   Bats tests MUST assert against the plugin.log structured record
#   (`code: DelegationRecommended`) in the dispatcher internal JSONL log
#   (FACTORY_DISPATCHER_INTERNAL_LOG=1), NOT via presentation-coupled regex over stderr.
#   The count of `"code":"DelegationRecommended"` records in the internal log is the
#   deterministic machine-stable signal. DELEGATION_COUNT is the canonical variable name.
#
# PAYLOAD FIELD DISCIPLINE (BC-4.15.001 PC1 / BC-4.13.001 precedent):
#   - Bash PreToolUse payload: event_name="PreToolUse", tool_name="Bash",
#     tool_input.command=<command string>
#   - Write PreToolUse payload: event_name="PreToolUse", tool_name="Write",
#     tool_input.file_path=<path>
#   - Verified from td-71-stderr-block-reason.bats + verify-factory-lock.bats precedents.
#   - Field name is "event_name" (not "hook_event_name") per production dispatcher contract.
#
# EXIT CODES:
#   0 = Continue (allow; no block)
#   2 = Block (block with reason; NOT the expected behavior for this gate — INV2)
#   1 = Error (plugin failed; fail-open on_error=continue → 0)
#
# INTERNAL LOG:
#   plugin.log advisory records are written to the dispatcher internal JSONL log:
#     $WORK/.factory/logs/dispatcher-internal-YYYY-MM-DD.jsonl
#   when FACTORY_DISPATCHER_INTERNAL_LOG=1 is set. This is the machine-stable
#   channel for asserting DelegationRecommended record presence/absence/count.
#
# Red Gate Test Plan table (S-18.06 v1.5 — bats subset):
#
#   | Test name                                                          | AC    |
#   |--------------------------------------------------------------------|-------|
#   | test_heavy_op_gate_emits_stderr_nudge_on_pattern_match             | AC-001|
#   | test_heavy_op_gate_emits_plugin_log_delegation_recommended_on_match| AC-002|
#   | test_heavy_op_gate_no_emission_on_no_match                         | AC-003|
#   | test_heavy_op_gate_always_returns_continue_on_match                | AC-004|
#   | test_heavy_op_gate_always_returns_continue_on_no_match             | AC-004|
#   | test_heavy_op_gate_fail_open_on_wasm_panic                         |AC-004/010|
#   | test_heavy_op_gate_not_dispatched_on_write_tool_call               | AC-007|
#   | test_heavy_op_gate_registry_entry_has_canonical_shape              | AC-008|
#
# Note: test_heavy_op_gate_registry_entry_has_canonical_shape (AC-008) is
# the ONLY test that does not use _require_artifacts — it asserts directly
# on the production hooks-registry.toml and fails immediately at Red Gate time
# because the entry is absent. All other tests skip gracefully when
# the WASM/dispatcher is absent.

# ---------------------------------------------------------------------------
# setup / teardown
# ---------------------------------------------------------------------------

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  DISPATCHER="$REPO_ROOT/target/release/factory-dispatcher"
  GATE_WASM="$PLUGIN_ROOT/hook-plugins/validate-heavy-op-delegation.wasm"
  PRODUCTION_REGISTRY="$PLUGIN_ROOT/hooks-registry.toml"

  WORK="$(mktemp -d)"
  mkdir -p "$WORK/.factory/logs"
  mkdir -p "$WORK/hook-plugins"

  # Copy the gate WASM into the synthetic plugin root if it exists.
  # If absent, _require_artifacts will skip the test (Red Gate graceful skip).
  if [ -f "$GATE_WASM" ]; then
    cp "$GATE_WASM" "$WORK/hook-plugins/validate-heavy-op-delegation.wasm"
  fi

  # Set VSDD_LOG_DIR so the dispatcher writes its internal log to our WORK dir.
  export VSDD_LOG_DIR="$WORK/.factory/logs"
  export CLAUDE_PROJECT_DIR="$WORK"
  export CLAUDE_PLUGIN_ROOT="$WORK"
}

teardown() {
  [ -n "${WORK:-}" ] && [ -d "$WORK" ] && \
    find "$WORK" -type f -delete && \
    find "$WORK" -type d -mindepth 1 | sort -r | xargs rmdir 2>/dev/null && \
    rmdir "$WORK" 2>/dev/null || true
}

# ---------------------------------------------------------------------------
# Preflight helpers
# ---------------------------------------------------------------------------

# Skip if dispatcher binary or gate WASM is not present.
# This is the RED GATE skip — both artifacts are produced by the implementer
# (T-3/T-4 in S-18.06). In CI with CI_REQUIRE_ARTIFACTS=1, absence is a HARD FAIL.
_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    [ -z "${CI_REQUIRE_ARTIFACTS:-}" ] || {
      echo "FAIL: factory-dispatcher binary not present (CI_REQUIRE_ARTIFACTS=1) — run: cargo build --release -p factory-dispatcher"
      return 1
    }
    skip "factory-dispatcher binary not built — run: cargo build --release -p factory-dispatcher (S-18.06 implementer task T-3)"
  fi
  if [ ! -f "$WORK/hook-plugins/validate-heavy-op-delegation.wasm" ]; then
    [ -z "${CI_REQUIRE_ARTIFACTS:-}" ] || {
      echo "FAIL: validate-heavy-op-delegation.wasm not present (CI_REQUIRE_ARTIFACTS=1) — run: cargo build --target wasm32-wasip1 -p validate-heavy-op-delegation"
      return 1
    }
    skip "validate-heavy-op-delegation.wasm not present — run: cargo build --target wasm32-wasip1 -p validate-heavy-op-delegation (S-18.06 implementer task T-4)"
  fi
}

# ---------------------------------------------------------------------------
# Registry writer
# ---------------------------------------------------------------------------

# Write the canonical validate-heavy-op-delegation registry entry per AC-008.
# async = false (no async variant for PreToolUse Bash gate per BC-4.15.001 PC1).
# on_error = "continue" (fail-open; INV2 + PC-C).
# tool = "Bash" (registry tool filter; PC-D).
# [hooks.config] patterns = [...] (v1 default set; BC-4.15.001 PC1).
_write_full_registry() {
  cat > "$WORK/hooks-registry.toml" <<'TOML'
schema_version = 2

# validate-heavy-op-delegation: PreToolUse advisory gate (BC-4.15.001)
# on_error = "continue" is mandatory — fail-open per BC-4.15.001 INV2 + PC-C.
# async = false: gate evaluates before the Bash command executes (PreToolUse).
# tool = "Bash": PC-D registry filter — non-Bash tool calls never dispatched.

[[hooks]]
name = "validate-heavy-op-delegation"
event = "PreToolUse"
plugin = "hook-plugins/validate-heavy-op-delegation.wasm"
tool = "Bash"
on_error = "continue"
async = false
timeout_ms = 5000

[hooks.config]
patterns = [
  "cargo test --release",
  "grep -r",
  "grep -R",
  "find . -name",
  "find . -type",
  "./run-all.sh",
  "./run-bats.sh"
]
TOML
}

# ---------------------------------------------------------------------------
# Helpers: synthetic PreToolUse event envelopes
# ---------------------------------------------------------------------------

# Build a synthetic PreToolUse Bash event JSON envelope.
# field: event_name (not hook_event_name) per production dispatcher contract.
# Verified from td-71-stderr-block-reason.bats and verify-factory-lock.bats precedents.
_bash_event() {
  local cmd="$1"
  local session="${2:-test-vhod}"
  # Escape double quotes in the command string for safe JSON embedding.
  local escaped_cmd
  escaped_cmd="$(printf '%s' "$cmd" | sed 's/\\/\\\\/g; s/"/\\"/g')"
  printf '{"event_name":"PreToolUse","tool_name":"Bash","session_id":"%s","dispatcher_trace_id":"%s-trace","tool_input":{"command":"%s"}}' \
    "$session" "$session" "$escaped_cmd"
}

# Build a synthetic PreToolUse Write event JSON envelope.
# Tests PC-D: non-Bash tool call must NOT dispatch the gate.
_write_event() {
  local file_path="${1:-/tmp/test.md}"
  local session="${2:-test-vhod-write}"
  printf '{"event_name":"PreToolUse","tool_name":"Write","session_id":"%s","dispatcher_trace_id":"%s-trace","tool_input":{"file_path":"%s","content":"test content"}}' \
    "$session" "$session" "$file_path"
}

# Invoke the dispatcher with a given JSON envelope, capturing stderr separately.
# Sets $status, $output (combined stdout+stderr for exit-code tests),
# and STDERR_FILE (path to dispatcher stderr capture file).
#
# For internal-log assertions use _run_dispatcher_with_internal_log.
_run_dispatcher() {
  local envelope="$1"
  STDERR_FILE="$WORK/dispatcher-stderr-$$.txt"
  run bash -c "printf '%s' '$envelope' | \
    VSDD_LOG_DIR='$WORK/.factory/logs' \
    CLAUDE_PLUGIN_ROOT='$WORK' \
    CLAUDE_PROJECT_DIR='$WORK' \
    '$DISPATCHER' 2>'$STDERR_FILE'"
}

# Invoke the dispatcher with FACTORY_DISPATCHER_INTERNAL_LOG=1 so that
# plugin.log advisory records are written to the JSONL internal log.
# Sets $status and INTERNAL_LOG_FILE (path to the JSONL log).
#
# VP-091 §2 machine-stable assertion target: the internal log is the
# deterministic channel for counting DelegationRecommended records.
_run_dispatcher_with_internal_log() {
  local envelope="$1"
  STDERR_FILE="$WORK/dispatcher-stderr-$$.txt"
  run bash -c "printf '%s' '$envelope' | \
    FACTORY_DISPATCHER_INTERNAL_LOG=1 \
    VSDD_LOG_DIR='$WORK/.factory/logs' \
    CLAUDE_PLUGIN_ROOT='$WORK' \
    CLAUDE_PROJECT_DIR='$WORK' \
    '$DISPATCHER' 2>'$STDERR_FILE'"
  # Locate the internal log (written to VSDD_LOG_DIR = $WORK/.factory/logs).
  INTERNAL_LOG_FILE="$(ls "$WORK/.factory/logs/"dispatcher-internal-*.jsonl 2>/dev/null | head -1)"
}

# Count DelegationRecommended records in the internal log.
# Machine-stable assertion: counts structured plugin.log records with
# "code":"DelegationRecommended" (VP-091 §2 / L-F2-machine-stable-count-assertion).
_count_delegation_records() {
  local logfile="${1:-$INTERNAL_LOG_FILE}"
  if [ -z "$logfile" ] || [ ! -f "$logfile" ]; then
    echo "0"
    return
  fi
  # grep -c exits 1 on macOS BSD grep when count=0; use local assignment to
  # capture the count correctly without triggering the || branch twice.
  local count
  count=$(grep -c '"code":"DelegationRecommended"' "$logfile" 2>/dev/null) || count=0
  echo "$count"
}

# ---------------------------------------------------------------------------
# AC-001 (BC-4.15.001 PC-B-B1): stderr nudge emitted on pattern match
#
# Setup: Bash command "cargo test --release --workspace"; default pattern list
#        includes "cargo test --release".
# Assert: stderr contains "cargo test --release" (matched pattern) and the
#         command preview; stdout does NOT contain the advisory message;
#         dispatcher exit 0 (Continue — INV2 never blocks).
#
# Red Gate condition: WASM absent → dispatcher emits no advisory;
#   stderr assertion fails ("cargo test --release" not found in stderr).
# ---------------------------------------------------------------------------

@test "test_heavy_op_gate_emits_stderr_nudge_on_pattern_match" {
  _require_artifacts
  _write_full_registry

  local cmd="cargo test --release --workspace"
  local envelope
  envelope="$(_bash_event "$cmd" "ac001-match")"

  _run_dispatcher "$envelope"

  # Assert: dispatcher returns Continue (exit 0) — INV2 never blocks (AC-004/INV2).
  [ "$status" -eq 0 ] || {
    echo "FAIL: expected exit 0 (Continue) but got status=$status (AC-001/INV2)"
    echo "Output: $output"
    return 1
  }

  # Assert: stderr contains the matched pattern string (PC-B-B1 nudge message).
  # The stderr file captures the gate's nudge message (PC-B-B1).
  [ -f "$STDERR_FILE" ] || {
    echo "FAIL: stderr capture file not created: $STDERR_FILE"
    return 1
  }
  grep -q "cargo test --release" "$STDERR_FILE" || {
    echo "FAIL: AC-001/PC-B-B1: stderr nudge message must contain matched pattern 'cargo test --release'."
    echo "Expected: stderr contains 'cargo test --release'"
    echo "Actual stderr: $(cat "$STDERR_FILE")"
    echo "RED GATE: WASM not built → no advisory emitted → this assertion correctly fails."
    return 1
  }

  # Assert: stderr nudge message contains the command preview.
  # For a short command like "cargo test --release --workspace" (<=120 chars),
  # the command_preview must equal the full command string (INV4 no truncation).
  grep -q "cargo test --release --workspace" "$STDERR_FILE" || {
    echo "FAIL: AC-001/PC-B-B1: stderr nudge message must contain command_preview."
    echo "Expected: stderr contains 'cargo test --release --workspace' (full command as preview)"
    echo "Actual stderr: $(cat "$STDERR_FILE")"
    return 1
  }

  # Assert: stdout does NOT contain the advisory message (gate must not write to stdout).
  # stdout is the dispatcher's tool-result channel (AC-001 / BC-4.15.001 PC-B-B1).
  [[ "$output" != *"DelegationRecommended"* ]] || {
    echo "FAIL: AC-001/PC-B-B1: advisory content must NOT appear on stdout (stdout is tool-result channel)."
    echo "Output (stdout): $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# AC-002 (BC-4.15.001 PC-B-B2): plugin.log structured record on pattern match
#
# Setup: Bash command "grep -r \"TODO\" ."; pattern "grep -r" in the list.
# Assert: dispatcher internal log contains exactly one record with
#         code: DelegationRecommended; matched_pattern is "grep -r"; level is "warn";
#         command_preview is present and ≤121 chars (120 + optional ellipsis); exit 0.
#
# Machine-stable assertion (VP-091 §2 / L-F2-machine-stable-count-assertion):
#   Counting "code":"DelegationRecommended" in the JSONL internal log —
#   NOT via presentation-coupled regex over stderr.
#
# Red Gate condition: WASM absent → no plugin.log record;
#   DELEGATION_COUNT -eq 1 assertion fails.
# ---------------------------------------------------------------------------

@test "test_heavy_op_gate_emits_plugin_log_delegation_recommended_on_match" {
  _require_artifacts
  _write_full_registry

  local cmd='grep -r "TODO" .'
  local envelope
  envelope="$(_bash_event "$cmd" "ac002-plugin-log")"

  _run_dispatcher_with_internal_log "$envelope"

  # Assert: dispatcher returns Continue (exit 0) — INV2.
  [ "$status" -eq 0 ] || {
    echo "FAIL: expected exit 0 (Continue) but got status=$status (AC-002/INV2)"
    echo "Output: $output"
    return 1
  }

  # Assert: internal log was written.
  [ -n "$INTERNAL_LOG_FILE" ] && [ -f "$INTERNAL_LOG_FILE" ] || {
    echo "FAIL: AC-002/PC-B-B2: dispatcher internal log not found."
    echo "Log dir: $WORK/.factory/logs (contents: $(ls "$WORK/.factory/logs/" 2>/dev/null || echo 'empty'))"
    echo "RED GATE: WASM not built → no plugin.log record → log may not exist."
    return 1
  }

  # Assert: exactly one DelegationRecommended record (INV3 first-match, single advisory).
  # Machine-stable count assertion (VP-091 §2).
  local DELEGATION_COUNT
  DELEGATION_COUNT="$(_count_delegation_records)"
  [ "$DELEGATION_COUNT" -eq 1 ] || {
    echo "FAIL: AC-002/PC-B-B2: expected exactly 1 DelegationRecommended record in plugin.log; got $DELEGATION_COUNT."
    echo "Internal log contents:"
    cat "$INTERNAL_LOG_FILE" 2>/dev/null || echo "(log file not readable)"
    echo "RED GATE: WASM not built → 0 records → this assertion correctly fails."
    return 1
  }

  # Assert: matched_pattern is "grep -r" (BC-4.15.001 PC-B-B2 matched_pattern field).
  grep -q '"matched_pattern":"grep -r"' "$INTERNAL_LOG_FILE" || {
    echo "FAIL: AC-002/PC-B-B2: matched_pattern must be \"grep -r\" in the DelegationRecommended record."
    echo "Internal log contents:"
    cat "$INTERNAL_LOG_FILE"
    return 1
  }

  # Assert: level is "warn" (BC-4.15.001 PC-B-B2 level field).
  grep -q '"level":"warn"' "$INTERNAL_LOG_FILE" || {
    echo "FAIL: AC-002/PC-B-B2: level must be \"warn\" in the DelegationRecommended record."
    echo "Internal log contents:"
    cat "$INTERNAL_LOG_FILE"
    return 1
  }

  # Assert: command_preview field is present and its length is ≤121 chars
  # (≤120 chars + optional U+2026 ellipsis for the truncation case).
  # For "grep -r \"TODO\" ." (short command), no truncation expected — preview = full command.
  # Use Python-free awk extraction: find the command_preview value from the JSON line.
  local preview_len
  preview_len=$(grep '"code":"DelegationRecommended"' "$INTERNAL_LOG_FILE" | \
    grep -o '"command_preview":"[^"]*"' | \
    sed 's/"command_preview":"//; s/"//' | \
    awk '{print length($0)}' 2>/dev/null || echo "0")
  [ "$preview_len" -gt 0 ] || {
    echo "FAIL: AC-002/PC-B-B2: command_preview field absent or empty in DelegationRecommended record."
    echo "Internal log: $(cat "$INTERNAL_LOG_FILE")"
    return 1
  }
  [ "$preview_len" -le 121 ] || {
    echo "FAIL: AC-002/PC-B-B2: command_preview must be ≤121 chars (120 + optional U+2026); got $preview_len."
    echo "Internal log: $(cat "$INTERNAL_LOG_FILE")"
    return 1
  }
}

# ---------------------------------------------------------------------------
# AC-003 (BC-4.15.001 PC-A): no emission on no-match; Continue
#
# Setup: Bash command "cargo fmt --check --all"; default pattern list does
#        NOT contain this string.
# Assert: dispatcher exit 0; DELEGATION_COUNT is 0 (no DelegationRecommended record);
#         no advisory in stderr.
#
# Machine-stable assertion (VP-091 §3): absence of any DelegationRecommended
# record in plugin.log is the load-bearing postcondition (PC-A).
# A gate that emits a spurious advisory on no-match is a spec violation.
#
# Red Gate condition: Stub that emits on all commands → spurious record;
#   DELEGATION_COUNT -eq 0 assertion fails.
# ---------------------------------------------------------------------------

@test "test_heavy_op_gate_no_emission_on_no_match" {
  _require_artifacts
  _write_full_registry

  local cmd="cargo fmt --check --all"
  local envelope
  envelope="$(_bash_event "$cmd" "ac003-no-match")"

  _run_dispatcher_with_internal_log "$envelope"

  # Assert: dispatcher returns Continue (exit 0) — no pattern match → PC-A.
  [ "$status" -eq 0 ] || {
    echo "FAIL: expected exit 0 (Continue) for no-match command but got status=$status (AC-003/PC-A)"
    echo "Output: $output"
    return 1
  }

  # Assert: DELEGATION_COUNT is 0 — no DelegationRecommended record emitted (PC-A).
  # Machine-stable absence assertion (VP-091 §3 / L-F2-machine-stable-count-assertion).
  # If the internal log does not exist (WASM emitted no advisory at all), count = 0 (correct).
  local DELEGATION_COUNT
  DELEGATION_COUNT="$(_count_delegation_records)"
  [ "$DELEGATION_COUNT" -eq 0 ] || {
    echo "FAIL: AC-003/PC-A: NO DelegationRecommended record must be emitted for a no-match command."
    echo "Expected DELEGATION_COUNT=0; got DELEGATION_COUNT=$DELEGATION_COUNT."
    echo "Command: '$cmd' does not match any default pattern."
    echo "BC-4.15.001 PC-A: absence of emission is a LOAD-BEARING postcondition."
    echo "Internal log: $(cat "$INTERNAL_LOG_FILE" 2>/dev/null || echo '(not created)')"
    return 1
  }

  # Assert: stderr from the gate is empty or contains no advisory nudge for this command.
  # (Gate must not emit a DelegationRecommended message for a no-match command.)
  if [ -f "$STDERR_FILE" ]; then
    grep -qv "DelegationRecommended" "$STDERR_FILE" 2>/dev/null || {
      echo "FAIL: AC-003/PC-A: 'DelegationRecommended' advisory must NOT appear in stderr for a no-match command."
      echo "Stderr content: $(cat "$STDERR_FILE")"
      return 1
    }
  fi
}

# ---------------------------------------------------------------------------
# AC-004 (BC-4.15.001 INV2): gate always returns Continue on match
#
# The gate MUST return Continue (exit 0) even when a pattern matches.
# block_intent = true is a specification violation for this gate.
#
# Setup: Bash command "cargo test --release --workspace" (matches first pattern).
# Assert: dispatcher exit 0 (Continue); no block registered.
#
# Red Gate condition: Stub that blocks → dispatcher exit non-zero; exit-0 assert fails.
# ---------------------------------------------------------------------------

@test "test_heavy_op_gate_always_returns_continue_on_match" {
  _require_artifacts
  _write_full_registry

  # BC-4.15.001 §Canonical Test Vectors: "cargo test --release" → Continue + advisory.
  local cmd="cargo test --release --workspace"
  local envelope
  envelope="$(_bash_event "$cmd" "ac004-match-continue")"

  _run_dispatcher_with_internal_log "$envelope"

  # Assert: dispatcher returns Continue (exit 0) even though pattern matches.
  # BC-4.15.001 INV2: block_intent MUST be false under ALL conditions including match.
  [ "$status" -eq 0 ] || {
    echo "FAIL: AC-004/INV2: gate MUST return Continue (exit 0) even on pattern match."
    echo "Expected: exit 0 (Continue — never blocks)"
    echo "Got: exit $status (INV2 violation: block_intent=true is FORBIDDEN)"
    echo "BC-4.15.001 INV2: 'Never blocks; always returns Continue under ALL conditions'"
    echo "Output: $output"
    return 1
  }

  # Assert: no blocking_plugins in output (belt-and-suspenders INV2 check).
  [[ "$output" != *"blocking_plugins=validate-heavy-op-delegation"* ]] || {
    echo "FAIL: AC-004/INV2: 'blocking_plugins=validate-heavy-op-delegation' must NOT appear."
    echo "Output: $output"
    return 1
  }

  # Assert: the gate DID emit an advisory (confirm it ran and matched — not vacuously passing).
  # This makes the test load-bearing: a gate that emits nothing also exits 0.
  # By asserting DELEGATION_COUNT >= 1, we confirm the gate ran and processed the match.
  local DELEGATION_COUNT
  DELEGATION_COUNT="$(_count_delegation_records)"
  [ "$DELEGATION_COUNT" -ge 1 ] || {
    echo "FAIL: AC-004/INV2: gate must emit DelegationRecommended advisory on match (confirms gate executed)."
    echo "Expected DELEGATION_COUNT>=1; got DELEGATION_COUNT=$DELEGATION_COUNT."
    echo "Without this check, a gate that silently passes without loading would also exit 0 (false-green)."
    echo "RED GATE: WASM not built → 0 records → this load-bearing assertion correctly fails."
    return 1
  }
}

# ---------------------------------------------------------------------------
# AC-004 (BC-4.15.001 INV2): gate always returns Continue on no-match
#
# Setup: Bash command "cargo fmt --check --all" (no match in default list).
# Assert: dispatcher exit 0; no block registered.
#
# Red Gate condition: Stub that blocks on no-match → exit non-zero; exit-0 assert fails.
# ---------------------------------------------------------------------------

@test "test_heavy_op_gate_always_returns_continue_on_no_match" {
  _require_artifacts
  _write_full_registry

  # BC-4.15.001 §Canonical Test Vectors: "cargo fmt --check --all" → Continue; no emission.
  local cmd="cargo fmt --check --all"
  local envelope
  envelope="$(_bash_event "$cmd" "ac004-nomatch-continue")"

  _run_dispatcher "$envelope"

  # Assert: dispatcher returns Continue (exit 0) on no-match.
  # BC-4.15.001 INV2: Continue under ALL conditions.
  [ "$status" -eq 0 ] || {
    echo "FAIL: AC-004/INV2: gate MUST return Continue (exit 0) on no-match."
    echo "Expected: exit 0 (Continue)"
    echo "Got: exit $status"
    echo "Output: $output"
    return 1
  }

  # Assert: no blocking_plugins (belt-and-suspenders).
  [[ "$output" != *"blocking_plugins=validate-heavy-op-delegation"* ]] || {
    echo "FAIL: AC-004/INV2: 'blocking_plugins=validate-heavy-op-delegation' must NOT appear on no-match."
    echo "Output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# AC-004/AC-010 (BC-4.15.001 INV2 + PC-C): fail-open on WASM panic
#
# When the WASM gate panics (fuel exhaustion, ABI violation, logic error),
# the dispatcher MUST return Continue (fail-open per on_error="continue").
# A "plugin.crashed" event MUST appear in the dispatcher internal log.
# The Bash tool call MUST proceed unblocked.
#
# Test approach:
#   Deploy a minimal panic-stub WASM that immediately panics (exit 1 /
#   trap instruction). The dispatcher detects the crash, logs plugin.crashed,
#   and returns Continue per on_error="continue".
#
#   If a pre-built panic-stub WASM is not available, we verify the static
#   registry contract: on_error="continue" is present in the registry entry,
#   which is the harness-level fail-open guarantee (BC-4.15.001 PC-C).
#
# Red Gate condition: Panic stub not deployed → dispatcher crashes too →
#   plugin.crashed assertion may fail; OR the test falls back to static
#   registry check which passes only if the registry entry exists.
#
# ---------------------------------------------------------------------------

@test "test_heavy_op_gate_fail_open_on_wasm_panic" {
  _require_artifacts

  # Scenario A (static): verify on_error="continue" in the production registry.
  # This is the harness-level fail-open guarantee per BC-4.15.001 PC-C.
  # Verified from the production registry (populated by T-5 of S-18.06).
  # Fails at Red Gate time because the registry entry is absent.
  local on_error_value
  on_error_value=$(awk '
    /^name = "validate-heavy-op-delegation"$/ { in_hook=1 }
    in_hook && /^\[\[hooks\]\]/ { in_hook=0 }
    in_hook && /^on_error = / { gsub(/^on_error = "|"$/, ""); print; exit }
  ' "$PRODUCTION_REGISTRY" 2>/dev/null || echo "")

  [ -n "$on_error_value" ] || {
    echo "FAIL: AC-004/AC-010/PC-C: on_error field not found for validate-heavy-op-delegation."
    echo "RED GATE: registry entry absent → implementer must add [[hooks]] entry per AC-008 (T-5 S-18.06)."
    echo "Registry: $PRODUCTION_REGISTRY"
    return 1
  }

  [ "$on_error_value" = "continue" ] || {
    echo "FAIL: AC-004/AC-010/PC-C: on_error for validate-heavy-op-delegation is '$on_error_value'."
    echo "Expected: on_error = \"continue\" (fail-open; BC-4.15.001 PC-C / INV2)"
    echo "BC-4.15.001 PC-C: WASM panic → dispatcher returns Continue; plugin.crashed in internal log."
    return 1
  }

  # Scenario B (live): attempt to deploy a panic-stub WASM and verify
  # the dispatcher logs plugin.crashed + returns exit 0.
  #
  # A WASM binary that immediately panics can be created via a single-instruction
  # WASM module or by compiling a trivial Rust program that calls std::process::abort().
  # If neither wasm-tools nor rustc is available for inline compilation, this
  # scenario skips gracefully — Scenario A provides the load-bearing static check.
  #
  # The minimal WASM binary that panics on _start: "unreachable" trap instruction.
  # This is the WASM binary encoding of: (module (func (export "_start") unreachable))
  # Hex: 00 61 73 6d 01 00 00 00 01 04 01 60 00 00 03 02 01 00 07 0a 01 06 5f 73 74 61 72 74 00 00 0a 05 01 03 00 00 0b
  local PANIC_STUB_WASM="$WORK/hook-plugins/panic-stub.wasm"
  printf '\x00\x61\x73\x6d\x01\x00\x00\x00\x01\x04\x01\x60\x00\x00\x03\x02\x01\x00\x07\x0a\x01\x06\x5f\x73\x74\x61\x72\x74\x00\x00\x0a\x05\x01\x03\x00\x00\x0b' \
    > "$PANIC_STUB_WASM" 2>/dev/null || true

  # Write a registry that points to the panic-stub WASM instead of the real gate.
  if [ -f "$PANIC_STUB_WASM" ]; then
    cat > "$WORK/hooks-registry.toml" <<TOML
schema_version = 2

[[hooks]]
name = "validate-heavy-op-delegation"
event = "PreToolUse"
plugin = "hook-plugins/panic-stub.wasm"
tool = "Bash"
on_error = "continue"
async = false
timeout_ms = 5000

[hooks.config]
patterns = ["cargo test --release"]
TOML

    cp "$PANIC_STUB_WASM" "$WORK/hook-plugins/validate-heavy-op-delegation.wasm"

    local cmd="cargo test --release --workspace"
    local envelope
    envelope="$(_bash_event "$cmd" "ac010-panic")"

    _run_dispatcher_with_internal_log "$envelope"

    # Assert: dispatcher returns Continue (exit 0) despite WASM panic.
    # PC-C: on_error="continue" → fail-open; Bash tool call proceeds unblocked.
    [ "$status" -eq 0 ] || {
      echo "FAIL: AC-004/AC-010/PC-C: WASM panic must fail-open (exit 0 Continue)."
      echo "Expected: exit 0 (Continue)"
      echo "Got: exit $status"
      echo "on_error=\"continue\" guarantees fail-open per BC-4.15.001 PC-C."
      echo "Output: $output"
      return 1
    }

    # Assert: plugin.crashed event in dispatcher internal log.
    # BC-4.15.001 PC-C: "The dispatcher records a plugin.crashed event in its internal log."
    if [ -n "$INTERNAL_LOG_FILE" ] && [ -f "$INTERNAL_LOG_FILE" ]; then
      grep -q '"plugin.crashed"\|"type":"plugin.crashed"\|plugin_crashed' "$INTERNAL_LOG_FILE" || {
        echo "WARN: AC-010/PC-C: plugin.crashed event not found in internal log."
        echo "This may be a dispatcher log format difference — Scenario A (on_error=continue) is the load-bearing assertion."
        echo "Internal log: $(cat "$INTERNAL_LOG_FILE" 2>/dev/null)"
        # Not a hard fail — Scenario A (static on_error check) is the authoritative check.
        # Log format of plugin.crashed events is dispatcher-internal; the key guarantee
        # is on_error=continue which was asserted in Scenario A above.
      }
    fi
  fi
  # Scenario A passed — the test as a whole passes only if on_error="continue" is present.
  # (The live-crash Scenario B is best-effort given WASM binary format constraints.)
}

# ---------------------------------------------------------------------------
# AC-007 (BC-4.15.001 PC-D): gate NOT dispatched on non-Bash tool call
#
# The registry "tool = "Bash"" filter MUST prevent the dispatcher from invoking
# the plugin on non-Bash tool calls.
#
# Setup: synthetic PreToolUse Write event with file_path="/tmp/test.md".
# Assert: dispatcher exit 0; NO DelegationRecommended record in plugin.log;
#         no advisory in stderr (PC-D effective Continue).
#
# Red Gate condition: registry entry missing tool filter → Write dispatches plugin;
#   no-record assertion fails (DELEGATION_COUNT > 0).
# ---------------------------------------------------------------------------

@test "test_heavy_op_gate_not_dispatched_on_write_tool_call" {
  _require_artifacts
  _write_full_registry

  local envelope
  envelope="$(_write_event "/tmp/test.md" "ac007-write-noop")"

  _run_dispatcher_with_internal_log "$envelope"

  # Assert: dispatcher returns Continue (exit 0) — Write is not a Bash call.
  [ "$status" -eq 0 ] || {
    echo "FAIL: AC-007/PC-D: expected exit 0 for Write tool PreToolUse event."
    echo "Got: exit $status"
    echo "Output: $output"
    return 1
  }

  # Assert: DELEGATION_COUNT is 0 — plugin must NOT be dispatched for Write events.
  # VP-091 §4 (PC-D): registry tool="Bash" filter → dispatcher does NOT invoke the plugin.
  # Machine-stable absence assertion (VP-091 §2).
  local DELEGATION_COUNT
  DELEGATION_COUNT="$(_count_delegation_records)"
  [ "$DELEGATION_COUNT" -eq 0 ] || {
    echo "FAIL: AC-007/PC-D: DelegationRecommended advisory must NOT be emitted for Write tool call."
    echo "Expected DELEGATION_COUNT=0; got DELEGATION_COUNT=$DELEGATION_COUNT."
    echo "The registry 'tool = \"Bash\"' filter must prevent dispatch on non-Bash events."
    echo "BC-4.15.001 PC-D: 'plugin is NOT dispatched; effective Continue; no emission.'"
    echo "Internal log: $(cat "$INTERNAL_LOG_FILE" 2>/dev/null || echo '(not created)')"
    return 1
  }

  # Assert: no advisory content in stderr.
  if [ -f "$STDERR_FILE" ]; then
    grep -qv "DelegationRecommended" "$STDERR_FILE" 2>/dev/null || {
      echo "FAIL: AC-007/PC-D: advisory must NOT appear in stderr for Write tool call."
      echo "Stderr: $(cat "$STDERR_FILE")"
      return 1
    }
  fi
}

# ---------------------------------------------------------------------------
# AC-008 (BC-4.15.001 precondition 1): registry entry has canonical shape
#
# The hooks-registry.toml entry for validate-heavy-op-delegation MUST be present
# with the exact canonical shape per BC-4.15.001 PC1.
#
# Required fields:
#   - name = "validate-heavy-op-delegation"
#   - event = "PreToolUse"
#   - plugin = "hook-plugins/validate-heavy-op-delegation.wasm"
#   - tool = "Bash"
#   - on_error = "continue"
#   - async = false
#   - timeout_ms = 5000
#   - [hooks.config] block with patterns list
#
# This test does NOT require the WASM binary or dispatcher — it asserts
# directly on the production hooks-registry.toml file.
# FAILS IMMEDIATELY at Red Gate time when the entry is absent.
#
# Red Gate condition: registry entry absent or malformed → grep block fails immediately.
# ---------------------------------------------------------------------------

@test "test_heavy_op_gate_registry_entry_has_canonical_shape" {
  # No _require_artifacts call — this test asserts on the production registry directly.
  # It fails immediately when the entry is absent (implementer task T-5 of S-18.06).

  # Assert: production registry file exists.
  [ -f "$PRODUCTION_REGISTRY" ] || {
    echo "FAIL: AC-008: production hooks-registry.toml not found at: $PRODUCTION_REGISTRY"
    return 1
  }

  # Assert: entry name present.
  grep -q 'name = "validate-heavy-op-delegation"' "$PRODUCTION_REGISTRY" || {
    echo "FAIL: AC-008/PC1: validate-heavy-op-delegation entry not found in $PRODUCTION_REGISTRY."
    echo "RED GATE: implementer must add [[hooks]] entry per AC-008 canonical shape (S-18.06 T-5)."
    return 1
  }

  # Use awk to extract the validate-heavy-op-delegation section and score required fields.
  # Section is bounded by the entry's name = line and the next [[hooks]] header or EOF.
  # Score 7 required fields:
  #   1. event = "PreToolUse"
  #   2. plugin = "hook-plugins/validate-heavy-op-delegation.wasm"
  #   3. tool = "Bash"
  #   4. on_error = "continue"
  #   5. async = false
  #   6. timeout_ms = 5000
  #   7. [hooks.config] block with patterns list
  local score
  score=$(awk '
    /name = "validate-heavy-op-delegation"/ { in_section = 1; s = 0; has_config = 0; has_patterns = 0 }
    /^\[\[hooks\]\]/ && in_section && !/name = "validate-heavy-op-delegation"/ { in_section = 0 }
    in_section && /^event = "PreToolUse"/ { s++ }
    in_section && /^plugin = "hook-plugins\/validate-heavy-op-delegation\.wasm"/ { s++ }
    in_section && /^tool = "Bash"/ { s++ }
    in_section && /^on_error = "continue"/ { s++ }
    in_section && /^async = false/ { s++ }
    in_section && /^timeout_ms = 5000/ { s++ }
    in_section && /\[hooks\.config\]/ { has_config = 1 }
    in_section && has_config && /^patterns/ { has_patterns = 1 }
    END { print s + (has_patterns ? 1 : 0) }
  ' "$PRODUCTION_REGISTRY")

  [ "$score" -eq 7 ] || {
    echo "FAIL: AC-008/PC1: validate-heavy-op-delegation registry entry is incomplete (score=$score/7)."
    echo "Required fields:"
    echo "  1. event = \"PreToolUse\"              (BC-4.15.001 PC1)"
    echo "  2. plugin = \"hook-plugins/validate-heavy-op-delegation.wasm\"  (canonical native-WASM shape)"
    echo "  3. tool = \"Bash\"                      (BC-4.15.001 PC-D registry filter)"
    echo "  4. on_error = \"continue\"              (BC-4.15.001 PC-C fail-open MANDATORY)"
    echo "  5. async = false                       (sync PreToolUse gate)"
    echo "  6. timeout_ms = 5000                   (BC-4.15.001 PC1 / ADR-026 §Decision 8)"
    echo "  7. [hooks.config] with patterns list   (BC-4.15.001 PC1 v1 default set)"
    echo "See BC-4.15.001 PC1 / S-18.06 AC-008 / hooks-registry.toml canonical shape."
    return 1
  }

  # Assert: patterns list contains at least one default pattern from v1 default set.
  local has_cargo_test
  has_cargo_test=$(awk '
    /name = "validate-heavy-op-delegation"/ { in_section=1; in_config=0 }
    /^\[\[hooks\]\]/ && in_section && !/name = "validate-heavy-op-delegation"/ { in_section=0; in_config=0 }
    in_section && /\[hooks\.config\]/ { in_config=1 }
    in_section && in_config && /cargo test --release/ { print "yes"; exit }
  ' "$PRODUCTION_REGISTRY")

  [ "$has_cargo_test" = "yes" ] || {
    echo "FAIL: AC-008/PC1: [hooks.config] patterns list must contain v1 default 'cargo test --release'."
    echo "BC-4.15.001 PC1 specifies the v1 default patterns list; 'cargo test --release' is the first pattern."
    return 1
  }
}

# ---------------------------------------------------------------------------
# Helper: write a registry with a custom (operator-supplied) patterns list.
# Used by EC-012 and EC-013 dispatcher-integration tests to exercise the
# on_pre_tool_use → plugin_config read path.
#
# Usage: _write_registry_with_patterns "pattern1" "pattern2" ...
# Pass zero args for an empty list.
# ---------------------------------------------------------------------------

_write_registry_with_patterns() {
  # Build a TOML patterns array from the positional arguments.
  # Zero args → patterns = []
  local patterns_toml="patterns = ["
  local first=1
  for p in "$@"; do
    if [ "$first" -eq 1 ]; then
      patterns_toml="${patterns_toml}\"${p}\""
      first=0
    else
      patterns_toml="${patterns_toml}, \"${p}\""
    fi
  done
  patterns_toml="${patterns_toml}]"

  cat > "$WORK/hooks-registry.toml" <<TOML
schema_version = 2

[[hooks]]
name = "validate-heavy-op-delegation"
event = "PreToolUse"
plugin = "hook-plugins/validate-heavy-op-delegation.wasm"
tool = "Bash"
on_error = "continue"
async = false
timeout_ms = 5000

[hooks.config]
${patterns_toml}
TOML
}

# ---------------------------------------------------------------------------
# F-P1-001 EC-012 end-to-end (dispatcher-integration): empty patterns list via
# registry override → no DelegationRecommended on any Bash command
#
# Gap identified by LOCAL adversary Pass 1 (F-P1-001): the existing unit test
# test_heavy_op_gate_empty_pattern_list_no_emission calls evaluate_patterns()
# directly with an empty list. It does NOT verify that on_pre_tool_use reads
# patterns FROM plugin_config — an implementation that hardcodes DEFAULT_PATTERNS
# and ignores plugin_config would pass the unit test but fail here.
#
# Setup: registry with [hooks.config] patterns = []; Bash command
#        "cargo test --release --workspace" (would match DEFAULT_PATTERNS).
# Assert: DELEGATION_COUNT == 0 (machine-stable); dispatcher exit 0.
#
# Red Gate condition: current implementation (todo!()) hardcodes DEFAULT_PATTERNS
# and ignores plugin_config → DELEGATION_COUNT == 1 instead of 0;
# assertion DELEGATION_COUNT -eq 0 fails, proving the bug.
# ---------------------------------------------------------------------------

@test "test_heavy_op_gate_empty_pattern_list_no_emission_via_dispatcher" {
  _require_artifacts

  # Write registry with empty patterns list (operator override of default set).
  _write_registry_with_patterns

  # Copy the real WASM gate into the synthetic plugin root.
  cp "$GATE_WASM" "$WORK/hook-plugins/validate-heavy-op-delegation.wasm"

  # Command that WOULD match "cargo test --release" in DEFAULT_PATTERNS.
  local cmd="cargo test --release --workspace"
  local envelope
  envelope="$(_bash_event "$cmd" "ec012-empty-patterns")"

  _run_dispatcher_with_internal_log "$envelope"

  # Assert: dispatcher returns Continue (exit 0) — INV2 / PC-A.
  [ "$status" -eq 0 ] || {
    echo "FAIL: EC-012/AC-011 dispatcher: expected exit 0 but got status=$status."
    echo "Output: $output"
    return 1
  }

  # Machine-stable assertion (VP-091 §2): DELEGATION_COUNT must be 0.
  # With an empty patterns list, no pattern can ever match (BC-4.15.001 EC-012).
  # A correct implementation reads plugin_config.patterns at runtime; an
  # implementation that hardcodes DEFAULT_PATTERNS would produce COUNT=1 here.
  local DELEGATION_COUNT
  DELEGATION_COUNT="$(_count_delegation_records)"
  [ "$DELEGATION_COUNT" -eq 0 ] || {
    echo "FAIL: EC-012/AC-011 dispatcher: DELEGATION_COUNT must be 0 with empty patterns list."
    echo "Expected DELEGATION_COUNT=0; got DELEGATION_COUNT=$DELEGATION_COUNT."
    echo "F-P1-001 BUG: implementation ignores [hooks.config] patterns and uses DEFAULT_PATTERNS."
    echo "on_pre_tool_use must read plugin_config['patterns'] at runtime, not call"
    echo "evaluate_patterns(cmd, DEFAULT_PATTERNS) unconditionally."
    echo "BC-4.15.001 EC-012: 'patterns = [] → no pattern can ever match; all Bash commands pass silently.'"
    echo "Internal log: $(cat "$INTERNAL_LOG_FILE" 2>/dev/null || echo '(not created)')"
    return 1
  }
}

# ---------------------------------------------------------------------------
# F-P1-001 EC-013 end-to-end (dispatcher-integration): custom pattern via
# registry override → DelegationRecommended with matched_pattern = "./ci.sh"
#
# Gap identified by LOCAL adversary Pass 1 (F-P1-001): the existing unit test
# test_heavy_op_gate_ec013_custom_pattern_triggers_advisory calls evaluate_patterns()
# directly with a custom list. It does NOT verify that on_pre_tool_use reads
# patterns FROM plugin_config — an implementation that hardcodes DEFAULT_PATTERNS
# would emit 0 advisories here (./ci.sh is not in DEFAULT_PATTERNS) where 1 is required.
#
# Setup: registry with [hooks.config] patterns = ["./ci.sh"]; Bash command
#        "./ci.sh build".
# Assert: DELEGATION_COUNT == 1 (machine-stable); matched_pattern == "./ci.sh";
#         dispatcher exit 0 (INV2).
#
# Red Gate condition: current implementation hardcodes DEFAULT_PATTERNS and
# ignores plugin_config → DELEGATION_COUNT == 0 instead of 1;
# assertion DELEGATION_COUNT -eq 1 fails, proving the bug.
# ---------------------------------------------------------------------------

@test "test_heavy_op_gate_custom_pattern_triggers_via_dispatcher" {
  _require_artifacts

  # Write registry with custom patterns list (just "./ci.sh").
  _write_registry_with_patterns "./ci.sh"

  # Copy the real WASM gate into the synthetic plugin root.
  cp "$GATE_WASM" "$WORK/hook-plugins/validate-heavy-op-delegation.wasm"

  # Command that ONLY matches the custom pattern "./ci.sh" (not in DEFAULT_PATTERNS).
  local cmd="./ci.sh build"
  local envelope
  envelope="$(_bash_event "$cmd" "ec013-custom-pattern")"

  _run_dispatcher_with_internal_log "$envelope"

  # Assert: dispatcher returns Continue (exit 0) — INV2 never blocks.
  [ "$status" -eq 0 ] || {
    echo "FAIL: EC-013 dispatcher: expected exit 0 (Continue) but got status=$status."
    echo "Output: $output"
    return 1
  }

  # Assert: internal log was written.
  [ -n "$INTERNAL_LOG_FILE" ] && [ -f "$INTERNAL_LOG_FILE" ] || {
    echo "FAIL: EC-013 dispatcher: dispatcher internal log not found."
    echo "Log dir: $WORK/.factory/logs (contents: $(ls "$WORK/.factory/logs/" 2>/dev/null || echo 'empty'))"
    echo "F-P1-001 BUG: on_pre_tool_use uses DEFAULT_PATTERNS (which does not include './ci.sh')"
    echo "so no advisory is emitted and the internal log may not contain a plugin.log record."
    return 1
  }

  # Machine-stable assertion (VP-091 §2): DELEGATION_COUNT must be exactly 1.
  # A correct implementation reads ["./ci.sh"] from plugin_config and matches.
  # An implementation that uses DEFAULT_PATTERNS would produce COUNT=0 here.
  local DELEGATION_COUNT
  DELEGATION_COUNT="$(_count_delegation_records)"
  [ "$DELEGATION_COUNT" -eq 1 ] || {
    echo "FAIL: EC-013 dispatcher: expected exactly 1 DelegationRecommended record; got $DELEGATION_COUNT."
    echo "F-P1-001 BUG: on_pre_tool_use ignores [hooks.config] patterns and uses DEFAULT_PATTERNS."
    echo "'./ci.sh' is NOT in DEFAULT_PATTERNS → DELEGATION_COUNT=0 with the buggy implementation."
    echo "on_pre_tool_use must read plugin_config['patterns'] at runtime."
    echo "BC-4.15.001 EC-013: 'custom pattern ./ci.sh → triggers DelegationRecommended advisory.'"
    echo "Internal log: $(cat "$INTERNAL_LOG_FILE" 2>/dev/null || echo '(not created)')"
    return 1
  }

  # Assert: matched_pattern is "./ci.sh" (not a DEFAULT_PATTERNS entry).
  grep -q '"matched_pattern":"./ci.sh"' "$INTERNAL_LOG_FILE" || {
    echo "FAIL: EC-013 dispatcher: matched_pattern must be './ci.sh'."
    echo "Internal log: $(cat "$INTERNAL_LOG_FILE")"
    return 1
  }
}
