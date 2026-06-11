#!/usr/bin/env bats
# factory-lock-status.bats — RED-phase (TDD) tests for BC-6.23.001 PC7 + PC8:
# three-state factory lock display helper.
#
# Traces to: BC-6.23.001 PC7 (three-state display), PC8 (shared-helper mandate),
#            AC-007, AC-008.
# Story: S-17.03 — /factory-lock + /factory-unlock skills + health status.
# Target: plugins/vsdd-factory/bin/factory-lock-status.sh
#
# RED GATE: All tests against factory-lock-status.sh MUST FAIL because the helper
# is a stub that exits 1 with a TODO message (without printing any display string).
# Tests fail with ASSERTION errors (wrong exit code, wrong output) — not "file not
# found" errors (the stub exists and validates its arguments before exiting 1).
#
# test_BC_6_23_001_factory_lock_status_sh_shared_by_both_health_skills is RED
# because neither factory-health/SKILL.md nor factory-worktree-health/SKILL.md
# currently invoke factory-lock-status.sh (the amendments are part of S-17.03
# implementation scope, not yet done).
#
# Fixture convention: each test builds a minimal STATE.md in BATS_TEST_TMPDIR
# containing a YAML frontmatter region bounded by --- delimiters. The helper
# accepts <state_md_path> <current_git_email> and prints its output to stdout.
#
# Run:
#   bats plugins/vsdd-factory/tests/factory-lock-status.bats

HELPER="$(cd "$(dirname "$BATS_TEST_FILENAME")/.." && pwd)/bin/factory-lock-status.sh"
SKILL_DIR="$(cd "$(dirname "$BATS_TEST_FILENAME")/.." && pwd)/skills"

# ---------------------------------------------------------------------------
# Fixed timestamps used across fixtures — far future so they never expire
# ---------------------------------------------------------------------------
FUTURE_LOCKED_AT="2099-01-01T00:00:00Z"
FUTURE_EXPIRES_AT="2099-01-01T00:45:00Z"

# ---------------------------------------------------------------------------
# Fixture helpers
# ---------------------------------------------------------------------------

# Write a minimal STATE.md to $1 with NO factory_lock key (unlocked baseline).
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
# test_BC_6_23_001_factory_lock_status_sh_free_when_absent
# AC-007 / BC-6.23.001 PC7 — absent factory_lock block → "Factory lock: FREE"
#
# When STATE.md has NO factory_lock key, the helper MUST print:
#   Factory lock: FREE
# Exit code MUST be 0.
#
# RED GATE: stub exits 1 with TODO — fails on non-zero exit assertion.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_factory_lock_status_sh_free_when_absent" {
  _fixture_no_lock "$FIXTURE_STATE"

  run bash "$HELPER" "$FIXTURE_STATE" "dev@example.com"

  # Must exit 0 (display-only helper always exits 0)
  [ "$status" -eq 0 ]

  # Output MUST be exactly the FREE display string
  [ "$output" = "Factory lock: FREE" ]
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_factory_lock_status_sh_free_when_expired
# AC-007 / BC-6.23.001 PC7 — expired factory_lock block → "Factory lock: FREE"
#
# When factory_lock.expires_at is in the past (expired), the helper MUST print:
#   Factory lock: FREE
# Exit code MUST be 0.
# The expiry check uses now >= expires_at (BC-4.13.001 PC2 boundary semantics).
#
# RED GATE: stub exits 1 with TODO — fails on non-zero exit assertion.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_factory_lock_status_sh_free_when_expired" {
  # Use a well-past expires_at to guarantee expiry on any system clock
  _fixture_with_lock "$FIXTURE_STATE" \
    "other@example.com" \
    "2020-01-01T00:00:00Z" \
    "2020-01-01T00:45:00Z"

  run bash "$HELPER" "$FIXTURE_STATE" "dev@example.com"

  # Must exit 0
  [ "$status" -eq 0 ]

  # Must output FREE (expired lock = unlocked)
  [ "$output" = "Factory lock: FREE" ]
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_factory_lock_status_sh_self_held
# AC-007 / BC-6.23.001 PC7 — self-held unexpired lock →
# "Factory lock: HELD by this session (expires <expires_at>)"
#
# When holder == current_git_email AND now < expires_at, the helper MUST print:
#   Factory lock: HELD by this session (expires <expires_at>)
# where <expires_at> is the exact value from the fixture.
# Exit code MUST be 0.
#
# RED GATE: stub exits 1 with TODO — fails on non-zero exit assertion.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_factory_lock_status_sh_self_held" {
  local test_email="self@example.com"

  _fixture_with_lock "$FIXTURE_STATE" \
    "$test_email" \
    "$FUTURE_LOCKED_AT" \
    "$FUTURE_EXPIRES_AT"

  run bash "$HELPER" "$FIXTURE_STATE" "$test_email"

  # Must exit 0
  [ "$status" -eq 0 ]

  # Output MUST be the self-held display string with the exact expires_at from fixture
  [ "$output" = "Factory lock: HELD by this session (expires ${FUTURE_EXPIRES_AT})" ]
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_factory_lock_status_sh_foreign_held
# AC-007 / BC-6.23.001 PC7 — foreign unexpired lock →
# "Factory lock: HELD by <holder> since <locked_at> (expires <expires_at>)"
#
# When holder != current_git_email AND now < expires_at, the helper MUST print:
#   Factory lock: HELD by <holder_email> since <locked_at> (expires <expires_at>)
# with the exact holder/locked_at/expires_at values from the fixture.
# Exit code MUST be 0.
#
# RED GATE: stub exits 1 with TODO — fails on non-zero exit assertion.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_factory_lock_status_sh_foreign_held" {
  local holder_email="other@example.com"
  local caller_email="dev@example.com"

  _fixture_with_lock "$FIXTURE_STATE" \
    "$holder_email" \
    "$FUTURE_LOCKED_AT" \
    "$FUTURE_EXPIRES_AT"

  run bash "$HELPER" "$FIXTURE_STATE" "$caller_email"

  # Must exit 0
  [ "$status" -eq 0 ]

  # Output MUST be the foreign-held display string with all three interpolated fields
  local expected="Factory lock: HELD by ${holder_email} since ${FUTURE_LOCKED_AT} (expires ${FUTURE_EXPIRES_AT})"
  [ "$output" = "$expected" ]
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_factory_lock_status_sh_malformed_block
# AC-007 / BC-6.23.001 PC7 — malformed factory_lock block →
# "Factory lock: FREE (malformed block — treated as unlocked)"
#
# When factory_lock: is present but required sub-fields are missing or
# unparseable, the helper MUST print:
#   Factory lock: FREE (malformed block — treated as unlocked)
# Exit code MUST be 0 (fail-open, display-only).
#
# RED GATE: stub exits 1 with TODO — fails on non-zero exit assertion.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_factory_lock_status_sh_malformed_block" {
  # Write a STATE.md with factory_lock: key but missing required sub-fields
  cat > "$FIXTURE_STATE" <<'FIXTURE'
---
document_type: state
version: "0.0.1-test"
phase: test
factory_lock:
  holder: ""
---

# STATE (test fixture)
Malformed block — holder is empty string, locked_at and expires_at absent.
FIXTURE

  run bash "$HELPER" "$FIXTURE_STATE" "dev@example.com"

  # Must exit 0 (malformed = fail-open)
  [ "$status" -eq 0 ]

  # Output MUST be the malformed-block FREE display string
  [ "$output" = "Factory lock: FREE (malformed block — treated as unlocked)" ]
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_factory_lock_status_sh_shared_by_both_health_skills
# AC-008 / BC-6.23.001 PC8 — structural test: BOTH health skills MUST invoke
# factory-lock-status.sh (the shared-helper mandate).
#
# Greps factory-health/SKILL.md AND factory-worktree-health/SKILL.md to confirm
# each file contains an invocation of factory-lock-status.sh. This enforces the
# BC-6.23.001 PC8 invariant: the two skills cannot diverge on lock status display
# format because they both delegate to the same shared helper.
#
# RED GATE: neither SKILL.md currently invokes factory-lock-status.sh (the
# amendments are S-17.03 implementation scope). Both greps return non-zero →
# assertions fail.
# ---------------------------------------------------------------------------

# ---------------------------------------------------------------------------
# test_BC_6_23_001_factory_lock_status_sh_crlf_foreign_held
# F-1 / BC-6.23.001 PC7 — CRLF line endings in STATE.md must not cause
# foreign-held status to be misreported as FREE (malformed).
#
# The S-17.02 guard normalizes \r\n→\n before parsing factory_lock; the
# status helper MUST apply the same normalization so a CRLF STATE.md with a
# valid foreign unexpired lock produces:
#   Factory lock: HELD by <holder> since <locked_at> (expires <expires_at>)
# NOT "Factory lock: FREE (malformed block — treated as unlocked)".
#
# Fixture: written via printf with \r\n line endings (CRLF throughout).
#
# RED GATE: the current stub exits 1 with TODO, so the exit-0 assertion fails
# first. After stub replacement the CRLF normalization gap will cause the
# assertion to fail on wrong output before implementation adds normalization.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_factory_lock_status_sh_crlf_foreign_held" {
  local holder_email="other@example.com"
  local caller_email="dev@example.com"

  # Build a CRLF fixture using printf '%s\r\n' (portable: no leading -- flag needed).
  {
    printf '%s\r\n' '---'
    printf '%s\r\n' 'document_type: state'
    printf '%s\r\n' 'version: "0.0.1-test"'
    printf '%s\r\n' 'phase: test'
    printf '%s\r\n' 'current_step: "test-step"'
    printf '%s\r\n' 'factory_lock:'
    printf '  holder: "%s"\r\n' "$holder_email"
    printf '  locked_at: "%s"\r\n' "$FUTURE_LOCKED_AT"
    printf '  expires_at: "%s"\r\n' "$FUTURE_EXPIRES_AT"
    printf '%s\r\n' '---'
    printf '\r\n'
    printf '%s\r\n' '# STATE (CRLF fixture)'
    printf '%s\r\n' 'Foreign unexpired lock — CRLF line endings throughout.'
  } > "$FIXTURE_STATE"

  run bash "$HELPER" "$FIXTURE_STATE" "$caller_email"

  # Must exit 0 (display-only helper always exits 0)
  [ "$status" -eq 0 ]

  # MUST report HELD (not FREE / malformed) — CRLF normalization required
  local expected="Factory lock: HELD by ${holder_email} since ${FUTURE_LOCKED_AT} (expires ${FUTURE_EXPIRES_AT})"
  [ "$output" = "$expected" ] \
    || { printf 'FAIL: expected "%s"\n  got "%s"\n' "$expected" "$output" >&2; false; }
}

# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_factory_lock_status_sh_shared_by_both_health_skills" {
  local health_skill="$SKILL_DIR/factory-health/SKILL.md"
  local worktree_health_skill="$SKILL_DIR/factory-worktree-health/SKILL.md"

  # Both files must exist (pre-condition — they are S-17.03 amendment targets)
  [ -f "$health_skill" ]
  [ -f "$worktree_health_skill" ]

  # factory-health/SKILL.md MUST contain an invocation of factory-lock-status.sh
  grep -q 'factory-lock-status.sh' "$health_skill"

  # factory-worktree-health/SKILL.md MUST contain an invocation of factory-lock-status.sh
  grep -q 'factory-lock-status.sh' "$worktree_health_skill"
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_factory_lock_status_sh_crlf_no_tempfile_leak
# F-1703-001 / BC-6.23.001 — CRLF normalization MUST NOT leak a temp file
# beside STATE.md in .factory/.
#
# Bug: _normalize_crlf_for_read sets _STATUS_TMPFILE INSIDE a command-
# substitution subshell.  The parent process's EXIT trap never sees it, so
# the CR-stripped temp file (mktemp "${file}.XXXXXX") survives the helper.
# Empirically: a CRLF read of STATE.md leaves STATE.md.XXXXXX beside it.
#
# Contract: after factory-lock-status.sh reads a CRLF STATE.md, the
# STATE.md's directory MUST contain no STATE.md.* temp file (neither
# beside it, nor any stray file).  This assertion is satisfied whether the
# implementer (a) properly cleans up the temp or (b) creates it in
# ${TMPDIR:-/tmp} instead — the .factory/ dir stays clean either way.
#
# RED GATE: the current implementation leaks STATE.md.XXXXXX, so the
# after-listing differs from the before-listing → assertion fails with
# detected-leftover-file evidence.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_factory_lock_status_sh_crlf_no_tempfile_leak" {
  # Use a dedicated subdirectory so the directory listing is fully controlled
  local leakdir="$WORK/leakcheck"
  mkdir -p "$leakdir"
  local crlf_state="$leakdir/STATE.md"

  # Write CRLF fixture (holder = foreign unexpired → helper exits 0 + HELD output)
  {
    printf '%s\r\n' '---'
    printf '%s\r\n' 'document_type: state'
    printf '%s\r\n' 'version: "0.0.1-test"'
    printf '%s\r\n' 'phase: test'
    printf '%s\r\n' 'current_step: "test-step"'
    printf '%s\r\n' 'factory_lock:'
    printf '  holder: "%s"\r\n' "other@example.com"
    printf '  locked_at: "%s"\r\n' "$FUTURE_LOCKED_AT"
    printf '  expires_at: "%s"\r\n' "$FUTURE_EXPIRES_AT"
    printf '%s\r\n' '---'
    printf '\r\n'
    printf '%s\r\n' '# STATE (CRLF leak-check fixture)'
  } > "$crlf_state"

  # Snapshot directory contents BEFORE the helper run
  local before_listing
  before_listing="$(ls "$leakdir")"

  # Run the helper — must succeed (exit 0) to ensure the CRLF path was exercised
  run bash "$HELPER" "$crlf_state" "caller@example.com"
  [ "$status" -eq 0 ] \
    || { printf 'FAIL: helper must exit 0, got exit %s\n  output: %s\n' "$status" "$output" >&2; false; }

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
