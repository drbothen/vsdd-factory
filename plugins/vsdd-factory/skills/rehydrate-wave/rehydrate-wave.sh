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
# EC-004: stories:[]/empty spec_files → WARN on stderr, inject arch_files+state_pointer, exit 0.
# EC-006: arch_files:[] → WARN on stderr, inject stories spec_files+state_pointer only, exit 0.
# Missing listed spec file: WARN (stderr), continue, exit 0 (AC-006 / PC6).
# wave-state.yaml absent AND no EPIC-COMPLETE HANDOFF.md: RehydrationError, exit 1 (AC-007/AC-008).
# EPIC-COMPLETE: read HANDOFF.md, inject STATE.md + arch_files, emit message, exit 0 (AC-009).
# No RAG / vector search / fuzzy matching (AC-008 / BC-6.24.001 Inv3).
#
# Usage (with explicit args — overrides env vars):
#   rehydrate-wave.sh --repo <main-repo-dir> --artifacts-worktree <path>
#
# Bare invocation (production defaults):
#   rehydrate-wave.sh
#   Uses REPO_DIR=. and ARTIFACTS_WT=.factory when neither flag nor env var is set.
#
# Arguments:
#   --repo <path>                Main repo dir; `git show factory-artifacts:` runs here.
#                                Defaults to REPO_DIR env var, then "." if unset.
#   --artifacts-worktree <path>  Factory-artifacts worktree path (for fallback detection only).
#                                Defaults to ARTIFACTS_WT env var, then ".factory" if unset.
#
# S-18.03 | BC-6.24.001 v1.10 | VP-088 v1.1

set -euo pipefail

# ---------------------------------------------------------------------------
# Argument parsing
# Production defaults: REPO_DIR="." and ARTIFACTS_WT=".factory" when neither
# flag nor env var is provided (F-P1-005 bare-invocation resolution).
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

# Apply production defaults for bare invocation (F-P1-005).
# --repo / REPO_DIR env var override the default. Same for --artifacts-worktree / ARTIFACTS_WT.
REPO_DIR="${REPO_DIR:-.}"
ARTIFACTS_WT="${ARTIFACTS_WT:-.factory}"

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
# _check_missing_file — check a single filepath for existence and emit a warning
# on stderr if absent from BOTH the filesystem AND factory-artifacts.
# Usage: _check_missing_file "<filepath>"
# A file is "missing" only when absent from both sources (PC6 / F-P1-008).
# Does NOT exit; the caller continues. Use the same stable warning string for
# both the wave-state and EPIC-COMPLETE paths (F-P2-003 sibling-path sweep).
# ---------------------------------------------------------------------------
_check_missing_file() {
  local filepath="$1"
  # Check filesystem existence relative to REPO_DIR first.
  if [ -f "${REPO_DIR}/${filepath}" ] || [ -f "$filepath" ]; then
    return 0  # Present on filesystem — not missing.
  fi
  # Corroborate: also check factory-artifacts branch via git cat-file (PC6 / F-P1-008).
  # Only warn if absent from BOTH filesystem AND factory-artifacts.
  if git -C "$REPO_DIR" cat-file -e "factory-artifacts:${filepath}" 2>/dev/null; then
    return 0  # Present in factory-artifacts — not missing.
  fi
  echo "WARNING: listed spec file not found on filesystem: ${filepath}" >&2
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

      # F-P2-004: Parse next_wave_stories to guard contradictory manifests.
      # VP-088 §4 + BC EC-EPIC define EPIC as epic_status:complete AND next_wave_stories:[].
      # HANDOFF.md has a single top-level next_wave_stories list (single-occurrence assumption;
      # the producer schema is defined as a top-level scalar list — no nesting).
      NEXT_WAVE_STORIES="$(_parse_yaml_list "next_wave_stories" "$HANDOFF_CONTENT")"
      if [ -n "$NEXT_WAVE_STORIES" ]; then
        # Contradictory manifest: epic_status=complete but non-empty next_wave_stories.
        # Emit a stable warning (greppable by test harness) and continue on the EPIC-COMPLETE
        # path — epic_status:complete is the authoritative discriminator (BC EC-EPIC); we
        # inject STATE.md + arch_files and surface the contradiction for operator review.
        echo "WARNING: HANDOFF.md epic_status=complete but next_wave_stories is non-empty; manifest is contradictory." >&2
      fi

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

      # F-P2-003: Check arch_files listed in HANDOFF.md for missing files (same helper and
      # stable warning string used by the wave-state path — sibling-path sweep complete).
      # Skip state_pointer file (same policy as wave-state path Step 5).
      while IFS= read -r filepath; do
        [ -z "$filepath" ] && continue
        [ "$filepath" = "$STATE_POINTER" ] && continue
        _check_missing_file "$filepath"
      done <<< "$INJECTED_SET"

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

# Step 3a: EC-004 warning — stories empty or all spec_files empty (F-P1-001).
# Emit to stderr BEFORE the transparency block; continue injecting arch_files + state_pointer.
if [ -z "$STORY_SPEC_FILES" ]; then
  echo "WARNING: wave-state.yaml lists no stories (stories: [] or no spec_files); injecting arch_files + state_pointer only." >&2
fi

# Step 3b: EC-006 warning — arch_files empty (F-P1-002).
# Emit to stderr; continue injecting stories spec_files + state_pointer only.
if [ -z "$ARCH_FILES_LIST" ]; then
  echo "WARNING: wave-state.yaml lists no arch_files; no architectural context will be injected." >&2
fi

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

# Step 5: Check for missing spec files and emit warnings (AC-006 / PC6).
# A file is "missing" only when ABSENT from BOTH the filesystem AND factory-artifacts
# (git cat-file -e corroboration per F-P1-008).
# Missing file = WARN on stderr and continue; do NOT exit non-zero.
# Uses _check_missing_file helper — same function used for EPIC-COMPLETE arch_files
# (F-P2-003 sibling-path sweep: both branches use the same stable warning string).
while IFS= read -r filepath; do
  [ -z "$filepath" ] && continue
  # Skip state_pointer file — STATE.md may not exist locally in test environments
  # but is always a valid rehydration target (it's a pointer, not a local file gate).
  # We only warn about story spec / arch files that are explicitly listed.
  [ "$filepath" = "$STATE_POINTER" ] && continue
  _check_missing_file "$filepath"
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
