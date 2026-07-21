# PR #742 — Fresh-Eyes Review (Pass 2)

**PR:** fix(adr-032): payload-targeted timestamp enforcement + bats placement tests
**Branch:** fix/adr-032-timestamp-hook-edit-enforcement → develop
**HEAD SHA reviewed:** 692ba433fba83ab5bd178ee8f527e5556096a8cb
**PR state at review time:** MERGED
**Reviewer:** pr-reviewer (fresh-eyes, information-asymmetry wall applied)
**Prior verdict:** REQUEST_CHANGES on fe9829d1 (5 findings)

## Verdict: APPROVE

All prior-pass findings are genuinely resolved (not paper-fixed). CI is fully green
(SAST, bats-full-suite, bats-darwin-leg, cargo-host x2, all 5 build-dispatcher
targets, validate, platforms-drift). No BLOCKING findings. Three non-blocking
findings (1 suggestion, 2 nits) and one informational note are recorded below.

## Prior-pass finding disposition (independently verified against the diff)

### Finding 1 (MAJOR → RESOLVED) — factory-lock timestamp-absent case
**Verified genuine, not paper-fixed.** The awk insertion trigger changed from
`front == 2` (before closing `---`) to `front == 1 && /^timestamp:/` (after the
`timestamp:` line). When `timestamp:` is absent the trigger never fires and the
factory_lock block is not written — but this is caught LOUDLY by the pre-existing
post-write assertion in the `acquire` path (factory-lock-write.sh lines 328-331):

```
if ! awk '/^---$/{f++} f==1 && /^factory_lock:/{found=1} f>=2{exit} END{exit !found}' "$STATE_MD"; then
  printf 'factory-lock-write: SchemaViolation — factory_lock block was not written ...'
  exit 1
```

This assertion is load-bearing (not a doc-comment). It is now documented in the
`_write_factory_lock_block` strategy header, and a new covering bats test —
`test_BC_5_40_001_acquire_timestamp_absent_fails_loud` — asserts non-zero exit +
`SchemaViolation` on a timestamp-less fixture. Fail-loud confirmed. Resolved.

### Finding 3 (MINOR → RESOLVED as tracked residual) — deletion-path pinning
Two documenting Rust tests added (`ac020_edit_timestamp_line_deletion_payload_neutral_continues`,
`ac020_multiedit_timestamp_line_deletion_payload_neutral_continues`) pinning that a
timestamp-line deletion (old_string = timestamp line, new_string empty) is
payload-neutral → Continue. Behavior is documented, security-accepted, and attached
to the ADR-032 v1.14 amendment candidate. See SUGGESTION-1 below for the residual
enforcement gap this pins.

### Finding 4 (MINOR → RESOLVED) — parser-divergence
Test `ac020_new_string_sets_field_unquoted_timestamp_detected` added. Both
`new_string_sets_field` and `extract_top_level_field` delegate to
`extract_yaml_string_value` — same parser internally; unquoted `timestamp:` values
are detected. No divergence exists; the test documents the disclosed behavior. Resolved.

### T-4 bats update (D-866 regression fix → RESOLVED)
`verify-state-timestamp-refresh.bats` T-4 renamed to
`test_verify_state_timestamp_refresh_edit_payload_neutral_continues` and rewritten to
assert **exit 0 (Continue)** for a phase-change-only Edit, plus the `guard_ran`
sentinel and `plugins_run=1`. The pre-ADR-032 reconstruction/Block semantics and the
D-866 root cause are documented verbatim in the test header. Genuine assertion of the
new behavior. Resolved.

## Advisory plugin + registry verification (review focus #2)

- **Source (`verify-state-timestamp-advisory/src/lib.rs`):** clean, exec-free,
  fail-open on every error path, always returns `Continue` (advisory-only), no
  `unwrap()`/`expect()` in production paths. 3 unit tests cover positive path,
  Pre-condition-2 gate, and git_context-absent fail-open.
- **Field parity:** the advisory reads `state_md_in_commit`, `head_state_timestamp`,
  `head_parent_state_timestamp` from `git_context`. The dispatcher (`invoke.rs`)
  injects exactly those three fields (Steps 5/6/7), with fail-open `String::new()` on
  every git error and an exact-match (`f.trim() == "STATE.md"`) discriminator — no
  substring false-positives. Consistent.
- **Registry entry:** priority 159 is collision-free (155–158 taken, 159 free),
  `PostToolUse`, `tool = "^Bash$"`, `on_error = "continue"`, `async = false`. Correct
  for an advisory that cannot and must not block commits.
- **WASM staleness:** `verify-state-timestamp-advisory.wasm` was built at the final
  commit (692ba433), after all its source changes — fresh.
  `verify-state-timestamp-refresh.wasm` was rebuilt at 0104a8d; the only subsequent
  refresh source edit (e9eacde) is **94 insertions entirely inside `mod tests`** (two
  `#[test]` fns), so production logic is unchanged and the deployed WASM is
  functionally consistent with HEAD source. See NIT-2.

## New findings introduced by the multi-commit implementation

### SUGGESTION-1 (coverage) — timestamp deletion escapes both enforcement layers
An Edit/MultiEdit that DELETES the `timestamp:` field (old_string = timestamp line,
new_string = "") is payload-neutral → the refresh guard returns Continue. The AC-021
commit-time advisory also fails open when `head_state_timestamp` is empty. So a
deletion escapes both the Edit-time guard and the commit-time advisory. This is
currently documented and pinned by tests, security-accepted, and flagged as an
ADR-032 v1.14 amendment candidate (architect task recorded). Non-blocking given the
formal adjudication and concrete amendment target, but it is a genuine residual
enforcement gap and should not be lost — the v1.14 amendment should land it as an
explicit Block condition rather than remaining Continue.

### NIT-1 (test-precision) — T-4 asserts only the generic `guard_ran` substring
T-4's documented intent is to prove the *payload-neutral* Continue branch, but the
assertion greps only `guard_ran`, which every Continue path emits (including
`fail-open` reasons). A fail-open Continue would also satisfy the assertion.
Tightening the match to `guard_ran (continue: payload-neutral)` would make the test
discriminate the intended branch. Combined with exit 0 + plugins_run=1 the current
test is adequate; this is precision only.

### NIT-2 (build-hygiene) — refresh.wasm not rebuilt after final source edit
`verify-state-timestamp-refresh.wasm` was last built at 0104a8d, before the
test-only edit at e9eacde. Because that edit is entirely `#[cfg(test)]`, the compiled
cdylib is unaffected and there is no functional divergence. Strict reproducibility
would rebuild the WASM at HEAD so a `cargo build --target wasm32-wasip1` byte-check
is clean. Informational.

### INFORMATIONAL — stricter acquire precondition
`acquire` now hard-requires a `timestamp:` line in STATE.md frontmatter; a
frontmatter with two `---` fences but no `timestamp:` now fails loud (SchemaViolation)
where it previously succeeded (old front==2 awk inserted before the closing fence).
This is a deliberate ADR-032 precondition, fails loud (not silent), and is covered by
the new test 22. No action required; noted for awareness.

## Checklist

1. Diff coherence — PASS. All 17 files trace to ADR-032 (dispatcher prereq, advisory
   crate, refresh guard, factory-lock placement, WASM artifacts, registry, bats/tests,
   Cargo.lock/Cargo.toml workspace member). No unrelated changes.
2. Description accuracy — PASS. PR body matches the diff; the T-4 "FAILING" checkbox
   is superseded by later commit 0104a8d (T-4 now updated + passing).
3. Test coverage — PASS. Rust unit tests (advisory 3, refresh guard incl. Red Gates +
   deletion pins), bats placement tests (4 + timestamp-absent loud-fail), T-4 updated.
4. Demo evidence — N/A (internal hook-enforcement change; Red Gate + bats behavioral
   evidence in lieu, as declared).
5. Commit quality — PASS. Conventional format, clear messages, no AI attribution.
6. Diff size — PASS. Source-line delta is modest; the large raw line count is
   Cargo.lock + WASM binaries.
7. Missing changes — none identified against the stated four deliverables.
8. Dependency status — advisory WASM depends on the dispatcher git_context prereq
   (both in this PR); operator-cache effect gated on rc.24 release, documented.

## Summary
- BLOCKING: 0
- SUGGESTION: 1 (timestamp-deletion enforcement gap — tracked to v1.14)
- NIT: 2 (T-4 assertion precision; refresh.wasm rebuild hygiene)
- INFORMATIONAL: 1 (stricter acquire precondition)

Verdict: **APPROVE**. Prior REQUEST_CHANGES findings are all genuinely closed with
load-bearing tests. Remaining items are non-blocking and, where a real residual gap
exists (deletion path), it is already adjudicated and attached to a concrete amendment.
