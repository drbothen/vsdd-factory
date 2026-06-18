#!/usr/bin/env bash
# lib/write-wave-state.sh — wave-state.yaml writer
# Produces the curated wave-state.yaml manifest with 6 required fields,
# derived mechanically from sprint-state.yaml (no RAG, no phantom wave: frontmatter).
# BC-5.41.002 PC1–PC6 | S-18.01 scaffold
#
# TODO S-18.01 — implemented in TDD green step
set -euo pipefail

# write_wave_state <output_path> <wave_id> <handoff_sha> <sprint_state_yaml> <story_index>
# Writes wave-state.yaml to <output_path> with 6 required fields:
#   wave_id, generated_at, generated_from_handoff_sha, stories, arch_files, state_pointer
# stories list MUST be derived exclusively from sprint-state.yaml status:pending/draft
# entries sorted by dependency-graph topological order.
# MUST NOT use RAG, phantom wave: frontmatter, or any source other than sprint-state.yaml
# (BC-5.41.002 PC3 INV3 INV4).
# Not called on EPIC-COMPLETE waves (BC-5.41.002 PC3 / AC-012).
#
# TODO S-18.01 — implemented in TDD green step
write_wave_state() {
  # TODO S-18.01 — implemented in TDD green step
  # Stub: does not write wave-state.yaml so Red Gate tests fail on file-existence assertions.
  echo "TODO S-18.01: write_wave_state not yet implemented" >&2
  return 1
}

# derive_stories_list <sprint_state_yaml> <story_index>
# Returns YAML list of {id, status, spec_files} objects via stdout.
# Source: sprint-state.yaml entries with status:pending or status:draft,
# ordered by STORY-INDEX.md depends_on dependency graph topological sort.
# Empty result = HARD ERROR (no EPIC-COMPLETE exception here — caller handles EPIC-COMPLETE).
#
# TODO S-18.01 — implemented in TDD green step
derive_stories_list() {
  # TODO S-18.01 — implemented in TDD green step
  echo "TODO S-18.01: derive_stories_list not yet implemented" >&2
  return 1
}

# derive_arch_files <stories_list>
# Returns minimum arch_files list (always includes ARCH-INDEX.md + ADR-026 + ADR-025
# + any ADRs directly referenced by stories).
# BC-5.41.002 PC5.
#
# TODO S-18.01 — implemented in TDD green step
derive_arch_files() {
  # TODO S-18.01 — implemented in TDD green step
  echo "TODO S-18.01: derive_arch_files not yet implemented" >&2
  return 1
}
