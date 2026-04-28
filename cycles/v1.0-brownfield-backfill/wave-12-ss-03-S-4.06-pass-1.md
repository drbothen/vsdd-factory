# S-4.06 Adversarial Review — Pass 1

## Summary
- Findings count: 13
- Severity breakdown: HIGH=4, MEDIUM=6, LOW=2, NITPICK=1
- Verdict: SUBSTANTIVE

## Findings

### F-001 [HIGH]: target_module path is wrong — Router lives in factory-dispatcher, not sink-core
**File:** /Users/jmagady/Dev/vsdd-factory/.factory/stories/S-4.06-routing-tag-enrichment.md (frontmatter line 37; Architecture Mapping line 132-133; File Structure line 197)
**Issue:** Story declares target_module: crates/sink-core/src/router.rs. That file does not exist. The Router struct lives at crates/factory-dispatcher/src/sinks/router.rs. BC-3.04.001 itself cites the correct path (line 78 of the BC). Story's own anchored BC contradicts the story's frontmatter.
**Suggestion:** Change every reference from sink-core/src/router.rs to factory-dispatcher/src/sinks/router.rs. Reconfirm pure-core classification — Router today calls into SinkRegistry which holds boxed Sink objects performing I/O.

### F-002 [HIGH]: AC #4 names filter fields that do not exist in the production struct
**File:** S-4.06 line 89
**Issue:** AC names include_event_types/exclude_event_types/include_plugins. Actual RoutingFilter has event_types_allow/event_types_deny — no include_plugins field at all. v1.1 BC candidate flags include_plugins as un-contracted but AC pretends it's in scope.
**Suggestion:** Rewrite AC #4 to use actual field names and explicitly defer include_plugins to follow-up, OR enumerate that this story adds include_plugins with a v1.0 BC.

### F-003 [HIGH]: Tag enrichment is already implemented at the FileSink driver level — story narrative conflicts with reality
**File:** S-4.06 Goal line 76-79; Related Contracts line 109-119; v1.1 candidates line 154
**Issue:** Story claims tag enrichment will be wired at the Router layer "not inside the individual sink drivers". But sink-file/src/lib.rs lines 345-354 already implements `fn enrich(&self, event: SinkEvent) -> SinkEvent` inside FileSink. Architectural ambiguity: refactor (move up to Router), duplication (Router enriches AND FileSink still enriches), or asymmetric (Router only for OtelGrpc).
**Suggestion:** Add a "Migration Decision" subsection. If refactor, enumerate BCs/tests that move from sink-file to Router and add task to retire FileSink::enrich. Update VP-031.

### F-004 [HIGH]: Partial Status table contradicts Tasks list — "Done" components have no exit criteria
**File:** S-4.06 Partial Status lines 56-61; Tasks lines 168-173
**Issue:** Done components have no commit/SHA/BC citation. Implementer cannot tell what's actually been done. Task #4 contradicts "Done" classification. Terminology drift across "wiring" / "filter application" / "dispatch path".
**Suggestion:** Add "Done at SHA" column. Reword to use exact same component names as Tasks/ACs. Either remove Task #4 or split it.

### F-005 [MEDIUM]: AC #1 silent-drop policy is uncontracted but encoded in v1.1 BC candidate
**File:** S-4.06 line 152
**Issue:** "events failing the filter are silently dropped" is a behavioral decision smuggled in via v1.1 candidate description. EC-001 references "default sink (if configured)" which contradicts silent-drop and references a non-existent default-sink mechanism.
**Suggestion:** Contract silent-drop now with v1.0 BC OR define observability event for filtered drops. Remove EC-001 phantom edge case OR design real default-sink BC.

### F-006 [MEDIUM]: crates/factory-dispatcher/src/sinks.rs does not exist — should be sinks/mod.rs
**File:** S-4.06 line 198
**Issue:** Actual file is sinks/mod.rs. The mod.rs already contains the TODO(integration) block (lines 10-21) describing what this story should do.
**Suggestion:** Correct path. Reference TODO(integration) block in Tasks or Previous Story Intelligence.

### F-007 [MEDIUM]: BC-3.04.001 and BC-3.04.002 invalidated by this story without lifecycle treatment
**File:** BC-3.04.001.md, BC-3.04.002.md
**Issue:** BC-3.04.001 explicitly contracts current state (Router as thin pass-through). BC-3.04.002 preconditions "Tier E S-4.4..S-4.6 haven't shipped". Once S-4.06 lands, both are invalidated. POLICY 1 requires deprecated_by/replaced_by treatment.
**Suggestion:** Add task: "Deprecate BC-3.04.001 and BC-3.04.002 with deprecated_by pointing to new v1.1 BCs, in same commit as implementation."

### F-008 [MEDIUM]: VP-031 anchors to sink-file, not Router — verification will silently miss new code path
**File:** VP-031.md lines 15, 38; S-4.06 line 124-127
**Issue:** VP-031 module: sink-file/lib.rs and test_evidence: sink-file::tests::tag_enrichment_writes_tags_onto_every_event. If S-4.06 moves enrichment to Router, existing test still passes — exercising sink-file's existing enrich, not Router. The story claims to satisfy VP-031 without touching its test evidence.
**Suggestion:** Decide F-003 first. If Router-layer, update VP-031's module/test_evidence/bcs in same commit.

### F-009 [MEDIUM]: AC #1 trace is malformed — points at postcondition the BC text doesn't cleanly support
**File:** S-4.06 line 84
**Issue:** AC #1 traces to BC-3.01.004 postcondition 1, but that postcondition is about accepts(event_type) honoring allow-then-deny — NOT contracting application in dispatch path. The "uncontracted — v1.1 BC candidate" parenthetical admits the gap. POLICY 8 violation: ACs must trace to v1.0 BCs in scope.
**Suggestion:** Promote BC-3.NN.NNN-router-filter-wired-in-dispatch from v1.1 candidate to v1.0 BC (allocate next free BC-3.04.NNN ID) before merging.

### F-010 [MEDIUM]: AC #6 integration test description too vague to verify
**File:** S-4.06 line 93-94
**Issue:** "Two sinks with different filters" doesn't specify driver types, filter dimensions, or oracle. POLICY 11 (no_test_tautologies) — vague tests can pass while bypassing new Router code.
**Suggestion:** Spell out: sink composition, input events, expected oracle, production entry point (Router::submit).

### F-011 [LOW]: Story missing CAP-003 traceability path
**File:** S-4.06 line 65-67
**Issue:** Story's capabilities: [CAP-003] is justified narratively, but the 6 anchored BCs all have capability: "TBD" in their frontmatter. POLICY 5 violation.
**Suggestion:** Same-burst, update each BC's capability frontmatter from "TBD" to "CAP-003" with brief justification.

### F-012 [LOW]: Estimated days vs. points contradiction
**File:** S-4.06 lines 17, 39, 53
**Issue:** points: 3, estimated_days: 2, "Estimated effort: S". F-001..F-004 will materially expand scope.
**Suggestion:** Re-estimate after F-001..F-004 fixed. If include_plugins lands in v1.0, closer to 5pts/3-4 days.

### F-013 [NITPICK]: BC-3.04.002 H1 truncated mid-word
**File:** BC-3.04.002.md line 29
**Issue:** H1 reads "...batching / routin" — truncated. POLICY 7 violation. BC-INDEX entry is similarly truncated.
**Suggestion:** Restore "...batching / routing". Bundle with F-007 lifecycle update.
