#!/usr/bin/env bats
# s-5.05-migration-guide.bats — RED-gate tests for S-5.05 migration guide ACs.
#
# These tests gate the 10 acceptance criteria for S-5.05 (operator-facing
# migration guide). All tests MUST FAIL in skeleton state (10 TODO(S-5.5)
# markers present, Status banner unchanged, README and v1.0-index.md
# unreformed). They MUST PASS when the implementer fills the guide.
#
# AC-9 is a manual human-review gate and is intentionally not tested here.

setup() {
  # Resolve repo root from test file location:
  # .../plugins/vsdd-factory/tests/docs-completeness/s-5.05-migration-guide.bats
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../../.." && pwd)"
  GUIDE="${REPO_ROOT}/docs/guide/migrating-from-0.79.md"
  README="${REPO_ROOT}/README.md"
  INDEX="${REPO_ROOT}/docs/guide/v1.0-index.md"
}

# Extract named section body (lines between heading and next ## heading).
_section() {
  local heading="$1"
  local file="$2"
  awk -v h="${heading}" '
    $0 == h { found=1; next }
    found && /^## / { exit }
    found { print }
  ' "${file}"
}

# Return true if a section has no TODO(S-5.5) markers anywhere in it.
_section_has_no_todo() {
  local heading="$1"
  local file="$2"
  ! _section "${heading}" "${file}" | grep -q "TODO(S-5\.5)"
}

# Return true if a section has non-empty non-comment lines (prose exists).
_section_has_prose() {
  local heading="$1"
  local file="$2"
  # Strip full HTML comment blocks (single-line and multi-line openers).
  local prose
  prose="$(_section "${heading}" "${file}" | grep -vE "^<!--.*-->$|^<!--$|^-->$|^[[:space:]]*$" || true)"
  [ -n "${prose}" ]
}

# ---------------------------------------------------------------------------
# AC-1: Zero TODO(S-5.5) markers remain in the migration guide.
# ---------------------------------------------------------------------------
@test "AC-1: migrating-from-0.79.md has zero TODO(S-5.5) markers" {
  count="$(grep -c "TODO(S-5\.5)" "${GUIDE}" || true)"
  [ "${count}" -eq 0 ]
}

# ---------------------------------------------------------------------------
# AC-2: "What changed" section is populated (no TODO comment in body).
# ---------------------------------------------------------------------------
@test "AC-2: 'What changed' section has no TODO markers and has prose" {
  _section_has_no_todo "## What changed" "${GUIDE}"
  _section_has_prose "## What changed" "${GUIDE}"
}

# ---------------------------------------------------------------------------
# AC-3: "Why v1.0" section is populated.
# ---------------------------------------------------------------------------
@test "AC-3: 'Why v1.0' section has no TODO markers and has prose" {
  _section_has_no_todo "## Why v1.0" "${GUIDE}"
  _section_has_prose "## Why v1.0" "${GUIDE}"
}

# ---------------------------------------------------------------------------
# AC-4: Prerequisites, Upgrade procedure, Verification checklist, Rollback
#        all populated; custom hooks.json migration referenced (EC-001).
# ---------------------------------------------------------------------------
@test "AC-4a: 'Prerequisites' section has no TODO markers and has prose" {
  _section_has_no_todo "## Prerequisites" "${GUIDE}"
  _section_has_prose "## Prerequisites" "${GUIDE}"
}

@test "AC-4b: 'Upgrade procedure' section has no TODO markers and has prose" {
  _section_has_no_todo "## Upgrade procedure" "${GUIDE}"
  _section_has_prose "## Upgrade procedure" "${GUIDE}"
}

@test "AC-4c: 'Verification checklist' section has no TODO markers and has prose" {
  _section_has_no_todo "## Verification checklist" "${GUIDE}"
  _section_has_prose "## Verification checklist" "${GUIDE}"
}

@test "AC-4d: 'Rollback' section has no TODO markers and has prose" {
  _section_has_no_todo "## Rollback" "${GUIDE}"
  _section_has_prose "## Rollback" "${GUIDE}"
}

@test "AC-4e: guide references custom hooks.json migration (EC-001 coverage)" {
  grep -qiE "hooks\.json" "${GUIDE}"
}

# ---------------------------------------------------------------------------
# AC-5: "Observability migration" populated; pre-filled Regenerating section
#        preserved.
# ---------------------------------------------------------------------------
@test "AC-5a: 'Observability migration' section has no TODO markers and has prose" {
  _section_has_no_todo "## Observability migration" "${GUIDE}"
  _section_has_prose "## Observability migration" "${GUIDE}"
}

@test "AC-5b: 'Regenerating hooks-registry.toml' pre-filled section is preserved" {
  grep -q "generate-registry-from-hooks-json.sh" "${GUIDE}"
}

# ---------------------------------------------------------------------------
# AC-6: "Windows-specific notes" section is populated.
# ---------------------------------------------------------------------------
@test "AC-6: 'Windows-specific notes' section has no TODO markers and has prose" {
  _section_has_no_todo "## Windows-specific notes" "${GUIDE}"
  _section_has_prose "## Windows-specific notes" "${GUIDE}"
}

# ---------------------------------------------------------------------------
# AC-7: "Troubleshooting" section has >= 5 distinct issues with resolutions.
#        Heuristic: count bullet items or sub-headings in that section >= 5.
# ---------------------------------------------------------------------------
@test "AC-7: 'Troubleshooting' section has at least 5 distinct issues" {
  section="$(_section "## Troubleshooting" "${GUIDE}")"
  count="$(echo "${section}" | grep -cE "^[-*]|^###" || true)"
  [ "${count}" -ge 5 ]
}

# ---------------------------------------------------------------------------
# AC-8: "Known regressions" populated; v1.0.0-beta.1 section preserved.
# ---------------------------------------------------------------------------
@test "AC-8a: 'Known regressions' section has no TODO markers and has prose" {
  _section_has_no_todo "## Known regressions" "${GUIDE}"
  _section_has_prose "## Known regressions" "${GUIDE}"
}

@test "AC-8b: 'Known regressions (v1.0.0-beta.1)' pre-filled section is preserved" {
  grep -q "Known regressions (v1.0.0-beta.1)" "${GUIDE}"
}

# AC-9: Manual human-review gate — not bats-testable. Skipped intentionally.

# ---------------------------------------------------------------------------
# AC-10: Status banner no longer says "skeleton — filled in by S-5.5".
# ---------------------------------------------------------------------------
@test "AC-10: Status banner no longer contains 'skeleton' text" {
  ! head -10 "${GUIDE}" | grep -q "skeleton"
}

# ---------------------------------------------------------------------------
# Task 16: README L264 description updated (no longer "finalized in S-5.5").
# ---------------------------------------------------------------------------
@test "Task-16: README no longer says 'skeleton, finalized in S-5.5'" {
  ! grep -q "skeleton, finalized in S-5.5" "${README}"
}

# ---------------------------------------------------------------------------
# Task 17: v1.0-index.md uses S-5.07 (not bare S-5.7).
# ---------------------------------------------------------------------------
@test "Task-17: v1.0-index.md contains no bare 'S-5.7' references" {
  ! grep -qE "S-5\.7\b" "${INDEX}"
}
