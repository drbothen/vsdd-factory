#!/usr/bin/env bats
# warn-lessons-exact-4000.bats — AC-9: exactly 4000 lines → exit 0 + warning (strict >4000 for block)
#
# Traces to: BC-7.04.051 extension: strict greater-than for hard threshold
# Canonical test vector: EC-007 — exactly 4000 lines (block boundary)
# D-442(e) hard threshold is STRICTLY greater-than 4000 (not ≥4000).
# At exactly 4000 lines: advisory warning only (>3500), NOT a block (not >4000).
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
# AC-9: exactly 4000 lines → exit 0 + warning (NOT a block; strict >4000)
# Traces to BC-7.04.051 extension: strict greater-than for hard threshold
# ---------------------------------------------------------------------------

@test "AC-9 WARN: exactly 4000-line lessons.md exits 0 with advisory warning (boundary: >4000 not >=4000 for block)" {
  local lessons_path="${WORK}/.factory/cycles/v1.0-test/lessons.md"
  _make_lessons_md 4000 "${lessons_path}"

  local payload
  payload="$(jq -nc --arg fp "${lessons_path}" '{tool_input: {file_path: $fp}}')"

  run bash -c "printf '%s' '${payload}' | bash '${HOOK}' 2>/tmp/stderr-ac9-$$"
  local stderr_content
  stderr_content="$(cat /tmp/stderr-ac9-$$ 2>/dev/null || true)"
  rm -f /tmp/stderr-ac9-$$

  # Exit 0: exactly 4000 is NOT above the hard threshold (strict >4000); advisory only
  [ "${status}" -eq 0 ]

  # Warning emitted (4000 > 3500 soft threshold, so advisory fires)
  [[ "${stderr_content}" == *"LESSONS.MD SIZE WARNING"* ]]

  # Must NOT be a block (the block threshold is >4000, not >=4000)
  [[ "${stderr_content}" != *"BLOCKED"* ]]
}
