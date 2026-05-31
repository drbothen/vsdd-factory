# S-15.17 LOCAL Adversary Cascade — Pass 3

**Date:** 2026-05-30
**Reviewer:** adversary (fresh-context)
**Target:** implementation diff 766ab7bc..3175ff14 (pass-2 fix burst 5e98ad7e/8694c947/3175ff14)
**Scope (BC-5.39.002):** validate-trajectory-tail-cell-completeness crate + hooks-registry.toml priority-158 entry + bats + fixtures. BC-5.39.009 v1.8.
**Prior passes:** Pass 1 = HIGH (0C/2H/4M/2L/1N); Pass 2 = LOW (0C/0H/0M/2L/1N). Streak entering pass 3: 0/3.

## Part A — Pass-2 Finding Verification

All 3 pass-2 findings CLOSED with load-bearing tests/structural changes, no paper-fixes, no regressions:
- F-P2-001 (CLOSED): first_table_cell() helper; site-6 selects Convergence Status row only when first non-empty pipe cell starts-with "convergence status" (case-insensitive). test_f_p2_001_convergence_status_only_in_notes_cell_not_selected + ..._first_cell_selected.
- F-P2-002 (CLOSED): decode_read_result monomorphized to concrete vsdd_hook_sdk::host::HostError (no generic <E: Debug>); both call sites pass HostError; test_f005 binds concrete OutputTooLarge variant; uniform fail-open preserved.
- F-P2-003 (CLOSED): advisory messages embed stable [PC7]/[PC8]/[PC9]/[PC10] tokens; 3 INDEX bats assert tokens (load-bearing, wording-drift-resistant).

## Part A — Overall Verdict

**Verdict: CLEAN** (0 CRITICAL / 0 HIGH / 0 MEDIUM / 0 LOW / 0 NITPICK). **Streak after pass 3: 1/3.**

## Part B — New Findings

NONE. Honest CLEAN. Confirmed correct: count==4 strict equality (LENGTH 3/4/5 cases); two-step marker-prefix (first ; OR newline, cross-line false-match defeated); path-component-walk .factory guard (not contains/starts_with/ends_with; non-factory STATE.md fails open EC-019/AC-23); INDEX cycle-guard secondary read + path-component-walk match; STATE cascade single Block (inv-8); advisory arms Continue+log_warn only (inv-6/9); lessons.md PC10 OUT-OF-SCOPE always Continue; fail-open all HostError + utf8 (inv-10/EC-020); MAX_BYTES 524288 u32 (inv-7); is_char_boundary (inv-11); no regex dep; trampoline __internal::run; POLICY 4 registry parity byte-identical (re-verified post pass-2 bats edits); POLICY 11 new tests call production fns; POLICY 3 clean; POLICY 16 ADR-017/018 present; all 24 ACs map to tests.

## Confirmed-Correct (carry forward to pass 4)
Entire load-bearing surface confirmed correct. Trajectory HIGH(9)→LOW(3)→CLEAN(0) = genuine convergence, not asymptotic-floor churn.

**Streak after pass 3: 1/3. Need passes 4 + 5 CLEAN for BC-5.39.001 convergence.**
