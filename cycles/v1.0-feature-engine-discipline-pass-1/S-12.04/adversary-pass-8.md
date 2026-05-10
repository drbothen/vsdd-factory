# Adversarial Review — S-12.04 Pass 8

## Metadata
- Story: S-12.04 WASM Resolver Loading, Lifecycle, and Error Isolation
- Branch SHA reviewed: cbada816
- Pass: 8
- Reviewer: adversary (fresh context)
- Classification: MEDIUM
- Within-story finding count: 4 (3 MEDIUM, 1 NITPICK)
- Recommendation: PROCEED_TO_FIX (exhaustive sweep needed)

## Findings

### F-S12.04-P8-001 — MEDIUM — resolver.registry_loaded / resolver.load_warning / resolver.load_error have NO HOST_ABI field tables
**Files:** main.rs:317, 331, 344 emit; HOST_ABI.md no tables for these 3 events.
**Description:** Pass-7's P-005 convention applied to executor.rs events (not_found/error/merge_collision) but missed startup-phase events emitted from main.rs. P-005's enumerated list was incomplete (missing registry_loaded).
**Suggested:** Add 3 HOST_ABI field tables.

### F-S12.04-P8-002 — MEDIUM — resolver.merge_collision integration test lacks POL-11 provenance triplet
**Files:** executor_resolver_integration.rs:685-710
**Description:** Tests for not_found + error assert trace_id/session_id/plugin_name literals (P4/P5/P6/P7). merge_collision test missed.
**Suggested:** Add 3 positive-coverage assertions parallel to F-P7-003.

### F-S12.04-P8-003 — MEDIUM — Field naming drift: error vs error_detail vs detail
**Files:** executor.rs:544 (error_detail), main.rs:347 (error), main.rs:335 (detail)
**Description:** Three resolver-tier events emit "human-readable error description" under three different field names. Observability consumers can't query uniformly. BC-4.12.004 PC2 mandates error_detail.
**Suggested:** Align all three to error_detail OR document the divergence per-event in HOST_ABI.

### F-S12.04-P8-004 — NITPICK — P-005 convention enumeration incomplete
**Files:** executor.rs:422-424
**Description:** P-005 convention list cites 5 events but main.rs emits a 6th (resolver.registry_loaded). Convention authority weakened on first sweep.
**Suggested:** Add registry_loaded to the list. Generalize convention scope to cover main.rs emit sites too.

## Cross-cutting observations

- F-P7-001/002/003 verified closed: HOST_ABI tables for merge_collision + not_found have full provenance triplet. Tests for not_found + error assert trace_id/session_id/plugin_name literals.
- F-P7-004 stale comment closure verified.
- P-005 convention IS at executor.rs:418-424 — correctly placed but per-source-file scoped (didn't reach main.rs).
- The sibling-propagation cycle continues. Iteration 4 of same pattern. Need EXHAUSTIVE cross-file sweep to break it.

## Convergence assessment

- Within-story findings: 4 (3 MEDIUM, 1 NITPICK)
- Severity floor: MEDIUM
- Classification: MEDIUM
- Reasoning: The very convention codified at pass-7 to prevent sibling-propagation regressions (P-005) was published with an incomplete sibling list. F-P8-001 + F-P8-002 + F-P8-003 are all the same defect class P-005 was authored to prevent, just at different file boundaries. Pass-7 covered executor.rs but missed main.rs entirely.
- Recommendation: PROCEED_TO_FIX with EXHAUSTIVE cross-file event audit. After this burst, pass-9 should achieve NITPICK_ONLY.
