#!/usr/bin/env bash
# lib/parse-sprint-state.sh — Pure parser for sprint-state.yaml
# Derives wave_id, classifies stories by status (terminal/pending/draft/broken),
# detects EPIC-COMPLETE and BrokenSprintState.
# BC-5.41.001 PC2 + BC-5.41.002 PC3 | S-18.01 scaffold
#
# TODO S-18.01 — implemented in TDD green step
set -euo pipefail

# parse_sprint_state <sprint_state_yaml_path>
# Sets global arrays/vars:
#   WAVE_ID, STORIES_PENDING, STORIES_DRAFT, STORIES_TERMINAL, STORIES_BROKEN
#   IS_EPIC_COMPLETE (0/1), IS_BROKEN_SPRINT_STATE (0/1)
#
# TODO S-18.01 — implemented in TDD green step
parse_sprint_state() {
  # TODO S-18.01 — implemented in TDD green step
  # Stub: does not set any output variables so callers detect missing state.
  echo "TODO S-18.01: parse_sprint_state not yet implemented" >&2
  return 1
}

# derive_wave_id <sprint_state_yaml_path> <state_md_path>
# Returns the integer wave_id via stdout.
# Derived from sprint-state.yaml dependency-order topo-sort ordinal (product)
# OR STATE.md current_step: pass number (engine).
# MUST NOT read any current_wave: field — that field does not exist.
#
# TODO S-18.01 — implemented in TDD green step
derive_wave_id() {
  # TODO S-18.01 — implemented in TDD green step
  echo "TODO S-18.01: derive_wave_id not yet implemented" >&2
  return 1
}

# classify_stories <sprint_state_yaml_path>
# Outputs classification: epic-complete | broken-sprint-state | has-next-wave
#
# TODO S-18.01 — implemented in TDD green step
classify_stories() {
  # TODO S-18.01 — implemented in TDD green step
  echo "TODO S-18.01: classify_stories not yet implemented" >&2
  return 1
}
