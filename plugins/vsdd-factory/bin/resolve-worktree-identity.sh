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
#   VSDD_REPO_ROOT      — optional override for the main repo root (test fixtures / hooks)
#
# Outputs (on success, to stdout):
#   worktree-abs-path:   <absolute-path-to-worktree>
#   feature-HEAD-SHA:    <sha>
#   story-id:            <story-id>
#   canonical-repo-root: <main-repo-root>
#
# Exit codes:
#   0   — identity resolved and asserted; tuple printed to stdout
#   1   — dispatch-error; error message printed to stderr + exit non-zero

set -euo pipefail

# ---------------------------------------------------------------------------
# Input validation
# ---------------------------------------------------------------------------

if [[ -z "${STORY_ID:-}" ]]; then
  printf 'dispatch-error: STORY_ID is required but not set\n' >&2
  exit 1
fi

if [[ -z "${EXPECTED_HEAD_SHA:-}" ]]; then
  printf 'dispatch-error: EXPECTED_HEAD_SHA is required but not set\n' >&2
  exit 1
fi

# ---------------------------------------------------------------------------
# Step 1: Resolve main repo root nesting-safe via git-common-dir, anchored to
# the script's own directory — NOT the ambient CWD.
#
# RATIONALE (C-1 fix, issues #169 + #176):
#   `git rev-parse --git-common-dir` returns a path RELATIVE TO THE -C TARGET,
#   not relative to the caller's CWD.  Without anchoring, naive
#     "$(git rev-parse --git-common-dir)/.."
#   resolves relative to whatever CWD the caller happened to have, which may be
#   outside the repo entirely, producing the wrong repo root and reading the
#   wrong .factory (the exact #169 regression).
#
#   The fix: use `-C "$_SCRIPT_DIR"` so git resolves the common-dir relative to
#   a directory the script CONTROLS (its own location).  Then cd into
#   `$_SCRIPT_DIR` first, and from there cd into the common-dir result, and
#   finally cd `..` to reach the main repo root.  This works in both a normal
#   checkout (script at <repo>/plugins/vsdd-factory/bin/) and a linked worktree
#   (--git-common-dir from any worktree always points at the main .git/).
#
#   `VSDD_REPO_ROOT` override is kept for test fixtures and explicit hooks that
#   already know the root.  It MUST also be exercised by tests (not just the
#   override path) — see resolve-worktree-identity.bats for the production-path
#   test that does NOT set VSDD_REPO_ROOT.
# ---------------------------------------------------------------------------

_SCRIPT_DIR="$(cd -- "$(dirname -- "$0")" && pwd)"

if [[ -n "${VSDD_REPO_ROOT:-}" ]]; then
  REPO_ROOT="$(cd -- "$VSDD_REPO_ROOT" && pwd)"
else
  # --git-common-dir is relative to the -C target; canonicalize by cd-ing there
  # first, then into the common-dir result, then one level up to the repo root.
  _common="$(git -C "$_SCRIPT_DIR" rev-parse --git-common-dir 2>/dev/null)" || {
    printf 'dispatch-error: git rev-parse --git-common-dir failed (is %s inside a git repo?)\n' "$_SCRIPT_DIR" >&2
    exit 1
  }
  REPO_ROOT="$(cd -- "$_SCRIPT_DIR" && cd -- "$_common" && cd -- .. && pwd)"
fi
CANONICAL_REPO_ROOT="$REPO_ROOT"

# ---------------------------------------------------------------------------
# Step 2: Guard — factory-artifacts must be mounted.
# ---------------------------------------------------------------------------

if [[ ! -d "$CANONICAL_REPO_ROOT/.factory" ]]; then
  printf 'dispatch-error: canonical .factory not mounted — %s/.factory does not exist. STOP.\n' "$CANONICAL_REPO_ROOT" >&2
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

# Capture git output first with an explicit failure check so git's non-zero
# exit code is not swallowed by a process substitution under set -euo pipefail.
if ! _wt_porcelain="$(git -C "$CANONICAL_REPO_ROOT" worktree list --porcelain 2>&1)"; then
  printf 'dispatch-error: git worktree list failed in %s\n' "$CANONICAL_REPO_ROOT" >&2
  exit 1
fi

# Parse porcelain output SPACE-SAFE using a here-document.
# Reset block variable at the start of each "worktree" line.
# A trailing newline on $_wt_porcelain ensures the last record's implicit
# blank-line terminator is present (the here-doc appends a trailing newline).
_path=""

while IFS= read -r line; do
  if [[ "$line" == worktree\ * ]]; then
    # Start of a new record — use ${line#worktree } to strip prefix SPACE-SAFE
    _path="${line#worktree }"
  elif [[ -z "$line" ]]; then
    # End of record — evaluate basename match (branch/detached state irrelevant)
    if [[ -n "$_path" ]]; then
      _basename="$(basename -- "$_path")"
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
done <<EOF
$_wt_porcelain

EOF
# LOAD-BEARING: The blank line between "$_wt_porcelain" and EOF above is intentional
# and mandatory.  git worktree list --porcelain separates records with blank lines but
# does NOT emit a trailing blank line after the LAST record.  Without the explicit
# blank line here, the final record never triggers the "elif [[ -z "$line" ]]" branch
# and is silently dropped.  Removing or collapsing this blank line breaks last-record
# matching (see resolve-worktree-identity.bats:
# test_resolve_wt_identity_matching_worktree_is_LAST_record_resolves — the test that
# exercises the matching worktree as the FINAL porcelain record and MUST fail when
# this blank line is removed).

# Disambiguate
if [[ "$MATCH_COUNT" -eq 0 ]]; then
  printf 'dispatch-error: no worktree matched STORY_ID=%s in git worktree list. STOP.\n' "'$STORY_ID'" >&2
  exit 1
fi

if [[ "$MATCH_COUNT" -gt 1 ]]; then
  printf 'dispatch-error: %d worktrees matched STORY_ID=%s — ambiguous. STOP.\n' "$MATCH_COUNT" "'$STORY_ID'" >&2
  exit 1
fi

# Verify the resolved path is a real directory
if [[ ! -d "$WORKTREE_ABS_PATH" ]]; then
  printf 'dispatch-error: resolved worktree path %s is not a directory. STOP.\n' "'$WORKTREE_ABS_PATH'" >&2
  exit 1
fi

# ---------------------------------------------------------------------------
# Step 4: Assert HEAD SHA equals the orchestrator-recorded expected SHA.
#
# L-1 fix: normalize both SHAs via `git rev-parse` before comparing so that
# a short/abbreviated EXPECTED_HEAD_SHA (e.g. 7-char abbrev) still matches the
# full 40-char actual HEAD.  `git rev-parse <sha>` returns the full 40-char SHA
# for any unambiguous prefix; if EXPECTED_HEAD_SHA is unresolvable (garbage or
# does not exist in this repo), the fallback preserves the original value so the
# mismatch error message still shows the caller-supplied token.
# ---------------------------------------------------------------------------

ACTUAL_HEAD_SHA="$(git -C "$WORKTREE_ABS_PATH" rev-parse HEAD)"
_normalized_expected="$(git -C "$WORKTREE_ABS_PATH" rev-parse "$EXPECTED_HEAD_SHA" 2>/dev/null \
  || printf '%s' "$EXPECTED_HEAD_SHA")"

if [[ "$ACTUAL_HEAD_SHA" != "$_normalized_expected" ]]; then
  printf 'dispatch-error: worktree HEAD %s != expected %s — STOP\n' \
    "$ACTUAL_HEAD_SHA" "$EXPECTED_HEAD_SHA" >&2
  exit 1
fi

# ---------------------------------------------------------------------------
# Step 5: Print the 4-field WORKTREE-IDENTITY TUPLE.
# ---------------------------------------------------------------------------

printf 'worktree-abs-path:   %s\n' "$WORKTREE_ABS_PATH"
printf 'feature-HEAD-SHA:    %s\n' "$ACTUAL_HEAD_SHA"
printf 'story-id:            %s\n' "$STORY_ID"
printf 'canonical-repo-root: %s\n' "$CANONICAL_REPO_ROOT"
