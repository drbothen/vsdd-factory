#!/usr/bin/env bats
# factory-lock-write.bats — RED-phase (TDD) tests for BC-5.40.001 schema correctness,
# TTL constant, mid-burst renewal, unlock key-removal, and absent-block state.
#
# Traces to: BC-5.40.001 PC1, PC2, PC3, PC4, PC6; Invariants 2 and 3.
# Story: S-17.01 — factory_lock STATE.md schema + state-burst CAS push (D3).
# Target: plugins/vsdd-factory/bin/factory-lock-write.sh
#
# RED GATE: All tests MUST FAIL before implementation because the helper is a
# stub that exits 1 with a TODO message (without writing any fields).
#
# Fixture convention: each test builds a minimal STATE.md in BATS_TEST_TMPDIR
# containing a YAML frontmatter region bounded by --- delimiters. The helper
# accepts a <state_md_path> argument and operates on that file in-place.
#
# Run:
#   bats plugins/vsdd-factory/tests/factory-lock-write.bats

HELPER="$(cd "$(dirname "$BATS_TEST_FILENAME")/.." && pwd)/bin/factory-lock-write.sh"

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

# Write a minimal STATE.md to $1 with NO lock key (unlocked baseline).
# IMPORTANT: must not contain the string 'factory_lock' anywhere so that
# precondition checks using `grep factory_lock` correctly return non-zero.
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

# Write a minimal STATE.md to $1 WITH an existing factory_lock block at the
# given holder/locked_at/expires_at values. Used to set up the renew and
# clear preconditions.
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

# Parse the value of a factory_lock sub-field from a STATE.md fixture.
# Usage: _get_lock_field <file> <field>
# Prints the trimmed value (without surrounding quotes) or empty string.
_get_lock_field() {
  local file="$1"
  local field="$2"
  # Extract from inside the frontmatter (between first --- and second ---).
  # The factory_lock block looks like:
  #   factory_lock:
  #     holder: "value"
  #     locked_at: "value"
  #     expires_at: "value"
  # Capture the 2-space-indented sub-field line value.
  awk '/^---$/{front++} front==1 && /^  '"$field"':/{
    # Strip leading whitespace, field name, colon, surrounding quotes
    gsub(/^  '"$field"': *"?/, "")
    gsub(/"$/, "")
    print
    exit
  }' "$file"
}

# Convert an ISO-8601 UTC timestamp (YYYY-MM-DDTHH:MM:SSZ) to epoch seconds.
# Uses date -jf on BSD/macOS or date -d on GNU/Linux.
_iso_to_epoch() {
  local ts="$1"
  if date --version >/dev/null 2>&1; then
    # GNU date
    date -d "$ts" +%s
  else
    # BSD date (macOS)
    date -u -jf '%Y-%m-%dT%H:%M:%SZ' "$ts" +%s
  fi
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
# test_BC_5_40_001_schema_correctness_on_acquire
# AC-001 / BC-5.40.001 PC1 — schema correctness
#
# After `factory-lock-write.sh acquire <state_md_path>`:
#   - Exit code MUST be 0.
#   - STATE.md frontmatter MUST contain factory_lock.holder == git config user.email (trimmed).
#   - STATE.md frontmatter MUST contain factory_lock.locked_at in format YYYY-MM-DDTHH:MM:SSZ.
#   - STATE.md frontmatter MUST contain factory_lock.expires_at in format YYYY-MM-DDTHH:MM:SSZ.
#   - expires_at - locked_at MUST equal exactly 2700 seconds.
#
# RED GATE: stub exits 1 with TODO message — fails on non-zero exit assertion.
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_schema_correctness_on_acquire" {
  _fixture_no_lock "$FIXTURE_STATE"

  run bash "$HELPER" acquire "$FIXTURE_STATE"

  # Must succeed
  [ "$status" -eq 0 ]

  # factory_lock block must exist
  grep -q 'factory_lock:' "$FIXTURE_STATE"

  # All three required fields must be present
  grep -q 'holder:' "$FIXTURE_STATE"
  grep -q 'locked_at:' "$FIXTURE_STATE"
  grep -q 'expires_at:' "$FIXTURE_STATE"

  # holder must equal git config user.email (trimmed trailing newline only)
  expected_holder="$(git config user.email | tr -d '\n')"
  actual_holder="$(_get_lock_field "$FIXTURE_STATE" holder)"
  [ "$actual_holder" = "$expected_holder" ]

  # locked_at must match ISO-8601 UTC format YYYY-MM-DDTHH:MM:SSZ
  actual_locked_at="$(_get_lock_field "$FIXTURE_STATE" locked_at)"
  [[ "$actual_locked_at" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$ ]]

  # expires_at must match ISO-8601 UTC format YYYY-MM-DDTHH:MM:SSZ
  actual_expires_at="$(_get_lock_field "$FIXTURE_STATE" expires_at)"
  [[ "$actual_expires_at" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$ ]]

  # expires_at - locked_at must equal exactly 2700 seconds
  epoch_locked="$(_iso_to_epoch "$actual_locked_at")"
  epoch_expires="$(_iso_to_epoch "$actual_expires_at")"
  delta=$(( epoch_expires - epoch_locked ))
  [ "$delta" -eq 2700 ]
}

# ---------------------------------------------------------------------------
# test_BC_5_40_001_unlock_removes_key_not_nulls
# AC-002 / BC-5.40.001 PC2 — unlock clears block (key absent, not null)
#
# Given STATE.md with an existing factory_lock block:
#   After `factory-lock-write.sh clear <state_md_path>`:
#   - Exit code MUST be 0.
#   - `grep factory_lock <file>` MUST return non-zero (key entirely absent).
#   - `factory_lock: null` MUST NOT appear (StaleNullBlock violation).
#
# RED GATE: stub exits 1 — fails on non-zero exit assertion.
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_unlock_removes_key_not_nulls" {
  _fixture_with_lock "$FIXTURE_STATE" \
    "developer@example.com" \
    "2026-06-10T14:00:00Z" \
    "2026-06-10T14:45:00Z"

  # Precondition: key is present before clear
  grep -q 'factory_lock:' "$FIXTURE_STATE"

  run bash "$HELPER" clear "$FIXTURE_STATE"

  # Must succeed
  [ "$status" -eq 0 ]

  # Key MUST be absent — grep returns non-zero when key is gone
  run grep 'factory_lock:' "$FIXTURE_STATE"
  [ "$status" -ne 0 ]

  # Stronger: must not contain null form (StaleNullBlock)
  run grep 'factory_lock: null' "$FIXTURE_STATE"
  [ "$status" -ne 0 ]
}

# ---------------------------------------------------------------------------
# test_BC_5_40_001_ttl_expired_block_persists_until_next_write
# AC-003 / BC-5.40.001 PC3 — expired block stays until next write
#
# Given STATE.md with factory_lock.expires_at in the PAST:
#   - The writer (factory-lock-write.sh) MUST NOT auto-remove expired blocks.
#   - The block MUST remain present (guard-side expiry check is S-17.02's scope).
#   - This test confirms the helper does NOT clean up expired locks on its own.
#
# Test strategy: write an expired lock fixture directly (not via helper), then
# invoke `factory-lock-write.sh renew` on it — renew's contract is to update
# expires_at but NOT to remove the block. We verify the block still has all
# three fields after renew. The no-auto-removal property is also tested via a
# direct read: the file content before any helper call contains the expired
# block; we confirm the helper does not delete it unprompted.
#
# RED GATE: stub exits 1 on renew — fails on non-zero exit assertion.
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_ttl_expired_block_persists_until_next_write" {
  # Write fixture with past expires_at (expired 1 hour ago relative to a known past time)
  _fixture_with_lock "$FIXTURE_STATE" \
    "developer@example.com" \
    "2026-01-01T00:00:00Z" \
    "2026-01-01T00:45:00Z"

  # The expired block is in the file before ANY helper invocation
  grep -q 'factory_lock:' "$FIXTURE_STATE"
  grep -q 'expires_at:' "$FIXTURE_STATE"

  # renew should still work on an expired block (expired blocks are valid for renewal;
  # the guard's expiry check is BC-4.13.001 PC2, not the writer's job)
  run bash "$HELPER" renew "$FIXTURE_STATE"
  [ "$status" -eq 0 ]

  # After renew, the block MUST still be present (not auto-removed)
  grep -q 'factory_lock:' "$FIXTURE_STATE"

  # All three fields must still be present after renew
  grep -q 'holder:' "$FIXTURE_STATE"
  grep -q 'locked_at:' "$FIXTURE_STATE"
  grep -q 'expires_at:' "$FIXTURE_STATE"

  # locked_at must remain unchanged at the original value
  actual_locked_at="$(_get_lock_field "$FIXTURE_STATE" locked_at)"
  [ "$actual_locked_at" = "2026-01-01T00:00:00Z" ]

  # expires_at must now be in the future (renewed to now + 2700s),
  # meaning it has changed from the original expired value
  actual_expires_at="$(_get_lock_field "$FIXTURE_STATE" expires_at)"
  [ "$actual_expires_at" != "2026-01-01T00:45:00Z" ]
}

# ---------------------------------------------------------------------------
# test_BC_5_40_001_mid_burst_renewal_updates_expires_at_preserves_locked_at
# AC-004 / BC-5.40.001 PC4 — mid-burst renewal
#
# Given STATE.md with an existing factory_lock block (known locked_at):
#   After `factory-lock-write.sh renew <state_md_path>`:
#   - Exit code MUST be 0.
#   - factory_lock.expires_at MUST be updated to approximately now + 2700s.
#   - factory_lock.locked_at MUST remain unchanged (immutable after acquire).
#   - factory_lock.holder MUST remain unchanged.
#
# RED GATE: stub exits 1 — fails on non-zero exit assertion.
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_mid_burst_renewal_updates_expires_at_preserves_locked_at" {
  local original_locked_at="2026-06-10T12:00:00Z"
  local original_expires_at="2026-06-10T12:45:00Z"
  local original_holder="state-manager@factory.test"

  _fixture_with_lock "$FIXTURE_STATE" \
    "$original_holder" \
    "$original_locked_at" \
    "$original_expires_at"

  # Capture current time BEFORE renew so we can check delta afterward
  before_epoch="$(date -u +%s)"

  run bash "$HELPER" renew "$FIXTURE_STATE"

  # Must succeed
  [ "$status" -eq 0 ]

  after_epoch="$(date -u +%s)"

  # locked_at MUST be unchanged (immutable — records original acquisition instant)
  actual_locked_at="$(_get_lock_field "$FIXTURE_STATE" locked_at)"
  [ "$actual_locked_at" = "$original_locked_at" ]

  # holder MUST be unchanged
  actual_holder="$(_get_lock_field "$FIXTURE_STATE" holder)"
  [ "$actual_holder" = "$original_holder" ]

  # expires_at MUST be updated (different from original)
  actual_expires_at="$(_get_lock_field "$FIXTURE_STATE" expires_at)"
  [ "$actual_expires_at" != "$original_expires_at" ]

  # expires_at MUST match ISO-8601 UTC format
  [[ "$actual_expires_at" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$ ]]

  # expires_at must be approximately now + 2700s:
  # epoch(expires_at) must be in [before_epoch + 2700, after_epoch + 2700 + 2]
  # (allow 2-second tolerance for slow systems)
  expires_epoch="$(_iso_to_epoch "$actual_expires_at")"
  [ "$expires_epoch" -ge $(( before_epoch + 2700 )) ]
  [ "$expires_epoch" -le $(( after_epoch + 2700 + 2 )) ]
}

# ---------------------------------------------------------------------------
# test_BC_5_40_001_absent_block_is_unlocked_state
# AC-006 / BC-5.40.001 PC6 — absent block = valid unlocked state
#
# A STATE.md with NO factory_lock key is the valid unlocked state.
# The helper's `acquire` mode must work from the absent (unlocked) state,
# and `clear` must restore the absent state.
#
# Test sequence:
#   1. Start with no factory_lock (unlocked state) — valid precondition.
#   2. acquire writes the block — block now present.
#   3. clear removes the block — back to absent (unlocked).
#   4. Final state: factory_lock key ABSENT.
#
# RED GATE: stub exits 1 on acquire — fails on non-zero exit assertion.
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_absent_block_is_unlocked_state" {
  _fixture_no_lock "$FIXTURE_STATE"

  # Precondition: no factory_lock key in file
  run grep 'factory_lock' "$FIXTURE_STATE"
  [ "$status" -ne 0 ]

  # acquire from absent state must succeed
  run bash "$HELPER" acquire "$FIXTURE_STATE"
  [ "$status" -eq 0 ]

  # block now present after acquire
  grep -q 'factory_lock:' "$FIXTURE_STATE"

  # clear from locked state must succeed
  run bash "$HELPER" clear "$FIXTURE_STATE"
  [ "$status" -eq 0 ]

  # factory_lock key must be absent after clear (back to unlocked state)
  run grep 'factory_lock' "$FIXTURE_STATE"
  [ "$status" -ne 0 ]
}

# ---------------------------------------------------------------------------
# test_BC_5_40_001_ttl_constant_is_2700_seconds
# AC-007 / BC-5.40.001 Invariant 2 — TTL constant = exactly 2700 seconds
#
# After `factory-lock-write.sh acquire <state_md_path>`:
#   expires_at - locked_at MUST equal exactly 2700 seconds.
#   This assertion is separate from the schema test to make the specific
#   invariant failure immediately visible in the test output.
#
# RED GATE: stub exits 1 — fails on non-zero exit assertion.
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_ttl_constant_is_2700_seconds" {
  _fixture_no_lock "$FIXTURE_STATE"

  run bash "$HELPER" acquire "$FIXTURE_STATE"

  # Must succeed
  [ "$status" -eq 0 ]

  # Both timestamps must be present and valid
  actual_locked_at="$(_get_lock_field "$FIXTURE_STATE" locked_at)"
  actual_expires_at="$(_get_lock_field "$FIXTURE_STATE" expires_at)"

  [[ "$actual_locked_at" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$ ]]
  [[ "$actual_expires_at" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$ ]]

  # Convert to epoch and compute delta
  epoch_locked="$(_iso_to_epoch "$actual_locked_at")"
  epoch_expires="$(_iso_to_epoch "$actual_expires_at")"
  delta=$(( epoch_expires - epoch_locked ))

  # The TTL constant MUST be exactly 2700 seconds — not 2699, not 2701
  [ "$delta" -eq 2700 ]
}
