# Adversarial Review — S-12.03 Pass 7

## Metadata
- Story: S-12.03 ContextResolver trait + ResolverRegistry
- Branch SHA reviewed: d1e46a18
- Pass: 7
- Reviewer: adversary (fresh context)
- Classification: NITPICK_ONLY
- Within-story finding count: 2 (both NITPICK)
- Recommendation: ACCEPT — clean pass. Counts toward 3-consecutive-NITPICK_ONLY convergence.

## Findings

### F-S12.03-P7-001 — NITPICK — build_plugin_config invokes entry.config_as_json() three times redundantly
**File(s):** `crates/factory-dispatcher/src/executor.rs:453, 464, 468`
**Description:** Inside non-empty needs_context branch, config_as_json() called 3 times — line 453 (for ResolverInput.plugin_config), line 464 (debug_assert!), line 468 (match for static_map). config_as_json walks entire TOML tree and rebuilds JSON each time. Zero-overhead path (needs_context.is_empty short-circuit at 427) preserved — does NOT impact BC-1.13.001 PC3 zero-overhead invariant. For non-empty needs_context, triples JSON conversion cost on resolver-enriched dispatch.
**Why NITPICK:** AC-002 zero-overhead preserved. Redundant calls only on resolver-enriched path where dispatch latency dominated by resolver invocation cost (WASM Store creation in S-12.04). No correctness defect. Trivial hoisting refactor.
**Suggested resolution:** `let static_json = entry.config_as_json();` binding above + reuse.

### F-S12.03-P7-002 — NITPICK (cross-story) — resolver.not_found event has no structured field table in HOST_ABI
**File(s):** HOST_ABI.md:895-897 (prose only) vs lines 1092-1097 (resolver.error field table) and 1142-1151 (resolver.merge_collision field table)
**Description:** Pass-6 fix added structured field table for resolver.merge_collision. resolver.error also has field table. resolver.not_found referenced only in narrative prose with no field table — yet executor.rs:487-495 emits it with `resolver_name`, plus envelope `trace_id` and `plugin_name`. For symmetry with sibling events, field table would make wire format machine-checkable. Documentation completeness, not wire-format bug.
**Severity:** NITPICK pending intent — HOST_ABI documentation ownership belongs to S-12.06. Recording for traceability since prompt asked about wire-format docs. Cross-story / wave-gate scope.

## Cross-cutting observations
- Pass-6 fix-burst propagation verified:
  - event_type rename in resolver.error consistent across executor.rs:522-525, HOST_ABI.md:1097, and tests/executor_resolver_integration.rs:500-507. No leftover hook_event_name in resolver.error context. hook_event_name references in payload.rs:23, executor.rs:434+450, resolver.rs:35 are intentional and orthogonal.
  - BC-4.12.005 v1.2 PC5 (lines 80-82) correctly mentions resolver_name. Implementation in executor.rs:551-554 threads registry name (not output key). NamedKeyResolver test (lines 518-553) verifies registry-name vs output-key distinction.
  - BC-INDEX entries match BC files' H1 titles exactly. No title drift.
  - debug_assert! replacement at executor.rs:463-467 preserves invariant statically without panicking in production.
- Frontmatter↔body coherence (S-12.03 spec): bcs:[BC-1.13.001, BC-4.12.005] matches body table. All 12 ACs trace correctly. No drift.
- TD acknowledged: tests/resolver_registry_test.rs:13-16 documents catch_unwind red-gate scaffolding as deferred TD via F-P4-002 reference. Pre-existing convergence tracked.
- Subsystem anchor: subsystems:[SS-01] correct for dispatcher core. merge function (BC-4.12.005, SS-04) cross-subsystem placement justified.
- Semantic anchoring audit: Architecture Anchors in BC-1.13.001:195 and BC-4.12.005:179-180 reference real files. ADR-018 reference resolves. VP-075 anchor traceable.
- No process-gap tags this pass.

## Convergence assessment
- Within-story findings: 2 (both NITPICK)
- Severity floor: NITPICK
- Classification: NITPICK_ONLY
- Reasoning: Pass-6 targeted fixes (event field rename, BC-4.12.005 v1.2 doc bump, merge_collision field table, non_object debug_assert) all propagated correctly. Wire format internally consistent. No silent failures, no spec drift, no resource leaks, no concurrency issues, no missing edge cases.
- Recommendation: Mark NITPICK_ONLY. passes_clean transitions 0→1. Continue to pass-8 to accumulate convergence credit.
