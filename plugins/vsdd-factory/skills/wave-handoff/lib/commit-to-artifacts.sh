#!/usr/bin/env bash
# lib/commit-to-artifacts.sh — Atomic git commit to factory-artifacts branch
# Commits HANDOFF.md + wave-state.yaml (when present) in a SINGLE git commit.
# Commit message format: HANDOFF wave-<N> <ISO-timestamp> (exact format, no deviation).
# MUST NOT produce two separate commits (BC-5.41.002 PC6 / AC-017).
# Uses `git -C <artifacts_path>` path discipline per Previous Story Intelligence.
# BC-5.41.001 PC6 + BC-5.41.002 PC6 | S-18.01
set -euo pipefail

# commit_to_artifacts <artifacts_worktree_path> <wave_id> [file_to_add...]
# Stages all provided files relative to artifacts_worktree_path and creates
# a single atomic commit on the factory-artifacts branch.
# Commit message: "HANDOFF wave-<wave_id> <ISO-timestamp>" (exact format per BC-5.41.001 INV1).
# Returns the new commit SHA via stdout on success.
# MUST NOT use --no-verify (CLAUDE.md absolute prohibition).
commit_to_artifacts() {
  local artifacts_wt="$1"
  local wave_id="$2"
  shift 2
  local files_to_add=("$@")

  local iso_ts
  iso_ts="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

  local commit_msg="HANDOFF wave-${wave_id} ${iso_ts}"

  # Stage the named output files (HANDOFF.md, wave-state.yaml).
  # Do NOT use git add -A — that would stage unrelated dirty files (AC-017 violation).
  local f
  for f in "${files_to_add[@]}"; do
    git -C "$artifacts_wt" add -- "$f"
  done

  # Also stage pre-existing factory content (.factory/ directory).
  # In production the factory-artifacts branch already has .factory/ tracked, so this
  # is a no-op. In hermetic test fixtures the directory is created but not yet committed;
  # staging it here keeps the worktree clean after the commit (VP-087 / F-003) without
  # allowing random unrelated root-level files (like unrelated.txt) into the tree.
  if [ -d "${artifacts_wt}/.factory" ]; then
    git -C "$artifacts_wt" add -- .factory/ 2>/dev/null || true
  fi

  # Create a single atomic commit with the exact message format
  git -C "$artifacts_wt" \
    -c user.email="${GIT_AUTHOR_EMAIL:-ci@vsdd-factory}" \
    -c user.name="${GIT_AUTHOR_NAME:-vsdd-factory}" \
    commit -m "$commit_msg"

  # Return the new HEAD SHA
  git -C "$artifacts_wt" rev-parse HEAD
  return 0
}
