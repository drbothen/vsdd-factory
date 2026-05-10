# Adversarial Review — S-12.03 Pass 1

## Metadata
- **Story:** S-12.03 ContextResolver trait + ResolverRegistry
- **Branch SHA reviewed:** 9fbdb680
- **Pass:** 1
- **Reviewer:** adversary (fresh context)
- **Classification:** CRITICAL
- **Within-story finding count:** 7
- **Recommendation:** PROCEED_TO_FIX

## Findings

### F-S12.03-P1-001 — CRITICAL — `executor.rs` MODIFIED requirement is unimplemented; AC-002/AC-003/AC-005/AC-007/AC-009/AC-010 cannot be satisfied at the dispatcher integration layer
**File(s):** `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.03/crates/factory-dispatcher/src/executor.rs` (entire file — zero references to `resolver`/`ResolverRegistry`/`needs_context`/`merge_resolver_outputs`); also `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.03/crates/factory-dispatcher/src/main.rs` (zero references).

Confirmed via `Grep "resolver|needs_context"` against both files returning **0 matches**. The static-config splice at executor.rs:131 (`map.insert("plugin_config".to_string(), entry_clone.config_as_json())`) and the equivalent in `spawn_async_plugin` at executor.rs:270 are still the only places `plugin_config` is built — there is no resolver invocation, no merge step, no `resolver.not_found`/`resolver.merge_collision` event emission, and no zero-overhead short-circuit on `entry.needs_context.is_empty()`.

**Anchor:** BC-1.13.001 PC3, PC4, PC5, PC6, PC7, INV5; BC-4.12.005 PC1, PC5; story Architecture Mapping table; story File List "MODIFIED" row for `executor.rs`; story Definition of Done bullet 3 ("Pre-dispatch resolver step in `executor.rs` with `needs_context.is_empty()` short-circuit"); Tasks T-2 and T-10.

**Description:** S-12.03 is explicitly scoped as building both the in-memory layer (resolver.rs) AND wiring it into dispatch (executor.rs). Six of the twelve ACs (AC-002, AC-003, AC-005, AC-007, AC-009, AC-010) are written against `executor.rs` behavior — for example AC-003: "When `executor.rs` processes a hooks-registry entry with `needs_context: ["foo"]` and a resolver named `"foo"` is registered in `ResolverRegistry`, the dispatcher calls `foo.resolve(input)` and merges the returned output into `plugin_config` under the key returned by `ResolverOutput.key`". The unit tests in `resolver_registry_test.rs` exercise the trait/registry surface in isolation but **none of them goes through `executor::execute_tiers` or `spawn_async_plugin`** to verify the integration. The dispatcher therefore today still receives the unmodified static `plugin_config` regardless of `needs_context` content.

**Why it matters:** This is the load-bearing deliverable. The trait + registry are useless without the integration. AC-002's "zero overhead path" and AC-005's `resolver.not_found` telemetry are dispatcher-level postconditions that the unit tests cannot demonstrate by construction (they assert the *registry layer* not the *dispatcher integration*). If this is left unfixed, S-12.04 (WASM loading) and S-12.07 (resolver impl) will be blocked because they assume executor.rs wires the registry — there is no consumer in tree.

**Suggested resolution:** Either (a) fix the implementation by adding `ResolverRegistry` to `ExecutorInputs`, calling `registry.resolve_context_for_entry(&entry.needs_context, &input, ...)` before the `map.insert("plugin_config", ...)` splice in `execute_tier` and `spawn_async_plugin`, then merging via `merge_resolver_outputs`, plus emitting `resolver.not_found` / `resolver.merge_collision` via `internal_log`. Add an integration test that invokes `execute_tiers` with a registry containing a `needs_context: ["foo"]` entry and asserts the merged `plugin_config` reaches the WASM boundary. OR (b) explicitly amend the story spec to defer executor wiring to a successor story and remove AC-002/003/005/007/009/010's executor language. The current state is a spec-implementation gap.

### F-S12.03-P1-002 — HIGH — `ResolverError::Crashed` is misused for duplicate-name registration; `Crashed` semantically denotes runtime trap/panic, not registry-load configuration error
**File(s):** `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.03/crates/factory-dispatcher/src/resolver.rs:165-172`

**Anchor:** BC-4.12.005 PC6: "If `resolvers-registry.toml` declares two resolver entries with the same `name`... the dispatcher MUST emit `resolver.load_error`". The `Crashed` variant's docstring at resolver.rs:78-80 says "Resolver panicked or trapped during WASM execution. S-12.04 populates this for WASM-backed resolvers." Co-opting it for a load-time configuration error conflates two distinct semantic categories.

**Description:** A duplicate-name registration is a **registry-load error**, parallel to BC-4.12.005 EC-005 and BC-1.13.001 PC2. The `ResolverError` enum has no `LoadError` / `DuplicateName` / `RegistryError` variant. Reusing `Crashed` will cause downstream code that pattern-matches on `ResolverError::Crashed` to treat duplicate registrations as runtime crashes (e.g., emitting wrong telemetry events, fail-closing the wrong way, populating wrong metric labels). When S-12.04 wires the WASM-load path that actually produces real `Crashed` errors, the discriminator is broken — there's no way for telemetry to distinguish "module trapped" from "registry author wrote two entries with the same name".

**Why it matters:** BC-4.12.005 PC6 explicitly names the event: `resolver.load_error`. With this implementation, the only path to that event is for the caller to inspect the error message string and pattern-match the substring "duplicate context_key". That's a fragile, anti-pattern signaling channel.

**Suggested resolution:** Add a new variant `ResolverError::DuplicateName { name: String }` (or `LoadError { detail: String }`) and use it here. Update the `register()` doc to reference this variant. The story Dev Notes section listed the minimum variants — duplicate registration is omitted there but should be added before convergence.

### F-S12.03-P1-003 — HIGH — `register()` doc-comment is internally contradictory: "Panics (fail-loud) if a resolver with the same `name()` has already been registered" vs. "Returns `Err` (does NOT panic)"
**File(s):** `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.03/crates/factory-dispatcher/src/resolver.rs:155-161`

**Anchor:** Documentation/spec consistency. BC-4.12.005 PC6 + S-12.03 AC-012 ("Falsifiable test: ... assert the second `register()` call returns `Err` (or panics with a clear message)").

**Description:** The first sentence says the function panics; the next paragraph says it doesn't panic. Either is plausible per AC-012, but the doc must pick one and stick with it. The implementation (`return Err(...)`) takes the second path, so the first sentence is stale/wrong.

**Why it matters:** Future readers (S-12.04 implementer, anyone debugging registry-load failures) will be misled. Drift between contract docstring and implementation is exactly the kind of latent bug that survives many adversarial passes by appearing "correct" if you only read one line at a time.

**Suggested resolution:** Replace the first sentence with: "Register a resolver. Returns `Err(ResolverError::DuplicateName { name })` if a resolver with the same `name()` has already been registered (BC-4.12.005 PC6 / EC-005 — fail-loud at registry-load time)."

### F-S12.03-P1-004 — HIGH — Test plan lists `prop_resolve_is_deterministic` as a VP-075 proptest covered in this story, but the implementation stubs it forward to S-12.05; AC-008's "200 trials" claim is met only by `prop_merge_is_deterministic`
**File(s):**
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.03/crates/factory-dispatcher/tests/resolver_determinism_proptest.rs:241-260` — the comment block stubs `prop_resolve_is_deterministic` forward to S-12.05.
- Story spec Test Plan table line `| prop_resolve_is_deterministic | proptest | resolver_determinism_proptest.rs | VP-075 |` lists it as in-scope here.
- Story spec AC-008 text says "The proptest for VP-075 (200 trials) verifies: calling `merge_resolver_outputs` twice with identical inputs produces identical results." This matches `prop_merge_is_deterministic` — fine.

**Anchor:** Story Test Plan table; AC-008.

**Description:** The story's Test Plan promises three property tests in this file: `prop_merge_is_deterministic`, `prop_merge_preserves_base_config_fields`, and `prop_resolve_is_deterministic`. The actual file delivers two (the merge ones) plus a fourth `prop_resolver_output_with_none_leaves_key_absent` (good) and a *commented-out stub* for `prop_resolve_is_deterministic` deferring to S-12.05. The commented-out stub is not a runnable proptest — VP-075's "Resolve is deterministic" property is not actually verified anywhere in this story's diff.

**Why it matters:** AC-008's text is satisfied (merge determinism, 200 trials), so AC-008 itself is OK. But the Test Plan and the file's module docstring (lines 7-8: "Three property tests") both promise `prop_resolve_is_deterministic`. The story claims to deliver VP-075's full surface; in reality it delivers only the merge half. If VP-075 has been re-decomposed into VP-075-A/B/C/D parts (the file uses these letters), this should be reflected in the story Test Plan, not silently elided. Otherwise downstream readers (S-12.05 implementer, the verification-coverage matrix) will assume VP-075 is fully covered by S-12.03 and skip the resolve-side proptest.

**Suggested resolution:** Either (a) add a real `prop_resolve_is_deterministic` proptest in this story by exercising a trivial mock `ContextResolver`'s `resolve()` for determinism (the trait surface exists in this story; the resolver_wave_context computation is in S-12.05, but you can use a `FixedResolver` and assert two calls produce identical output); or (b) amend the Test Plan table to mark `prop_resolve_is_deterministic` as deferred to S-12.05 and update the proptest file's module docstring "Three property tests" to "Two merge property tests + one None-value boundary".

### F-S12.03-P1-005 — MEDIUM — `arb_json_object()` strategy uses `0..8` map size, but `arb_resolver_output_with_value()` keys use prefix `"resolver_"` while `arb_json_object()` allows any `[a-z_]{1,16}` key — collision is structurally possible and the determinism proptest may exercise the no-collision path only most of the time, missing the collision path entirely
**File(s):** `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.03/crates/factory-dispatcher/tests/resolver_determinism_proptest.rs:32-68`

**Anchor:** VP-075 / BC-4.12.005 PC5 + INV1.

**Description:** `arb_json_object()` (line 33) generates keys matching `[a-z_]{1,16}` which can produce strings like `"resolver_foo"` (the underscore is in the character class). `arb_resolver_output_with_value()` (line 59) generates keys matching `"resolver_[a-z]{1,16}"`. So while collision is *unlikely*, it is *possible*: the base could randomly contain a key like `"resolver_x"` and the resolver output could be `"resolver_x"`. When this happens, the merge collision callback fires — but `prop_merge_is_deterministic` passes a no-op closure `|_k, _o, _n| {}`, so behavior is fine. However, `prop_merge_preserves_base_config_fields` has a different base strategy (line 138: `[a-z]{1,16}` — no underscore) explicitly to avoid this. The mixed strategy is internally inconsistent.

**Why it matters:** Two concerns: (a) `prop_merge_is_deterministic` may sometimes test the collision path silently — fine for determinism but the trial count of "200" no longer guarantees "200 collision-free determinism trials"; this confuses future debugging. (b) The author of `prop_merge_preserves_base_config_fields` evidently knew underscore was problematic and excluded it; the inconsistency suggests one of the two strategies was authored first and the other not updated.

**Suggested resolution:** Make `arb_json_object()` use `[a-z]{1,16}` (no underscore), matching `prop_merge_preserves_base_config_fields`. Or, if collision *is* desired in `prop_merge_is_deterministic`, add a comment explaining that and add a `prop_assume!(...)` guard to ensure both paths get reasonable coverage.

### F-S12.03-P1-006 — MEDIUM — `merge_resolver_outputs` silently drops static_config that is not a JSON object; the documented contract says `plugin_config` is "always an Object" but the code falls back to empty Map without warning
**File(s):** `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.03/crates/factory-dispatcher/src/resolver.rs:293-298`

```rust
let mut map = match static_config {
    Value::Object(m) => m,
    // If static_config is not an object (should not happen in practice),
    // treat it as an empty object — resolver outputs still apply.
    _ => serde_json::Map::new(),
};
```

**Anchor:** BC-4.12.005 PC1 ("static hooks-registry `plugin_config` ... preserved as-is"), VP-075 (determinism — but this branch silently *discards* input data).

**Description:** If `static_config` is e.g. `Value::Array([1,2,3])` or `Value::String("x")`, the function silently throws it away and returns just the resolver outputs. This is a SOUL #4 violation (silent failures): the function loses data without telemetry, panic, or error return. The comment "should not happen in practice" is exactly the assumption that bites later — the function is `pub` and exposed as a VP-075 proptest target; a future caller (e.g. a malformed payload from S-12.04 WASM resolver) could trigger this path and the dispatcher would silently produce wrong context.

**Why it matters:** The pure-function determinism guarantee holds (same input → same output, including the silent drop), but the function violates BC-4.12.005 PC1's preservation contract for non-object inputs. Compounding: the current proptest strategies always generate `Value::Object`, so this branch is **never exercised**. A regression test should at minimum assert that this case is debug-asserted or returned as an error.

**Suggested resolution:** Either (a) make the signature `merge_resolver_outputs(static_config: serde_json::Map<String, Value>, ...)` so non-object inputs are unrepresentable at the type level; or (b) add `debug_assert!(matches!(static_config, Value::Object(_)))` and a doc-comment line clarifying the production-time invariant. Option (a) is preferred — types are cheaper than discipline.

### F-S12.03-P1-007 — MEDIUM — `resolve_context_for_entry` silently swallows `ResolverError` without invoking any telemetry callback; BC-4.12.004 says failed resolvers contribute no key, but BC-1.13.001 PC2 + BC-4.12.005 INV3 imply the failure should be observable
**File(s):** `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.03/crates/factory-dispatcher/src/resolver.rs:230-235`

```rust
Some(Err(_err)) => {
    // Resolver errored — skip this key; dispatch continues.
}
```

**Anchor:** BC-1.13.001 PC2 ("If `resolvers-registry.toml` is present but malformed... the dispatcher MUST emit a `resolver.load_error` event"), BC-4.12.005 INV3 ("Per BC-4.12.004, a failed resolver contributes nothing to the merge"), BC-4.12.004 (crash isolation — implied telemetry).

**Description:** When a resolver returns `Err(ResolverError::Crashed/Timeout/AbiViolation/...)`, the function discards the error (note `_err` binding) and proceeds without invoking any callback. The function signature accepts only `emit_not_found` — there is no `emit_resolver_error` (or equivalent) callback. This means the *only* observability of a resolver failure today is whatever the trait implementor logs internally — which by construction can't reach the dispatcher's telemetry sink, since the trait is object-safe and Send+Sync but does not carry a logging context.

**Why it matters:** Silent failure of resolver invocations is a SOUL #4 violation. When a resolver crashes (or times out, or returns ABI-invalid output) under S-12.04, the dispatcher will silently dispatch the hook with missing context as if the resolver had returned `Ok(None)`. Operators investigating "why is my context missing?" will have no signal. This is exactly the class of bug the resolver platform was designed to prevent — see ADR-018's fail-loud philosophy.

**Suggested resolution:** Extend the signature: `resolve_context_for_entry(&self, requested_names: &[String], input: &ResolverInput, emit_not_found: impl Fn(&str), emit_resolver_error: impl Fn(&str, &ResolverError))`. Or, return `HashMap<String, Result<Value, ResolverError>>` (or similar) so the caller can decide what to log. Either way, the silent-drop branch must become observable. Add a unit test that registers a resolver returning `Err(...)` and asserts the error callback fires.

## Cross-cutting observations

**Theme 1 — Story scope vs. delivered scope mismatch (informs F-001, F-004):** The story spec is explicit that S-12.03 delivers (a) the trait/registry, (b) the merge function, AND (c) executor wiring. The branch delivers (a) and (b) but completely omits (c). The proptest file's "Three property tests" docstring + the Test Plan table list more proptests than are actually delivered. Both gaps suggest the story was sliced down during implementation but neither the spec, Test Plan, nor module docstring were updated to reflect that. Either the implementation is incomplete or the spec needs amendment — the burden is on the author to pick one.

**Theme 2 — Telemetry callback shape is asymmetric (informs F-002, F-007):** The current API surface emits telemetry only for "not found" via callback, while merge emission goes through a different `on_collision` callback in `merge_resolver_outputs`, and resolver-invocation errors emit nothing. Three different shapes for three event categories will be brittle to wire together at the executor.rs layer once F-001 is fixed. Consider unifying via a single `ResolverTelemetry` trait or sink with three methods.

**Theme 3 — POLICY 10 (demo_evidence_story_scoped):** No flat `.md` files at `docs/demo-evidence/*.md` (verified via Glob). `docs/demo-evidence/S-12.03/` does not yet exist, which is correct because Step 5 follows convergence. POLICY 10: PASS.

**Theme 4 — POLICY 11 (no_test_tautologies):** Inspected `resolver_registry_test.rs` — every `test_BC_*` function calls a production fn (`registry.register`, `registry.resolve_context_for_entry`, `merge_resolver_outputs`, `Registry::parse_str`). `resolver_determinism_proptest.rs` proptest bodies call `merge_resolver_outputs`. POLICY 11: PASS.

**Theme 5 — POLICY 12 (bc_tv_emitter_consistency):** BC-1.13.001 Canonical Test Vectors and BC-4.12.005 Canonical Test Vectors do not declare any field as "excluded" — the tables list all fields under "Static plugin_config", "Resolver Outputs", "Expected Merged plugin_config" with no exclusions. `RegistryEntry` fields are all part of the schema. POLICY 12: PASS (vacuously — no excluded fields declared).

**Theme 6 — Rebase mechanical resolution:** Verified that `RegistryEntry` correctly carries both `async_flag: bool` (S-15.01) and `needs_context: Vec<String>` (this story), with both `#[serde(default)]`. All five test fixture sites that hand-build `RegistryEntry` (executor_integration.rs:49, async_partition_integration.rs:51,70, full_stack_plugin_invocation.rs:89,115, partition.rs:120 Kani harness) explicitly set `needs_context: vec![]`. Rebase resolution: clean. (Pre-existing artifact: `executor_integration.rs:72` hand-builds `Registry { schema_version: 1, ... }` — inconsistent with `REGISTRY_SCHEMA_VERSION = 2`. Not S-12.03 scope; the hand-built path bypasses `validate()` so no current failure, but flag for a subsequent cleanup.)

## Convergence assessment
- **Within-story findings:** 7
- **Severity floor:** CRITICAL (one finding at CRITICAL — F-001)
- **Classification:** **CRITICAL**
- **Reasoning:** F-001 is a bona-fide spec-implementation gap: six of twelve ACs (AC-002, AC-003, AC-005, AC-007, AC-009, AC-010) are written against `executor.rs` integration behavior that is entirely absent from the diff. The story Definition of Done lists "Pre-dispatch resolver step in `executor.rs` with `needs_context.is_empty()` short-circuit" as a separate bullet from the trait/registry bullet, and Tasks T-2 and T-10 explicitly call out modifying executor.rs. The branch's tests pass cargo only because they exercise the trait/registry layer in isolation — the integration is unverified and unimplemented. F-002 (wrong error variant) and F-003 (contradictory doc) are HIGH and would be MEDIUM in isolation but compound with F-001's incomplete implementation to indicate the story is at "trait surface scaffolding" stage rather than "ready for convergence" stage. Three additional MEDIUM findings (F-005/F-006/F-007) target test strategy precision and silent-failure pathways. **Recommendation: PROCEED_TO_FIX** — fix F-001 by either implementing executor wiring or amending the spec; address F-002 + F-003 by introducing a `DuplicateName` variant and correcting doc; address F-007 by adding a resolver-error callback path; tighten F-005 and F-006 in the same fix burst. After fix, re-run pass 2.
