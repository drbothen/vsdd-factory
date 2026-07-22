# PR Review — #718 fix(bin): compute-input-hash --update upserts absent input-hash field

**Verdict:** APPROVE
**Reviewer role:** pr-reviewer (fresh-context diff-only review)
**Date:** 2026-07-22
**Closes:** #623

---

## Summary

This PR fixes a silent no-op bug in `compute-input-hash --update`. When the target file's frontmatter had no `input-hash:` field at all, `--update` printed the hash, emitted "updated" to stderr, and exited 0 — while writing nothing. The root cause was that both branches of the original update logic ran `sed s/^input-hash:.*//`, which produces no output when the pattern matches no line. The fix separates field presence (a distinct awk check) from field value, adds a find-or-create insert path (awk inserts before the closing `---` fence), and adds a post-write verification guard that fails loudly on malformed frontmatter.

---

## Changes reviewed

**`plugins/vsdd-factory/bin/compute-input-hash`** — 1 hunk (`--update` case, ~line 399)

- Adds `FIELD_PRESENT` check via awk: parses frontmatter (fm counter on `---` lines) and emits "yes" if `input-hash:` is found in the first block. Presence and value are now cleanly separated.
- Early-exit ("already current") is now gated on `[[ -n "$FIELD_PRESENT" ]] && [[ "$CURRENT" == "$HASH" ]]`. Prevents early-exit short-circuiting an absent-field insert.
- If `FIELD_PRESENT` is set: existing in-place `sed` replace. The redundant if/else in the original (both branches did the same sed) is collapsed to a single path.
- If `FIELD_PRESENT` is unset: awk find-or-create — inserts `input-hash: "<hash>"` immediately before the second `---` fence. The `!inserted` guard in the awk prevents duplicate insertions if `---` appears more than twice (defensive). The `--update` mode is only reachable after `inputs:` was parsed from frontmatter, so a closing fence is expected; when it is absent the post-write verification guard catches it.
- Post-write verification: re-reads the field via awk after writing; if the written value does not equal the computed hash, calls `die` (nonzero exit, explicit stderr). This closes the malformed-frontmatter edge case (no closing fence → awk insert finds no anchor → file unchanged → post-write check fails loudly). Silent success on a no-op is structurally impossible after this change.
- `rm -f "${FILE}.bak"` cleanup moved inside the `FIELD_PRESENT` (sed) branch where the `.bak` file is actually created.

**`plugins/vsdd-factory/tests/input-hash.bats`** — 3 new tests

- Field absent → field created with correct 7-char hash; fence and body preserved; single field; exit 0.
- Idempotence: second `--update` run on an already-current file produces stable value, no duplicate field, exit 0.
- Malformed frontmatter (no closing `---` fence) → nonzero exit; `failed to write input-hash` on stderr; field not written.

---

## Findings

No findings. The FIELD_PRESENT check is the correct mechanism for distinguishing absence from placeholder. The awk insert logic correctly places the new field before the second `---` fence. The post-write verification guard is a solid defensive measure that makes the tool's success contract trustworthy.

One minor observation (not a finding): the error message on the FAIL path still uses `actual=$ACTUAL_DEV` (8-char truncation) while the comparison uses `FIELD_PRESENT` logic — this is unrelated to #718's scope and is not a defect here.

---

## Cross-PR note (#715 / #718 same file)

See cross-PR note in PR-715/pr-review.md. Both PRs' changes are non-overlapping; either can merge first. Prefer merging #715 first.

---

## Verdict rationale

The fix is production-grade: it addresses the root cause (conflated presence/value leading to silent sed no-op), uses correct awk idioms for both detection and insert, and ships with three focused regression tests covering the primary bug, idempotence, and the malformed-frontmatter edge case.
