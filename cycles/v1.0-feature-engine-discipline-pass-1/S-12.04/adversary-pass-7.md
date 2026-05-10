# Adversarial Review — S-12.04 Pass 7

## Metadata
- Story: S-12.04 WASM Resolver Loading, Lifecycle, and Error Isolation
- Branch SHA reviewed: c3d430e4
- Pass: 7
- Reviewer: adversary (fresh context)
- Classification: MEDIUM
- Within-story finding count: 4 (1 MEDIUM, 2 LOW, 1 NITPICK)
- Recommendation: PROCEED_TO_FIX

## Findings

### F-S12.04-P7-001 — MEDIUM — resolver.merge_collision HOST_ABI table omits session_id
**Files:** HOST_ABI.md:1163-1170; executor.rs:571 wires `.with_session_id`
**Description:** Pass-6 fix landed 7-field table for resolver.error but left sibling resolver.merge_collision at 6 fields. Same drift pattern as F-P6-001. Implementation emits session_id but doc claims it doesn't.
**Suggested:** Add session_id row to resolver.merge_collision field table.

### F-S12.04-P7-002 — LOW — resolver.not_found has NO field table in HOST_ABI; comment self-defers
**Files:** HOST_ABI.md:896 (prose only); executor.rs:495-499 (deferral comment)
**Description:** Comment self-defers to S-12.06 but the gap is localized to S-12.04's own artifacts (HOST_ABI.md is in this story's diff). Comment also stale: claims 3 fields, code emits 4.
**Suggested:** Add full resolver.not_found field table in HOST_ABI (4 fields: resolver_name, trace_id, session_id, plugin_name) — same pattern as F-P6-001 for resolver.error.

### F-S12.04-P7-003 — LOW — resolver.not_found integration test missing positive-coverage assertions [POL-11 sibling-gap]
**Files:** executor_resolver_integration.rs:380-395 (ac005 test)
**Description:** Sibling test for resolver.error has positive-coverage for trace_id + session_id + plugin_name (P4/P5/P6 fixes). Resolver.not_found test asserts only `resolver.not_found`, `unknown`, `not-found-hook` literals — no provenance triplet check.
**Suggested:** Add `assert!(all_log_content.contains("resolver-test-trace"))` and `assert!(all_log_content.contains("sess-resolver-test"))` parallel to the F-P4-005/F-P5-002/F-P6-002 pattern.

### F-S12.04-P7-004 — NITPICK — Stale comment in executor.rs:497 says "3 fields" but emits 4
**Files:** executor.rs:497
**Description:** Comment reads "Wire format: { resolver_name, trace_id, plugin_name }" but lines 502-512 emit 4 fields including session_id (added by F-P4-001).
**Suggested:** Update comment to list 4 fields.

## Cross-cutting observations

- Pass-6 fix-burst landed correctly for resolver.error (verified): HOST_ABI 1107-1115 has all 7 BC-4.12.004 v1.2 PC2 fields.
- BC-4.12.004 v1.2 body↔HOST_ABI sync confirmed for resolver.error.
- [process-gap] POL-11 sibling-coverage convention: each new event type added to executor.rs should ship with (a) HOST_ABI field table, (b) integration test with positive-coverage for every provenance field. Pattern was applied retroactively to resolver.error across passes 4-6 but resolver.not_found and resolver.merge_collision did not get the same treatment. Codifying as a sibling-event-coverage rule (lessons-codification.md or CI check) would prevent the 3-pass regression cycle observed for resolver.error.

## Convergence assessment

- Within-story findings: 4 (1 MEDIUM, 2 LOW, 1 NITPICK)
- Severity floor: MEDIUM
- Classification: MEDIUM
- Reasoning: F-P7-001 is sibling-blast-radius drift on canonical ABI doc — same pattern fixed at pass-6 for resolver.error, missed for resolver.merge_collision. F-P7-002/003 are sibling gaps for resolver.not_found. Pattern: fixing one event fully, missing siblings. Comprehensive fix all 3 events at once breaks the cycle.
- Recommendation: PROCEED_TO_FIX. After comprehensive sibling sweep, pass-8 should achieve NITPICK_ONLY and passes_clean = 1.
