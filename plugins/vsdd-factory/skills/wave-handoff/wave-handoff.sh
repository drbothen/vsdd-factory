#!/usr/bin/env bash
# wave-handoff.sh — Main entrypoint for the wave-handoff skill
# Writes HANDOFF.md + wave-state.yaml atomically to factory-artifacts at wave close.
# BC-5.41.001 + BC-5.41.002 | S-18.01
set -euo pipefail

# Require bash 4+ for associative arrays (declare -A / local -A in write-wave-state.sh).
# macOS ships bash 3.2; Homebrew bash 5 is required. Install with: brew install bash
if [ "${BASH_VERSINFO[0]:-0}" -lt 4 ]; then
  echo "ERROR: wave-handoff requires bash >= 4.0 (associative arrays)." >&2
  echo "  Current bash: ${BASH_VERSION:-unknown}" >&2
  echo "  On macOS: install Homebrew bash with 'brew install bash' and ensure it is on PATH" >&2
  echo "  before /bin/bash (e.g., export PATH=\"\$(brew --prefix bash)/bin:\$PATH\")" >&2
  exit 1
fi

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
# Subcommand mode: --emit-handoff | --emit-wave-state | --commit | "" (legacy monolithic)
SUBCOMMAND="${SUBCOMMAND:-}"

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
    --emit-handoff)
      SUBCOMMAND="--emit-handoff"; shift ;;
    --emit-wave-state)
      SUBCOMMAND="--emit-wave-state"; shift ;;
    --commit)
      SUBCOMMAND="--commit"; shift ;;
    *)
      echo "ERROR: unknown argument '$1'" >&2
      exit 1 ;;
  esac
done

: "${ARTIFACTS_WT:?ERROR: --artifacts-worktree or ARTIFACTS_WT is required}"
: "${SPRINT_STATE_YAML:?ERROR: --sprint-state or SPRINT_STATE_YAML is required}"
: "${STATE_MD_PATH:?ERROR: --state-md or STATE_MD_PATH is required}"
: "${BC_DIR:?ERROR: --bc-dir or BC_DIR is required}"

# Validate ARTIFACTS_WT is an accessible git worktree (CWE-73 explicit guard).
# Canonicalization via GNU realpath -e (Linux only) — BSD realpath on macOS resolves
# /var → /private/var which breaks the relative-path stripping in write-handoff.sh.
# We intentionally skip symlink resolution on platforms where it causes path drift.
if realpath --version >/dev/null 2>&1; then
  # GNU realpath is available (-e checks existence; exits non-zero if path absent)
  _resolved_awt="$(realpath -e "$ARTIFACTS_WT" 2>/dev/null)" || {
    echo "ERROR: ARTIFACTS_WT path does not exist: '$ARTIFACTS_WT'" >&2
    exit 1
  }
  ARTIFACTS_WT="$_resolved_awt"
else
  # BSD/macOS: existence check only — do NOT call realpath (symlink resolution drift)
  [ -d "$ARTIFACTS_WT" ] || {
    echo "ERROR: ARTIFACTS_WT path does not exist or is not a directory: '$ARTIFACTS_WT'" >&2
    exit 1
  }
fi
git -C "$ARTIFACTS_WT" rev-parse --git-dir >/dev/null 2>&1 || {
  echo "ERROR: ARTIFACTS_WT is not a git repository: '$ARTIFACTS_WT'" >&2
  exit 1
}

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
# Subcommand: --emit-handoff
# Assembles the complete HANDOFF.md payload with full anti-fabrication cross-checks
# and emits it to stdout. NO file written to disk. NO git commit.
# BC-5.41.001 PC10: The agent then invokes the Write tool to write HANDOFF.md.
# EC-016: HANDOFF_WRITE_TOOL_UNAVAILABLE=1 → hard error, no bash-redirect fallback.
# ---------------------------------------------------------------------------
cmd_emit_handoff() {
  # EC-016: fail loud if Write tool is marked unavailable (BC-5.41.001 EC-016)
  # HANDOFF_WRITE_TOOL_UNAVAILABLE is an internal harness flag for testing EC-016.
  # It is NOT a user-facing configuration variable. Do not set this in production.
  if [ "${HANDOFF_WRITE_TOOL_UNAVAILABLE:-0}" = "1" ]; then
    echo "HandoffWriteToolUnavailable: HANDOFF.md must be written via the Write tool (Claude Code native tool call); bash redirection is forbidden. Ensure the Write tool is available in the current harness context." >&2
    exit 1
  fi

  # Derive wave_id and classify stories
  local wave_id
  wave_id="$(derive_wave_id "$SPRINT_STATE_YAML" "$STATE_MD_PATH")"

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
      # EPIC-COMPLETE: emit HANDOFF.md payload with epic_status: complete to stdout
      write_handoff \
        "$wave_id" \
        "$BC_DIR" \
        "$PRECOMPACT_FLUSH_LOG" \
        "$STATE_MD_PATH" \
        "1"
      ;;
    has-next-wave)
      # Build story pairs
      local story_pairs=()
      local i
      for i in "${!NEXT_WAVE_STORY_IDS[@]}"; do
        story_pairs+=("${NEXT_WAVE_STORY_IDS[$i]}:${NEXT_WAVE_STORY_STATUSES[$i]}")
      done

      # Pre-flight anti-fabrication validation
      local preflight_story_index="${ARTIFACTS_WT}/stories/STORY-INDEX.md"
      if [ "${#story_pairs[@]}" -gt 0 ] && [ ! -f "$preflight_story_index" ]; then
        echo "ERROR: StoryIndexMissing — STORY-INDEX.md not found at '${preflight_story_index}'" >&2
        exit 1
      fi
      local preflight_pair
      for preflight_pair in "${story_pairs[@]+"${story_pairs[@]}"}"; do
        local preflight_sid="${preflight_pair%%:*}"
        if [ -f "$preflight_story_index" ]; then
          local preflight_escaped_sid
          preflight_escaped_sid="$(printf '%s' "$preflight_sid" | sed 's/\./\\./g')"
          if ! grep -qE "\| *${preflight_escaped_sid} *\|" "$preflight_story_index"; then
            echo "ERROR: AntiFabricationFailed — story ID '${preflight_sid}' not found in STORY-INDEX.md" >&2
            exit 1
          fi
        fi
      done

      # Emit HANDOFF.md payload to stdout (no disk write)
      write_handoff \
        "$wave_id" \
        "$BC_DIR" \
        "$PRECOMPACT_FLUSH_LOG" \
        "$STATE_MD_PATH" \
        "0" \
        "${story_pairs[@]+"${story_pairs[@]}"}"
      ;;
    *)
      echo "ERROR: unexpected classification result: $classification" >&2
      exit 1
      ;;
  esac
}

# ---------------------------------------------------------------------------
# Subcommand: --emit-wave-state
# Writes wave-state.yaml to ${ARTIFACTS_WT}/wave-state.yaml via bash.
# Skipped (exits 0 silently) on EPIC-COMPLETE (BC-5.41.002 PC3 EPIC-COMPLETE exception).
# ---------------------------------------------------------------------------
cmd_emit_wave_state() {
  # Derive wave_id and classify
  local wave_id
  wave_id="$(derive_wave_id "$SPRINT_STATE_YAML" "$STATE_MD_PATH")"

  NEXT_WAVE_STORY_IDS=()
  NEXT_WAVE_STORY_STATUSES=()
  BROKEN_STORY_IDS=()
  CLASSIFY_RESULT=""
  classify_stories "$SPRINT_STATE_YAML"
  local classification="$CLASSIFY_RESULT"

  case "$classification" in
    broken-sprint-state)
      echo "BrokenSprintState: stories in non-terminal, non-pending states exist." >&2
      exit 1
      ;;
    epic-complete)
      # EPIC-COMPLETE: skip wave-state.yaml (BC-5.41.002 PC3)
      exit 0
      ;;
    has-next-wave)
      local story_pairs=()
      local i
      for i in "${!NEXT_WAVE_STORY_IDS[@]}"; do
        story_pairs+=("${NEXT_WAVE_STORY_IDS[$i]}:${NEXT_WAVE_STORY_STATUSES[$i]}")
      done

      local prior_handoff_sha
      prior_handoff_sha="$(_get_prior_handoff_sha)"

      local next_wave_id=$(( wave_id + 1 ))
      write_wave_state \
        "${ARTIFACTS_WT}/wave-state.yaml" \
        "$next_wave_id" \
        "$prior_handoff_sha" \
        "$SPRINT_STATE_YAML" \
        "$ARTIFACTS_WT" \
        "${story_pairs[@]+"${story_pairs[@]}"}"
      ;;
    *)
      echo "ERROR: unexpected classification result: $classification" >&2
      exit 1
      ;;
  esac
}

# ---------------------------------------------------------------------------
# Subcommand: --commit
# Creates ONE atomic git commit via commit_to_artifacts.
# Two-arm conditional (BC-5.41.001 PC10 step 4 / EC-017):
#   HAS-NEXT-WAVE: verifies BOTH HANDOFF.md + wave-state.yaml present; stages both.
#   EPIC-COMPLETE: verifies HANDOFF.md present only; removes stale wave-state.yaml; stages HANDOFF.md alone.
# HandoffFileAbsent hard-abort if HANDOFF.md is absent on either path (EC-017).
# ---------------------------------------------------------------------------
cmd_commit() {
  # Derive wave_id and classify to determine which arm to use
  local wave_id
  wave_id="$(derive_wave_id "$SPRINT_STATE_YAML" "$STATE_MD_PATH")"

  NEXT_WAVE_STORY_IDS=()
  NEXT_WAVE_STORY_STATUSES=()
  BROKEN_STORY_IDS=()
  CLASSIFY_RESULT=""
  classify_stories "$SPRINT_STATE_YAML"
  local classification="$CLASSIFY_RESULT"

  # EC-017: verify HANDOFF.md is present on disk (both arms require it)
  if [ ! -f "${ARTIFACTS_WT}/HANDOFF.md" ]; then
    echo "HandoffFileAbsent: HANDOFF.md not found at ${ARTIFACTS_WT}/HANDOFF.md before commit; aborting atomic commit" >&2
    exit 1
  fi

  case "$classification" in
    broken-sprint-state)
      echo "BrokenSprintState: cannot commit — stories in non-terminal, non-pending states." >&2
      exit 1
      ;;
    epic-complete)
      # EPIC-COMPLETE arm: stage HANDOFF.md alone; remove stale wave-state.yaml
      # wave-state.yaml absence is expected and correct on this path (NOT an error)
      if git -C "$ARTIFACTS_WT" ls-files --error-unmatch wave-state.yaml >/dev/null 2>&1; then
        _git_wt rm wave-state.yaml
      elif [ -f "${ARTIFACTS_WT}/wave-state.yaml" ]; then
        rm -f "${ARTIFACTS_WT}/wave-state.yaml"
      fi

      commit_to_artifacts "$ARTIFACTS_WT" "$wave_id" HANDOFF.md > /dev/null

      local epic_id
      epic_id="$(_get_epic_id "$STATE_MD_PATH")"
      echo "EPIC-COMPLETE: All stories in sprint-state.yaml have reached terminal status."
      echo "Epic ${epic_id} is complete. No wave-state.yaml written for next wave."
      echo "HANDOFF.md committed to factory-artifacts with epic_status: complete."
      ;;
    has-next-wave)
      # HAS-NEXT-WAVE arm: verify BOTH files present before staging
      if [ ! -f "${ARTIFACTS_WT}/wave-state.yaml" ]; then
        echo "HandoffFileAbsent: wave-state.yaml not found at ${ARTIFACTS_WT}/wave-state.yaml before commit; aborting atomic commit" >&2
        exit 1
      fi

      # Single atomic commit of both files (BC-5.41.002 PC6)
      commit_to_artifacts "$ARTIFACTS_WT" "$wave_id" HANDOFF.md wave-state.yaml > /dev/null
      ;;
    *)
      echo "ERROR: unexpected classification result: $classification" >&2
      exit 1
      ;;
  esac
}

# ---------------------------------------------------------------------------
# Dispatch: subcommand or legacy monolithic main()
# ---------------------------------------------------------------------------
case "$SUBCOMMAND" in
  --emit-handoff)
    cmd_emit_handoff
    ;;
  --emit-wave-state)
    cmd_emit_wave_state
    ;;
  --commit)
    cmd_commit
    ;;
  "")
    echo "ERROR: monolithic wave-handoff invocation is removed; use the agent-orchestrated subcommands: --emit-handoff → (agent Write HANDOFF.md) → --emit-wave-state → --commit (see SKILL.md / ADR-026 §Decision 8)." >&2
    exit 1
    ;;
  *)
    echo "ERROR: unknown subcommand '$SUBCOMMAND'" >&2
    exit 1
    ;;
esac
