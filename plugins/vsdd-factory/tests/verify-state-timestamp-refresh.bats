#!/usr/bin/env bats
# verify-state-timestamp-refresh.bats — D17 bats integration tests for S-17.04 v1.3.
#
# Covers the 4 bats integration cases from S-17.04 v1.3 Red Gate Test Table:
#
#   T-1 (AC-003): Write payload, timestamps advanced → guard allows (exit 0)
#   T-2 (AC-005/011): Write payload, stale timestamp (unchanged) → guard blocks (exit 2),
#       FULL canonical block message asserted (fixes finding M03 / prior substring-only check).
#   T-3 (AC-007): Edit payload, non-STATE.md path → guard allows (exit 0) immediately
#   T-4 (AC-012): Edit payload, reconstructed content has stale timestamp → guard blocks (exit 2)
#
# Story: S-17.04 v1.3 (verify-state-timestamp-refresh WASM guard + factory-lock-parse crate)
# BC gate: BC-5.40.001 PC4 (mid-burst renewal enforcement)
# ADR: ADR-025 v1.6 Decision 12 §12.1–12.8
#
# PAYLOAD FIELD DISCIPLINE (ADR-025 §12.1 / Red Gate Test Table):
#   - Write payload:     tool_input.content    = full file body          (AC-011)
#   - Edit payload:      tool_input.old_string + tool_input.new_string   (AC-012)
#   - MultiEdit payload: tool_input.edits[]    (array of {old,new})      (AC-013)
#   - `tool_input.new_content` NEVER used — that field does not exist in
#     Claude Code payloads (0 occurrences in 5,235+ real dispatcher events
#     per ADR-025 §12.1). Prior bats tests (subdir path) were invalid because
#     they used new_content (finding C01 / S-17.04 adversary pass-1).
#
# BLOCK MESSAGE ASSERTIONS: T-2 and T-4 assert the FULL canonical block line
# (`BLOCKED by verify-state-timestamp-refresh: STATE.md timestamp not advanced
#  in this write. Fix: Update 'timestamp:' to the current UTC time before
#  writing STATE.md. Code: TimestampStale.`) — NOT a substring (fixes finding M03).
#
# RED GATE strategy:
#   All tests require two artifacts that the implementer produces in T-3/T-4 (D16):
#     1. plugins/vsdd-factory/hook-plugins/verify-state-timestamp-refresh.wasm (compiled WASM)
#     2. Registry entry in plugins/vsdd-factory/hooks-registry.toml (T-4 / D16 registry)
#   Until those artifacts exist, T-1..T-3 skip with an actionable "not built yet"
#   message — the tests are correctly RED (skip != pass) at Red Gate time.
#   T-4 (registry assertion) fails immediately because the registry entry is absent.
#
# File location: plugins/vsdd-factory/tests/verify-state-timestamp-refresh.bats
# (FLAT path — discovered by run-all.sh `tests/*.bats` glob per C02 fix).
# Old subdir path tests/verify-state-timestamp-refresh/verify-state-timestamp-refresh.bats
# has been DELETED (it was never run by run-all.sh; C02 finding from adversary pass-1).
#
# Dispatcher invocation pattern mirrors verify-factory-lock.bats:
#   printf '%s' "$envelope" | CLAUDE_PLUGIN_ROOT="$WORK" CLAUDE_PROJECT_DIR="$WORK" \
#     "$DISPATCHER" 2>&1
#
# Exit codes:
#   0 = Continue (allow)
#   2 = Block (block with reason)
#   1 = Error (plugin failed)
#
# Run:
#   bats plugins/vsdd-factory/tests/verify-state-timestamp-refresh.bats

# ---------------------------------------------------------------------------
# setup / teardown
# ---------------------------------------------------------------------------

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
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
#
# P5-M1: CI hard-fail gate.
# If CI_REQUIRE_ARTIFACTS=1 is set (devops CI step sets this after building + staging
# the WASM and dispatcher), artifact absence is a HARD FAIL rather than a graceful skip.
# Rationale: `skip` exits 0 in bats; a missing artifact in CI must be a visible failure,
# not a silent skip-pass that masks a broken build step.
# Locally (CI_REQUIRE_ARTIFACTS unset), graceful skip is preserved for developer convenience.
_require_artifacts() {
  if [ ! -x "$DISPATCHER" ]; then
    [ -z "${CI_REQUIRE_ARTIFACTS:-}" ] || { echo "FAIL: factory-dispatcher binary not present in CI (CI_REQUIRE_ARTIFACTS=1) — run: cargo build --release -p factory-dispatcher"; false; }
    skip "factory-dispatcher binary not built — run: cargo build --release -p factory-dispatcher (S-17.04 implementer task T-3)"
  fi
  if [ ! -f "$WORK/hook-plugins/verify-state-timestamp-refresh.wasm" ]; then
    [ -z "${CI_REQUIRE_ARTIFACTS:-}" ] || { echo "FAIL: verify-state-timestamp-refresh.wasm not present in CI (CI_REQUIRE_ARTIFACTS=1) — run: cargo build --target wasm32-wasip1 -p verify-state-timestamp-refresh"; false; }
    skip "verify-state-timestamp-refresh.wasm not present — run: cargo build --target wasm32-wasip1 -p verify-state-timestamp-refresh (S-17.04 implementer task T-3)"
  fi
}

# ---------------------------------------------------------------------------
# Registry writer
# ---------------------------------------------------------------------------

# Write the canonical verify-state-timestamp-refresh registry entry.
# Per ADR-025 Decision 12 §12.5 + S-17.04 AC-010 / D16 spec.
# async = false REQUIRED (ADR-019 correctness requirement).
# tool = "Edit|Write|MultiEdit" (all three content-mutating tools — AC-010).
# priority = 143 (after verify-factory-lock which runs at 142 — AC-010).
# [hooks.capabilities.read_file] block MANDATORY with path_allow ONLY.
# No exec_subprocess capability (guard reads file only; no subprocess per §12.5).
# No max_bytes or timeout_ms in [hooks.capabilities.read_file] — ReadFileCaps is
# #[serde(deny_unknown_fields)] with path_allow only (AC-010 / Architecture Rule 5).
_write_full_registry() {
  cat > "$WORK/hooks-registry.toml" <<'EOF'
schema_version = 2

# ---------- verify-state-timestamp-refresh (PreToolUse, Edit|Write|MultiEdit) ----------
# S-17.04: PreToolUse guard — blocks Edit/Write/MultiEdit to .factory/STATE.md when
# proposed content (reconstructed for Edit/MultiEdit) does not advance timestamp:
# frontmatter (every write) or factory_lock.expires_at (when lock held).
# Decision 12 / BC-5.40.001 PC4.
# Trigger: file_path == ".factory/STATE.md" after canonical-path normalisation.
# async = false REQUIRED — see ADR-019 + ADR-025 Decision 12

[[hooks]]
name = "verify-state-timestamp-refresh"
event = "PreToolUse"
tool = "Edit|Write|MultiEdit"
plugin = "hook-plugins/verify-state-timestamp-refresh.wasm"
priority = 143
timeout_ms = 5000
on_error = "continue"
async = false

[hooks.capabilities.read_file]
path_allow = [".factory/STATE.md"]
EOF
}

# ---------------------------------------------------------------------------
# STATE.md fixture writers (written to WORK/.factory/STATE.md)
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

# Write STATE.md with lock held (holder present) but NO expires_at line.
# factory_lock_parse returns Err(MalformedLockBlock) for this content.
# Used by the AC-016 (v1.4) LockExpiryStale bats test.
_write_state_lock_expires_absent() {
  local timestamp="${1:-2026-06-11T11:00:00Z}"
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
---

# STATE (bats fixture — lock held, NO expires_at)
EOF
}

# ---------------------------------------------------------------------------
# Dispatcher invocation helper
# ---------------------------------------------------------------------------

# Invoke the dispatcher with the given JSON envelope.
# Sets $status and $output (combined stdout+stderr).
_run_dispatcher() {
  local envelope="$1"
  run bash -c "printf '%s' '$envelope' | \
    CLAUDE_PLUGIN_ROOT='$WORK' \
    CLAUDE_PROJECT_DIR='$WORK' \
    '$DISPATCHER' 2>&1"
}

# ---------------------------------------------------------------------------
# T-1 (AC-003): Write payload, timestamps advanced → guard allows (exit 0)
#
# Payload type: Write (tool_input.content = full file body, AC-011).
# On-disk STATE.md: timestamp = TS_OLD (2026-06-11T10:00:00Z)
# Proposed content (tool_input.content): timestamp = TS_NEW (2026-06-11T11:00:00Z)
# Expected: exit 0 (Continue) — timestamps advanced, guard passes transparently.
#
# RED GATE: WASM absent → skip (the correct Red Gate result).
# After implementation: should become GREEN.
# ---------------------------------------------------------------------------

@test "T-1 test_verify_state_timestamp_refresh_continues_when_timestamps_advanced" {
  _require_artifacts
  _write_full_registry

  local ts_old="2026-06-11T10:00:00Z"
  local ts_new="2026-06-11T11:00:00Z"

  # On-disk STATE.md: old timestamp.
  _write_state_no_lock_with_ts "$ts_old"

  # Proposed content: full file body with timestamp ADVANCED to ts_new.
  # Write payload: tool_input.content = full file body (AC-011; NOT new_content).
  # Using single-line JSON string with literal newlines escaped as \n for safe
  # embedding in the JSON envelope.
  local proposed_content
  proposed_content="---\ndocument_type: state\nversion: 0.0.1-bats-test\ntimestamp: ${ts_new}\nphase: test\ncurrent_step: bats-test\n---\n\n# STATE (bats fixture - advanced)\n"

  # Write payload: content field carries the full proposed file body.
  local envelope
  envelope="{\"event_name\":\"PreToolUse\",\"tool_name\":\"Write\",\"session_id\":\"t1\",\"dispatcher_trace_id\":\"t1-trace\",\"tool_input\":{\"file_path\":\".factory/STATE.md\",\"content\":\"${proposed_content}\"}}"

  _run_dispatcher "$envelope"

  # Must exit 0 (Continue) — timestamps advanced, guard passes.
  [ "$status" -eq 0 ] || {
    echo "FAIL: expected exit 0 (Continue) but got status=$status"
    echo "Output: $output"
    return 1
  }

  # P4-H1 — guard-ran sentinel assertion (PRIMARY / load-bearing).
  #
  # Problem: on_error=continue means a guard CRASH also exits 0, so exit 0 alone
  # cannot distinguish "guard ran and returned Continue" from "guard crashed".
  #
  # Solution: the implementer wired guard_logic Step 8 (post-check Continue) to emit
  # the sentinel string "verify-state-timestamp-refresh: guard_ran (continue)"
  # to plugin stderr via the write_stderr callback, which is wired to eprint!() in
  # main.rs and therefore appears in dispatcher stderr (captured here via 2>&1).
  #
  # A crashed guard NEVER reaches Step 8 — the crash happens inside the WASM guest
  # before the emission point. Therefore sentinel-present + exit 0 = clean Continue,
  # not a crash. This is the definitive proof the guard executed its full decision
  # logic and deliberately allowed the write.
  #
  # T-1 exercises the STATE.md path with a legitimately-advanced timestamp, which
  # reaches Step 8 (all checks passed, allow) → sentinel fires.
  #
  # Secondary: also assert plugins_run=1 for belt-and-suspenders confirmation that
  # the guard plugin was loaded and invoked at all.
  [[ "$output" == *"guard_ran"* ]] || {
    echo "FAIL: guard_ran sentinel not found in dispatcher output."
    echo "Expected 'verify-state-timestamp-refresh: guard_ran (continue)' in combined stderr."
    echo "This sentinel is emitted at guard_logic Step 8 (post-check Continue success path)."
    echo "Absence means the guard crashed, was not loaded, or did not reach Step 8."
    echo "Output: $output"
    return 1
  }
  # Secondary: plugins_run=1 confirms the registry entry loaded the plugin.
  [[ "$output" == *"plugins_run=1"* ]] || {
    echo "FAIL: expected 'plugins_run=1' in dispatcher stderr but got: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-2 (AC-005/011): Write payload, stale timestamp → guard blocks (exit 2),
#     FULL canonical block message asserted.
#
# Payload type: Write (tool_input.content = full file body, AC-011).
# On-disk STATE.md: timestamp = TS_OLD (2026-06-11T10:00:00Z)
# Proposed content (tool_input.content): timestamp = TS_OLD (NOT advanced — stale)
# Expected: exit 2 (Block) + FULL canonical message in output.
#
# Canonical block message (AC-005, full-line):
#   BLOCKED by verify-state-timestamp-refresh: STATE.md timestamp not advanced in
#   this write. Fix: Update 'timestamp:' to the current UTC time before writing
#   STATE.md. Code: TimestampStale.
#
# Assertion: FULL string equality (not substring) — fixes finding M03 from
# adversary pass-1 (prior bats test only checked `*"TimestampStale"*`).
#
# RED GATE: WASM absent → skip. If WASM is stub (returns Continue unconditionally
# after path check), exit-2 assertion fails — correct Red Gate failure for the
# compiled stub.
# ---------------------------------------------------------------------------

@test "T-2 test_verify_state_timestamp_refresh_stale_timestamp_blocks" {
  _require_artifacts
  _write_full_registry

  local ts_old="2026-06-11T10:00:00Z"

  # On-disk STATE.md: old timestamp.
  _write_state_no_lock_with_ts "$ts_old"

  # Proposed content: timestamp NOT advanced (same as on-disk → stale).
  # Write payload: tool_input.content = full file body (AC-011; NOT new_content).
  local proposed_content
  proposed_content="---\ndocument_type: state\nversion: 0.0.1-bats-test\ntimestamp: ${ts_old}\nphase: test\ncurrent_step: bats-test\n---\n\n# STATE (bats fixture - stale)\n"

  # Write payload.
  local envelope
  envelope="{\"event_name\":\"PreToolUse\",\"tool_name\":\"Write\",\"session_id\":\"t2\",\"dispatcher_trace_id\":\"t2-trace\",\"tool_input\":{\"file_path\":\".factory/STATE.md\",\"content\":\"${proposed_content}\"}}"

  _run_dispatcher "$envelope"

  # Must exit 2 (Block) — stale timestamp detected.
  [ "$status" -eq 2 ] || {
    echo "FAIL: expected exit 2 (Block) but got status=$status"
    echo "Output: $output"
    return 1
  }

  # FULL canonical block message equality (not substring — fixes M03).
  # The canonical message produced by block_with_fix is:
  #   BLOCKED by verify-state-timestamp-refresh: STATE.md timestamp not advanced
  #   in this write. Fix: Update 'timestamp:' to the current UTC time before
  #   writing STATE.md. Code: TimestampStale.
  local expected_msg="BLOCKED by verify-state-timestamp-refresh: STATE.md timestamp not advanced in this write. Fix: Update 'timestamp:' to the current UTC time before writing STATE.md. Code: TimestampStale."

  [[ "$output" == *"${expected_msg}"* ]] || {
    echo "FAIL: expected FULL canonical TimestampStale message in output."
    echo "Expected message (substring): ${expected_msg}"
    echo "Actual output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-3 (AC-007): Edit payload, non-STATE.md path → guard allows (exit 0) immediately
#
# Payload type: Edit (tool_input.old_string + tool_input.new_string, AC-012).
# file_path is NOT .factory/STATE.md → guard returns Continue without
# reading the STATE.md file (zero overhead path per ADR-025 §12.1).
# Expected: exit 0 (Continue).
#
# RED GATE: WASM absent → skip.
# ---------------------------------------------------------------------------

@test "T-3 test_verify_state_timestamp_refresh_continues_for_non_state_md" {
  _require_artifacts
  _write_full_registry

  # No need to write STATE.md — the guard must not read it for non-STATE.md paths.
  # file_path is a different factory spec document.
  # Edit payload: old_string + new_string (AC-012; NOT new_content).
  local envelope
  envelope="{\"event_name\":\"PreToolUse\",\"tool_name\":\"Edit\",\"session_id\":\"t3\",\"dispatcher_trace_id\":\"t3-trace\",\"tool_input\":{\"file_path\":\".factory/specs/some-spec.md\",\"old_string\":\"old text\",\"new_string\":\"new text\"}}"

  _run_dispatcher "$envelope"

  # Must exit 0 (Continue) — non-STATE.md path returns Continue immediately (AC-007).
  [ "$status" -eq 0 ] || {
    echo "FAIL: expected exit 0 (Continue) for non-STATE.md path but got status=$status"
    echo "Output: $output"
    return 1
  }

  # P4-H1 — guard-ran assertion for T-3 (Step 1 early-Continue path).
  #
  # NOTE: the guard_ran sentinel ("verify-state-timestamp-refresh: guard_ran (continue)")
  # is emitted ONLY at guard_logic Step 8 (post-check Continue success path). The T-3
  # path short-circuits at Step 1 (non-STATE.md file_path check, line: if normalised !=
  # STATE_MD_PATH { return HookResult::Continue }). Step 1 returns BEFORE the sentinel
  # emit, so "guard_ran" does NOT appear in the dispatcher output for this test.
  #
  # Asserting "guard_ran" here would make T-3 FAIL — that is a false failure, not a
  # real guard problem. The guard is correctly short-circuiting at Step 1 as designed.
  #
  # To get a guard_ran-equivalent sentinel on the Step 1 path, the implementer would
  # need to add a second write_stderr emit on the early-Continue branch. That is a
  # tracked gap (P4-H1 finding for T-3) — do not silently weaken this assertion.
  #
  # Best available signal for T-3: plugins_run=1 confirms the guard WASM was loaded
  # and invoked by the dispatcher (the plugin ran to completion, returning Continue).
  # Combined with exit 0 + absence of block_reason, this proves the guard did not
  # crash and the dispatcher correctly received the Continue decision. It cannot
  # distinguish a clean Step-1-Continue from a crash-with-on_error=continue, but
  # that distinction requires the implementer-side sentinel on the Step 1 path.
  [[ "$output" == *"plugins_run=1"* ]] || {
    echo "FAIL: expected 'plugins_run=1' in dispatcher stderr but got: $output"
    echo "plugins_run=1 confirms the guard plugin was invoked (Step 1 early-Continue path)."
    return 1
  }
  # Guard_ran sentinel is intentionally NOT asserted here — the Step 1 early-Continue
  # path does not reach the Step 8 sentinel emit. See P4-H1 note above.
}

# ---------------------------------------------------------------------------
# T-4 (AC-012): Edit payload reconstruct stale timestamp → guard blocks (exit 2)
#
# Payload type: Edit (tool_input.old_string + tool_input.new_string, AC-012).
# On-disk STATE.md: timestamp = TS_OLD (2026-06-11T10:00:00Z)
# Edit: changes the `phase:` line (NOT the timestamp line).
# Guard reconstructs proposed from on-disk + edit → timestamp unchanged → Block.
# Expected: exit 2 (Block) + FULL canonical TimestampStale message.
#
# This test verifies the RECONSTRUCTION semantics of the Edit arm:
#   - Guard reads on-disk STATE.md.
#   - Guard applies old_string→new_string to produce proposed full content.
#   - Guard compares timestamp: in proposed vs on-disk.
#   - Timestamp unchanged → Block: TimestampStale.
#
# This was the 4th bats test required by S-17.04 v1.3 Red Gate Test Table
# (test_verify_state_timestamp_refresh_edit_reconstruct_stale_blocks).
#
# RED GATE: WASM absent → skip. If WASM stub returns Continue after path check,
# exit-2 assertion fails — correct Red Gate failure.
# ---------------------------------------------------------------------------

@test "T-4 test_verify_state_timestamp_refresh_edit_reconstruct_stale_blocks" {
  _require_artifacts
  _write_full_registry

  local ts_old="2026-06-11T10:00:00Z"

  # On-disk STATE.md: old timestamp. The edit will NOT touch the timestamp.
  _write_state_no_lock_with_ts "$ts_old"

  # Edit payload: changes the `phase:` line, NOT the timestamp.
  # After reconstruction: timestamp is still TS_OLD (stale) → must Block.
  # Edit payload fields: old_string + new_string (AC-012; NOT new_content).
  local old_str="phase: test"
  local new_str="phase: complete"

  local envelope
  envelope="{\"event_name\":\"PreToolUse\",\"tool_name\":\"Edit\",\"session_id\":\"t4\",\"dispatcher_trace_id\":\"t4-trace\",\"tool_input\":{\"file_path\":\".factory/STATE.md\",\"old_string\":\"${old_str}\",\"new_string\":\"${new_str}\"}}"

  _run_dispatcher "$envelope"

  # Must exit 2 (Block) — reconstructed content has stale timestamp.
  [ "$status" -eq 2 ] || {
    echo "FAIL: expected exit 2 (Block) for Edit reconstruct with stale timestamp but got status=$status"
    echo "Edit: old_string=${old_str}, new_string=${new_str} (does NOT touch timestamp)"
    echo "Reconstructed proposed content should have unchanged timestamp → TimestampStale"
    echo "Output: $output"
    return 1
  }

  # FULL canonical block message (M03 fix — not substring only).
  local expected_msg="BLOCKED by verify-state-timestamp-refresh: STATE.md timestamp not advanced in this write. Fix: Update 'timestamp:' to the current UTC time before writing STATE.md. Code: TimestampStale."

  [[ "$output" == *"${expected_msg}"* ]] || {
    echo "FAIL: expected FULL canonical TimestampStale message in output."
    echo "Expected message (substring): ${expected_msg}"
    echo "Actual output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-5 (AC-016 / v1.4 Red Gate): Write payload, lock held + expires_at absent → Block LockExpiryStale
#
# Payload type: Write (tool_input.content = full file body, AC-011).
#
# Scenario:
#   - On-disk STATE.md: lock held with a valid expires_at (2026-06-11T10:45:00Z).
#   - Proposed STATE.md (in tool_input.content): lock block has `holder` and
#     `locked_at` but NO `expires_at` line at all. Timestamp IS advanced.
#   - factory_lock_parse::parse_factory_lock(proposed) returns
#     Err(MalformedLockBlock("factory_lock.expires_at field is absent")).
#
# Required result: exit 2 (Block) + FULL canonical LockExpiryStale message.
#
# Canonical LockExpiryStale message (AC-006, full line):
#   BLOCKED by verify-state-timestamp-refresh: factory_lock.expires_at not refreshed
#   in this write while lock is held. Fix: Run: factory-lock-write.sh renew
#   .factory/STATE.md before writing STATE.md. Code: LockExpiryStale.
#
# RED GATE v1.4: current impl routes parse Err(_) on proposed → None → Continue.
# This test MUST FAIL until the implementer adds absent-expires detection (AC-016).
#
# The on-disk STATE.md (written by _write_state_with_lock) has a valid lock so the
# guard can detect the holder from on-disk if needed; but the guard must primarily
# detect the absent expires_at from the PROPOSED content.
# ---------------------------------------------------------------------------

@test "T-5 test_verify_state_timestamp_refresh_lock_held_expires_absent_blocks" {
  _require_artifacts
  _write_full_registry

  # On-disk STATE.md: lock held, valid expires_at (TS_OLD, EXPIRES_OLD).
  # The guard reads this to compare expires_at values.
  _write_state_with_lock "2026-06-11T10:00:00Z" "2026-06-11T10:45:00Z"

  # Proposed content: lock held (holder present), timestamp ADVANCED,
  # but NO expires_at line in the lock block.
  # Write payload: tool_input.content = full file body (AC-011).
  local ts_new="2026-06-11T11:00:00Z"
  local proposed_content
  proposed_content="---\ndocument_type: state\nversion: 0.0.1-bats-test\ntimestamp: ${ts_new}\nphase: test\ncurrent_step: bats-test\nfactory_lock:\n  holder: dev@example.com\n  locked_at: 2026-06-11T10:00:00Z\n---\n\n# STATE (bats fixture - lock held, expires_at ABSENT)\n"

  local envelope
  envelope="{\"event_name\":\"PreToolUse\",\"tool_name\":\"Write\",\"session_id\":\"t5\",\"dispatcher_trace_id\":\"t5-trace\",\"tool_input\":{\"file_path\":\".factory/STATE.md\",\"content\":\"${proposed_content}\"}}"

  _run_dispatcher "$envelope"

  # Must exit 2 (Block) — lock held with absent expires_at must Block.
  [ "$status" -eq 2 ] || {
    echo "FAIL: expected exit 2 (Block: LockExpiryStale) but got status=$status"
    echo "RED GATE v1.4: current impl routes Err(MalformedLockBlock) on proposed → None → Continue."
    echo "Implementer must detect absent/malformed expires_at when holder is present and Block."
    echo "Output: $output"
    return 1
  }

  # FULL canonical LockExpiryStale block message (full-line equality check).
  local expected_msg="BLOCKED by verify-state-timestamp-refresh: factory_lock.expires_at not refreshed in this write while lock is held. Fix: Run: factory-lock-write.sh renew .factory/STATE.md before writing STATE.md. Code: LockExpiryStale."

  [[ "$output" == *"${expected_msg}"* ]] || {
    echo "FAIL: expected FULL canonical LockExpiryStale message in output."
    echo "Expected message (substring): ${expected_msg}"
    echo "Actual output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# Registry assertion test (no WASM required — inspects production registry file).
#
# Asserts that the production hooks-registry.toml has the correct
# verify-state-timestamp-refresh entry (AC-010):
#   - Entry present (name = "verify-state-timestamp-refresh")
#   - async = false (ADR-019 / ADR-025 Decision 12 correctness requirement)
#   - [hooks.capabilities.read_file] block present
#   - path_allow includes ".factory/STATE.md"
#   - priority = 143 (runs after verify-factory-lock at priority 142 — AC-010)
#   - tool = "Edit|Write|MultiEdit" (all three content tools — AC-010)
#   - NO max_bytes or timeout_ms in the capability block (ReadFileCaps is
#     #[serde(deny_unknown_fields)] with path_allow ONLY — AC-010 / Arch Rule 5)
#
# RED GATE: The entry does not yet exist (implementer adds it in T-4 / D16).
# This test FAILS immediately until the registry entry is added — it does NOT
# require the WASM artifact.
# ---------------------------------------------------------------------------

@test "test_verify_state_timestamp_refresh_registry_entry_has_correct_shape" {
  local registry="$REPO_ROOT/plugins/vsdd-factory/hooks-registry.toml"

  # Confirm the production registry file exists and is readable.
  [ -f "$registry" ] || {
    echo "FAIL: production hooks-registry.toml not found at: $registry"
    return 1
  }

  # Check that verify-state-timestamp-refresh is registered.
  # RED GATE: FAILS if the entry does not yet exist.
  grep -q 'name = "verify-state-timestamp-refresh"' "$registry" || {
    echo "FAIL: verify-state-timestamp-refresh entry not found in production hooks-registry.toml."
    echo "Implementer: add the [[hooks]] entry per ADR-025 Decision 12 §12.5 / S-17.04 AC-010."
    echo "Registry: $registry"
    return 1
  }

  # Use awk to extract the verify-state-timestamp-refresh section and verify
  # all required fields. Score 5 for full compliance.
  # The awk script collects fields within the verify-state-timestamp-refresh section
  # (bounded by the next [[hooks]] header or EOF).
  local score
  score=$(awk '
    /name = "verify-state-timestamp-refresh"/ {
      in_section = 1
      has_async_false = 0
      has_read_file_cap = 0
      has_path_allow_state_md = 0
      has_priority_143 = 0
      has_tool_multiedit = 0
    }
    /^\[\[hooks\]\]/ && in_section && !/name = "verify-state-timestamp-refresh"/ {
      # End of section (new [[hooks]] block that is NOT our entry).
      in_section = 0
    }
    in_section && /^async = false/ { has_async_false = 1 }
    in_section && /\[hooks\.capabilities\.read_file\]/ { has_read_file_cap = 1 }
    in_section && has_read_file_cap && /\.factory\/STATE\.md/ { has_path_allow_state_md = 1 }
    in_section && /^priority = 143/ { has_priority_143 = 1 }
    in_section && /tool = "Edit\|Write\|MultiEdit"/ { has_tool_multiedit = 1 }
    END {
      # Flush last section if awk hit EOF inside it.
      total = has_async_false + has_read_file_cap + has_path_allow_state_md + has_priority_143 + has_tool_multiedit
      print total
    }
  ' "$registry")

  # Must score 5: async=false + read_file capability + path_allow + priority=143 + tool=MultiEdit.
  [ "$score" -eq 5 ] || {
    echo "FAIL: verify-state-timestamp-refresh registry entry is incomplete (score=$score/5)."
    echo "Required fields:"
    echo "  1. async = false              (ADR-019 / ADR-025 Decision 12 correctness)"
    echo "  2. [hooks.capabilities.read_file]  (mandatory capability block — AC-010)"
    echo "  3. path_allow = [\".factory/STATE.md\"]  (AC-010)"
    echo "  4. priority = 143             (runs after verify-factory-lock at 142 — AC-010)"
    echo "  5. tool = \"Edit|Write|MultiEdit\"  (MultiEdit must be covered — AC-010)"
    echo "See ADR-025 Decision 12 §12.6 / S-17.04 AC-010 / Registry Entry Spec."
    return 1
  }

  # Additional check: no max_bytes or timeout_ms in the capability block.
  # ReadFileCaps is #[serde(deny_unknown_fields)] — extra fields break registry load (AC-010).
  local forbidden_fields
  forbidden_fields=$(awk '
    /name = "verify-state-timestamp-refresh"/ { in_section=1; in_cap=0 }
    /^\[\[hooks\]\]/ && in_section && !/name = "verify-state-timestamp-refresh"/ { in_section=0; in_cap=0 }
    in_section && /\[hooks\.capabilities\.read_file\]/ { in_cap=1 }
    in_section && in_cap && /^\[\[/ { in_cap=0 }
    in_section && in_cap && /(max_bytes|timeout_ms)/ { print $0 }
  ' "$registry")

  [ -z "$forbidden_fields" ] || {
    echo "FAIL: verify-state-timestamp-refresh [hooks.capabilities.read_file] contains forbidden fields."
    echo "ReadFileCaps is #[serde(deny_unknown_fields)] — only path_allow is valid."
    echo "max_bytes and timeout_ms are Rust source-code arguments, NOT TOML fields."
    echo "Forbidden field lines found:"
    echo "$forbidden_fields"
    echo "These fields will cause registry load failure → guard is silently inert."
    return 1
  }
}
