#!/usr/bin/env bats
# pure-parse-invariant-gate.bats — S-18.08 v1.7 pure-parse invariant consistency gate.
#
# Story:   S-18.08 v1.7 — O-P8-002 Pure-Parse Invariant Consistency Gate (ADR-026 §Decision 14)
# BCs enforced: BC-4.14.001 Invariant 1 (pure-parse; no git or filesystem side effects)
#               BC-4.15.001 Invariant 1 (pure-parse; no filesystem, subprocess, or context access)
# VPs scanned:  VP-083, VP-081, VP-091 (D-572 VP-body extension)
# O-P8-002:     Adversarial finding — BC/VP prose must not contradict pure-parse invariant claims.
#
# F-P1-001/F-P1-002 remediation (architect-refined §Decision 14 v1.31):
#   - Discovery anchored to ## Invariants section (replaces grep -rl "pure.parse" which
#     over-matched BC-INDEX.md and ~190 SS-07 prose mentions).
#   - Layer-1 awk uses whitelist terminator — stops at the first NON-normative heading,
#     robust to ## Related BCs (Recommended) variants.
#   - Fail-loud scannability guard: a discovered BC lacking ## Preconditions FAILS loudly,
#     not vacuously passes.
#
# Detection algorithm (ADR-026 §Decision 14 v1.31) — three-layer pipeline:
#   Layer 1 (BC files only): normative-section extraction via awk
#     awk '/^## Preconditions$/{ found=1 }
#          found && /^## / && !/^## (Preconditions|Postconditions|Invariants|Edge Cases|Error Paths|Canonical Test Vectors)$/{ exit }
#          found{ print }'
#     (whitelist terminator stops at first NON-normative heading)
#   Layer 2: verb+substrate collocation grep
#     grep -Ei "(reads?|loads?|fetches|derives?|access(es)?|retrieves?|opens?|parses?)\s+.{0,80}
#             (sprint-state\.yaml|git-log|git-cat-file)"
#     (VP scans ADD factory-artifacts to substrate set)
#   Layer 3: negation-cue exclusion grep
#     grep -Eiv "no |not |NOT |without|never|does not|MUST NOT|is NOT|cannot|do NOT|only from|exclusively"
#     (VP scans ADD grep -Ev "^\s*//" to strip Rust/bash comment lines)
#
# @test fatal-path contract (O-P7-001 / story v1.5+):
#   Every @test MUST use:
#     run bash -c '<snippet>'
#     assert_success
#     refute_output --partial "FAIL"
#   so that echo "FAIL: ..." lines emitted by the gate snippets become real test
#   failures. assert_success and refute_output are defined as helpers below
#   (bats-assert is not installed system-wide; helpers are inlined here per
#   project convention of no external bats library dependencies).
#
# .factory/ resolution (worktree topology):
#   .factory/ is an orphan-branch (factory-artifacts) worktree mounted ONLY at
#   the main checkout root. It is NOT present in feature worktrees.
#   Resolution: BATS_TEST_DIRNAME/../../.. gives the feature worktree root.
#   If $ROOT/.factory/specs does NOT exist, fall back to the primary worktree root
#   derived from `git -C $ROOT rev-parse --git-common-dir` (parent of .git).
#   This makes the suite pass both locally and in CI.
#
# Pipefail note: grep exits 1 when no lines match (correct 0-hit outcome). Using
# `|| true` on HITS=... assignments prevents set -e from aborting when the scan
# correctly finds 0 hits. Explicit `[ "$HITS" -eq 0 ] || echo "FAIL: ..."` provides
# the load-bearing assertion.

# ---------------------------------------------------------------------------
# Inline assert_success / refute_output helpers
# (bats-assert API surface; implemented without the bats-assert package)
# ---------------------------------------------------------------------------

assert_success() {
  if [ "$status" -ne 0 ]; then
    echo "assert_success: expected exit status 0, got $status" >&2
    echo "output: $output" >&2
    return 1
  fi
}

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
# setup — resolve FACTORY_ROOT with worktree-topology fallback
# ---------------------------------------------------------------------------

setup() {
  # Candidate root: BATS_TEST_DIRNAME is .../plugins/vsdd-factory/tests — 3 levels up
  local candidate_root
  candidate_root="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"

  if [ -d "${candidate_root}/.factory/specs" ]; then
    FACTORY_ROOT="${candidate_root}"
  else
    # Feature-worktree case: .factory/ is not mounted here.
    # git --git-common-dir gives the common .git dir (in the main worktree).
    # Its parent is the main worktree root where .factory/ is mounted.
    local git_common_dir
    git_common_dir="$(git -C "${candidate_root}" rev-parse --git-common-dir 2>/dev/null || true)"
    FACTORY_ROOT="$(dirname "${git_common_dir}")"
  fi

  export FACTORY_ROOT
}

# ---------------------------------------------------------------------------
# AC-001 / test_bc_4_14_001_pure_parse_invariant_zero_verb_substrate_hits_normative
#
# Three-layer pipeline on BC-4.14.001.md:
#   Layer 1: awk normative-section extraction (whitelist terminator — stops at first
#            NON-normative heading; robust to ## Related BCs (Recommended) variants)
#   Layer 2: verb+substrate collocation grep
#   Layer 3: negation-cue exclusion grep
# Expected: 0 hits.
#
# Red Gate: BC-4.14.001.md absent → file check fails → echo FAIL + exit 1
#           → assert_success fails.
# ---------------------------------------------------------------------------

@test "test_bc_4_14_001_pure_parse_invariant_zero_verb_substrate_hits_normative" {
  local factory_root="$FACTORY_ROOT"
  run bash -c '
    BC_FILE="'"${factory_root}"'/.factory/specs/behavioral-contracts/ss-04/BC-4.14.001.md"
    if [ ! -f "$BC_FILE" ]; then echo "FAIL: $BC_FILE does not exist"; exit 1; fi
    HITS=$(awk '"'"'/^## Preconditions$/{ found=1 }
     found && /^## / && !/^## (Preconditions|Postconditions|Invariants|Edge Cases|Error Paths|Canonical Test Vectors)$/{ exit }
     found{ print }'"'"' "$BC_FILE" \
      | grep -Ei "(reads?|loads?|fetches|derives?|access(es)?|retrieves?|opens?|parses?)[[:space:]]+.{0,80}(sprint-state\.yaml|git-log|git-cat-file)" \
      | grep -Eiv "no |not |NOT |without|never|does not|MUST NOT|is NOT|cannot|do NOT|only from|exclusively" \
      | wc -l) || true
    if [ "$HITS" -ne 0 ]; then
      echo "FAIL: BC-4.14.001.md has $HITS verb+substrate collocation hits in normative sections"
      exit 1
    fi
  '
  assert_success
  refute_output --partial "FAIL"
}

# ---------------------------------------------------------------------------
# AC-002 / test_bc_4_15_001_pure_parse_invariant_zero_verb_substrate_hits_normative
#
# Three-layer pipeline on BC-4.15.001.md:
#   Layer 1: awk normative-section extraction (whitelist terminator — stops at first
#            NON-normative heading; robust to ## Related BCs (Recommended) variants)
#   Layer 2: verb+substrate collocation grep
#   Layer 3: negation-cue exclusion grep
# Expected: 0 hits.
#
# Red Gate: BC-4.15.001.md absent → file check fails → echo FAIL + exit 1
#           → assert_success fails.
# ---------------------------------------------------------------------------

@test "test_bc_4_15_001_pure_parse_invariant_zero_verb_substrate_hits_normative" {
  local factory_root="$FACTORY_ROOT"
  run bash -c '
    BC_FILE="'"${factory_root}"'/.factory/specs/behavioral-contracts/ss-04/BC-4.15.001.md"
    if [ ! -f "$BC_FILE" ]; then echo "FAIL: $BC_FILE does not exist"; exit 1; fi
    HITS=$(awk '"'"'/^## Preconditions$/{ found=1 }
     found && /^## / && !/^## (Preconditions|Postconditions|Invariants|Edge Cases|Error Paths|Canonical Test Vectors)$/{ exit }
     found{ print }'"'"' "$BC_FILE" \
      | grep -Ei "(reads?|loads?|fetches|derives?|access(es)?|retrieves?|opens?|parses?)[[:space:]]+.{0,80}(sprint-state\.yaml|git-log|git-cat-file)" \
      | grep -Eiv "no |not |NOT |without|never|does not|MUST NOT|is NOT|cannot|do NOT|only from|exclusively" \
      | wc -l) || true
    if [ "$HITS" -ne 0 ]; then
      echo "FAIL: BC-4.15.001.md has $HITS verb+substrate collocation hits in normative sections"
      exit 1
    fi
  '
  assert_success
  refute_output --partial "FAIL"
}

# ---------------------------------------------------------------------------
# AC-003 / test_all_pure_parse_bcs_dynamic_discovery_zero_verb_substrate_hits
#
# Dynamic discovery: find BC-*.md files whose ## Invariants section contains
# "pure-parse" (case-insensitive). Anchored to ## Invariants to avoid over-matching
# BC-INDEX.md and prose-only mentions (F-P1-001 remediation).
#
# Discovery guard: MUST FAIL if zero files discovered (broken glob protection).
# Scannability guard: MUST FAIL if a discovered BC lacks ## Preconditions (F-P1-002
# remediation — ensures the Layer-1 awk has a normative section to scan rather than
# vacuously passing on 0 output).
# Three-layer scan per discovered file; asserts 0 hits each.
#
# Red Gate: zero files discovered (pre-S-18.01..07 state) → discovery guard fires →
# echo FAIL + exit 1 → assert_success fails.
# ---------------------------------------------------------------------------

@test "test_all_pure_parse_bcs_dynamic_discovery_zero_verb_substrate_hits" {
  local factory_root="$FACTORY_ROOT"
  run bash -c '
    BC_DIR="'"${factory_root}"'/.factory/specs/behavioral-contracts"
    if [ ! -d "$BC_DIR" ]; then
      echo "FAIL: BC directory does not exist: $BC_DIR"
      exit 1
    fi
    # Discovery: files whose ## Invariants section contains "pure-parse" (case-insensitive)
    any_fail=0
    discovered_count=0
    while IFS= read -r -d '"'"''"'"' BC_FILE; do
      MATCH=$(awk '"'"'/^## Invariants$/{ found=1; next } found && /^## /{ exit } found{ print }'"'"' "$BC_FILE" \
        | grep -i "pure-parse") || true
      if [ -z "$MATCH" ]; then continue; fi
      discovered_count=$((discovered_count + 1))
      # Scannability guard: discovered BC must have a ## Preconditions section
      if ! grep -q "^## Preconditions$" "$BC_FILE"; then
        echo "FAIL: discovered BC $BC_FILE has no ## Preconditions section — scannability guard triggered"
        any_fail=1
        continue
      fi
      # Three-layer scan on normative sections
      HITS=$(awk '"'"'/^## Preconditions$/{ found=1 }
     found && /^## / && !/^## (Preconditions|Postconditions|Invariants|Edge Cases|Error Paths|Canonical Test Vectors)$/{ exit }
     found{ print }'"'"' "$BC_FILE" \
        | grep -Ei "(reads?|loads?|fetches|derives?|access(es)?|retrieves?|opens?|parses?)[[:space:]]+.{0,80}(sprint-state\.yaml|git-log|git-cat-file)" \
        | grep -Eiv "no |not |NOT |without|never|does not|MUST NOT|is NOT|cannot|do NOT|only from|exclusively" \
        | wc -l) || true
      if [ "$HITS" -ne 0 ]; then
        echo "FAIL: $BC_FILE has $HITS verb+substrate collocation hits in normative sections"
        any_fail=1
      fi
    done < <(find "$BC_DIR" -name "BC-*.md" -print0)
    # Discovery guard: at least one file must have been discovered
    if [ "$discovered_count" -eq 0 ]; then
      echo "FAIL: no pure-parse BCs discovered — discovery guard triggered"
      exit 1
    fi
    if [ "$any_fail" -ne 0 ]; then exit 1; fi
  '
  assert_success
  refute_output --partial "FAIL"
}

# ---------------------------------------------------------------------------
# AC-004 / test_vp_083_081_091_zero_verb_substrate_hits_whole_file
#
# VP scans: layers 2+3 only (no awk section extraction — VPs are fully normative).
# Substrate set adds factory-artifacts (VPs describe broader execution context).
# Rust/bash comment lines (^\s*//) are stripped by an additional grep -Ev step.
#
# VP files: VP-083.md, VP-081.md, VP-091.md
# Expected: 0 hits per file.
#
# Red Gate: any VP file absent → echo FAIL + any_fail=1 → final exit 1
#           → assert_success fails.
# ---------------------------------------------------------------------------

@test "test_vp_083_081_091_zero_verb_substrate_hits_whole_file" {
  local factory_root="$FACTORY_ROOT"
  run bash -c '
    any_fail=0
    for VP_FILE in \
      "'"${factory_root}"'/.factory/specs/verification-properties/VP-083.md" \
      "'"${factory_root}"'/.factory/specs/verification-properties/VP-081.md" \
      "'"${factory_root}"'/.factory/specs/verification-properties/VP-091.md"; do
      if [ ! -f "$VP_FILE" ]; then
        echo "FAIL: $VP_FILE does not exist"
        any_fail=1
        continue
      fi
      HITS=$(grep -Ei "(reads?|loads?|fetches|derives?|access(es)?|retrieves?|opens?|parses?)[[:space:]]+.{0,80}(sprint-state\.yaml|git-log|git-cat-file|factory-artifacts)" "$VP_FILE" \
        | grep -Eiv "no |not |NOT |without|never|does not|MUST NOT|is NOT|cannot|do NOT|only from|exclusively" \
        | grep -Ev "^[[:space:]]*//" \
        | wc -l) || true
      if [ "$HITS" -ne 0 ]; then
        echo "FAIL: $VP_FILE has $HITS verb+substrate collocation hits"
        any_fail=1
      fi
    done
    if [ "$any_fail" -ne 0 ]; then exit 1; fi
  '
  assert_success
  refute_output --partial "FAIL"
}

# ---------------------------------------------------------------------------
# AC-005 / test_positive_control_genuine_substrate_read_yields_exactly_one_hit
#
# Positive control: the injected sentence
#   "The gate reads wave context directly from sprint-state.yaml before parsing the payload."
# piped through layers 2+3 MUST yield exactly 1 hit.
# If it yields 0 the verb pattern is over-restrictive (silent false-negative regression).
# If it yields >1 there is a pattern duplication bug.
#
# This test MUST remain GREEN at all times. Any refactor of the verb pattern that
# causes this test to fail means ALL other AC results are untrusted until this is GREEN.
# ---------------------------------------------------------------------------

@test "test_positive_control_genuine_substrate_read_yields_exactly_one_hit" {
  run bash -c '
    HITS=$(echo "The gate reads wave context directly from sprint-state.yaml before parsing the payload." \
      | grep -Ei "(reads?|loads?|fetches|derives?|access(es)?|retrieves?|opens?|parses?)[[:space:]]+.{0,80}(sprint-state\.yaml|git-log|git-cat-file)" \
      | grep -Eiv "no |not |NOT |without|never|does not|MUST NOT|is NOT|cannot|do NOT|only from|exclusively" \
      | wc -l) || true
    if [ "$HITS" -ne 1 ]; then
      echo "FAIL: positive control expected exactly 1 hit, got $HITS — verb pattern may be over-restrictive or duplicated"
      exit 1
    fi
  '
  assert_success
  refute_output --partial "FAIL"
}

# ---------------------------------------------------------------------------
# AC-006 / test_positive_control_opens_parses_verbs_detected
#
# Recall-guard positive control: locks the F-P2-001 fix (8-verb expansion).
# The injected sentence
#   "The gate opens sprint-state.yaml and parses git-log output to derive wave context."
# piped through the SAME Layer-2 (8-verb) + Layer-3 logic MUST yield >= 1 hit.
# If it yields 0 the pattern has regressed to the 6-verb set and `opens`/`parses`
# are silently excluded — all AC-001..AC-004 scans would miss these verbs.
#
# This test MUST remain GREEN at all times. A regression to the 6-verb form will
# cause this test to fail with HITS=0, making the regression immediately visible.
# ---------------------------------------------------------------------------

@test "test_positive_control_opens_parses_verbs_detected" {
  run bash -c '
    HITS=$(echo "The gate opens sprint-state.yaml and parses git-log output to derive wave context." \
      | grep -Ei "(reads?|loads?|fetches|derives?|access(es)?|retrieves?|opens?|parses?)[[:space:]]+.{0,80}(sprint-state\.yaml|git-log|git-cat-file)" \
      | grep -Eiv "no |not |NOT |without|never|does not|MUST NOT|is NOT|cannot|do NOT|only from|exclusively" \
      | wc -l) || true
    if [ "$HITS" -lt 1 ]; then
      echo "FAIL: opens/parses recall-guard expected >= 1 hit, got $HITS — verb pattern may have regressed to 6-verb set"
      exit 1
    fi
  '
  assert_success
  refute_output --partial "FAIL"
}
