#!/usr/bin/env bats
# resolve-worktree-identity.bats — EXECUTION tests for the
# plugins/vsdd-factory/bin/resolve-worktree-identity.sh helper.
#
# ROOT INSIGHT (Finding 6, issues #169+#176): The static grep-only suite cannot
# catch broken shell logic — which is why the awk bugs (Findings 1+2) slipped in.
# These tests exercise the REAL helper against a real git worktree, proving:
#   1. Space-safe path resolution (naive awk $2 truncates at space)
#   2. Anchored S-12.08 vs S-12.088 disambiguation (naive unanchored match fails)
#   3. SHA assertion
#   4. Detached-HEAD filtering
#   5. Missing .factory guard
#
# Tests that MUST FAIL against a naive awk-$2 / unanchored implementation:
#   - test_resolve_wt_identity_space_in_path (truncated path → dir not found → non-zero)
#   - test_resolve_wt_identity_S1208_vs_S12088_no_superstring (wrong worktree selected)
#
# Run:
#   bats plugins/vsdd-factory/tests/resolve-worktree-identity.bats

HELPER="$(cd "$(dirname "$BATS_TEST_FILENAME")/.." && pwd)/bin/resolve-worktree-identity.sh"

# ---------------------------------------------------------------------------
# setup: build a fresh git repo + worktree for each test.
# Each test runs in its own WORK tempdir to ensure isolation.
#
# Tests pass VSDD_REPO_ROOT="$MAIN_REPO" to the helper so it can locate the
# git common directory without requiring a `cd` into the repo.
# ---------------------------------------------------------------------------

setup() {
  WORK="$(mktemp -d)"
  # Resolve all symlinks in WORK so path comparisons work on macOS where
  # mktemp returns /var/folders/... but git resolves to /private/var/folders/...
  WORK="$(cd "$WORK" && pwd -P)"

  # Main repo with .factory (required by the factory-artifacts guard in the helper)
  git init "$WORK/main-repo" >/dev/null 2>&1
  git -C "$WORK/main-repo" config user.email "test@vsdd-factory.test"
  git -C "$WORK/main-repo" config user.name "VSDD Test"
  mkdir -p "$WORK/main-repo/.factory"
  echo "factory-root" > "$WORK/main-repo/.factory/state.md"
  git -C "$WORK/main-repo" add .
  git -C "$WORK/main-repo" commit -m "init" >/dev/null 2>&1

  MAIN_REPO="$WORK/main-repo"
  INIT_SHA="$(git -C "$MAIN_REPO" rev-parse HEAD)"
}

teardown() {
  # Prune worktrees first (otherwise git complains about locked worktrees on rm)
  git -C "$MAIN_REPO" worktree prune 2>/dev/null || true
  rm -rf "$WORK"
}

# ---------------------------------------------------------------------------
# Helpers
# ---------------------------------------------------------------------------

# Create a standard worktree for STORY_ID at $WORK/<story_id>.
# Basename equals story_id — matches the engine convention (.worktrees/<STORY-ID>/)
# and satisfies the adversary.md Rule 2 basename check.
# Returns the path via the WT_PATH variable (avoids echo/stdout contamination).
_make_worktree() {
  local story_id="$1"
  WT_PATH="$WORK/${story_id}"
  git -C "$MAIN_REPO" worktree add -b "feature/${story_id}" "$WT_PATH" >/dev/null 2>&1
}

# Same but with a space in the PARENT directory — proves space-safe path parsing.
# The basename is still the story_id (satisfies adversary.md Rule 2); the space
# is in the parent segment to stress IFS-split / awk-$2 naive implementations.
# Returns the path via WT_PATH.
_make_worktree_with_space() {
  local story_id="$1"
  mkdir -p "$WORK/my worktrees"
  WT_PATH="$WORK/my worktrees/${story_id}"
  git -C "$MAIN_REPO" worktree add -b "feature/${story_id}" "$WT_PATH" >/dev/null 2>&1
}

# ---------------------------------------------------------------------------
# Test 1: happy path — correct worktree + matching SHA → all 4 tuple fields, exit 0
# ---------------------------------------------------------------------------

@test "test_resolve_wt_identity_happy_path_prints_all_4_tuple_fields_exit0" {
  _make_worktree "S-12.08"
  wt_path="$WT_PATH"
  sha="$(git -C "$wt_path" rev-parse HEAD)"

  run env STORY_ID="S-12.08" EXPECTED_HEAD_SHA="$sha" VSDD_REPO_ROOT="$MAIN_REPO" \
    bash "$HELPER"

  [ "$status" -eq 0 ]
  [[ "$output" == *"worktree-abs-path:"* ]]
  [[ "$output" == *"feature-HEAD-SHA:"* ]]
  [[ "$output" == *"story-id:"* ]]
  [[ "$output" == *"canonical-repo-root:"* ]]
  # The printed story-id must match what we passed
  [[ "$output" == *"story-id:            S-12.08"* ]]
  # The printed SHA must be the one we expected
  [[ "$output" == *"feature-HEAD-SHA:    $sha"* ]]
}

# ---------------------------------------------------------------------------
# Test 2: SHA mismatch → dispatch-error, non-zero exit
# ---------------------------------------------------------------------------

@test "test_resolve_wt_identity_sha_mismatch_emits_dispatch_error_nonzero" {
  _make_worktree "S-12.08"
  # Deliberately wrong SHA
  wrong_sha="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"

  run env STORY_ID="S-12.08" EXPECTED_HEAD_SHA="$wrong_sha" VSDD_REPO_ROOT="$MAIN_REPO" \
    bash "$HELPER"

  [ "$status" -ne 0 ]
  [[ "$output" == *"dispatch-error"* ]]
  [[ "$output" == *"!="* ]]
}

# ---------------------------------------------------------------------------
# Test 3: S-12.08 vs S-12.088 disambiguation
#
# This test MUST FAIL against naive awk-$2 / unanchored regex implementations.
# The naïve awk command:
#   awk '/^worktree /{path=$2} /^branch /{branch=$2} branch ~ "S-12.08" {print path}
# matches BOTH S-12.08 and S-12.088 because the regex is unanchored.
# The real helper must select ONLY S-12.08.
# ---------------------------------------------------------------------------

@test "test_resolve_wt_identity_S1208_vs_S12088_no_superstring_selects_exact_match" {
  # Create BOTH worktrees — the helper must pick only S-12.08
  _make_worktree "S-12.08"
  wt_08="$WT_PATH"
  _make_worktree "S-12.088"
  sha_08="$(git -C "$wt_08" rev-parse HEAD)"

  run env STORY_ID="S-12.08" EXPECTED_HEAD_SHA="$sha_08" VSDD_REPO_ROOT="$MAIN_REPO" \
    bash "$HELPER"

  [ "$status" -eq 0 ]
  # Must print the S-12.08 path, not the S-12.088 path
  [[ "$output" == *"worktree-abs-path:   $wt_08"* ]]
  # S-12.088 path must NOT appear in output
  [[ "$output" != *"wt-S-12.088"* ]]
}

# When ONLY S-12.088 exists, resolving S-12.08 must fail (no false match).
@test "test_resolve_wt_identity_S1208_with_only_S12088_present_fails_nonzero" {
  _make_worktree "S-12.088"
  sha_088="$(git -C "$WT_PATH" rev-parse HEAD)"

  run env STORY_ID="S-12.08" EXPECTED_HEAD_SHA="$sha_088" VSDD_REPO_ROOT="$MAIN_REPO" \
    bash "$HELPER"

  [ "$status" -ne 0 ]
  [[ "$output" == *"dispatch-error"* ]]
}

# ---------------------------------------------------------------------------
# Test 4: worktree path containing a SPACE resolves correctly (not truncated)
#
# This test MUST FAIL against naive awk-$2 which splits on whitespace:
#   awk '/^worktree /{path=$2}' — $2 stops at the first space in the path.
# ---------------------------------------------------------------------------

@test "test_resolve_wt_identity_space_in_path_resolves_correctly_not_truncated" {
  _make_worktree_with_space "S-12.08"
  wt_path="$WT_PATH"
  sha="$(git -C "$wt_path" rev-parse HEAD)"

  run env STORY_ID="S-12.08" EXPECTED_HEAD_SHA="$sha" VSDD_REPO_ROOT="$MAIN_REPO" \
    bash "$HELPER"

  [ "$status" -eq 0 ]
  # The full path including space must appear in the tuple output
  [[ "$output" == *"worktree-abs-path:   ${wt_path}"* ]]
  # Stronger: the resolved path must be a real directory
  resolved="$(echo "$output" | grep "^worktree-abs-path:" | sed 's/^worktree-abs-path:[[:space:]]*//')"
  [ -d "$resolved" ]
}

# ---------------------------------------------------------------------------
# Test 5: detached-HEAD worktree present → not falsely matched
#
# A detached HEAD worktree record in porcelain has "detached" instead of
# "branch <ref>". The helper must skip it — not match it against STORY_ID.
# ---------------------------------------------------------------------------

@test "test_resolve_wt_identity_detached_head_worktree_not_falsely_matched" {
  # Create a normal worktree for the story we want
  _make_worktree "S-12.08"
  wt_path="$WT_PATH"
  sha="$(git -C "$wt_path" rev-parse HEAD)"

  # Also create a detached worktree — its basename happens to contain the story ID
  # to maximally stress the matcher
  git -C "$MAIN_REPO" worktree add --detach "$WORK/wt-detached-S-12.08-extra" >/dev/null 2>&1

  run env STORY_ID="S-12.08" EXPECTED_HEAD_SHA="$sha" VSDD_REPO_ROOT="$MAIN_REPO" \
    bash "$HELPER"

  [ "$status" -eq 0 ]
  # Must resolve to the branched worktree, not the detached one
  [[ "$output" == *"worktree-abs-path:   $wt_path"* ]]
}

# ---------------------------------------------------------------------------
# Test 6: missing .factory mount → dispatch-error, non-zero exit
# ---------------------------------------------------------------------------

@test "test_resolve_wt_identity_missing_factory_mount_emits_dispatch_error" {
  # Create a repo WITHOUT .factory
  git init "$WORK/repo-no-factory" >/dev/null 2>&1
  git -C "$WORK/repo-no-factory" config user.email "t@t.test"
  git -C "$WORK/repo-no-factory" config user.name "T"
  echo "readme" > "$WORK/repo-no-factory/README.md"
  git -C "$WORK/repo-no-factory" add .
  git -C "$WORK/repo-no-factory" commit -m "init" >/dev/null 2>&1

  git -C "$WORK/repo-no-factory" worktree add \
    -b "feature/S-12.08" "$WORK/S-12.08" >/dev/null 2>&1
  sha="$(git -C "$WORK/S-12.08" rev-parse HEAD)"

  run env STORY_ID="S-12.08" EXPECTED_HEAD_SHA="$sha" \
    VSDD_REPO_ROOT="$WORK/repo-no-factory" \
    bash "$HELPER"

  [ "$status" -ne 0 ]
  [[ "$output" == *"dispatch-error"* ]]
  [[ "$output" == *".factory"* ]]
}

# ---------------------------------------------------------------------------
# Test 7: STORY_ID not set → dispatch-error, non-zero exit
# ---------------------------------------------------------------------------

@test "test_resolve_wt_identity_missing_STORY_ID_env_var_fails_nonzero" {
  _make_worktree "S-12.08"
  sha="$(git -C "$WT_PATH" rev-parse HEAD)"

  run env EXPECTED_HEAD_SHA="$sha" VSDD_REPO_ROOT="$MAIN_REPO" bash "$HELPER"

  [ "$status" -ne 0 ]
  [[ "$output" == *"dispatch-error"* ]]
  [[ "$output" == *"STORY_ID"* ]]
}

# ---------------------------------------------------------------------------
# Test 8: EXPECTED_HEAD_SHA not set → dispatch-error, non-zero exit
# ---------------------------------------------------------------------------

@test "test_resolve_wt_identity_missing_EXPECTED_HEAD_SHA_env_var_fails_nonzero" {
  _make_worktree "S-12.08"

  run env STORY_ID="S-12.08" VSDD_REPO_ROOT="$MAIN_REPO" bash "$HELPER"

  [ "$status" -ne 0 ]
  [[ "$output" == *"dispatch-error"* ]]
  [[ "$output" == *"EXPECTED_HEAD_SHA"* ]]
}
