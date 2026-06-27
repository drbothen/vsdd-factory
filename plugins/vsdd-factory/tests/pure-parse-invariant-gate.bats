#!/usr/bin/env bats
# pure-parse-invariant-gate.bats — S-18.08 pure-parse invariant consistency gate.
#
# Story:   S-18.08 — O-P8-002 Pure-Parse Invariant Consistency Gate
# BCs enforced: BC-4.14.001 Invariant 1 (pure-parse; no git or filesystem side effects)
#               BC-4.15.001 Invariant 1 (pure-parse; no filesystem, subprocess, or context access)
# VPs scanned:  VP-083, VP-081, VP-091 (D-572 VP-body extension)
# O-P8-002:     Adversarial finding — BC/VP prose must not contradict pure-parse invariant claims.
#
# Red Gate condition:
#   All 5 tests MUST FAIL before implementation (S-18.01..S-18.07 artifacts) because:
#   - The BC files (.factory/specs/behavioral-contracts/ss-04/BC-4.14.001.md,
#     BC-4.15.001.md) do not yet exist in the worktree (pre-S-18.01..07 state).
#   - The VP files (.factory/specs/verification-properties/VP-083.md, VP-081.md,
#     VP-091.md) do not yet exist.
#   - grep on non-existent files → non-zero exit → assertion failures.
#
# @test fatal-path contract (O-P7-001 / story v1.5):
#   Every @test MUST use:
#     run bash -c '<snippet>'
#     assert_success
#     refute_output --partial "FAIL"
#   so that echo "FAIL: ..." lines emitted by the gate snippets become real test
#   failures. assert_success and refute_output are defined as helpers below
#   (bats-assert is not installed system-wide; helpers are inlined here per
#   project convention of no external bats library dependencies).
#
# EC coverage (L-BB-red-gate-test-plan-ec-coverage-parity, D-699):
#   EC-001: AC-001 test — BC-4.14.001 normative sections contain 0 hits → gate passes
#   EC-002: AC-002 test — BC-4.15.001 normative sections contain 0 hits → gate passes
#   EC-003: AC-005 discovery — BCs not declaring pure-parse are not scanned (no false positive)
#   EC-004: AC-005 discovery — future pure-parse BCs discovered automatically via grep -rl
#   EC-005: AC-004 test — VP loop scans all 3 VPs; changelog-row hits would show non-zero HITS
#           and cause test failure (implementer must add section filter per AC-001 pattern)
#   EC-006: AC-001 exclusion filter handles "caller's responsibility" §Description prose —
#           lines containing "caller", "shell wave-handoff", "shell layer", or
#           "derives from real substrate" are excluded from the load-bearing hit count.
#
# Paths: all repo-root-relative paths are resolved via REPO_ROOT derived from
# BATS_TEST_DIRNAME (tests/ → plugins/vsdd-factory/ → .. → .. = repo root).

# ---------------------------------------------------------------------------
# Inline assert_success / refute_output helpers
# (bats-assert API surface; implemented without the bats-assert package)
# ---------------------------------------------------------------------------

# assert_success: asserts that the most recent `run` command exited with status 0.
assert_success() {
  if [ "$status" -ne 0 ]; then
    echo "assert_success: expected exit status 0, got $status" >&2
    echo "output: $output" >&2
    return 1
  fi
}

# refute_output --partial <substring>: asserts that $output does NOT contain <substring>.
# Only the --partial form is used in this file.
refute_output() {
  local mode=""
  local substring=""
  while [ "$#" -gt 0 ]; do
    case "$1" in
      --partial) mode="partial"; shift ;;
      *) substring="$1"; shift ;;
    esac
  done
  if [ "$mode" = "partial" ]; then
    if echo "$output" | grep -qF "$substring"; then
      echo "refute_output --partial: output contains forbidden substring '$substring'" >&2
      echo "output: $output" >&2
      return 1
    fi
  fi
}

# ---------------------------------------------------------------------------
# setup
# ---------------------------------------------------------------------------

setup() {
  # Derive repo root from BATS_TEST_DIRNAME:
  #   plugins/vsdd-factory/tests/ → plugins/vsdd-factory/ → plugins/ → repo root
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
}

# ---------------------------------------------------------------------------
# AC-001 / BC-4.14.001 Invariant 1 pure-parse body scan
#
# Scans BC-4.14.001.md normative behavioral sections for substrate-read patterns.
# Excludes: §Related BCs, §Architecture Anchors, §Story Anchor, §VP Anchors,
# §Traceability, §Changelog sections, and lines about caller/shell layer
# responsibility (EC-006 — these describe the calling shell layer, not the WASM gate).
#
# Red Gate: BC-4.14.001.md absent → grep exits non-zero → bash -c exits non-zero
# → assert_success fails.
# ---------------------------------------------------------------------------

@test "test_bc_4_14_001_pure_parse_invariant_zero_substrate_reads_in_normative_sections" {
  run bash -c '
    set -euo pipefail
    BC_FILE="'"$REPO_ROOT"'/.factory/specs/behavioral-contracts/ss-04/BC-4.14.001.md"
    # File must exist — gate cannot scan a missing artifact.
    [ -f "$BC_FILE" ] || { echo "FAIL: $BC_FILE does not exist"; exit 1; }
    HITS=$(grep -Ei "sprint-state\.yaml|HANDOFF\.md[^-]|git-log|git-cat-file" \
      "$BC_FILE" \
      | grep -Ev "^(##? (Related BCs|Architecture Anchors|Story Anchor|VP Anchors|Traceability|Changelog)|.*caller|.*shell wave-handoff|.*shell layer|.*derives from real substrate)" \
      | wc -l)
    [ "$HITS" -eq 0 ]
  '
  assert_success
  refute_output --partial "FAIL"
}

# ---------------------------------------------------------------------------
# AC-002 / BC-4.15.001 Invariant 1 pure-parse body scan
#
# Scans BC-4.15.001.md normative behavioral sections for substrate-read patterns.
# Note: `factory-artifacts` exclusion not needed in the grep pattern — it is a
# load-bearing hit only if it appears as a read substrate in normative sections;
# in §Traceability/§Changelog it is excluded by the section-header filter.
#
# Red Gate: BC-4.15.001.md absent → grep exits non-zero → assert_success fails.
# ---------------------------------------------------------------------------

@test "test_bc_4_15_001_pure_parse_invariant_zero_substrate_reads_in_normative_sections" {
  run bash -c '
    set -euo pipefail
    BC_FILE="'"$REPO_ROOT"'/.factory/specs/behavioral-contracts/ss-04/BC-4.15.001.md"
    [ -f "$BC_FILE" ] || { echo "FAIL: $BC_FILE does not exist"; exit 1; }
    HITS=$(grep -Ei "sprint-state\.yaml|HANDOFF\.md[^-]|git-log|git-cat-file|git show" \
      "$BC_FILE" \
      | grep -Ev "^(##? (Related BCs|Architecture Anchors|Story Anchor|VP Anchors|Traceability|Changelog))" \
      | wc -l)
    [ "$HITS" -eq 0 ]
  '
  assert_success
  refute_output --partial "FAIL"
}

# ---------------------------------------------------------------------------
# AC-003 / VP-091 body scan: 0 substrate-read hits contradicting pure-parse
#
# VP-091 verifies BC-4.15.001. Scans all sections for substrate-read patterns
# that would contradict the pure-parse invariant. VP-091 §0 "Structural
# Precondition" may reference factory-artifacts in a prohibition context only.
#
# Red Gate: VP-091.md absent → grep exits non-zero → assert_success fails.
# ---------------------------------------------------------------------------

@test "test_vp_091_body_zero_substrate_reads_contradicting_pure_parse" {
  run bash -c '
    set -euo pipefail
    VP_FILE="'"$REPO_ROOT"'/.factory/specs/verification-properties/VP-091.md"
    [ -f "$VP_FILE" ] || { echo "FAIL: $VP_FILE does not exist"; exit 1; }
    HITS=$(grep -Ei "sprint-state\.yaml|git-log|git-cat-file" "$VP_FILE" | wc -l)
    [ "$HITS" -eq 0 ]
  '
  assert_success
  refute_output --partial "FAIL"
}

# ---------------------------------------------------------------------------
# AC-004 / D-572 VP-body extension: scan all VP files associated with pure-parse BCs
#
# BC-4.14.001 VP Anchors: VP-083, VP-081
# BC-4.15.001 VP Anchors: VP-091 (covered by AC-003 above)
# Gate scans all three. 0 load-bearing hits required across all.
#
# EC-005 coverage: if any VP file body contains a sprint-state.yaml / git-log /
# git-cat-file reference (including in historical §Changelog rows), HITS > 0
# and the loop emits "FAIL: <file> has N load-bearing substrate-read hits".
# The refute_output --partial "FAIL" assertion then fails the test correctly.
#
# Red Gate: VP-083.md or VP-081.md absent → grep exits non-zero inside the loop
# (non-zero from missing file) → bash -c exits non-zero → assert_success fails.
# ---------------------------------------------------------------------------

@test "test_vp_083_081_091_zero_substrate_reads_in_pure_parse_vp_set" {
  run bash -c '
    set -euo pipefail
    any_fail=0
    for VP_FILE in \
      "'"$REPO_ROOT"'/.factory/specs/verification-properties/VP-083.md" \
      "'"$REPO_ROOT"'/.factory/specs/verification-properties/VP-081.md" \
      "'"$REPO_ROOT"'/.factory/specs/verification-properties/VP-091.md"; do
      if [ ! -f "$VP_FILE" ]; then
        echo "FAIL: $VP_FILE does not exist"
        any_fail=1
        continue
      fi
      HITS=$(grep -Ei "sprint-state\.yaml|git-log|git-cat-file" "$VP_FILE" | wc -l)
      [ "$HITS" -eq 0 ] || echo "FAIL: $VP_FILE has $HITS load-bearing substrate-read hits"
    done
    [ "$any_fail" -eq 0 ]
  '
  assert_success
  refute_output --partial "FAIL"
}

# ---------------------------------------------------------------------------
# AC-005 / complete BC discovery: find all pure-parse BCs in ss-04 dynamically
#
# Discovery-first pattern (Architecture Compliance Rule 2): uses grep -rl rather
# than a hardcoded list so future pure-parse BCs are scanned automatically.
# Verifies that BC-4.14.001 and BC-4.15.001 are in the discovered set.
# Then scans each discovered BC for substrate-read violations.
#
# EC-003 coverage: BCs that do not contain "pure-parse" or "pure function"
# are not discovered and not scanned — no false positive.
# EC-004 coverage: future pure-parse BCs added to ss-04 are discovered
# automatically by grep -rl.
#
# Red Gate: ss-04 directory absent → grep -rl returns nothing → BC-4.14.001
# not discovered → "FAIL: BC-4.14.001 not discovered" emitted → refute_output
# --partial "FAIL" catches it. Also: bash -c exits non-zero if `grep -q` on
# empty PURE_PARSE_BCS fails.
# ---------------------------------------------------------------------------

@test "test_pure_parse_bc_discovery_and_scan_finds_all_ss04_pure_parse_bcs" {
  run bash -c '
    # Discovery: find all pure-parse BCs in ss-04
    PURE_PARSE_BCS=$(grep -rl "pure-parse\|pure function" \
      "'"$REPO_ROOT"'/.factory/specs/behavioral-contracts/ss-04/" 2>/dev/null)
    # Verify at least BC-4.14.001 and BC-4.15.001 are discovered
    echo "$PURE_PARSE_BCS" | grep -q "BC-4.14.001" || echo "FAIL: BC-4.14.001 not discovered"
    echo "$PURE_PARSE_BCS" | grep -q "BC-4.15.001" || echo "FAIL: BC-4.15.001 not discovered"
    # Scan each discovered BC for substrate-read violations
    for BC_FILE in $PURE_PARSE_BCS; do
      HITS=$(grep -Ei "sprint-state\.yaml|git-log|git-cat-file" "$BC_FILE" | wc -l)
      [ "$HITS" -eq 0 ] || echo "FAIL: $BC_FILE has $HITS load-bearing substrate-read hits"
    done
  '
  assert_success
  refute_output --partial "FAIL"
}
