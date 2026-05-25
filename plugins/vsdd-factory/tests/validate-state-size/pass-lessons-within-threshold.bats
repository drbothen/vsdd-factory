#!/usr/bin/env bats
# pass-lessons-within-threshold.bats — AC-1: 925-line lessons.md → exit 0 (clean pass)
#
# Traces to: BC-7.04.051 extension L5 (Pass within threshold)
# Canonical test vector: EC-001 — 925 lines, well within ≤3500 range
#
# When the lessons.md arm is implemented (T-6 of S-15.16-Part-B), the script will
# actively match the lessons.md path, check line count (925 ≤ 3500), and exit 0
# with NO warning on stderr.
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
# AC-1: 925-line lessons.md → exit 0 (no warning, no block)
# Traces to BC-7.04.051 extension L5
# ---------------------------------------------------------------------------

@test "AC-1 PASS: 925-line lessons.md exits 0 with no stderr output (within threshold)" {
  skip "pending lessons.md arm implementation (S-15.16-Part-B T-6)"

  local lessons_path="${WORK}/.factory/cycles/v1.0-test/lessons.md"
  _make_lessons_md 925 "${lessons_path}"

  local payload
  payload="$(jq -nc --arg fp "${lessons_path}" '{tool_input: {file_path: $fp}}')"

  run bash -c "printf '%s' '${payload}' | bash '${HOOK}' 2>/tmp/stderr-ac1-$$"
  local stderr_content
  stderr_content="$(cat /tmp/stderr-ac1-$$ 2>/dev/null || true)"
  rm -f /tmp/stderr-ac1-$$

  # Exit 0: clean pass, no block
  [ "${status}" -eq 0 ]

  # No warning on stderr: script actively processed the file and found it within threshold
  [[ "${stderr_content}" != *"LESSONS.MD SIZE WARNING"* ]]
  [[ "${stderr_content}" != *"BLOCKED"* ]]
}
