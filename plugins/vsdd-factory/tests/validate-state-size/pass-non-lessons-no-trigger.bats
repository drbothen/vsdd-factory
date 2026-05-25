#!/usr/bin/env bats
# pass-non-lessons-no-trigger.bats — AC-5: non-lessons.md path → exit 0 (no arm fires)
#
# Traces to: BC-7.04.051 extension L6 (Non-target file)
# Canonical test vector: EC-005 — file path is lessons-backup.md (not lessons.md)
#
# The lessons.md arm must only fire when FILE_PATH matches */.factory/cycles/*/lessons.md.
# A file named lessons-backup.md in the same directory must NOT trigger the arm.
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

# ---------------------------------------------------------------------------
# AC-5: lessons-backup.md path → exit 0 (path check rejects non-lessons.md)
# Traces to BC-7.04.051 extension L6
# ---------------------------------------------------------------------------

@test "AC-5 PASS: lessons-backup.md path does not trigger lessons.md arm (exit 0, no output)" {
  skip "pending lessons.md arm implementation (S-15.16-Part-B T-6)"

  # lessons-backup.md with >4000 lines — would block if lessons.md arm incorrectly matched it
  local backup_path="${WORK}/.factory/cycles/v1.0-test/lessons-backup.md"
  seq 4001 > "${backup_path}"

  local payload
  payload="$(jq -nc --arg fp "${backup_path}" '{tool_input: {file_path: $fp}}')"

  run bash -c "printf '%s' '${payload}' | bash '${HOOK}' 2>/tmp/stderr-ac5-$$"
  local stderr_content
  stderr_content="$(cat /tmp/stderr-ac5-$$ 2>/dev/null || true)"
  rm -f /tmp/stderr-ac5-$$

  # Exit 0: non-target path → immediate exit 0 (no arm fires)
  [ "${status}" -eq 0 ]

  # No block signal and no warning (arm did not fire at all)
  [[ "${stderr_content}" != *"BLOCKED"* ]]
  [[ "${stderr_content}" != *"LESSONS.MD SIZE WARNING"* ]]
}
