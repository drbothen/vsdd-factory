# Adversarial Review — S-12.05 Pass 3

## Metadata
- **Story:** S-12.05 hook-sdk resolver-authoring extensions
- **Branch SHA reviewed:** 13f7413c
- **Pass:** 3
- **Reviewer:** adversary (fresh context)
- **Classification:** LOW
- **Within-story finding count:** 4
- **Recommendation:** PROCEED_TO_FIX

(Adversary noted "NITPICK_ONLY" but per strict severity rule — any LOW finding makes classification LOW, not NITPICK_ONLY. Orchestrator records LOW.)

## Findings

### F-S12.05-P3-001 — LOW — `valid_resolver.rs` trybuild does not assert `resolve` export presence on host
**File(s):** `crates/hook-sdk-macros/src/resolver_macro.rs:66-92`; `crates/hook-sdk/tests/ui/valid_resolver.rs`; `crates/hook-sdk/tests/resolver_types_test.rs:271`
**Description:** On host (trybuild compile target), `#[cfg(target_arch = "wasm32")]` block elides entire `extern "C" fn resolve` generation. `t.pass(...)` only verifies macro doesn't error on valid signature; never observes export generated. AC-005's "generated `resolve` export is present in WASM exports" delegated entirely to `wasm32_resolver_export_integration.rs::test_BC_4_12_002_resolver_macro_generates_wasm_export`, which is `#[ignore]`d. Workspace WASM build at `.github/workflows/ci.yml:155-162` does compile the example crate (a `cdylib`) — covers the compile path. Only the named-export wasmparser inspection unverified in CI.
**Suggested resolution:** Add one-line comment on `valid_resolver.rs` near `#[resolver]` site noting trybuild only verifies macro acceptance on host; export-name check lives in `wasm32_resolver_export_integration.rs`.

### F-S12.05-P3-002 — LOW — `wasm32_resolver_export_integration.rs` recursively spawns `cargo build` while `cargo test` is running
**File(s):** `crates/hook-sdk/tests/wasm32_resolver_export_integration.rs:43-54`
**Description:** Test invokes `Command::new("cargo").args(["build", "--target", "wasm32-wasip1", "-p", "wasm-resolver-export", "--release"]).current_dir(&workspace_root)`. Spawning `cargo build` inside running `cargo test` is anti-pattern: lock-file may serialize build (slow), or with different CARGO_TARGET_DIR layout, races on `target/.rustc_info.json`. Test is `#[ignore]`d so doesn't affect default CI. If anyone removes `#[ignore]`, may hit "blocking waiting for file lock on build directory".
**Suggested resolution:** Add comment warning that running this test must be done after top-level `cargo build --target wasm32-wasip1 -p wasm-resolver-export --release` (recursive cargo invocation = no-op cache hit). Current header instructs users to "build the example crate" before `cargo test`, but doesn't say test will ALSO try to rebuild — making prebuild seem optional.

### F-S12.05-P3-003 — LOW — `arb_resolver_input` proptest does not test `Some(Null)` carve-out for `agent_type`
**File(s):** `crates/hook-sdk/tests/resolver_types_test.rs:480-499` (`arb_resolver_input`); 516-530 (`arb_resolver_output`)
**Description:** `arb_resolver_output` documents `Some(Null)` exclusion with verbose justification anchored to BC-4.12.002 EC-001 (lines 502-515). `arb_resolver_input.agent_type: Option<String>` also subject to similar serde semantics — but `Option<String>` does NOT have same JSON-null degeneracy as `Option<Value>`, because `null` deserializes back to `None` and `Some("null")` deserializes back to `Some("null")` (a string). Input proptest correctly unconstrained — but future maintainer could mistakenly think same carve-out needed.
**Suggested resolution:** One-line comment on `arb_resolver_input` confirming "Option<String> has no Some(Null) ambiguity, unlike Option<Value> below" — prevent symmetric edits.

### F-S12.05-P3-004 — LOW — Story spec body's File List still references `Resolver` trait deleted in pass-2
**File(s):** `.factory/stories/S-12.05-hook-sdk-resolver-extensions.md:174` (File List); `.factory/specs/behavioral-contracts/ss-04/BC-4.12.002.md:178` (Architecture Anchors)
**Description:** Architect Path B in pass-2 chose to DELETE the Resolver trait. Implementation matches. Story File List still claims `Resolver trait (optional architectural companion to Hook trait)`. BC-4.12.002 Architecture Anchors row says `Resolver trait ...`. CHANGELOG v1.1 entry on line 319 only calls out ResolverError removal, not Resolver trait removal. Body propagation incomplete (S-7.01 partial-fix-regression class).
**Severity classification:** LOW per intent: references are descriptive prose, not testable obligations or implementer instructions, so HIGH would be excessive. But still within-story per Iron Law of perimeter scope.
**Suggested resolution:** Update File List entry on line 174 to drop "Resolver trait" reference. Update BC-4.12.002 Architecture Anchors row to drop "Resolver trait" mention. Story v1.2 + CHANGELOG entry.

## Cross-cutting observations
- O-1 — Convergence quality high. Pass-2 fix burst landed substantive engineering: compilable wasm32 example crate registered as workspace member, wasmparser-based integration test, asyncness/unsafety guards in macro with matching trybuild fixtures, line-number pinning via comment block, deletion of size_of tautology, documented Some(Null) proptest carve-out. POL-10/11/12 axes clean.
- O-2 — Trybuild stderr fixtures pinned correctly. Line 20 of wrong_sig.rs verified at column 1; --> tests/ui/wrong_sig.rs:20:1 in wrong_sig.stderr matches. async_resolver.stderr line 14 also matches.
- O-3 — Feature gate correctly bidirectional. lib.rs lines 42-45 + 53-54 gate via `#[cfg(feature = "resolver-authoring")]`. AC-007 negative-compile-test concern mitigated by structural source-scan.
- O-4 — `#[resolver]` attribute name shadowing benign but unobvious. Both `pub mod resolver` and `pub use vsdd_hook_sdk_macros::resolver` exist in `vsdd_hook_sdk` root namespace. Rust namespace separation makes this work. Future maintainer awareness only.
- O-5 — Workspace WASM build covers example crate compile path. .github/workflows/ci.yml line 155-162 doesn't exclude wasm-resolver-export. Macro regression that emits invalid wasm32 code fails CI. Export-name check (wasmparser inspection) only behind `#[ignore]`.

## Convergence assessment
- Within-story findings: 4 (all LOW)
- Severity floor: LOW
- Classification: LOW
- Reasoning: Pass-2 fix burst landed clean. All 4 findings are advisory comment improvements + 1 spec body line referencing deleted trait. Path to NITPICK_ONLY in pass-4: address F-P3-004 spec edit (story-writer one-liner) + 3 LOW comment improvements.
- Recommendation: PROCEED_TO_FIX
