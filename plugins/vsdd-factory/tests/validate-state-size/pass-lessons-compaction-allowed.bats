#!/usr/bin/env bats
# pass-lessons-compaction-allowed.bats — AC-4: compaction-in-progress → exit 0 unconditionally
#
# Traces to: BC-7.04.051 invariant (compaction-always-allowed); extension L2
# Canonical test vector: EC-004 — write reduces lessons.md from 6000 to 5000 lines
#
# When LINE_COUNT < PRIOR_COUNT (prior = git HEAD line count), the hook must exit 0
# unconditionally regardless of absolute line count (5000 > 4000 but it's REDUCING).
#
# Fixture: temp git repo with lessons.md committed at 6000 lines.
# Test: modify lessons.md to 5000 lines (still above hard threshold) then invoke hook.
# Expected: exit 0 (compaction exemption applies).
#
# Red gate: all tests skip pending implementation of the lessons.md arm.

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  HOOK="${REPO_ROOT}/plugins/vsdd-factory/hooks/validate-state-size.sh"
  WORK="$(mktemp -d)"
  # Build a minimal git repo that simulates .factory/cycles/*/lessons.md
  mkdir -p "${WORK}/.factory/cycles/v1.0-test"

  # Initialize git repo at WORK so git commands resolve correctly
  git -C "${WORK}" init -q
  git -C "${WORK}" config user.email "test@example.com"
  git -C "${WORK}" config user.name "Test"

  # Commit lessons.md at 6000 lines (the "prior" state)
  seq 6000 > "${WORK}/.factory/cycles/v1.0-test/lessons.md"
  git -C "${WORK}" add "${WORK}/.factory/cycles/v1.0-test/lessons.md"
  git -C "${WORK}" commit -q -m "prior: lessons.md at 6000 lines"
}

teardown() {
  rm -rf "${WORK}"
}

# ---------------------------------------------------------------------------
# AC-4: lessons.md write REDUCES line count (compaction) → exit 0 always
# Traces to BC-7.04.051 invariant (compaction-always-allowed); extension L2
# ---------------------------------------------------------------------------

@test "AC-4 PASS: compaction write (6000→5000 lines) exits 0 unconditionally despite exceeding hard threshold" {
  skip "pending lessons.md arm implementation (S-15.16-Part-B T-6)"

  local lessons_path="${WORK}/.factory/cycles/v1.0-test/lessons.md"

  # Write 5000 lines (above 4000 hard threshold, but REDUCING from 6000 committed)
  seq 5000 > "${lessons_path}"

  local payload
  payload="$(jq -nc --arg fp "${lessons_path}" '{tool_input: {file_path: $fp}}')"

  run bash -c "printf '%s' '${payload}' | bash '${HOOK}' 2>/tmp/stderr-ac4-$$"
  local stderr_content
  stderr_content="$(cat /tmp/stderr-ac4-$$ 2>/dev/null || true)"
  rm -f /tmp/stderr-ac4-$$

  # Exit 0: compaction always allowed
  [ "${status}" -eq 0 ]

  # Must NOT block (compaction exemption)
  [[ "${stderr_content}" != *"BLOCKED"* ]]
}
