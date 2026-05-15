---
document_type: architect-decision
level: ops
status: final
producer: architect
timestamp: 2026-05-15T00:00:00Z
subject: "TD #66 + TD #67 scope adjudication — split into S-15.04 (TD #66) + S-15.05 (TD #67)"
cycle: v1.0-brownfield-backfill
refs: [S-15.04, S-15.05, "TD #66", "TD #67"]
---

# Architect Decision — TD #66 + TD #67 Split (2026-05-15)

## Context

The original Section 12 Step 2 plan described both TD #66 and TD #67 "wrapped in S-15.02". However,
S-15.02 is already occupied by the Dispatcher cold-start optimization story (daemon mode + WASM AOT
cache; F5 pass-1 path-A follow-up per ADR-020). Cite drift was identified during the TD #74 post-merge
burst, requiring architect adjudication.

## Decisions

### TD #66 — trace_id field-name canonicalization

**Verdict A: Internal-log canonicalization only (NOT ABI-breaking external API change)**

Story: **S-15.04** — "Tighten bats trace_id assertion to canonical wire format (TD #66 closure)"

Rationale:
- PR #113 relaxed the bats grep as a workaround (`.` match instead of exact `trace_id` field name).
- The correct fix is to tighten the grep back to the canonical wire format AND add a negative assertion
  that confirms the old/wrong field name is NOT present.
- Scope: single-file bats edit. No production Rust code changes required.
- Effort: sub-day. This is a test-harness canonicalization, not a code change.
- ABI impact: none. The `trace_id` field name is already canonical in production output; only the bats
  assertion was relaxed as a workaround.

Implementation plan:
1. Update the relevant bats assertion from relaxed `.` grep to `trace_id` exact match.
2. Add negative assertion confirming the deprecated/wrong field name is absent.
3. No Rust source changes. No BC changes. No VP changes.

### TD #67 — 4 timing-flaky e2e tests (TC-4/5/7/9)

**Strategy B: Replace wall-clock sleeps with internal-log event observation**

Story: **S-15.05** — "De-flake TC-4/5/7/9 async timing tests via internal-log event observation (TD #67 closure)"

Rationale:
- TC-4, TC-5, TC-7, TC-9 are marked `#[ignore]` in `full_stack_plugin_invocation.rs` due to
  timing-dependent failures in CI environments with variable load.
- Strategy A (increase wall-clock timeouts) does not eliminate the non-determinism; just reduces
  frequency. Rejected.
- Strategy B (event observation via internal log) is the production-grade fix: instead of sleeping N ms
  and hoping the async operation completed, poll the dispatcher internal log for a sentinel event that
  confirms completion. This is deterministic and environment-independent.
- Requires a new `wait_for_log_event` helper in the test harness.
- Effort: multi-day. Requires: (a) helper implementation, (b) TC-4/5/7/9 rewrites, (c) `#[ignore]`
  removal, (d) CI validation that tests now pass reliably.

Implementation plan:
1. Implement `wait_for_log_event(path, event_type, timeout)` helper in test utilities.
2. Rewrite TC-4, TC-5, TC-7, TC-9 to use event observation instead of fixed sleep.
3. Remove `#[ignore]` from all 4 test cases.
4. Validate in CI over multiple runs to confirm flakiness eliminated.
5. No BC changes. VP updates may be needed if VP coverage gaps are discovered.

## Cite-Drift Fix

The original Section 12 Step 2 row and related narrative incorrectly referred to "S-15.02" as the
container for both TDs. The correct cite mapping after this adjudication:

| Location | Old cite | Correct cite |
|----------|----------|--------------|
| Section 12 Step 2 row | "wrapped in S-15.02" | "S-15.04 (TD #66) + S-15.05 (TD #67)" |
| Drift Items TD #66 | "DEFERRED to S-15.02" | "DEFERRED to S-15.04" |
| Drift Items TD #67 | "DEFERRED to S-15.02" | "DEFERRED to S-15.05" |
| Section 4 New Tier-A prose | "wrapped in S-15.02" | "S-15.04 + S-15.05" |
| Section 11 checklist step 4 | "dispatch TD #66 + TD #67 in S-15.02" | "dispatch per-story-delivery for S-15.04 + S-15.05" |
| Operating Mode section | "S-15.02 story authoring" | "S-15.04 + S-15.05" |
| frontmatter phase: | "pivot-to-S-15.02-tier-A" | "S-15.04-S-15.05-tier-A" |

## Story Independence

S-15.04 and S-15.05 are independent of each other (no blocking relationship). Both depend on S-15.01
(merged). Both are under E-15. Either can be dispatched first or in parallel.

Recommendation: dispatch S-15.04 first (sub-day; ships fast; simpler scope), then S-15.05. Parallel
dispatch is also acceptable if the team has bandwidth for both simultaneously.
