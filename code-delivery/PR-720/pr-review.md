# PR Review — #720 fix(hooks): accept 7-char short SHAs in verify-sha-currency develop cite

**Verdict:** REQUEST_CHANGES
**Reviewer role:** pr-reviewer (fresh-context diff-only review)
**Date:** 2026-07-22
**Closes:** #629

---

## Summary

`verify-sha-currency.sh` extracted the `develop_head:` SHA with `{8,40}` and then truncated to 8 chars via `cut -c1-8`. A correct 7-char cite (`develop_head: 4c276d9`) produced `CITED_DEV_STATE=NOT_FOUND` and the currency check was silently skipped while the script exited 0 — a false negative on the field this check exists to verify. The PR fixes extraction to accept 7..40 chars, removes the truncation, and switches the comparison to a prefix match against the full actual SHA.

The fix itself is technically correct. The blocking finding is the absence of bats regression tests.

---

## Changes reviewed

**`plugins/vsdd-factory/templates/verify-sha-currency.sh`** — 2 hunks

- Hunk 1 (extraction): `{8,40}` → `{7,40}`, removes `cut -c1-8`, adds `head -1` for determinism on multi-match. The cited value is now preserved at its original length.
- Hunk 2 (comparison and WARN): Replaces `$ACTUAL_DEV` (fixed 8-char) comparison with `${ACTUAL_DEV_FULL:0:${#CITED_DEV_STATE}}`. `ACTUAL_DEV_FULL` is already defined in the script (`git -C "$PROJECT_ROOT" rev-parse develop`); the prefix slice is the git-canonical comparison for a short SHA of any length.
  - Adds WARN block: if `develop_head:` field is present in STATE.md but no hex SHA could be extracted, emits a WARN and sets `WARN=1`. Prevents the silent-skip-PASS class from recurring on malformed cites.
  - HANDOFF comparison similarly upgraded to `${ACTUAL_DEV_FULL:0:${#CITED_DEV_HANDOFF}}`. HANDOFF extraction still uses `{8}` fixed length (unchanged by this PR); the prefix comparison is effectively a no-op change for HANDOFF since the extracted value will always be 8 chars, but it is consistent.
- Exit code behavior: `WARN=1` does not affect exit code (the script exits 0 when `FAIL=0`, with or without WARNs). The WARN case for unextractable `develop_head:` correctly exits 0 rather than failing — an absent or malformed cite should surface as a warning, not block an otherwise clean burst.

---

## Findings

### F1 — MEDIUM: No bats regression tests for the fix

**Severity:** MEDIUM
**Triage routing:** test-writer

The PR author explicitly states "templates/ scripts have no bats suite in the repo (verified: no .bats file references verify-sha-currency or invokes any templates/*.sh). Red/green was done by direct invocation against a throwaway git fixture." This is acknowledged but does not satisfy the production-grade default.

The bats framework is present in `plugins/vsdd-factory/tests/` with 90+ test files, including `wave-gate-hooks.bats` which covers behavior that transitively depends on `verify-sha-currency.sh` via `validate-wave-gate-prerequisite.sh`. The infrastructure to add a `verify-sha-currency.bats` (or extend `wave-gate-hooks.bats`) exists. The specific regression scenarios are well-specified in the PR body:

1. Correct 7-char cite → exit 0, SHA extracted and matched (the primary bug case).
2. Correct 8-char cite → exit 0 (no regression).
3. Correct 40-char cite → exit 0.
4. Stale 7-char cite → exit 1, FAIL emitted.
5. `develop_head:` field present but no hex SHA extractable → exit 0, WARN emitted.

The fix closes a false-negative in a pipeline gate. Without a regression test, a future refactor of the extraction regex can silently restore the bug.

**Required action:** Add a `plugins/vsdd-factory/tests/verify-sha-currency.bats` covering at least cases 1 and 4 above (the primary bug case and its FAIL counterpart). Dispatching to test-writer.

---

### F2 — LOW: Error message shows 8-char `$ACTUAL_DEV` when comparison uses variable-length prefix

**Severity:** LOW (cosmetic; no correctness impact)

The FAIL message reads:
```
echo "FAIL: develop SHA in STATE.md is stale (cited=$CITED_DEV_STATE actual=$ACTUAL_DEV)"
```

`$ACTUAL_DEV` is the 8-char truncation; the comparison used `${ACTUAL_DEV_FULL:0:${#CITED_DEV_STATE}}`. For a 7-char cite that is stale, the message shows an 8-char actual against a 7-char cited, which can be confusing to read. A more consistent message would use `${ACTUAL_DEV_FULL:0:${#CITED_DEV_STATE}}` or just `${ACTUAL_DEV_FULL:0:8}` as context.

This does not change behavior (the FAIL is still emitted, the exit is still 1). It can be resolved in the same commit as F1.

---

## Verdict rationale

The core fix is technically correct: `ACTUAL_DEV_FULL` is properly defined, the prefix-match comparison is the git-canonical approach for variable-length short SHAs, and the WARN path for unextractable cites closes the silent-skip class. The reason for REQUEST_CHANGES is F1: a pipeline gate fix that closes a false-negative should ship with regression bats coverage. The bats infrastructure exists and the test scenarios are fully specified by the PR's own empirical evidence section.
