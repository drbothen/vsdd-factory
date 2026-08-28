# PR Review — Cycle 3 ADDENDUM: BLOCKING-1 RESOLVED → **APPROVE**

**PR:** #787 — `feat(S-17.06): factory-lock shared functions`
**Supersedes:** the cycle-3 REQUEST_CHANGES verdict posted against `57510062`
**Re-reviewed SHA:** `0ec90901a84d788f8d3978863163df49df3762c0`
**Verdict:** **APPROVE** — 0 blocking findings

> Posted as a comment rather than a formal `--approve` review: GitHub rejects
> `addPullRequestReview` on one's own pull request because the authenticated account is also
> the PR author. Treat this as an APPROVE verdict.

---

## What changed

Commit `0ec90901` (`style(S-17.06): fix redundant_closure clippy error in new tests`) landed
while I was writing up cycle 3. I diffed `57510062..0ec90901`: it is **exactly two lines**,
both in the test module, and it is byte-identical to the fix I had already verified in my
scratch worktree:

```diff
-            || chrono::Utc::now(),
+            chrono::Utc::now,
```

at lines 1277 and 1318. No logic change, no change outside `#[cfg(test)]`.

**BLOCKING-1 is RESOLVED.**

---

## Re-verification at `0ec90901`

| Gate | Result |
|------|--------|
| `cargo fmt --check --all` | **PASS** (exit 0) |
| `cargo clippy -p factory-lock -p verify-factory-lock --all-targets -- -D warnings` | **PASS** — clean, 0 warnings |
| `cargo test -p factory-lock` | **PASS** — 19 unit + 8 integration |
| `cargo test -p verify-factory-lock` | **PASS** — 32 |
| CI `bats-full-suite (linux)` | **PASS** (20m53s) |
| CI `cargo-host` (ubuntu + macos) | re-running at `0ec90901`; locally reproduced green on the pinned 1.95.0 toolchain against identical content |

I did not take the lint fix on trust. Because it altered the form of the `now_fn` argument in
both new tests, I re-ran **both mutation probes** at `0ec90901` to confirm the change did not
weaken their discriminating power:

| Probe | Mutation applied | Result |
|---|---|---|
| **B** | removed the `if !has_factory_lock_key(content)` pre-check from `renew_lock_if_holder` | **18 passed, 1 failed** — `test_renew_lock_if_holder_unclosed_fence_no_lock_key_returns_noop` is the sole failure |
| **C** | reverted `email != trim_git_email(&lock_state.holder)` → `email != lock_state.holder` | **18 passed, 1 failed** — `test_renew_lock_if_holder_holder_with_trailing_whitespace_still_matches` is the sole failure |

Both mutants are still killed, by exactly one test each. Passing the fn item
`chrono::Utc::now` instead of the wrapper closure is semantically identical (both satisfy
`F: FnOnce() -> DateTime<Utc>`), as expected.

---

## Final state of cycle-3 findings

| Finding | Severity | Status at `0ec90901` |
|---|---|---|
| **BLOCKING-1** — new tests break `cargo clippy -D warnings`; CI red | blocking | **RESOLVED** in `0ec90901` |
| SUGGESTION-1 — `holder_with_trailing_whitespace` uses a wall clock + 2099 expiry | suggestion | Open — non-blocking |
| SUGGESTION-2 — assert renewed content leaves `holder` byte-identical | suggestion | Open — non-blocking |
| SUGGESTION-3 — PR body test counts stale (`55` claimed; actual `59`) | suggestion | Open — non-blocking |
| NIT-1 — `RenewOutcome` lacks `Clone` unlike its sibling types | nit | Open — non-blocking |

None of the four remaining items blocks merge. Two housekeeping asks that would be good to
fold into a future push (or a follow-up commit on this branch, at the author's discretion):

- Refresh the PR body's test counts — it claims `tests-55/55` and "factory-lock 23 +
  verify-factory-lock 32"; actual is 19 + 8 + 32 = **59**.
- The pre-merge checkbox "[x] All CI status checks passing (fmt + clippy + cargo test + bats
  — verified GREEN locally)" was false at `57510062`. It is true again at `0ec90901`, so it no
  longer misrepresents the branch — but it was checked before it was true, which is the
  underlying process issue (see below).

---

## Process note for the pipeline (not a finding against this PR)

Two consecutive review cycles were consumed by the same failure class: a test added to satisfy
a review comment was pushed without running the repo's own combined pre-push gate, breaking a
CI lint step.

- Cycle 2 BLOCKING-1: the new B-1 test broke `cargo fmt --check --all`.
- Cycle 3 BLOCKING-1: the two new discriminating tests broke `cargo clippy -D warnings`.

Both were caught by the exact command documented in `CLAUDE.md`:

```bash
cargo fmt --check --all && cargo clippy --workspace --all-targets -- -D warnings && cargo test --workspace --all-targets
```

Running that gate before every push — including pushes that only touch `#[cfg(test)]` code —
would have closed both cycles in one round trip instead of three. Test-only changes are not
lint-exempt: `--all-targets` puts test modules under `-D warnings`.

---

## Verdict

**APPROVE** — 0 blocking findings.

`covered_sha: 0ec90901a84d788f8d3978863163df49df3762c0`

All 6 ACs verify end-to-end. Both cycle-2 blocking findings are closed with mutation-verified
discriminating tests — not paper-fixes. The `#[derive(PartialEq)]` addition to `RenewOutcome`
is complete for the assertions used and semver-safe. `trim_git_email` has a single canonical
home with a real delegation from `verify-factory-lock`, and the new
`verify-factory-lock → factory-lock` dependency respects the documented direction with no
cycle. Diff is coherent against `origin/develop` (23 files, all in scope), commits are
conventional and story-tagged, and demo evidence is present as real `.gif`/`.webm` recordings
for every AC.

Merge once `cargo-host` reports green at `0ec90901`.
