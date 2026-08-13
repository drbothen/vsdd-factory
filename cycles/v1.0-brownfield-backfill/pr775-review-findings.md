---
document_type: pr-review-finding-set
level: ops
title: "S-21.09 PR #775 Review — Test-Quality Findings (Vacuity / Tautology / False-Narrative Rubric)"
story_id: "S-21.09"
pr: 775
producer: pr-manager (pr-reviewer + code-reviewer chain)
consumer: state-manager
phase: D-983-CONVERGENCE-RETRACTION-AND-PR775-TEST-QUALITY-FIX-BURST
date: 2026-08-13
scope: >
  crates/factory-dispatcher/tests/bundle_orphan_check.rs — reviewed as a PR diff
  (feature/S-21.09 @ 6ae075a6, PR #775 targeting develop), NOT a numbered LOCAL
  adversary pass. Dispatched via pr-manager's pr-reviewer + code-reviewer chain
  after LOCAL cascade convergence (D-982), independently of the 19-pass LOCAL
  BC-5.39.001 cascade and the D-977 exhaustive mutation-completeness audit.
disposition: >
  8 weak-assertion findings (F1-F8: vacuous, tautological, or narrative-mismatched
  test bodies) + 1 MAJOR code-quality finding (git-fixture duplication) + 3 minor
  findings, all fixed in test-writer commit c9cccea9 on feature/S-21.09. Every
  fix empirically re-verified (mutation applied locally → target test alone goes
  RED → mutation reverted). Suite remains 51 tests T-006..T-056 green (no IDs
  added or removed); fmt/clippy/full workspace suite clean. Net -318 lines
  (926 changed: 304 insertions, 622 deletions) via extracted git-fixture helpers.
covered_sha: 6ae075a6d6d197ac56182e04de93ffffab69c3dd
fixed_sha: c9cccea93488d2565feccd96d0b0482634509ec8
closes:
  - F1
  - F2
  - F3
  - F4
  - F5
  - F6
  - F7
  - F8
  - "code-quality-MAJOR-git-fixture-duplication"
routes:
  - "D-982 convergence RETRACTED — see decision-log.md D-983"
---

# S-21.09 PR #775 Review — Test-Quality Findings

## Why this artifact exists

BC-5.39.001's TRUE 3-CLEAN convergence, declared at D-982 (LOCAL adversary passes
17/18/19, all CLEAN, plus the D-977 exhaustive mutation-completeness audit), was
scoped to those reviewers' rubrics: gate-logic correctness, determinant isolation
(POLICY 13), traceability, count-parity, SHA currency, and disclosed-residual
honesty. None of those passes, nor the mutation audit, included an explicit
**vacuity check** (does a plausible mutant make this assertion RED?), **tautology
check** (does the test exercise the SAME code path it asserts about?), or
**narrative-accuracy check** (does the claimed mutant actually produce the
asserted output?) against the *test file's own assertions and comments* —
as opposed to the production gate logic those assertions target.

pr-manager's pr-reviewer + code-reviewer chain on PR #775 ran exactly that lens
as a fresh-eyes PR diff review and found 8 test-quality weak-assertion findings
plus 1 MAJOR code-quality finding the 19-pass LOCAL cascade and the mutation
audit had missed. This is **not** a numbered LOCAL adversary pass (it used a
different rubric and a different dispatch mechanism — PR diff review, not
fresh-context spec/impl adversarial review) — it is recorded here as the
artifact of record for the PR-review finding set that triggered the D-982
convergence retraction (D-983).

## Test-quality findings (F1-F8)

| ID | Location | Class | Description | Disposition |
|----|----------|-------|--------------|-------------|
| F1 | T-007 L1362-1373 | Tautological | Builds `format!("ORPHAN: {}", name)` from a collection already proven (L1354) to contain `name`, then asserts the string it just built is present. No production code emits `ORPHAN:` lines (`collect_orphans_dual` returns bare filenames) — the assertion is checked against a test-local `format!`, not the function's real return value. Pre-existing S-19.04 code on `develop`, untouched by this PR's diff (`git diff` shows no T-007 hunks); flagged as in-diff-scope for THIS fix burst per the production-grade default (fix in scope, not defer). | Fixed — tightened to exact-set `assert_eq!` against `collect_orphans_dual`'s real return value. |
| F2 | T-048 (module docstring L4740-4742 + test body) | Wrong mutation-kill attribution | The M4 (`>=+2→+1`) mutation-kill comment credited "the GATED assertion for `hook-plugins`" with catching the mutant, but `"hook-plugins"` is classified `UNGATED-DECLARATION` in the 18-candidate table, never exercised by the GATED branch — the catching assertion is actually `extract_result.is_none()` in the UNGATED arm. | Fixed — M4 attribution corrected to cite the actual UNGATED-arm assertion (`c9cccea9` story v1.29 sync). |
| F3 | T-026(a) L2644 | Vacuous | `assert!(!refs.contains("ghost-absolute.wasm"))`. Post-pass-9, `extract_hook_plugin_name` returns `joined_parts[expected_depth..].join("/")` and can never return a bare basename — this assertion can never fail regardless of gate correctness. The adjacent `refs.is_empty()` (L2652) is the load-bearing check. | Fixed — vacuous `.contains()` check removed; `is_empty()` retained as the sole (already load-bearing) assertion. |
| F4 | T-026(b) L2693 + attached mutation narrative | Vacuous + false narrative | `assert!(!refs_depth.contains("evil.wasm"))` — same vacuity as F3 (full paths returned, never bare basenames). Worse: the attached comment claimed deleting the gate-2 prefix loop yields `{"evil.wasm"}`; empirically, it yields `{"hook-plugins/evil.wasm"}`. The dead assertion was credited with a kill only `is_empty()` (L2703) actually achieves. | Fixed — vacuous check removed; code comment corrected to state the true mutant output (`hook-plugins/evil.wasm` admitted, i.e. the gate-2 deletion admits `hook-plugins/evil.wasm`, not a bare basename). |
| F5 | T-055 (malformed-TOML assertion) | Vacuous against a coarser mutant | The malformed-TOML `assert!(result.is_empty())` is empty-by-construction — a whole-function `detect_ungated_declarations` body replaced with `Vec::new()` (globally inert) would ALSO pass it, since malformed input is empty under both the live fail-open arm and an inert function. | Fixed — added a positive control in the same test: valid TOML containing a genuine ungated declaration (`ghost-bare.wasm`) asserted non-empty; a whole-function `Vec::new()` mutant now fails this second assertion. |
| F6 | T-054 `#[should_panic(expected = "but production requires 1")]` | Over-broad `should_panic` | The substring matches ANY value ≠ 1, so `.unwrap_or(0)` / `.unwrap_or(2)` mutants also panic and the test still passes — only `.unwrap_or(1)` is genuinely killed. Does NOT invalidate the underlying SURV-04 closure (the security property "absent key must not silently coerce to production-required value" is still enforced); the test's self-description was overstated. | Fixed — pinned to `schema_version=-1 but production requires 1`, tightening the match to the specific fail-closed sentinel value. |
| F7 | T-015 | NIT — indent mismatch | Comment claims the assertion "locks the string format" verbatim, but asserts `"MISSING: hook-plugins/hooks-only.wasm"` without the two-space indent the production format string `"  MISSING: {}"` actually emits. (T-021 does include the indent, so the indent IS pinned elsewhere — just not where T-015 claims.) | Fixed — assertion now pins the two-space `  MISSING:` form. |
| F8 | T-048 (see F2) | (folded into F2) | Same root cause as F2 — the biconditional `extract.is_some() ⟺ detect.is_empty()` was partly definitional/tautological for UNGATED rows, since `detect_ungated_declarations` decides its UNGATED classification by internally calling `extract_hook_plugin_name(...).is_none()`. The paired `extract`/`detect` assertions restated the same code path rather than cross-checking two independent implementations (the independence that pass-11's single-copy refactor removed). | Fixed — collapsed the paired assertions into a single exact-vec-equality check against `detect_ungated_declarations`'s own output (the function `run_t012_gate` actually calls), which subsumes the former "other identifier absent" check via exact-set equality and removes the tautological framing. |

## Code-quality findings

| Severity | Description | Disposition |
|----------|-------------|-------------|
| MAJOR | ~800 lines of byte-for-byte duplicated git-fixture bootstrap (`git init` / `git add` / `git commit` invocation sequences) and comment-filtering boilerplate repeated across T-013/T-014 and the nine git-backed T-0NN fixtures (T-030, T-034, T-036, T-037, T-039, T-049, T-052, T-053, T-054). | Fixed — extracted `git_init_fixture` / `git_add_all_fixture` / `git_commit_fixture` + `fixture_body_without_comments` helpers, applied across all 9 git-backed tests. Net -318 lines (926 changed: 304 insertions, 622 deletions). |
| minor | T-013/T-014 near-duplicate fixture-setup bodies. | Fixed — subsumed by the same helper extraction above. |
| minor | PR #775 description off-by-one: "Total suite (T-006..T-056 + registry.rs unit) \| 51/51" — T-006..T-056 is itself exactly 51 (6 pre-existing + 45 new); adding the `registry.rs` unit test makes 52. The 51/51 figure is correct for `bundle_orphan_check.rs` alone; the row label double-counts. | Routed — PR #775 description correction is scoped to the PR re-review after re-convergence (see STATE.md §8); not a test-file change. |
| minor | Module-docstring test-plan table duplication (redundant rows describing the same T-0NN fixture pattern across multiple preamble sections). | Fixed — folded into the same fixture-extraction pass; docstring rows deduplicated to reference the shared helpers. |

## Empirical re-verification

Every F1-F8 fix was verified per TD-VSDD-059 (paper-fix detection — implementer
self-disclosure is not authoritative): the specific mutation the finding
describes was applied locally to a scratch copy, the corrected test alone
was confirmed to go RED under that mutation, then the mutation was reverted.
This matches the same empirical-verification discipline the D-977 mutation
audit itself established (`L-BB-mutation-kill-attestation-requires-empirical-verification`).

Suite count unchanged: 51 tests T-006..T-056 (45 S-21.09-owned + 1
`registry.rs` unit test), no IDs added or removed. `cargo fmt --check --all`,
`cargo clippy --workspace --all-targets -- -D warnings`, and
`cargo test --workspace --all-targets` all clean at `feature/S-21.09` HEAD
`c9cccea9`.

## Why the LOCAL cascade and mutation audit missed this class

The 19-pass LOCAL adversary cascade (BC-5.39.001) and the D-977 exhaustive
mutation-completeness audit both independently re-derived **gate/production
logic** soundness — determinant isolation, fail-open arm pinning, count
parity, SHA currency — repeatedly and correctly. Their rubric, however, did
not include a dedicated pass asking, for each individual test assertion:
"is this specific assertion itself falsifiable by a plausible mutant, or
does it restate the implementation it's meant to verify?" A test can sit
inside a suite that is holistically sound (every determinant IS isolated by
SOME assertion in the suite) while individual assertions within it are
vacuous, tautological, or attached to an incorrect narrative — because the
suite's aggregate correctness was verified, not each assertion's individual
falsifiability. See `L-BB-...` lesson appended to `lessons.md` (this burst)
and decision-log.md D-983 for the full rubric-gap codification.

## Disposition summary

All 8 test-quality findings (F1-F8) and the MAJOR code-quality finding are
FIXED in test-writer commit `c9cccea9`. Two minor findings (PR description
off-by-one, module-docstring duplication) are FIXED / ROUTED as noted above.
This finding set is the direct cause of the D-982 convergence RETRACTION
recorded in decision-log.md D-983 — see `INDEX.md` S-21.09 LOCAL Adversary
Reviews Convergence Status for the retraction record and the reopened
streak (0/3), with LOCAL adversary pass-20 (strengthened rubric: vacuity +
tautology + mutation-narrative-accuracy checks) recorded as NEXT.
