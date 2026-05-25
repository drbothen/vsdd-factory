#!/usr/bin/env bats
# warn-lessons-soft-threshold.bats — AC-2: 3501-line lessons.md → exit 0 + stderr warning
#
# Traces to: BC-7.04.051 extension L3 (Soft advisory at >3500)
# Canonical test vector: EC-002 — 3501 lines (1 over soft threshold)
# D-442(e) soft threshold: >3500 lines → stderr warning + exit 0 (NOT a block)
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
# AC-2: 3501-line lessons.md → exit 0 + "LESSONS.MD SIZE WARNING" on stderr
# Traces to BC-7.04.051 extension L3; D-442(e) soft=3500
# ---------------------------------------------------------------------------

@test "AC-2 WARN: 3501-line lessons.md exits 0 with LESSONS.MD SIZE WARNING on stderr" {
  skip "pending lessons.md arm implementation (S-15.16-Part-B T-6)"

  local lessons_path="${WORK}/.factory/cycles/v1.0-test/lessons.md"
  _make_lessons_md 3501 "${lessons_path}"

  local payload
  payload="$(jq -nc --arg fp "${lessons_path}" '{tool_input: {file_path: $fp}}')"

  run bash -c "printf '%s' '${payload}' | bash '${HOOK}' 2>/tmp/stderr-ac2-$$"
  local stderr_content
  stderr_content="$(cat /tmp/stderr-ac2-$$ 2>/dev/null || true)"
  rm -f /tmp/stderr-ac2-$$

  # Exit 0: advisory only, not a block
  [ "${status}" -eq 0 ]

  # stderr must contain the advisory marker
  [[ "${stderr_content}" == *"LESSONS.MD SIZE WARNING"* ]]

  # Must NOT be a block signal
  [[ "${stderr_content}" != *"BLOCKED"* ]]
}
