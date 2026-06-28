#!/usr/bin/env bats
# check-state-health-autocompact.bats — GREEN suite for S-18.10 (17 tests).
#
# Tests the helper (jq-based rewrite at commit 65284066):
#   plugins/vsdd-factory/skills/check-state-health/lib/check-autocompact-setting.sh
#
# The helper reads .claude/settings.json (project-local) then
# ~/.claude/settings.json (global fallback), parses env.CLAUDE_AUTOCOMPACT_PCT_OVERRIDE
# via jq, and emits:
#   PASS     — key present, numeric value in [1, 80]
#   ADVISORY — key absent, value > 80, value ≤ 0, non-numeric (treated as absent),
#              missing settings.json, malformed/unreadable settings.json, or jq absent
# Advisory-only; never blocks; no side effects; set -euo pipefail.
#
# 17 tests map to AC-001..AC-007 + EC-001..EC-012 + jq-absent explicit coverage
# (BC-6.25.001 v1.1 PC1–PC5, INV1–INV5; VP-092):
#
#   test_autocompact_check_absent_key_emits_advisory                   — AC-001 / BC-6.25.001 PC1
#   test_autocompact_check_value_70_is_pass                            — AC-003 / BC-6.25.001 PC3
#   test_autocompact_check_value_85_is_advisory                        — AC-002 / BC-6.25.001 PC2
#   test_autocompact_check_value_80_is_pass_boundary                   — AC-003 / BC-6.25.001 PC3 (boundary)
#   test_autocompact_check_no_settings_json_emits_advisory             — AC-001 + AC-004 / PC1 + PC4(c)
#   test_autocompact_check_project_local_takes_precedence_over_global  — AC-004 / PC4 + INV2
#   test_autocompact_check_non_numeric_value_treated_as_absent         — AC-006 / INV3
#   test_autocompact_check_global_fallback_pass_when_local_absent      — EC-005 / PC4(b)
#   test_autocompact_check_empty_string_value_emits_advisory           — EC-008 / INV3
#   test_autocompact_check_unreadable_settings_json_emits_advisory     — EC-011 / INV1 + INV5 (unreadable)
#   test_autocompact_check_single_line_json_value_70_is_pass           — F-P2-001 / BC-6.25.001 PC3 (canonical TV)
#   test_autocompact_check_single_line_json_value_85_is_advisory       — F-P2-001 / BC-6.25.001 PC2 (canonical TV)
#   test_autocompact_check_single_line_json_value_80_is_pass_boundary  — F-P2-001 / BC-6.25.001 PC3 boundary (canonical TV)
#   test_autocompact_check_real_malformed_json_emits_ec011_advisory    — F-P2-002 / EC-011 (syntactically malformed JSON)
#   test_autocompact_check_ec012_value_zero_is_advisory                — EC-012 / BC-6.25.001 INV3 lower-bound
#   test_autocompact_check_ec012_value_negative_is_advisory            — EC-012 / BC-6.25.001 INV3 lower-bound
#   test_autocompact_check_jq_absent_degrades_gracefully               — F-P1-001 class / BC-6.25.001 INV1+INV5
#
# All 17 tests must pass (GREEN).
#
# Edge Cases exercised per BC-6.25.001 v1.1 EC→test mapping (NO "implicitly exercised" claims):
#   EC-001 (value "70", PASS; multi-line fixture)
#                                     → test_autocompact_check_value_70_is_pass
#   EC-001 (value "70", PASS; SINGLE-LINE canonical TV form — F-P2-001 regression guard)
#                                     → test_autocompact_check_single_line_json_value_70_is_pass
#   EC-002 (env block present, key absent)
#                                     → test_autocompact_check_absent_key_emits_advisory
#   EC-003 (value "85", ADVISORY; multi-line fixture)
#                                     → test_autocompact_check_value_85_is_advisory
#   EC-003 (value "85", ADVISORY; SINGLE-LINE canonical TV form — F-P2-001 regression guard)
#                                     → test_autocompact_check_single_line_json_value_85_is_advisory
#   EC-004 (value "80", PASS boundary; multi-line fixture)
#                                     → test_autocompact_check_value_80_is_pass_boundary
#   EC-004 (value "80", PASS boundary; SINGLE-LINE canonical TV form — F-P2-001 regression guard)
#                                     → test_autocompact_check_single_line_json_value_80_is_pass_boundary
#   EC-005 (global fallback value "70", project-local absent)
#                                     → test_autocompact_check_global_fallback_pass_when_local_absent
#   EC-006 (both settings.json absent)
#                                     → test_autocompact_check_no_settings_json_emits_advisory
#   EC-007 (env block absent entirely)
#                                     → test_autocompact_check_absent_key_emits_advisory
#   EC-008 (empty string value "")
#                                     → test_autocompact_check_empty_string_value_emits_advisory
#   EC-009 (value "auto", non-integer)
#                                     → test_autocompact_check_non_numeric_value_treated_as_absent
#   EC-010 (project-local 85, global 70; project-local wins)
#                                     → test_autocompact_check_project_local_takes_precedence_over_global
#   EC-011 (UNREADABLE settings.json — chmod 000; parse-error ADVISORY path)
#                                     → test_autocompact_check_unreadable_settings_json_emits_advisory
#   EC-011 (READABLE but SYNTACTICALLY MALFORMED JSON — truncated JSON; parse-error ADVISORY path)
#                                     → test_autocompact_check_real_malformed_json_emits_ec011_advisory
#   EC-012 (value "0", zero — out-of-range lower-bound ADVISORY)
#                                     → test_autocompact_check_ec012_value_zero_is_advisory
#   EC-012 (value "-5", negative — out-of-range lower-bound ADVISORY)
#                                     → test_autocompact_check_ec012_value_negative_is_advisory
#   jq absent (F-P1-001-class invariant — INV1/INV5 under missing-parser)
#                                     → test_autocompact_check_jq_absent_degrades_gracefully
#
# Story:   S-18.10
# BC:      BC-6.25.001 v1.1 — all 5 PCs, INV1–INV5, EC-001..EC-012
# VP:      VP-092
# ADR:     ADR-026 §Decision 5 (value ceiling 80; canonical value 70)
#          ADR-026 §F-11 (check-state-health must verify this key)
# File:    plugins/vsdd-factory/tests/check-state-health-autocompact.bats
#
# Run:
#   bats plugins/vsdd-factory/tests/check-state-health-autocompact.bats

# ---------------------------------------------------------------------------
# setup / teardown — isolated temp dirs; HOME stubbed for global-fallback tests
# ---------------------------------------------------------------------------

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  HELPER="$REPO_ROOT/plugins/vsdd-factory/skills/check-state-health/lib/check-autocompact-setting.sh"

  # Each test gets a fresh temp workdir for fixtures.
  # WORK acts as the project root (the dir that would contain .factory/).
  WORK="$(mktemp -d)"

  # FAKE_HOME is used when we need to stub ~/.claude/settings.json so the real
  # global settings.json is never touched or consulted.
  FAKE_HOME="$(mktemp -d)"

  # Create the .claude subdirs so mkdir -p isn't needed in each test.
  mkdir -p "$WORK/.claude"
  mkdir -p "$FAKE_HOME/.claude"
}

teardown() {
  rm -rf "$WORK"
  rm -rf "$FAKE_HOME"
}

# ---------------------------------------------------------------------------
# Preflight helper
# ---------------------------------------------------------------------------

# Verify the helper script exists and is executable.
# Used by all 17 tests; fails with a descriptive message if the helper is absent.
_require_helper() {
  if [ ! -f "$HELPER" ]; then
    echo "RED GATE — helper not yet implemented."
    echo "Implementer: create $HELPER (S-18.10 T-3)."
    echo "Helper must: read project-local .claude/settings.json (then global fallback);"
    echo "  parse env.CLAUDE_AUTOCOMPACT_PCT_OVERRIDE; emit PASS (value ≤ 80) or ADVISORY."
    echo "set -euo pipefail required; advisory-only, never blocks (BC-6.25.001 INV1)."
    return 1
  fi
  if [ ! -x "$HELPER" ]; then
    echo "RED GATE — helper exists but is not executable: $HELPER"
    echo "Implementer: chmod +x $HELPER"
    return 1
  fi
}

# ---------------------------------------------------------------------------
# T-1 / AC-001 / BC-6.25.001 PC1 — key absent emits ADVISORY
#
# When CLAUDE_AUTOCOMPACT_PCT_OVERRIDE is absent from the env block of the
# resolved settings.json (or env block is present but key missing), the helper
# MUST emit a row containing "ADVISORY" and "CLAUDE_AUTOCOMPACT_PCT_OVERRIDE"
# and a remediation hint referencing ADR-026 §Decision 5.
#
# The row MUST NOT contain "PASS".
# The helper MUST exit 0 (advisory, never blocking; BC-6.25.001 INV1 + PC5).
#
# Edge cases exercised: EC-002 (env block present; key absent), EC-007 (env block absent).
# This test uses EC-007 (env block absent entirely — the simpler absent-key case).
# ---------------------------------------------------------------------------

@test "test_autocompact_check_absent_key_emits_advisory" {
  _require_helper

  # Fixture: project-local settings.json with env block but key absent (EC-007).
  cat > "$WORK/.claude/settings.json" <<'JSON'
{
  "env": {}
}
JSON

  # Run the helper with PROJECT_ROOT pointing to the fixture workdir.
  # HOME is stubbed to FAKE_HOME (no global settings.json there) so the
  # global fallback path is also absent — only project-local is consulted.
  run env HOME="$FAKE_HOME" PROJECT_ROOT="$WORK" bash "$HELPER" 2>&1

  # AC-001 / PC1: helper exits 0 (advisory; non-blocking; INV1 + PC5).
  [ "$status" -eq 0 ] || {
    echo "FAIL (AC-001): expected exit 0 (advisory non-blocking) but got status=$status"
    echo "BC-6.25.001 INV1: the check MUST NOT emit exit 2 or block any operation."
    echo "BC-6.25.001 PC5: advisory-only; no non-zero exit from this check."
    echo "Output: $output"
    return 1
  }

  # AC-001: check name must appear in output.
  [[ "$output" == *"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE"* ]] || {
    echo "FAIL (AC-001): output does not contain check name 'CLAUDE_AUTOCOMPACT_PCT_OVERRIDE'."
    echo "BC-6.25.001 PC1: check name must be present in the emitted row."
    echo "AC-007 / INV5: row MUST always be emitted."
    echo "Output: $output"
    return 1
  }

  # AC-001 / PC1: status word must be ADVISORY (not PASS).
  [[ "$output" == *"ADVISORY"* ]] || {
    echo "FAIL (AC-001): output does not contain 'ADVISORY' when key is absent."
    echo "BC-6.25.001 PC1: absent key → ADVISORY row."
    echo "Output: $output"
    return 1
  }

  # AC-001: PASS must NOT appear when key is absent.
  [[ "$output" != *"PASS"* ]] || {
    echo "FAIL (AC-001): output contains 'PASS' when key is absent — incorrect."
    echo "BC-6.25.001 PC1: absent key → ADVISORY, not PASS."
    echo "Output: $output"
    return 1
  }

  # AC-001: remediation hint must reference ADR-026 §Decision 5.
  [[ "$output" == *"ADR-026"* ]] || {
    echo "FAIL (AC-001): advisory output does not reference 'ADR-026'."
    echo "AC-001 specifies: details must mention ADR-026 §Decision 5."
    echo "Output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-2 / AC-003 / BC-6.25.001 PC3 — value 70 emits PASS (canonical value)
#
# When CLAUDE_AUTOCOMPACT_PCT_OVERRIDE is present and its numeric value is 70
# (the canonical value per ADR-026 §Decision 5), the helper MUST emit a PASS row.
# The row MUST contain the check name and "PASS". Exit 0.
#
# Edge case exercised: EC-001 (value "70").
# ---------------------------------------------------------------------------

@test "test_autocompact_check_value_70_is_pass" {
  _require_helper

  # Fixture: project-local settings.json with canonical value 70 (EC-001).
  cat > "$WORK/.claude/settings.json" <<'JSON'
{
  "env": {
    "CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "70"
  }
}
JSON

  run env HOME="$FAKE_HOME" PROJECT_ROOT="$WORK" bash "$HELPER" 2>&1

  # PC3: helper exits 0.
  [ "$status" -eq 0 ] || {
    echo "FAIL (AC-003): expected exit 0 (PASS) but got status=$status"
    echo "BC-6.25.001 PC3: value ≤ 80 → PASS row; exit 0."
    echo "Output: $output"
    return 1
  }

  # AC-003 / PC3: check name present.
  [[ "$output" == *"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE"* ]] || {
    echo "FAIL (AC-003): check name 'CLAUDE_AUTOCOMPACT_PCT_OVERRIDE' missing from output."
    echo "AC-007 / INV5: row MUST always be emitted."
    echo "Output: $output"
    return 1
  }

  # AC-003 / PC3: status word must be PASS.
  [[ "$output" == *"PASS"* ]] || {
    echo "FAIL (AC-003): output does not contain 'PASS' for value 70."
    echo "BC-6.25.001 PC3: value 70 ≤ 80 → PASS row."
    echo "ADR-026 §Decision 5: 70 is the canonical value."
    echo "Output: $output"
    return 1
  }

  # AC-003: ADVISORY must NOT appear when value is 70.
  [[ "$output" != *"ADVISORY"* ]] || {
    echo "FAIL (AC-003): output contains 'ADVISORY' for value 70 — incorrect."
    echo "BC-6.25.001 PC3: value ≤ 80 → PASS, not ADVISORY."
    echo "Output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-3 / AC-002 / BC-6.25.001 PC2 — value 85 emits ADVISORY (exceeds ceiling 80)
#
# When CLAUDE_AUTOCOMPACT_PCT_OVERRIDE is present but its numeric value exceeds 80,
# the helper MUST emit an ADVISORY row. The advisory MUST contain the actual value
# (85), reference the ceiling of 80, and reference ADR-026 §Decision 5.
# Exit 0 (non-blocking per INV1).
#
# Edge case exercised: EC-003 (value "85", > 80).
# ---------------------------------------------------------------------------

@test "test_autocompact_check_value_85_is_advisory" {
  _require_helper

  # Fixture: project-local settings.json with value 85 (EC-003).
  cat > "$WORK/.claude/settings.json" <<'JSON'
{
  "env": {
    "CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "85"
  }
}
JSON

  run env HOME="$FAKE_HOME" PROJECT_ROOT="$WORK" bash "$HELPER" 2>&1

  # PC2: helper exits 0 (advisory non-blocking; INV1).
  [ "$status" -eq 0 ] || {
    echo "FAIL (AC-002): expected exit 0 (advisory non-blocking) but got status=$status"
    echo "BC-6.25.001 INV1: advisory check must never block; exit 0 regardless of value."
    echo "Output: $output"
    return 1
  }

  # AC-002 / PC2: check name present.
  [[ "$output" == *"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE"* ]] || {
    echo "FAIL (AC-002): check name missing. AC-007 / INV5: row always emitted."
    echo "Output: $output"
    return 1
  }

  # AC-002 / PC2: status word must be ADVISORY.
  [[ "$output" == *"ADVISORY"* ]] || {
    echo "FAIL (AC-002): output does not contain 'ADVISORY' for value 85 (> 80 ceiling)."
    echo "BC-6.25.001 PC2: value > 80 → ADVISORY row."
    echo "ADR-026 §Decision 5: ceiling is 80 (MEDIUM-confidence 83% harness cap)."
    echo "Output: $output"
    return 1
  }

  # AC-002: advisory details must contain the actual value.
  [[ "$output" == *"85"* ]] || {
    echo "FAIL (AC-002): advisory output does not include the actual value '85'."
    echo "AC-002 specifies: details must contain the actual configured value <N>."
    echo "Output: $output"
    return 1
  }

  # AC-002: PASS must NOT appear when value exceeds ceiling.
  [[ "$output" != *"PASS"* ]] || {
    echo "FAIL (AC-002): output contains 'PASS' for value 85 — incorrect."
    echo "BC-6.25.001 PC2: value > 80 → ADVISORY, not PASS."
    echo "Output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-4 / AC-003 / BC-6.25.001 PC3 (boundary) — value 80 emits PASS (80 ≤ 80)
#
# The ceiling comparison is inclusive: value 80 satisfies value ≤ 80 and therefore
# MUST emit a PASS row, not ADVISORY. This boundary test guards against
# off-by-one errors in the numeric comparison.
#
# Edge case exercised: EC-004 (value "80"; boundary inclusive).
# ---------------------------------------------------------------------------

@test "test_autocompact_check_value_80_is_pass_boundary" {
  _require_helper

  # Fixture: project-local settings.json with boundary value 80 (EC-004).
  cat > "$WORK/.claude/settings.json" <<'JSON'
{
  "env": {
    "CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "80"
  }
}
JSON

  run env HOME="$FAKE_HOME" PROJECT_ROOT="$WORK" bash "$HELPER" 2>&1

  # PC3 / boundary: exit 0.
  [ "$status" -eq 0 ] || {
    echo "FAIL (AC-003 boundary): expected exit 0 for value 80 but got status=$status"
    echo "BC-6.25.001 PC3: value ≤ 80 → PASS (boundary inclusive: 80 ≤ 80 is true)."
    echo "Output: $output"
    return 1
  }

  # AC-003 / PC3 / boundary: PASS must be emitted.
  [[ "$output" == *"PASS"* ]] || {
    echo "FAIL (AC-003 boundary): output does not contain 'PASS' for value 80."
    echo "BC-6.25.001 PC3: 80 ≤ 80 is true → PASS row (ceiling is inclusive)."
    echo "EC-004: value '80' is the boundary; it must not trigger ADVISORY."
    echo "Output: $output"
    return 1
  }

  # AC-003 / boundary: ADVISORY must NOT appear.
  [[ "$output" != *"ADVISORY"* ]] || {
    echo "FAIL (AC-003 boundary): output contains 'ADVISORY' for value 80 — incorrect."
    echo "BC-6.25.001 PC3: ceiling comparison is ≤ 80; value 80 is PASS, not ADVISORY."
    echo "EC-004 boundary-inclusive: only values > 80 trigger ADVISORY."
    echo "Output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-5 / AC-001 + AC-004 / BC-6.25.001 PC1 + PC4(c) — no settings.json → ADVISORY
#
# When NEITHER project-local (.claude/settings.json) NOR global (~/.claude/settings.json)
# exists, the helper treats the key as absent (AC-001 ADVISORY fires) and MUST
# additionally note that no settings.json was found at either path (AC-004 PC4(c)).
# Exit 0 (non-blocking; INV1).
#
# Edge cases exercised: EC-006 (both files absent).
# ---------------------------------------------------------------------------

@test "test_autocompact_check_no_settings_json_emits_advisory" {
  _require_helper

  # Both project-local and global settings.json are absent.
  # WORK/.claude/ exists but contains no settings.json (created in setup with mkdir -p).
  # FAKE_HOME/.claude/ exists but contains no settings.json (created in setup with mkdir -p).
  # Neither file is present — EC-006.

  run env HOME="$FAKE_HOME" PROJECT_ROOT="$WORK" bash "$HELPER" 2>&1

  # PC1 / PC4(c): helper exits 0 (advisory non-blocking).
  [ "$status" -eq 0 ] || {
    echo "FAIL (AC-001+AC-004): expected exit 0 but got status=$status"
    echo "BC-6.25.001 INV1: check must never block; exit 0 even when no settings.json found."
    echo "Output: $output"
    return 1
  }

  # AC-001: check name present.
  [[ "$output" == *"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE"* ]] || {
    echo "FAIL (AC-001+AC-004): check name missing. AC-007 / INV5: row always emitted."
    echo "Output: $output"
    return 1
  }

  # AC-001 / PC1: ADVISORY must appear.
  [[ "$output" == *"ADVISORY"* ]] || {
    echo "FAIL (AC-001+AC-004): output does not contain 'ADVISORY' when no settings.json found."
    echo "BC-6.25.001 PC1 + PC4(c): key absent (no settings.json at either path) → ADVISORY."
    echo "Output: $output"
    return 1
  }

  # AC-004 / PC4(c): advisory must note that no settings.json was found.
  # The note is required ("no settings.json found at .claude/settings.json or ~/.claude/settings.json").
  # We check for the informational substring "no settings.json" or "settings.json" to be lenient
  # on exact phrasing while enforcing the required note per AC-004.
  [[ "$output" == *"settings.json"* ]] || {
    echo "FAIL (AC-004): advisory output does not mention 'settings.json' path context."
    echo "AC-004 / PC4(c): when neither settings.json exists, advisory must note"
    echo "  'no settings.json found at .claude/settings.json or ~/.claude/settings.json'."
    echo "Output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-6 / AC-004 / BC-6.25.001 PC4 + INV2 — project-local takes precedence over global
#
# When project-local .claude/settings.json (value 85) and global ~/.claude/settings.json
# (value 70) BOTH exist, the helper MUST use the project-local file exclusively.
# The expected result is ADVISORY (value 85 > 80) from the project-local file.
# If the global file were consulted, the result would be PASS (value 70 ≤ 80) —
# so an erroneous PASS proves the precedence rule was violated.
#
# Edge case exercised: EC-010 (project-local 85, global 70 — project-local wins).
# ---------------------------------------------------------------------------

@test "test_autocompact_check_project_local_takes_precedence_over_global" {
  _require_helper

  # Fixture: project-local settings.json with value 85 (would trigger ADVISORY).
  cat > "$WORK/.claude/settings.json" <<'JSON'
{
  "env": {
    "CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "85"
  }
}
JSON

  # Fixture: global settings.json with value 70 (would trigger PASS if consulted).
  # Placed in FAKE_HOME to avoid touching the real global settings.json.
  cat > "$FAKE_HOME/.claude/settings.json" <<'JSON'
{
  "env": {
    "CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "70"
  }
}
JSON

  run env HOME="$FAKE_HOME" PROJECT_ROOT="$WORK" bash "$HELPER" 2>&1

  # PC4 / INV2: exit 0.
  [ "$status" -eq 0 ] || {
    echo "FAIL (AC-004): expected exit 0 but got status=$status"
    echo "BC-6.25.001 INV1: advisory check never blocks."
    echo "Output: $output"
    return 1
  }

  # AC-004 / INV2: result must be ADVISORY (from project-local value 85).
  # If PASS appears, the helper incorrectly consulted the global file (value 70).
  [[ "$output" == *"ADVISORY"* ]] || {
    echo "FAIL (AC-004): expected ADVISORY (project-local value 85 > 80) but output contains no ADVISORY."
    echo "BC-6.25.001 PC4 + INV2: project-local always takes precedence over global."
    echo "EC-010: project-local value 85 must produce ADVISORY (not PASS from global 70)."
    echo "If PASS appears, the helper incorrectly fell back to the global file."
    echo "Output: $output"
    return 1
  }

  # Reinforce: PASS must NOT appear (it would indicate global fallback was used).
  [[ "$output" != *"PASS"* ]] || {
    echo "FAIL (AC-004 / INV2): output contains 'PASS' — helper used global file instead of project-local."
    echo "BC-6.25.001 PC4: project-local .claude/settings.json ALWAYS takes precedence."
    echo "BC-6.25.001 INV2: global file is NEVER consulted when project-local file exists."
    echo "EC-010: both files present → project-local (85, ADVISORY) wins over global (70, PASS)."
    echo "Output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-7 / AC-006 / BC-6.25.001 INV3 — non-numeric value treated as absent
#
# Values that cannot be parsed as decimal integers (empty string "", "auto", etc.)
# MUST be treated as absent (AC-001 advisory fires) with a note in the advisory:
# "Value '<raw>' is not a valid integer; treating as absent" (BC-6.25.001 INV3).
# Exit 0 (INV1).
#
# Edge cases exercised: EC-008 (empty string ""), EC-009 (value "auto").
# This test uses EC-009 ("auto") as the representative non-numeric case.
# ---------------------------------------------------------------------------

@test "test_autocompact_check_non_numeric_value_treated_as_absent" {
  _require_helper

  # Fixture: project-local settings.json with non-numeric value "auto" (EC-009).
  cat > "$WORK/.claude/settings.json" <<'JSON'
{
  "env": {
    "CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "auto"
  }
}
JSON

  run env HOME="$FAKE_HOME" PROJECT_ROOT="$WORK" bash "$HELPER" 2>&1

  # INV3 / PC1: exit 0 (advisory; non-blocking).
  [ "$status" -eq 0 ] || {
    echo "FAIL (AC-006): expected exit 0 but got status=$status"
    echo "BC-6.25.001 INV1: check must never block; exit 0 for non-numeric values."
    echo "BC-6.25.001 INV3: non-numeric value treated as absent → ADVISORY, exit 0."
    echo "Output: $output"
    return 1
  }

  # AC-006 / INV3: check name present.
  [[ "$output" == *"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE"* ]] || {
    echo "FAIL (AC-006): check name missing. AC-007 / INV5: row always emitted."
    echo "Output: $output"
    return 1
  }

  # AC-006 / INV3: ADVISORY must appear (non-numeric treated as absent → PC1 fires).
  [[ "$output" == *"ADVISORY"* ]] || {
    echo "FAIL (AC-006): output does not contain 'ADVISORY' for non-numeric value 'auto'."
    echo "BC-6.25.001 INV3: non-numeric value is treated as absent → ADVISORY (PC1)."
    echo "EC-009: value 'auto' is not a valid integer; must trigger ADVISORY."
    echo "Output: $output"
    return 1
  }

  # AC-006 / INV3: advisory must note that the raw value is not a valid integer.
  # The raw value "auto" must appear so the operator knows what was configured.
  [[ "$output" == *"auto"* ]] || {
    echo "FAIL (AC-006): advisory output does not include the raw value 'auto'."
    echo "BC-6.25.001 INV3 / AC-006: advisory note must include the raw value."
    echo "Required note form: \"Value 'auto' is not a valid integer; treating as absent\"."
    echo "Output: $output"
    return 1
  }

  # AC-006: PASS must NOT appear for non-numeric values.
  [[ "$output" != *"PASS"* ]] || {
    echo "FAIL (AC-006): output contains 'PASS' for non-numeric value 'auto' — incorrect."
    echo "BC-6.25.001 INV3: non-numeric → treated as absent → ADVISORY, never PASS."
    echo "Output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-8 / EC-005 / BC-6.25.001 PC4(b) — global fallback consulted and returns PASS
#
# When project-local .claude/settings.json is ABSENT but global ~/.claude/settings.json
# IS present with value 70, the helper MUST fall back to the global file, parse it, and
# emit PASS (value 70 ≤ 80). This test was previously "implicitly covered" by T-5 but
# never actually exercised a global file that produces a PASS — T-5 tests both-absent (EC-006).
#
# Explicit verification that: (a) global file IS read when local is absent, and (b) the
# content of the global file correctly produces PASS.
#
# Edge case exercised: EC-005 (global fallback with value 70 → PASS).
# ---------------------------------------------------------------------------

@test "test_autocompact_check_global_fallback_pass_when_local_absent" {
  _require_helper

  # Fixture: project-local .claude/settings.json is ABSENT (dir exists; no file).
  # WORK/.claude/ was created in setup with mkdir -p but contains no settings.json.
  [ ! -f "$WORK/.claude/settings.json" ] || {
    echo "TEST SETUP ERROR: expected no project-local settings.json but one exists."
    return 1
  }

  # Fixture: global settings.json in FAKE_HOME (stubbed HOME) with canonical value 70 (EC-005).
  cat > "$FAKE_HOME/.claude/settings.json" <<'JSON'
{
  "env": {
    "CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "70"
  }
}
JSON

  run env HOME="$FAKE_HOME" PROJECT_ROOT="$WORK" bash "$HELPER" 2>&1

  # INV1: helper exits 0 (advisory-only, never blocking).
  [ "$status" -eq 0 ] || {
    echo "FAIL (EC-005): expected exit 0 but got status=$status"
    echo "BC-6.25.001 INV1: check must never block; exit 0 regardless of which file is used."
    echo "Output: $output"
    return 1
  }

  # INV5: check name must appear (row always emitted).
  [[ "$output" == *"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE"* ]] || {
    echo "FAIL (EC-005): check name 'CLAUDE_AUTOCOMPACT_PCT_OVERRIDE' missing from output."
    echo "AC-007 / INV5: row MUST always be emitted."
    echo "Output: $output"
    return 1
  }

  # EC-005 / PC4(b): PASS must be emitted because the global file was consulted and value 70 ≤ 80.
  # If ADVISORY appears instead, the helper failed to read the global file or misclassified value 70.
  [[ "$output" == *"PASS"* ]] || {
    echo "FAIL (EC-005): output does not contain 'PASS' when global fallback has value 70."
    echo "BC-6.25.001 PC4(b): when project-local is absent, helper MUST consult global file."
    echo "EC-005: global file present with value 70 → PASS (70 ≤ 80)."
    echo "Output: $output"
    return 1
  }

  # EC-005: ADVISORY must NOT appear (global file has value 70, which is PASS).
  [[ "$output" != *"ADVISORY"* ]] || {
    echo "FAIL (EC-005): output contains 'ADVISORY' — global fallback not correctly read."
    echo "EC-005: global value 70 satisfies the ≤ 80 ceiling → PASS, not ADVISORY."
    echo "Output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-9 / EC-008 / BC-6.25.001 INV3 — empty string value emits ADVISORY (distinct from "auto")
#
# An empty string ("") is a distinct non-numeric case from "auto" (EC-009).
# T-7 uses "auto" as its representative; this test uses "" explicitly so EC-008
# is directly fixtured and exercised rather than inferred.
#
# The helper MUST treat "" as non-numeric (INV3), emit ADVISORY, include the raw
# value (empty string) in the advisory note, and exit 0 (INV1).
#
# Edge case exercised: EC-008 (empty string value "").
# ---------------------------------------------------------------------------

@test "test_autocompact_check_empty_string_value_emits_advisory" {
  _require_helper

  # Fixture: project-local settings.json with explicitly empty string value (EC-008).
  cat > "$WORK/.claude/settings.json" <<'JSON'
{
  "env": {
    "CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": ""
  }
}
JSON

  run env HOME="$FAKE_HOME" PROJECT_ROOT="$WORK" bash "$HELPER" 2>&1

  # INV1 / PC5: exit 0 (advisory; non-blocking).
  [ "$status" -eq 0 ] || {
    echo "FAIL (EC-008): expected exit 0 but got status=$status"
    echo "BC-6.25.001 INV1: check must never block; exit 0 for empty-string values."
    echo "BC-6.25.001 INV3: empty string is not a valid integer; treated as absent → exit 0."
    echo "Output: $output"
    return 1
  }

  # INV5: check name present.
  [[ "$output" == *"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE"* ]] || {
    echo "FAIL (EC-008): check name missing. AC-007 / INV5: row always emitted."
    echo "Output: $output"
    return 1
  }

  # EC-008 / INV3: ADVISORY must appear (empty string treated as absent → PC1 fires).
  [[ "$output" == *"ADVISORY"* ]] || {
    echo "FAIL (EC-008): output does not contain 'ADVISORY' for empty string value."
    echo "BC-6.25.001 INV3: empty string is not a valid integer; treated as absent → ADVISORY."
    echo "EC-008: value '' must trigger ADVISORY (same path as non-numeric per INV3)."
    echo "Output: $output"
    return 1
  }

  # EC-008: PASS must NOT appear for empty string.
  [[ "$output" != *"PASS"* ]] || {
    echo "FAIL (EC-008): output contains 'PASS' for empty string value — incorrect."
    echo "BC-6.25.001 INV3: empty string → treated as absent → ADVISORY, never PASS."
    echo "Output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-10 / EC-011 (UNREADABLE) / BC-6.25.001 INV1 + INV5
#   — unreadable settings.json (chmod 000) → ADVISORY, exit 0
#
# When settings.json EXISTS but is UNREADABLE (permission denied), jq cannot
# parse it. The helper MUST degrade gracefully:
#   (a) NEVER exit non-zero — advisory-only, no crash (INV1 / PC5)
#   (b) ALWAYS emit exactly one row containing CLAUDE_AUTOCOMPACT_PCT_OVERRIDE (INV5)
#   (c) Emit ADVISORY (not PASS) with a parse-error description referencing ADR-026
#
# NOTE: This test covers the UNREADABLE-file variant of EC-011.
# The SYNTACTICALLY-MALFORMED JSON variant of EC-011 is covered separately by
# test_autocompact_check_real_malformed_json_emits_ec011_advisory (F-P2-002).
# Both variants exercise the same ADVISORY path but with different failure triggers.
#
# F-P2-003 rename: this test was previously named
# "test_autocompact_check_malformed_json_emits_advisory_not_crash" — that name
# was misleading because the fixture uses chmod 000 (unreadable), not malformed JSON.
# Renamed to accurately describe the fixture. The genuine malformed-JSON case has
# its own test (T-14).
#
# Edge case exercised: EC-011 (unreadable settings.json → parse-error ADVISORY).
# ---------------------------------------------------------------------------

@test "test_autocompact_check_unreadable_settings_json_emits_advisory" {
  _require_helper

  # Fixture: project-local settings.json exists but is UNREADABLE (chmod 000).
  # chmod 000 triggers the jq read-failure path (permission denied).
  printf '{"env": {"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "70"}}' > "$WORK/.claude/settings.json"
  chmod 000 "$WORK/.claude/settings.json"

  run env HOME="$FAKE_HOME" PROJECT_ROOT="$WORK" bash "$HELPER" 2>&1

  # Restore permissions before any assertion so teardown can clean up regardless.
  chmod 644 "$WORK/.claude/settings.json"

  # INV1 / PC5 — MUST NOT exit non-zero (no crash on degenerate input).
  [ "$status" -eq 0 ] || {
    echo "FAIL (EC-011-unreadable / INV1): helper exited with status=$status on unreadable settings.json."
    echo "BC-6.25.001 INV1: advisory check MUST NEVER exit non-zero; degenerate input is not an exception."
    echo "BC-6.25.001 PC5: advisory-only; no blocking, no non-zero exit under any input."
    echo "Output: $output"
    return 1
  }

  # INV5 — row MUST always be emitted (even on parse error).
  [[ "$output" == *"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE"* ]] || {
    echo "FAIL (EC-011-unreadable / INV5): no row emitted for unreadable settings.json."
    echo "BC-6.25.001 INV5: the check row MUST ALWAYS be emitted, including on parse errors."
    echo "Output: $output"
    return 1
  }

  # EC-011: ADVISORY must appear (parse error → cannot verify → ADVISORY).
  [[ "$output" == *"ADVISORY"* ]] || {
    echo "FAIL (EC-011-unreadable): output does not contain 'ADVISORY' for unreadable settings.json."
    echo "BC-6.25.001 EC-011: unreadable file must produce an ADVISORY row."
    echo "Output: $output"
    return 1
  }

  # EC-011: PASS must NOT appear when settings.json is unreadable.
  [[ "$output" != *"PASS"* ]] || {
    echo "FAIL (EC-011-unreadable): output contains 'PASS' for unreadable settings.json — incorrect."
    echo "BC-6.25.001 EC-011: unreadable file → ADVISORY (parse-error path), never PASS."
    echo "Output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-11 / F-P2-001 / BC-6.25.001 PC3 — single-line JSON value 70 (canonical TV form)
#
# F-P2-001 finding: the OLD pure-bash parser failed single-line JSON; the jq
# rewrite MUST parse it correctly. This test uses the EXACT canonical test vector
# from BC-6.25.001 §Canonical Test Vectors:
#   `{"env": {"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "70"}}` → PASS row.
#
# The existing T-2 uses a multi-line (pretty-printed) fixture.
# This test uses the COMPACT single-line form to permanently guard the regression
# that F-P2-001 caught.
#
# Edge case exercised: EC-001 (single-line canonical TV — F-P2-001 regression guard).
# ---------------------------------------------------------------------------

@test "test_autocompact_check_single_line_json_value_70_is_pass" {
  _require_helper

  # Fixture: single-line JSON exactly matching the BC canonical test vector.
  # printf avoids trailing newline — this is the compact form the old parser failed on.
  printf '{"env": {"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "70"}}' > "$WORK/.claude/settings.json"

  run env HOME="$FAKE_HOME" PROJECT_ROOT="$WORK" bash "$HELPER" 2>&1

  # PC3 / F-P2-001: helper exits 0 (PASS).
  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P2-001 / EC-001 single-line): expected exit 0 but got status=$status"
    echo "BC-6.25.001 PC3: value 70 ≤ 80 → PASS. jq must handle single-line JSON."
    echo "F-P2-001: old pure-bash parser failed this form; jq rewrite must pass it."
    echo "Output: $output"
    return 1
  }

  # F-P2-001 / PC3: check name present.
  [[ "$output" == *"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE"* ]] || {
    echo "FAIL (F-P2-001 / EC-001 single-line): check name missing. INV5: row always emitted."
    echo "Output: $output"
    return 1
  }

  # F-P2-001 / PC3: PASS must be emitted for single-line JSON value 70.
  [[ "$output" == *"PASS"* ]] || {
    echo "FAIL (F-P2-001 / EC-001 single-line): output does not contain 'PASS' for single-line value 70."
    echo "BC-6.25.001 PC3: value 70 ≤ 80 → PASS (canonical value)."
    echo "F-P2-001 regression: jq must parse single-line JSON correctly."
    echo "Canonical TV: {\"env\": {\"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE\": \"70\"}} → PASS."
    echo "Output: $output"
    return 1
  }

  # F-P2-001: ADVISORY must NOT appear for single-line value 70.
  [[ "$output" != *"ADVISORY"* ]] || {
    echo "FAIL (F-P2-001 / EC-001 single-line): output contains 'ADVISORY' for single-line value 70."
    echo "BC-6.25.001 PC3: value ≤ 80 → PASS, not ADVISORY."
    echo "Output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-12 / F-P2-001 / BC-6.25.001 PC2 — single-line JSON value 85 (canonical TV form)
#
# Companion to T-11. Uses the exact single-line compact form from the BC canonical
# test vector table: `{"env": {"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "85"}}` → ADVISORY.
#
# The existing T-3 uses a multi-line fixture. This guards the single-line regression.
#
# Edge case exercised: EC-003 (single-line canonical TV — F-P2-001 regression guard).
# ---------------------------------------------------------------------------

@test "test_autocompact_check_single_line_json_value_85_is_advisory" {
  _require_helper

  # Fixture: single-line JSON with value 85, compact form (canonical TV for EC-003).
  printf '{"env": {"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "85"}}' > "$WORK/.claude/settings.json"

  run env HOME="$FAKE_HOME" PROJECT_ROOT="$WORK" bash "$HELPER" 2>&1

  # PC2 / F-P2-001: exit 0 (advisory; non-blocking; INV1).
  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P2-001 / EC-003 single-line): expected exit 0 but got status=$status"
    echo "BC-6.25.001 INV1: advisory check never blocks; exit 0 for single-line JSON too."
    echo "Output: $output"
    return 1
  }

  # F-P2-001 / PC2: check name present.
  [[ "$output" == *"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE"* ]] || {
    echo "FAIL (F-P2-001 / EC-003 single-line): check name missing. INV5: row always emitted."
    echo "Output: $output"
    return 1
  }

  # F-P2-001 / PC2: ADVISORY must appear for single-line value 85.
  [[ "$output" == *"ADVISORY"* ]] || {
    echo "FAIL (F-P2-001 / EC-003 single-line): output does not contain 'ADVISORY' for single-line value 85."
    echo "BC-6.25.001 PC2: value > 80 → ADVISORY."
    echo "F-P2-001: jq must parse single-line JSON and produce the correct classification."
    echo "Output: $output"
    return 1
  }

  # F-P2-001 / PC2: actual value (85) must appear in the advisory.
  [[ "$output" == *"85"* ]] || {
    echo "FAIL (F-P2-001 / EC-003 single-line): advisory does not include the actual value '85'."
    echo "BC-6.25.001 PC2: advisory details must contain the actual configured value."
    echo "Output: $output"
    return 1
  }

  # F-P2-001: PASS must NOT appear for single-line value 85.
  [[ "$output" != *"PASS"* ]] || {
    echo "FAIL (F-P2-001 / EC-003 single-line): output contains 'PASS' for value 85 — incorrect."
    echo "BC-6.25.001 PC2: value > 80 → ADVISORY, not PASS."
    echo "Output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-13 / F-P2-001 / BC-6.25.001 PC3 (boundary) — single-line JSON value 80
#
# Companion to T-11/T-12. Single-line compact form boundary test.
# BC canonical TV: `{"env": {"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "80"}}` → PASS (boundary).
#
# Edge case exercised: EC-004 (single-line canonical TV boundary — F-P2-001 regression guard).
# ---------------------------------------------------------------------------

@test "test_autocompact_check_single_line_json_value_80_is_pass_boundary" {
  _require_helper

  # Fixture: single-line JSON with boundary value 80.
  printf '{"env": {"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "80"}}' > "$WORK/.claude/settings.json"

  run env HOME="$FAKE_HOME" PROJECT_ROOT="$WORK" bash "$HELPER" 2>&1

  # PC3 / boundary: exit 0.
  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P2-001 / EC-004 single-line boundary): expected exit 0 but got status=$status"
    echo "BC-6.25.001 PC3: value 80 ≤ 80 → PASS (boundary inclusive)."
    echo "Output: $output"
    return 1
  }

  # F-P2-001 / PC3 / boundary: PASS must be emitted.
  [[ "$output" == *"PASS"* ]] || {
    echo "FAIL (F-P2-001 / EC-004 single-line boundary): output does not contain 'PASS' for single-line value 80."
    echo "BC-6.25.001 PC3: ceiling comparison is ≤ 80; value 80 is PASS (boundary inclusive)."
    echo "F-P2-001: jq must parse single-line JSON and apply correct boundary comparison."
    echo "Output: $output"
    return 1
  }

  # F-P2-001 / boundary: ADVISORY must NOT appear.
  [[ "$output" != *"ADVISORY"* ]] || {
    echo "FAIL (F-P2-001 / EC-004 single-line boundary): output contains 'ADVISORY' for value 80 — incorrect."
    echo "BC-6.25.001 PC3: value 80 ≤ 80 is PASS, not ADVISORY."
    echo "Output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-14 / F-P2-002 / BC-6.25.001 EC-011 — READABLE but SYNTACTICALLY MALFORMED JSON
#
# This test distinguishes from T-10 (unreadable file):
#   T-10 fixtures chmod 000 (UNREADABLE) — jq fails due to permission denied.
#   T-14 fixtures a READABLE file containing syntactically malformed JSON (truncated).
#
# F-P2-002 finding: EC-011 was only tested with an unreadable file (T-10). The
# syntactically-malformed-but-readable variant must be explicitly guarded.
#
# Requirements per BC-6.25.001 EC-011:
#   (a) Exit 0 (INV1 — never blocks)
#   (b) Emit a row containing CLAUDE_AUTOCOMPACT_PCT_OVERRIDE (INV5)
#   (c) Emit ADVISORY with details containing:
#         "settings.json parse error:" (canonical EC-011 prefix from BC)
#         "cannot verify" (canonical EC-011 wording)
#       Canonical TV (BC-6.25.001 §Canonical Test Vectors, parse-error row):
#         "settings.json parse error: <error>; cannot verify CLAUDE_AUTOCOMPACT_PCT_OVERRIDE"
#       No ADR-026 suffix in the canonical TV — the ADR-026 assertion was removed (F-P4-001).
#
# Edge case exercised: EC-011 (readable malformed JSON — F-P2-002 explicit coverage).
# ---------------------------------------------------------------------------

@test "test_autocompact_check_real_malformed_json_emits_ec011_advisory" {
  _require_helper

  # Fixture: READABLE file with syntactically malformed JSON (truncated — missing closing braces).
  # This is readable (chmod default 644) but jq cannot parse it.
  printf '{"env": {"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "70"' > "$WORK/.claude/settings.json"

  run env HOME="$FAKE_HOME" PROJECT_ROOT="$WORK" bash "$HELPER" 2>&1

  # F-P2-002 / INV1: exit 0 (advisory; never blocks).
  [ "$status" -eq 0 ] || {
    echo "FAIL (F-P2-002 / EC-011 malformed): expected exit 0 but got status=$status"
    echo "BC-6.25.001 INV1: advisory check MUST NEVER exit non-zero; malformed JSON is not an exception."
    echo "BC-6.25.001 PC5: advisory-only; no blocking, no non-zero exit."
    echo "Output: $output"
    return 1
  }

  # F-P2-002 / INV5: row MUST always be emitted.
  [[ "$output" == *"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE"* ]] || {
    echo "FAIL (F-P2-002 / EC-011 malformed): no row emitted for malformed settings.json."
    echo "BC-6.25.001 INV5: the check row MUST ALWAYS be emitted, including on parse errors."
    echo "Output: $output"
    return 1
  }

  # F-P2-002 / EC-011: ADVISORY must appear.
  [[ "$output" == *"ADVISORY"* ]] || {
    echo "FAIL (F-P2-002 / EC-011 malformed): output does not contain 'ADVISORY' for malformed JSON."
    echo "BC-6.25.001 EC-011: readable but syntactically malformed JSON must produce ADVISORY."
    echo "Output: $output"
    return 1
  }

  # F-P2-002 / EC-011: advisory must contain the canonical EC-011 prefix "settings.json parse error:".
  [[ "$output" == *"settings.json parse error:"* ]] || {
    echo "FAIL (F-P2-002 / EC-011 malformed): advisory does not contain 'settings.json parse error:'."
    echo "BC-6.25.001 EC-011: details must note 'settings.json parse error: <error>'."
    echo "Output: $output"
    return 1
  }

  # F-P2-002 / EC-011: advisory must contain "cannot verify".
  [[ "$output" == *"cannot verify"* ]] || {
    echo "FAIL (F-P2-002 / EC-011 malformed): advisory does not contain 'cannot verify'."
    echo "BC-6.25.001 EC-011 canonical wording: 'settings.json parse error: <error>; cannot verify CLAUDE_AUTOCOMPACT_PCT_OVERRIDE'."
    echo "Output: $output"
    return 1
  }

  # F-P2-002 / EC-011: PASS must NOT appear.
  [[ "$output" != *"PASS"* ]] || {
    echo "FAIL (F-P2-002 / EC-011 malformed): output contains 'PASS' for malformed JSON — incorrect."
    echo "BC-6.25.001 EC-011: malformed JSON → ADVISORY (parse-error path), never PASS."
    echo "Output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-15 / EC-012 / BC-6.25.001 INV3 — value "0" (zero) is out-of-range, emits ADVISORY
#
# EC-012 (added in BC-6.25.001 v1.1): values ≤ 0 are not valid compaction percentages.
# They are neither absent nor non-numeric, but they are out-of-range. The helper must
# emit a DISTINCT advisory that differs from both:
#   - PC1 "treating as absent" advisory (non-numeric/empty)
#   - PC2 ceiling advisory (value > 80)
#
# BC-6.25.001 INV3 verbatim EC-012 advisory details:
#   "Value <N> is not a valid compaction percentage (must be in range 1–100);
#    treating as misconfigured — recommend 70 per ADR-026 §Decision 5"
# where <N> = 0 for this test.
#
# Edge case exercised: EC-012 (value "0" — zero, out-of-range lower-bound).
# ---------------------------------------------------------------------------

@test "test_autocompact_check_ec012_value_zero_is_advisory" {
  _require_helper

  # Fixture: value "0" — zero, which is ≤ 0 (out-of-range lower-bound per EC-012).
  printf '{"env": {"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "0"}}' > "$WORK/.claude/settings.json"

  run env HOME="$FAKE_HOME" PROJECT_ROOT="$WORK" bash "$HELPER" 2>&1

  # EC-012 / INV1: exit 0 (advisory; never blocks).
  [ "$status" -eq 0 ] || {
    echo "FAIL (EC-012 / value 0): expected exit 0 but got status=$status"
    echo "BC-6.25.001 INV1: advisory check MUST NEVER exit non-zero; out-of-range value is not an exception."
    echo "Output: $output"
    return 1
  }

  # EC-012 / INV5: row MUST be emitted.
  [[ "$output" == *"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE"* ]] || {
    echo "FAIL (EC-012 / value 0): check name missing. INV5: row always emitted."
    echo "Output: $output"
    return 1
  }

  # EC-012: ADVISORY must appear.
  [[ "$output" == *"ADVISORY"* ]] || {
    echo "FAIL (EC-012 / value 0): output does not contain 'ADVISORY' for value 0."
    echo "BC-6.25.001 EC-012: value ≤ 0 → ADVISORY (out-of-range lower-bound)."
    echo "Output: $output"
    return 1
  }

  # EC-012: advisory must contain the canonical BC-6.25.001 v1.1 INV3 wording.
  # Verbatim: "Value 0 is not a valid compaction percentage (must be in range 1–100);
  #            treating as misconfigured — recommend 70 per ADR-026 §Decision 5"
  [[ "$output" == *"is not a valid compaction percentage"* ]] || {
    echo "FAIL (EC-012 / value 0): advisory does not contain 'is not a valid compaction percentage'."
    echo "BC-6.25.001 INV3 EC-012 verbatim: 'Value 0 is not a valid compaction percentage"
    echo "  (must be in range 1–100); treating as misconfigured — recommend 70 per ADR-026 §Decision 5'."
    echo "Output: $output"
    return 1
  }

  [[ "$output" == *"must be in range 1"* ]] || {
    echo "FAIL (EC-012 / value 0): advisory does not contain 'must be in range 1'."
    echo "BC-6.25.001 INV3 EC-012: advisory must state the valid range 1–100."
    echo "Output: $output"
    return 1
  }

  [[ "$output" == *"treating as misconfigured"* ]] || {
    echo "FAIL (EC-012 / value 0): advisory does not contain 'treating as misconfigured'."
    echo "BC-6.25.001 INV3 EC-012 verbatim: '...treating as misconfigured — recommend 70 per ADR-026 §Decision 5'."
    echo "Output: $output"
    return 1
  }

  # EC-012: the actual value (0) must appear in the advisory.
  [[ "$output" == *"Value 0"* ]] || {
    echo "FAIL (EC-012 / value 0): advisory does not contain 'Value 0'."
    echo "BC-6.25.001 INV3 EC-012: details must include the actual parsed value."
    echo "Output: $output"
    return 1
  }

  # EC-012: PASS must NOT appear.
  [[ "$output" != *"PASS"* ]] || {
    echo "FAIL (EC-012 / value 0): output contains 'PASS' for out-of-range value 0 — incorrect."
    echo "BC-6.25.001 EC-012: value ≤ 0 → ADVISORY (out-of-range), never PASS."
    echo "Output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-16 / EC-012 / BC-6.25.001 INV3 — value "-5" (negative) is out-of-range, emits ADVISORY
#
# Companion to T-15. Exercises the negative value arm of EC-012.
# BC-6.25.001 INV3 verbatim EC-012 advisory details (with <N> = -5):
#   "Value -5 is not a valid compaction percentage (must be in range 1–100);
#    treating as misconfigured — recommend 70 per ADR-026 §Decision 5"
#
# Edge case exercised: EC-012 (value "-5" — negative, out-of-range lower-bound).
# ---------------------------------------------------------------------------

@test "test_autocompact_check_ec012_value_negative_is_advisory" {
  _require_helper

  # Fixture: value "-5" — negative, ≤ 0 (out-of-range lower-bound per EC-012).
  printf '{"env": {"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "-5"}}' > "$WORK/.claude/settings.json"

  run env HOME="$FAKE_HOME" PROJECT_ROOT="$WORK" bash "$HELPER" 2>&1

  # EC-012 / INV1: exit 0 (advisory; never blocks).
  [ "$status" -eq 0 ] || {
    echo "FAIL (EC-012 / value -5): expected exit 0 but got status=$status"
    echo "BC-6.25.001 INV1: advisory check MUST NEVER exit non-zero; negative value is not an exception."
    echo "Output: $output"
    return 1
  }

  # EC-012 / INV5: row MUST be emitted.
  [[ "$output" == *"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE"* ]] || {
    echo "FAIL (EC-012 / value -5): check name missing. INV5: row always emitted."
    echo "Output: $output"
    return 1
  }

  # EC-012: ADVISORY must appear.
  [[ "$output" == *"ADVISORY"* ]] || {
    echo "FAIL (EC-012 / value -5): output does not contain 'ADVISORY' for value -5."
    echo "BC-6.25.001 EC-012: value ≤ 0 → ADVISORY (out-of-range lower-bound)."
    echo "Output: $output"
    return 1
  }

  # EC-012: advisory must contain the canonical BC-6.25.001 v1.1 INV3 wording.
  [[ "$output" == *"is not a valid compaction percentage"* ]] || {
    echo "FAIL (EC-012 / value -5): advisory does not contain 'is not a valid compaction percentage'."
    echo "BC-6.25.001 INV3 EC-012 verbatim: 'Value -5 is not a valid compaction percentage"
    echo "  (must be in range 1–100); treating as misconfigured — recommend 70 per ADR-026 §Decision 5'."
    echo "Output: $output"
    return 1
  }

  # EC-012: the actual value (-5) must appear in the advisory.
  [[ "$output" == *"Value -5"* ]] || {
    echo "FAIL (EC-012 / value -5): advisory does not contain 'Value -5'."
    echo "BC-6.25.001 INV3 EC-012: details must include the actual parsed value."
    echo "Output: $output"
    return 1
  }

  # EC-012: PASS must NOT appear.
  [[ "$output" != *"PASS"* ]] || {
    echo "FAIL (EC-012 / value -5): output contains 'PASS' for out-of-range value -5 — incorrect."
    echo "BC-6.25.001 EC-012: value ≤ 0 → ADVISORY (out-of-range), never PASS."
    echo "Output: $output"
    return 1
  }
}

# ---------------------------------------------------------------------------
# T-17 / jq-absent / BC-6.25.001 INV1 + INV5 — jq not found degrades gracefully
#
# When jq is not available in PATH, the helper MUST:
#   (a) Exit 0 (INV1 — never blocks)
#   (b) Emit exactly one row containing CLAUDE_AUTOCOMPACT_PCT_OVERRIDE (INV5)
#   (c) Emit ADVISORY naming jq as missing
#
# This guards the F-P1-001-class invariant: INV1/INV5 hold even when the parser
# dependency is absent. The helper's Step 0 guard: `command -v jq > /dev/null 2>&1`.
#
# Implementation: BASH_ENV is used to inject a function override that makes
# `command -v jq` return non-zero, simulating jq absence on a system where
# /usr/bin/jq or a brew-installed jq would otherwise be found.
#
# Edge case exercised: jq-absent path (F-P1-001-class INV1/INV5 robustness).
# ---------------------------------------------------------------------------

@test "test_autocompact_check_jq_absent_degrades_gracefully" {
  _require_helper

  # BASH_ENV file: inject a function override so `command -v jq` fails.
  # This simulates jq absence without manipulating filesystem state.
  # BASH_ENV is sourced by non-interactive bash on startup.
  local BASH_ENV_OVERRIDE
  BASH_ENV_OVERRIDE="$(mktemp)"
  cat > "$BASH_ENV_OVERRIDE" <<'BASH_ENV_EOF'
# Simulate jq absent: override command builtin so "command -v jq" returns failure.
command() {
  if [ "$1" = "-v" ] && [ "$2" = "jq" ]; then
    return 1
  fi
  builtin command "$@"
}
BASH_ENV_EOF

  # Fixture: a valid settings.json (we never reach parsing; jq guard fires first).
  printf '{"env": {"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE": "70"}}' > "$WORK/.claude/settings.json"

  run env HOME="$FAKE_HOME" PROJECT_ROOT="$WORK" BASH_ENV="$BASH_ENV_OVERRIDE" bash "$HELPER" 2>&1

  rm -f "$BASH_ENV_OVERRIDE"

  # INV1: exit 0 (advisory; never blocks; jq absence is not fatal).
  [ "$status" -eq 0 ] || {
    echo "FAIL (jq-absent / INV1): helper exited with status=$status when jq is absent."
    echo "BC-6.25.001 INV1: advisory check MUST NEVER exit non-zero; missing jq is not an exception."
    echo "Architecture Anchor (BC-6.25.001): jq absent → degrade to ADVISORY row (never fatal)."
    echo "Output: $output"
    return 1
  }

  # INV5: row MUST be emitted (even when jq is absent).
  [[ "$output" == *"CLAUDE_AUTOCOMPACT_PCT_OVERRIDE"* ]] || {
    echo "FAIL (jq-absent / INV5): no row emitted when jq is absent."
    echo "BC-6.25.001 INV5: the check row MUST ALWAYS be emitted, including when jq is missing."
    echo "Output: $output"
    return 1
  }

  # jq-absent: ADVISORY must appear.
  [[ "$output" == *"ADVISORY"* ]] || {
    echo "FAIL (jq-absent): output does not contain 'ADVISORY' when jq is absent."
    echo "BC-6.25.001 INV1 + Architecture Anchor: jq absent → ADVISORY row."
    echo "Output: $output"
    return 1
  }

  # jq-absent: advisory must name jq as the missing dependency.
  [[ "$output" == *"jq"* ]] || {
    echo "FAIL (jq-absent): advisory does not name 'jq'."
    echo "Helper Step 0 guard: advisory must identify jq as the required but absent tool."
    echo "Expected: '...jq is required but not found...'."
    echo "Output: $output"
    return 1
  }

  # jq-absent: PASS must NOT appear (cannot verify without jq).
  [[ "$output" != *"PASS"* ]] || {
    echo "FAIL (jq-absent): output contains 'PASS' when jq is absent — incorrect."
    echo "BC-6.25.001 INV1: cannot verify settings.json without jq → ADVISORY, never PASS."
    echo "Output: $output"
    return 1
  }
}
