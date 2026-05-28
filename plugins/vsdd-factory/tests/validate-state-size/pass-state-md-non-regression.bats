#!/usr/bin/env bats
# pass-state-md-non-regression.bats — AC-6: STATE.md arm unchanged after lessons.md extension
#
# Traces to: BC-7.04.051 non-regression invariants
# Canonical test vector: EC-011 — STATE.md write with 400 lines (within threshold)
#
# After the T-6 refactor (routing via $TARGET variable), the STATE.md arm behavior
# must remain byte-for-byte identical to the original script. This test exercises
# the STATE.md path with a 400-line file (within STATE.md's 500-line limit) to confirm
# the existing arm still exits 0 cleanly after the refactor.
#
# AC-6 tests the EXISTING behavior (STATE.md arm) — it invokes the CURRENT script
# and verifies the non-regression baseline before and after the lessons.md arm is added.
#
# Note: this is the only test in this suite NOT marked skip. It tests existing behavior
# that the current (unmodified) script already handles correctly. It serves as the
# non-regression anchor: if this test breaks after T-6 refactor, the implementer
# has broken the STATE.md arm.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  HOOK="${REPO_ROOT}/plugins/vsdd-factory/hooks/validate-state-size.sh"
  WORK="$(mktemp -d)"
  # Build a minimal git repo so git show HEAD doesn't produce errors
  git -C "${WORK}" init -q
  git -C "${WORK}" config user.email "test@example.com"
  git -C "${WORK}" config user.name "Test"
  mkdir -p "${WORK}/.factory"
}

teardown() {
  rm -rf "${WORK}"
}

# ---------------------------------------------------------------------------
# AC-6a: 400-line STATE.md → exit 0 (within 500-line limit, no warning at 400)
# Traces to BC-7.04.051 existing STATE.md arm (non-regression)
# ---------------------------------------------------------------------------

@test "AC-6a NON-REGRESSION: 400-line STATE.md exits 0 after lessons.md arm refactor (STATE.md arm unchanged)" {
  local state_path="${WORK}/.factory/STATE.md"

  # 400 lines: above the 200-line advisory but below the 500-line block limit
  # The current script emits a STATE.md SIZE WARNING at >200 lines.
  seq 400 > "${state_path}"

  # Commit so git show HEAD doesn't return an error (PRIOR_COUNT defaults to 0 if no commit)
  git -C "${WORK}" add "${WORK}/.factory/STATE.md"
  git -C "${WORK}" commit -q -m "state: 400 lines"

  local payload
  payload="$(jq -nc --arg fp "${state_path}" '{tool_input: {file_path: $fp}}')"

  run bash -c "printf '%s' '${payload}' | bash '${HOOK}' 2>/tmp/stderr-ac6a-$$"
  local stderr_content
  stderr_content="$(cat /tmp/stderr-ac6a-$$ 2>/dev/null || true)"
  rm -f /tmp/stderr-ac6a-$$

  # Exit 0: advisory warning, not a block
  [ "${status}" -eq 0 ]

  # STATE.md arm emits a warning at >200 lines
  [[ "${stderr_content}" == *"STATE.md SIZE WARNING"* ]]

  # Must NOT emit LESSONS.MD warning (wrong arm)
  [[ "${stderr_content}" != *"LESSONS.MD SIZE WARNING"* ]]
}

# ---------------------------------------------------------------------------
# AC-6b: 600-line STATE.md → exit 2 (exceeds 500-line block limit)
# Traces to BC-7.04.051 existing STATE.md arm (non-regression — block path)
# ---------------------------------------------------------------------------

@test "AC-6b NON-REGRESSION: 600-line STATE.md exits 2 (BLOCKED) after lessons.md arm refactor" {
  local state_path="${WORK}/.factory/STATE.md"

  # 600 lines: above the 500-line block threshold
  seq 600 > "${state_path}"

  # Do NOT commit: PRIOR_COUNT defaults to 0; LINE_COUNT (600) > PRIOR_COUNT (0) → check thresholds
  local payload
  payload="$(jq -nc --arg fp "${state_path}" '{tool_input: {file_path: $fp}}')"

  run bash -c "printf '%s' '${payload}' | bash '${HOOK}' 2>/tmp/stderr-ac6b-$$"
  local stderr_content
  stderr_content="$(cat /tmp/stderr-ac6b-$$ 2>/dev/null || true)"
  rm -f /tmp/stderr-ac6b-$$

  # Exit 2: hard block (STATE.md exceeds 500-line limit)
  [ "${status}" -eq 2 ]

  # Block message present
  [[ "${stderr_content}" == *"BLOCKED"* ]]

  # Block message references STATE.md (not lessons.md)
  [[ "${stderr_content}" == *"STATE.md"* ]]
}
