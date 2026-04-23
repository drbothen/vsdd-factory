#!/bin/bash
# purity-check.sh — PostToolUse hook (warn-only)
#
# Enforces the pure-core boundary from SOUL.md. Files under `*/pure/**`,
# `*/core/**`, or ending in `_pure.rs` / `.pure.ts` must not contain
# side-effecting operations. This hook greps for known side-effect patterns
# and emits a warning to stderr when any are found.
#
# Non-blocking by design — architectural drift is surfaced, not enforced.
# The regression gate and CI catch hard failures.
#
# Ports dark-factory's purity-boundary-check.ts runtime extension.

set -euo pipefail

if ! command -v jq &>/dev/null; then
  echo "purity-check.sh: jq is required but not found" >&2
  exit 0
fi

INPUT=$(cat)
FILE_PATH=$(echo "$INPUT" | jq -r '.tool_input.file_path // empty')

_emit() {
  if [ -n "${CLAUDE_PLUGIN_ROOT:-}" ] && [ -x "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" ]; then
    "${CLAUDE_PLUGIN_ROOT}/bin/emit-event" "$@" 2>/dev/null || true
  fi
  return 0
}

if [[ -z "$FILE_PATH" ]] || [[ ! -f "$FILE_PATH" ]]; then
  exit 0
fi

# Match pure-core paths across common layouts
case "$FILE_PATH" in
  */pure/*|*/core/*|*_pure.rs|*.pure.ts|*/kernel/*) ;;
  *) exit 0 ;;
esac

# Forbidden patterns: I/O, networking, concurrency spawning, environment access, printing
PATTERNS=(
  'std::fs'
  'std::env::var'
  'std::process'
  'std::io::stdin'
  'reqwest'
  'hyper::'
  'tokio::spawn'
  'tokio::time::sleep'
  'tokio::fs'
  'println!'
  'eprintln!'
  'File::open'
  'File::create'
  'fetch('
  'process.env'
  'fs.readFile'
  'fs.writeFile'
)

HITS=()
for p in "${PATTERNS[@]}"; do
  if grep -qF -- "$p" "$FILE_PATH"; then
    HITS+=("$p")
  fi
done

if (( ${#HITS[@]} > 0 )); then
  _emit type=hook.block hook=purity-check matcher=PostToolUse \
        reason=pure_core_boundary_violation file_path="$FILE_PATH" severity=warn \
        patterns="${HITS[*]}"
  echo "purity-check: $FILE_PATH is in a pure-core path but contains side-effect patterns:" >&2
  for h in "${HITS[@]}"; do
    echo "  - $h" >&2
  done
  echo "Consider moving side effects to an adapter layer. See SOUL.md (pure-core boundary)." >&2
fi

exit 0
