#!/bin/bash
# red-gate.sh — PreToolUse hook enforcing TDD red-before-green discipline
#
# Blocks Edit|Write on source files when strict TDD mode is active and the
# file being edited is not declared as "currently red" in the red-gate state.
#
# Strict mode is opt-in per-project: create `.factory/red-gate-state.json`
# with shape:
#   { "mode": "strict", "red": ["crates/foo/src/bar.rs", ...] }
#
# When mode is "off" or the file is missing, the hook allows all edits.
# When mode is "strict", only files listed in `red` may be edited.
# Test files are always allowed (test-first is the whole point).
#
# Ports dark-factory's tdd-enforcement.ts / red-gate.ts runtime extensions.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  echo "red-gate.sh: jq is required but not found" >&2
  exit 1
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

# No file path → allow
[[ -z "$FILE_PATH" ]] && exit 0

# Test files are always allowed
if [[ "$FILE_PATH" =~ (^|/)(tests?|__tests__|spec)/ ]] \
   || [[ "$FILE_PATH" =~ \.test\.[a-z]+$ ]] \
   || [[ "$FILE_PATH" =~ \.spec\.[a-z]+$ ]] \
   || [[ "$FILE_PATH" =~ _test\.(rs|go|py)$ ]]; then
  exit 0
fi

# Only guard recognized source files
case "$FILE_PATH" in
  *.rs|*.ts|*.tsx|*.js|*.jsx|*.py|*.go|*.java|*.kt|*.swift) ;;
  *) exit 0 ;;
esac

STATE_FILE=".factory/red-gate-state.json"
[[ ! -f "$STATE_FILE" ]] && exit 0

# Capture stderr safely
jq_stderr="$(mktemp)"
MODE=$(jq -r '.mode // "off"' "$STATE_FILE" 2>"$jq_stderr") || {
  if [[ -s "$jq_stderr" ]]; then
    echo "red-gate: failed to parse $STATE_FILE: $(cat "$jq_stderr")" >&2
  fi
  rm -f "$jq_stderr"
  exit 1
}
rm -f "$jq_stderr"

[[ "$MODE" != "strict" ]] && exit 0

# In strict mode: file must appear in .red[]
if jq -e --arg f "$FILE_PATH" '.red // [] | index($f)' "$STATE_FILE" >/dev/null; then
  exit 0
fi

# Also allow relative-path match (state file may store paths relative to repo root)
REL="${FILE_PATH#./}"
if jq -e --arg f "$REL" '.red // [] | index($f)' "$STATE_FILE" >/dev/null; then
  exit 0
fi

# Try stripping absolute path prefix to get repo-relative path
# Handles: /abs/path/to/project/src/lib.rs → src/lib.rs
if [[ "$FILE_PATH" == /* ]]; then
  # Try stripping PWD prefix
  REPO_REL="${FILE_PATH#"$PWD/"}"
  if [[ "$REPO_REL" != "$FILE_PATH" ]]; then
    if jq -e --arg f "$REPO_REL" '.red // [] | index($f)' "$STATE_FILE" >/dev/null; then
      exit 0
    fi
  fi
  # Try stripping git root prefix
  if command -v git &>/dev/null; then
    GIT_ROOT=$(git rev-parse --show-toplevel 2>/dev/null || true)
    if [[ -n "$GIT_ROOT" ]]; then
      GIT_REL="${FILE_PATH#"$GIT_ROOT/"}"
      if [[ "$GIT_REL" != "$FILE_PATH" ]]; then
        if jq -e --arg f "$GIT_REL" '.red // [] | index($f)' "$STATE_FILE" >/dev/null; then
          exit 0
        fi
      fi
    fi
  fi
fi

echo "Blocked: red-gate is in strict mode and $FILE_PATH is not in the red list." >&2
echo "Write a failing test for this code first, then add the path to .factory/red-gate-state.json under .red[]." >&2
echo "To disable strict mode for this session, set .mode to \"off\" in $STATE_FILE." >&2
exit 2
