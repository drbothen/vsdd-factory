#!/usr/bin/env bats
# factory-unlock-decide.bats — RED-phase (TDD) tests for BC-6.23.001 PC4, PC5, PC6,
# EC-003, EC-005, EC-010. AC-004, AC-005, AC-006, AC-014.
#
# Traces to: BC-6.23.001 PC4 (self-release), PC5 (non-holder rejection), PC6
#            (force-steal 4-field audit event), EC-003 (absent lock noop), EC-005
#            (force on absent = noop), EC-010 (self-force emits released not stolen).
#            AC-004, AC-005, AC-006, AC-014.
# Story: S-17.03 — /factory-lock + /factory-unlock skills + health status.
# Target: plugins/vsdd-factory/bin/factory-unlock-decide.sh
#
# RED GATE: All tests MUST FAIL because the helper is a stub that exits 1 with a
# TODO message. Tests fail with ASSERTION errors (wrong exit code, wrong output
# content) — NOT "file not found" errors.
#
# The helper signature (from stub header):
#   factory-unlock-decide.sh <state_md_path> <current_git_email> [--force]
#
# This helper is pure-core: no git operations, no STATE.md writes.
# All output (decision tokens + field blocks) goes to stdout.
# Error messages (REFUSED_NOT_HOLDER) go to stderr.
#
# Run:
#   bats plugins/vsdd-factory/tests/factory-unlock-decide.bats

HELPER="$(cd "$(dirname "$BATS_TEST_FILENAME")/.." && pwd)/bin/factory-unlock-decide.sh"

# ---------------------------------------------------------------------------
# Fixed timestamps used across fixtures — far future so they never expire
# ---------------------------------------------------------------------------
FUTURE_LOCKED_AT="2099-01-01T00:00:00Z"
FUTURE_EXPIRES_AT="2099-01-01T00:45:00Z"

# ---------------------------------------------------------------------------
# Fixture helpers
# ---------------------------------------------------------------------------

# Write a minimal STATE.md to $1 with NO factory_lock key.
_fixture_no_lock() {
  local path="$1"
  cat > "$path" <<'FIXTURE'
---
document_type: state
version: "0.0.1-test"
phase: test
current_step: "test-step"
---

# STATE (test fixture)
Unlocked baseline — no lock block present.
FIXTURE
}

# Write a minimal STATE.md to $1 with a factory_lock block.
# Arguments: path holder locked_at expires_at
_fixture_with_lock() {
  local path="$1"
  local holder="$2"
  local locked_at="$3"
  local expires_at="$4"
  cat > "$path" <<FIXTURE
---
document_type: state
version: "0.0.1-test"
phase: test
current_step: "test-step"
factory_lock:
  holder: "${holder}"
  locked_at: "${locked_at}"
  expires_at: "${expires_at}"
---

# STATE (test fixture)
Has a factory_lock block.
FIXTURE
}

# ---------------------------------------------------------------------------
# setup / teardown
# ---------------------------------------------------------------------------

setup() {
  WORK="$BATS_TEST_TMPDIR"
  FIXTURE_STATE="$WORK/STATE.md"
}

teardown() {
  : # BATS_TEST_TMPDIR is cleaned automatically by bats
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_unlock_decide_self_release_proceed
# AC-004 / BC-6.23.001 PC4 — self-held lock, plain unlock → PROCEED_RELEASE
#
# When holder == current_email and no --force flag is passed, the helper MUST:
#   - Exit 0.
#   - Print "PROCEED_RELEASE" to stdout.
#   - Print the 3 release event fields to stdout:
#       holder=<email>
#       locked_at=<original_locked_at>
#       released_at=<now_iso8601>
#
# RED GATE: stub exits 1 with TODO — fails on exit-0 assertion.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_unlock_decide_self_release_proceed" {
  local test_email="self@example.com"

  _fixture_with_lock "$FIXTURE_STATE" \
    "$test_email" \
    "$FUTURE_LOCKED_AT" \
    "$FUTURE_EXPIRES_AT"

  run bash "$HELPER" "$FIXTURE_STATE" "$test_email"

  # Must exit 0 (self-release is a success path)
  [ "$status" -eq 0 ]

  # stdout MUST contain the PROCEED_RELEASE decision token
  [[ "$output" == *"PROCEED_RELEASE"* ]]

  # stdout MUST contain the holder field
  [[ "$output" == *"holder="* ]] || [[ "$output" == *"holder:"* ]]

  # stdout MUST contain the original locked_at
  [[ "$output" == *"${FUTURE_LOCKED_AT}"* ]]

  # stdout MUST contain a released_at field with an ISO-8601 UTC timestamp
  [[ "$output" == *"released_at="* ]] || [[ "$output" == *"released_at:"* ]]
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_unlock_decide_non_holder_rejected
# AC-005 / BC-6.23.001 PC5 — foreign lock, plain unlock → REFUSED_NOT_HOLDER
#
# When holder != current_email and no --force flag, the helper MUST:
#   - Exit 1.
#   - Print "REFUSED_NOT_HOLDER" to stderr.
#   - Print error message to stderr: "Cannot unlock — factory is held by <holder_email>.
#     Use /factory-unlock --force to force-release."
#
# RED GATE: stub exits 1 with TODO (wrong error message) — exit-1 assertion
# happens to pass but all content assertions fail.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_unlock_decide_non_holder_rejected" {
  local holder_email="other@example.com"
  local caller_email="dev@example.com"

  _fixture_with_lock "$FIXTURE_STATE" \
    "$holder_email" \
    "$FUTURE_LOCKED_AT" \
    "$FUTURE_EXPIRES_AT"

  run bash "$HELPER" "$FIXTURE_STATE" "$caller_email"

  # Must exit 1 (non-holder rejection)
  [ "$status" -eq 1 ]

  # stderr (captured in $output by bats) MUST contain the REFUSED_NOT_HOLDER token
  [[ "$output" == *"REFUSED_NOT_HOLDER"* ]]

  # MUST contain the error message with the holder email
  [[ "$output" == *"Cannot unlock"* ]]
  [[ "$output" == *"${holder_email}"* ]]

  # MUST contain the /factory-unlock --force command hint
  [[ "$output" == *"/factory-unlock --force"* ]]
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_unlock_decide_force_steal_four_fields
# AC-006 / BC-6.23.001 PC6 — foreign lock + --force → PROCEED_FORCE_STEAL
# with all 4 required audit event fields
#
# When holder != current_email and --force is passed, the helper MUST:
#   - Exit 0.
#   - Print "PROCEED_FORCE_STEAL" to stdout.
#   - Print the 4-field factory.lock.stolen audit event block to stdout:
#       1. stolen_by=<current_git_email>
#       2. stolen_from=<factory_lock.holder>
#       3. holder_locked_at=<factory_lock.locked_at>
#       4. stolen_at=<now_iso8601>
#
# RED GATE: stub exits 1 with TODO — fails on exit-0 assertion.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_unlock_decide_force_steal_four_fields" {
  local holder_email="other@example.com"
  local caller_email="dev@example.com"

  _fixture_with_lock "$FIXTURE_STATE" \
    "$holder_email" \
    "$FUTURE_LOCKED_AT" \
    "$FUTURE_EXPIRES_AT"

  run bash "$HELPER" "$FIXTURE_STATE" "$caller_email" --force

  # Must exit 0 (force-steal proceeds)
  [ "$status" -eq 0 ]

  # stdout MUST contain the PROCEED_FORCE_STEAL decision token
  [[ "$output" == *"PROCEED_FORCE_STEAL"* ]]

  # Field 1: stolen_by MUST be the caller email
  [[ "$output" == *"${caller_email}"* ]]
  [[ "$output" == *"stolen_by="* ]] || [[ "$output" == *"stolen_by:"* ]]

  # Field 2: stolen_from MUST be the holder email
  [[ "$output" == *"${holder_email}"* ]]
  [[ "$output" == *"stolen_from="* ]] || [[ "$output" == *"stolen_from:"* ]]

  # Field 3: holder_locked_at MUST be the original locked_at from fixture
  [[ "$output" == *"${FUTURE_LOCKED_AT}"* ]]
  [[ "$output" == *"holder_locked_at="* ]] || [[ "$output" == *"holder_locked_at:"* ]]

  # Field 4: stolen_at MUST be an ISO-8601 UTC timestamp
  [[ "$output" == *"stolen_at="* ]] || [[ "$output" == *"stolen_at:"* ]]
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_unlock_decide_self_force_emits_released_not_stolen
# EC-010 / BC-6.23.001 EC-010 — self-held lock + --force →
# PROCEED_RELEASE_SELF_FORCE (NOT PROCEED_FORCE_STEAL)
#
# When holder == current_email and --force is passed, the helper MUST:
#   - Exit 0.
#   - Print "PROCEED_RELEASE_SELF_FORCE" to stdout.
#   - NOT print "PROCEED_FORCE_STEAL" (stolen_by == stolen_from is meaningless audit).
#
# This corresponds to emit factory.lock.released (not factory.lock.stolen).
#
# RED GATE: stub exits 1 with TODO — fails on exit-0 assertion.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_unlock_decide_self_force_emits_released_not_stolen" {
  local test_email="self@example.com"

  _fixture_with_lock "$FIXTURE_STATE" \
    "$test_email" \
    "$FUTURE_LOCKED_AT" \
    "$FUTURE_EXPIRES_AT"

  run bash "$HELPER" "$FIXTURE_STATE" "$test_email" --force

  # Must exit 0 (self-force proceeds like a plain release)
  [ "$status" -eq 0 ]

  # stdout MUST contain PROCEED_RELEASE_SELF_FORCE (NOT PROCEED_FORCE_STEAL)
  [[ "$output" == *"PROCEED_RELEASE_SELF_FORCE"* ]]

  # MUST NOT output PROCEED_FORCE_STEAL (EC-010 invariant)
  [[ "$output" != *"PROCEED_FORCE_STEAL"* ]]
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_unlock_decide_force_on_absent_lock_noop
# AC-014 / BC-6.23.001 EC-005 — absent lock + --force → NOOP_ABSENT
#
# When factory_lock block is absent and --force is passed, the helper MUST:
#   - Exit 0.
#   - Print "NOOP_ABSENT" to stdout.
#   - NOT emit any event fields (no holder to name in stolen_from).
#
# RED GATE: stub exits 1 with TODO — fails on exit-0 assertion.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_unlock_decide_force_on_absent_lock_noop" {
  _fixture_no_lock "$FIXTURE_STATE"

  run bash "$HELPER" "$FIXTURE_STATE" "dev@example.com" --force

  # Must exit 0 (absent lock + --force is a no-op, not an error)
  [ "$status" -eq 0 ]

  # stdout MUST contain the NOOP_ABSENT decision token
  [[ "$output" == *"NOOP_ABSENT"* ]]

  # MUST NOT contain stolen_from or PROCEED_FORCE_STEAL (nothing to steal)
  [[ "$output" != *"stolen_from"* ]]
  [[ "$output" != *"PROCEED_FORCE_STEAL"* ]]
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_unlock_decide_already_unlocked_noop
# BC-6.23.001 EC-003 — absent lock, plain unlock → NOOP_ABSENT
#
# When factory_lock block is absent and no --force is passed, the helper MUST:
#   - Exit 0.
#   - Print "NOOP_ABSENT" to stdout.
# This is the EC-003 "already unlocked" case.
#
# RED GATE: stub exits 1 with TODO — fails on exit-0 assertion.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_unlock_decide_already_unlocked_noop" {
  _fixture_no_lock "$FIXTURE_STATE"

  run bash "$HELPER" "$FIXTURE_STATE" "dev@example.com"

  # Must exit 0 (already unlocked is a no-op, not an error)
  [ "$status" -eq 0 ]

  # stdout MUST contain the NOOP_ABSENT decision token
  [[ "$output" == *"NOOP_ABSENT"* ]]
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_unlock_decide_crlf_self_release
# F-1 / BC-6.23.001 PC4 — CRLF line endings in STATE.md must not cause
# the self-held holder field to be mis-parsed (CR-polluted), resulting in a
# holder mismatch and spurious REFUSED_NOT_HOLDER.
#
# When factory_lock.holder == current_email and the STATE.md uses CRLF line
# endings, the helper MUST normalize before comparing and MUST return
# PROCEED_RELEASE (exit 0), NOT REFUSED_NOT_HOLDER (exit 1) caused by a
# CR-trailing holder value not matching the caller's clean email string.
#
# Fixture: written via printf with \r\n line endings (CRLF throughout).
#
# RED GATE: the current stub exits 1 with TODO, so the exit-0 assertion
# fails first. After stub replacement the CRLF gap will produce a CR-polluted
# holder ("self@example.com\r") that does not equal "self@example.com", so
# REFUSED_NOT_HOLDER fires and the exit-0 + PROCEED_RELEASE assertions fail.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_unlock_decide_crlf_self_release" {
  local test_email="self@example.com"

  # Build a CRLF fixture using printf '%s\r\n' (portable: no leading -- flag needed).
  {
    printf '%s\r\n' '---'
    printf '%s\r\n' 'document_type: state'
    printf '%s\r\n' 'version: "0.0.1-test"'
    printf '%s\r\n' 'phase: test'
    printf '%s\r\n' 'current_step: "test-step"'
    printf '%s\r\n' 'factory_lock:'
    printf '  holder: "%s"\r\n' "$test_email"
    printf '  locked_at: "%s"\r\n' "$FUTURE_LOCKED_AT"
    printf '  expires_at: "%s"\r\n' "$FUTURE_EXPIRES_AT"
    printf '%s\r\n' '---'
    printf '\r\n'
    printf '%s\r\n' '# STATE (CRLF fixture)'
    printf '%s\r\n' 'Self-held lock — CRLF line endings throughout.'
  } > "$FIXTURE_STATE"

  run bash "$HELPER" "$FIXTURE_STATE" "$test_email"

  # Must exit 0 — CRLF normalization must not produce a CR-polluted holder
  # that fails the equality check, causing REFUSED_NOT_HOLDER
  [ "$status" -eq 0 ] \
    || { printf 'FAIL: expected exit 0 (PROCEED_RELEASE), got exit %s\n  output: %s\n' "$status" "$output" >&2; false; }

  # stdout MUST contain PROCEED_RELEASE (not REFUSED_NOT_HOLDER)
  [[ "$output" == *"PROCEED_RELEASE"* ]] \
    || { printf 'FAIL: expected PROCEED_RELEASE in output, got: %s\n' "$output" >&2; false; }

  # MUST NOT contain REFUSED_NOT_HOLDER
  [[ "$output" != *"REFUSED_NOT_HOLDER"* ]] \
    || { printf 'FAIL: REFUSED_NOT_HOLDER must not appear in CRLF self-release output\n' >&2; false; }
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_unlock_decide_crlf_no_tempfile_leak
# F-1703-001 / BC-6.23.001 — CRLF normalization MUST NOT leak a temp file
# beside STATE.md in .factory/.
#
# Bug: _normalize_crlf_for_read sets _UNLOCK_TMPFILE INSIDE a command-
# substitution subshell.  The parent process's EXIT trap never sees it, so
# the CR-stripped temp file (mktemp "${file}.XXXXXX") survives the helper.
# Empirically: a CRLF read of STATE.md leaves STATE.md.XXXXXX beside it.
#
# Contract: after factory-unlock-decide.sh reads a CRLF STATE.md, the
# STATE.md's directory MUST contain no STATE.md.* temp file.  This
# assertion holds whether the implementer cleans up the temp or moves its
# creation to ${TMPDIR:-/tmp} — the .factory/ directory stays clean either way.
#
# RED GATE: the current implementation leaks STATE.md.XXXXXX, so the
# after-listing differs from the before-listing → assertion fails with
# detected-leftover-file evidence.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_unlock_decide_crlf_no_tempfile_leak" {
  # Use a dedicated subdirectory so the directory listing is fully controlled
  local leakdir="$WORK/leakcheck"
  mkdir -p "$leakdir"
  local crlf_state="$leakdir/STATE.md"
  local test_email="self@example.com"

  # Write CRLF fixture (self-held lock → PROCEED_RELEASE, exercising CRLF path)
  {
    printf '%s\r\n' '---'
    printf '%s\r\n' 'document_type: state'
    printf '%s\r\n' 'version: "0.0.1-test"'
    printf '%s\r\n' 'phase: test'
    printf '%s\r\n' 'current_step: "test-step"'
    printf '%s\r\n' 'factory_lock:'
    printf '  holder: "%s"\r\n' "$test_email"
    printf '  locked_at: "%s"\r\n' "$FUTURE_LOCKED_AT"
    printf '  expires_at: "%s"\r\n' "$FUTURE_EXPIRES_AT"
    printf '%s\r\n' '---'
    printf '\r\n'
    printf '%s\r\n' '# STATE (CRLF leak-check fixture)'
  } > "$crlf_state"

  # Snapshot directory contents BEFORE the helper run
  local before_listing
  before_listing="$(ls "$leakdir")"

  # Run the helper — exit code 0 (PROCEED_RELEASE) confirms the CRLF path ran
  run bash "$HELPER" "$crlf_state" "$test_email"
  [ "$status" -eq 0 ] \
    || { printf 'FAIL: helper must exit 0 (PROCEED_RELEASE), got exit %s\n  output: %s\n' "$status" "$output" >&2; false; }

  # Snapshot directory contents AFTER the helper run
  local after_listing
  after_listing="$(ls "$leakdir")"

  # The directory listing MUST be unchanged — no STATE.md.* temp file left behind
  [ "$before_listing" = "$after_listing" ] \
    || {
      printf 'FAIL (F-1703-001): temp file leaked beside STATE.md after CRLF read\n' >&2
      printf '  BEFORE: %s\n' "$before_listing" >&2
      printf '  AFTER:  %s\n' "$after_listing" >&2
      printf '  Leaked: %s\n' "$(comm -13 <(printf '%s\n' "$before_listing" | sort) <(printf '%s\n' "$after_listing" | sort))" >&2
      false
    }
}
