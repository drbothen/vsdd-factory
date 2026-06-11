#!/usr/bin/env bats
# factory-lock-acquire-precheck.bats — RED-phase (TDD) tests for BC-6.23.001
# Preconditions 2+3, PC3, AC-003, AC-009, AC-010, AC-012, EC-001, EC-002, EC-006, EC-007.
#
# Traces to: BC-6.23.001 Pre-2 (fetch-before-check), Pre-3 (git email required),
#            PC3 (foreign lock refusal 5-field message), EC-001 (self-held noop),
#            EC-002 (expired treated as absent), EC-006 (fetch failure abort),
#            EC-007 (empty email abort). AC-003, AC-009, AC-010, AC-012.
# Story: S-17.03 — /factory-lock + /factory-unlock skills + health status.
# Target: plugins/vsdd-factory/bin/factory-lock-acquire-precheck.sh
#
# RED GATE: All tests MUST FAIL because the helper is a stub that exits 1 with a
# TODO message. Tests fail with ASSERTION errors (wrong exit code, wrong output/error
# content), NOT "file not found" errors — the stub exists, validates its arguments,
# then exits 1 via the TODO path.
#
# Git stub strategy:
#   For tests exercising git operations (fetch failure, empty email), a minimal
#   git shim is installed at $WORK/stub-bin/git. The shim intercepts 'fetch' and
#   'config' subcommands; all others delegate to the real git binary.
#   The helper runs `git fetch origin factory-artifacts` and `git config user.email`
#   internally — the shim controls their exit codes/output.
#
# Self-contained fixtures in BATS_TEST_TMPDIR — never touch real .factory/ or origin.
#
# Run:
#   bats plugins/vsdd-factory/tests/factory-lock-acquire-precheck.bats

HELPER="$(cd "$(dirname "$BATS_TEST_FILENAME")/.." && pwd)/bin/factory-lock-acquire-precheck.sh"
REAL_GIT_PATH="$(command -v git)"

# ---------------------------------------------------------------------------
# Fixed timestamps used across fixtures — far future so they never expire
# ---------------------------------------------------------------------------
FUTURE_LOCKED_AT="2099-01-01T00:00:00Z"
FUTURE_EXPIRES_AT="2099-01-01T00:45:00Z"
PAST_LOCKED_AT="2020-01-01T00:00:00Z"
PAST_EXPIRES_AT="2020-01-01T00:45:00Z"

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

# Write a git shim to $WORK/stub-bin/git.
# The shim controls fetch and config subcommand behavior.
# Arguments:
#   $1: fetch_exit_code — exit code for 'git fetch' (0 = success, 1 = failure)
#   $2: email_output — output for 'git config user.email' (empty string = unconfigured)
#   $3: email_exit_code — exit code for 'git config user.email' (0 or 1)
_write_git_shim() {
  local fetch_exit="$1"
  local email_output="$2"
  local email_exit="$3"

  local stub_bin="$WORK/stub-bin"
  mkdir -p "$stub_bin"

  cat > "$stub_bin/git" <<SHIM
#!/usr/bin/env bash
# Git shim for factory-lock-acquire-precheck.bats
# Parses subcommand (skipping -C <path> flag pairs) and dispatches.
REAL_GIT="${REAL_GIT_PATH}"

args=("\$@")
i=0
subcommand=""
while [ \$i -lt \${#args[@]} ]; do
  arg="\${args[\$i]}"
  if [ "\$arg" = "-C" ]; then
    i=\$(( i + 1 ))
  else
    subcommand="\$arg"
    break
  fi
  i=\$(( i + 1 ))
done

case "\$subcommand" in
  fetch)
    if [ "${fetch_exit}" -ne 0 ]; then
      printf 'fatal: unable to connect to origin\\n' >&2
    fi
    exit ${fetch_exit}
    ;;
  config)
    # Only intercept 'git config user.email'
    if [[ "\$*" == *"user.email"* ]]; then
      if [ -n "${email_output}" ]; then
        printf '%s\\n' "${email_output}"
      fi
      exit ${email_exit}
    fi
    # All other config lookups delegate to real git
    "\$REAL_GIT" "\$@"
    ;;
  *)
    "\$REAL_GIT" "\$@"
    ;;
esac
SHIM
  chmod +x "$stub_bin/git"
  STUB_BIN="$stub_bin"
}

# ---------------------------------------------------------------------------
# setup / teardown
# ---------------------------------------------------------------------------

setup() {
  WORK="$(mktemp -d)"
  WORK="$(cd "$WORK" && pwd -P)"
  FIXTURE_STATE="$WORK/STATE.md"
  STUB_BIN=""
}

teardown() {
  rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_acquire_precheck_fetch_failure_aborts
# AC-009 / BC-6.23.001 Pre-2, EC-006 — fetch failure → hard abort
#
# When 'git fetch origin factory-artifacts' returns non-zero, the helper MUST:
#   - Exit NON-ZERO (exit 2 per stub header spec).
#   - Print "Fetch failed before lock check. Cannot acquire safely." to stderr.
#   - NOT read or modify STATE.md.
#
# RED GATE: stub exits 1 with TODO (wrong exit path entirely, wrong message) →
# exit-code assertion and stderr-content assertions both fail.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_acquire_precheck_fetch_failure_aborts" {
  _fixture_no_lock "$FIXTURE_STATE"
  # Shim: fetch fails, email would succeed (irrelevant — fetch guard fires first)
  _write_git_shim 1 "dev@example.com" 0

  run env PATH="${STUB_BIN}:${PATH}" bash "$HELPER" "$FIXTURE_STATE"

  # Must exit non-zero (EC-006 hard abort)
  [ "$status" -ne 0 ]

  # Stderr MUST contain the exact EC-006 abort message
  [[ "$output" == *"Fetch failed before lock check. Cannot acquire safely."* ]]
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_acquire_precheck_empty_email_rejected
# AC-010 / BC-6.23.001 Pre-3, EC-007 — empty git user.email → hard abort
#
# When 'git config user.email' returns empty output (or exits non-zero),
# the helper MUST:
#   - Exit NON-ZERO (exit 2 per spec).
#   - Print a message to stderr referencing "git user.email not configured".
#   - NOT proceed to lock check or write.
#
# RED GATE: stub exits 1 with TODO — fails on non-zero exit assertion.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_acquire_precheck_empty_email_rejected" {
  _fixture_no_lock "$FIXTURE_STATE"
  # Shim: fetch succeeds, email returns empty + exit 1 (unconfigured)
  _write_git_shim 0 "" 1

  run env PATH="${STUB_BIN}:${PATH}" bash "$HELPER" "$FIXTURE_STATE"

  # Must exit non-zero (EC-007 hard abort)
  [ "$status" -ne 0 ]

  # Stderr MUST contain the EC-007 message referencing user.email
  [[ "$output" == *"git user.email not configured"* ]]
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_acquire_precheck_self_held_noop
# AC-012 / BC-6.23.001 EC-001 — self-held unexpired lock → NOOP_SELF_HELD
#
# When factory_lock.holder == current git email and the lock is unexpired,
# the helper MUST:
#   - Exit 0.
#   - Print "NOOP_SELF_HELD" to stdout.
#   - Print "Already held by this session." message (to stdout or stderr).
#
# RED GATE: stub exits 1 with TODO — fails on exit-0 assertion.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_acquire_precheck_self_held_noop" {
  local test_email="self@example.com"
  _fixture_with_lock "$FIXTURE_STATE" \
    "$test_email" \
    "$FUTURE_LOCKED_AT" \
    "$FUTURE_EXPIRES_AT"
  # Shim: fetch succeeds, email returns test_email
  _write_git_shim 0 "$test_email" 0

  run env PATH="${STUB_BIN}:${PATH}" bash "$HELPER" "$FIXTURE_STATE"

  # Must exit 0 (self-held is a noop, not an error)
  [ "$status" -eq 0 ]

  # stdout MUST contain the NOOP_SELF_HELD decision token
  [[ "$output" == *"NOOP_SELF_HELD"* ]]

  # Output MUST also mention "Already held by this session"
  [[ "$output" == *"Already held by this session"* ]]
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_acquire_precheck_foreign_lock_refusal_all_five_fields
# AC-003 / BC-6.23.001 PC3 — foreign unexpired lock → REFUSED_FOREIGN_LOCK
# with all 5 required fields in the refusal message (BC-4.13.001 PC1 format)
#
# When factory_lock.holder != current_email and lock is unexpired, the helper MUST:
#   - Exit 1.
#   - Print "REFUSED_FOREIGN_LOCK" to stderr.
#   - Print a refusal message to stderr containing ALL FIVE required fields:
#     1. Holder email (factory_lock.holder value)
#     2. locked_at timestamp
#     3. expires_at timestamp
#     4. time_remaining (human-readable)
#     5. "/factory-unlock --force" command string
#
# RED GATE: stub exits 1 with TODO (wrong message entirely) — exit-1 assertion
# happens to pass but all field-content assertions fail.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_acquire_precheck_foreign_lock_refusal_all_five_fields" {
  local holder_email="other@example.com"
  local caller_email="dev@example.com"

  _fixture_with_lock "$FIXTURE_STATE" \
    "$holder_email" \
    "$FUTURE_LOCKED_AT" \
    "$FUTURE_EXPIRES_AT"
  # Shim: fetch succeeds, email returns caller (not holder)
  _write_git_shim 0 "$caller_email" 0

  run env PATH="${STUB_BIN}:${PATH}" bash "$HELPER" "$FIXTURE_STATE"

  # Must exit 1 (REFUSED_FOREIGN_LOCK)
  [ "$status" -eq 1 ]

  # Field 1: holder email MUST appear in the output
  [[ "$output" == *"${holder_email}"* ]]

  # Field 2: locked_at MUST appear in the output
  [[ "$output" == *"${FUTURE_LOCKED_AT}"* ]]

  # Field 3: expires_at MUST appear in the output
  [[ "$output" == *"${FUTURE_EXPIRES_AT}"* ]]

  # Field 4: time_remaining — must contain a duration indication
  # (the BC prescribes "human-readable duration"; match "min" or "remaining")
  [[ "$output" == *"remaining"* ]] || [[ "$output" == *"min"* ]]

  # Field 5: /factory-unlock --force command MUST appear verbatim
  [[ "$output" == *"/factory-unlock --force"* ]]

  # Decision token MUST also be present
  [[ "$output" == *"REFUSED_FOREIGN_LOCK"* ]]
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_acquire_precheck_proceed_when_absent
# BC-6.23.001 PC1 — absent factory_lock → PROCEED_ACQUIRE
#
# When no factory_lock block is present, the helper MUST:
#   - Exit 0.
#   - Print "PROCEED_ACQUIRE" to stdout.
#
# RED GATE: stub exits 1 with TODO — fails on exit-0 assertion.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_acquire_precheck_proceed_when_absent" {
  _fixture_no_lock "$FIXTURE_STATE"
  # Shim: fetch succeeds, email returns a configured address
  _write_git_shim 0 "dev@example.com" 0

  run env PATH="${STUB_BIN}:${PATH}" bash "$HELPER" "$FIXTURE_STATE"

  # Must exit 0 (absent lock = proceed)
  [ "$status" -eq 0 ]

  # stdout MUST contain the PROCEED_ACQUIRE decision token
  [[ "$output" == *"PROCEED_ACQUIRE"* ]]
}

# ---------------------------------------------------------------------------
# test_BC_6_23_001_acquire_precheck_proceed_when_expired
# BC-6.23.001 EC-002 — expired foreign lock → PROCEED_ACQUIRE
#
# When factory_lock is present but expires_at is in the past, the lock is
# treated as absent. The helper MUST:
#   - Exit 0.
#   - Print "PROCEED_ACQUIRE" to stdout.
#
# RED GATE: stub exits 1 with TODO — fails on exit-0 assertion.
# ---------------------------------------------------------------------------

@test "test_BC_6_23_001_acquire_precheck_proceed_when_expired" {
  # Expired lock held by some other developer
  _fixture_with_lock "$FIXTURE_STATE" \
    "other@example.com" \
    "$PAST_LOCKED_AT" \
    "$PAST_EXPIRES_AT"
  # Shim: fetch succeeds, email returns caller (not holder — but lock expired anyway)
  _write_git_shim 0 "dev@example.com" 0

  run env PATH="${STUB_BIN}:${PATH}" bash "$HELPER" "$FIXTURE_STATE"

  # Must exit 0 (expired lock = treat as absent = proceed)
  [ "$status" -eq 0 ]

  # stdout MUST contain the PROCEED_ACQUIRE decision token
  [[ "$output" == *"PROCEED_ACQUIRE"* ]]
}
