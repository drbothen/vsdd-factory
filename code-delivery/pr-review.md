# PR #801 — Fresh-Eyes Pre-Merge Review

**PR:** fix(precompact-flush): remove independent TTL literal, single-source from factory_lock_parse::TTL_SECONDS (F-WG5-P2-001)
**Head:** `438a7aad7d828a5463316e2e029efcb1739b7044`
**Base:** develop
**Reviewer:** pr-reviewer (fresh-eyes diff review)
**Date:** 2026-08-29

## VERDICT: APPROVE

covered_sha: `438a7aad7d828a5463316e2e029efcb1739b7044`

The entire diff is a 3-line deletion of a dead `pub const` from `crates/hook-plugins/precompact-flush/src/lib.rs`. Zero blocking findings. Production-grade; fixes wave-integration finding F-WG5-P2-001 (MEDIUM). The ci.yml comment-only S-17.07 corrections (`verify-state-timestamp-refresh` → `stamp-state-timestamp`) are benign doc-comment updates already on develop HEAD.

---

## Diff Review — 0 blocking findings

Verified against both `origin/develop` and PR HEAD:

1. **Correctness — CONFIRMED.** `LOCK_RENEWAL_TTL_SECS` had zero workspace callers on develop (`git grep` shows only its own definition at `precompact-flush/src/lib.rs:65`). On PR HEAD it is absent entirely (grep exit=1). Deleting it is the right fix. The canonical source `factory_lock_parse::TTL_SECONDS` (`crates/factory-lock-parse/src/lib.rs:43`, `pub const TTL_SECONDS: u32 = 2700`) exists and is Red-Gate-tested via `test_ttl_seconds_constant_equals_2700`. Removing the local literal satisfies Architecture Compliance Rule 7 / ADR-046 (no local TTL literal when a canonical source exists) and matches the sibling `stamp-state-timestamp` hook's single-source pattern.

2. **Test coverage — none needed.** The deleted const had no consumer, so there is nothing to test and no behavior to regress. The canonical `TTL_SECONDS` already carries assertion coverage. No coverage gap is introduced.

3. **Side effects / external import risk — NONE.** `precompact-flush` Cargo.toml declares `publish = false` (not publishable to crates.io), and no workspace Cargo.toml depends on it as a library for this const. Zero external-importer risk for the removed `pub const`.

### Non-blocking note (optional)
The canonical `TTL_SECONDS` is typed `u32`; the deleted `LOCK_RENEWAL_TTL_SECS` was `u64`. Irrelevant here (no consumer), but any future precompact-flush consumer should import `factory_lock_parse::TTL_SECONDS` and cast to `u64` where needed rather than reintroducing a local literal.
