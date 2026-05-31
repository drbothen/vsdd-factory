# S-15.17 LOCAL Adversary Cascade — Pass 2

**Date:** 2026-05-30
**Reviewer:** adversary (fresh-context)
**Target:** implementation diff 766ab7bc..HEAD (HEAD 95b69b51; pass-1 fix burst bccdfcf8/35e39618/5b6f338b/95b69b51)
**Scope (BC-5.39.002):** validate-trajectory-tail-cell-completeness crate + hooks-registry.toml priority-158 entry + bats + fixtures. BC-5.39.009 v1.8.
**Prior pass:** Pass 1 = HIGH (0C/2H/4M/2L/1N), streak 0/3.

## Part A — Pass-1 Finding Verification

| Finding | Pass-1 sev | Verdict | Evidence |
|---|---|---|---|
| F-001 | HIGH | CLOSED | check_index_sites now rows_after_heading(content, "## Adversarial Reviews").last(); site-6 binds Convergence Status data row not heading line. Both PC7/PC8 target real data rows. |
| F-002 | HIGH | CLOSED | fail-* INDEX bats assert per-site advisory substring present + the other absent; pass-index asserts both absent. Load-bearing. |
| F-003 | MEDIUM | CLOSED | rows_after_heading doc matches code; test_F003 pins header-inclusion behavior. |
| F-004 | MEDIUM | CLOSED | fixture printf-generated non-UTF-8 in setup; bats asserts "invalid UTF-8" advisory → proves String::from_utf8 Err branch ran. |
| F-005 | MEDIUM | CLOSED | test_F005_decode_read_result_output_too_large_maps_to_advisory constructs Err(HostError::OutputTooLarge); bats read-failure corrected to canonical basename. |
| F-006 | MEDIUM | CLOSED | is_separator_row requires >=1 '-' AND every cell matches :?-+:?; test_F006 thin-row added. |
| F-007 | LOW | CLOSED | block-scalar join documented marker-detection-oriented. |
| F-008 | LOW | CLOSED | extract_current_cycle unquotes first then strips bare-form #-comment; test_F008 added. |
| F-009 | NITPICK | ACCEPTED | site_name kept (matches BC verbatim across all 5 sites); AC-22 parenthetical is documentary. |

All 9 pass-1 findings hold (8 CLOSED + 1 accepted). No paper-fixes detected.

## Part A — Overall Verdict

**Verdict: LOW** (0C / 0H / 0M / 2L / 1N). Streak resets to 0/3 (any non-CLEAN finding resets per BC-5.39.001). Fix burst was high-quality; no regression in STATE.md Block arm or corrected INDEX advisory arm.

## Part B — New Findings

**F-S15.17-LOCAL-P2-001 — LOW — Convergence Status row selected by whole-line substring.** check_index_sites site-6 (PC7) matches "convergence status" anywhere in line; a future row mentioning it in a Notes cell could mis-select. Fix: anchor to first data cell (cells[0] equals/starts-with "Convergence Status"). No live false-match (one such row today).

**F-S15.17-LOCAL-P2-002 — LOW — decode_read_result<E: Debug> generic seam weakens OutputTooLarge test's production binding.** The F-005 seam is generic; test exercises mapping but doesn't statically bind to production HostError. POLICY 11-adjacent. Fix: monomorphize seam to HostError, or add a test driving the actual production error path. Behavior correct today (uniform HostError→Continue+log_warn).

**F-S15.17-LOCAL-P2-003 — NITPICK — INDEX advisory bats assert on prose substrings (wording-drift fragility).** Future log_warn reword breaks tests though behavior correct. Optional: assert on stable site-ID token.

## Confirmed-Correct (carry forward to pass 3)
STATE Block arm count==4 strict (LENGTH=3 and 5 block); two-step marker-prefix (first ; OR newline); INDEX PC7/PC8 bind data rows; cycle-guard path-component-walk (not contains); fail-open all HostError + utf8; MAX_BYTES 524288 u32; is_char_boundary; no regex; trampoline __internal::run; lessons.md PC10 OUT-OF-SCOPE always Continue; POLICY 4 registry parity byte-identical (re-verified post fixture rewrite); POLICY 3 clean; ADR-017/018 present; new unit tests call production fns (POLICY 11 pass modulo F-P2-002 nuance).

**Streak after pass 2: 0/3 (LOW resets). Routed 3 minor findings to implementer; expect pass 3 CLEAN. Trajectory HIGH(9)→LOW(3).**
