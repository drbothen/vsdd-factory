# PR #742 — Fresh-Eyes Review (pr-reviewer)

**PR:** #742 — `fix(adr-032): payload-targeted timestamp enforcement + bats placement tests`
**Branch:** `fix/adr-032-timestamp-hook-edit-enforcement` → `develop`
**Scope:** ADR-032 v1.13 — resolves D-866 (spurious `TimestampStale`/`LockExpiryStale` blocks on payload-neutral Edits).
**Verdict:** REQUEST_CHANGES

Reviewer sees only the diff, PR description, and cited test evidence (information-asymmetry wall).

---

## Findings

### Finding 1 — MAJOR (correctness regression, silent failure)
**File:** `plugins/vsdd-factory/bin/factory-lock-write.sh` (`_write_factory_lock_block`, awk block)

| Field | Value |
|-------|-------|
| Severity | MAJOR |
| Category | correctness / silent-failure |
| Finding | The rewritten awk inserts `factory_lock:` **only** when a `timestamp:` line exists in the frontmatter (`front == 1 && /^timestamp:/`). The old logic inserted unconditionally before the closing `---`. If a STATE.md frontmatter lacks a `timestamp:` line, the insertion never fires, `inserted` stays 0, and the file is emitted unchanged — **no lock block is written, yet the script proceeds**. `timestamp:` has silently become a hard precondition (every fixture in the diff was edited to add it, with comments "timestamp: field is required"), but the code has no fallback and no error when it is absent, and no test covers the timestamp-absent acquire path. |
| Failure scenario | `acquire` runs against a STATE.md with no `timestamp:` field → lock block silently not written → caller believes it holds a lock it does not → lost-update / concurrent-write risk. |
| Suggestion | Add a fallback to before-closing-`---` insertion when no `timestamp:` line is found, OR fail loudly with a clear error; add a test covering acquire on a timestamp-less fixture. |
| Caveat | Only `_write_factory_lock_block` is visible in the diff. If `acquire` performs a post-write `grep factory_lock` verification that aborts on a missing block, the failure would be loud rather than silent — confirm against the full script. Even then, the new hard dependency is undocumented and untested. |

### Finding 2 — ADVISORY / ENFORCEMENT-WEAKENING (escalate to security-reviewer)
**File:** `crates/hook-plugins/verify-state-timestamp-refresh/src/lib.rs` (`guard_logic` — payload-neutral early return; Step 7 gate `(sets_factory_lock || sets_timestamp) && …`)

| Field | Value |
|-------|-------|
| Severity | ADVISORY (enforcement-weakening) |
| Category | enforcement-weakening / security |
| Finding | The guard now returns `Continue` for any Edit/MultiEdit whose `new_string` sets neither `timestamp:` nor `factory_lock:` at column 0. This is the intended D-866 fix and is well-tested, but it reduces the PreToolUse enforcement surface. `ac020_edit_body_lock_held_no_factory_lock_continues` confirms a body edit made **while a held lock is expired** now passes (previously blocked). Stale-lock protection for payload-neutral body edits is no longer enforced by this guard. |
| Failure scenario | Operator edits STATE.md body under an expired `factory_lock` → guard returns Continue → concurrent writer may hold the lock → lost update. The compensating control (`verify-state-timestamp-advisory`) is advisory-only (`on_error=continue`, cannot block) and only covers the "STATE.md-in-commit but timestamp not advanced" case at git-commit time — NOT the stale-lock-body-edit case. |
| Suggestion | Security-reviewer sign-off on converting a fail-closed concurrency/staleness gate into a partially non-blocking post-commit advisory. Spec-sanctioned per ADR-032, but the net enforcement change should be explicitly accepted. |

### Finding 3 — MINOR / ADVISORY (untested enforcement gap)
**File:** `crates/hook-plugins/verify-state-timestamp-refresh/src/lib.rs` (payload-scan branch, `sets_timestamp` computation)

| Field | Value |
|-------|-------|
| Severity | MINOR |
| Category | test-coverage / enforcement-gap |
| Finding | The scan only detects whether `new_string` *sets* `timestamp:`. An Edit/MultiEdit that **deletes** the timestamp line (old_string = `timestamp: "…"`, new_string = body/empty) is classified payload-neutral → `Continue`, allowing the enforcement anchor field to be stripped. Pre-fix this reconstructed to absent-timestamp proposed content and blocked (Step 4 `NotFound`). No test covers timestamp-field deletion via Edit. |
| Failure scenario | MultiEdit replaces the `timestamp:` line with body text → guard returns Continue → STATE.md left with no timestamp field. |
| Suggestion | Add a covering test for timestamp-field deletion, or an explicit deletion check. |

### Finding 4 — MINOR (latent parser divergence)
**File:** `crates/hook-plugins/verify-state-timestamp-refresh/src/lib.rs` (`new_string_sets_field`)

| Field | Value |
|-------|-------|
| Severity | MINOR |
| Category | correctness / robustness |
| Finding | The payload scan uses `factory_lock_parse::extract_yaml_string_value`, while on-disk/proposed extraction uses `extract_top_level_field` (Steps 4/5). Two different parsers decide "is this the timestamp field." If they disagree on a form one accepts and the other doesn't (e.g. unquoted `timestamp: 2026-…`), a stale timestamp set in the unrecognized form could yield `sets_timestamp=false` → enforcement skipped → `Continue` instead of `Block`. The doc comment only reasons about the safe false-positive direction; the false-negative direction is the concern. |
| Failure scenario | Unquoted stale `timestamp:` in `new_string` not recognized by `extract_yaml_string_value` → payload-neutral → Continue, bypassing the stale-timestamp block. |
| Suggestion | Use a single shared parser for both payload-scan and content extraction, or add a test with an unquoted timestamp value. `extract_yaml_string_value`'s body is not in this diff; behavior on unquoted values unconfirmed. |

### Finding 5 — NIT (description accuracy)
| Field | Value |
|-------|-------|
| Severity | NIT |
| Category | description |
| Finding | PR description labels deliverable (1) as "payload-targeted guard rewrite with 11 bats tests." Those 11 tests are Rust unit tests in `verify-state-timestamp-refresh/src/lib.rs`, not bats. The bats tests are the 4 separate `factory-lock-write.bats` placement tests. |
| Suggestion | Correct the wording (11 Rust unit tests + 4 bats placement tests). |

---

## Checklist items that passed

- **Red Gate discipline (item 2):** verified. All four claimed Red Gates (`ac020_edit_body_only_no_timestamp_continues`, `ac020_multiedit_no_timestamp_in_any_new_string_continues`, `ac020_edit_body_lock_held_no_factory_lock_continues`, `ac020_edit_factory_lock_only_stale_expires_blocks`) produce a different result pre-fix (Block) vs post-fix (Continue / different Block reason) — each genuinely fails against unmodified code. Regression guards correctly assert identical pre/post behavior; Red Gate 4 discriminates on message identity (`TimestampStale` vs `LockExpiryStale`).
- **Paper-fix detection (item 4):** none. `guard_logic` is a real structural rewrite (new branching + `new_string_sets_field` helper), not a doc/rename/assert-only patch. Tests were rewritten to assert new behavior, not commented out.
- **Sibling-site sweep (item 5):** the three new `GitContext` fields are threaded through every construction site in the diff (`empty()`, `to_json()`, `build_git_context`, both test-file literals). `GitContext` has all-required fields; with CI green, any un-updated construction site would have failed to compile — sweep complete.
- git_context injection (dispatcher) ↔ consumption (advisory `payload.extra["git_context"]`) is consistent; both gate on the same `git + " commit" + ".factory"` heuristic; advisory is exec-free and always returns `Continue`.

## Verdict
**REQUEST_CHANGES.** No BLOCKER. Finding 1 (MAJOR silent-failure regression) should be resolved or explicitly proven safe against the full `acquire` flow before merge. Findings 2–3 (enforcement-weakening + timestamp-deletion gap) should be routed to security-reviewer for sign-off, since this PR converts a fail-closed concurrency/staleness gate into a partially non-blocking advisory path.
