#!/usr/bin/env bash
# lib/write-handoff.sh — HANDOFF.md writer
# Gathers all 9 base required fields with anti-fabrication cross-checks and
# writes HANDOFF.md to the factory-artifacts working tree.
# BC-5.41.001 PC1–PC9 | S-18.01 scaffold
#
# TODO S-18.01 — implemented in TDD green step
set -euo pipefail

# write_handoff <output_path> <wave_id> <sprint_state_yaml> <state_md>
# Writes HANDOFF.md to <output_path> with all 9 base required fields:
#   wave_id, last_verified_develop_sha, active_bcs, next_wave_stories,
#   open_decisions, pending_fixes, process_gaps, precompact_flush_sha,
#   factory_lock_holder
# On EPIC-COMPLETE wave: additionally writes epic_status: complete.
# On non-final wave: epic_status field MUST be absent (not null — absent).
# Anti-fabrication cross-checks run before any write (BC-5.41.001 PC3).
# Hard errors if any required field is absent or any cross-check fails (BC-5.41.001 PC4).
#
# TODO S-18.01 — implemented in TDD green step
write_handoff() {
  # TODO S-18.01 — implemented in TDD green step
  # Stub: does not write HANDOFF.md so Red Gate tests fail on file-existence assertions.
  echo "TODO S-18.01: write_handoff not yet implemented" >&2
  return 1
}

# get_last_verified_develop_sha
# Returns 40-char lowercase hex SHA via stdout from `git rev-parse origin/develop`.
# MUST NOT be hardcoded or derived from cache (BC-5.41.001 INV4).
#
# TODO S-18.01 — implemented in TDD green step
get_last_verified_develop_sha() {
  # TODO S-18.01 — implemented in TDD green step
  echo "TODO S-18.01: get_last_verified_develop_sha not yet implemented" >&2
  return 1
}

# get_precompact_flush_sha <flush_log_path>
# Implements the three-state rule (BC-5.41.001 PC5):
#   1. Log genuinely absent: returns "null"
#   2. Log present but FIELD-4 != "commit": returns "null" (corrupt/stale)
#   3. Log present + FIELD-4 == "commit": returns FIELD-2 SHA (hard blocks on mismatch)
#
# TODO S-18.01 — implemented in TDD green step
get_precompact_flush_sha() {
  # TODO S-18.01 — implemented in TDD green step
  echo "TODO S-18.01: get_precompact_flush_sha not yet implemented" >&2
  return 1
}

# check_active_bcs <bc_dir>
# Returns a non-empty YAML list of active BC file paths.
# Hard errors (exit 1) if no active BCs can be determined (BC-5.41.001 PC2 / AC-004).
#
# TODO S-18.01 — implemented in TDD green step
check_active_bcs() {
  # TODO S-18.01 — implemented in TDD green step
  echo "TODO S-18.01: check_active_bcs not yet implemented" >&2
  return 1
}
