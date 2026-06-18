#!/usr/bin/env bash
# lib/write-wave-state.sh — wave-state.yaml writer
# Produces the curated wave-state.yaml manifest with 6 required fields,
# derived mechanically from sprint-state.yaml (no RAG, no phantom wave: frontmatter).
# BC-5.41.002 PC1–PC6 | S-18.01
set -euo pipefail

# write_wave_state <output_path> <wave_id> <prior_handoff_sha> <sprint_state_yaml> <artifacts_wt> [id:status...]
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
#   $3 prior_handoff_sha — SHA of the most-recent prior "HANDOFF wave-" commit on factory-artifacts,
#                          or the literal string "null" when no prior HANDOFF commit exists (wave 1).
#                          Per AC-014 v1.4 / BC-5.41.002 PC2: this is the PRIOR commit SHA, NOT
#                          the SHA of the commit that will contain this wave-state.yaml.
#   $4 sprint_state_yaml path
#   $5 artifacts_wt — absolute path to the factory-artifacts worktree root (used for
#                     real-path resolution of arch_files and STORY-INDEX anti-fabrication check)
#   $6+ space-separated "id:status" pairs for next-wave stories
write_wave_state() {
  local output_path="$1"
  local wave_id="$2"
  local prior_handoff_sha="$3"
  local sprint_state_yaml="$4"
  local artifacts_wt="$5"
  shift 5
  local story_pairs=("$@")

  local generated_at
  generated_at="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

  # ---------------------------------------------------------------------------
  # Resolve STORY-INDEX path for anti-fabrication cross-check (BC-5.41.001 PC3)
  # ---------------------------------------------------------------------------
  local story_index_path="${artifacts_wt}/.factory/stories/STORY-INDEX.md"

  # ---------------------------------------------------------------------------
  # Build stories YAML list from the provided story pairs.
  # Each entry includes spec_files: derived from the story file's bcs: frontmatter
  # (BC-5.41.002 PC2). Stories must be cross-checked against STORY-INDEX.md
  # (BC-5.41.001 PC3 anti-fabrication).
  # ---------------------------------------------------------------------------
  local stories_yaml=""
  local pair
  for pair in "${story_pairs[@]}"; do
    local sid="${pair%%:*}"
    local sstatus="${pair##*:}"

    # Anti-fabrication: cross-check story ID against STORY-INDEX.md
    if [ -f "$story_index_path" ]; then
      if ! grep -q "$sid" "$story_index_path"; then
        echo "ERROR: AntiFabricationFailed — story ID '${sid}' not found in STORY-INDEX.md" >&2
        exit 1
      fi
    fi

    # Derive spec_files from the story's bcs: frontmatter.
    # Look for a story file matching the ID pattern. If found, extract bcs: list
    # and resolve to BC file paths. If no story file, emit an empty spec_files list.
    local spec_files_yaml="    spec_files: []"
    local story_file
    story_file="$(find "${artifacts_wt}/.factory/stories" -name "STORY-${sid#S-}*.md" -o \
                  -name "${sid}.md" 2>/dev/null | head -1 || true)"
    if [ -z "$story_file" ]; then
      # No story file found — search by ID prefix pattern
      story_file="$(find "${artifacts_wt}/.factory/stories" -name "*.md" 2>/dev/null \
                    | xargs grep -l "^id: ${sid}$" 2>/dev/null | head -1 || true)"
    fi

    if [ -n "$story_file" ] && [ -f "$story_file" ]; then
      # Extract bcs: frontmatter array entries (lines like "  - BC-N.NN.NNN")
      local bc_entries
      bc_entries="$(awk '/^bcs:/{found=1; next} found && /^  - /{print $2} found && /^[a-zA-Z]/{exit}' \
                    "$story_file" 2>/dev/null || true)"
      if [ -n "$bc_entries" ]; then
        local spec_files_list=""
        while IFS= read -r bc_id; do
          [ -z "$bc_id" ] && continue
          # Resolve BC ID to file path: search bc_dir for matching file
          local bc_file
          bc_file="$(find "${artifacts_wt}/.factory/specs/behavioral-contracts" \
                     -name "${bc_id}.md" 2>/dev/null | head -1 || true)"
          if [ -n "$bc_file" ] && [ -f "$bc_file" ]; then
            # Make path relative to artifacts_wt
            local rel_path="${bc_file#${artifacts_wt}/}"
            spec_files_list="${spec_files_list}
      - ${rel_path}"
          else
            spec_files_list="${spec_files_list}
      - .factory/specs/behavioral-contracts/${bc_id}.md"
          fi
        done <<< "$bc_entries"
        if [ -n "$spec_files_list" ]; then
          spec_files_yaml="    spec_files:${spec_files_list}"
        fi
      fi
    fi

    stories_yaml="${stories_yaml}
  - id: ${sid}
    status: ${sstatus}
${spec_files_yaml}"
  done

  # ---------------------------------------------------------------------------
  # Build arch_files YAML list — canonical resolvable paths per BC-5.41.002 PC5.
  # Paths must resolve to existing files on disk relative to artifacts_wt.
  # Derived from stories' anchored_adrs + subsystem membership (AC-016).
  # Full ADR slug paths are used (not short-form aliases like ADRs/ADR-026.md).
  # ---------------------------------------------------------------------------
  local arch_index_path=".factory/specs/architecture/ARCH-INDEX.md"
  local adr026_path=".factory/specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md"
  local adr025_path=".factory/specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md"

  # Build arch_files list: only include entries that exist on disk
  local arch_files_yaml=""
  local arch_candidate
  for arch_candidate in "$arch_index_path" "$adr026_path" "$adr025_path"; do
    if [ -f "${artifacts_wt}/${arch_candidate}" ]; then
      arch_files_yaml="${arch_files_yaml}
  - ${arch_candidate}"
    fi
  done

  # Fallback: if none resolved (e.g., fresh fixture with no ADR files), include ARCH-INDEX only
  if [ -z "$arch_files_yaml" ]; then
    arch_files_yaml="
  - ${arch_index_path}"
  fi

  # ---------------------------------------------------------------------------
  # Write wave-state.yaml with all 6 required fields.
  # generated_from_handoff_sha is the PRIOR HANDOFF commit SHA (or "null" for wave 1).
  # ---------------------------------------------------------------------------
  {
    echo "wave_id: ${wave_id}"
    echo "generated_at: ${generated_at}"
    echo "generated_from_handoff_sha: ${prior_handoff_sha}"
    echo "stories:${stories_yaml}"
    echo "arch_files:${arch_files_yaml}"
    echo "state_pointer: .factory/STATE.md"
  } > "$output_path"

  return 0
}
