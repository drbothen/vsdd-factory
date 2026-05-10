# Adversarial Review — S-12.03 Pass 5

## Metadata
- Story: S-12.03 ContextResolver trait + ResolverRegistry
- Branch SHA reviewed: 86621c8f
- Pass: 5
- Reviewer: adversary (fresh context)
- Classification: HIGH
- Within-story finding count: 6
- Recommendation: PROCEED_TO_FIX

## Findings

### F-S12.03-P5-001 — HIGH — error_kind wire format diverges from HOST_ABI specification
**File(s):** `crates/factory-dispatcher/src/executor.rs:506-512`; `crates/hook-sdk/HOST_ABI.md:1095`
**Description:** HOST_ABI line 1095 specifies error_kind values are lowercase snake_case: `"trap", "timeout", "abi_violation", "capability_denied", "not_found", "load_error"`. Implementation uses `serde_json::to_value(err)` against ResolverError with `#[serde(tag = "kind")]` — produces PascalCase `"Crashed"`, `"AbiViolation"`, `"CapabilityDenied"`, `"NotFound"`. Note `"trap"` token expected for crashes, but variant is named `Crashed`. F-P2-007 integration test asserts `all_log_content.contains("Crashed")` — pinning wrong wire format into regression suite.
**Suggested resolution:** Either `#[serde(tag = "kind", rename_all = "snake_case")]` (and rename Crashed → Trap) or explicit kind-string mapping function.

### F-S12.03-P5-002 — HIGH — error_detail field name diverges; drops Display string; missing hook_event_name
**File(s):** `crates/factory-dispatcher/src/executor.rs:521`; `HOST_ABI.md:1096-1097`
**Description:** HOST_ABI line 1096 specifies field name `error_detail` (singular). Implementation emits `error_details` (plural, attached object). HOST_ABI describes "Human-readable description" → corresponds to Display formatting (`format!("{err}")`), not structured serde dump. Also `hook_event_name` per HOST_ABI line 1097 should appear on resolver.error event but is not emitted.
**Suggested resolution:** Rename `error_details` → `error_detail`; emit Display string; add `hook_event_name` field.

### F-S12.03-P5-003 — HIGH — CollisionInfo.resolver_name misderived; uses output.key instead of resolver identity
**File(s):** `crates/factory-dispatcher/src/resolver.rs:344-352`
**Description:** `merge_resolver_outputs` populates `CollisionInfo.resolver_name` with `output.key.clone()`. Doc comment relies on convention "each resolver writes under its own name" — but pure function has lost resolver-identity context. Sees only `ResolverOutput.key`, not `ContextResolver::name()`. Concrete failure: resolver named "foo" writing key "bar" colliding with static "bar" emits `resolver_name="bar"` (semantically wrong). Integration test `f_p4_001b_merge_collision_event_carries_resolver_name` masks bug — engineered such that resolver_name == colliding key.
**Suggested resolution:** Thread resolver-identity through merge — `Vec<(String resolver_name, ResolverOutput)>` or wrapper struct. Update test to use distinct resolver name vs collision key.

### F-S12.03-P5-004 — MEDIUM — ResolverError::Malformed field naming inconsistent with sibling variants
**File(s):** `crates/factory-dispatcher/src/resolver.rs:107`
**Description:** All other variants use `name: String` (NotFound, Crashed, AbiViolation, Timeout, CapabilityDenied, DuplicateName). Only Malformed uses `resolver: String, detail: String`. Wire shape `{"kind": "Malformed", "resolver": "...", "detail": "..."}` diverges from siblings `{"kind": "NotFound", "name": "..."}`. Downstream parsers filtering by resolver_name silently miss Malformed events.
**Suggested resolution:** Rename `Malformed { resolver, detail }` → `Malformed { name, detail }`.

### F-S12.03-P5-005 — MEDIUM — ResolverInput.hook_event_name set to entry.name; risk of conflation with event_type
**File(s):** `crates/factory-dispatcher/src/executor.rs:432-454`
**Description:** event_type extracted from payload `event_name`/`hook_event_name` (typically "PreToolUse"). Then `hook_event_name: entry.name.clone()` (registry entry name like "capture-commit-activity"). Same field name carrying two semantics across dispatcher boundary is footgun. Resolver receives `event_type="PreToolUse"` AND `hook_event_name="capture-commit-activity"` — contradicts every other place where `hook_event_name == event_type`. Undertested.
**Suggested resolution:** Either (a) add ResolverInput-shape test asserting `hook_event_name == entry.name` is intentional, or (b) clarify in BC and rename to `hook_registry_name` for explicitness.

### F-S12.03-P5-006 — MEDIUM — payload field extraction order doesn't match canonical Claude Code envelope
**File(s):** `crates/factory-dispatcher/src/executor.rs:432-437`; `payload.rs:23` (alias = "hook_event_name"); CHANGELOG.md:1541
**Description:** payload.rs:23 has `#[serde(alias = "hook_event_name")]` indicating canonical field is `event_name`, with `hook_event_name` as legacy alias. Real Claude Code envelopes send `hook_event_name`. Executor prefers `event_name` over `hook_event_name`. payload_value is serde_json::Value not typed payload — may carry original Claude Code field name. If real envelope reaches build_plugin_config with only hook_event_name set, extraction succeeds via fallback. No test verifies this for resolver dispatch.
**Suggested resolution:** Add unit test: pass real Claude Code envelope shape (only `hook_event_name`) and verify event_type extraction. Document the field aliasing in BC-4.12.002.

## Cross-cutting observations
- Pass-4 burst added error_kind to resolver.error events but did not validate wire format against HOST_ABI — gap is exactly what F-001/002 identifies. Pass-5 reviewer first to read HOST_ABI + impl side-by-side.
- Integration tests use substring-presence assertions (`.contains("Crashed")`, `.contains("resolver_name")`). Anti-pattern (POL-11): substring "resolver_name" appears in any JSON serialization even if value wrong. F-P4-001B test `.contains("collision-key")` same anti-pattern — static config key, resolver name, and collision-event resolver_name field all happen to be "collision-key", so test cannot distinguish. **[process-gap]** consider extracting NDJSON entries and asserting on parsed JSON field values, not substring presence.
- No test asserts ResolverInput shape that resolvers receive. TestSpyResolver capturing received input would catch F-005/006 in CI.
- F-P4-002 deferral note acknowledged in test file header. Acceptable.
- Cargo.toml proptest row correctly present at line 54 (F-P4-003 addressed).

## Convergence assessment
- Within-story findings: 6 (3 HIGH, 3 MEDIUM)
- Severity floor: HIGH
- Classification: HIGH
- Reasoning: Three HIGH findings + three MEDIUM. Fixes from Pass 4 (error_kind field, resolver_name in collision) addressed presence of data but not correctness of wire format. NITPICK_ONLY streak resets to 0. Novelty HIGH — findings expose contract drift not localized to fix sites of prior passes. Pass-4 burst increased quantity of structured fields but did not validate any against HOST_ABI source of truth.
- Recommendation: PROCEED_TO_FIX. Priority: F-001/002 (HOST_ABI alignment), F-003 (thread resolver-identity), F-004 (Malformed naming), F-005/006 (test additions).
