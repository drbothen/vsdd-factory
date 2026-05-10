# Adversarial Review — S-12.05 Pass 2

## Metadata
- **Story:** S-12.05 hook-sdk resolver-authoring extensions
- **Branch SHA reviewed:** 57681db7
- **Pass:** 2
- **Reviewer:** adversary (fresh context)
- **Classification:** HIGH
- **Within-story finding count:** 8 (5 actionable + 3 LOW)
- **Recommendation:** PROCEED_TO_FIX

## Findings

### F-S12.05-P2-001 — HIGH — AC-005 trybuild test does NOT verify the WASM `resolve()` export, defeating the postcondition's purpose
**File(s):** `crates/hook-sdk-macros/src/resolver_macro.rs:66-92`, `crates/hook-sdk/tests/ui/valid_resolver.rs:1-23`, `crates/hook-sdk/tests/resolver_types_test.rs:264-270`
**Anchor:** AC-005 / BC-4.12.002 PC5
**Description:** `#[resolver]` macro now wraps generated `pub extern "C" fn resolve(...)` in `#[cfg(target_arch = "wasm32")]`. Trybuild test `tests/ui/valid_resolver.rs` runs on host arch where the entire block is compiled out. What macro emits on host: just user's `resolve_impl` plus `fn main() {}`. Trybuild "pass" does NOT prove `resolve()` export generated — merely proves macro doesn't reject valid signature. AC-005 explicitly requires "assert the crate compiles without error and the generated `resolve` export is present in the WASM exports (`wasm-objdump -x` or equivalent)."
**Why it matters:** Packed-i64 ABI logic — highest-risk part of story per BC-4.12.002 PC5 — entirely unverified. Future refactor breaking `(output_ptr << 32) | output_len` encoding would compile cleanly on host and pass all tests. Silent-failure surface (SOUL #4).
**Suggested resolution:** Add `wasm32-wasip1` cross-compilation test — either (a) `tests/wasm32_resolver_export.rs` integration test shelling out to `cargo build --target wasm32-wasip1 -p <fixture>` + `wasm-objdump -x` (or `wasmparser`) asserting `resolve` is exported, or (b) example crate under `crates/hook-sdk/examples/` whose CI build target asserts export. Update AC-005 falsifiable test text to match.

### F-S12.05-P2-002 — HIGH — `Resolver` trait introduced in `resolver.rs` is not part of any AC, BC anchor, or test — surface drift from BC-4.12.002
**File(s):** `crates/hook-sdk/src/resolver.rs:44-50`, `crates/hook-sdk/CHANGELOG.md:7`
**Anchor:** BC-4.12.002 (story's only declared BC) — does not specify Resolver trait. CHANGELOG attributes trait to BC-4.12.001, NOT in story `behavioral_contracts:` frontmatter.
**Description:** New `pub trait Resolver { fn context_key(&self) -> &str; fn resolve(&self, input: ResolverInput) -> ResolverOutput; }` added to public SDK and re-exported via `pub use resolver::*;`. Story spec lists parenthetically once but no AC traces, no test references (zero matches in tests/). BC-4.12.002 doesn't introduce. CHANGELOG attributes to BC-4.12.001 outside story scope.
**Why it matters:** Adding exported trait to publicly-published crate creates stability commitment without architectural review or test coverage. Anchor drift: API additions must trace to BC in `bcs:` frontmatter. If trait was authoring intent, BC-4.12.001 should be in `bcs:` (and body) — currently not.
**Suggested resolution:** (a) Remove `Resolver` trait — unused in story tests, S-12.07 can introduce when consuming. (b) Add BC-4.12.001 to story's `bcs:` frontmatter array AND body's Behavioral Contracts table, add AC for trait shape, add test constructing type implementing `Resolver`.

### F-S12.05-P2-003 — MEDIUM — `tests/ui/wrong_sig.stderr` pins compiler line numbers (`:20:1`) — fragile to whitespace changes
**File(s):** `crates/hook-sdk/tests/ui/wrong_sig.stderr:2,4`
**Anchor:** AC-006 / BC-4.12.002 PC5
**Description:** `.stderr` golden file pins diagnostic source location: `--> tests/ui/wrong_sig.rs:20:1`. Any future edit to comment block in `wrong_sig.rs` shifts `#[resolver]` attribute off line 20 and breaks trybuild. rustc version drift would do same.
**Why it matters:** Known fragility in trybuild negative tests. Remediation either non-line-pinned form OR comment in source warning maintainers. Currently neither.
**Suggested resolution:** Add comment immediately above `#[resolver]` attribute (`// !! Do not change line count above — wrong_sig.stderr pins line 20.`), OR refactor macro error to use `syn::Error::new_spanned(&f.sig, ...)` against function signature span (more stable than attribute span) and update `.stderr`. Document rustc version that produced golden file in `.stderr` comment.

### F-S12.05-P2-004 — MEDIUM — `test_BC_4_12_002_resolver_input_is_not_hook_payload` relies on `mem::size_of` inequality — coincidence-fragile
**File(s):** `crates/hook-sdk/tests/resolver_types_test.rs:229-233`
**Anchor:** AC-004 / BC-4.12.002 INV1
**Description:** Structural witness for "ResolverInput and HookPayload distinct" is `assert_ne!(std::mem::size_of::<ResolverInput>(), std::mem::size_of::<HookPayload>())`. Two structurally distinct Rust types can coincidentally have same `size_of` (padding, identical primitive layouts). If future SubagentStop extension brings HookPayload size into alignment, assert silently passes false guarantee.
**Why it matters:** Tautology-adjacent: size comparison not meaningful structural distinctness check. Authoritative INV1 check is trybuild compile-fail in `type_mismatch.rs` — already exists. size_of adds noise without coverage. Per POLICY 11, test_BC_* functions must call production fn with discriminating power.
**Suggested resolution:** Remove `assert_ne!(size_of...)` — trybuild already authoritative. Or replace with `TypeId::of::<ResolverInput>() != TypeId::of::<HookPayload>()` (still weak), or simply delete and let trybuild be AC-004 anchor.

### F-S12.05-P2-005 — MEDIUM — Story spec File List & DoD reference `ResolverError` as exported from `resolver.rs`, contradicting final implementation
**File(s):** `.factory/stories/S-12.05-hook-sdk-resolver-extensions.md:50, 146, 163, 271`, `crates/hook-sdk/CHANGELOG.md:20-21`, `crates/hook-sdk/Cargo.toml:23`
**Anchor:** Story spec / BC-4.12.004 boundary
**Description:** Story narrative (line 50: "the SDK to provide ... `ResolverInput`, `ResolverOutput`, `ResolverError` types"), Architecture Mapping table (line 146), File List (line 163: "ResolverError structs ... in resolver.rs"), Definition of Done (line 271) all assert `ResolverError` ships from `crates/hook-sdk/src/resolver.rs`. Implementation does NOT — CHANGELOG, Cargo.toml comment, HOST_ABI.md all consistently state `ResolverError` is host-side type owned by `factory-dispatcher`. Frontmatter-Body / Spec-Implementation Coherence violation.
**Why it matters:** Reviewers checking DoD against impl find missing export and either (a) reject convergence incorrectly, or (b) silently accept drift, contaminating spec for downstream (S-12.07 reads to know SDK surface).
**Suggested resolution:** Update story spec body in three places: (1) narrative line 50 — drop `ResolverError` from SDK-provided list, OR add clarifying parenthetical that `ResolverError` is host-side; (2) Architecture Mapping table — remove `ResolverError` row; (3) Definition of Done — strike `ResolverError` from resolver.rs created list. Add CHANGELOG entry on story (v1.1) noting F-burst correction.

### F-S12.05-P2-006 — LOW — `unsafe` not validated by macro: `unsafe fn resolve_impl` would generate code without diagnostic
**File(s):** `crates/hook-sdk-macros/src/resolver_macro.rs:111-141` (compare to `crates/hook-sdk-macros/src/lib.rs:74-93`)
**Anchor:** BC-4.12.002 PC5 ("user function MUST have signature `fn resolve_impl(input: ResolverInput) -> ResolverOutput`")
**Description:** `#[hook]` macro's `validate_signature` rejects `async fn` and `unsafe fn` with explicit diagnostics. Parallel `validate_resolver_signature` only checks arg count and return type. `unsafe fn resolve_impl` or `async fn resolve_impl` would pass. Confusing downstream errors instead of clear macro diagnostics.
**Why it matters:** Story Architecture Compliance Rule #4: "fn resolve_impl signature exact per BC-4.12.002 PC5". Parity with `#[hook]` is design intent.
**Suggested resolution:** Add asyncness/unsafety checks mirroring `validate_signature`. Add third trybuild test `tests/ui/async_resolver.rs` with matching `.stderr`.

### F-S12.05-P2-007 — LOW — AC-010 proptest silently narrows the universal claim by filtering `Some(Value::Null)`; AC text not updated
**File(s):** `crates/hook-sdk/tests/resolver_types_test.rs:493-515`, `.factory/stories/S-12.05-hook-sdk-resolver-extensions.md:138`
**Anchor:** AC-010 / VP-075
**Description:** `arb_resolver_output` strategy filters `Some(Value::Null)` via `.prop_filter("Some(Null) excluded ...", |v| !v.is_null())`. AC-010 text reads "deserialize(serialize(input)) == input" without carve-out. Reviewer reading AC-010 gets impression of unconditional roundtrip determinism; actual proptest excludes known divergent case. `.proptest-regressions` confirms this case was previously a witness.
**Why it matters:** Frontmatter-Body axis: AC asserts more than test verifies.
**Suggested resolution:** Update AC-010 text to explicitly exclude `Some(Value::Null)` with one-sentence rationale referencing EC-001.

### F-S12.05-P2-008 — LOW — Tests import `#[resolver]` from `vsdd_hook_sdk_macros` directly, bypassing the feature-gated re-export
**File(s):** `crates/hook-sdk/tests/ui/wrong_sig.rs:18`, `crates/hook-sdk/tests/ui/valid_resolver.rs:15`, `crates/hook-sdk/src/lib.rs:42-45` (no `pub use vsdd_hook_sdk_macros::resolver;`)
**Anchor:** BC-4.12.002 PC8 / AC-007
**Description:** Both trybuild fixtures `use vsdd_hook_sdk_macros::resolver;` directly. End users per HOST_ABI.md should import via `vsdd_hook_sdk::resolver`. Existing `#[hook]` IS re-exported (`pub use vsdd_hook_sdk_macros::hook;` line 52), but `#[resolver]` is NOT re-exported even under `resolver-authoring` feature gate. AC-007 says "A crate without `resolver-authoring` feature MUST NOT have access to `#[resolver]`" — currently any consumer pulling in `vsdd-hook-sdk-macros` directly gets `#[resolver]` regardless of features.
**Why it matters:** AC-007 structurally violated. POLICY 12 adjacent: BC-4.12.002 PC8's intent violated.
**Suggested resolution:** Add `#[cfg(feature = "resolver-authoring")] pub use vsdd_hook_sdk_macros::resolver;` to `crates/hook-sdk/src/lib.rs` adjacent to lines 42-45. Update `valid_resolver.rs` and `wrong_sig.rs` to `use vsdd_hook_sdk::resolver;` (matching HOST_ABI.md guidance line 1002). Coordinate with F-S12.05-P2-003 (line numbers shift).

## Cross-cutting observations

- [process-gap] AC-005 falsifiability template doesn't enforce target-arch coverage. The story-writer prompt accepted "trybuild test ... assert generated `resolve` export present in WASM exports" but implementation answered with host-arch trybuild that cannot detect export presence. No template gate "if BC postcondition references wasm32-* artifact behavior, falsifiable test MUST run against that target." Consider rule in `rules/lessons-codification.md`: "WASM-export postconditions REQUIRE `cargo build --target wasm32-*` step + objdump-style assertion. Host-arch trybuild not sufficient."
- Macro generates `fn main() {}` even on success path (resolver_macro.rs:96). For real WASM resolver crates declared as `[[bin]]` (matching `#[hook]`), fine. For `cdylib` crates (typical WASM exports without `_start`), `main()` is dead code. Story spec doesn't pin crate type expected.
- CHANGELOG attributes `Resolver` trait to BC-4.12.001 but story doesn't anchor BC-4.12.001. If F-P2-002 resolved by removing trait, drops away. If kept, frontmatter MUST add BC-4.12.001 to `bcs:` per Frontmatter-Body Coherence.
- POLICY 10 not blocked: no `docs/demo-evidence/S-12.05/` yet, story mid-cycle (per-story adversary not complete). Consistent with policy.

## Convergence assessment
- Within-story findings: 8 (2 HIGH, 2 MEDIUM, 3 LOW + 1 process-gap obs)
- Severity floor: HIGH
- Classification: HIGH
- Reasoning: Two HIGH findings — F-001 (AC-005 silent-failure: WASM export untested — packed-i64 ABI unverified) and F-002 (`Resolver` trait without BC anchor or test coverage) — would individually justify non-NITPICK classification. F-001 stronger: verification gap on highest-risk story part where green light gives false signal. MEDIUM findings include real spec-impl drift (`ResolverError` references) and test-quality issues. Pass 2 identified novel substantive problems beyond rubber-stamp. Recommend fix-burst on F-001/F-002/F-005 (and F-008 since small) followed by Pass 3.
