# Adversarial Review — S-12.05 Pass 4

## Metadata
- Story: S-12.05 hook-sdk Resolver-Authoring Extensions
- Branch SHA reviewed: 8b23840c
- Pass: 4
- Reviewer: adversary (fresh context)
- Classification: MEDIUM
- Within-story finding count: 5 (1 MEDIUM, 1 LOW, 3 NITPICK)
- Recommendation: PROCEED_TO_FIX

## Findings

### F-S12.05-P4-001 — MEDIUM — `crates/hook-sdk/Cargo.toml:22` references the deleted `Resolver trait` (S-7.01 partial-fix-regression)
**File(s):** `crates/hook-sdk/Cargo.toml:22`
**Description:** Pass-3 fix burst (F-P3-004 v1.2) propagated "Resolver trait deleted" decision to BC-4.12.002 Architecture Anchors and Story File List, but missed Cargo.toml feature doc-comment. Line 22:
```
## Gates ResolverInput, ResolverOutput, RESOLVER_ABI_VERSION, and the Resolver trait.
```
Resolver trait was deleted in pass-2 per F5-pass-2-architect-decisions.md "Decision 2 / Path B". Module contains only RESOLVER_ABI_VERSION + ResolverInput + ResolverOutput. Documentation lies to readers. S-7.01 partial-fix-regression: spec-side artifacts updated but implementation-side artifact in same architectural layer missed. Blast radius >1 file → MEDIUM.
**Suggested resolution:** Replace line 22 with `## Gates ResolverInput, ResolverOutput, and RESOLVER_ABI_VERSION.`

### F-S12.05-P4-002 — LOW — `crates/hook-sdk/tests/resolver_types_test.rs` module docstring lies about Red-Gate state
**File(s):** `crates/hook-sdk/tests/resolver_types_test.rs:1, 14-15, 25-27, 256, 261, 270`
**Description:** Macro is fully implemented (resolver_macro.rs:33-100, no todo!()). Test-file documentation still claims Red-Gate stub harness. Multiple lines including "S-12.05 Step 3 (Red Gate)", "trybuild — FAILS in Red Gate", "AC-005 and AC-006 trybuild tests FAIL because the #[resolver] macro body is `todo!()`", "RED GATE: This test FAILS because the macro body is `todo!()`."
Reader misled into thinking AC-005/006 are expected-failing today. Tests are GREEN. Direct contradiction code↔prose. SOUL #4-adjacent: misleading docs can mask future regressions where macro genuinely breaks but reviewer treats failure as expected.
**Suggested resolution:** After GREEN, file should read "S-12.05 GREEN — covers AC-001..AC-010". Remove every Red Gate / todo!() reference. Mirror cleanup at tests/ui/valid_resolver.rs:8-10.

### F-S12.05-P4-003 — LOW — `tests/ui/valid_resolver.rs:8-10` claims macro is `todo!()` but is fully implemented
**File(s):** `crates/hook-sdk/tests/ui/valid_resolver.rs:8-10`
**Description:** Trybuild fixture comment:
```
// RED GATE: FAILS because the macro body is `todo!()` — the macro panics
// during compilation instead of generating the `resolve()` export.
// Will pass GREEN after the Step 4 implementer completes the macro.
```
Step 4 complete; macro implemented. Sibling-blast-radius confirms S-7.01 systematic pattern (3 files: Cargo.toml, resolver_types_test.rs, valid_resolver.rs).
**Suggested resolution:** Replace RED GATE block with "// GREEN: macro implemented. This trybuild case asserts that #[resolver] applied to the canonical signature compiles cleanly and emits a #[cfg(target_arch=\"wasm32\")] resolve() body. (Host build elides body; wasm32 export verified by tests/wasm32_resolver_export_integration.rs.)"

### F-S12.05-P4-004 — NITPICK — Duplicate fn main() emission paths in resolver_macro.rs
**File(s):** `crates/hook-sdk-macros/src/resolver_macro.rs:43, 96`
**Description:** Both error-path expansion (line 43) and success-path expansion (line 96) emit `fn main() {}`. Intentional (trybuild bin crates need main). But success-path emits regardless of whether user wrote one. If user adds `#[resolver]` to resolve_impl in binary crate that ALSO defines fn main, macro generates second fn main causing E0428.
**Suggested resolution:** (a) document in #[resolver] rustdoc that macro emits fn main() and user's crate must not, OR (b) detect sibling fn main via syn parsing and skip emission. (a) cheaper.

### F-S12.05-P4-005 — NITPICK — `valid_resolver.rs:25` uses `serde_json::json!` without explicit dep declaration in fixture context
**File(s):** `crates/hook-sdk/tests/ui/valid_resolver.rs:25`
**Description:** Trybuild fixture imports serde_json::json!. trybuild compiles fixture with parent crate's dep resolution; serde_json is regular dep of vsdd-hook-sdk so import resolves. Works today but fragile — if vsdd-hook-sdk ever moves serde_json to dev-dependencies only, fixture would silently break AC-005.
**Suggested resolution:** Optional. Add one-line comment noting fixture relies on hook-sdk's [dependencies] serde_json.

## Cross-cutting observations
- Test plan ↔ test file traceability solid. Every Test Plan row maps to test_BC_4_12_002_* function. async-fn rejection test (line 293) + trybuild fixture is undocumented coverage extension; could add (extension test) row but not required.
- Trybuild stderr line-pinning instrumentation appropriately defensive. wrong_sig.rs:11-19 has prominent !! warnings. wrong_sig.stderr:2 (--> tests/ui/wrong_sig.rs:20:1). #[resolver] currently on line 20.
- async_resolver.stderr line-pinning implicit but accurate. async fn resolve_impl on line 14 of async_resolver.rs, matches stderr:2. No defensive comment block — could add for consistency. NITPICK.
- resolver re-export at lib.rs:53-54 correctly relies on Rust's macro/type/value namespace separation.
- No "Resolver trait" references survive in any test or example crate beyond the F-P4-001 Cargo.toml.
- VP-075 proptest filter justification sound. arb_resolver_output excludes Some(Value::Null) with textual rationale anchored to BC-4.12.002 EC-001.

## Convergence assessment
- Within-story findings: 5 (1 MEDIUM, 1 LOW, 3 NITPICK)
- Severity floor: MEDIUM (F-P4-001)
- Classification: MEDIUM
- Reasoning: F-P4-001 partial-fix-regression discovered via S-7.01 review axis (sibling files in same architectural layer carrying stale post-implementation prose). F-P4-002 + F-P4-003 sibling defects of same pattern — combined form 3-file blast-radius cluster around "post-implementation prose cleanup" gap. Three findings mechanical sweeps; two NITPICK observation-only. Even one MEDIUM = MEDIUM per strict rule. Three-consecutive-clean count resets — pass 4 does NOT count toward BC-5.39.001.
- Recommendation: PROCEED_TO_FIX
