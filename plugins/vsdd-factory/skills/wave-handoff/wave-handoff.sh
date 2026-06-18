#!/usr/bin/env bash
# wave-handoff.sh — Main entrypoint for the wave-handoff skill
# Writes HANDOFF.md + wave-state.yaml atomically to factory-artifacts at wave close.
# BC-5.41.001 + BC-5.41.002 | S-18.01
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# shellcheck source=lib/parse-sprint-state.sh
source "${SCRIPT_DIR}/lib/parse-sprint-state.sh"
# shellcheck source=lib/write-handoff.sh
source "${SCRIPT_DIR}/lib/write-handoff.sh"
# shellcheck source=lib/write-wave-state.sh
source "${SCRIPT_DIR}/lib/write-wave-state.sh"
# shellcheck source=lib/commit-to-artifacts.sh
source "${SCRIPT_DIR}/lib/commit-to-artifacts.sh"

# ---------------------------------------------------------------------------
# Argument parsing — supports both CLI flags and env var fallbacks
# ---------------------------------------------------------------------------

ARTIFACTS_WT="${ARTIFACTS_WT:-}"
SPRINT_STATE_YAML="${SPRINT_STATE_YAML:-}"
STATE_MD_PATH="${STATE_MD_PATH:-}"
BC_DIR="${BC_DIR:-}"
PRECOMPACT_FLUSH_LOG="${PRECOMPACT_FLUSH_LOG:-}"

while [ $# -gt 0 ]; do
  case "$1" in
    --artifacts-worktree)
      ARTIFACTS_WT="$2"; shift 2 ;;
    --sprint-state)
      SPRINT_STATE_YAML="$2"; shift 2 ;;
    --state-md)
      STATE_MD_PATH="$2"; shift 2 ;;
    --bc-dir)
      BC_DIR="$2"; shift 2 ;;
    --precompact-flush-log)
      PRECOMPACT_FLUSH_LOG="$2"; shift 2 ;;
    *)
      echo "ERROR: unknown argument '$1'" >&2
      exit 1 ;;
  esac
done

: "${ARTIFACTS_WT:?ERROR: --artifacts-worktree or ARTIFACTS_WT is required}"
: "${SPRINT_STATE_YAML:?ERROR: --sprint-state or SPRINT_STATE_YAML is required}"
: "${STATE_MD_PATH:?ERROR: --state-md or STATE_MD_PATH is required}"
: "${BC_DIR:?ERROR: --bc-dir or BC_DIR is required}"

if [ -z "$PRECOMPACT_FLUSH_LOG" ]; then
  # ADR-027 Decision 1: ARTIFACTS_WT is the factory-artifacts worktree root (= .factory
  # in production). The precompact-flush-log lives directly at $ARTIFACTS_WT/hooks/precompact-flush-log,
  # NOT at $ARTIFACTS_WT/.factory/hooks/precompact-flush-log (which would be double-nested
  # and FORBIDDEN by ADR-027 Decision 1). With ARTIFACTS_WT=.factory this resolves correctly
  # to .factory/hooks/precompact-flush-log per SKILL.md and S-18.01 §File Structure table.
  PRECOMPACT_FLUSH_LOG="${ARTIFACTS_WT}/hooks/precompact-flush-log"
fi

# Unset GIT_DIR if set — tests inject GIT_DIR pointing to the fixture repo root,
# but our git -C calls explicitly target the artifacts worktree. An ambient GIT_DIR
# overrides -C and would send git operations to the wrong repository.
unset GIT_DIR 2>/dev/null || true

# ---------------------------------------------------------------------------
# _git_wt — run git command in the artifacts worktree with author identity set
# ---------------------------------------------------------------------------
_git_wt() {
  git -C "$ARTIFACTS_WT" \
    -c user.email="${GIT_AUTHOR_EMAIL:-ci@vsdd-factory}" \
    -c user.name="${GIT_AUTHOR_NAME:-vsdd-factory}" \
    "$@"
}

# ---------------------------------------------------------------------------
# _get_prior_handoff_sha — return the factory-artifacts HEAD SHA captured
# immediately before the atomic commit (per AC-014 v1.4 / BC-5.41.002 PC2).
#
# Operative definition: generated_from_handoff_sha = `git -C ARTIFACTS_WT rev-parse HEAD`
# at the moment the skill is invoked, i.e., the current HEAD of the factory-artifacts
# branch before the atomic commit creates a new HEAD.
#
# "null" is returned for wave 1 (no prior HANDOFF commit exists):
#   Wave 1 is detected by checking the rev-list depth of factory-artifacts HEAD.
#   Depth == 1 means only the orphan-root init commit exists on the branch and no
#   prior HANDOFF wave commit has been made yet → return "null".
#   This is the BC-5.41.002 PC2 "HEAD before commit" semantics: the prior-SHA field
#   in wave-state.yaml records the HEAD that existed before the current wave's commit,
#   not the SHA of the commit we are about to create.
# ---------------------------------------------------------------------------
_get_prior_handoff_sha() {
  # Check if the factory-artifacts branch has any commits at all
  local head_sha
  head_sha="$(git -C "$ARTIFACTS_WT" rev-parse HEAD 2>/dev/null || true)"
  if [ -z "$head_sha" ]; then
    echo "null"
    return 0
  fi

  # Count commits on factory-artifacts to detect orphan-root (wave-1 case).
  # When there is only the init commit (depth=1), treat as wave-1 → null.
  local depth
  depth="$(git -C "$ARTIFACTS_WT" rev-list --count HEAD 2>/dev/null || echo "0")"
  if [ "$depth" -le 1 ]; then
    echo "null"
    return 0
  fi

  # Return the current HEAD SHA — this is the value BEFORE the atomic commit runs.
  # Per BC-5.41.002 PC2 / AC-014 v1.4: captures the prior state of factory-artifacts
  # regardless of whether HEAD points to a HANDOFF commit or any other commit.
  echo "$head_sha"
}

# ---------------------------------------------------------------------------
# _get_epic_id — extract current_cycle from STATE.md frontmatter for EPIC-COMPLETE
# canonical message (BC-5.41.002 PC7 / BC-5.41.001 PC8).
# ---------------------------------------------------------------------------
_get_epic_id() {
  local state_md="$1"
  local epic_id=""
  if [ -f "$state_md" ]; then
    epic_id="$(grep -E '^current_cycle:' "$state_md" | head -1 | awk '{print $2}' | tr -d '"')"
  fi
  echo "${epic_id:-unknown}"
}

# ---------------------------------------------------------------------------
# Main orchestration
# ---------------------------------------------------------------------------

main() {
  # Step 1: Derive wave_id from STATE.md current_step (no phantom current_wave: field)
  local wave_id
  wave_id="$(derive_wave_id "$SPRINT_STATE_YAML" "$STATE_MD_PATH")"

  # Step 2: Classify stories — sets CLASSIFY_RESULT + global arrays (must run in current shell)
  NEXT_WAVE_STORY_IDS=()
  NEXT_WAVE_STORY_STATUSES=()
  BROKEN_STORY_IDS=()
  CLASSIFY_RESULT=""
  classify_stories "$SPRINT_STATE_YAML"
  local classification="$CLASSIFY_RESULT"

  case "$classification" in
    broken-sprint-state)
      echo "BrokenSprintState: stories in non-terminal, non-pending states exist but no next-wave stories are pending/draft. Update sprint-state.yaml to reflect actual story states." >&2
      exit 1
      ;;

    epic-complete)
      # EPIC-COMPLETE: write HANDOFF.md with epic_status: complete. No wave-state.yaml.
      # Per F-008 / BC-5.41.002 PC3: if wave-state.yaml pre-exists on factory-artifacts,
      # remove it so the resulting commit tree has no stale wave-state.yaml.
      write_handoff \
        "${ARTIFACTS_WT}/HANDOFF.md" \
        "$wave_id" \
        "$BC_DIR" \
        "$PRECOMPACT_FLUSH_LOG" \
        "$STATE_MD_PATH" \
        "1"

      # Remove stale wave-state.yaml from the commit tree if it exists (F-008 / AC-012).
      # This staging step must happen BEFORE commit_to_artifacts so the deletion is
      # included in the `git diff --cached` check (EC-015 guard) that commit_to_artifacts
      # performs. commit_to_artifacts only stages the files passed to it (HANDOFF.md);
      # the wave-state.yaml deletion is pre-staged here and persists through the commit.
      if git -C "$ARTIFACTS_WT" ls-files --error-unmatch wave-state.yaml >/dev/null 2>&1; then
        _git_wt rm wave-state.yaml
      elif [ -f "${ARTIFACTS_WT}/wave-state.yaml" ]; then
        # Untracked working-tree copy — just remove it so it won't be staged
        rm -f "${ARTIFACTS_WT}/wave-state.yaml"
      fi

      # Route commit through commit_to_artifacts (single-commit + EC-015 idempotency guard).
      # F-P11-004: consolidates the EC-015 empty-staged-diff guard into ONE place so
      # the EPIC-COMPLETE path and the has-next-wave path share the same guard logic.
      # commit_to_artifacts stages HANDOFF.md (idempotent after pre-staging above),
      # then checks diff --cached --quiet before committing.
      commit_to_artifacts "$ARTIFACTS_WT" "$wave_id" HANDOFF.md > /dev/null

      # Canonical EPIC-COMPLETE stdout message per BC-5.41.002 PC7 / BC-5.41.001 PC8
      local epic_id
      epic_id="$(_get_epic_id "$STATE_MD_PATH")"
      echo "EPIC-COMPLETE: All stories in sprint-state.yaml have reached terminal status."
      echo "Epic ${epic_id} is complete. No wave-state.yaml written for next wave."
      echo "HANDOFF.md committed to factory-artifacts with epic_status: complete."
      exit 0
      ;;

    has-next-wave)
      # Build story pairs array from classified arrays
      local story_pairs=()
      local i
      for i in "${!NEXT_WAVE_STORY_IDS[@]}"; do
        story_pairs+=("${NEXT_WAVE_STORY_IDS[$i]}:${NEXT_WAVE_STORY_STATUSES[$i]}")
      done

      # Step 2b: Pre-flight anti-fabrication validation — BEFORE writing any files.
      # BC-5.41.001 PC4: "If any required field is absent or any anti-fabrication check
      # fails, wave-gate blocks wave close … and does NOT write a partial HANDOFF.md."
      #
      # F-P8-001 fix: validate ALL next_wave_stories IDs against STORY-INDEX.md here,
      # before calling write_handoff in Step 4. The old ordering (write_handoff first,
      # then write_wave_state which performs the check) left a partial HANDOFF.md on disk
      # when write_wave_state exited 1 on AntiFabricationFailed.
      #
      # F-P8-002 is also handled here: if story_pairs is non-empty and STORY-INDEX.md is
      # absent, hard-error (StoryIndexMissing) before writing any file.
      #
      # ADR-027 path discipline: ARTIFACTS_WT is the worktree root; STORY-INDEX lives at
      # $ARTIFACTS_WT/stories/STORY-INDEX.md (no nested .factory/ prefix).
      local preflight_story_index="${ARTIFACTS_WT}/stories/STORY-INDEX.md"
      if [ "${#story_pairs[@]}" -gt 0 ] && [ ! -f "$preflight_story_index" ]; then
        echo "ERROR: StoryIndexMissing — STORY-INDEX.md not found at '${preflight_story_index}'" >&2
        echo "  BC-5.41.002 PC2 precondition 2: STORY-INDEX.md must be current and accessible at wave-close." >&2
        echo "  Cannot perform anti-fabrication cross-check on next_wave_stories without STORY-INDEX.md." >&2
        exit 1
      fi
      local preflight_pair
      for preflight_pair in "${story_pairs[@]}"; do
        local preflight_sid="${preflight_pair%%:*}"
        if [ -f "$preflight_story_index" ]; then
          local preflight_escaped_sid
          preflight_escaped_sid="$(printf '%s' "$preflight_sid" | sed 's/\./\\./g')"
          if ! grep -qE "\| *${preflight_escaped_sid} *\|" "$preflight_story_index"; then
            echo "ERROR: AntiFabricationFailed — story ID '${preflight_sid}' not found in STORY-INDEX.md" >&2
            echo "  Pre-flight validation failed: no file was written." >&2
            exit 1
          fi
        fi
      done

      # Step 3: Find prior HANDOFF commit SHA BEFORE writing any files or committing.
      # This SHA goes into generated_from_handoff_sha in wave-state.yaml (AC-014 v1.4).
      # Must be captured before the atomic commit because the commit will become the new HEAD.
      local prior_handoff_sha
      prior_handoff_sha="$(_get_prior_handoff_sha)"

      # Step 4: Write HANDOFF.md with final content
      write_handoff \
        "${ARTIFACTS_WT}/HANDOFF.md" \
        "$wave_id" \
        "$BC_DIR" \
        "$PRECOMPACT_FLUSH_LOG" \
        "$STATE_MD_PATH" \
        "0" \
        "${story_pairs[@]+"${story_pairs[@]}"}"

      # Step 5: Write wave-state.yaml with final content, using prior_handoff_sha.
      # The content written here is EXACTLY what will be committed — no post-hoc patches.
      # BC-5.41.002 PC2: wave-state.yaml describes the NEXT wave, so its wave_id is
      # the current wave_id + 1 (HANDOFF.md keeps the current/closing wave_id).
      local next_wave_id=$(( wave_id + 1 ))
      write_wave_state \
        "${ARTIFACTS_WT}/wave-state.yaml" \
        "$next_wave_id" \
        "$prior_handoff_sha" \
        "$SPRINT_STATE_YAML" \
        "$ARTIFACTS_WT" \
        "${story_pairs[@]+"${story_pairs[@]}"}"

      # Step 6: Atomic single commit of both files via commit_to_artifacts helper.
      # Both files are staged and committed in one git commit (BC-5.41.002 PC6 / AC-017).
      commit_to_artifacts "$ARTIFACTS_WT" "$wave_id" HANDOFF.md wave-state.yaml > /dev/null

      exit 0
      ;;

    *)
      echo "ERROR: unexpected classification result: $classification" >&2
      exit 1
      ;;
  esac
}

main "$@"
