#!/usr/bin/env bash
# precompact-flush-prune.sh — prune precompact-flush-log when entry count > 1000
#
# USAGE
#   precompact-flush-prune.sh <log-file-path>
#
# CONTRACT (VP-090 + AC-009..AC-013)
#
#   Structural precondition (AC-009 / VP-090 §0):
#     The log file MUST end with a newline (\n) before pruning is attempted.
#     If the file does NOT end with \n, the script exits non-zero with message:
#       "precompact-flush-log structural violation: file must end with newline before pruning"
#     No modification is performed.
#
#     Special case: an empty (0-byte) file is treated as a no-op (AC-013 boundary).
#     An empty file vacuously satisfies the no-prune path (no entries to count).
#
#   Threshold (AC-010 / VP-090 §1):
#     Count physical lines via `wc -l`. When count EXCEEDS 1000 (>= 1001),
#     prune to the last 500 lines (`tail -n 500`). When count <= 1000, exit 0
#     with no modification.
#
#   Exact boundary conditions (AC-013 / VP-090 §4):
#     - 0 lines  (empty file): no-op (AC-013 empty-file boundary)
#     - 500 lines: no prune
#     - 1000 lines: no prune (threshold is strictly > 1000)
#     - 1001 lines: prune to 500 (threshold met; last line preserved)
#
#   Atomic write (AC-011 / VP-090 §2):
#     Pruned content is written to a temporary file in the SAME directory as the
#     log, then renamed atomically (POSIX `mv`). The original log is NOT modified
#     if the write fails. The last line (most recent entry) is preserved.
#
#   Invocation context (AC-012 / VP-090 §3):
#     This script is invoked ONLY by check-state-health (or equivalent maintenance
#     entrypoint). It MUST NOT be called from the `precompact-flush` WASM plugin.
#     It MUST NOT be registered as a hook plugin (no hooks-registry.toml entry).
#
#   Dependencies:
#     - bash >= 4.x
#     - wc -l     (POSIX; available in all CI environments)
#     - tail -n   (POSIX)
#     - mv        (POSIX rename; atomic on same-filesystem)
#     - mktemp    (POSIX; for temporary file creation)
#
#   NOT required / FORBIDDEN:
#     - python, jq, node, or any non-standard tools
#     - Registration in hooks-registry.toml
#     - Invocation from the `precompact-flush` WASM plugin
#
set -euo pipefail

# ---------------------------------------------------------------------------
# Arguments
# ---------------------------------------------------------------------------

if [ "$#" -ne 1 ]; then
  echo "usage: precompact-flush-prune.sh <log-file-path>" >&2
  exit 1
fi

LOG_FILE="$1"

# SEC-002: Restrict log file path to .factory/ subdirectory (defense-in-depth; CWE-22).
# AC-012 restricts callers to check-state-health, but we enforce this structurally too.
case "$LOG_FILE" in
  */.factory/*) ;;
  .factory/*) ;;
  *)
    echo "precompact-flush-prune: log-file path must be under .factory/" >&2
    exit 1
    ;;
esac

# ---------------------------------------------------------------------------
# AC-013 boundary: empty file is a no-op exit 0.
# An empty file has no entries to prune; not a structural violation.
# ---------------------------------------------------------------------------

file_size=$(wc -c < "$LOG_FILE")
if [ "$file_size" -eq 0 ]; then
  exit 0
fi

# ---------------------------------------------------------------------------
# AC-009 / VP-090 §0: structural precondition — file must end with \n.
# Read the last byte and check it is 0x0a (newline).
# ---------------------------------------------------------------------------

# Use xxd first (faster, consistent output); fall back to od -An -tx1 (POSIX).
# Both are stripped of whitespace including tabs (\t) for portability.
last_byte_hex=$(tail -c 1 "$LOG_FILE" | xxd -p 2>/dev/null | tr -d ' \t\n')
if [ -z "$last_byte_hex" ]; then
  last_byte_hex=$(tail -c 1 "$LOG_FILE" | od -An -tx1 | tr -d ' \t\n')
fi

if [ "$last_byte_hex" != "0a" ]; then
  echo "precompact-flush-log structural violation: file must end with newline before pruning" >&2
  exit 1
fi

# ---------------------------------------------------------------------------
# AC-010 / VP-090 §1: count lines and prune if count > 1000.
# ---------------------------------------------------------------------------

line_count=$(wc -l < "$LOG_FILE")

if [ "$line_count" -le 1000 ]; then
  # Threshold not met — no prune needed. AC-013 boundary.
  exit 0
fi

# Count exceeds 1000: prune to the last 500 lines. AC-010.
# Write to a temp file in the SAME directory for atomic rename. AC-011.
log_dir="$(dirname "$LOG_FILE")"
tmp_file="$(mktemp "$log_dir/.precompact-flush-prune.XXXXXX")"

# Ensure temp file is removed on failure (set -e will exit; trap cleans up).
trap 'rm -f "$tmp_file"' EXIT

# Write last 500 lines to temp file. AC-011: last line is preserved.
tail -n 500 "$LOG_FILE" > "$tmp_file"

# Atomic rename (POSIX mv on same filesystem). AC-011.
mv "$tmp_file" "$LOG_FILE"

# Disable trap now that rename succeeded.
trap - EXIT

exit 0
