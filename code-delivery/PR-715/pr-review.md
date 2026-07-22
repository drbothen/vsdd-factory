# PR Review — #715 fix(bin): preserve trailing newlines in compute-input-hash accumulation

**Verdict:** APPROVE
**Reviewer role:** pr-reviewer (fresh-context diff-only review)
**Date:** 2026-07-22
**Closes:** #637

---

## Summary

This PR fixes a byte-fidelity bug in `plugins/vsdd-factory/bin/compute-input-hash`. The accumulation loop used `CONCAT="${CONCAT}$(cat file)"` — Bash command substitution strips all trailing newlines, causing the computed hash to diverge from any raw-byte reader (`md5sum <file`, Python `open(f,"rb").read()`). The pipeline's `validate-input-hash` hook then hard-blocked on false-positive drift for any text file ending in a newline (the normal case), making the hook's block untrustworthy.

---

## Changes reviewed

**`plugins/vsdd-factory/bin/compute-input-hash`** — 4 hunks

- Hunk 1 (line 215): Introduces `HASH_INPUT=$(mktemp)` + `trap 'rm -f "$HASH_INPUT"' EXIT`. Replaces the shell-variable accumulation with a temp file. The EXIT trap ensures cleanup on error paths.
- Hunk 2 (line 337): Glob-expansion path — `CONCAT="${CONCAT}$(cat "$expanded_file")"` → `cat "$expanded_file" >> "$HASH_INPUT"`. Redirection does not go through a subshell; all bytes including trailing newlines are preserved.
- Hunk 3 (line 345): Resolved-file path — same replacement. Also changes the empty-input guard from `[[ -z "$CONCAT" ]]` (checks empty string) to `[[ ! -s "$HASH_INPUT" ]]` (checks zero-size temp file). Semantically equivalent for the no-inputs case.
- Hunk 4 (line 383): Hash computation — `echo -n "$CONCAT" | md5sum` → `md5sum < "$HASH_INPUT"` (and `md5 < "$HASH_INPUT"` on macOS). Stdin redirection preserves all bytes.

**`plugins/vsdd-factory/tests/input-hash.bats`** — 2 new tests

- Single-input: tool hash must equal `md5sum < file` reference for a file with trailing newlines.
- Multi-input: tool hash must equal `md5sum < concatenated-files` reference.

---

## Findings

No findings. The fix is minimal, mechanically correct, and directly targets the constraint that command substitution cannot be used when trailing-newline byte fidelity is required. The bats tests provide byte-accurate regression coverage for both the single-input and multi-input paths.

The migration note (existing artifacts on `factory-artifacts` carry stored hashes computed under the old accumulation; a one-pass `--scan --update` sweep at merge clears the backlog) is accurate and appropriately documented in the PR body.

---

## Cross-PR note (#715 / #718 same file)

Both #715 and #718 touch `plugins/vsdd-factory/bin/compute-input-hash` and `plugins/vsdd-factory/tests/input-hash.bats`. Their hunks are non-overlapping:

- #715 modifies the accumulation loop (lines ~215, ~337, ~345) and hash computation (~383).
- #718 modifies the `--update` case branch (~399+).
- The gap between #715's last changed line (~393) and #718's first changed line (~399) is sufficient; git context windows do not overlap.
- In `input-hash.bats`: #715 inserts at line ~295; #718 inserts at line ~84. Non-overlapping.
- Either PR can merge first. Merge #715 first is the lower-risk order (lower line-number insertions; #718's higher-line-number changes are unaffected by the line-count delta from #715).

---

## Verdict rationale

The fix is production-grade: it addresses the root cause (command substitution strips newlines), uses a well-established shell idiom (temp file + EXIT trap), does not introduce new failure modes, and ships with adequate bats regression coverage.
