# PR Review — #719 fix(hooks): recognize named S-<PREFIX>.<NAME> story IDs in merge-prereq extraction

**Verdict:** APPROVE
**Reviewer role:** pr-reviewer (fresh-context diff-only review)
**Date:** 2026-07-22
**Closes:** #658

---

## Summary

`validate-pr-merge-prerequisites.sh` extracts the target story ID from the github-ops dispatch prompt using two patterns: `STORY-[0-9]+` then `S-[0-9]+\.[0-9]+`. Named story IDs such as `S-BL.DISCOVERY-WIRE` matched neither, so extraction fell through to the first pure-numeric `S-N.NN` substring — typically a merged dependency like `S-7.02`. The hook then blocked the merge citing the wrong story's missing evidence files. This PR inserts a named-ID branch between the two existing patterns, in most-specific-first order.

---

## Changes reviewed

**`plugins/vsdd-factory/hooks/validate-pr-merge-prerequisites.sh`** — 1 hunk

- Adds a comment block explaining the precedence requirement and why appending would not fix the bug (the pure-numeric pattern would win first).
- Inserts a second `if [[ -z "$STORY_ID" ]]; then` block with `grep -oE 'S-[A-Za-z]+\.[A-Za-z0-9-]+' | head -1` between STORY-NNN and pure-numeric extraction. Order: STORY-NNN → S-<PREFIX>.<NAME> → S-N.NN.
- The named regex `S-[A-Za-z]+\.[A-Za-z0-9-]+` correctly requires an alphabetic prefix, so `S-7.02` cannot match it. `STORY-NNN` is distinct (starts with `STORY-` not `S-`), so no cross-contamination.
- `|| true` appended to each `grep` call is consistent with the existing branches and prevents `set -e` from exiting on zero-match.

**`plugins/vsdd-factory/tests/pr-lifecycle-hooks.bats`** — 4 new tests

- Named target fully evidenced, prompt also contains a numeric dependency → passes (story-id extraction resolves the named ID, not the dependency).
- Named target missing evidence → blocks citing the named story ID, not the dependency's ID.
- `STORY-NNN` still takes precedence over any S-form.
- Pure-numeric `S-N.NN` still resolves when no named ID present.

---

## Findings

No findings. The most-specific-first ordering is the correct fix approach; appending the named branch after the numeric branch would not have fixed the bug (as the PR body correctly explains). The regex correctly scopes to alphabetic prefixes, preventing the numeric `S-N.NN` form from accidentally matching the named branch. Test coverage is complete across all four precedence cases.

---

## Verdict rationale

The fix is minimal, correctly ordered, and ships with full four-case regression coverage for the extraction precedence chain. No correctness issues were found.
