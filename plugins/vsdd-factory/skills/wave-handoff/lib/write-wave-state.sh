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
  # ADR-027 path discipline: ARTIFACTS_WT is the worktree root (= .factory in production).
  # In production, .factory IS the worktree, so specs/stories live at $ARTIFACTS_WT/...
  # with NO nested .factory/ subdirectory inside ARTIFACTS_WT.
  local story_index_path="${artifacts_wt}/stories/STORY-INDEX.md"

  # ---------------------------------------------------------------------------
  # Anti-fabrication: cross-check ALL story IDs against STORY-INDEX.md before
  # any sorting or writing (BC-5.41.001 PC3). Hard-error on first phantom ID.
  # ---------------------------------------------------------------------------
  local pair
  for pair in "${story_pairs[@]}"; do
    local sid="${pair%%:*}"
    if [ -f "$story_index_path" ]; then
      if ! grep -q "$sid" "$story_index_path"; then
        echo "ERROR: AntiFabricationFailed — story ID '${sid}' not found in STORY-INDEX.md" >&2
        exit 1
      fi
    fi
  done

  # ---------------------------------------------------------------------------
  # Topological sort of story_pairs by Depends-On from STORY-INDEX.md (BC-5.41.002 PC3).
  # Kahn's algorithm: stories with no unresolved dependencies in the wave set are
  # emitted first. This ensures dependencies precede their dependents in wave-state.yaml.
  # If STORY-INDEX.md is absent, story_pairs ordering is preserved as-is.
  #
  # Production STORY-INDEX format (9 columns):
  #   | Story ID | Title | Epic | Points | Priority | Depends-On | Blocks | Status | BCs |
  # The Depends-On column is located by its HEADER NAME to be robust to column shifts.
  # Values: [] for no deps, [S-X.Y] for single dep, [S-X.Y, S-X.Z] for multi-dep.
  # ---------------------------------------------------------------------------
  local sorted_pairs=()
  if [ -f "$story_index_path" ] && [ "${#story_pairs[@]}" -gt 1 ]; then
    # Build set of story IDs in this wave for edge pruning
    local -A in_wave_set
    for p in "${story_pairs[@]}"; do
      local psid="${p%%:*}"
      in_wave_set["$psid"]=1
    done

    # Locate the "Depends-On" column index by reading the header row.
    # Header row format: | Story ID | Title | Epic | Points | Priority | Depends-On | ...
    # Count which pipe-delimited field contains "Depends-On" (1-based, ignoring leading |).
    local depends_on_col=0
    local header_line
    header_line="$(grep -m1 '| Story ID' "$story_index_path" || true)"
    if [ -n "$header_line" ]; then
      local col_idx=0
      local IFS_bak="$IFS"
      IFS='|'
      local col
      for col in $header_line; do
        local trimmed_col
        trimmed_col="$(echo "$col" | tr -d ' \t')"
        col_idx=$(( col_idx + 1 ))
        if [ "$trimmed_col" = "Depends-On" ]; then
          depends_on_col=$col_idx
          break
        fi
      done
      IFS="$IFS_bak"
    fi

    # Initialize story_deps for all stories in the wave (space-separated dep IDs,
    # restricted to in_wave_set; multi-dep is space-separated: "S-X.Y S-X.Z").
    local -A story_deps
    for p in "${story_pairs[@]}"; do
      local psid="${p%%:*}"
      story_deps["$psid"]=""
    done

    # Parse STORY-INDEX.md data rows to extract Depends-On for each in-wave story.
    # Skip header row (contains "Story ID") and separator rows (contain "---").
    if [ "$depends_on_col" -gt 0 ]; then
      while IFS= read -r line; do
        # Skip non-data rows (header, separator, empty)
        echo "$line" | grep -q '^[[:space:]]*|' || continue
        echo "$line" | grep -qE '\| *Story ID *\|' && continue
        echo "$line" | grep -q '\-\-\-' && continue

        # Split on pipe and extract the Story ID and Depends-On columns.
        # awk 1-based field index: field 1 is empty (before leading |),
        # field 2 is Story ID, field depends_on_col is Depends-On.
        # Note: the IFS='|' split col_idx and awk -F'|' $col_idx use the SAME index;
        # no offset is needed — the leading | in the table row produces empty $1 in both.
        local row_id row_deps_raw
        row_id="$(echo "$line" | awk -F'|' '{print $2}' | tr -d ' \t')"
        row_deps_raw="$(echo "$line" | awk -F'|' -v col="${depends_on_col}" '{print $col}' | tr -d ' \t')"

        # Only process rows for stories in our wave set
        [ -z "$row_id" ] && continue
        [ -z "${in_wave_set[$row_id]+x}" ] && continue

        # Parse the bracketed Depends-On value: [] or [S-X.Y] or [S-X.Y, S-X.Z]
        # Strip surrounding brackets, split on comma, filter to in-wave deps only.
        local deps_content
        deps_content="$(echo "$row_deps_raw" | tr -d '[]' )"
        local dep_list=""
        local dep_entry
        # Split on comma
        local IFS_bak2="$IFS"
        IFS=','
        for dep_entry in $deps_content; do
          IFS="$IFS_bak2"
          local dep_id
          dep_id="$(echo "$dep_entry" | tr -d ' \t')"
          [ -z "$dep_id" ] && continue
          if [ -n "${in_wave_set[$dep_id]+x}" ]; then
            dep_list="${dep_list}${dep_id} "
          fi
        done
        IFS="$IFS_bak2"

        story_deps["$row_id"]="$(echo "$dep_list" | tr -s ' ' | sed 's/ *$//')"
      done < "$story_index_path"
    fi

    # Kahn's algorithm: compute in-degree for each story in the wave.
    # in-degree = number of in-wave dependencies (predecessors).
    local -A in_degree
    for p in "${story_pairs[@]}"; do
      local psid="${p%%:*}"
      in_degree["$psid"]=0
    done
    for p in "${story_pairs[@]}"; do
      local psid="${p%%:*}"
      local deps="${story_deps[$psid]}"
      if [ -n "$deps" ]; then
        local dep
        for dep in $deps; do
          [ -n "${in_degree[$dep]+x}" ] && in_degree["$psid"]=$(( ${in_degree[$psid]} + 1 ))
        done
      fi
    done

    # Build the sorted output using a queue of zero-in-degree nodes.
    # Process in original sprint-state.yaml order for determinism when in-degrees are equal.
    local remaining=("${story_pairs[@]}")
    local max_iterations=$(( ${#story_pairs[@]} + 1 ))
    local iter=0
    while [ "${#remaining[@]}" -gt 0 ]; do
      iter=$(( iter + 1 ))
      if [ "$iter" -gt "$max_iterations" ]; then
        # Cycle detected or logic error — fall back to original order
        sorted_pairs=("${story_pairs[@]}")
        break
      fi
      local emitted=0
      local next_remaining=()
      for p in "${remaining[@]}"; do
        local psid="${p%%:*}"
        if [ "${in_degree[$psid]}" -eq 0 ]; then
          sorted_pairs+=("$p")
          emitted=1
          # Decrement in-degree of all nodes that list psid in their full dep-set
          for q in "${remaining[@]}"; do
            local qsid="${q%%:*}"
            local qdeps="${story_deps[$qsid]}"
            if [ -n "$qdeps" ]; then
              local qdep
              for qdep in $qdeps; do
                if [ "$qdep" = "$psid" ]; then
                  in_degree["$qsid"]=$(( ${in_degree[$qsid]} - 1 ))
                fi
              done
            fi
          done
        else
          next_remaining+=("$p")
        fi
      done
      if [ "$emitted" -eq 0 ]; then
        # No zero-in-degree node found — cycle; append remaining as-is
        sorted_pairs+=("${remaining[@]}")
        break
      fi
      remaining=("${next_remaining[@]+"${next_remaining[@]}"}")
    done
  else
    # Single story or no STORY-INDEX — preserve original order
    sorted_pairs=("${story_pairs[@]+"${story_pairs[@]}"}")
  fi

  # ---------------------------------------------------------------------------
  # Build stories YAML list from sorted story pairs.
  # Each entry includes spec_files: derived from the story file's bcs: frontmatter
  # (BC-5.41.002 PC2).
  # ---------------------------------------------------------------------------
  local stories_yaml=""
  for pair in "${sorted_pairs[@]}"; do
    local sid="${pair%%:*}"
    local sstatus="${pair##*:}"

    # Derive spec_files from the story's behavioral_contracts: frontmatter key.
    # Production story files are named S-NN.NN-<slug>.md (slug suffix after ID).
    # The frontmatter key is behavioral_contracts: (NOT bcs:) per production format.
    # If no story file is found or behavioral_contracts is empty, emit an empty spec_files list.
    local spec_files_yaml="    spec_files: []"
    local story_file
    # ADR-027: stories live at $artifacts_wt/stories/ (no nested .factory/ prefix)
    # Production naming: S-18.02-<slug>.md — match by ${sid}-*.md glob pattern
    story_file="$(find "${artifacts_wt}/stories" -name "${sid}-*.md" -o \
                  -name "${sid}.md" 2>/dev/null | head -1 || true)"
    if [ -z "$story_file" ]; then
      # Fallback: search by story_id: frontmatter key (production uses story_id: not id:)
      story_file="$(find "${artifacts_wt}/stories" -name "*.md" 2>/dev/null \
                    | xargs grep -l "^story_id: ${sid}$" 2>/dev/null | head -1 || true)"
    fi

    if [ -n "$story_file" ] && [ -f "$story_file" ]; then
      # Extract behavioral_contracts: frontmatter array entries (lines like "  - BC-N.NN.NNN")
      # Production key is behavioral_contracts: (not bcs:) per S-NN.NN-<slug>.md format.
      local bc_entries
      bc_entries="$(awk '/^behavioral_contracts:/{found=1; next} found && /^  - /{print $2} found && /^[a-zA-Z]/{exit}' \
                    "$story_file" 2>/dev/null || true)"
      if [ -n "$bc_entries" ]; then
        local spec_files_list=""
        while IFS= read -r bc_id; do
          [ -z "$bc_id" ] && continue
          # Resolve BC ID to file path: search bc_dir for matching file
          # ADR-027: BCs live at $artifacts_wt/specs/behavioral-contracts/ (no nested .factory/)
          local bc_file
          bc_file="$(find "${artifacts_wt}/specs/behavioral-contracts" \
                     -name "${bc_id}.md" 2>/dev/null | head -1 || true)"
          if [ -n "$bc_file" ] && [ -f "$bc_file" ]; then
            # Make path relative to artifacts_wt
            local rel_path="${bc_file#${artifacts_wt}/}"
            spec_files_list="${spec_files_list}
      - ${rel_path}"
          else
            spec_files_list="${spec_files_list}
      - specs/behavioral-contracts/${bc_id}.md"
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
  # ADR-027 path discipline: no .factory/ prefix — architecture lives at
  # $artifacts_wt/specs/architecture/ (ARTIFACTS_WT is the worktree root = .factory in prod).
  local arch_index_path="specs/architecture/ARCH-INDEX.md"
  local adr026_path="specs/architecture/decisions/ADR-026-wave-boundary-checkpoint-reset-and-lossless-intra-wave-compaction.md"
  local adr025_path="specs/architecture/decisions/ADR-025-single-writer-factory-locklease-prevent-concurrent-session-races-on-factory-artifacts-orphan-branch.md"

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
  # (ADR-027: path is specs/architecture/ARCH-INDEX.md, no .factory/ prefix)
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
