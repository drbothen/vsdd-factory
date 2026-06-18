#!/usr/bin/env bash
# lib/parse-sprint-state.sh — Pure parser for sprint-state.yaml
# Derives wave_id, classifies stories by status (terminal/pending/draft/broken),
# detects EPIC-COMPLETE and BrokenSprintState.
# BC-5.41.001 PC2 + BC-5.41.002 PC3 | S-18.01
set -euo pipefail

# Terminal statuses — stories in these states are considered complete.
_TERMINAL_STATUSES="merged withdrawn cancelled"
# Next-wave statuses — stories eligible for the upcoming wave.
_NEXT_WAVE_STATUSES="pending draft"

# _is_terminal <status> — returns 0 if terminal, 1 otherwise
_is_terminal() {
  local status="$1"
  local s
  for s in $_TERMINAL_STATUSES; do
    [ "$s" = "$status" ] && return 0
  done
  return 1
}

# _is_next_wave <status> — returns 0 if pending/draft, 1 otherwise
_is_next_wave() {
  local status="$1"
  local s
  for s in $_NEXT_WAVE_STATUSES; do
    [ "$s" = "$status" ] && return 0
  done
  return 1
}

# classify_stories <sprint_state_yaml_path>
# Sets global arrays and prints classification to stdout.
# Output (stdout): one of: epic-complete | broken-sprint-state | has-next-wave
# Side effects (global vars set):
#   NEXT_WAVE_STORY_IDS   — array of story IDs with pending/draft status
#   NEXT_WAVE_STORY_STATUSES — parallel array of their statuses
#   BROKEN_STORY_IDS       — array of story IDs in non-terminal, non-pending/draft states
#
# IMPORTANT: This function sets globals. Call it in the CURRENT shell (not via $(...)).
# Use: classify_stories "$yaml" ; RESULT="$CLASSIFY_RESULT"
# The CLASSIFY_RESULT global is set instead of stdout to avoid subshell issues.
classify_stories() {
  local sprint_state_yaml="$1"

  NEXT_WAVE_STORY_IDS=()
  NEXT_WAVE_STORY_STATUSES=()
  BROKEN_STORY_IDS=()
  CLASSIFY_RESULT=""

  # If sprint-state.yaml absent or empty, treat as EPIC-COMPLETE
  if [ ! -f "$sprint_state_yaml" ]; then
    CLASSIFY_RESULT="epic-complete"
    return 0
  fi

  local has_any_story=0
  local has_terminal=0
  local has_next_wave=0
  local has_broken=0

  local current_id=""
  local current_status=""

  while IFS= read -r line; do
    # Match "  - id: <value>" (story block start)
    if echo "$line" | grep -qE '^\s+-\s+id:\s+\S+'; then
      # Save previous story if we have a complete pair
      if [ -n "$current_id" ] && [ -n "$current_status" ]; then
        has_any_story=1
        if _is_terminal "$current_status"; then
          has_terminal=1
        elif _is_next_wave "$current_status"; then
          has_next_wave=1
          NEXT_WAVE_STORY_IDS+=("$current_id")
          NEXT_WAVE_STORY_STATUSES+=("$current_status")
        else
          has_broken=1
          BROKEN_STORY_IDS+=("$current_id")
        fi
      fi
      current_id="$(echo "$line" | awk '{print $NF}')"
      current_status=""
    # Match "    status: <value>"
    elif echo "$line" | grep -qE '^\s+status:\s+\S+'; then
      current_status="$(echo "$line" | awk '{print $NF}')"
    fi
  done < "$sprint_state_yaml"

  # Process the last story
  if [ -n "$current_id" ] && [ -n "$current_status" ]; then
    has_any_story=1
    if _is_terminal "$current_status"; then
      has_terminal=1
    elif _is_next_wave "$current_status"; then
      has_next_wave=1
      NEXT_WAVE_STORY_IDS+=("$current_id")
      NEXT_WAVE_STORY_STATUSES+=("$current_status")
    else
      has_broken=1
      BROKEN_STORY_IDS+=("$current_id")
    fi
  fi

  # No stories at all → EPIC-COMPLETE
  if [ "$has_any_story" -eq 0 ]; then
    CLASSIFY_RESULT="epic-complete"
    return 0
  fi

  # Broken: non-terminal/non-pending/draft stories exist AND no next-wave stories
  if [ "$has_broken" -eq 1 ] && [ "$has_next_wave" -eq 0 ]; then
    CLASSIFY_RESULT="broken-sprint-state"
    return 0
  fi

  # All stories terminal → EPIC-COMPLETE
  if [ "$has_terminal" -ge 1 ] && [ "$has_next_wave" -eq 0 ] && [ "$has_broken" -eq 0 ]; then
    CLASSIFY_RESULT="epic-complete"
    return 0
  fi

  CLASSIFY_RESULT="has-next-wave"
  return 0
}

# derive_wave_id <sprint_state_yaml_path> <state_md_path>
# Returns the integer wave_id via stdout.
# Derived from STATE.md current_step: "pass-N" → extracts N.
# MUST NOT read any current_wave: field — that field does not exist.
derive_wave_id() {
  local sprint_state_yaml="$1"
  local state_md="$2"

  # Primary: extract pass number from STATE.md current_step: "pass-N"
  if [ -f "$state_md" ]; then
    local step_val
    step_val="$(grep -E '^current_step:' "$state_md" | head -1 | awk '{print $2}' | tr -d '"')"
    if echo "$step_val" | grep -qE '^pass-[0-9]+$'; then
      echo "$step_val" | grep -oE '[0-9]+'
      return 0
    fi
  fi

  # Fallback: wave 1 if STATE.md absent or unparseable
  echo "1"
  return 0
}
