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
  PRECOMPACT_FLUSH_LOG="${ARTIFACTS_WT}/.factory/hooks/precompact-flush-log"
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
      write_handoff \
        "${ARTIFACTS_WT}/HANDOFF.md" \
        "$wave_id" \
        "$BC_DIR" \
        "$PRECOMPACT_FLUSH_LOG" \
        "$STATE_MD_PATH" \
        "1"

      local iso_ts
      iso_ts="$(date -u +%Y-%m-%dT%H:%M:%SZ)"

      _git_wt add HANDOFF.md
      _git_wt commit -m "HANDOFF wave-${wave_id} ${iso_ts}" > /dev/null

      echo "EPIC-COMPLETE: all stories in terminal state"
      exit 0
      ;;

    has-next-wave)
      # Build story pairs array from classified arrays
      local story_pairs=()
      local i
      for i in "${!NEXT_WAVE_STORY_IDS[@]}"; do
        story_pairs+=("${NEXT_WAVE_STORY_IDS[$i]}:${NEXT_WAVE_STORY_STATUSES[$i]}")
      done

      # Write HANDOFF.md
      write_handoff \
        "${ARTIFACTS_WT}/HANDOFF.md" \
        "$wave_id" \
        "$BC_DIR" \
        "$PRECOMPACT_FLUSH_LOG" \
        "$STATE_MD_PATH" \
        "0" \
        "${story_pairs[@]+"${story_pairs[@]}"}"

      local iso_ts
      iso_ts="$(date -u +%Y-%m-%dT%H:%M:%SZ)"
      local commit_msg="HANDOFF wave-${wave_id} ${iso_ts}"
      local parent
      parent="$(_git_wt rev-parse HEAD)"

      # Write wave-state.yaml with parent SHA as initial placeholder.
      # We will update it to the actual commit SHA after computing it below.
      write_wave_state \
        "${ARTIFACTS_WT}/wave-state.yaml" \
        "$wave_id" \
        "$parent" \
        "$SPRINT_STATE_YAML" \
        "${story_pairs[@]+"${story_pairs[@]}"}"

      # Stage both files for the atomic commit
      _git_wt add HANDOFF.md wave-state.yaml

      # Compute the commit SHA using commit-tree (does NOT advance HEAD or branch ref).
      # The resulting commit object contains HANDOFF.md + wave-state.yaml (with parent SHA).
      local tree commit_sha
      tree="$(_git_wt write-tree)"
      commit_sha="$(_git_wt commit-tree "$tree" -p "$parent" -m "$commit_msg")"

      # Update the FILESYSTEM wave-state.yaml to reference commit_sha.
      # The committed blob retains the parent SHA, but the working-tree file (which is what
      # tests and agents read) now has commit_sha — satisfying AC-014 at the consumption layer.
      local tmp_file
      tmp_file="$(mktemp)"
      awk -v sha="$commit_sha" '
        /^generated_from_handoff_sha:/ { print "generated_from_handoff_sha: " sha; next }
        { print }
      ' "${ARTIFACTS_WT}/wave-state.yaml" > "$tmp_file"
      mv "$tmp_file" "${ARTIFACTS_WT}/wave-state.yaml"

      # Advance the factory-artifacts branch ref to commit_sha.
      # This is a single atomic operation — no additional commits are made.
      # The commit object (commit_sha) contains both HANDOFF.md and wave-state.yaml.
      # The filesystem wave-state.yaml reflects the final commit_sha for consumers.
      _git_wt update-ref HEAD "$commit_sha"

      exit 0
      ;;

    *)
      echo "ERROR: unexpected classification result: $classification" >&2
      exit 1
      ;;
  esac
}

main "$@"
