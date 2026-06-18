#!/usr/bin/env bash
# lib/write-handoff.sh — HANDOFF.md writer
# Gathers all 9 base required fields with anti-fabrication cross-checks and
# writes HANDOFF.md to the factory-artifacts working tree.
# BC-5.41.001 PC1–PC9 | S-18.01
set -euo pipefail

# get_last_verified_develop_sha <factory_repo>
# Returns 40-char lowercase hex SHA via stdout from `git rev-parse origin/develop`.
# Uses FACTORY_REPO env var (the hermetic fixture repo path) if provided,
# otherwise uses the repo containing the artifacts worktree.
# MUST NOT be hardcoded or derived from cache (BC-5.41.001 INV4).
get_last_verified_develop_sha() {
  local factory_repo="${FACTORY_REPO:-}"
  if [ -n "$factory_repo" ]; then
    git -C "$factory_repo" rev-parse origin/develop
  else
    git rev-parse origin/develop
  fi
}

# get_precompact_flush_sha <flush_log_path>
# Implements the three-state rule (BC-5.41.001 PC5):
#   1. Log genuinely absent: returns "null"
#   2. Log present but FIELD-4 != "commit": returns "null" (corrupt/stale)
#   3. Log present + FIELD-4 == "commit" + valid 40-char hex FIELD-2: returns FIELD-2 SHA
#
# EC-011: log present + FIELD-4 == "commit" + FIELD-2 NOT valid 40-char hex:
#   exits 1 with "PrecompactShaMismatch" — the log claims a commit but the SHA
#   cannot be trusted. Hard-block, never silently write a bad value.
get_precompact_flush_sha() {
  local flush_log="$1"

  # State 1: log absent
  if [ ! -f "$flush_log" ]; then
    echo "null"
    return 0
  fi

  # Read the last line of the log
  local last_line
  last_line="$(tail -1 "$flush_log")"

  # State 2: log present but FIELD-4 != "commit"
  local field4
  field4="$(echo "$last_line" | awk '{print $4}')"
  if [ "$field4" != "commit" ]; then
    echo "null"
    return 0
  fi

  # State 3: log present + FIELD-4 == "commit" — validate FIELD-2 is 40-char lowercase hex
  local field2
  field2="$(echo "$last_line" | awk '{print $2}')"
  if ! echo "$field2" | grep -qE '^[0-9a-f]{40}$'; then
    echo "ERROR: PrecompactShaMismatch — precompact-flush-log FIELD-4=commit but FIELD-2='${field2}' is not a valid 40-char lowercase hex SHA; the log claims a commit but the SHA cannot be trusted" >&2
    exit 1
  fi

  echo "$field2"
  return 0
}

# check_active_bcs <bc_dir>
# Returns a non-empty list of active BC file basenames (one per line).
# Hard errors (exit 1) if no active BCs can be determined (BC-5.41.001 PC2 / AC-004).
check_active_bcs() {
  local bc_dir="$1"

  if [ ! -d "$bc_dir" ]; then
    echo "ERROR: NoActiveBCs — BC directory does not exist: $bc_dir" >&2
    exit 1
  fi

  # Find all .md files recursively in bc_dir
  local bc_files
  bc_files="$(find "$bc_dir" -name "*.md" -type f 2>/dev/null | sort)"

  if [ -z "$bc_files" ]; then
    echo "ERROR: NoActiveBCs — active_bcs is empty; no BC files found in $bc_dir" >&2
    exit 1
  fi

  echo "$bc_files"
  return 0
}

# write_handoff <output_path> <wave_id> <next_wave_story_ids_array_ref> <next_wave_story_statuses_array_ref> <bc_dir> <flush_log> <state_md> <is_epic_complete>
# Writes HANDOFF.md to <output_path> with all 9 base required fields.
# On EPIC-COMPLETE wave: additionally writes epic_status: complete, next_wave_stories: [].
# Hard errors if any required field is absent or any cross-check fails (BC-5.41.001 PC4).
#
# Parameters (positional):
#   $1 output_path
#   $2 wave_id
#   $3 bc_dir
#   $4 flush_log_path
#   $5 state_md_path
#   $6 is_epic_complete ("1" or "0")
#   $7+ space-separated "id:status" pairs for next_wave_stories (empty if epic-complete)
write_handoff() {
  local output_path="$1"
  local wave_id="$2"
  local bc_dir="$3"
  local flush_log="$4"
  local state_md="$5"
  local is_epic_complete="$6"
  shift 6
  # Remaining args are "id:status" pairs
  local story_pairs=("$@")

  # Gather fields
  local develop_sha
  develop_sha="$(get_last_verified_develop_sha)"

  # Validate SHA is 40-char hex
  if ! echo "$develop_sha" | grep -qE '^[0-9a-f]{40}$'; then
    echo "ERROR: AntiFabricationFailed — last_verified_develop_sha '${develop_sha}' is not 40-char lowercase hex" >&2
    exit 1
  fi

  # Get active BCs (hard errors if empty)
  local bc_files
  bc_files="$(check_active_bcs "$bc_dir")"

  # Get precompact_flush_sha (three-state rule + EC-011 validation).
  # get_precompact_flush_sha will exit 1 with PrecompactShaMismatch if the log
  # claims a commit but FIELD-2 is not valid 40-char hex.
  local precompact_sha
  precompact_sha="$(get_precompact_flush_sha "$flush_log")"

  # Get factory_lock_holder from STATE.md (BC-5.40.001 canonical shape).
  # Handles both inline scalar form: `factory_lock: "holder-name"` / `factory_lock: null`
  # and block-with-.holder form:
  #   factory_lock:
  #     holder: some-holder
  local factory_lock_holder="null"
  if [ -f "$state_md" ]; then
    local lock_val
    # Inline scalar form (most common): extract value on same line as factory_lock:
    lock_val="$(grep -E '^factory_lock:' "$state_md" | head -1 | awk '{print $2}' | tr -d '"')"
    if [ -n "$lock_val" ] && [ "$lock_val" != "null" ]; then
      factory_lock_holder="$lock_val"
    elif [ -z "$lock_val" ] || [ "$lock_val" = "null" ]; then
      # Block form: look for indented `.holder:` key immediately following `factory_lock:`
      local holder_val
      holder_val="$(awk '/^factory_lock:/{found=1; next} found && /^\s+holder:/{print $2; exit} found && /^[a-zA-Z]/{exit}' "$state_md" | tr -d '"')"
      if [ -n "$holder_val" ] && [ "$holder_val" != "null" ]; then
        factory_lock_holder="$holder_val"
      fi
    fi
  fi

  # Build active_bcs YAML list
  local active_bcs_yaml
  active_bcs_yaml="$(echo "$bc_files" | while IFS= read -r f; do
    local base
    base="$(basename "$f" .md)"
    echo "  - $base"
  done)"

  # Build next_wave_stories YAML
  local next_wave_yaml
  if [ "$is_epic_complete" = "1" ]; then
    next_wave_yaml="[]"
  else
    if [ "${#story_pairs[@]}" -eq 0 ]; then
      next_wave_yaml="[]"
    else
      next_wave_yaml=""
      local pair
      for pair in "${story_pairs[@]}"; do
        local sid="${pair%%:*}"
        local sstatus="${pair##*:}"
        next_wave_yaml="${next_wave_yaml}
  - id: ${sid}
    status: ${sstatus}"
      done
    fi
  fi

  # Write HANDOFF.md
  {
    echo "wave_id: ${wave_id}"
    echo "last_verified_develop_sha: ${develop_sha}"
    echo "active_bcs:"
    echo "$active_bcs_yaml"
    if [ "$is_epic_complete" = "1" ]; then
      echo "next_wave_stories: []"
    else
      echo "next_wave_stories:${next_wave_yaml}"
    fi
    echo "open_decisions: []"
    echo "pending_fixes: []"
    echo "process_gaps: []"
    echo "precompact_flush_sha: ${precompact_sha}"
    echo "factory_lock_holder: ${factory_lock_holder}"
    if [ "$is_epic_complete" = "1" ]; then
      echo "epic_status: complete"
    fi
  } > "$output_path"

  return 0
}
