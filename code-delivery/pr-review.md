# PR #779 — Final Fresh-Eyes Review

- **PR:** #779 `fix(policy15): empty/unresolvable range → SkippedEmptyRange inert-skip (exit 0)`
- **Branch:** `fix/policy15-empty-range-inert` → `develop`
- **Reviewed HEAD (covered_sha):** `591d3a0aeb9c32af8bd46e8af928e887b9ffd7ec`
- **Commits reviewed (3 on top of develop):**
  - `bc3b689d` — TDD red-gate tests for SkippedEmptyRange (ADR-040 v1.19 Ruling 9(f))
  - `910eb1b3` — implementation: empty/unresolvable range → SkippedEmptyRange inert-skip (exit 0)
  - `591d3a0a` — chore: update stale EmptyRange→SkippedEmptyRange doc comments/assertion messages

## Verdict: APPROVE

No BLOCKER or REQUEST_CHANGES findings.

## What this PR does

Fixes a false-FAIL in the `policy-15-attestation-location` CI gate per ADR-040 v1.19 Ruling 9(f):
- New `GateOutcome::SkippedEmptyRange` variant (exit 0) for empty/unresolvable commit ranges.
- `UnreachableCause::EmptyRange` retired; its two call sites now return `SkippedEmptyRange`.
- Guard ordering preserved: `StalePin` fires before range computation → stale pin still blocks (exit 2).
- `ci.yml` job gains `if: github.event_name == 'pull_request'` (PR-only event guard).
- 3 new TDD unit tests (red-first in `bc3b689d`, green in `910eb1b3`).

## Review focus — all verified

1. **Correctness — all call sites updated.** `run_gate()` unresolvable-base arm and `run_gate_inner()` empty-commits arm both return `GateOutcome::SkippedEmptyRange`. `UnreachableCause::EmptyRange` enum variant fully retired; grep across the crate shows zero remaining references to the variant — only doc/comment mentions of the string, all now describing SkippedEmptyRange. TD-VSDD-060 sibling-sweep complete.
2. **is_pass()/exit_code().** `SkippedEmptyRange` added to the `is_pass()` match arm; `exit_code()` derives from `is_pass()` → returns 0. Correct.
3. **identifier() exhaustiveness.** Exhaustive over all variants (Fail, PassWithActivations, PassZeroActivations, EmptyOrUnreachable(StalePin), EmptyOrUnreachable(UnmeasurableDiff), SkippedEmptyRange) with NO `_` wildcard — the deliberately-absent `#[non_exhaustive]` compile-safety property is preserved. SKIP prefix ("SKIP: empty or unresolvable commit range — inert...") is distinct from PASS/FAIL/EMPTY. `main.rs` match also uses explicit arms; comment correctly updated 5th→6th variant.
4. **Guard ordering.** `StalePin` still wins when both crate-absent AND range-empty. `test_run_gate_guard1_stale_pin_beats_unresolvable_base` and `test_guard_ordering_stale_pin_beats_empty_range` UNCHANGED in assertion logic and pass. Guard 1 fires before merge-base computation.
5. **TDD red evidence.** `bc3b689d` adds the two `test_adr040_v119_*` tests referencing `GateOutcome::SkippedEmptyRange` before the variant exists → legitimate compile-failure red. Variant + logic land green in `910eb1b3`.
6. **Cleanup commit.** `591d3a0a` updates 4 stale references (lib.rs test docs, main.rs, binary_integration_test.rs module + test docs). All now read SkippedEmptyRange.
7. **ci.yml `if:` placement.** At JOB level (line 358, under `name:`/`runs-on:`, above `steps:`), not step level. Confirmed.
8. **No AI attribution.** Commit messages checked; zero `Co-Authored-By`/Claude/robot references.

## Local verification

- `cargo test -p policy15-attestation-gate --all-targets` → 24 lib + 5 integration tests pass, 0 failed.
- `cargo clippy -p policy15-attestation-gate --all-targets -- -D warnings` → clean.
- `cargo fmt --check` → clean.

## Findings

- **INFO (non-blocking).** The ci.yml comment correctly notes that branch protection for the `policy-15-attestation-location` required check must be configured for `pull_request` contexts only, so the job being SKIPPED on push events is not treated as a pending/failed required check. This is a repo-settings action the human must apply after merge; it is properly surfaced inline (not a forbidden defer). Recommend confirming branch-protection config post-merge so post-merge pushes to `develop` aren't blocked waiting on a check that no longer runs on push.

covered_sha: 591d3a0aeb9c32af8bd46e8af928e887b9ffd7ec
