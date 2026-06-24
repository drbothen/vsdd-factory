#!/usr/bin/env bash
# precompact-flush-prune.sh — prune precompact-flush-log when entry count > 1000
#
# USAGE
#   precompact-flush-prune.sh
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
#     entrypoint). It MUST NOT be called from precompact-flush.sh.
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
#     - Invocation from precompact-flush.sh
#
# STUB STATUS
#   S-18.04b stub skeleton. Functional implementation is NOT provided here.
#   The Red Gate bats tests (plugins/vsdd-factory/tests/precompact-flush-prune.bats)
#   MUST FAIL against this stub. The implementer fills in the body at T-8..T-11.
#
set -euo pipefail

# ---------------------------------------------------------------------------
# S-18.04b STUB: No functional implementation below this line.
# The implementer replaces this stub with the real implementation per T-8..T-11.
# All bats Red Gate tests for precompact-flush-prune.sh will FAIL against this stub.
# ---------------------------------------------------------------------------

echo "precompact-flush-prune: stub not implemented (S-18.04b T-8..T-11)" >&2
exit 1
