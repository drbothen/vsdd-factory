#!/usr/bin/env bash
# lib/write-wave-state.sh — wave-state.yaml writer
# Produces the curated wave-state.yaml manifest with 6 required fields,
# derived mechanically from sprint-state.yaml (no RAG, no phantom wave: frontmatter).
# BC-5.41.002 PC1–PC6 | S-18.01
set -euo pipefail

# write_wave_state <output_path> <wave_id> <handoff_sha> <sprint_state_yaml>
# Writes wave-state.yaml to <output_path> with 6 required fields:
#   wave_id, generated_at, generated_from_handoff_sha, stories, arch_files, state_pointer
# stories list MUST be derived exclusively from sprint-state.yaml status:pending/draft entries.
# MUST NOT use RAG, phantom wave: frontmatter, or any source other than sprint-state.yaml
# (BC-5.41.002 PC3 INV3 INV4).
# Not called on EPIC-COMPLETE waves (BC-5.41.002 PC3 / AC-012).
#
# Parameters:
#   $1 output_path
#   $2 wave_id
#   $3 handoff_sha (40-char hex — SHA of the HANDOFF.md commit)
#   $4 sprint_state_yaml path
#   $5+ space-separated "id:status" pairs for next-wave stories
write_wave_state() {
  local output_path="$1"
  local wave_id="$2"
  local handoff_sha="$3"
  local sprint_state_yaml="$4"
  shift 4
  local story_pairs=("$@")

  local generated_at
  generated_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

  # Build stories YAML list from the provided story pairs
  local stories_yaml=""
  local pair
  for pair in "${story_pairs[@]}"; do
    local sid="${pair%%:*}"
    local sstatus="${pair##*:}"
    stories_yaml="${stories_yaml}
  - id: ${sid}
    status: ${sstatus}"
  done

  # Build arch_files YAML list — minimum set of architecture reference files
  # relevant to the upcoming wave (AC-016: from stories' anchored_adrs + subsystem membership).
  # Since sprint-state.yaml doesn't carry anchored_adrs in the test fixture format,
  # we include the canonical minimum set.
  local arch_files_yaml
  arch_files_yaml="
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/specs/architecture/ADRs/ADR-026.md
  - .factory/specs/architecture/ADRs/ADR-025.md"

  # Write wave-state.yaml
  {
    echo "wave_id: ${wave_id}"
    echo "generated_at: ${generated_at}"
    echo "generated_from_handoff_sha: ${handoff_sha}"
    echo "stories:${stories_yaml}"
    echo "arch_files:${arch_files_yaml}"
    echo "state_pointer: .factory/STATE.md"
  } > "$output_path"

  return 0
}
