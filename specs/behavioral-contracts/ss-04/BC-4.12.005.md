---
document_type: behavioral-contract
level: L3
version: "1.2"
status: draft
producer: product-owner
timestamp: 2026-05-07T00:00:00Z
phase: 1a
inputs:
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-platform-amendment-delta-analysis.md
  - .factory/cycles/v1.0-feature-engine-discipline-pass-1/decision-log.md
input-hash: "[pending-recompute]"
traces_to: .factory/cycles/v1.0-feature-engine-discipline-pass-1/F1-platform-amendment-delta-analysis.md
origin: greenfield
subsystem: "SS-04"
capability: "CAP-009"
lifecycle_status: active
introduced: v1.0-feature-engine-discipline-pass-1
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-4.12.005
section: "4.12"
last_amended: 2026-05-10
---

# BC-4.12.005: Context-injection merging MUST be additive (resolver output overlays `plugin_config` under its key); two resolvers with the same `context_key` is a registry-load error

## Description

When multiple resolvers contribute context to the same dispatch, the dispatcher merges their
`ResolverOutput` into `plugin_config` additively: each resolver's output is set at
`plugin_config[resolver.key] = resolver.value`. The existing static `plugin_config` from the
hooks-registry entry is preserved — resolver outputs are additive overlays, not replacements.
Resolver outputs are merged in registry declaration order (first resolver declared wins on
key collision with a `resolver.merge_collision` event emitted). Two resolvers with the same
`context_key` is a registry-load-time error (fail-loud). A resolver that returns `value: None`
does NOT write its key to `plugin_config` (the key remains absent, not present-but-null). The
merge is a pure JSON value overlay — no deep merge; the resolver's `value` replaces the key
wholesale.

## Preconditions

1. One or more resolver invocations have completed (per BC-4.12.001/002 lifecycle and ABI).
2. Each invocation produced either a `ResolverOutput { key, value: Some(v) }` or
   `ResolverOutput { key, value: None }` (or failed with a `ResolverError` per BC-4.12.004).
3. The static `plugin_config` from the hooks-registry entry has been deserialized as a
   `serde_json::Value::Object`.
4. The `executor.rs` merge step is executing after all resolver invocations complete but
   before `invoke_plugin` is called.

## Postconditions

1. **Additive overlay:** The final `plugin_config` passed to `invoke_plugin` is the union of:
   - All fields from the static hooks-registry `plugin_config` (preserved as-is).
   - All fields from resolver outputs (one field per resolver: `plugin_config[key] = value`).
   Resolver output fields do NOT remove or overwrite existing static `plugin_config` fields
   (except on key collision — see PC5).
2. **`value: None` → key absent:** If a resolver returns `ResolverOutput { key: "foo",
   value: None }`, the key `"foo"` is NOT written to `plugin_config`. The key is absent from
   the merged `plugin_config`. The hook reading `plugin_config["foo"]` sees a missing key,
   not a null value.
3. **`value: Some(v)` → key present:** If a resolver returns `ResolverOutput { key: "foo",
   value: Some(v) }`, `plugin_config["foo"] = v` is set. The previous value at `"foo"` (if
   any, from static config) is replaced wholesale (not deep-merged). The hook reading
   `plugin_config["foo"]` sees `v`.
4. **Merge in declaration order:** Resolvers are invoked and merged in the order they appear
   in the `needs_context` array of the hooks-registry entry. If resolver A is declared first
   and resolver B is declared second, A's output is merged first, then B's output is merged.
   This means: if A and B both output the same key (which is a registry-load error per PC6,
   but if somehow reached at dispatch time), B's value wins over A's value (last-write-wins
   at dispatch time; fail-loud at registry-load time is the correct enforcement).
5. **Static config key collision:** If a resolver outputs a key that already exists in the
   static hooks-registry `plugin_config`, the resolver output wins (resolver output overlays
   static config). A `resolver.merge_collision` telemetry event is emitted with the key name,
   static value, resolver value, and `resolver_name` (the registry name of the resolver whose
   output produced the collision). This collision is not an error — it is an expected
   override pattern (e.g., a resolver enriching a static key with dynamic data).
6. **Duplicate `context_key` at registry-load time (fail-loud):** If `resolvers-registry.toml`
   declares two resolver entries with the same `name` (i.e., two entries that would produce
   the same `context_key`), the dispatcher MUST emit `resolver.load_error` and fail startup.
   Two resolvers with the same key cannot coexist in the same registry.
7. **Whole-value replacement (no deep merge):** If resolver A outputs
   `{ key: "wave_context", value: { "stories": ["S-1"], "wave_id": "w1" } }` and the static
   config already has `plugin_config["wave_context"]["foo"] = "bar"`, the resolver output
   REPLACES the entire `plugin_config["wave_context"]` value. The result is
   `plugin_config["wave_context"] = { "stories": ["S-1"], "wave_id": "w1" }` — "foo" is
   gone. This is intentional: whole-value replacement is simpler and avoids deep-merge
   semantics ambiguity.
8. **Resolver cannot modify other resolvers' keys:** Each resolver returns
   `ResolverOutput { key: String, value: Option<Value> }` — it can only set its own key.
   The merge algorithm enforces this: only the `key` field from `ResolverOutput` determines
   the merge target. A resolver cannot name a key it does not own.

## Invariants

1. **Pure merge algorithm:** The merge step is a pure function:
   `merge(static_config: Value, resolver_outputs: Vec<ResolverOutput>) -> Value`.
   Given the same inputs, it produces the same output. The merge does not depend on
   dispatcher state, filesystem state, or wall-clock time. This is the VP-075 determinism
   invariant at the merge level.
2. **Resolver owns exactly one key:** Each resolver entry in `resolvers-registry.toml` has
   exactly one `name` field. The resolver's output `key` MUST match its registry `name`.
   If they diverge, the merge uses the `key` from `ResolverOutput` (which may be the wrong
   key). This is an authoring error, not a dispatcher error; the dispatcher does not validate
   key consistency.
3. **Absent failed-resolver key:** Per BC-4.12.004, a failed resolver contributes nothing to
   the merge. The key is absent from `plugin_config`. This invariant composes with the
   merge-is-additive postcondition: additive merge + absent failed resolver = the only keys
   in `plugin_config` after merge are static keys plus successfully-resolved keys.
4. **No resolver output on `plugin_config` root fields:** Resolvers set `plugin_config[key]`
   where `key` is the resolver's name. Resolvers CANNOT set arbitrary top-level fields by
   returning a JSON object with multiple keys. The `ResolverOutput.key` field determines the
   single top-level key; `ResolverOutput.value` is the value at that key.
5. **`needs_context` is the merge scope:** Only resolvers named in the `needs_context` field
   of the hooks-registry entry contribute to the merge for that dispatch. Other registered
   resolvers (in `resolvers-registry.toml` but not in `needs_context`) are NOT invoked and
   do NOT contribute any keys to `plugin_config`.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Single resolver returns `None` | Merge result: `plugin_config` unchanged (static config only). Key absent. |
| EC-002 | Single resolver returns `Some({})` (empty object) | `plugin_config["resolver_key"] = {}`. Key present with empty object value. |
| EC-003 | Two resolvers; both succeed; different keys | Additive merge: both keys present in `plugin_config`. No collision. |
| EC-004 | Resolver output key collides with static config key | Resolver wins. `resolver.merge_collision` event emitted. Static value replaced. |
| EC-005 | Two resolvers in `resolvers-registry.toml` with same `name` | `resolver.load_error` at registry-load time. Startup fails. |
| EC-006 | Resolver A fails; resolver B succeeds | A's key absent. B's key present. Merge is partial but deterministic. |
| EC-007 | `needs_context = []` (no resolvers declared) | Merge step is a no-op. `plugin_config` = static config only. |
| EC-008 | Resolver outputs a JSON array as its value | `plugin_config["key"] = [...]`. Valid — `ResolverOutput.value` is `Option<serde_json::Value>` which accepts arrays. |

## Canonical Test Vectors

| Static `plugin_config` | Resolver Outputs | Expected Merged `plugin_config` |
|------------------------|-----------------|--------------------------------|
| `{"foo": "bar"}` | `[{key: "wave_context", value: {"stories": ["S-1"]}}]` | `{"foo": "bar", "wave_context": {"stories": ["S-1"]}}` |
| `{"foo": "bar"}` | `[{key: "wave_context", value: null}]` | `{"foo": "bar"}` (key absent) |
| `{"wave_context": {"old": 1}}` | `[{key: "wave_context", value: {"new": 2}}]` | `{"wave_context": {"new": 2}}` — whole replacement; "old" gone; `resolver.merge_collision` event |
| `{}` | `[{key: "a", value: 1}, {key: "b", value: 2}]` | `{"a": 1, "b": 2}` |
| `{}` | `[A fails, B returns {key: "b", value: 2}]` | `{"b": 2}` (A's key absent) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-075 | Context-injection determinism — given identical inputs (static config + resolver outputs), merge produces identical result | proptest (200 trials; generate arbitrary static config + resolver outputs; assert merge result is identical on two calls) |
| (unit-test) | `value: None` leaves key absent from `plugin_config` | Rust unit test |
| (unit-test) | Resolver output overlays static config key wholesale (no deep merge) | Rust unit test |
| (unit-test) | `resolver.merge_collision` event emitted when resolver overlays existing static key | Rust unit test (assert telemetry event) |
| (unit-test) | Two resolvers with same name at load time → `resolver.load_error` | Rust unit test (registry parse) |
| (unit-test) | `needs_context = []` → merge step is no-op (no resolver invoked) | Rust unit test (assert resolver mock not called) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-009 |
| Capability Anchor Justification | CAP-009 ("Author and publish WASM hook plugins using the Rust SDK") per capabilities.md §CAP-009 — this BC governs the context-injection merge semantics for WASM resolver plugins. The merge algorithm operates at the dispatcher layer (`executor.rs`) and determines how resolver plugin outputs are combined with the static `plugin_config` before hook dispatch. CAP-009 defines the SDK and dispatch pipeline; the merge contract is the final step in the pre-dispatch resolver invocation sequence before the hook plugin (also a CAP-009 artifact) is called with the enriched payload. |
| L2 Domain Invariants | none |
| Architecture Module | `crates/factory-dispatcher/src/executor.rs` (merge step between resolver invocation and invoke_plugin); `crates/factory-dispatcher/src/resolver.rs` (merge_resolver_outputs pure function) |
| Stories | S-12.03, S-12.06, S-12.07, S-12.08 |
| FR | FR-RESOLVER-001 (factory-agnostic runtime context injection — resolver outputs merged into plugin_config) |
| ADR Reference | ADR-018 (WASM-plugin Context Resolvers — merge semantics: additive overlay; whole-value replacement; first-declared-wins on collision at dispatch; fail-loud on duplicate context_key at registry load) |

## Related BCs

- BC-1.13.001 — depends on (dispatcher pre-dispatch injection contract; this BC defines the merge step that BC-1.13.001's postcondition 5 references)
- BC-4.12.001 — sibling (lifecycle — resolver outputs are the product of per-invocation Store calls; this BC merges those outputs)
- BC-4.12.002 — depends on (`ResolverOutput.key` and `ResolverOutput.value` are the merge inputs defined by the ABI BC)
- BC-4.12.004 — composes with (crash isolation — failed resolvers contribute nothing to the merge; this BC specifies the positive case)

## Architecture Anchors

- `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/executor.rs` — merge step (apply resolver outputs to plugin_config before invoke_plugin)
- `/Users/jmagady/Dev/vsdd-factory/crates/factory-dispatcher/src/resolver.rs` — `merge_resolver_outputs(static_config, resolver_outputs) -> Value` pure function (proptest target)
- `/Users/jmagady/Dev/vsdd-factory/.factory/specs/architecture/decisions/ADR-018-wasm-plugin-context-resolvers.md` — merge semantics decision

## Story Anchor

S-12.03 (ContextResolver trait + ResolverRegistry in-memory) — v1.0-feature-engine-discipline-pass-1 F3-amendment. The in-memory resolver mock in S-12.03 validates the merge algorithm before WASM loading is added in S-12.04.

## VP Anchors

- VP-075 — Context-injection determinism

## Changelog

| Version | Date | Description |
|---------|------|-------------|
| 1.0 | 2026-05-07 | Initial authoring (product-owner; F2-amendment phase of v1.0-feature-engine-discipline-pass-1). Additive overlay semantics; whole-value replacement (no deep merge); duplicate context_key is a registry-load error (fail-loud); None output leaves key absent. OD-5 (no inter-resolver dependencies) encoded in Invariant 4 and PC8. |
| 1.1 | 2026-05-09 | F-P45-001 — Traceability Stories row propagated from BC-INDEX v1.57: S-12.03 → S-12.03, S-12.06, S-12.07, S-12.08. BC-INDEX was updated in fix-burst-39 (v1.55) to add S-12.06 + S-12.07 + S-12.08; body was not updated in that burst. Refs: F-P45-001, fix-burst-42. |
| 1.2 | 2026-05-10 | F-P6-003 — PC5 updated to include `resolver_name` in `resolver.merge_collision` event field description; HOST_ABI resolver.merge_collision field table added with `resolver_name` column. Refs: F-P6-003, adversary-pass-6. |
