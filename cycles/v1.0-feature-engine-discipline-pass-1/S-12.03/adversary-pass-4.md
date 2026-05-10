# Adversarial Review — S-12.03 Pass 4

## Metadata
- Story: S-12.03 ContextResolver trait + ResolverRegistry
- Branch SHA reviewed: ea40f96a
- Pass: 4
- Reviewer: adversary (fresh context)
- Classification: LOW
- Within-story finding count: 4
- Recommendation: PROCEED_TO_FIX

## Findings

### F-S12.03-P4-001 — LOW — `resolver.error` event emits Debug-formatted error string instead of structured `error_kind`; `resolver.merge_collision` lacks `resolver_name` field
**File(s):** `crates/factory-dispatcher/src/executor.rs:478-537`
**Anchor:** HOST_ABI.md §`resolver.error` line 1087 (structured `error_kind` shape); BC-4.12.004 referenced from BC-4.12.002 line 173.
**Description:** Three resolver telemetry events emit different shapes:
- `resolver.not_found` (482-491): `plugin_name` + `resolver_name` ✓
- `resolver.error` (497-507): `plugin_name` + `resolver_name` + `error: format!("{err:?}")` — Debug formatting, not structured
- `resolver.merge_collision` (530-537): `plugin_name` + `key` + `static_value` + `resolver_value` — no `resolver_name`

HOST_ABI.md §`resolver.error` documents structured `error_kind` shape. Current Debug-formatted string is not machine-greppable for `error_kind`. Future parsers expecting `{"error_kind":"Crashed"}` get `{"error":"Crashed { name: \"foo\", detail: \"test\" }"}`. ResolverError already has `#[serde(tag = "kind")]` — easy to map.

For `resolver.merge_collision`: when hook H has `needs_context: ["a", "b"]` and both resolvers output key "foo", emitted events have only `plugin_name=H` + `key=foo` — indistinguishable. Observability degraded (story narrative + BC-4.12.005 PC5 don't require, but audit trail incomplete).
**Suggested resolution:** (a) Map ResolverError variants to `error_kind` string (or use the serde-tag value) and emit as separate field. (b) Add `resolver_name` to merge_collision event.

### F-S12.03-P4-002 — NITPICK — Tautological catch_unwind scaffolding around production calls documented as "fully implemented"
**File(s):** `crates/factory-dispatcher/tests/resolver_registry_test.rs:7-12 + ~30 occurrences`
**Description:** File's own module doc admits the scaffolding is dead. Test bodies bloated with `catch_unwind(AssertUnwindSafe(|| ...))` + `.is_ok()` whose only effect is converting Rust panic to test failure with slightly different message. The actual `.unwrap()`-then-assert lines below would already fail on panic.
**Suggested resolution:** Cleanup pass to remove catch_unwind wrappers (out of scope for adversary convergence — flag for follow-up).

### F-S12.03-P4-003 — NITPICK — Story spec File List entry references conditional Cargo.toml change
**File(s):** `.factory/stories/S-12.03-context-resolver-trait-and-registry.md:181`
**Description:** Entry reads "Add proptest dev-dependency if not already present (check; VP-075 requires proptest in dev-dependencies)." Conditional phrasing. Actual `Cargo.toml` was not modified (proptest already in dev-deps).
**Suggested resolution:** Either remove the row entirely (since no change was made) or rephrase definitively (e.g., "Verified proptest dev-dep already present, no change required").

### F-S12.03-P4-004 — NITPICK — `agent_type` extraction from envelope is dead code
**File(s):** `crates/factory-dispatcher/src/executor.rs:438-441`
**Description:** Code extracts `agent_type` field from envelope. Standard Claude Code lifecycle envelopes do not carry this field. ResolverInput.agent_type Option<String> defaults to None (correct). But reader sees field extracted without docs that it's reserved-for-future-use. If `subagent_type` is ever added (the Anthropic SDK term), lookup misses the renamed field.
**Suggested resolution:** Add a doc-comment explaining the field is reserved (BC-4.12.002 forward-compat) and noting the absent-default semantic.

## Cross-cutting observations
- POLICY 10: docs/demo-evidence/S-12.03/ not yet present, post-convergence step (per DoD line 287). Not blocking.
- POLICY 11: prop_resolve_is_deterministic removal consistent with Dev Notes. 3 remaining proptests call merge_resolver_outputs (production pure fn). Pass.
- POLICY 12: resolver.merge_collision emits 3 fields per BC-4.12.005 PC5. Wire-format-correct.
- AC-005 integration test exercises production path via real ResolverRegistry → execute_tiers → build_plugin_config → internal_log.write. Genuine coverage.
- AC-012 variant identity assertion (line 849): `err == ResolverError::DuplicateName { name: "foo".to_string() }`. Sound.
- Test Plan ↔ test file: Test Plan has 14 entries, file has 20+. Story Test Plan abridged.
- f_p2_007 has dead `let _ = log_entries;` line — cosmetic only.

## Convergence assessment
- Within-story findings: 4 (1 LOW, 3 NITPICK)
- Severity floor: LOW
- Classification: LOW
- Reasoning: F-P4-001 names a concrete spec-implementation gap (HOST_ABI documents structured error_kind; dispatcher emits Debug). Fixable with small diff. Other 3 are NITPICK quality. One LOW = LOW per strict rule.
- Recommendation: PROCEED_TO_FIX. After F-P4-001 lands, one more pass should reach NITPICK_ONLY.
