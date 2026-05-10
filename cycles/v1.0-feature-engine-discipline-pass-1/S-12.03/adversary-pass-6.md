# Adversarial Review — S-12.03 Pass 6

## Metadata
- Story: S-12.03 ContextResolver trait + ResolverRegistry
- Branch SHA reviewed: 425a466f
- Pass: 6
- Reviewer: adversary (fresh context)
- Classification: MEDIUM
- Within-story finding count: 4
- Recommendation: PROCEED_TO_FIX

## Findings

### F-S12.03-P6-001 — MEDIUM — Stale Crashed variant references in 3 test doc-comments
**File(s):**
- `crates/factory-dispatcher/tests/resolver_registry_test.rs:1051` — `/// A resolver that always returns Err(ResolverError::Crashed).`
- `crates/factory-dispatcher/tests/resolver_registry_test.rs:1080` — `/// registered resolver returns Err(ResolverError::Crashed).`
- `crates/factory-dispatcher/tests/executor_resolver_integration.rs:404` — `/// returns Err(ResolverError::Crashed), execute_tiers must:`
**Description:** Pass-5 fix burst renamed Crashed→Trap (resolver.rs:86-92). All actual code uses Trap. Three test doc-comments still claim Crashed. Test bodies actually emit Trap (lines 1072, 326). S-7.01 partial-fix-regression: variant rename applied to source + test bodies but doc-comments in same files not updated. Blast radius = 3 references in 2 files → MEDIUM.
**Suggested resolution:** s/Crashed/Trap/ in three doc-comments.

### F-S12.03-P6-002 — MEDIUM — hook_event_name field has two divergent semantics
**File(s):**
- HOST_ABI line 949 + 959 (`ResolverInput.hook_event_name` = "the name of the hook being dispatched")
- `crates/factory-dispatcher/src/executor.rs:450` (`hook_event_name: entry.name.clone()` — registry entry name)
- HOST_ABI line 1097 (`resolver.error event hook_event_name` = "the hook dispatch context that triggered this resolver")
- `crates/factory-dispatcher/src/executor.rs:526` (emits Claude Code envelope event_type "PreToolUse" into hook_event_name field of resolver.error)
**Description:** Same field name, two meanings. ResolverInput.hook_event_name = hook registry entry name. resolver.error event hook_event_name = Claude Code envelope event_type. Operator joining resolver.error events to triggering input would join on wrong column. F-P5-005 introduced the convention as "documented but suspicious"; pass-6 confirms.
**Suggested resolution:** Rename resolver.error event field hook_event_name → event_type (matches ResolverInput.event_type — same value). Keep ResolverInput.hook_event_name as registry-entry-name. Update HOST_ABI line 1097.

### F-S12.03-P6-003 — MEDIUM — resolver.merge_collision wire format: resolver_name field emitted but undocumented in HOST_ABI
**File(s):**
- HOST_ABI line 1137-1140 (resolver.merge_collision: "the key name, static value, and resolver value" — no resolver_name)
- `crates/factory-dispatcher/src/executor.rs:553-555` (emits resolver_name field)
- BC-4.12.005 PC5 also doesn't mention resolver_name
**Description:** Implementation has 4th wire field (resolver_name) required by tests but absent from both HOST_ABI documentation and BC-4.12.005 postcondition. Inverse of F-P4-001B intent: field added for traceability — good — but not propagated to HOST_ABI. Future plugin authors won't know field exists. Future implementers may "clean up" the apparently-undocumented field and break F-P4-001B.
**Suggested resolution:** Add resolver_name to HOST_ABI line 1137 paragraph (and BC-4.12.005 PC5) so wire format documentation matches code. Equivalent treatment to resolver.error event's structured field table (HOST_ABI lines 1087-1097).

### F-S12.03-P6-004 — LOW — plugin_config.non_object warning path unreachable in practice
**File(s):** `crates/factory-dispatcher/src/executor.rs:457-474`
**Description:** Warning emitted when `entry.config_as_json()` is not Value::Object. registry.rs default_config returns Value::Table; TOML [plugin.config] section can only be Table by syntax; config_as_json always converts Table to JSON Object. No test exercises this path. Either dead code (silent noise) or reachable through some construct (TOML datetime as entire config? non-table TOML scalar?) but no test demonstrates either case.
**Suggested resolution:** Either (a) excise with comment explaining why impossible after Registry::parse_str succeeds, or (b) add unit test demonstrating the path. (pending intent verification)

## Cross-cutting observations
- Proptest harness (resolver_determinism_proptest.rs) well-structured. VP-075 spec requires 200 trials; only prop_merge_is_deterministic runs 200 (correctly mapped to AC-008). Two additional proptests run 100 — extra coverage beyond spec.
- merge_resolver_outputs correctly takes serde_json::Map<String, Value> instead of broader Value (F-006 type-narrowing). Good.
- CollisionInfo.resolver_name correctly populated from threaded registry name (F-P5-003). De-masked F-P4-001B test (executor_resolver_integration.rs lines 588-591, registry_name="test_resolver_alpha", output_key="collision-key") DOES distinguish identity from output key as intended.
- HOST_ABI line 1095 wire format ("trap"/"timeout"/"abi_violation"/"capability_denied"/"not_found"/"load_error") correctly matches snake_case serde tag in resolver.rs lines 80-125. Malformed and DuplicateName variants serialize to "malformed" and "duplicate_name" — these aren't in HOST_ABI line 1095 enumeration but also aren't surfaced through resolver.error wire surface in this story. Acceptable.
- Frontmatter coherence: S-12.03 frontmatter bcs:[BC-1.13.001, BC-4.12.005] matches body Behavioral Contracts table. AC traces correct.

## Convergence assessment
- Within-story findings: 4 (3 MEDIUM, 1 LOW)
- Severity floor: MEDIUM
- Classification: MEDIUM
- Reasoning: Pass-5 fix burst landed substantial improvements (Trap rename, snake_case tag, field name fixes, F-P5-003 resolver-identity threading). However, four findings remain — three MEDIUM are sibling-propagation gaps from pass-5 burst (variant rename not propagated to test doc-comments; new wire fields not propagated into HOST_ABI). One LOW is real semantic-conflict finding about field-name reuse with divergent meanings.
- Novelty: Genuinely new findings, not retreads. F-001 sibling-gap. F-002 confirms suspicious convention from F-P5-005. F-003 HOST_ABI propagation gap. F-004 fresh observation about untested defensive branch.
- Recommendation: PROCEED_TO_FIX. Three MEDIUM need addressing. After fix burst, novelty should decay sharply on pass 7.
