# S-15.17 LOCAL Adversary Cascade — Pass 1

**Date:** 2026-05-30
**Reviewer:** adversary (fresh-context)
**Target:** implementation diff feature/S-15.17-validate-trajectory-tail-cell-completeness vs develop (HEAD 4045061b: stubs 6ebf4fda + tests 78ca5d1f + impl d38319df + registry 4045061b)
**Scope (BC-5.39.002):** validate-trajectory-tail-cell-completeness crate (lib.rs, main.rs) + hooks-registry.toml priority-158 entry + 28 bats + 28 fixtures. BC-5.39.009 v1.8.

## Part A — Verdict

**Verdict: HIGH** (3-CLEAN streak resets to 0/3)

Finding count by severity:
- CRITICAL: 0
- HIGH: 2
- MEDIUM: 4
- LOW: 2
- NITPICK: 1

The load-bearing STATE.md Block arm (the META-LEVEL-30 cure) is CONFIRMED correct: strict count==4 equality (LENGTH=3 and LENGTH=5 both block), two-step marker-prefix scoping, path-component-walk .factory guard, cascade accumulation, fail-open on all HostError + UTF-8 decode failure, MAX_BYTES=524288 u32, trampoline via __internal::run, POLICY 4 registry parity (byte-identical path_allow=[".factory"] + priority 158 + tool="Edit|Write" + on_error="continue"). Findings concentrate in the advisory INDEX.md arm (PC7/PC8) — mis-implemented against the BC and tested only by exit-code assertions that cannot detect the mis-implementation (false-green cluster).

## Part B — Findings

**F-S15.17-LOCAL-P1-001 — HIGH — INDEX.md site 7 (PC8) checks the wrong row.** `check_index_sites` selects the global-bottommost markdown row via rfind. In canonical INDEX.md the Adversarial Review Summary table precedes the Convergence Status table, so the bottommost row is the Streak row, not the adv-review latest-pass row PC8 names. Spurious advisory on every well-formed INDEX.md; real adv-review row never inspected. TD-VSDD-059 paper-fix class (covered-looking, wrong region). Fix: scope site 7 to "## Adversarial Review Summary" table via rows_after_heading; strengthen fail-index-adv-table test to assert advisory text.

**F-S15.17-LOCAL-P1-002 — HIGH — INDEX advisory arm has no load-bearing test of advisory emission.** All INDEX tests assert only exit 0 + no blocking_plugins; none assert log_warn emitted/absent. Arm always returns Continue, so assertions hold for ANY path (fail-open, file never read, empty result, spurious warning). PC7/PC8/inv-6 false-green. Fix: assert dispatcher advisory log contains specific message on fail-* fixtures; assert absent on pass fixture.

**F-S15.17-LOCAL-P1-003 — MEDIUM — rows_after_heading doc-comment claims header-row skip the code doesn't perform.** Doc says skips header row; code skips only separators, pushes header as data row. Benign now (bottommost selection) but latent + misleading. Fix: implement header-skip or correct doc; add single-row-section unit test.

**F-S15.17-LOCAL-P1-004 — MEDIUM — UTF-8 decode test (AC-24/EC-020) not load-bearing.** pass-utf8-decode-failure-failopen.bats asserts only exit 0; cannot distinguish decode-Err path from successful read; fixture bytes ambiguous (renders as replacement char). Fix: assert "invalid UTF-8" advisory string; generate non-UTF-8 fixture via printf '\xff'.

**F-S15.17-LOCAL-P1-005 — MEDIUM — file-too-large test (AC-14) doesn't exercise HostError::OutputTooLarge; cites nonexistent compensating unit test.** Test triggers read-failure variant, not OutputTooLarge; comment claims a Rust unit test covers it but none exists (paper-fix attestation). Fix: add unit test mapping Err(OutputTooLarge)→Continue+log_warn via a testable seam, or correct the claim.

**F-S15.17-LOCAL-P1-006 — MEDIUM — is_separator_row accepts thin/empty data rows as separators.** All-whitespace/empty-cell data row dropped from rows_after_heading → extract_*_latest_row may return older row or None → false-negative on Block-severity STATE.md site (the silent-degradation class this hook prevents). Fix: tighten separator detection; add thin-row unit test.

**F-S15.17-LOCAL-P1-007 — LOW — extract_frontmatter_scalar block-scalar folded-> blank-line semantics not honored.** YAML-fidelity gap; no functional break (marker+arrows on one physical line in production). Fix: document the simplification or honor folded blank-line paragraph breaks.

**F-S15.17-LOCAL-P1-008 — LOW — extract_current_cycle strips #-comment before unquoting.** A quoted value containing # is wrongly truncated (BC says strip only when # not inside quotes). Fix: unquote first, then strip bare-form trailing comment; add unit test.

**F-S15.17-LOCAL-P1-009 — NITPICK — LENGTH=5 Block site_name omits AC-22's exact string.** Functionally correct (Block fires); cosmetic divergence from AC-22 verbatim site-name. Fix optional.

## Novelty Assessment
HIGH (pass 1; no prior LOCAL passes). Substantive findings, not wording nitpicks. F-001 genuine PC8 correctness defect; F-002/F-004/F-005 a coherent false-green cluster in advisory-arm test design; F-003 real doc-vs-code mismatch. STATE.md Block arm solid → fast convergence expected once INDEX advisory arm + tests corrected.

## Confirmed-Correct (carry forward to pass 2)
STATE Block arm count==4 strict; two-step marker-prefix (first ; OR newline boundary); path-component-walk .factory guard; INDEX cycle guard path-component-walk (not contains); fail-open all HostError + UTF-8 Err; MAX_BYTES 524288 u32; trampoline __internal::run; is_char_boundary guards; no regex dep; lessons.md PC10 OUT-OF-SCOPE always Continue; POLICY 4 registry parity byte-identical; POLICY 3 clean; ADR-017/018 traceability present.

**Streak after pass 1: 0/3. Routed to implementer fix burst.**
