#!/usr/bin/env bats
# check-state-health-autocompact.bats — Red Gate tests for S-18.10.
#
# Tests the NOT-YET-CREATED helper:
#   plugins/vsdd-factory/skills/check-state-health/lib/check-autocompact-setting.sh
#
# The helper reads .claude/settings.json (project-local) then
# ~/.claude/settings.json (global fallback), parses env.CLAUDE_AUTOCOMPACT_PCT_OVERRIDE,
# and emits:
#   PASS     — key present and numeric value ≤ 80
#   ADVISORY — key absent, value > 80, or non-numeric (treated as absent)
# Advisory-only; never blocks; no side effects; set -euo pipefail.
#
# 7 tests map to AC-001..AC-007 (BC-6.25.001 PC1–PC5, INV1–INV5; VP-092):
#
#   test_autocompact_check_absent_key_emits_advisory       — AC-001 / BC-6.25.001 PC1
#   test_autocompact_check_value_70_is_pass                — AC-003 / BC-6.25.001 PC3
#   test_autocompact_check_value_85_is_advisory            — AC-002 / BC-6.25.001 PC2
#   test_autocompact_check_value_80_is_pass_boundary       — AC-003 / BC-6.25.001 PC3 (boundary)
#   test_autocompact_check_no_settings_json_emits_advisory — AC-001 + AC-004 / PC1 + PC4(c)
#   test_autocompact_check_project_local_takes_precedence_over_global  — AC-004 / PC4 + INV2
#   test_autocompact_check_non_numeric_value_treated_as_absent         — AC-006 / INV3
#
# Red Gate minimum: all 7 MUST fail before check-autocompact-setting.sh exists.
# Failure must be because the helper is absent / behavior unimplemented, NOT because
# of a malformed test (BC-5.38.001).
#
# Edge Cases exercised per AC/test mapping:
#   EC-001 (value 70, PASS)           → test_autocompact_check_value_70_is_pass
#   EC-002 (env block present, no key)→ test_autocompact_check_absent_key_emits_advisory
#   EC-003 (value 85, ADVISORY)       → test_autocompact_check_value_85_is_advisory
#   EC-004 (value 80, PASS boundary)  → test_autocompact_check_value_80_is_pass_boundary
#   EC-005 (global fallback value 70) → test_autocompact_check_no_settings_json_emits_advisory
#                                        (no settings.json path; EC-006 subset)
#   EC-006 (both absent)              → test_autocompact_check_no_settings_json_emits_advisory
#   EC-007 (env block absent)         → test_autocompact_check_absent_key_emits_advisory
#   EC-008 (empty string value)       → test_autocompact_check_non_numeric_value_treated_as_absent
#   EC-009 (value "auto", non-integer)→ test_autocompact_check_non_numeric_value_treated_as_absent
#   EC-010 (project-local 85, global 70; project-local wins)
#                                     → test_autocompact_check_project_local_takes_precedence_over_global
#   EC-011 (malformed JSON)           → covered by absent-key path (ADVISORY emitted, no crash)
#
# Story:   S-18.10
# BC:      BC-6.25.001 v1.0 — all 5 PCs and INV1–INV5
# VP:      VP-092
# ADR:     ADR-026 §Decision 5 (value ceiling 80; canonical value 70)
#          ADR-026 §F-11 (check-state-health must verify this key)
# File:    plugins/vsdd-factory/tests/check-state-health-autocompact.bats
#
# Run (once helper exists and is GREEN):
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
# Used by all 7 tests: if the helper is absent, the test fails with a
# descriptive message that guides the implementer (T-3 in S-18.10).
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
