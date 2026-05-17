---
document_type: adversary-pass-report
level: ops
title: "S-15.09 LOCAL Adversary Cascade — Pass 2"
producer: adversary
timestamp: 2026-05-17T00:00:00Z
phase: per-story-step-4-local-adversary-cascade
story: S-15.09
pass: 2
verdict: HIGH
finding_count: { critical: 0, high: 2, medium: 2, low: 2, nitpick: 1, process_gap: 0 }
streak_3_clean: "0/3"
---

# S-15.09 LOCAL Adversary Cascade — Pass 2

## Part A — Findings

### F-P2-001 [HIGH] — `extract_trajectory_tail_line` banner-block anchoring produces false-positive trajectory-tail violation against real `.factory/STATE.md` (regression introduced by pass-1 F-P1-007 fix)

- **Severity:** HIGH (CRITICAL-adjacent — would block the very production target the hook exists to validate)
- **Category:** Spec-implementation gap; partial-fix regression; coverage gap
- **Location:** `crates/hook-plugins/validate-state-structure/src/lib.rs:352-373`, exercised via `validate_trajectory_tail` at line 454-477; target file `.factory/STATE.md:29`
- **Evidence:** `.factory/STATE.md:29` (inside `<!--...-->` SIZE BUDGET banner block delimited by `:23` and `:30`) contains: `D-430(a) compaction authorization: Pass-49 Commit E surgical compaction (363→310 lines) authorized retroactively per D-430(a); ...`. `lib.rs:355-363` scans banner block for any `→[0-9]` via `contains_arrow_digit_sequence` and returns the first matching line.
- **Issue:** Pass-1 F-P1-007 fix anchors trajectory-tail extraction to scan the SIZE BUDGET banner block first, falling back to the full document only if the banner contains zero arrow-digit sequences. This works for every test fixture because every fixture banner is hand-authored to contain NO `→` character. However, the real production target `.factory/STATE.md` contains the substring `(363→310 lines)` on line 29 *inside* the banner block. `contains_arrow_digit_sequence` returns `true` for line 29 because `→310` is arrow-followed-by-ASCII-digit. `extract_trajectory_tail_line` therefore returns line 29 and `count_arrow_digit_matches` returns 1 (only `→310` qualifies). The hook then emits `BlockWithFix` reading "trajectory-tail has 1 components; required LENGTH=4". The hook **falsely blocks every future Edit/Write to the real STATE.md**, neutering the entire validation chain. Orchestrator independently verified: `grep -nE '→[0-9]' .factory/STATE.md` confirms line 29 contains `(363→310 lines)` inside the banner block. The full document does contain canonical `→9→9→9→9` trajectory tails in body content, but those are never reached because the banner-block scan short-circuits on line 29. The unit test `test_BC_5_39_005_f_p1_001_real_state_md_banner_wc_passes` (lib.rs:1068) is mis-scoped: it tests `extract_banner_line_count`+`count_newlines` against real STATE.md but does NOT exercise `validate_trajectory_tail` or `on_post_tool_use` against the real file, so the regression is uncaught. Classic partial-fix regression.
- **Recommendation:** Strengthen the banner-block trajectory anchor so it accepts only lines that "look like" canonical trajectory tails — at minimum, require `count_arrow_digit_matches(line) >= 3` before claiming the line as the tail. Better: require the line to also match a `Trajectory`-prefix or ≥3 adjacent-component run predicate. Add a load-bearing unit test that calls `on_post_tool_use` against the actual `.factory/STATE.md`, asserting `None` (no violation). Add a bats fixture `fail-banner-has-narrative-arrow` that mirrors real STATE.md (banner with `(N→M lines)` narrative AND body containing canonical 4-component trajectory) and assert `Continue`.

### F-P2-002 [HIGH] — Real-STATE.md test coverage gap: `on_post_tool_use` is never exercised against the actual production target, leaving false-positive class undetectable

- **Severity:** HIGH (POL-11 no-test-tautologies concern; production-target binding gap)
- **Category:** Untested production path; test design defect
- **Location:** `crates/hook-plugins/validate-state-structure/src/lib.rs:1068-1102` (`test_BC_5_39_005_f_p1_001_real_state_md_banner_wc_passes`)
- **Evidence:** The only test that touches the actual `.factory/STATE.md` exercises ONLY `extract_banner_line_count` and `count_newlines`. It does NOT call `validate_dual_margin`, `validate_trajectory_tail`, or `on_post_tool_use`.
- **Issue:** Because the F-P1-007 fix to `extract_trajectory_tail_line` introduces banner-block anchoring whose correctness depends on banner-block contents, leaving `validate_trajectory_tail` untested against the real production target is what allowed F-P2-001 to ship undetected. Per POLICY 11 and the production-grade Self-Audit checklist, this is a coverage-gap that masks live defects. The story spec's AC-9..AC-12 also do not require any real-STATE.md integration assertion — a structural spec gap.
- **Recommendation:** Extend `test_BC_5_39_005_f_p1_001_real_state_md_banner_wc_passes` (or sibling-add) to call `on_post_tool_use` against real STATE.md OR at minimum call all three `validate_*` functions and assert each returns `None`. Additionally, document in the story spec an acceptance criterion (AC-13) requiring real-STATE.md test coverage so future modifications cannot regress.

### F-P2-003 [MEDIUM] — `cited_raw` field on `Violation` is dead structural plumbing — never serialized into block reason

- **Severity:** MEDIUM (TD-VSDD-059 paper-fix detection; structural drift)
- **Category:** Paper-fix; dead code; spec-implementation gap
- **Location:** `crates/hook-plugins/validate-state-structure/src/lib.rs:56-63`, `:484-500`
- **Evidence:** `emit_block` formats violations using only `v.description`. `cited_raw` is populated at every violation construction site but never consumed in the production hook path nor asserted in tests.
- **Issue:** Per TD-VSDD-059 and the production-grade Self-Audit checklist ("Did I paper-fix a finding by renaming, doc-commenting, or asserting-only when the real fix is structural?"), this is a recurrence-pattern smell. The field is unused load-bearing-claim scaffolding.
- **Recommendation:** Option A (preferred): wire `cited_raw` through `emit_block` (append `cited: "..."` per violation; assert in unit test). Option B: remove the field entirely. Pick one path; do not leave dead plumbing.

### F-P2-004 [MEDIUM] — Spec EC-007 / Continue-with-3-components contradiction: BC EC-007 enumerates exactly-4 components → Continue, EC-008 enumerates tail-absent → block, but spec is silent on banner-narrative-arrow class

- **Severity:** MEDIUM (spec test-vector vs implementation behavior — unanchored class)
- **Category:** Spec gap; coverage gap
- **Location:** `.factory/specs/behavioral-contracts/ss-05/BC-5.39.005.md:129-130` (EC-007/EC-008), `lib.rs:985-1009` (`test_BC_5_39_005_trajectory_tail_anchored_in_banner_block`)
- **Issue:** BC enumerates exactly-4 and absent; silent on "banner contains narrative arrow-digit sequence that is NOT a canonical trajectory tail." Real STATE.md occupies this case. The lib.rs test only exercises canonical-in-banner form, which is unrealistic.
- **Recommendation:** Add BC-5.39.005 EC-015 covering banner-narrative-arrow class. Update `extract_trajectory_tail_line` per F-P2-001 recommendation. Add a lib.rs unit test mirroring real STATE.md banner structure.

### F-P2-005 [LOW] — `count_arrow_digit_matches` byte-iteration variables retained alongside `char_indices()` iteration — mixed-paradigm code smell post F-P1-009 fix

- **Severity:** LOW (cosmetic / structural clarity)
- **Category:** Code quality; mixed paradigm
- **Location:** `crates/hook-plugins/validate-state-structure/src/lib.rs:418-445`
- **Issue:** F-P1-009 migrated outer iteration to `char_indices()` but inner walk still pokes raw bytes via `bytes[j].is_ascii_digit()`. Works correctly (ASCII digits are 1-byte UTF-8), but mixed paradigm increases cognitive load and risks future regression if extended to non-ASCII numerals.
- **Recommendation:** Either rewrite inner loop using `chars().take_while(...)` or add explicit doc-comment justifying mixed iteration. Document the contract.

### F-P2-006 [LOW] — `fail-no-banner` fixture: outcome-precision could be stricter than just status=2

- **Severity:** LOW (test thoroughness gap)
- **Category:** Test design — outcome-precision
- **Location:** `plugins/vsdd-factory/tests/validate-state-structure/fail-no-banner.bats:97-99`
- **Issue:** Fuzzy substring assertion `*"SIZE BUDGET banner"* || *"no SIZE BUDGET"*` does not assert exact violation count. The `emit_block` reason on this fixture will say "2 violation(s)" (banner-wc absent AND dual-margin absent), not "1". Stricter `*"2 violation(s)"*` assertion would catch regression where one path silently drops a violation.
- **Recommendation:** Add `*"2 violation(s)"*` to fail-no-banner; `*"3 violation(s)"*` to fail-no-banner-no-tail.

### F-P2-007 [NITPICK] — `pass-real-prose` fixture line 14 contains misleading filler comment

- **Severity:** NITPICK
- **Category:** Fixture clarity
- **Location:** `plugins/vsdd-factory/tests/fixtures/validate-state-structure/pass-real-prose/factory/STATE.md:14`
- **Issue:** Trailing comment "net -373 from pass-68; fixture is minimal" adds no test value and could confuse a future maintainer.
- **Recommendation:** Add intent-clarifying text: `canonical test claim — must match actual newline count`.

## Part B — Production-Grade Default Audit

- **F-P2-001** is "good enough for v1" — pass-1 fix was partial. Shipping a hook that misfires on its very target. Fix in scope.
- **F-P2-002** is synthetic-fixture-only validation that "passes" while the real target false-positives. Pass-1 added a partial real-STATE.md test for one function but stopped short of testing the others.
- **F-P2-003** is TD-VSDD-059 paper-fix pattern: field added with load-bearing comment but no load-bearing behavior.
- **No "TODO for architect"** anti-patterns observed.
- **Sibling-site sweep**: `validate-index-cite-refresh` (S-15.07) and `validate-burst-log` (S-15.11) used the same path-component-strict pattern; sweep applied. Production-grade pattern consistency intact.

## Part C — Self-Application Audit (META-LEVEL)

- All evidence is verbatim file:line excerpts. No pseudocode.
- Real `.factory/STATE.md` Grep invocations are captured-stdout-class evidence.
- F-P2-001 + F-P2-002 are paired — spec gap (no production-target test obligation) enabled the implementation defect. Both surfaced.
- Partial-fix regression discipline working: pass-1 fix over-corrected, pass-2 catches. Fresh-context adversary mechanism (BC-5.39.001) validated.
- POLICY 4 semantic anchoring check on diff: story spec body line 167 cites "SS-05 Pipeline Orchestration" — ARCH-INDEX:247 confirms canonical name match. PASS.

## Verdict & Streak

- Pass-1 verdict: HIGH. Streak entering pass-2: 0/3.
- Pass-2 verdict: **HIGH** (F-P2-001 + F-P2-002 both HIGH). Streak: **0/3**.
- Mandatory fix-burst before pass-3 per BC-5.39.001.

## Fix-burst routing (orchestrator routed; complete at time of persistence)

- implementer @ `64e0a83f` — F-P2-001/002 impl/003/005/007 closed (5 micro-commits; tightened predicate to ≥3 adjacent arrow-digit components; cited_raw wired via Option A; full-surface real-STATE.md test added; pass-real-state-md-snapshot bats fixture; 23/23 S-15.09 bats; 4-gate PASS)
- test-writer @ `2080a871` — F-P2-006 closed (exact violation count assertions on fail-no-banner + fail-no-banner-no-tail; counts 2 + 3 match EC-014)
- product-owner @ `465fc419` factory-artifacts — F-P2-004 closed (BC-5.39.005 v1.0→v1.1; EC-015 + Invariant 9 + test vector + within-BC VP row added; POLICY 14 parity)
- story-writer @ `d34e73e2` factory-artifacts — F-P2-002 spec half closed (story v1.1→v1.2; AC-13 + Token Budget updates; POLICY 14/17 parity)
- state-manager — this persistence commit
- Adversary pass-3 dispatched after this persistence commit.

## Closure verification

Implementer's new full-surface test `test_BC_5_39_005_full_validation_against_real_state_md` reads `.factory/STATE.md` and calls all three `validate_*` functions; all return `None`; hook continues against real production target. F-P2-001 + F-P2-002 closed STRUCTURALLY, not paper-fixed.
