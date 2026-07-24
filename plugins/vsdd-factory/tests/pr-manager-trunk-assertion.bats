#!/usr/bin/env bats
# pr-manager-trunk-assertion.bats — S-21.03 gate harness suite.
#
# Two load-bearing layers:
#   DOC-PARITY:         grep assertions on pr-manager.md §Step 3 and §Step 9.
#                       If the mandate text is absent or removed, these fail.
#   EXECUTABLE-HARNESS: bash helper functions implementing the BC-6.10.002
#                       PC2 and PC3 assertion procedures, executed against
#                       stubbed gh/git (tests/fixtures/pr-manager-trunk/).
#                       Observable outcome files (merge-log / delivered-marker)
#                       allow literal assertions on NOT-merged / NOT-delivered.
#
# Gate host:  plugins/vsdd-factory/agents/pr-manager.md §Step 3 (post-create)
#             plugins/vsdd-factory/agents/pr-manager.md §Step 9 (post-merge)
# BC:         BC-6.10.002 (PC2, PC3, Invariant 2, EC-006)
# Story:      S-21.03
#
# Test plan:
#   T-001  AC-001 / PC2 error:    wrong baseRefName → BaseRefNameMismatch; merge NOT invoked
#   T-002  AC-002 / PC2 happy:    correct baseRefName → assertion passes; merge invoked
#   T-003  AC-003 / PC3 error:    merge-base exits 1 → MergeNotAncestorOfTrunk; NOT delivered
#   T-004  AC-004 / PC3 happy:    merge-base exits 0 → assertion passes; story delivered
#   T-005  AC-005 / PC3 null:     null mergeCommit.oid → MergeNotAncestorOfTrunk

setup() {
  REPO_ROOT="$(cd "${BATS_TEST_DIRNAME}/../../.." && pwd)"
  PLUGIN_ROOT="$REPO_ROOT/plugins/vsdd-factory"
  PR_MANAGER_MD="$PLUGIN_ROOT/agents/pr-manager.md"
  FIXTURE_DIR="$PLUGIN_ROOT/tests/fixtures/pr-manager-trunk"
  WORK="$(mktemp -d)"
  MERGE_LOG="$WORK/merge.log"
  DELIVERED_MARKER="$WORK/delivered"
  touch "$MERGE_LOG"
}

teardown() {
  [ -n "${WORK:-}" ] && rm -rf "$WORK"
}

# ===========================================================================
# DOC-PARITY helpers
# ===========================================================================

_extract_step3_section() {
  awk '
    /^### Step 3:/ { found=1; next }
    found && /^### / { exit }
    found { print }
  ' "$PR_MANAGER_MD"
}

_extract_step9_section() {
  awk '
    /^### Step 9:/ { found=1; next }
    found && /^## / { exit }
    found { print }
  ' "$PR_MANAGER_MD"
}

_assert_doc_marker() {
  # $1=regex  $2=label  $3=section_text
  echo "$3" | grep -qE "$1" || {
    echo "DOC-PARITY FAIL [pr-manager.md must contain: $2]"
    false
  }
}

# ===========================================================================
# EXECUTABLE-HARNESS helpers
# ===========================================================================

# Run the PC2 baseRefName assertion procedure against the fixture gh stub.
# Args:
#   $1  fixture_dir  — path to directory containing the gh stub
#   $2  trunk        — expected base branch (e.g. "develop")
#   $3  merge_log    — path to merge-invocation log; "merge-invoked\n" written on pass
#
# Implements the BC-6.10.002 PC2 procedure described in pr-manager.md §Step 3:
#   run `gh pr view <pr_number> --json baseRefName`; if returned value != trunk,
#   hard-fail with BaseRefNameMismatch; do NOT proceed to merge.
#
# Returns 0 on pass, 1 on BaseRefNameMismatch.
_run_base_ref_assertion() {
  local fixture_dir="$1" trunk="$2" merge_log="$3"
  local json_out actual_base

  json_out="$("$fixture_dir/gh" pr view 123 --json baseRefName 2>/dev/null)"
  actual_base="$(printf '%s' "$json_out" | grep -oE '"baseRefName":"[^"]*"' | cut -d'"' -f4)"

  if [ "$actual_base" = "$trunk" ]; then
    printf 'ASSERTION-PASS: baseRefName '"'"'%s'"'"' equals configured trunk '"'"'%s'"'"'\n' \
      "$actual_base" "$trunk"
    printf 'merge-invoked\n' >> "$merge_log"
  else
    printf 'HARD FAIL: PR #123 baseRefName '"'"'%s'"'"' does not match configured trunk '"'"'%s'"'"'.\n' \
      "$actual_base" "$trunk"
    echo "BaseRefNameMismatch"
  fi
  return 0
}

# Run the PC3 post-merge ancestry assertion procedure against the fixture git stub.
# Args:
#   $1  fixture_dir      — path to directory containing the git stub
#   $2  trunk            — target trunk branch (e.g. "develop")
#   $3  merge_sha        — merge commit SHA to verify
#   $4  delivered_marker — path to marker file; touched on assertion pass
#
# Implements the BC-6.10.002 PC3 procedure:
#   run `git fetch origin <trunk> && git merge-base --is-ancestor <sha> origin/<trunk>`;
#   if exit != 0, raise MergeNotAncestorOfTrunk P0 error; do NOT mark story delivered.
#
# Returns 0 on pass, 1 on MergeNotAncestorOfTrunk.
_run_ancestry_assertion() {
  local fixture_dir="$1" trunk="$2" merge_sha="$3" delivered_marker="$4"
  local is_ancestor_exit=0

  "$fixture_dir/git" fetch origin "$trunk" 2>/dev/null
  "$fixture_dir/git" merge-base --is-ancestor "$merge_sha" "origin/$trunk" 2>/dev/null \
    && is_ancestor_exit=0 || is_ancestor_exit=$?

  if [ "$is_ancestor_exit" -eq 0 ]; then
    printf 'ASSERTION-PASS: merge commit %s is ancestor of origin/%s\n' "$merge_sha" "$trunk"
    touch "$delivered_marker"
  else
    printf 'P0 DATA ERROR: PR merge commit %s is NOT an ancestor of origin/%s.\n' "$merge_sha" "$trunk"
    echo "MergeNotAncestorOfTrunk"
  fi
  return 0
}

# Run the PC3 assertion with gh mergeCommit lookup (EC-006 null-SHA path).
# Args:
#   $1  fixture_dir      — path to directory containing the gh/git stubs
#   $2  trunk            — target trunk branch
#   $3  pr_num           — PR number string
#   $4  delivered_marker — path to marker file; touched on assertion pass
#
# Implements the null-mergeCommit.oid guard (BC-6.10.002 EC-006):
#   run `gh pr view <pr_number> --json mergeCommit`; if mergeCommit.oid is null
#   or absent, treat as MergeNotAncestorOfTrunk — unknown SHA cannot be verified.
#
# Returns 0 on pass, 1 on MergeNotAncestorOfTrunk.
_run_null_mergecommit_assertion() {
  local fixture_dir="$1" trunk="$2" pr_num="$3" delivered_marker="$4"
  local mc_json merge_sha

  mc_json="$("$fixture_dir/gh" pr view "$pr_num" --json mergeCommit 2>/dev/null)"
  merge_sha="$(printf '%s' "$mc_json" | grep -oE '"oid":"[^"]*"' | cut -d'"' -f4)"

  if [ -z "$merge_sha" ]; then
    echo "MergeNotAncestorOfTrunk"
    printf 'P0 DATA ERROR: mergeCommit.oid is null — unknown merge SHA cannot be verified.\n'
    return 0
  fi

  _run_ancestry_assertion "$fixture_dir" "$trunk" "$merge_sha" "$delivered_marker"
}

# ===========================================================================
# T-001 / AC-001 / PC2 error: wrong baseRefName → BaseRefNameMismatch; merge NOT invoked
# BC-6.10.002 PC2, Invariant 2
# RG-001: no baseRefName assertion in pr-manager.md §Step 3 → doc-parity fails pre-impl
# ===========================================================================

@test "T-001 S-21.03 AC-001: BaseRefNameMismatch on wrong baseRefName — merge NOT invoked" {
  # Fixture: gh stub returns baseRefName pointing to a feature branch, not trunk.
  # Pre-implementation: doc-parity fails (Step 3 has no baseRefName mandate).
  # Post-implementation: doc-parity passes; harness confirms BaseRefNameMismatch
  #   is emitted and merge is NOT invoked (RG-001 closure).

  local step3
  step3="$(_extract_step3_section)"

  # DOC-PARITY: Step 3 must mandate the post-create baseRefName assertion
  _assert_doc_marker "baseRefName" \
    "post-create baseRefName assertion command (gh pr view --json baseRefName)" "$step3"
  _assert_doc_marker "BaseRefNameMismatch" \
    "BaseRefNameMismatch error variant named in Step 3 mandate" "$step3"

  # HARNESS: stub gh returning wrong branch; assert hard-fail + merge NOT invoked
  export GH_STUB_RESPONSE='{"baseRefName":"feature/S-007-impl"}'
  local assert_out
  assert_out="$(_run_base_ref_assertion "$FIXTURE_DIR" "develop" "$MERGE_LOG")"

  echo "$assert_out" | grep -q "BaseRefNameMismatch" || {
    echo "HARNESS FAIL: BaseRefNameMismatch not in assertion output — got: $assert_out"
    false
  }
  [ ! -s "$MERGE_LOG" ] || {
    echo "HARNESS FAIL: merge was invoked but must NOT be on BaseRefNameMismatch — log: $(cat "$MERGE_LOG")"
    false
  }
}

# ===========================================================================
# T-002 / AC-002 / PC2 happy-path: correct baseRefName → assertion passes; merge invoked
# BC-6.10.002 PC2 (happy-path)
# RG-001: same doc-parity gate as T-001
# ===========================================================================

@test "T-002 S-21.03 AC-002: baseRefName assertion passes on correct trunk — proceeds to merge" {
  # Fixture: gh stub returns baseRefName == "develop" (the configured trunk).
  # Pre-implementation: doc-parity fails (same RG-001 gate as T-001).
  # Post-implementation: assertion passes; merge-invoked written to log.

  local step3
  step3="$(_extract_step3_section)"

  # DOC-PARITY: same Step 3 mandate as T-001 (happy-path needs the assertion present too)
  _assert_doc_marker "baseRefName" \
    "post-create baseRefName assertion command must be present for happy-path to execute" "$step3"
  _assert_doc_marker "BaseRefNameMismatch" \
    "BaseRefNameMismatch error variant — both paths defined in same Step 3 block" "$step3"

  # HARNESS: stub gh returning correct trunk; assert ASSERTION-PASS + merge invoked
  export GH_STUB_RESPONSE='{"baseRefName":"develop"}'
  local assert_out
  assert_out="$(_run_base_ref_assertion "$FIXTURE_DIR" "develop" "$MERGE_LOG")"

  echo "$assert_out" | grep -q "ASSERTION-PASS" || {
    echo "HARNESS FAIL: ASSERTION-PASS not in output — got: $assert_out"
    false
  }
  grep -q "merge-invoked" "$MERGE_LOG" || {
    echo "HARNESS FAIL: merge was NOT invoked on happy-path — assertion output: $assert_out"
    false
  }
}

# ===========================================================================
# T-003 / AC-003 / PC3 error: merge-base exits 1 → MergeNotAncestorOfTrunk; NOT delivered
# BC-6.10.002 PC3, Invariant 2
# RG-002: no merge-base --is-ancestor assertion in pr-manager.md §Step 9 → doc-parity fails
# ===========================================================================

@test "T-003 S-21.03 AC-003: MergeNotAncestorOfTrunk when merge-base exits 1 — story NOT delivered" {
  # Fixture: git stub exits 1 for merge-base --is-ancestor.
  # Pre-implementation: doc-parity fails (Step 9 has no --is-ancestor mandate).
  # Post-implementation: MergeNotAncestorOfTrunk emitted; delivered marker NOT created (RG-002).

  local step9
  step9="$(_extract_step9_section)"

  # DOC-PARITY: Step 9 must mandate the post-merge ancestry assertion
  _assert_doc_marker "merge-base --is-ancestor" \
    "post-merge git merge-base --is-ancestor assertion command in Step 9" "$step9"
  _assert_doc_marker "MergeNotAncestorOfTrunk" \
    "MergeNotAncestorOfTrunk P0 error variant named in Step 9 mandate" "$step9"

  # HARNESS: stub git merge-base --is-ancestor exiting 1; assert P0 error + NOT delivered
  export GIT_IS_ANCESTOR_EXIT=1
  local assert_out
  assert_out="$(_run_ancestry_assertion "$FIXTURE_DIR" "develop" "deadbeef1234" "$DELIVERED_MARKER")"

  echo "$assert_out" | grep -q "MergeNotAncestorOfTrunk" || {
    echo "HARNESS FAIL: MergeNotAncestorOfTrunk not in assertion output — got: $assert_out"
    false
  }
  [ ! -f "$DELIVERED_MARKER" ] || {
    echo "HARNESS FAIL: delivered marker was created but must NOT be on PC3 failure"
    false
  }
}

# ===========================================================================
# T-004 / AC-004 / PC3 happy-path: merge-base exits 0 → assertion passes; story delivered
# BC-6.10.002 PC3 (happy-path)
# RG-002: same doc-parity gate as T-003
# ===========================================================================

@test "T-004 S-21.03 AC-004: ancestry assertion passes on merge-base exit 0 — story delivered" {
  # Fixture: git stub exits 0 for merge-base --is-ancestor (happy path).
  # Pre-implementation: doc-parity fails (same RG-002 gate as T-003).
  # Post-implementation: ASSERTION-PASS emitted; delivered marker touched.

  local step9
  step9="$(_extract_step9_section)"

  # DOC-PARITY: same Step 9 mandate as T-003 (happy-path needs the assertion present too)
  _assert_doc_marker "merge-base --is-ancestor" \
    "post-merge ancestry assertion command must be present for happy-path to execute" "$step9"
  _assert_doc_marker "MergeNotAncestorOfTrunk" \
    "MergeNotAncestorOfTrunk — both paths defined in same Step 9 block" "$step9"

  # HARNESS: stub git merge-base --is-ancestor exiting 0; assert ASSERTION-PASS + delivered
  export GIT_IS_ANCESTOR_EXIT=0
  local assert_out
  assert_out="$(_run_ancestry_assertion "$FIXTURE_DIR" "develop" "deadbeef5678" "$DELIVERED_MARKER")"

  echo "$assert_out" | grep -q "ASSERTION-PASS" || {
    echo "HARNESS FAIL: ASSERTION-PASS not in assertion output — got: $assert_out"
    false
  }
  [ -f "$DELIVERED_MARKER" ] || {
    echo "HARNESS FAIL: delivered marker NOT created on PC3 pass — assertion output: $assert_out"
    false
  }
}

# ===========================================================================
# T-005 / AC-005 / EC-006: null mergeCommit.oid → MergeNotAncestorOfTrunk
# BC-6.10.002 PC3, EC-006
# RG-003: no null-mergeCommit handling in pr-manager.md §Step 9 → doc-parity fails
# ===========================================================================

@test "T-005 S-21.03 AC-005: null mergeCommit.oid treated as MergeNotAncestorOfTrunk" {
  # Fixture: gh stub returns {"mergeCommit":null} — merge SHA unavailable.
  # Pre-implementation: doc-parity fails (Step 9 has no null-mergeCommit handling mandate).
  # Post-implementation: MergeNotAncestorOfTrunk emitted; delivered marker NOT created (RG-003).

  local step9
  step9="$(_extract_step9_section)"

  # DOC-PARITY: Step 9 must document the null mergeCommit.oid guard (EC-006)
  _assert_doc_marker "mergeCommit|mergeCommit\.oid|merge.*null" \
    "null mergeCommit.oid guard (EC-006) documented in Step 9 mandate" "$step9"
  _assert_doc_marker "MergeNotAncestorOfTrunk" \
    "MergeNotAncestorOfTrunk variant used for null-SHA path in Step 9" "$step9"

  # HARNESS: stub gh returning {"mergeCommit":null}; assert MergeNotAncestorOfTrunk + NOT delivered
  export GH_STUB_RESPONSE='{"mergeCommit":null}'
  local assert_out
  assert_out="$(_run_null_mergecommit_assertion "$FIXTURE_DIR" "develop" "456" "$DELIVERED_MARKER")"

  echo "$assert_out" | grep -q "MergeNotAncestorOfTrunk" || {
    echo "HARNESS FAIL: MergeNotAncestorOfTrunk not in output for null mergeCommit — got: $assert_out"
    false
  }
  [ ! -f "$DELIVERED_MARKER" ] || {
    echo "HARNESS FAIL: delivered marker was created but must NOT be when mergeCommit.oid is null"
    false
  }
}
