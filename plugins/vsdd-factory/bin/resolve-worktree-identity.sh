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
# Parse porcelain SPACE-SAFE: each record is one or more lines:
#   worktree <path>
#   HEAD <sha>
#   branch <ref>  OR  detached
# (blank line separates records)
#
# Matching rule (case-insensitive on STORY_ID, basename-only):
#   A worktree matches IFF its BASENAME equals STORY_ID (case-insensitive)
#   OR its BASENAME begins with <STORY_ID>- (case-insensitive, anchored).
#
# Identity is (right directory basename) + (right HEAD SHA), independent of
# branch ref.  A detached-HEAD worktree whose basename and HEAD match IS valid
# and must be accepted — detached-HEAD is NOT a disqualifier.
#
# "ANCHORED" means S-12.08 does NOT match S-12.088.
# ---------------------------------------------------------------------------

STORY_ID_LOWER="$(printf '%s' "$STORY_ID" | tr '[:upper:]' '[:lower:]')"  # POSIX lower-case (3.2-safe)

WORKTREE_ABS_PATH=""
MATCH_COUNT=0

# Parse git worktree list --porcelain
# Reset block variable at the start of each "worktree" line.
_path=""

while IFS= read -r line; do
  if [[ "$line" == worktree\ * ]]; then
    # Start of a new record — use ${line#worktree } to strip prefix SPACE-SAFE
    _path="${line#worktree }"
  elif [[ -z "$line" ]]; then
    # End of record — evaluate basename match (branch/detached state irrelevant)
    if [[ -n "$_path" ]]; then
      _basename="$(basename "$_path")"
      _basename_lower="$(printf '%s' "$_basename" | tr '[:upper:]' '[:lower:]')"

      # Check: worktree basename == STORY_ID (case-insensitive, exact)
      _basename_is_story=0
      if [[ "$_basename_lower" == "$STORY_ID_LOWER" ]]; then
        _basename_is_story=1
      fi

      # Check: worktree basename starts with <STORY_ID>- (case-insensitive, anchored)
      _basename_starts_story=0
      if [[ "$_basename_lower" == "${STORY_ID_LOWER}-"* ]]; then
        _basename_starts_story=1
      fi

      if [[ "$_basename_is_story" -eq 1 || "$_basename_starts_story" -eq 1 ]]; then
        WORKTREE_ABS_PATH="$_path"
        MATCH_COUNT=$((MATCH_COUNT + 1))
      fi
    fi
    _path=""
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
