# Delivery Record — S-17.02

**Story:** S-17.02 v1.5 — verify-factory-lock WASM guard crate + registry entries (D1+D2+D9 guard bats)
**PR:** #182
**Branch:** feature/S-17.02-verify-factory-lock-wasm-guard
**Merged:** df4f26b8 to develop 2026-06-11
**CI Run:** 27331109884

## Delivery Metrics

| Metric | Value |
|--------|-------|
| Unit tests (final) | 23 green |
| Bats tests (final) | 13 green |
| LOCAL adversary trend | 1H+2M+4L → 1M → 0 → 0 → 0 (3-CLEAN streak achieved) |
| pr-reviewer verdict | APPROVE (Cycle 1; 0 blocking 0 non-blocking) |
| Security scan | CLEAN |
| CI jobs | all-green: cargo-host ubuntu+macos, 5× build-dispatcher cross-compile, bats 13/13 |

## Adversary Convergence Summary

| Pass | Findings | Verdict | Action |
|------|----------|---------|--------|
| adv-pass-1 | 1H+2M+4L (7 total) | HIGH | Remediation: story v1.0→v1.1 (H1 env_allow footgun; M2 boundary `>`→`>=`; other fixes) |
| adv-pass-2 | 1M | MEDIUM | Remediation: story v1.1→v1.2 (M residual boundary semantics in ACs/ECs) |
| adv-pass-3 | 0 | CLEAN | Streak 1/3 |
| adv-pass-4 | 0 | CLEAN | Streak 2/3 |
| adv-pass-5 | 0 | CLEAN | Streak 3/3 — BC-5.39.001 3-CLEAN SATISFIED |

## Key Findings Closed

### H1 — env_allow footgun (CRITICAL — silent no-op path)

**Finding:** `verify-factory-lock` story spec omitted `env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]` from the `exec_subprocess` capability block. Without it, the dispatcher's `env_clear()` strips HOME+GIT_CONFIG_GLOBAL+XDG_CONFIG_HOME. `git config user.email` falls back to empty string. `IdentityResolutionFailed` hook returns `HookResult::Continue` — the lock is silently never enforced against foreign-holder blocks. This is the 3rd silent-no-op footgun vector for verify-factory-lock (after async=false and deny-by-default capability enumeration). ADR-025 amended v1.2→v1.3 to enumerate this as footgun vector 3.

**Fix:** Added `env_allow = ["HOME", "GIT_CONFIG_GLOBAL", "XDG_CONFIG_HOME"]` to capabilities block in story spec + AC/EC traces + BC-4.13.001 v1.2 PC7 + EC-016. ADR-025 v1.3 documents the footgun. ARCH-INDEX v2.20 reflects ADR-025 v1.3.

### M2 — Boundary semantics fix

**Finding:** Story spec used `now > expires_at` (strict greater-than) for expiry checks. The correct predicate is `now >= expires_at` — the instant `now == expires_at` is expired (block condition). The precondition for blocking was `now < expires_at` (not `now <= expires_at`). Two separately inconsistent predicates.

**Fix:** BC-4.13.001 v1.2 corrected PC1 (`now < expires_at` for block condition) and PC2 (`now >= expires_at` for expired/pass condition). Story spec ACs and ECs updated to `>=` / `<` consistently. STORY-INDEX v3.89.

## Files Delivered

| File | Action |
|------|--------|
| `crates/hook-plugins/verify-factory-lock/src/lib.rs` | CREATED — WASM guard crate (D1) |
| `crates/hook-plugins/verify-factory-lock/src/identity.rs` | CREATED — identity resolution with env_allow paths |
| `crates/hook-plugins/verify-factory-lock/src/lock.rs` | CREATED — factory_lock frontmatter parser + expiry check |
| `crates/hook-plugins/verify-factory-lock/Cargo.toml` | CREATED — crate manifest |
| `plugins/vsdd-factory/hooks-registry.toml` | MODIFIED — verify-factory-lock PreToolUse entry (D2; async=false; env_allow block) |
| `plugins/vsdd-factory/tests/verify-factory-lock.bats` | CREATED — 13 bats tests (D9 guard bats) |

## POL-14 Auto-Promotion

BC-4.13.001 lifecycle_status: draft → **active** on this PR merge per POLICY 14.
BC-5.40.001 remains active (promoted at S-17.01 merge).
BC-6.23.001 remains draft (S-17.03 not yet merged).

## 4-Index Versions at Delivery

| Index | Version Before | Version After |
|-------|---------------|---------------|
| BC-INDEX | v2.67 | v2.70 |
| STORY-INDEX | v3.88 | v3.90 |
| ARCH-INDEX | v2.19 | v2.20 |
| VP-INDEX | v2.06 | v2.06 (unchanged) |

## Issue Status

Issue #170 **partially closed** — S-17.01 (W1) and S-17.02 (W2) delivered. S-17.03 (W3 — /factory-lock+/factory-unlock skills + /factory-health) remains draft. Issue #170 stays open until S-17.03 merges. E-17 progress: 2/3 stories merged.

**Next:** S-17.03 test-writer Red Gate on feature/S-17.03-factory-lock-skills (E-17 Wave 3).
