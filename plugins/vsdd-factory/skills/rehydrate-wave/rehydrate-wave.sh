#!/usr/bin/env bash
# rehydrate-wave.sh — Rehydrate session context from wave-state.yaml on factory-artifacts.
#
# Reads wave-state.yaml EXCLUSIVELY via `git show factory-artifacts:wave-state.yaml`
# (BC-6.24.001 Inv1 / AC-001). Never reads the working-tree copy.
#
# Computes injected set = Set(stories[*].spec_files) ∪ Set(arch_files) ∪ {state_pointer}
# with deduplication (BC-6.24.001 Inv2 / AC-002 / AC-010).
#
# Emits INJECTED_FILE_COUNT=<n> sentinel (VP-088 §2 PC2-SIGNAL / AC-002).
# Always injects state_pointer even if not in spec_files (AC-004).
# Missing listed spec file: WARN, continue, exit 0 (AC-006).
# wave-state.yaml absent AND no EPIC-COMPLETE HANDOFF.md: RehydrationError, exit 1 (AC-007/AC-008).
# EPIC-COMPLETE: read HANDOFF.md, inject STATE.md + arch_files, emit message, exit 0 (AC-009).
# No RAG / vector search / fuzzy matching (AC-008 / BC-6.24.001 Inv3).
#
# Usage:
#   rehydrate-wave.sh --repo <main-repo-dir> --artifacts-worktree <path>
#
# Arguments:
#   --repo <path>                Main repo dir; `git show factory-artifacts:` runs here.
#   --artifacts-worktree <path>  Factory-artifacts worktree path (for fallback detection only).
#
# S-18.03 | BC-6.24.001 v1.10 | VP-088 v1.1

set -euo pipefail

# ---------------------------------------------------------------------------
# Argument parsing
# ---------------------------------------------------------------------------

REPO_DIR="${REPO_DIR:-}"
ARTIFACTS_WT="${ARTIFACTS_WT:-}"

while [ $# -gt 0 ]; do
  case "$1" in
    --repo)
      REPO_DIR="$2"; shift 2 ;;
    --artifacts-worktree)
      ARTIFACTS_WT="$2"; shift 2 ;;
    *)
      echo "ERROR: unknown argument '$1'" >&2
      exit 1 ;;
  esac
done

: "${REPO_DIR:?ERROR: --repo <main-repo-dir> is required}"
: "${ARTIFACTS_WT:?ERROR: --artifacts-worktree <path> is required}"

# Unset GIT_DIR — tests may inject GIT_DIR pointing at the fixture repo root, but
# our git -C calls explicitly target the repo dir. An ambient GIT_DIR overrides -C.
unset GIT_DIR 2>/dev/null || true

# ---------------------------------------------------------------------------
# _git_show_factory — read a file from factory-artifacts branch via git show.
# Returns 0 on success (content to stdout), 1 if the file is not in the branch.
# This is the ONLY allowed mechanism to read factory-artifacts files (BC-6.24.001 Inv1).
# ---------------------------------------------------------------------------
_git_show_factory() {
  local filepath="$1"
  git -C "$REPO_DIR" show "factory-artifacts:${filepath}" 2>/dev/null
}

_git_show_factory_ok() {
  local filepath="$1"
  git -C "$REPO_DIR" show "factory-artifacts:${filepath}" >/dev/null 2>&1
}

# ---------------------------------------------------------------------------
# _parse_yaml_scalar — extract a scalar value from naive YAML (no jq).
# Usage: _parse_yaml_scalar "key" <<< "$yaml_content"
# Returns the trimmed value after "key: " from stdin.
# ---------------------------------------------------------------------------
_parse_yaml_scalar() {
  local key="$1"
  local content="$2"
  printf '%s\n' "$content" \
    | grep -E "^${key}:" \
    | head -1 \
    | sed "s/^${key}:[[:space:]]*//" \
    | tr -d '"'"'" \
    | tr -d '\r'
}

# ---------------------------------------------------------------------------
# _parse_yaml_list — extract a simple YAML list (indented "- item" lines)
# that follows a given key, until the next non-indented key.
# Usage: _parse_yaml_list "key" "$content"
# Emits one item per line (trimmed).
# ---------------------------------------------------------------------------
_parse_yaml_list() {
  local key="$1"
  local content="$2"
  # Find lines after "key:" that start with "  -" (2-space indented list items).
  # Stop at the next top-level key (line starting with a non-space, non-dash char).
  printf '%s\n' "$content" \
    | awk -v key="${key}:" '
      /^[^[:space:]]/ { in_key=0 }
      $0 ~ "^"key"[[:space:]]*$" { in_key=1; next }
      in_key && /^[[:space:]]+-[[:space:]]/ {
        sub(/^[[:space:]]+-[[:space:]]+/, "")
        gsub(/^[[:space:]]+|[[:space:]]+$/, "")
        gsub(/"/, "")
        gsub(/'"'"'/, "")
        print
      }
    '
}

# ---------------------------------------------------------------------------
# _deduplicate — emit unique lines from stdin (preserving order of first occurrence).
# POSIX awk implementation — no sort needed.
# ---------------------------------------------------------------------------
_deduplicate() {
  awk '!seen[$0]++'
}

# ---------------------------------------------------------------------------
# _parse_wave_state_story_spec_files — extract all spec_files from all stories.
# wave-state.yaml stories block format:
#   stories:
#     - id: S-18.01
#       spec_files:
#         - foo.md
#         - bar.md
#     - id: S-18.02
#       spec_files:
#         - baz.md
# This parser collects all lines under any "spec_files:" block inside the stories
# section (4-space indented "- file" entries).
# ---------------------------------------------------------------------------
_parse_all_story_spec_files() {
  local content="$1"
  printf '%s\n' "$content" \
    | awk '
      /^stories:/ { in_stories=1; next }
      in_stories && /^[^[:space:]]/ { in_stories=0 }
      in_stories && /^[[:space:]]+spec_files:/ { in_spec=1; next }
      in_stories && /^[[:space:]]+-[[:space:]]+id:/ { in_spec=0; next }
      in_stories && /^[[:space:]]+[a-z_]+:/ { in_spec=0; next }
      in_stories && in_spec && /^[[:space:]]+-[[:space:]]/ {
        line=$0
        sub(/^[[:space:]]+-[[:space:]]+/, "", line)
        gsub(/^[[:space:]]+|[[:space:]]+$/, "", line)
        gsub(/"/, "", line)
        gsub(/'"'"'/, "", line)
        if (line != "") print line
      }
    '
}

# ---------------------------------------------------------------------------
# Main rehydration logic
# ---------------------------------------------------------------------------

# Step 1: Try to read wave-state.yaml from factory-artifacts branch (BC-6.24.001 Inv1).
WAVE_STATE_CONTENT=""
WAVE_STATE_FOUND=0
if _git_show_factory_ok "wave-state.yaml"; then
  WAVE_STATE_CONTENT="$(_git_show_factory "wave-state.yaml")"
  WAVE_STATE_FOUND=1
fi

# Step 2: If wave-state.yaml is absent, check for EPIC-COMPLETE HANDOFF.md (AC-009).
if [ "$WAVE_STATE_FOUND" -eq 0 ]; then
  HANDOFF_CONTENT=""
  HANDOFF_FOUND=0
  if _git_show_factory_ok "HANDOFF.md"; then
    HANDOFF_CONTENT="$(_git_show_factory "HANDOFF.md")"
    HANDOFF_FOUND=1
  fi

  if [ "$HANDOFF_FOUND" -eq 1 ]; then
    # Check epic_status field
    EPIC_STATUS="$(_parse_yaml_scalar "epic_status" "$HANDOFF_CONTENT")"
    if [ "$EPIC_STATUS" = "complete" ]; then
      # EC-EPIC path (AC-009 / BC-6.24.001 EC-EPIC / VP-088 §4):
      # Read STATE.md + arch_files from HANDOFF.md only.
      # Do NOT emit RehydrationError.
      STATE_POINTER="$(_parse_yaml_scalar "state_pointer" "$HANDOFF_CONTENT")"
      # Fallback if state_pointer absent in HANDOFF.md
      STATE_POINTER="${STATE_POINTER:-.factory/STATE.md}"

      # Collect arch_files from HANDOFF.md
      ARCH_FILES_LIST=""
      ARCH_FILES_LIST="$(_parse_yaml_list "arch_files" "$HANDOFF_CONTENT")"

      # Build injected set: state_pointer + arch_files (deduplicated)
      ALL_FILES="$(
        printf '%s\n' "$STATE_POINTER"
        printf '%s\n' "$ARCH_FILES_LIST"
      )"
      # Filter empty lines and deduplicate
      INJECTED_SET="$(printf '%s\n' "$ALL_FILES" | grep -v '^[[:space:]]*$' | _deduplicate)"
      INJECTED_COUNT="$(printf '%s\n' "$INJECTED_SET" | grep -c '.' || true)"

      echo "Epic complete — no next-wave stories"
      echo ""
      echo "Session context to rehydrate (from HANDOFF.md, epic_status: complete):"
      while IFS= read -r f; do
        [ -n "$f" ] && echo "  - $f"
      done <<< "$INJECTED_SET"
      echo ""
      echo "INJECTED_FILE_COUNT=${INJECTED_COUNT}"
      echo ""
      echo "Rehydration complete. Confirm to proceed: this is an EPIC-COMPLETE session — no next-wave stories are listed in the manifest. Operator: please read the injected files above and confirm this session's scope."
      exit 0
    fi
  fi

  # No wave-state.yaml AND no EPIC-COMPLETE HANDOFF.md → hard block (AC-007 / AC-008).
  echo "RehydrationError: wave-state.yaml not found on factory-artifacts; cannot rehydrate. Run /wave-handoff on wave N to produce the manifest." >&2
  exit 1
fi

# Step 3: Parse wave-state.yaml (wave-state.yaml found on factory-artifacts).

# Extract state_pointer (AC-004 — always injected)
STATE_POINTER="$(_parse_yaml_scalar "state_pointer" "$WAVE_STATE_CONTENT")"
STATE_POINTER="${STATE_POINTER:-.factory/STATE.md}"

# Extract arch_files
ARCH_FILES_LIST="$(_parse_yaml_list "arch_files" "$WAVE_STATE_CONTENT")"

# Extract all spec_files from all stories
STORY_SPEC_FILES="$(_parse_all_story_spec_files "$WAVE_STATE_CONTENT")"

# Step 4: Build injected set = Set(stories[*].spec_files) ∪ Set(arch_files) ∪ {state_pointer}
# with deduplication (BC-6.24.001 Inv2 / AC-002 / AC-010).
ALL_FILES="$(
  printf '%s\n' "$STATE_POINTER"
  printf '%s\n' "$STORY_SPEC_FILES"
  printf '%s\n' "$ARCH_FILES_LIST"
)"
# Filter empty lines and deduplicate
INJECTED_SET="$(printf '%s\n' "$ALL_FILES" | grep -v '^[[:space:]]*$' | _deduplicate)"
INJECTED_COUNT="$(printf '%s\n' "$INJECTED_SET" | grep -c '.' || true)"

# Step 5: Check for missing spec files and emit warnings (AC-006).
# We check if each listed file is accessible (on repo dir or artifacts worktree).
# Missing file = WARN and continue; do NOT exit non-zero.
HAS_MISSING=0
while IFS= read -r filepath; do
  [ -z "$filepath" ] && continue
  # Skip state_pointer file — STATE.md may not exist locally in test environments
  # but is always a valid rehydration target (it's a pointer, not a local file gate).
  # We only warn about story spec / arch files that are explicitly listed.
  if [ "$filepath" = "$STATE_POINTER" ]; then
    continue
  fi
  # Check filesystem existence relative to REPO_DIR
  if [ ! -f "${REPO_DIR}/${filepath}" ] && [ ! -f "$filepath" ]; then
    echo "WARNING: listed spec file not found on filesystem: ${filepath}" >&2
    HAS_MISSING=1
  fi
done <<< "$INJECTED_SET"

# Step 6: Emit transparency output — human-readable injected file list (BC-6.24.001 Inv4 / AC-005).
echo "Rehydration context for wave session:"
echo ""
echo "Files to inject into session context:"
while IFS= read -r filepath; do
  [ -n "$filepath" ] && echo "  - $filepath"
done <<< "$INJECTED_SET"
echo ""

# Step 7: Emit machine-stable INJECTED_FILE_COUNT sentinel (VP-088 §2 PC2-SIGNAL / AC-002).
echo "INJECTED_FILE_COUNT=${INJECTED_COUNT}"
echo ""

# Step 8: Operator confirmation prompt (BC-6.24.001 postcondition 5 / AC-005 / Inv4).
echo "Confirm rehydration: ${INJECTED_COUNT} file(s) listed above will be injected into the session context. Operator: please read each file above to rehydrate the session scope, then proceed with pipeline work."

exit 0
