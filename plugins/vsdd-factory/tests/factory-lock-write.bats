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

# ---------------------------------------------------------------------------
# test_BC_5_40_001_expires_at_derived_from_captured_locked_at
# F-P1-001 / AC-008 / BC-5.40.001 PC1 + Invariant 3 — expires_at MUST be derived
# from the same captured epoch as locked_at (zero-tolerance, deterministic).
#
# The contract (BC-5.40.001 Invariant 3) requires:
#   expires_at = locked_at + 2700s EXACTLY (not "approximately").
#
# The failure mode: if the implementation calls the clock TWICE — once for
# locked_at and once for expires_at via _now_plus_seconds — a second boundary
# crossing during acquire will produce delta = 2701 (or rarely 2699).
# The correct implementation must capture ONE epoch and derive BOTH timestamps
# from that single captured value.
#
# Test strategy: run acquire 60 times in rapid succession.  Across 60 invocations
# of a two-clock implementation, at least one will cross a second boundary and
# produce delta != 2700.  A single-clock implementation always produces delta
# == 2700 regardless of boundary crossings.
#
# RED GATE: the current two-clock implementation will produce at least one
# delta == 2701 across 60 iterations, causing this test to fail.
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_expires_at_derived_from_captured_locked_at" {
  # The correct implementation MUST capture ONE epoch and derive BOTH locked_at and
  # expires_at from that single captured value:
  #   locked_at  = epoch_at_acquire (formatted as ISO-8601)
  #   expires_at = epoch_at_acquire + 2700 (formatted as ISO-8601)
  #
  # The structural defect in the two-clock implementation:
  #   locked_at  = _now_iso()           → reads clock at time T
  #   expires_at = _now_plus_seconds()  → reads clock AGAIN at time T'
  # When a second boundary crosses between T and T', delta = T' - T + 2700 = 2701 (or 2699).
  #
  # Test strategy: inject a `date` shim that returns the real timestamp for the
  # FIRST call (locked_at) and returns a timestamp +1 second for the SECOND call
  # (the _now_plus_seconds call).  A two-clock implementation will produce delta = 2701.
  # A single-clock implementation captures the epoch ONCE before calling date at all,
  # so the shim's extra second only affects the formatting — but because the correct
  # implementation derives expires_at arithmetically from the captured epoch (not by
  # calling `date` a second time for the base), the delta remains 2700.
  #
  # The shim tests the structural invariant, not the absolute timestamp value.
  # This approach is deterministic: it always produces delta = 2701 on a two-clock
  # implementation, regardless of system load or second-boundary timing.

  local stub_bin="$BATS_TEST_TMPDIR/date-stub-bin"
  mkdir -p "$stub_bin"

  # Resolve the real date binary BEFORE writing the shim
  local real_date_path
  real_date_path="$(command -v date)"

  # Call counter file — the shim increments it on each invocation
  local call_counter="$BATS_TEST_TMPDIR/date-call-count"
  echo "0" > "$call_counter"

  cat > "$stub_bin/date" <<STUB
#!/usr/bin/env bash
# date shim for test_BC_5_40_001_expires_at_derived_from_captured_locked_at
# On the SECOND call (the _now_plus_seconds call for expires_at), inject +1 extra
# second to simulate a second-boundary crossing between the two date calls.
REAL_DATE="${real_date_path}"
CALL_COUNT_FILE="${call_counter}"

count=\$(cat "\$CALL_COUNT_FILE")
count=\$(( count + 1 ))
echo "\$count" > "\$CALL_COUNT_FILE"

if [ "\$count" -ge 2 ]; then
  # Second or later call: inject an extra +1 second to simulate boundary crossing.
  # For BSD date: -v+2700S becomes -v+2701S
  # For GNU date: "+2700 seconds" becomes "+2701 seconds"
  new_args=()
  for arg in "\$@"; do
    if [[ "\$arg" == *"+2700S"* ]]; then
      new_args+=( "\${arg/+2700S/+2701S}" )
    elif [[ "\$arg" == "+2700 seconds" ]]; then
      new_args+=( "+2701 seconds" )
    elif [[ "\$arg" == "-d" ]]; then
      new_args+=( "\$arg" )
    else
      new_args+=( "\$arg" )
    fi
  done
  "\$REAL_DATE" "\${new_args[@]}"
else
  "\$REAL_DATE" "\$@"
fi
STUB
  chmod +x "$stub_bin/date"

  _fixture_no_lock "$FIXTURE_STATE"

  # Run acquire with the date shim on PATH
  run env PATH="${stub_bin}:${PATH}" bash "$HELPER" acquire "$FIXTURE_STATE"
  [ "$status" -eq 0 ]

  local actual_locked_at actual_expires_at epoch_locked epoch_expires delta
  actual_locked_at="$(_get_lock_field "$FIXTURE_STATE" locked_at)"
  actual_expires_at="$(_get_lock_field "$FIXTURE_STATE" expires_at)"

  # Both fields must be valid ISO-8601 UTC timestamps
  [[ "$actual_locked_at" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$ ]]
  [[ "$actual_expires_at" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$ ]]

  epoch_locked="$(_iso_to_epoch "$actual_locked_at")"
  epoch_expires="$(_iso_to_epoch "$actual_expires_at")"
  delta=$(( epoch_expires - epoch_locked ))

  # ZERO tolerance: expires_at MUST equal locked_at + EXACTLY 2700 seconds.
  # A two-clock implementation will produce delta = 2701 (shim's extra second
  # affects the second date call for _now_plus_seconds, not the first).
  # A single-clock implementation derives expires_at from the captured locked_at epoch,
  # so the shim's extra second does NOT shift the delta — delta remains 2700.
  [ "$delta" -eq 2700 ]
}

# ---------------------------------------------------------------------------
# test_BC_5_40_001_clear_preserves_body_factory_lock_mention
# F-P1-002 / BC-5.40.001 PC2 — clear removes ONLY the frontmatter key,
# not any prose/code-block lines in the document body that happen to begin
# with `factory_lock:`.
#
# Failure mode: the current `_remove_factory_lock` awk matches `^factory_lock:`
# file-wide (not frontmatter-scoped), so it deletes body lines too.
#
# Fixture: STATE.md with a real factory_lock block in frontmatter AND a line
# beginning with `factory_lock:` in the document body.
#
# After clear:
#   - The frontmatter factory_lock key MUST be absent (grep returns non-zero).
#   - The body line beginning with `factory_lock:` MUST be preserved.
#
# RED GATE: the current awk in _remove_factory_lock matches ^factory_lock:
# globally, so the body line is also deleted — this assertion will fail.
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_clear_preserves_body_factory_lock_mention" {
  # Write fixture with factory_lock in BOTH frontmatter AND a body code-block
  cat > "$FIXTURE_STATE" <<'FIXTURE'
---
document_type: state
version: "0.0.1-test"
phase: test
factory_lock:
  holder: "dev@example.com"
  locked_at: "2026-06-10T14:00:00Z"
  expires_at: "2026-06-10T14:45:00Z"
---

# STATE (test fixture)
This section documents the factory lock schema.

Example YAML block showing the lock schema:

```yaml
factory_lock:
  holder: "developer@example.com"
  locked_at: "2026-06-10T14:00:00Z"
  expires_at: "2026-06-10T14:45:00Z"
```

See ADR-025 for details.
FIXTURE

  # Precondition: frontmatter factory_lock key is present
  grep -q 'factory_lock:' "$FIXTURE_STATE"
  # Precondition: body line beginning factory_lock: is also present
  grep -q '^factory_lock:' "$FIXTURE_STATE"

  run bash "$HELPER" clear "$FIXTURE_STATE"
  [ "$status" -eq 0 ]

  # The frontmatter factory_lock key MUST be gone.
  # Because the body still contains `factory_lock:`, a file-global grep
  # would return 0 (found) — so we must check the FRONTMATTER specifically.
  # Strategy: extract frontmatter region and assert factory_lock is absent there.
  local frontmatter_only
  frontmatter_only="$(awk '/^---$/{f++} f==1{print} f==2{exit}' "$FIXTURE_STATE")"
  if echo "$frontmatter_only" | grep -q 'factory_lock:'; then
    echo "FAIL: factory_lock key still present in frontmatter after clear" >&2
    false
  fi

  # The body line beginning with `factory_lock:` MUST be preserved.
  # Extract body (everything after the closing --- of frontmatter).
  local body_only
  body_only="$(awk '/^---$/{f++} f>=2{if(f==2){f=3; next} print}' "$FIXTURE_STATE")"
  if ! echo "$body_only" | grep -q '^factory_lock:'; then
    echo "FAIL: body line starting with 'factory_lock:' was incorrectly deleted" >&2
    false
  fi

  # The prose sentence MUST also be preserved
  grep -q 'This section documents the factory lock schema.' "$FIXTURE_STATE"
}

# ---------------------------------------------------------------------------
# test_BC_5_40_001_acquire_on_malformed_frontmatter_fails_loud
# F-P1-003 / BC-5.40.001 PC1 — SchemaViolation on malformed frontmatter
#
# When STATE.md has NO closing `---` delimiter (malformed frontmatter),
# the helper MUST:
#   - Exit NON-ZERO (not silently exit 0 having written nothing).
#   - Emit an actionable error message containing a schema/frontmatter-related
#     term (e.g., "SchemaViolation", "frontmatter", "malformed", "---").
#   - NOT write a partial factory_lock block to the corrupted file.
#
# Failure mode: the current _write_factory_lock_block awk inserts the block
# "before the second ---" — with no closing ---, there is no second --- so
# the awk produces no insertion at all, yet the helper exits 0 having silently
# written nothing.  PC1 requires a SchemaViolation signal on malformed input.
#
# RED GATE: current impl exits 0 without writing the block and without emitting
# any error — both the non-zero exit assertion and the error-message assertion fail.
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_acquire_on_malformed_frontmatter_fails_loud" {
  # Fixture: opening --- but NO closing --- (malformed frontmatter)
  cat > "$FIXTURE_STATE" <<'FIXTURE'
---
document_type: state
version: "0.0.1-test"
phase: test
FIXTURE
  # Deliberately: no closing ---

  run bash "$HELPER" acquire "$FIXTURE_STATE"

  # MUST exit non-zero — a silent exit 0 with no block written is a PC1 violation
  [ "$status" -ne 0 ]

  # MUST emit an actionable error message referencing schema/frontmatter problems
  # (matches any of: SchemaViolation, frontmatter, malformed, ---)
  [[ "$output" =~ SchemaViolation|frontmatter|malformed|'---' ]]

  # MUST NOT have written a factory_lock block (no partial/silent write)
  run grep 'factory_lock' "$FIXTURE_STATE"
  [ "$status" -ne 0 ]
}

# ---------------------------------------------------------------------------
# test_BC_5_40_001_acquire_fails_when_git_email_unset
# F-P1-004 / BC-5.40.001 PC1 — SchemaViolation when git user.email is unset
#
# When `git config user.email` returns an empty string (user email not configured),
# the helper MUST:
#   - Exit NON-ZERO.
#   - Emit an actionable error message that helps the developer understand they
#     need to configure git user.email (e.g., references "git config", "user.email",
#     "holder", or "email").
#   - NOT write a factory_lock block with an empty holder (which would violate
#     BC-5.40.001 PC1: holder must be non-empty git user.email).
#
# Test strategy: use GIT_CONFIG_GLOBAL=/dev/null and a fresh HOME directory
# with no local git config so that `git config user.email` returns empty.
#
# Failure mode: the current implementation runs `git config user.email | tr -d '\n'`
# with `set -euo pipefail`.  When email is unconfigured, git config exits 1 and the
# pipeline exits non-zero — BUT the error message emitted is empty (bash's
# `set -e` exits the script silently).  PC1 requires an ACTIONABLE error message.
#
# RED GATE: current impl exits non-zero but emits NO error message.
# The assertion checking for an actionable message will fail.
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_acquire_fails_when_git_email_unset" {
  _fixture_no_lock "$FIXTURE_STATE"

  # Isolate git config: use /dev/null as global config and a clean HOME
  # so no user.email is set anywhere in the resolution chain.
  local fake_home="$BATS_TEST_TMPDIR/fake-home"
  mkdir -p "$fake_home"

  run env GIT_CONFIG_GLOBAL=/dev/null HOME="$fake_home" \
    bash "$HELPER" acquire "$FIXTURE_STATE"

  # MUST exit non-zero (empty holder is a SchemaViolation)
  [ "$status" -ne 0 ]

  # MUST emit an actionable message referencing git config, email, or holder
  # so the developer knows what to fix.
  [[ "$output" =~ git|config|email|holder|user ]]

  # MUST NOT have written a factory_lock block with an empty holder
  # (a block with holder: "" would silently satisfy grep but violate PC1)
  run grep 'factory_lock' "$FIXTURE_STATE"
  [ "$status" -ne 0 ]
}

# ---------------------------------------------------------------------------
# test_BC_5_40_001_renew_on_malformed_block_fails_loud
# F-P1-006 / BC-5.40.001 PC4 — RenewalMissed on malformed block (missing expires_at)
#
# When factory_lock block is present but MISSING the `expires_at` sub-field,
# the helper in `renew` mode MUST:
#   - Exit NON-ZERO with a RenewalMissed-class error.
#   - NOT silently exit 0 claiming a successful renewal.
#   - NOT leave the block in a partially-updated state.
#
# Failure mode: the current _update_expires_at awk processes the file, finds
# no `expires_at:` line, and writes the file unchanged — then the helper exits 0
# printing "renewed lock expires_at to ..." despite having changed nothing.
# This is a silent no-op that violates PC4 (renewal MUST refresh expires_at).
#
# RED GATE: current impl exits 0 without actually writing expires_at — both the
# non-zero exit assertion and the error-message assertion fail.
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_renew_on_malformed_block_fails_loud" {
  # Fixture: factory_lock block with holder + locked_at but NO expires_at
  _fixture_with_lock "$FIXTURE_STATE" \
    "dev@example.com" \
    "2026-06-10T14:00:00Z" \
    "PLACEHOLDER_EXPIRES_WILL_BE_REMOVED"

  # Remove the expires_at line to create the malformed-block condition
  local tmpfile
  tmpfile="$(mktemp "$BATS_TEST_TMPDIR/state-noexpiry.XXXXXX")"
  grep -v 'expires_at:' "$FIXTURE_STATE" > "$tmpfile"
  mv "$tmpfile" "$FIXTURE_STATE"

  # Precondition: block is present but expires_at is absent
  grep -q 'factory_lock:' "$FIXTURE_STATE"
  run grep 'expires_at:' "$FIXTURE_STATE"
  [ "$status" -ne 0 ]

  run bash "$HELPER" renew "$FIXTURE_STATE"

  # MUST exit non-zero — a silent exit 0 with no expires_at written is RenewalMissed
  [ "$status" -ne 0 ]

  # MUST emit a RenewalMissed-class error message
  [[ "$output" =~ RenewalMissed|expires_at|malformed|missing|renewal ]]
}

# ---------------------------------------------------------------------------
# test_BC_5_40_001_crlf_frontmatter_handled
# F-P1-010 / BC-5.40.001 PC1 — CRLF line endings MUST NOT cause silent no-op
#
# When STATE.md has CRLF line endings (e.g., from a Windows checkout), the
# helper in `acquire` mode MUST either:
#   (a) Normalize the file to LF and write a valid factory_lock block, OR
#   (b) Fail loud with a non-zero exit and an actionable error.
#
# Under no circumstances may the helper exit 0 with no block written (silent no-op).
#
# Failure mode: the current awk pattern `/^---$/` matches a bare `---` line
# terminated by LF.  With CRLF, the line is `---\r` which does NOT match the
# pattern — the closing --- is invisible to awk, so _write_factory_lock_block
# never fires the insertion, yet the helper exits 0 having written nothing.
#
# The production-grade behavior is normalize-and-write (option a).
# The test asserts option (a): factory_lock block IS written after acquire.
#
# RED GATE: current impl exits 0 without writing the block — the assertion
# that factory_lock is present in the file will fail.
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_crlf_frontmatter_handled" {
  # Build a CRLF fixture using printf with octal escapes (portable across
  # bash versions that may not support printf '--' syntax)
  local crlf_state="$BATS_TEST_TMPDIR/crlf-state.md"
  python3 -c "
import sys
content = (
    '---\r\n'
    'document_type: state\r\n'
    'version: \"0.0.1-test\"\r\n'
    'phase: test\r\n'
    'current_step: \"test-step\"\r\n'
    '---\r\n'
    '\r\n'
    '# STATE (CRLF fixture)\r\n'
)
sys.stdout.buffer.write(content.encode('utf-8'))
" > "$crlf_state"

  # Precondition: file has CRLF line endings (the --- lines end in \r\n)
  # Verify by checking for carriage-return bytes
  local cr_count
  cr_count="$(tr -cd '\r' < "$crlf_state" | wc -c | tr -d ' ')"
  [ "$cr_count" -gt 0 ]

  run bash "$HELPER" acquire "$crlf_state"

  # MUST exit 0 (normalize-and-write is the production-grade path)
  [ "$status" -eq 0 ]

  # MUST have written a factory_lock block (no silent no-op)
  grep -q 'factory_lock:' "$crlf_state"
  grep -q 'holder:' "$crlf_state"
  grep -q 'locked_at:' "$crlf_state"
  grep -q 'expires_at:' "$crlf_state"

  # The written block MUST have correct TTL (delta == 2700)
  local actual_locked_at actual_expires_at epoch_locked epoch_expires delta
  actual_locked_at="$(_get_lock_field "$crlf_state" locked_at)"
  actual_expires_at="$(_get_lock_field "$crlf_state" expires_at)"
  [[ "$actual_locked_at" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$ ]]
  [[ "$actual_expires_at" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$ ]]
  epoch_locked="$(_iso_to_epoch "$actual_locked_at")"
  epoch_expires="$(_iso_to_epoch "$actual_expires_at")"
  delta=$(( epoch_expires - epoch_locked ))
  [ "$delta" -eq 2700 ]
}

# ---------------------------------------------------------------------------
# test_BC_5_40_001_three_sequential_renewals_preserve_locked_at
# EC-009 / BC-5.40.001 PC4 + Invariant 3 — locked_at is immutable across renewals;
# expires_at advances on each renewal.
#
# Scenario: acquire once, then renew 3 times (with small real sleeps to produce
# distinct expires_at values). After all renewals:
#   - locked_at MUST be byte-identical to the value written by acquire.
#   - expires_at MUST have a DIFFERENT value after EACH renewal (advancing each time).
#   - The holder MUST be unchanged throughout.
#
# This is the EC-009 long-burst scenario executed at small scale (3 renewals).
#
# RED GATE: the current implementation correctly preserves locked_at across
# renewals (this part passes today). However, EC-009 is not yet covered by any
# test — the test is NEW and RED because a potential regression in renewal
# (e.g., renew accidentally overwriting locked_at) is not yet caught.
#
# Wait — actually: this test may PASS with the current impl. Re-reading the
# RED gate requirement: a test for EC-009 doesn't exist yet (gap in coverage),
# and the adversary noted it as missing. The test exercises EC-009's edge case
# explicitly. Given the current impl does handle this correctly, this test will
# GREEN on current impl — which means it is a COVERAGE addition, not a defect
# discovery. Per instructions, EC-009 is listed as a new test to add.
#
# NOTE: This test is expected to be GREEN on the current implementation (it
# covers a gap in test coverage rather than a code defect). It is listed as a
# required addition by the adversary's gap analysis.
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_three_sequential_renewals_preserve_locked_at" {
  _fixture_no_lock "$FIXTURE_STATE"

  # Step 1: acquire to establish the lock and capture locked_at
  run bash "$HELPER" acquire "$FIXTURE_STATE"
  [ "$status" -eq 0 ]

  local original_locked_at original_holder
  original_locked_at="$(_get_lock_field "$FIXTURE_STATE" locked_at)"
  original_holder="$(_get_lock_field "$FIXTURE_STATE" holder)"

  # Verify locked_at is a valid ISO-8601 UTC timestamp
  [[ "$original_locked_at" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$ ]]

  local prev_expires_at
  prev_expires_at="$(_get_lock_field "$FIXTURE_STATE" expires_at)"

  # Steps 2–4: three sequential renewals
  local i
  for i in 1 2 3; do
    # Small sleep to ensure the renewal timestamp differs from the previous one
    sleep 1

    run bash "$HELPER" renew "$FIXTURE_STATE"
    [ "$status" -eq 0 ]

    # locked_at MUST be byte-identical to the original acquire value
    local current_locked_at
    current_locked_at="$(_get_lock_field "$FIXTURE_STATE" locked_at)"
    [ "$current_locked_at" = "$original_locked_at" ]

    # holder MUST be unchanged
    local current_holder
    current_holder="$(_get_lock_field "$FIXTURE_STATE" holder)"
    [ "$current_holder" = "$original_holder" ]

    # expires_at MUST have advanced from the previous value
    local current_expires_at
    current_expires_at="$(_get_lock_field "$FIXTURE_STATE" expires_at)"
    [ "$current_expires_at" != "$prev_expires_at" ]

    # expires_at MUST be a valid ISO-8601 UTC timestamp
    [[ "$current_expires_at" =~ ^[0-9]{4}-[0-9]{2}-[0-9]{2}T[0-9]{2}:[0-9]{2}:[0-9]{2}Z$ ]]

    # expires_at - locked_at MUST still be exactly 2700 seconds
    local epoch_locked epoch_expires delta
    epoch_locked="$(_iso_to_epoch "$original_locked_at")"
    epoch_expires="$(_iso_to_epoch "$current_expires_at")"
    delta=$(( epoch_expires - epoch_locked ))
    # After each renewal: expires_at = now+2700, locked_at = original_locked_at
    # delta grows beyond 2700 — that is correct and expected (renewal is not constrained
    # to exactly 2700 from original locked_at; it is now+2700 from the renewal instant).
    # Assert only that expires_at is in the FUTURE relative to original locked_at.
    [ "$delta" -gt 2700 ]

    prev_expires_at="$current_expires_at"
  done

  # Final state: locked_at must still be byte-identical to the original
  local final_locked_at
  final_locked_at="$(_get_lock_field "$FIXTURE_STATE" locked_at)"
  [ "$final_locked_at" = "$original_locked_at" ]
}

# ---------------------------------------------------------------------------
# test_BC_5_40_001_clear_handles_crlf_frontmatter
# F-R1-001 / BC-5.40.001 PC2 — clear MUST remove factory_lock from CRLF STATE.md
#
# When STATE.md has CRLF line endings AND holds a real factory_lock block,
# `factory-lock-write.sh clear` MUST remove the factory_lock key entirely.
#
# Failure mode: _remove_factory_lock's awk uses /^---$/ to count frontmatter
# fences. With CRLF, the delimiter line is `---\r` which does NOT match /^---$/.
# awk's fence counter stays at 0 (never enters "fence == 1" mode), so the
# /^factory_lock:/ branch is never reached. The file is written back unchanged
# — the factory_lock key REMAINS — yet the helper prints success and exits 0.
#
# RED GATE: current clear mode does NOT call _normalize_crlf before
# _remove_factory_lock, so factory_lock key is still present after clear.
# The colon-anchored grep assertion will return 0 (key found) where we
# assert non-zero (key absent) → RED.
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_clear_handles_crlf_frontmatter" {
  # Build a CRLF fixture that HOLDS a factory_lock block in frontmatter
  local crlf_state="$BATS_TEST_TMPDIR/crlf-locked-state.md"
  python3 -c "
import sys
content = (
    '---\r\n'
    'document_type: state\r\n'
    'version: \"0.0.1-test\"\r\n'
    'phase: test\r\n'
    'current_step: \"test-step\"\r\n'
    'factory_lock:\r\n'
    '  holder: \"dev@example.com\"\r\n'
    '  locked_at: \"2026-06-10T14:00:00Z\"\r\n'
    '  expires_at: \"2026-06-10T14:45:00Z\"\r\n'
    '---\r\n'
    '\r\n'
    '# STATE (CRLF locked fixture)\r\n'
)
sys.stdout.buffer.write(content.encode('utf-8'))
" > "$crlf_state"

  # Precondition: file has CRLF line endings
  local cr_count
  cr_count="$(tr -cd '\r' < "$crlf_state" | wc -c | tr -d ' ')"
  [ "$cr_count" -gt 0 ]

  # Precondition: factory_lock key is present before clear
  grep -q 'factory_lock:' "$crlf_state"

  run bash "$HELPER" clear "$crlf_state"

  # Must exit 0 (clear succeeds)
  [ "$status" -eq 0 ]

  # The factory_lock KEY must be ABSENT from the frontmatter after clear.
  # Use a colon-anchored grep on the frontmatter region specifically.
  # Current impl: _remove_factory_lock's awk /^---$/ doesn't match ---\r so
  # the block is NOT removed → factory_lock: still present → grep returns 0
  # → [ $status -ne 0 ] assertion FAILS → RED.
  local frontmatter_only
  frontmatter_only="$(awk '/^---/{gsub(/\r/,"")} /^---$/{f++} f==1{print} f==2{exit}' "$crlf_state")"
  if echo "$frontmatter_only" | grep -q 'factory_lock:'; then
    printf 'FAIL: factory_lock key still present in frontmatter after clear on CRLF file\n' >&2
    printf 'Frontmatter contents:\n%s\n' "$frontmatter_only" >&2
    false
  fi
}

# ---------------------------------------------------------------------------
# test_BC_5_40_001_renew_handles_crlf_frontmatter
# F-R1-001 / BC-5.40.001 PC4 — renew MUST advance expires_at on CRLF STATE.md
#
# When STATE.md has CRLF line endings AND holds a valid factory_lock block
# (with expires_at), `factory-lock-write.sh renew` MUST update expires_at.
#
# Failure mode: renew's no-op check uses awk /^---$/ to detect the factory_lock
# block in frontmatter. With CRLF, /^---$/ never matches → the frontmatter-
# scoped factory_lock check returns "not found" → renew treats it as a no-op
# and exits 0 without updating expires_at. The expires_at value is UNCHANGED.
#
# RED GATE: current renew mode does NOT call _normalize_crlf before its
# frontmatter-scoped awk checks. The no-op branch fires silently. The
# expires_at UNCHANGED assertion will hold (values equal) → the
# [ "$actual_expires_at" != "$original_expires_at" ] assertion FAILS → RED.
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_renew_handles_crlf_frontmatter" {
  local original_expires_at="2026-06-10T14:45:00Z"

  # Build a CRLF fixture holding a valid lock with a known expires_at
  local crlf_state="$BATS_TEST_TMPDIR/crlf-locked-renew.md"
  python3 -c "
import sys
content = (
    '---\r\n'
    'document_type: state\r\n'
    'version: \"0.0.1-test\"\r\n'
    'phase: test\r\n'
    'current_step: \"test-step\"\r\n'
    'factory_lock:\r\n'
    '  holder: \"dev@example.com\"\r\n'
    '  locked_at: \"2026-06-10T14:00:00Z\"\r\n'
    '  expires_at: \"2026-06-10T14:45:00Z\"\r\n'
    '---\r\n'
    '\r\n'
    '# STATE (CRLF renew fixture)\r\n'
)
sys.stdout.buffer.write(content.encode('utf-8'))
" > "$crlf_state"

  # Precondition: CRLF line endings present
  local cr_count
  cr_count="$(tr -cd '\r' < "$crlf_state" | wc -c | tr -d ' ')"
  [ "$cr_count" -gt 0 ]

  # Precondition: factory_lock and expires_at are present (raw grep, ignoring \r)
  grep -q 'factory_lock' "$crlf_state"
  grep -q 'expires_at' "$crlf_state"

  run bash "$HELPER" renew "$crlf_state"

  # Must exit 0 (renew succeeds — normalize-and-renew is the production-grade path)
  [ "$status" -eq 0 ]

  # expires_at MUST have been updated (not still the original value).
  # Current impl: renew's awk no-op check fires on CRLF → exits 0 without
  # modifying expires_at → value is UNCHANGED → assertion fails → RED.
  local actual_expires_at
  # Strip \r when reading the value so comparison works
  actual_expires_at="$(tr -d '\r' < "$crlf_state" | awk '/^---$/{f++} f==1 && /^  expires_at:/{gsub(/^  expires_at: *"?/,""); gsub(/"$/,""); print; exit} f>=2{exit}')"

  if [ "$actual_expires_at" = "$original_expires_at" ]; then
    printf 'FAIL: expires_at was NOT updated after renew on CRLF file (still: %s)\n' \
      "$actual_expires_at" >&2
    printf 'Indicates renew hit the CRLF-caused no-op branch\n' >&2
    false
  fi

  # locked_at MUST remain unchanged (immutable after acquire)
  local actual_locked_at
  actual_locked_at="$(tr -d '\r' < "$crlf_state" | awk '/^---$/{f++} f==1 && /^  locked_at:/{gsub(/^  locked_at: *"?/,""); gsub(/"$/,""); print; exit} f>=2{exit}')"
  [ "$actual_locked_at" = "2026-06-10T14:00:00Z" ]
}

# ---------------------------------------------------------------------------
# test_BC_5_40_001_clear_on_held_lock_asserts_removal
# F-R1-001 (LF regression guard) / BC-5.40.001 PC2 — clear on LF STATE.md
# removes the factory_lock key (regression guard for the LF path).
#
# This test pins the post-clear assertion that the implementer will verify.
# The LF path MAY already pass today (the existing _remove_factory_lock awk
# works on LF files). Its purpose is to prevent a future regression where
# a CRLF fix accidentally breaks the LF path.
#
# If this test passes on current impl (LF path works), it is a GREEN
# regression guard — that is acceptable per instructions. The CRLF-specific
# RED tests are tests 1 and 2 above.
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_clear_on_held_lock_asserts_removal" {
  # Use the standard LF fixture helper
  _fixture_with_lock "$FIXTURE_STATE" \
    "developer@example.com" \
    "2026-06-10T14:00:00Z" \
    "2026-06-10T14:45:00Z"

  # Precondition: key is present
  grep -q 'factory_lock:' "$FIXTURE_STATE"

  run bash "$HELPER" clear "$FIXTURE_STATE"

  # Must exit 0
  [ "$status" -eq 0 ]

  # factory_lock key MUST be absent from the frontmatter
  local frontmatter_only
  frontmatter_only="$(awk '/^---$/{f++} f==1{print} f==2{exit}' "$FIXTURE_STATE")"
  if echo "$frontmatter_only" | grep -q 'factory_lock:'; then
    printf 'FAIL: factory_lock key still present in frontmatter after clear on LF file\n' >&2
    false
  fi
}

# ---------------------------------------------------------------------------
# _file_mode helper
#
# Returns the octal permission bits of a file as a 3-digit string (e.g. "644").
# Mirrors the BSD/GNU branch pattern used by _iso_to_epoch above:
#   BSD/macOS: stat -f '%Lp' <file>
#   GNU/Linux: stat -c '%a'  <file>
# ---------------------------------------------------------------------------
_file_mode() {
  local file="$1"
  if stat --version >/dev/null 2>&1; then
    # GNU stat
    stat -c '%a' "$file"
  else
    # BSD stat (macOS)
    stat -f '%Lp' "$file"
  fi
}

# ---------------------------------------------------------------------------
# test_BC_5_40_001_acquire_preserves_file_mode
# Portability / BC-5.40.001 PC1 — mktemp+mv write MUST preserve original
# file permissions on BSD/macOS.
#
# Defect: every awk helper that performs mktemp+mv uses:
#   chmod --reference="$file" "$tmpfile" 2>/dev/null || true
# `chmod --reference` is GNU-only.  On BSD/macOS it is silently a no-op,
# so the mv replaces the original file with the mktemp-created tmpfile whose
# permissions are 0600 (mktemp default).  A STATE.md that was 0644 becomes
# 0600 after the first acquire (or renew, or clear) on macOS.
#
# This test:
#   1. Creates a fixture STATE.md and sets its mode to 0644.
#   2. Runs factory-lock-write.sh acquire on it.
#   3. Asserts that the resulting file mode is STILL 0644.
#
# RED GATE on macOS: chmod --reference silently fails, mktemp leaves tmpfile
# at 0600, mv replaces the 0644 original with 0600 — the mode assertion fails
# with actual=600 expected=644.
#
# Also asserts mode preservation after renew and clear (cheap: same code path).
# ---------------------------------------------------------------------------

@test "test_BC_5_40_001_acquire_preserves_file_mode" {
  _fixture_no_lock "$FIXTURE_STATE"

  # Set an explicit mode that differs from mktemp's default (0600)
  chmod 0644 "$FIXTURE_STATE"

  # Precondition: mode is 644 before any helper invocation
  local pre_mode
  pre_mode="$(_file_mode "$FIXTURE_STATE")"
  [ "$pre_mode" = "644" ]

  # --- acquire ---
  run bash "$HELPER" acquire "$FIXTURE_STATE"
  [ "$status" -eq 0 ]

  local post_acquire_mode
  post_acquire_mode="$(_file_mode "$FIXTURE_STATE")"
  if [ "$post_acquire_mode" != "644" ]; then
    printf 'FAIL acquire: expected mode 644 but got %s\n' "$post_acquire_mode" >&2
    false
  fi

  # --- renew ---
  run bash "$HELPER" renew "$FIXTURE_STATE"
  [ "$status" -eq 0 ]

  local post_renew_mode
  post_renew_mode="$(_file_mode "$FIXTURE_STATE")"
  if [ "$post_renew_mode" != "644" ]; then
    printf 'FAIL renew: expected mode 644 but got %s\n' "$post_renew_mode" >&2
    false
  fi

  # --- clear ---
  run bash "$HELPER" clear "$FIXTURE_STATE"
  [ "$status" -eq 0 ]

  local post_clear_mode
  post_clear_mode="$(_file_mode "$FIXTURE_STATE")"
  if [ "$post_clear_mode" != "644" ]; then
    printf 'FAIL clear: expected mode 644 but got %s\n' "$post_clear_mode" >&2
    false
  fi
}
