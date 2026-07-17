# PR #670 — Delta Re-Review Addendum

- **PR:** #670 — `feat(S-19.07): verify-factory-lock read_prefix migration (BC-4.13.001 v1.17 Phase-B)`
- **Base of delta:** `e7b518e7` (prior APPROVE review — see `pr-review-670.md`)
- **HEAD reviewed:** `26d90ac6`
- **Delta commits:**
  - `b3900d6f` — pr-review follow-ups: stable ADR/VP cites, remove dead fixture, named prefix constant (F-670-01/04/05/06/07)
  - `26d90ac6` — stable story-section anchor in T-006/T-007 mapping comments (TD-VSDD-091)
- **Review date:** 2026-07-17
- **Scope:** `git diff e7b518e7..26d90ac6` only + TD-VSDD-059 closure verification (no paper fixes).
- **Verdict:** **DELTA-CLEAN**

---

## Delta composition

Two files touched: `crates/hook-plugins/verify-factory-lock/src/lib.rs` and `docs/demo-evidence/S-19.07/evidence-report.md`. The delta is exclusively comments/doc-comments/error-message strings/metadata, plus one named-constant introduction and one dead-code removal. No production control-flow or value change. Behavior-preserving as required.

## Per-finding closure verdicts

| Finding | Verdict | Evidence |
|---------|---------|----------|
| **F-670-01** (MEDIUM — volatile/inconsistent cites) | **CLOSED** | All `ADR-025 §D15 v1.17` occurrences in `lib.rs` (≥9 sites: doc-comments, Step-2 comment, and the `max_bytes != 262144 → Err` closure strings) are now the stable `ADR-025 §Decision 15` form. `VP-095 v1.3` → `v1.5` (lib.rs:1803). evidence-report.md volatile site fixed (`ADR-025 §Decision 15 v1.18` → `ADR-025 §Decision 15`; VP-095 cite now v1.5). Post-delta grep of both files for `v1.17\|v1.18\|v1.19\|v1.3(non-digit)\|§D15`: **zero** ADR-025/VP-095 volatile pins remain. The residual `BC-4.13.001 v1.17` cites in evidence-report.md are the BC contract version (the PR's canonical scope identifier, in the PR title) — out of F-670-01 scope and correctly retained. |
| **F-670-04** (dead fixture) | **CLOSED** | `state_md_malformed_expires_at()` fully removed. `git grep state_md_malformed_expires_at` across `crates/`: no matches — no dangling references. The malformed-timestamp path remains covered by `test_..._parse_iso8601_errors_on_invalid_timestamp` (function-level), so no coverage lost. |
| **F-670-05** (test-name vs task-role mapping) | **CLOSED** | Mapping comments added above `test_S1907_T003_...` (→ Test Plan row T-006) and `test_S1907_T004_...` (→ row T-007). Version-free, anchored to "story §Test Plan". Verified factually accurate against the story spec Test Plan (`.factory/stories/S-19.07-...md` rows T-006/T-007): T-006 → `test_S1907_T003_read_prefix_262144_large_fixture_foreign_lock_blocks`, T-007 → `test_S1907_T004_..._no_lock_continues_without_warns` — exact match. |
| **F-670-06** (mutation-branch comment overstatement) | **CLOSED** | Comments reworded from "MUTATION CHECK / mutates the bound" to "OLD-CAP REGRESSION BRANCH … simulates the bug's observable effect by hardcoding `fixture[..8192]` … does NOT mutate the production bound — real proof is `max_bytes != 262144 → Err` in ASSERTION (a)". Verified against actual code: the old-cap closure ignores `max_bytes` and returns `Ok(fixture_clone[..8192].to_vec())` (hardcoded slice), while ASSERTION (a) does carry the `max_bytes != 262144 → Err` guard. Comment now factually describes what the branch proves. |
| **F-670-07** (bare literal at call site) | **CLOSED** | `pub const STATE_MD_PREFIX_BYTES: u32 = 262144;` introduced with an ADR-025 §Decision 15 doc-comment (and an explicit note that tests keep independent 262144 literals as an oracle). Used at the production call site in place of the bare literal; value and type (`u32`) identical → behavior-preserving. Test literals left hardcoded 262144 (intentional oracle, unchanged). |

F-670-02, F-670-03, F-670-08 were routing/advisory items (bats POLICY-21 exemption; platform-binary asymmetry; observability spec-intent) not targeted by these fix commits and not part of the delta — no regression either way.

## New-defect sweep

No new defects. Confirmed the delta introduces no behavioral change:
- The error-message strings that changed (`§D15 v1.17` → `§Decision 15`) live inside the `max_bytes != 262144 → Err` guard closures, which never fire on the happy path (guard always calls with 262144); no test asserts on those exact strings.
- Dead-code removal is a `#[allow(dead_code)]` test-module helper with no callers.
- Named constant equals the prior literal exactly.
- No new volatile version pins introduced (grep clean).

## CI status at 26d90ac6 (informational — not gating)

CI re-running at new HEAD. PASS: SAST (Semgrep), bats-darwin-leg, bats-wave-handoff, platforms-drift, validate. PENDING (in progress, no failures): cargo-host (ubuntu + macos), build-dispatcher (all 5 platforms), bats-full-suite (linux). Release-branch guardrail skipping (N/A). Orchestrator gates on CI separately.

## Verdict

**DELTA-CLEAN.** All five targeted findings (F-670-01/04/05/06/07) are genuinely closed with load-bearing edits — no paper fixes (TD-VSDD-059 satisfied). The prior APPROVE stands, now with the recommended MEDIUM (F-670-01) resolved. No new findings.
