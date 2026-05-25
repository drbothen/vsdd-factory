#!/usr/bin/env bats
# fail-lessons-hard-threshold.bats — AC-3 + AC-7: 4001-line lessons.md → exit 2 + block signal
#
# Traces to: BC-7.04.051 extension L4 (Hard block at >4000)
# Canonical test vector: EC-003 — 4001 lines (1 over hard threshold)
# D-442(e) hard threshold: >4000 lines → block_pre call + exit 2
#
# AC-3: exit 2 (block) when lessons.md has 4001 lines
# AC-7: block message contains "D-442(e)" citation and "compact-lessons" remediation hint
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
# AC-3: 4001-line lessons.md → exit 2
# Traces to BC-7.04.051 extension L4; D-442(e) hard=4000
# ---------------------------------------------------------------------------

@test "AC-3 BLOCK: 4001-line lessons.md exits 2 (hard block threshold exceeded)" {
  skip "pending lessons.md arm implementation (S-15.16-Part-B T-6)"

  local lessons_path="${WORK}/.factory/cycles/v1.0-test/lessons.md"
  _make_lessons_md 4001 "${lessons_path}"

  local payload
  payload="$(jq -nc --arg fp "${lessons_path}" '{tool_input: {file_path: $fp}}')"

  run bash -c "printf '%s' '${payload}' | bash '${HOOK}' 2>/tmp/stderr-ac3-$$"
  local stderr_content
  stderr_content="$(cat /tmp/stderr-ac3-$$ 2>/dev/null || true)"
  rm -f /tmp/stderr-ac3-$$

  # Exit 2: hard block
  [ "${status}" -eq 2 ]

  # Block signal must be present on stderr
  [[ "${stderr_content}" == *"BLOCKED"* ]]
}

# ---------------------------------------------------------------------------
# AC-7: block message contains D-442(e) citation and compact-lessons hint
# Traces to BC-7.04.051 extension L4 (block_pre message content)
# ---------------------------------------------------------------------------

@test "AC-7 BLOCK: 4001-line lessons.md block message contains D-442(e) and compact-lessons" {
  skip "pending lessons.md arm implementation (S-15.16-Part-B T-6)"

  local lessons_path="${WORK}/.factory/cycles/v1.0-test/lessons.md"
  _make_lessons_md 4001 "${lessons_path}"

  local payload
  payload="$(jq -nc --arg fp "${lessons_path}" '{tool_input: {file_path: $fp}}')"

  run bash -c "printf '%s' '${payload}' | bash '${HOOK}' 2>/tmp/stderr-ac7-$$"
  local stderr_content
  stderr_content="$(cat /tmp/stderr-ac7-$$ 2>/dev/null || true)"
  rm -f /tmp/stderr-ac7-$$

  # Exit 2: hard block
  [ "${status}" -eq 2 ]

  # Block message must cite D-442(e) so state-manager knows WHY
  [[ "${stderr_content}" == *"D-442(e)"* ]]

  # Block message must include compact-lessons remediation hint
  [[ "${stderr_content}" == *"compact-lessons"* ]]
}
