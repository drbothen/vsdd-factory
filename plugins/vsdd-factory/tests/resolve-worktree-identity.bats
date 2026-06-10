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
  wt_088="$WT_PATH"
  sha_08="$(git -C "$wt_08" rev-parse HEAD)"

  run env STORY_ID="S-12.08" EXPECTED_HEAD_SHA="$sha_08" VSDD_REPO_ROOT="$MAIN_REPO" \
    bash "$HELPER"

  [ "$status" -eq 0 ]
  # The resolved worktree-abs-path must be EXACTLY the S-12.08 path
  [[ "$output" == *"worktree-abs-path:   $wt_08"* ]]
  # S-12.088 path must NOT appear anywhere in the output
  [[ "$output" != *"${wt_088}"* ]]
  # Extra: the path we got must end with /S-12.08 (not /S-12.088)
  resolved="$(echo "$output" | grep "^worktree-abs-path:" | sed 's/^worktree-abs-path:[[:space:]]*//')"
  [[ "$resolved" == */S-12.08 ]]
  [[ "$resolved" != */S-12.088* ]]
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
# Test 5: detached-HEAD worktree with matching basename + SHA → ACCEPTED
#
# Identity is (right directory basename) + (right HEAD SHA), independent of
# branch ref.  A detached-HEAD worktree whose basename equals the story-id
# and whose HEAD matches the expected SHA MUST be accepted (resolves to success).
# This test MUST FAIL if the implementation skips detached records.
#
# Additionally: a detached worktree with matching basename but WRONG SHA must
# still produce a dispatch-error (the SHA assertion is mandatory regardless of
# branch state).
# ---------------------------------------------------------------------------

@test "test_resolve_wt_identity_detached_head_matching_basename_and_sha_is_accepted" {
  # Create a detached worktree whose basename == S-12.08 (matching basename rule)
  git -C "$MAIN_REPO" worktree add --detach "$WORK/S-12.08" >/dev/null 2>&1
  sha="$(git -C "$WORK/S-12.08" rev-parse HEAD)"

  run env STORY_ID="S-12.08" EXPECTED_HEAD_SHA="$sha" VSDD_REPO_ROOT="$MAIN_REPO" \
    bash "$HELPER"

  # Must succeed — detached-HEAD is NOT a disqualifier
  [ "$status" -eq 0 ]
  [[ "$output" == *"worktree-abs-path:   $WORK/S-12.08"* ]]
  [[ "$output" == *"feature-HEAD-SHA:    $sha"* ]]
}

@test "test_resolve_wt_identity_detached_head_matching_basename_wrong_sha_fails" {
  # Detached worktree with correct basename but wrong SHA must still be rejected
  git -C "$MAIN_REPO" worktree add --detach "$WORK/S-12.08" >/dev/null 2>&1
  wrong_sha="bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb"

  run env STORY_ID="S-12.08" EXPECTED_HEAD_SHA="$wrong_sha" VSDD_REPO_ROOT="$MAIN_REPO" \
    bash "$HELPER"

  [ "$status" -ne 0 ]
  [[ "$output" == *"dispatch-error"* ]]
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

# ---------------------------------------------------------------------------
# Test 9 (C-1 production-path): helper resolves REPO_ROOT WITHOUT VSDD_REPO_ROOT,
# invoked from a CWD OUTSIDE the worktree.
#
# This test MUST FAIL against the old CWD-relative code:
#   REPO_ROOT="$(cd -- "$(git rev-parse --git-common-dir)/.." && pwd)"
# When the CWD is /tmp (outside the repo), `git rev-parse --git-common-dir`
# fails (not in a git repo) → the helper would error or resolve the wrong root.
# With the C-1 fix the helper anchors to its own _SCRIPT_DIR, so CWD is irrelevant.
#
# This test exercises the PRODUCTION branch (no VSDD_REPO_ROOT override) and
# asserts that CANONICAL_REPO_ROOT resolves to the MAIN_REPO, not a tempdir.
# ---------------------------------------------------------------------------

@test "test_resolve_wt_identity_production_path_no_VSDD_REPO_ROOT_resolves_correct_root" {
  # C-1 production-path test: the helper MUST resolve the repo root via its
  # own _SCRIPT_DIR anchor, NOT via ambient CWD.  To exercise this:
  #   1. Place a COPY of the helper inside the test repo (so _SCRIPT_DIR points
  #      at the test repo, not the real vsdd-factory repo).
  #   2. Run WITHOUT VSDD_REPO_ROOT.
  #   3. CWD is an entirely different tmpdir (outside the test repo).
  #
  # If the old CWD-relative code were used:
  #   git rev-parse --git-common-dir (run from outside_dir) would either fail
  #   (not in a git repo) or resolve the wrong repo — the test would exit non-zero.
  # With the C-1 fix:
  #   git -C "$_SCRIPT_DIR" rev-parse --git-common-dir anchors to the test repo
  #   regardless of CWD — the test exits 0 with the correct canonical-repo-root.

  _make_worktree "S-12.08"
  sha="$(git -C "$WT_PATH" rev-parse HEAD)"

  # Install a copy of the helper inside the test repo at a bin/ path
  mkdir -p "$MAIN_REPO/bin"
  cp "$HELPER" "$MAIN_REPO/bin/resolve-worktree-identity.sh"
  chmod +x "$MAIN_REPO/bin/resolve-worktree-identity.sh"
  local_helper="$MAIN_REPO/bin/resolve-worktree-identity.sh"

  # Stage the copy so git is aware (not strictly necessary for the helper to run,
  # but keeps the repo clean for teardown)
  git -C "$MAIN_REPO" add "$local_helper" >/dev/null 2>&1 || true

  # Outside directory — entirely different from both the test repo and the helper
  outside_dir="$(mktemp -d)"
  outside_dir="$(cd "$outside_dir" && pwd -P)"

  # Run WITHOUT VSDD_REPO_ROOT — production path; CWD is outside_dir
  run bash -c "cd '$outside_dir' && STORY_ID='S-12.08' EXPECTED_HEAD_SHA='$sha' bash '$local_helper'"

  rm -rf "$outside_dir"

  # Must succeed — _SCRIPT_DIR is inside MAIN_REPO so git anchors correctly
  [ "$status" -eq 0 ]
  [[ "$output" == *"worktree-abs-path:"* ]]
  [[ "$output" == *"canonical-repo-root:"* ]]

  # canonical-repo-root in the output must equal MAIN_REPO
  resolved_root="$(printf '%s\n' "$output" | grep '^canonical-repo-root:' | sed 's/^canonical-repo-root:[[:space:]]*//')"
  [ "$resolved_root" = "$MAIN_REPO" ]
}

# ---------------------------------------------------------------------------
# Test 10 (M-1 non-final-record): matching worktree is NOT the last record
# in git worktree list --porcelain — still resolves correctly.
#
# This test MUST FAIL if the trailing-blank-line in the heredoc is removed:
# without it, the LAST porcelain record never triggers the blank-line branch
# and is dropped.  But here the MATCHING worktree is the FIRST (non-last)
# record, and a non-matching worktree follows it — so both orderings are
# covered: the matching worktree fires during the loop (non-final), and the
# trailing blank ensures the LAST record (non-matching) also gets evaluated
# (proving the load-bearing blank handles the final record correctly too).
# ---------------------------------------------------------------------------

@test "test_resolve_wt_identity_non_final_record_still_resolves_correctly" {
  # Create the MATCHING worktree FIRST (S-12.08)
  _make_worktree "S-12.08"
  wt_target="$WT_PATH"
  sha="$(git -C "$wt_target" rev-parse HEAD)"

  # Then create a LATER (non-matching) worktree — it comes AFTER S-12.08 in
  # git worktree list output, making S-12.08 a non-final record.
  _make_worktree "S-99.99"

  run env STORY_ID="S-12.08" EXPECTED_HEAD_SHA="$sha" VSDD_REPO_ROOT="$MAIN_REPO" \
    bash "$HELPER"

  # The matching worktree (non-final record) must still resolve
  [ "$status" -eq 0 ]
  [[ "$output" == *"worktree-abs-path:   $wt_target"* ]]
  [[ "$output" == *"story-id:            S-12.08"* ]]
  # The non-matching worktree must not appear
  [[ "$output" != *"S-99.99"* ]]
}

# ---------------------------------------------------------------------------
# Test 11 (M-2 last-record): matching worktree IS the LAST record in git
# worktree list --porcelain — still resolves correctly.
#
# This test MUST FAIL if the trailing blank line in the heredoc is removed:
# without it, the LAST porcelain record never triggers the blank-line branch
# and is silently dropped, so the matching worktree (being last) is never
# evaluated and MATCH_COUNT stays 0 → exit non-zero.
#
# git worktree list --porcelain lists in creation order with the main worktree
# first.  To guarantee S-12.08 is the LAST record, create the non-matching
# worktrees (S-99.01, S-99.02) FIRST, then create S-12.08 last.
#
# Non-vacuousness proof: temporarily remove the trailing blank line from the
# heredoc in resolve-worktree-identity.sh and run this test — it goes RED.
# Restore the blank line and the test returns GREEN.
# ---------------------------------------------------------------------------

@test "test_resolve_wt_identity_matching_worktree_is_LAST_record_resolves" {
  # Create the MATCHING worktree as the ONLY additional worktree beyond the main
  # repo — this guarantees it is the FINAL record in git worktree list --porcelain
  # output and that no preceding non-matching worktree provides a "free" inter-
  # record blank line for the last record.
  #
  # git worktree list --porcelain format:
  #   worktree <main-repo>    <- record 1 (main)
  #   HEAD ...
  #   branch ...
  #   <blank>
  #   worktree <S-12.08>      <- record 2 (LAST, no trailing blank from git)
  #   HEAD ...
  #   branch ...
  #   <-- NO trailing blank from git -->
  #
  # The load-bearing blank line in the heredoc is the ONLY terminator for this
  # last record.  Without it the blank-line branch is never triggered and
  # MATCH_COUNT stays 0, causing the helper to exit non-zero.
  _make_worktree "S-12.08"
  wt_target="$WT_PATH"
  sha="$(git -C "$wt_target" rev-parse HEAD)"

  run env STORY_ID="S-12.08" EXPECTED_HEAD_SHA="$sha" VSDD_REPO_ROOT="$MAIN_REPO" \
    bash "$HELPER"

  # The matching worktree (FINAL record, sole non-main worktree) must resolve
  [ "$status" -eq 0 ]
  [[ "$output" == *"worktree-abs-path:   $wt_target"* ]]
  [[ "$output" == *"story-id:            S-12.08"* ]]
}

# ---------------------------------------------------------------------------
# Test 12 (L-1 short-SHA): abbreviated EXPECTED_HEAD_SHA still matches full SHA.
#
# The orchestrator typically records the full 40-char SHA, but abbreviated SHAs
# (7-char git abbrev) should also work.  Without normalization, the comparison
# "abcdef1" != "abcdef1234567890..." fails even though they name the same commit.
# ---------------------------------------------------------------------------

@test "test_resolve_wt_identity_short_sha_matches_full_sha" {
  _make_worktree "S-12.08"
  full_sha="$(git -C "$WT_PATH" rev-parse HEAD)"
  # Use a 7-char abbreviated SHA
  short_sha="${full_sha:0:7}"

  run env STORY_ID="S-12.08" EXPECTED_HEAD_SHA="$short_sha" VSDD_REPO_ROOT="$MAIN_REPO" \
    bash "$HELPER"

  # Must succeed — short SHA names the same commit
  [ "$status" -eq 0 ]
  [[ "$output" == *"feature-HEAD-SHA:    $full_sha"* ]]
}
