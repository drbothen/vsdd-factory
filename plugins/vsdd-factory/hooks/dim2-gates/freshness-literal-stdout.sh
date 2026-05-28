#!/usr/bin/env bash
# freshness-literal-stdout.sh — D-454(b) freshness re-execution with literal stdout capture
#
# Closes: D-454(b) — Freshness re-execution persists literal command + exit code + raw stdout.
# No forward-narrative allowed in Dim-2 attestation blocks.
#
# Usage: freshness-literal-stdout.sh <command-to-rerun>
# Exits 0 if the re-run command exits 0.
# Exits 1 if the re-run command exits non-zero, printing failure info.
#
# Per D-449(a): outputs literal command + exit code + raw stdout for Dim-2 attestation.

set -euo pipefail

# ---- help ----
if [[ "${1:-}" == "--help" ]] || [[ "${1:-}" == "-h" ]]; then
  cat <<'USAGE'
freshness-literal-stdout.sh — D-454(b) freshness re-execution with literal stdout capture

USAGE:
  freshness-literal-stdout.sh <command-to-rerun>

ARGUMENTS:
  command-to-rerun   Shell command string to re-execute (quoted as single argument)

EXIT CODES:
  0 — re-run command exited 0 (freshness gate passes)
  1 — re-run command exited non-zero (freshness gate fails)

EXAMPLES:
  freshness-literal-stdout.sh "grep -c 'trajectory_tail' .factory/STATE.md"
  freshness-literal-stdout.sh "wc -l < .factory/STATE.md"

NOTES:
  Invoked during Dim-2 fix-burst attestation per D-449(a). Paste literal output
  (command + exit code + stdout) into burst-log Dim-2 block. Replaces hand-rolled
  one-off grep commands that silently narrow scope across attestation cycles (D-453(e)).
USAGE
  exit 0
fi

# ---- argument validation ----
if [[ $# -lt 1 ]]; then
  echo "ERROR: freshness-literal-stdout.sh requires 1 argument: <command-to-rerun>" >&2
  echo "Run with --help for usage." >&2
  exit 1
fi

CMD="$1"

echo "$ ${CMD}"

# Run the command, capture stdout and exit code without triggering set -e
CMD_OUTPUT=""
CMD_EXIT=0
CMD_OUTPUT=$(eval "$CMD" 2>&1) || CMD_EXIT=$?

echo "${CMD_OUTPUT}"
echo "exit code: ${CMD_EXIT}"

if [[ "$CMD_EXIT" -eq 0 ]]; then
  exit 0
else
  echo "FAIL: command exited ${CMD_EXIT} — freshness gate did not pass"
  exit 1
fi
