# Adversarial Review — S-12.03 Pass 2

## Metadata
- **Story:** S-12.03 ContextResolver trait + ResolverRegistry
- **Branch SHA reviewed:** fa8babed
- **Pass:** 2
- **Reviewer:** adversary (fresh context)
- **Classification:** CRITICAL
- **Within-story finding count:** 7
- **Recommendation:** PROCEED_TO_FIX

## Findings

### F-S12.03-P2-001 — CRITICAL — `resolve_context_for_entry` returns `HashMap` and breaks declaration-order invariant + VP-075 determinism
**File(s):** `crates/factory-dispatcher/src/resolver.rs:225-253` (signature), `crates/factory-dispatcher/src/executor.rs:512-527` (call site)
**Anchor:** BC-1.13.001 PC7; BC-4.12.005 PC4 + INV1 (VP-075).
**Description:** `ResolverRegistry::resolve_context_for_entry` returns `HashMap<String, Value>`. In `build_plugin_config` (executor.rs:519-527), this map is converted back to `Vec<ResolverOutput>` via `.into_iter()`. `HashMap::into_iter` does NOT preserve insertion order. The declaration order from `needs_context: ["a", "b", "c"]` is lost before the merge — the output that "wins" inter-resolver collisions per BC-4.12.005 PC4 last-write-wins is non-deterministic across runs. The proptest passes only because it constructs the `Vec<ResolverOutput>` directly, bypassing the HashMap roundtrip used in production.
**Why it matters:** VP-075 is meaningless if the inputs to `merge_resolver_outputs` are themselves randomly ordered. `resolver.merge_collision` event payload also non-deterministic. AC-009's test only registers two resolvers — flaky test, not a correctness test.
**Suggested resolution:** Change `resolve_context_for_entry` to return `Vec<(String, Value)>` (or `Vec<ResolverOutput>` directly) preserving the iteration order over `requested_names`. Drop the HashMap. Update AC-009 to verify order preservation through the production path.

### F-S12.03-P2-002 — CRITICAL — Unjustified `unsafe { &*internal_log_ref }` raw-pointer dereference in `build_plugin_config`
**File(s):** `crates/factory-dispatcher/src/executor.rs:480-541` (three `unsafe` blocks)
**Anchor:** SOUL.md (no silent footguns); story Architecture Compliance Rules §5.
**Description:** `build_plugin_config` constructs three closures that capture `internal_log` as a raw `*const InternalLog` pointer with `unsafe { &*internal_log_ref }.write(&ev)`. The SAFETY comments rationalise with "internal_log outlives this closure" — but `InternalLog: Clone` (internal_log.rs:217 derives `Debug, Clone`) and is just a `PathBuf` wrapper. There is zero technical reason for the raw pointer. The unsafe is also wrong-by-construction: closures pass to callees as `impl Fn(...)` which could in principle store + invoke later (today they don't, but a future async refactor would silently UB).
**Why it matters:** Correctness-fragile pattern in security-critical dispatch path. SOUL #4 forbids silent failures; UB regression here is a silent failure. Cargo clippy with `-D warnings` does not flag deref of a raw pointer; this hole is invisible to the lint gate.
**Suggested resolution:** Replace raw pointer with `let log = internal_log.clone();` captured by `move` into each closure (PathBuf clone — well within dispatch budget). Remove all three `unsafe` blocks. Add `#![deny(unsafe_code)]` crate attribute to `factory-dispatcher` to prevent regression.

### F-S12.03-P2-003 — HIGH — `ResolverError` derives no `Serialize`/`Deserialize`/`PartialEq`/`Clone`, breaking ABI parity with `ResolverInput`/`ResolverOutput`
**File(s):** `crates/factory-dispatcher/src/resolver.rs:70-112`
**Anchor:** Story Dev Notes; BC-4.12.002 (resolver ABI); BC-4.12.004 (resolver-error isolation).
**Description:** `ResolverInput`/`ResolverOutput` derive `Serialize, Deserialize, PartialEq` (lines 30, 53). `ResolverError` derives only `Debug, Error`. Cannot round-trip across WASM boundary that S-12.04 will introduce. Cannot be compared in tests with `assert_eq!`. The `Io(#[from] std::io::Error)` variant cannot derive `PartialEq`/`Clone` because `std::io::Error` doesn't implement them — design smell suggesting a future retrofit will break ABI.
**Why it matters:** Locking in non-serializable, non-comparable error type creates work for S-12.04 manifesting as breaking change. Tests cannot easily assert variant identity without `matches!()` boilerplate.
**Suggested resolution:** (a) Drop `Io` variant — `resolver.rs` is in-memory layer per Forbidden Dependencies. Move `Io` to S-12.04 in `ResolverLoadError`. (b) Derive `Serialize, Deserialize, Clone, PartialEq`. (c) Use `#[serde(tag = "kind")]` for forward-compat wire format.

### F-S12.03-P2-004 — HIGH — `prop_resolve_is_deterministic` is a tautology and violates POLICY 11
**File(s):** `crates/factory-dispatcher/tests/resolver_determinism_proptest.rs:249-313`
**Anchor:** POLICY 11; VP-075; AC-008.
**Description:** `FixedDeterministicResolver::resolve` ignores its input and returns a stored value. Proptest calls `resolver.resolve(&input)` twice and asserts equality — testing that `Option<ResolverOutput>::clone()` is deterministic. Does not exercise `ResolverRegistry`, `merge_resolver_outputs`, or `resolve_context_for_entry`. POLICY 11 explicitly bans test_BC_*/test_TV_* functions that "construct and assert on its own data".
**Why it matters:** VP-075 promised "context-injection determinism" — requires (a) real production resolver impl (lands in S-12.07), or (b) property that registry preserves determinism. Current shape gives green checkmark on vacuous property — survives into v1.0 as a "covered VP" with no actual coverage.
**Suggested resolution:** (a) Delete `prop_resolve_is_deterministic` and `FixedDeterministicResolver`; defer VP-075-A to S-12.07 in VP-INDEX with note. (b) Add `//! tautology-allowed: VP-075-A placeholder; real resolver lands in S-12.07` per POLICY 11. (c) Re-cast as `prop_registry_resolve_preserves_resolver_determinism`: register `FixedDeterministicResolver`, call `resolve_context_for_entry` twice with same input, assert resulting maps equal — exercises registry path.

### F-S12.03-P2-005 — HIGH — `merge_resolver_outputs` mutates `static_config` parameter; signature claims pure but takes ownership and is not idempotent across signatures
**File(s):** `crates/factory-dispatcher/src/resolver.rs:301-322`
**Anchor:** BC-4.12.005 INV1 (`merge(static_config: Value, resolver_outputs: Vec<ResolverOutput>) -> Value`); story AC-008.
**Description:** Story spec AC-008 and BC-4.12.005 INV1 both specify `static_config: Value` and return `Value`. Actual signature is `(static_config: serde_json::Map<String, Value>, resolver_outputs: &[ResolverOutput], on_collision: impl Fn(&str, &Value, &Value)) -> serde_json::Map<String, Value>`. Doc comment at lines 282-285 acknowledges divergence ("typed as `serde_json::Map<String, Value>` (not the broader `Value` enum)") with F-006 reference, but divergence not reflected in BC text or AC text. Either BC or implementation is wrong. Additionally, `on_collision` callback makes function NOT pure by BC-4.12.005 INV1 definition ("No I/O, no side effects, no global state"). Production passes `internal_log.write(&ev)` closure — file I/O during merge step. **Production merge calls perform file I/O** — direct violation of BC-4.12.005 INV1.
**Why it matters:** (a) BC-spec drift. (b) "Pure function" claim formally false in production — file I/O happens inside merge. VP-075 proptest only proves purity *given pure callback*, not that production merge is pure. Reviewer trusting "VP-075 verifies determinism of merge" is being misled.
**Suggested resolution:** (i) Update BC-4.12.005 INV1 to "merge is pure modulo telemetry sink callback, telemetry non-blocking and order-irrelevant" (document in BC v1.2 changelog), or (ii) Restructure: `merge_resolver_outputs` returns `(merged_map, Vec<CollisionInfo>)` — caller emits telemetry. Option (ii) preserves VP-075's testable claim. Option (i) admits impurity.

### F-S12.03-P2-006 — MEDIUM — `ResolverError::Crashed`/`Timeout`/`AbiViolation`/`CapabilityDenied`/`Malformed` are dead variants
**File(s):** `crates/factory-dispatcher/src/resolver.rs:78-99`
**Anchor:** Story scope ("In-memory resolver layer only — no WASM loading").
**Description:** Of 8 variants, only `NotFound` and `DuplicateName` are constructible. Others are pre-emptive scaffolding for S-12.04. `#[non_exhaustive]` not present, so external pattern-matches require exhaustive arms covering all 8.
**Why it matters:** Violates story's own scope guidance. Locking in variant shapes creates ABI churn risk (e.g., `Crashed` likely needs `trap_string` field to match `PluginResult::Crashed`).
**Suggested resolution:** Add `#[non_exhaustive]`. Or trim enum to `NotFound`+`DuplicateName`; let S-12.04 add WASM variants with right shapes.

### F-S12.03-P2-007 — MEDIUM — `noop_emit`/`noop_error` test helpers + `ContextResolver` trait usage produce tautology pattern in 8 of 12 ACs
**File(s):** `crates/factory-dispatcher/tests/resolver_registry_test.rs:99-102`, used in tests AC-002, AC-003, AC-004, AC-006, AC-007, AC-009, AC-010, AC-011
**Anchor:** POLICY 11; BC-1.13.001 PC2; AC-005.
**Description:** All AC tests pass `noop_emit`/`noop_error` *except* AC-005 (which captures not_found name). Only AC-005 verifies registry actually invokes `emit_not_found`. No test verifies `emit_resolver_error` is invoked when resolver returns `Err(...)`. The promise "we emit telemetry on failed resolver" is undertested. `executor_resolver_integration.rs::CountingResolver` only ever returns `Ok(Some(...))`.
**Why it matters:** SOUL #4 silent-failure regression vector. Telemetry is operational visibility — losing silently is worst kind of bug.
**Suggested resolution:** Add unit test in `resolver_registry_test.rs`: register `ErroringResolver` returning `Err(ResolverError::Crashed{...})`, dispatch with `needs_context: ["foo"]`, capture `emit_resolver_error` calls, assert invoked once with right name+variant. Add complementary test asserting `emit_not_found` is NOT called when resolver IS registered. Add integration test in `executor_resolver_integration.rs` with erroring resolver.

## Cross-cutting observations
- POLICY 10: not blocking, demo dir not yet present (post-convergence).
- POLICY 12: ResolverRegistry/RegistryEntry derive Serde, no field excluded in BC TV. Pass.
- AC-005 telemetry assertion verifies callback invocation but not `InternalEvent` wire shape (`type: "resolver.not_found"`). Wire format unverified.
- `async_partition_integration.rs` and `executor_integration.rs` sibling propagation: `Arc::new(ResolverRegistry::new())` insertions correct (empty registries with `needs_context: []` → zero-overhead path).
- Test naming convention drift [process-gap]: `test_BC_1_13_001_ac001_*` style mixes BC ID and AC ID, dot-vs-underscore rendering inconsistent.

## Convergence assessment
- Within-story findings: 7 (2 CRITICAL, 3 HIGH, 2 MEDIUM)
- Severity floor: CRITICAL
- Classification: CRITICAL
- Reasoning: F-001 breaks BC-1.13.001 PC7 declaration-order invariant in production via HashMap roundtrip (invalidates VP-075 production claim). F-002 introduces unjustified `unsafe` raw-pointer dereferences in security-critical dispatch path with zero technical reason. Three HIGH findings concern test-coverage gaps and BC/impl drift. Recommend PROCEED_TO_FIX.
