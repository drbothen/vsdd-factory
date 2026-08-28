# PR Review — Cycle 3 (fresh-eyes, pre-merge)

**PR:** #787 — `feat(S-17.06): factory-lock shared functions`
**Branch:** `feature/S-17.06` → `develop`
**Reviewed SHA:** `57510062ca8b190bf9f17f94b44dda73eb7968ce`
**Verdict:** REQUEST_CHANGES — 1 blocking finding

> If GitHub rejects a formal `--request-changes` review because the authenticated account is
> also the PR author, treat this comment as a blocking REQUEST_CHANGES verdict — **do not
> merge** until BLOCKING-1 below is resolved.

---

## Summary

Both cycle-2 blocking findings are **genuinely fixed**. I re-ran the mutation experiments
against `57510062` and both new tests are true discriminators — each one, and only that one,
fails when its corresponding production guard is reverted. That is exactly what cycle 2 asked
for, and the tests are well-written: the doc comments state the mutant they kill, and both
assert on the *specific* discriminating property rather than on an incidental side effect.

However, the two new tests introduce a **new CI-breaking lint failure**. `cargo clippy
--workspace --all-targets -- -D warnings` now fails with two `clippy::redundant_closure`
errors, and CI on this PR is currently **RED** on both `cargo-host (ubuntu-latest)` and
`cargo-host (macos-latest)`, failing at exactly that step. The fix is a two-token edit.

This is the same failure *class* as cycle-2's BLOCKING-1 (`cargo fmt --check` broken by the
newly-added test) — a new test that satisfies the review but is not run through the repo's own
pre-push gate. Worth noting for the pipeline, not just for this PR: the combined gate in
`CLAUDE.md` (`cargo fmt --check --all && cargo clippy --workspace --all-targets -- -D warnings
&& cargo test --workspace --all-targets`) would have caught both.

---

## Cycle-2 fix verification (mutation experiments)

Built the branch at `57510062` in an isolated worktree, reverted each fix in turn, and re-ran
the committed suite. Baseline: **19 unit + 8 integration pass** in `factory-lock`, **32 pass**
in `verify-factory-lock`.

| Cycle-2 finding | New test | Mutant killed? |
|---|---|---|
| **BLOCKING-2** `has_factory_lock_key` pre-check | `test_renew_lock_if_holder_unclosed_fence_no_lock_key_returns_noop` | **YES — 18 passed, 1 failed** |
| **BLOCKING-3** `trim_git_email(&lock_state.holder)` | `test_renew_lock_if_holder_holder_with_trailing_whitespace_still_matches` | **YES — 18 passed, 1 failed** |

**Probe B** — removed the `if !has_factory_lock_key(content) { return Ok((NoOp, None)); }`
pre-check from `renew_lock_if_holder`. Result: exactly one failure, the new test. Notably the
pre-existing `test_renew_lock_if_holder_no_factory_lock_key_returns_noop` **still passed**,
which confirms the cycle-2 finding was correct (the old fixture routes through Case 0, not the
guard) and that the new test is the sole discriminator.

**Probe C** — reverted `email != trim_git_email(&lock_state.holder)` to `email !=
lock_state.holder`. Result: exactly one failure, the new test.

I also verified the mechanism from the parse side rather than trusting the test's premise:
`factory_lock_parse::extract_yaml_string_value` does **not** trim its value (it only strips
surrounding double quotes), and the scan loop in `parse_factory_lock` slices `&line[2..]`
without trimming. So `holder: holder@example.com   ` really does reach `LockState.holder`
with the trailing spaces intact, and the trim on the holder side is load-bearing. The test is
not vacuous.

**`#[derive(PartialEq)]` on `RenewOutcome`** — complete and correct for the assertions used.
`assert_eq!(result, Ok((RenewOutcome::NoOp, None)))` needs `PartialEq` + `Debug` on
`RenewOutcome`, `SkipReason`, and `LockError`; `SkipReason` and `LockError` already derive
`Debug, Clone, PartialEq`, and `RenewOutcome` now has `Debug, PartialEq`. Adding `PartialEq`
to a public enum is a backward-compatible change — no downstream crate can have a conflicting
impl (orphan rule), so there is no semver hazard. Verified by compilation.

---

## Findings

### BLOCKING-1 — the two new tests break `cargo clippy -D warnings`; CI is RED

| Field | Value |
|-------|-------|
| Severity | **blocking** |
| Category | coherence / CI |
| File | `crates/factory-lock/src/lib.rs` lines 1277 and 1318 |

Both new tests pass `|| chrono::Utc::now()` as `now_fn`. Clippy's `redundant_closure` lint
fires on a closure whose body is nothing but a call to a function, and this repo runs clippy
with `-D warnings`, so it is a hard error:

```
error: redundant closure
    --> crates/factory-lock/src/lib.rs:1277:13
     |
1277 |             || chrono::Utc::now(),
     |             ^^^^^^^^^^^^^^^^^^^^^ help: replace the closure with the associated function itself: `chrono::Utc::now`
     |
     = note: `-D clippy::redundant-closure` implied by `-D warnings`

error: redundant closure
    --> crates/factory-lock/src/lib.rs:1318:13

error: could not compile `factory-lock` (lib test) due to 2 previous errors
```

Reproduced locally on the pinned toolchain (`rust-toolchain.toml` → 1.95.0, clippy 0.1.95),
and confirmed on CI: run `33144936948`, job `cargo-host (ubuntu-latest)`, step
**`cargo clippy (workspace, deny warnings)`** → `Process completed with exit code 101`. Same
failure on `cargo-host (macos-latest)`. `cargo fmt --check --all` passes; the failure is
clippy-only.

**Fix** — drop the wrapper closure at both sites. `chrono::Utc::now` is a `fn() ->
DateTime<Utc>` item, which satisfies the `F: FnOnce() -> DateTime<Utc>` bound directly:

```rust
        let result = renew_lock_if_holder(
            content,
            || {
                called += 1;
                IdentityResolution::Resolved("anyone@example.com".to_string())
            },
            chrono::Utc::now,          // was: || chrono::Utc::now(),
        );
```

I applied exactly this two-line change in a scratch worktree and verified:
`cargo clippy -p factory-lock -p verify-factory-lock --all-targets -- -D warnings` → clean;
`cargo fmt --check --all` → exit 0; `cargo test -p factory-lock -p verify-factory-lock` →
19 + 8 + 32 pass, 0 failures. Both discriminating tests still kill their mutants after the
change (the lint fix does not weaken them — it is the same call, unwrapped).

Please also correct the PR body checklist item **"[x] All CI status checks passing (fmt +
clippy + cargo test + bats — verified GREEN locally)"**, which is currently false.

---

### SUGGESTION-1 — `holder_with_trailing_whitespace` test uses a wall clock and a 2099 expiry

| Field | Value |
|-------|-------|
| Severity | suggestion |
| Category | coverage / test quality |
| File | `crates/factory-lock/src/lib.rs` (`test_renew_lock_if_holder_holder_with_trailing_whitespace_still_matches`) |

The test injects `|| chrono::Utc::now()` (real wall clock) with `expires_at:
2099-01-01T10:45:00Z`. It discriminates correctly and cannot flake for the next 73 years, so
this is not a correctness problem. Two reasons to tighten it anyway:

1. **Determinism / house style.** `now_fn` exists precisely so tests don't need a wall clock,
   and the sibling tests in this module inject fixed instants. Using a fixed `now_fn` here
   makes the fixture self-documenting — the reader can see *why* Case 2 doesn't fire without
   reasoning about today's date.
2. **The fixture exercises a slightly odd path.** With `now + 2700s ≈ 2026` and an existing
   expiry of `2099`, the "renewal" moves `expires_at` **backwards** by 73 years. That is the
   specified behaviour (`renew_lock_with_now` sets `expires_at = now + TTL` unconditionally,
   so this is not a defect), but it means the happy-path assertion is made on an input no real
   caller would produce.

A fixed clock with a realistic expiry gives the same discrimination with neither wrinkle:

```rust
let now = "2026-08-28T10:00:00Z".parse::<DateTime<Utc>>().unwrap();
let content = format!(
    "---\ndocument_type: state\nfactory_lock:\n  holder: holder@example.com   \n  \
     locked_at: 2026-08-28T09:30:00Z\n  expires_at: 2026-08-28T10:30:00Z\n---\n"
);
let result = renew_lock_if_holder(
    &content,
    || IdentityResolution::Resolved("holder@example.com".to_string()),
    || now,
);
```

`now + 2700s` = `10:45:00Z` ≠ the existing `10:30:00Z`, so the byte-identical-expiry no-op
guard still cannot fire, and `now < expires_at` so Case 2 still doesn't fire.

---

### SUGGESTION-2 — assert the renewed content leaves `holder` alone

| Field | Value |
|-------|-------|
| Severity | suggestion |
| Category | coverage |
| File | `crates/factory-lock/src/lib.rs` (`test_renew_lock_if_holder_holder_with_trailing_whitespace_still_matches`) |

The test matches `Ok((RenewOutcome::Renewed(_), None))` and discards the new content. Since
the whole point of this fixture is a holder value with unusual whitespace, it's worth one more
line asserting that renewal rewrote *only* `expires_at` and did not normalise, re-quote, or
otherwise mutate the holder line — the property `renew_lock_updates_expires_at_only` checks
for clean fixtures, now confirmed for the whitespace case:

```rust
match result {
    Ok((RenewOutcome::Renewed(new_content), None)) => {
        assert!(
            new_content.contains("  holder: holder@example.com   \n"),
            "renewal must rewrite only expires_at, leaving holder byte-identical"
        );
    }
    other => panic!("Expected Renewed but got {other:?}"),
}
```

---

### SUGGESTION-3 — PR body test counts are stale

| Field | Value |
|-------|-------|
| Severity | suggestion |
| Category | description |

The PR body reports `tests-55/55`, "factory-lock unit tests 15/15", and "Total passing: 55
tests (factory-lock 23 + verify-factory-lock 32)". Actual counts at `57510062` are
**19 unit + 8 integration in `factory-lock` + 32 in `verify-factory-lock` = 59**. The counts
predate the cycle-1/2/3 test additions. Refresh them when pushing the BLOCKING-1 fix.

---

### NIT-1 — `RenewOutcome` derive set is now inconsistent with its siblings

| Field | Value |
|-------|-------|
| Severity | nit |
| Category | coherence |
| File | `crates/factory-lock/src/lib.rs` (`RenewOutcome`) |

Every other public type in this crate derives `Debug, Clone, PartialEq` (`LockError`,
`FactoryLock`, `IdentityResolution`, `SkipReason`). `RenewOutcome` is now `Debug, PartialEq`.
Consider `#[derive(Debug, Clone, PartialEq)]` for consistency — `Clone` on a
`Renewed(String)` payload is cheap to offer and downstream WASM callers (S-17.05 / S-17.07)
may well want it. Purely cosmetic; not worth a round trip on its own.

---

## Checklist

| # | Item | Result |
|---|------|--------|
| 1 | **Diff coherence** | PASS. Against `origin/develop` the diff is 23 files: `crates/factory-lock/src/lib.rs`, `verify-factory-lock/{Cargo.toml,src/lib.rs}`, `Cargo.lock` (+1 line), and `docs/demo-evidence/S-17.06/`. No unrelated changes. (Note: `git diff develop...` against the *stale local* `develop` ref pulls in rc.24 release noise — dispatcher binaries, WASM, bats. Reviewed against `origin/develop`, which is clean.) |
| 2 | **Description accuracy** | PARTIAL. Architecture, traceability, and AC mapping are accurate. Test counts are stale (SUGGESTION-3) and the "CI passing" checkbox is false (BLOCKING-1). |
| 3 | **Test coverage** | PASS. Both cycle-2 gaps are now covered by true discriminators (mutation-verified above). All changed production lines in `renew_lock_if_holder` are exercised. |
| 4 | **Demo evidence** | PASS. `docs/demo-evidence/S-17.06/` has `evidence-report.md` plus a `.tape` + `.gif` + `.webm` triple for each of AC-001..AC-006. Real recordings, not `.txt` placeholders. |
| 5 | **Commit quality** | PASS. 11 commits, all conventional format with the `S-17.06` story ID and clear scope (`stub:` → `test:` → `feat:` → `fix:` → `demo:` → review-cycle fixes). No AI attribution. |
| 6 | **Diff size** | PASS in context. 1,460 insertions, but ~1,000 of those are in `factory-lock/src/lib.rs` and are overwhelmingly test code + doc comments; the remainder is demo evidence and tape scripts. Production logic added is on the order of 120 lines. Above the 500-line flag threshold, but appropriate for a test-heavy library-surface story. |
| 7 | **Missing changes** | PASS — all 6 ACs verified, see below. |
| 8 | **Dependency status** | PASS. Story spec declares no upstream deps; `crates/factory-lock/` is already on `develop` from S-17.01. This PR is the Wave-5 topological base and *blocks* S-17.05 / S-17.07, so nothing needs to land first. New `verify-factory-lock → factory-lock` path dependency is correctly declared in `Cargo.toml` + `Cargo.lock` and respects the documented direction (no cycle). |

---

## AC re-verification (end-to-end at `57510062`)

| AC | Requirement | Evidence | Status |
|----|-------------|----------|--------|
| AC-001 | `renew_lock_if_holder` 6-case decision tree | All 6 cases present and ordered per spec. 10 `test_renew_lock_if_holder_*` tests pass, including both new discriminators and the `now == expires_at` and unparseable-`expires_at` boundaries. Case 3/5 comparison trims both sides per spec. | PASS |
| AC-002 | `resolve_identity` at most once, never for Cases 0/1/2 | `I: FnOnce` bound makes >1 call a compile error. `test_resolve_identity_called_at_most_once` plus `called == 0` assertions in the Case 0/1/2 tests and in the new unclosed-fence test. | PASS |
| AC-003 | `SkipReason::IdentityResolutionFailed` 4 fields from parsed `LockState` | Struct variant carries `reason, holder, locked_at, expires_at`; all three lock fields are moved out of `lock_state`, not caller-supplied. `test_skip_reason_identity_resolution_failed_carries_four_fields` passes. | PASS |
| AC-004 | `classify_identity_resolution` 4-shape rule | All four arms present and distinct; the `Err`, non-zero-exit, empty-stdout, and non-empty-stdout shapes each have a test. 4/4 pass. | PASS |
| AC-005 | `trim_git_email` single canonical home + delegation | `grep -rn '^pub fn trim_git_email' crates/` → exactly 1 hit (`factory-lock/src/lib.rs:564`). `verify-factory-lock/src/lib.rs:469` calls `factory_lock::trim_git_email(&git_email_raw)`; no local body remains. Delegation test additionally asserts the absence of a local `fn trim_git_email` definition. | PASS |
| AC-006 | 3 stale doc-comment loci corrected to post-F-P56-001 semantics | 3 `F-P56-001` references, one per required locus: line 124 (`renew_lock_with_now` algorithm doc, fn at 161), line 175 (inline comment on the `Ok(None)` match arm inside that fn body), line 335 (`parse_lock` doc, fn at 339). Doc-only; no logic change. Located by function name per TD-VSDD-091. | PASS |

**Gate results at `57510062` (pinned toolchain 1.95.0):**

| Gate | Result |
|------|--------|
| `cargo fmt --check --all` | PASS (exit 0) |
| `cargo clippy -p factory-lock -p verify-factory-lock --all-targets -- -D warnings` | **FAIL — 2 × `redundant_closure` (BLOCKING-1)** |
| `cargo test -p factory-lock` | PASS — 19 unit + 8 integration |
| `cargo test -p verify-factory-lock` | PASS — 32 |
| CI `cargo-host` (ubuntu + macos) | **FAIL at `cargo clippy (workspace, deny warnings)`** |

---

## Verdict

**REQUEST_CHANGES** — 1 blocking finding.

The substance of cycle 3 is good: both cycle-2 blocking findings are properly closed with
mutation-verified discriminating tests, the `PartialEq` derive is complete and
semver-safe, and all 6 ACs verify end-to-end. The only thing standing between this PR and
approval is BLOCKING-1, a two-token lint fix that is already turning CI red. Push
`chrono::Utc::now` in place of `|| chrono::Utc::now()` at both call sites, refresh the two
stale claims in the PR body, and this is ready to merge. The three suggestions and the nit
are non-blocking and can be folded into the same push if convenient.
