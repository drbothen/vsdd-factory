#!/usr/bin/env bash
# lib/commit-to-artifacts.sh — Atomic git commit to factory-artifacts branch
# Commits HANDOFF.md + wave-state.yaml (when present) in a SINGLE git commit.
# Commit message format: HANDOFF wave-<N> <ISO-timestamp> (exact format, no deviation).
# MUST NOT produce two separate commits (BC-5.41.002 PC6 / AC-017).
# Uses `git -C .factory ...` path discipline per Previous Story Intelligence.
# BC-5.41.001 PC6 + BC-5.41.002 PC6 | S-18.01 scaffold
#
# TODO S-18.01 — implemented in TDD green step
set -euo pipefail

# commit_to_artifacts <factory_artifacts_path> <wave_id> <iso_timestamp> [file...]
# Stages all provided files and creates a single atomic commit on factory-artifacts.
# Commit message: "HANDOFF wave-<wave_id> <iso_timestamp>" (exact format per BC-5.41.001 INV1).
# Returns the new commit SHA via stdout on success.
# Exits non-zero if the push to factory-artifacts fails (AC-EC-008).
# MUST NOT use --no-verify (CLAUDE.md absolute prohibition).
#
# TODO S-18.01 — implemented in TDD green step
commit_to_artifacts() {
  # TODO S-18.01 — implemented in TDD green step
  # Stub: does not create any commit so Red Gate atomicity tests fail.
  echo "TODO S-18.01: commit_to_artifacts not yet implemented" >&2
  return 1
}

# get_handoff_commit_sha <factory_artifacts_path>
# Returns the SHA of the most recent HANDOFF commit on factory-artifacts via stdout.
# Used by write-wave-state.sh to populate generated_from_handoff_sha (BC-5.41.002 PC2).
#
# TODO S-18.01 — implemented in TDD green step
get_handoff_commit_sha() {
  # TODO S-18.01 — implemented in TDD green step
  echo "TODO S-18.01: get_handoff_commit_sha not yet implemented" >&2
  return 1
}
