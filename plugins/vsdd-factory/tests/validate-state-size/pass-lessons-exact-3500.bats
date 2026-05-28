#!/usr/bin/env bats
# pass-lessons-exact-3500.bats — AC-8: exactly 3500 lines → exit 0 (strict >3500 boundary)
#
# Traces to: BC-7.04.051 extension: strict greater-than for soft threshold
# Canonical test vector: EC-006 — exactly 3500 lines (boundary)
# D-442(e) soft threshold is STRICTLY greater-than 3500 (not ≥3500).
# At exactly 3500 lines, the hook must exit 0 with NO warning.
#
# Red gate: all tests skip pending implementation of the lessons.md arm.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  HOOK="${REPO_ROOT}/plugins/vsdd-factory/hooks/validate-state-size.sh"
  WORK="$(mktemp -d)"
  mkdir -p "${WORK}/.factory/cycles/v1.0-test"
}

teardown() {
  rm -rf "${WORK}"
}

_make_lessons_md() {
  local line_count="$1"
  local path="$2"
  seq "${line_count}" > "${path}"
}

# ---------------------------------------------------------------------------
# AC-8: exactly 3500 lines → exit 0, no warning (strict > not >=)
# Traces to BC-7.04.051 extension: strict greater-than for soft threshold
# ---------------------------------------------------------------------------

@test "AC-8 PASS: exactly 3500-line lessons.md exits 0 with no warning (boundary: >3500 not >=3500)" {
  local lessons_path="${WORK}/.factory/cycles/v1.0-test/lessons.md"
  _make_lessons_md 3500 "${lessons_path}"

  local payload
  payload="$(jq -nc --arg fp "${lessons_path}" '{tool_input: {file_path: $fp}}')"

  run bash -c "printf '%s' '${payload}' | bash '${HOOK}' 2>/tmp/stderr-ac8-$$"
  local stderr_content
  stderr_content="$(cat /tmp/stderr-ac8-$$ 2>/dev/null || true)"
  rm -f /tmp/stderr-ac8-$$

  # Exit 0: exactly 3500 is NOT above the soft threshold (strict >3500)
  [ "${status}" -eq 0 ]

  # No warning emitted (exactly at boundary, not over it)
  [[ "${stderr_content}" != *"LESSONS.MD SIZE WARNING"* ]]

  # No block signal
  [[ "${stderr_content}" != *"BLOCKED"* ]]
}
