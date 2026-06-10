#!/usr/bin/env bash
# resolve-worktree-identity.sh — resolve + assert the 4-field worktree-identity tuple
# for per-story adversary dispatch (issues #169 + #176).
#
# Usage:
#   STORY_ID=S-12.08 EXPECTED_HEAD_SHA=<sha> \
#     plugins/vsdd-factory/bin/resolve-worktree-identity.sh
#
# Inputs (environment variables):
#   STORY_ID            — the story identifier, e.g. S-12.08 (required)
#   EXPECTED_HEAD_SHA   — the orchestrator-recorded implementer-final-commit SHA (required)
#
# Outputs (on success, to stdout):
#   worktree-abs-path:   <absolute-path-to-worktree>
#   feature-HEAD-SHA:    <sha>
#   story-id:            <story-id>
#   canonical-repo-root: <main-repo-root>
#
# Exit codes:
#   0   — identity resolved and asserted; tuple printed to stdout
#   1   — dispatch-error; error message printed to stdout

set -euo pipefail

# ---------------------------------------------------------------------------
# Input validation
# ---------------------------------------------------------------------------

if [[ -z "${STORY_ID:-}" ]]; then
  echo "dispatch-error: STORY_ID is required but not set"
  exit 1
fi

if [[ -z "${EXPECTED_HEAD_SHA:-}" ]]; then
  echo "dispatch-error: EXPECTED_HEAD_SHA is required but not set"
  exit 1
fi

# ---------------------------------------------------------------------------
# Step 1: Resolve main repo root nesting-safe via git-common-dir.
# git rev-parse --show-toplevel returns the WORKTREE root when run from inside
# a linked worktree; --git-common-dir always points to the main .git/ directory,
# so "$(git rev-parse --git-common-dir)/.." is always the main checkout root.
#
# VSDD_REPO_ROOT may be set by callers (e.g. test fixtures) to override the
# auto-detected root without requiring the caller to cd into the repo.
# ---------------------------------------------------------------------------

if [[ -n "${VSDD_REPO_ROOT:-}" ]]; then
  REPO_ROOT="$(cd "$VSDD_REPO_ROOT" && pwd)"
else
  REPO_ROOT="$(cd "$(git rev-parse --git-common-dir)/.." && pwd)"
fi
CANONICAL_REPO_ROOT="$REPO_ROOT"

# ---------------------------------------------------------------------------
# Step 2: Guard — factory-artifacts must be mounted.
# ---------------------------------------------------------------------------

if [[ ! -d "$CANONICAL_REPO_ROOT/.factory" ]]; then
  echo "dispatch-error: canonical .factory not mounted — $CANONICAL_REPO_ROOT/.factory does not exist. STOP."
  exit 1
fi

# ---------------------------------------------------------------------------
# Step 3: Resolve the story's worktree from git worktree list --porcelain.
# Parse porcelain SPACE-SAFE: each record is three lines:
#   worktree <path>
#   HEAD <sha>
#   branch <ref>  OR  detached
# (blank line separates records)
#
# Matching rules (case-insensitive on STORY_ID):
#   - Branch ends with /<STORY_ID> exactly (ANCHORED)
#   - OR worktree basename equals STORY_ID (ANCHORED, case-insensitive)
#   - OR worktree basename starts with <STORY_ID>- (ANCHORED, case-insensitive)
#
# "ANCHORED" means S-12.08 does NOT match S-12.088.
# ---------------------------------------------------------------------------

STORY_ID_LOWER="$(printf '%s' "$STORY_ID" | tr '[:upper:]' '[:lower:]')"  # POSIX lower-case (3.2-safe)

WORKTREE_ABS_PATH=""
MATCH_COUNT=0

# Parse git worktree list --porcelain
# Reset block variables at the start of each "worktree" line.
_path=""
_branch=""
_detached=0

while IFS= read -r line; do
  if [[ "$line" == worktree\ * ]]; then
    # Start of a new record — use ${line#worktree } to strip prefix SPACE-SAFE
    _path="${line#worktree }"
    _branch=""
    _detached=0
  elif [[ "$line" == branch\ * ]]; then
    _branch="${line#branch }"
  elif [[ "$line" == "detached" ]]; then
    _detached=1
  elif [[ -z "$line" ]]; then
    # End of record — evaluate match
    if [[ "$_detached" -eq 0 && -n "$_path" && -n "$_branch" ]]; then
      # Anchored branch-end match: branch == refs/heads/feature/<STORY_ID> (case-insensitive)
      _branch_lower="$(printf '%s' "$_branch" | tr '[:upper:]' '[:lower:]')"
      _basename="$(basename "$_path")"
      _basename_lower="$(printf '%s' "$_basename" | tr '[:upper:]' '[:lower:]')"

      _branch_suffix_exact="${_branch_lower##*/}"  # everything after last /
      _story_lower="${STORY_ID_LOWER}"

      # Check: branch ends with /<STORY_ID> exactly
      _branch_ends_with_story=0
      if [[ "$_branch_suffix_exact" == "$_story_lower" ]]; then
        _branch_ends_with_story=1
      fi

      # Check: worktree basename == STORY_ID (case-insensitive)
      _basename_is_story=0
      if [[ "$_basename_lower" == "$_story_lower" ]]; then
        _basename_is_story=1
      fi

      # Check: worktree basename starts with <STORY_ID>- (case-insensitive, anchored)
      _basename_starts_story=0
      if [[ "$_basename_lower" == "${_story_lower}-"* ]]; then
        _basename_starts_story=1
      fi

      # A candidate matches if any of the three conditions fire AND the basename
      # satisfies the basename rule (== story-id OR starts with <story-id>-).
      # This keeps helper and adversary.md Rule 2 aligned: the emitted
      # worktree-abs-path MUST have a basename that passes the same check the
      # adversary enforces.  A worktree whose branch ends with /<STORY_ID> but
      # whose basename is something like "wt-S-12.08" would be resolved by the
      # old helper yet REJECTED by the adversary → false dispatch-error halt.
      _candidate_matched=0
      if [[ "$_branch_ends_with_story" -eq 1 || "$_basename_is_story" -eq 1 || "$_basename_starts_story" -eq 1 ]]; then
        # Enforce basename rule: basename must equal story-id OR start with <story-id>-
        if [[ "$_basename_is_story" -eq 1 || "$_basename_starts_story" -eq 1 ]]; then
          _candidate_matched=1
        fi
      fi

      if [[ "$_candidate_matched" -eq 1 ]]; then
        WORKTREE_ABS_PATH="$_path"
        MATCH_COUNT=$((MATCH_COUNT + 1))
      fi
    fi
    _path=""
    _branch=""
    _detached=0
  fi
done < <(git -C "$CANONICAL_REPO_ROOT" worktree list --porcelain; echo "")
# The extra echo "" ensures the last record's blank-line terminator is present.

# Disambiguate
if [[ "$MATCH_COUNT" -eq 0 ]]; then
  echo "dispatch-error: no worktree matched STORY_ID='$STORY_ID' in git worktree list. STOP."
  exit 1
fi

if [[ "$MATCH_COUNT" -gt 1 ]]; then
  echo "dispatch-error: $MATCH_COUNT worktrees matched STORY_ID='$STORY_ID' — ambiguous. STOP."
  exit 1
fi

# Verify the resolved path is a real directory
if [[ ! -d "$WORKTREE_ABS_PATH" ]]; then
  echo "dispatch-error: resolved worktree path '$WORKTREE_ABS_PATH' is not a directory. STOP."
  exit 1
fi

# ---------------------------------------------------------------------------
# Step 4: Assert HEAD SHA equals the orchestrator-recorded expected SHA.
# ---------------------------------------------------------------------------

ACTUAL_HEAD_SHA="$(git -C "$WORKTREE_ABS_PATH" rev-parse HEAD)"

if [[ "$ACTUAL_HEAD_SHA" != "$EXPECTED_HEAD_SHA" ]]; then
  echo "dispatch-error: worktree HEAD $ACTUAL_HEAD_SHA != expected $EXPECTED_HEAD_SHA — STOP"
  exit 1
fi

# ---------------------------------------------------------------------------
# Step 5: Print the 4-field WORKTREE-IDENTITY TUPLE.
# ---------------------------------------------------------------------------

echo "worktree-abs-path:   $WORKTREE_ABS_PATH"
echo "feature-HEAD-SHA:    $ACTUAL_HEAD_SHA"
echo "story-id:            $STORY_ID"
echo "canonical-repo-root: $CANONICAL_REPO_ROOT"
