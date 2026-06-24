#!/usr/bin/env bats
# precompact-flush-prune.bats — Red Gate bats tests for S-18.04b.
#
# Tests the `precompact-flush-prune.sh` helper against the VP-090 specification
# and story ACs AC-009..AC-013.
#
# # Red Gate condition
# All tests that invoke the prune script FAIL against the stub (which always
# exits 1 with "stub not implemented"). Tests are genuine behavioral assertions
# that will PASS once the implementer fills in the real logic.
#
# # Coverage map (Red Gate Test Table rows)
#   test_prune_structural_precondition_no_newline  → AC-009 / VP-090 §0
#   test_prune_threshold_1000_no_prune             → AC-013 / VP-090 §4 (exact boundary)
#   test_prune_threshold_1001_prunes_to_500        → AC-010 / VP-090 §1
#   test_prune_atomic_write_preserves_last_line    → AC-011 / VP-090 §2 / §4
#   test_prune_empty_file_noop                     → AC-013 / VP-090 §4 (empty)
#
# Additional edge-case coverage:
#   test_prune_threshold_500_no_prune              → AC-013 (500 lines: no prune)
#   test_prune_preserves_first_retained_line       → AC-010 (first-of-500 is entry 502 out of 1001)
#   test_prune_result_ends_with_newline            → VP-090 §1 PC1 (file ends with \n)
#   test_prune_error_message_on_no_newline         → AC-009 (exact stderr message)
#
# # VP / BC trace
#   VP-090 §0 — structural precondition (trailing \n required before prune)
#   VP-090 §1 — threshold: >1000 → prune to 500
#   VP-090 §2 — atomic write (temp + mv rename)
#   VP-090 §3 — invocation context (script exists; not in hooks-registry.toml)
#   VP-090 §4 — boundary conditions (0, 500, 1000, 1001 lines)
#   AC-009 — structural precondition check with exact error message
#   AC-010 — threshold: >1000 → prune to 500 (strictly greater than)
#   AC-011 — atomic write; last line preserved
#   AC-012 — NOT registered as hook plugin (static file check)
#   AC-013 — boundary: 0, 500, 1000, 1001 lines

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  PRUNE_SCRIPT="$REPO_ROOT/plugins/vsdd-factory/hooks/precompact-flush-prune.sh"
  WORK="$(mktemp -d)"
  TEST_LOG_FILE="$WORK/precompact-flush-log"
}

teardown() {
  rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# Helper: generate N canonical 4-field log entries (each \n-terminated).
# Format: "<ISO-timestamp> sha_<NNNN> v1.0/S-18.04 commit"
# The generated file ends with \n (§0 structural precondition satisfied).
# ---------------------------------------------------------------------------

_generate_log_fixture() {
  local count="$1"
  local file="$2"
  : > "$file"
  local i
  for i in $(seq 1 "$count"); do
    printf "2026-01-01T%08dZ sha_%04d v1.0/S-18.04 commit\n" "$i" "$i" >> "$file"
  done
}

# ---------------------------------------------------------------------------
# Helper: verify a file ends with \n (byte 0x0a = newline).
# Returns 0 (success) if the last byte is \n.
# ---------------------------------------------------------------------------

_assert_ends_with_newline() {
  local file="$1"
  local last_byte
  last_byte=$(tail -c 1 "$file" | xxd -p 2>/dev/null || tail -c 1 "$file" | od -An -tx1 | tr -d ' \n')
  [ "$last_byte" = "0a" ]
}

# ---------------------------------------------------------------------------
# Helper: script exists check.
# ---------------------------------------------------------------------------

_require_prune_script() {
  if [ ! -f "$PRUNE_SCRIPT" ]; then
    skip "precompact-flush-prune.sh not found at $PRUNE_SCRIPT"
  fi
}

# ---------------------------------------------------------------------------
# AC-009 / VP-090 §0 — structural precondition: file NOT ending with \n
# Red Gate Test Table row: test_prune_structural_precondition_no_newline
# ---------------------------------------------------------------------------

@test "test_prune_structural_precondition_no_newline: exits non-zero without modification" {
  _require_prune_script

  # Create a 1001-line file (above threshold) that does NOT end with \n.
  # We write 1001 entries but strip the final newline.
  _generate_log_fixture 1001 "$TEST_LOG_FILE"
  # Remove the trailing newline to simulate a corrupted partial write.
  truncate -s -1 "$TEST_LOG_FILE" 2>/dev/null \
    || perl -i -pe 'chomp if eof' "$TEST_LOG_FILE" 2>/dev/null \
    || python3 -c "
import sys
with open('$TEST_LOG_FILE', 'rb') as f:
    data = f.read()
with open('$TEST_LOG_FILE', 'wb') as f:
    f.write(data.rstrip(b'\n'))
"
  # Record the byte count before the script runs.
  local size_before
  size_before=$(wc -c < "$TEST_LOG_FILE")

  # Run the prune script — it should exit non-zero.
  run bash "$PRUNE_SCRIPT" "$TEST_LOG_FILE"
  [ "$status" -ne 0 ]

  # Verify the file was NOT modified (same byte count).
  local size_after
  size_after=$(wc -c < "$TEST_LOG_FILE")
  [ "$size_after" -eq "$size_before" ]
}

@test "test_prune_error_message_on_no_newline: emits canonical error message on stderr" {
  _require_prune_script

  # Create 5-entry file without trailing newline.
  _generate_log_fixture 5 "$TEST_LOG_FILE"
  truncate -s -1 "$TEST_LOG_FILE" 2>/dev/null \
    || python3 -c "
with open('$TEST_LOG_FILE', 'rb') as f:
    data = f.read()
with open('$TEST_LOG_FILE', 'wb') as f:
    f.write(data.rstrip(b'\n'))
"

  run bash "$PRUNE_SCRIPT" "$TEST_LOG_FILE" 2>&1
  # AC-009: exact error message required.
  echo "$output" | grep -q "precompact-flush-log structural violation: file must end with newline before pruning"
}

# ---------------------------------------------------------------------------
# AC-013 / VP-090 §4 — empty file: no-op
# Red Gate Test Table row: test_prune_empty_file_noop
# ---------------------------------------------------------------------------

@test "test_prune_empty_file_noop: empty file exits 0 with no modification" {
  _require_prune_script

  # Create empty (0-byte) file.
  : > "$TEST_LOG_FILE"
  local size_before
  size_before=$(wc -c < "$TEST_LOG_FILE")
  [ "$size_before" -eq 0 ]

  run bash "$PRUNE_SCRIPT" "$TEST_LOG_FILE"
  # AC-013: empty file is a no-op exit 0 (not a structural violation error).
  [ "$status" -eq 0 ]

  local size_after
  size_after=$(wc -c < "$TEST_LOG_FILE")
  [ "$size_after" -eq 0 ]
}

# ---------------------------------------------------------------------------
# AC-013 / VP-090 §4 — exactly 1000 lines: no prune (strictly > 1000 threshold)
# Red Gate Test Table row: test_prune_threshold_1000_no_prune
# ---------------------------------------------------------------------------

@test "test_prune_threshold_1000_no_prune: 1000-line file exits 0 without modification" {
  _require_prune_script

  _generate_log_fixture 1000 "$TEST_LOG_FILE"

  # Verify structural precondition: file ends with \n.
  _assert_ends_with_newline "$TEST_LOG_FILE"

  local size_before
  size_before=$(wc -c < "$TEST_LOG_FILE")

  run bash "$PRUNE_SCRIPT" "$TEST_LOG_FILE"
  [ "$status" -eq 0 ]

  # File must be unchanged.
  local size_after
  size_after=$(wc -c < "$TEST_LOG_FILE")
  [ "$size_after" -eq "$size_before" ]

  # Line count must still be 1000.
  local line_count
  line_count=$(wc -l < "$TEST_LOG_FILE")
  [ "$line_count" -eq 1000 ]
}

@test "test_prune_threshold_500_no_prune: 500-line file exits 0 without modification" {
  _require_prune_script

  _generate_log_fixture 500 "$TEST_LOG_FILE"
  _assert_ends_with_newline "$TEST_LOG_FILE"

  local size_before
  size_before=$(wc -c < "$TEST_LOG_FILE")

  run bash "$PRUNE_SCRIPT" "$TEST_LOG_FILE"
  [ "$status" -eq 0 ]

  local size_after
  size_after=$(wc -c < "$TEST_LOG_FILE")
  [ "$size_after" -eq "$size_before" ]

  local line_count
  line_count=$(wc -l < "$TEST_LOG_FILE")
  [ "$line_count" -eq 500 ]
}

# ---------------------------------------------------------------------------
# AC-010 / VP-090 §1 — 1001 lines: prune to 500
# Red Gate Test Table row: test_prune_threshold_1001_prunes_to_500
# ---------------------------------------------------------------------------

@test "test_prune_threshold_1001_prunes_to_500: 1001-line file pruned to 500 lines" {
  _require_prune_script

  _generate_log_fixture 1001 "$TEST_LOG_FILE"
  _assert_ends_with_newline "$TEST_LOG_FILE"

  run bash "$PRUNE_SCRIPT" "$TEST_LOG_FILE"
  [ "$status" -eq 0 ]

  # AC-010: file must have exactly 500 lines after prune.
  local line_count
  line_count=$(wc -l < "$TEST_LOG_FILE")
  [ "$line_count" -eq 500 ]
}

# ---------------------------------------------------------------------------
# AC-011 / VP-090 §2 + §4 — atomic write; last line preserved
# Red Gate Test Table row: test_prune_atomic_write_preserves_last_line
# ---------------------------------------------------------------------------

@test "test_prune_atomic_write_preserves_last_line: last line is unchanged after prune" {
  _require_prune_script

  # Generate 1001 entries. Entry 1001 is the most recent (last line).
  _generate_log_fixture 1001 "$TEST_LOG_FILE"
  _assert_ends_with_newline "$TEST_LOG_FILE"

  # Record the last line before pruning.
  local last_line_before
  last_line_before=$(tail -1 "$TEST_LOG_FILE")

  run bash "$PRUNE_SCRIPT" "$TEST_LOG_FILE"
  [ "$status" -eq 0 ]

  # Assert: last line after prune is identical.
  local last_line_after
  last_line_after=$(tail -1 "$TEST_LOG_FILE")
  [ "$last_line_after" = "$last_line_before" ]

  # Assert: last entry is sha_1001 (the most recent entry is preserved).
  echo "$last_line_after" | grep -q "sha_1001"
}

@test "test_prune_result_ends_with_newline: pruned file ends with \\n" {
  _require_prune_script

  _generate_log_fixture 1001 "$TEST_LOG_FILE"

  run bash "$PRUNE_SCRIPT" "$TEST_LOG_FILE"
  [ "$status" -eq 0 ]

  # VP-090 §1 PC1: pruned file must still end with \n.
  _assert_ends_with_newline "$TEST_LOG_FILE"
}

@test "test_prune_preserves_first_retained_line: first line after prune is entry 502 of 1001" {
  _require_prune_script

  # With 1001 entries, keeping the last 500 means entry 502 becomes the first line.
  _generate_log_fixture 1001 "$TEST_LOG_FILE"

  run bash "$PRUNE_SCRIPT" "$TEST_LOG_FILE"
  [ "$status" -eq 0 ]

  # First retained entry must be sha_0502.
  local first_line
  first_line=$(head -1 "$TEST_LOG_FILE")
  echo "$first_line" | grep -q "sha_0502"
}

# ---------------------------------------------------------------------------
# AC-012 / VP-090 §3 — invocation context: NOT registered as hook plugin.
# Static check: grep hooks-registry.toml for prune script name.
# This test is GREEN-BY-DESIGN (static file inspection; no stub to fail against).
# ---------------------------------------------------------------------------

@test "test_prune_not_in_hooks_registry: precompact-flush-prune.sh not registered as hook" {
  local registry
  registry="$(cd "${BATS_TEST_DIRNAME}/.." && pwd)/hooks-registry.toml"
  if [ ! -f "$registry" ]; then
    skip "hooks-registry.toml not found at $registry"
  fi
  # AC-012: the prune script must NOT appear in the registry.
  run grep -c "precompact-flush-prune" "$registry"
  # grep -c returns 0 when no match found; assert 0 count.
  [ "$output" = "0" ] || [ "$status" -ne 0 ]
}

# ---------------------------------------------------------------------------
# VP-090 §3 — prune.sh syntax check (structural, not behavioral).
# This test is RED only if the script is missing (stub exists → PASS for syntax).
# Included to ensure the bats suite runs a syntax gate.
# ---------------------------------------------------------------------------

@test "test_prune_script_syntax_valid: passes bash syntax check" {
  _require_prune_script
  run bash -n "$PRUNE_SCRIPT"
  [ "$status" -eq 0 ]
}
