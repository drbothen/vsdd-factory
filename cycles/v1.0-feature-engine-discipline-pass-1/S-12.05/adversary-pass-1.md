# Adversarial Review — S-12.05 Pass 1

## Metadata
- **Story:** S-12.05 hook-sdk resolver-authoring extensions
- **Branch SHA reviewed:** 78c44f7a
- **Pass:** 1
- **Reviewer:** adversary (fresh context)
- **Classification:** HIGH
- **Within-story finding count:** 7
- **Recommendation:** PROCEED_TO_FIX

## Findings

### F-S12.05-P1-001 — HIGH — AC-004 trybuild test (`type_mismatch.rs`) is referenced everywhere but never created or invoked
**File(s):**
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.05/crates/hook-sdk/tests/resolver_types_test.rs:13` (coverage map)
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.05/crates/hook-sdk/tests/resolver_types_test.rs:194` ("via trybuild")
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-12.05-hook-sdk-resolver-extensions.md:295` (Test Plan)
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.05/crates/hook-sdk/tests/ui/` (file does not exist)

**Anchor:** AC-004, BC-4.12.002 INV1
**Description:** The story Test Plan, the coverage map in `resolver_types_test.rs`, and AC-004's prose all promise a trybuild compile-fail test at `tests/ui/type_mismatch.rs` named `test_BC_4_12_002_type_mismatch_compile_error`. Neither the `.rs` file, the matching `.stderr`, nor a `#[test]` function with that name exists. AC-004 is the only test for **the entire architecture-compliance rule INV1** (distinct types — no aliasing) that goes beyond a `mem::size_of` heuristic. The structural test at line 198-235 is a weak substitute — `mem::size_of` could coincidentally match for two unrelated types, so the assertion would silently produce a false-positive equality if implementations drift. The negative compile test was supposed to be the authoritative falsifiable witness.

**Why it matters:** Without this trybuild case, AC-004's "the Rust compiler rejects it with a type mismatch" guarantee is unproven. A future refactor that introduces a `From<HookPayload> for ResolverInput` (perhaps to share fields) would satisfy `mem::size_of` inequality but violate INV1's "no implicit conversions". The promised compile-error gate is missing.

**Suggested resolution:** Add `tests/ui/type_mismatch.rs` that does `let _: ResolverInput = some_hook_payload;` (or a function-call mismatch), the matching `.stderr`, and a `#[test] fn test_BC_4_12_002_type_mismatch_compile_error` that calls `t.compile_fail("tests/ui/type_mismatch.rs")`.

### F-S12.05-P1-002 — HIGH — AC-007 trybuild test (`no_feature_gate.rs`) is never invoked, and its `.stderr` is malformed
**File(s):**
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.05/crates/hook-sdk/tests/ui/no_feature_gate.rs` (orphan — present)
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.05/crates/hook-sdk/tests/ui/no_feature_gate.stderr:1-5` (malformed)
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.05/crates/hook-sdk/tests/resolver_types_test.rs:289-336` (no `compile_fail` call for this file)

**Anchor:** AC-007, BC-4.12.002 PC8
**Description:** The file `tests/ui/no_feature_gate.rs` exists but is never registered with `trybuild::TestCases::compile_fail(...)` anywhere — `Grep` for `no_feature_gate` returns zero hits in the test file. It is dead code. Worse, its `.stderr` is internally inconsistent and clearly hand-edited:

```
error[E0603]: module `resolver` is private
  --> tests/ui/no_feature_gate.rs:22:35
   |
$DIR/no_feature_gate.rs:22:35 | error: module `resolver` not found
```

Line 4 (`$DIR/...`) is not valid trybuild stderr syntax and the message contradicts line 1 ("private" vs "not found"). If this file were ever wired up, the trybuild assertion would fail. The actual AC-007 test (`test_BC_4_12_002_resolver_authoring_feature_gates_types`) instead does:
1. A tautological `cfg!(feature = "resolver-authoring")` assertion inside a `#[cfg(feature = "resolver-authoring")]` module (always true; the inline `#[allow]` even acknowledges this).
2. A `lib.rs` source-file substring grep — checking that the source contains the literal string `cfg(feature = "resolver-authoring")`. This is a **doc-comment-or-source check**, not a behavior check; if someone refactors to a different attribute syntax it false-fails, and if someone moves the gate but leaves the literal in a comment it false-passes.

**Why it matters:** AC-007 is the only test that proves the feature flag actually gates types. Today it does not; the "without the feature, this fails to compile" assertion is structurally absent. Combined with a malformed orphan file, this is a textbook false-green: cargo test is green but the contract is unverified.

**Suggested resolution:** Either (a) wire up `no_feature_gate.rs` via a separate trybuild crate that explicitly does NOT enable `resolver-authoring` (the trybuild docs cover this — use a sub-crate with its own Cargo.toml, since the parent's features bleed into the test crate); fix its `.stderr` to match real cargo error output; OR (b) delete the orphan file + fix the coverage-map docstring claiming this was done.

### F-S12.05-P1-003 — HIGH — `#[resolver]` macro emits packed-i64 code that is wrong on 64-bit hosts (no `#[cfg(target_arch = "wasm32")]` guard)
**File(s):** `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.05/crates/hook-sdk-macros/src/resolver_macro.rs:62-85`

**Anchor:** BC-4.12.002 PC1, EC-005 (story line 201), Architecture Compliance Rule 5 (story line 219)
**Description:** The generated body does:
```
let output_ptr = output_bytes.leak().as_ptr() as i64;
(output_ptr << 32) | (output_len as i64)
```

On wasm32 (the intended target), `as_ptr()` returns a 32-bit pointer; `as i64` zero-extends; `<< 32` cleanly packs into the upper word. Correct.

On a 64-bit host (which is what `trybuild` uses to compile `valid_resolver.rs` — trybuild compiles for the *host* target), `as_ptr()` returns a 64-bit pointer. The cast `as i64` reinterprets bits (could be negative on high-half-set addresses). The shift `output_ptr << 32` then **discards the upper 32 bits of the actual pointer** and shifts the lower 32 bits into the high word. The packing is silently wrong, and the upper 32 bits of any allocation address > 4 GiB are lost.

There is no `#[cfg(target_arch = "wasm32")]` guard around the generated `extern "C" fn resolve` — it is unconditional. The hook macro compares: `crates/hook-sdk-macros/src/lib.rs` similarly emits `fn main()` without an arch guard, but `fn main()` is target-agnostic — the resolver export packs pointers and is not.

**Why it matters:** The `valid_resolver.rs` trybuild case will compile on the host, but the generated code's semantic correctness is target-dependent. If anyone ever runs a host-side unit test that calls into the generated `resolve()`, results are silently corrupt above the 4 GiB boundary. More immediately: the BC contract specifies a wasm32 ABI, and the macro should statically reject (or `#[cfg]`-out) non-wasm targets so the contract is enforced where it matters. PC1 is "encoded as `((ptr as i64) << 32) | (len as i64)`" — but `ptr` per the spec is i32, not the *host's* `*const u8`. The macro is conflating widths.

**Suggested resolution:** Either (a) gate the generated `extern "C" fn resolve` with `#[cfg(target_arch = "wasm32")]` so it only emits on the intended target; or (b) explicitly cast through `i32` first: `let output_ptr_i32 = output_bytes.leak().as_ptr() as i32;` then `((output_ptr_i32 as i64) << 32) | ...` — this matches the BC spec exactly (both halves are i32), and on a 64-bit host produces a clean compile error or warning if ptr exceeds i32, surfacing the inappropriateness early.

### F-S12.05-P1-004 — MEDIUM — AC-008 implementation contradicts its own spec rationale by hard-asserting `host_version == 1`
**File(s):**
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.05/crates/hook-sdk/tests/resolver_types_test.rs:349-373`
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-12.05-hook-sdk-resolver-extensions.md:122-128`

**Anchor:** AC-008, BC-4.12.002 INV2
**Description:** The story spec for AC-008 explicitly says:
> "**Falsifiable test:** Rust unit test: `assert_ne!(vsdd_hook_sdk::HOST_ABI_VERSION, vsdd_hook_sdk::RESOLVER_ABI_VERSION as u32)` — no, this would be wrong if both happen to equal 1. Instead: assert both constants EXIST and are independently defined."
> "Actually: **Falsifiable test:** Code review check — must be defined in separate source locations… A grep confirms their distinct definition lines."

The spec deliberately rejects value-equality assertions and pivots to a structural/source-location check. The implementation at line 360 does `assert_eq!(host_version, 1u32, "HOST_ABI_VERSION must be 1 (AC-008)")`. This pins HOST_ABI_VERSION to 1 in a test owned by S-12.05 — but HOST_ABI_VERSION belongs to a *separate* versioning track (BC-4.12.002 INV2). When SS-04 bumps `HOST_ABI_VERSION` to 2 in a future cycle for unrelated reasons, S-12.05's test breaks even though the resolver ABI is unchanged. This is the exact coupling INV2 forbids.

**Why it matters:** The test couples the two versions in CI, contradicting the invariant it claims to test. AC-008 is the *only* test for INV2 independence; its implementation re-introduces the coupling.

**Suggested resolution:** Replace `assert_eq!(host_version, 1u32, …)` with a structural check that doesn't pin HOST_ABI_VERSION's value. Per the spec rationale: assert both constants are accessible (done — lines 351-353), then add a source-location check that the two constants are declared in different files (e.g., file-scan `resolver.rs` for `RESOLVER_ABI_VERSION` and `lib.rs` for `HOST_ABI_VERSION`).

### F-S12.05-P1-005 — MEDIUM — `ResolverError` is dead-weight scaffolding without a producer or test
**File(s):**
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.05/crates/hook-sdk/src/resolver.rs:40-47`
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.05/crates/hook-sdk/HOST_ABI.md:1104` (where `ResolverError` is referenced as a return type)

**Anchor:** Story File List (line 163), BC-4.12.004 (composes-with from BC-4.12.002 line 173)
**Description:** `ResolverError { message: String }` is declared `pub`, derives serde + PartialEq, but has zero call sites in the worktree (`Grep ResolverError` returns only the declaration, the Cargo.toml comment, and a stale HOST_ABI.md reference). It is feature-gated like the rest, exported from `lib.rs` via `pub use resolver::*`, and has no constructor, no documentation about who emits it, no `From` impl, no test. The doc-comment even says "Reserved for future resolver SDK error handling; currently a placeholder to satisfy story file-list requirements."

The `Resolver` trait at lines 53-59 returns `ResolverOutput` directly — not `Result<ResolverOutput, ResolverError>`. So `ResolverError` is unreachable through the documented authoring surface. Meanwhile HOST_ABI.md line 1104 documents `invoke_resolver` as returning `Result<ResolverOutput, ResolverError>` — but `invoke_resolver` is host-side (S-12.04), not author-side. The author-facing `ResolverError` is unconnected to that.

**Why it matters:** Public API surface added without semantics. Either (a) it is needed and the author trait should return `Result<ResolverOutput, ResolverError>` (which would propagate to BC-4.12.002 PC5's contract — currently spec'd as bare `ResolverOutput`, see line 89), or (b) it's not needed for author-side and should be deleted to avoid SemVer-stable scaffolding. Public types in a published crate are a SemVer commitment; "placeholder to satisfy the story" is a poor reason to expose one.

**Suggested resolution:** Either delete `ResolverError` from the author-side surface (and remove from `Cargo.toml` comment) — the host-side equivalent will live in factory-dispatcher, not hook-sdk — OR write a one-line spec in BC-4.12.002 / story explaining the author-side semantics and add at least one test exercising serde round-trip. Defer-to-S-12.07 is acceptable but the placeholder shouldn't ship publicly with no doc until then.

### F-S12.05-P1-006 — MEDIUM — Cargo.toml version not bumped; no CHANGELOG entry; SemVer-minor surface added silently
**File(s):**
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.05/crates/hook-sdk/Cargo.toml:3` (`version = "0.2.0"`)
- `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.05/crates/hook-sdk/` (no CHANGELOG.md)
- `/Users/jmagady/Dev/vsdd-factory/.factory/stories/S-12.05-hook-sdk-resolver-extensions.md:124` ("documents this independence in the `crates/hook-sdk/CHANGELOG.md` entry")
- Original review prompt: "Per BC-2.06.001, this constitutes a minor version bump to hook-sdk. Verify Cargo.toml version was bumped."

**Anchor:** AC-008 falsifiable text + project SemVer policy (BC-2.06.001 referenced in dispatch context)
**Description:** This story adds:
- A new public `resolver-authoring` feature flag.
- New `pub` items behind that flag: `ResolverInput`, `ResolverOutput`, `ResolverError`, `RESOLVER_ABI_VERSION`, `Resolver` trait.
- A new public proc-macro: `#[resolver]`.

These are SemVer-minor additions to a published crate (`publish = true` per Cargo.toml line 15). The `version = "0.2.0"` was not bumped, and `crates/hook-sdk/CHANGELOG.md` does not exist (only the project-root `CHANGELOG.md` does, and it has no S-12.05 entry). Story AC-008 explicitly mentions the CHANGELOG.md entry as the documentation site for ABI-version independence.

**Why it matters:** A future operator running `cargo publish` will publish a 0.2.0 with new public API but the same version number as a prior 0.2.0 — registry rejects, or worse, causes downstream dependents to silently get the new API without knowing. The matching `vsdd-hook-sdk-macros` at `version = "0.1.0"` likewise gains the public `#[resolver]` macro export and is unbumped.

**Suggested resolution:** Bump `crates/hook-sdk/Cargo.toml` to `0.3.0` (minor — adds public API), bump `crates/hook-sdk-macros/Cargo.toml` to `0.2.0`, and update the path-version pin at `hook-sdk/Cargo.toml:29` (`version = "0.1.0"` → `"0.2.0"`). Create `crates/hook-sdk/CHANGELOG.md` with a "0.3.0 — resolver-authoring feature" entry and explicitly state the independence of `RESOLVER_ABI_VERSION` from `HOST_ABI_VERSION` per AC-008's documentation requirement.

### F-S12.05-P1-007 — LOW — AC-007 implementation contains a tautological assertion (POLICY 11 borderline)
**File(s):** `/Users/jmagady/Dev/vsdd-factory/.worktrees/S-12.05/crates/hook-sdk/tests/resolver_types_test.rs:299-306`

**Anchor:** Project policy 11 (no_test_tautologies)
**Description:** Inside a `#[cfg(feature = "resolver-authoring")]` module, the test asserts `cfg!(feature = "resolver-authoring")` — which is constant-true within that module. The test acknowledges this with `#[allow(clippy::assertions_on_constants)]` and a justification comment ("intentional self-documentation, not a logic guard"). The function does also call a production fn (`vsdd_hook_sdk::resolver::RESOLVER_ABI_VERSION`) at line 310 and does the lib.rs source-grep at lines 326-335, so it's not a pure tautology. But the cfg! line is dead weight.

POLICY 11's recognized opt-out is `/// data-shape pin` or file-level `//! tautology-allowed: <reason>` — neither marker is present. The inline justification is not the recognized form.

**Why it matters:** Borderline policy violation; flagging for adjudication. The assertion is harmless but neither informative nor compliant with the documented opt-out grammar.

**Suggested resolution:** Either delete the cfg! assertion entirely (the production-fn calls below are sufficient) or add the recognized opt-out marker (`/// data-shape pin` above the assertion).

## Cross-cutting observations

- **[process-gap]** The story's Test Plan table at lines 295-301 is misaligned with the implementation: it lists `tests/ui/type_mismatch.rs` (missing) and `tests/ui/no_feature_gate.rs` (orphan, not invoked). A test-writer/red-gate validator that checked test-plan coverage against actual `#[test]` functions and trybuild file presence would have caught both F-001 and F-002 before adversary review. Consider a hook that diffs the story Test Plan rows against `cargo test --list` + `tests/ui/*.rs` filenames + `compile_fail`/`pass` invocations.
- **POLICY 10 (demo_evidence_story_scoped):** No `docs/demo-evidence/S-12.05/` directory yet, no flat `docs/demo-evidence/*.md` blockers — this is OK; demo recording is post-convergence per the Definition of Done.
- **POLICY 12 (bc_tv_emitter_consistency):** BC-4.12.002 Canonical Test Vectors (lines 137-145) show all five `ResolverInput` fields and both `ResolverOutput` fields populated; no field is marked excluded, so no exclusion mismatch exists. The `#[serde(...)]` attributes on the struct are absent → all fields serialize unconditionally → consistent with TV.
- The AC-010 `Some(Null)` resolution (filter out `Some(Null)` from the proptest strategy) is **technically defensible** — `Some(Null)` is degenerate under standard serde Option semantics — but the resolution lives only in a comment in the test file (lines 467-475). It is not codified into BC-4.12.002 EC-001 or the story body as a normative resolution. A future implementer touching the proptest strategy may innocently delete the `prop_filter` and reintroduce a flake. **Suggested follow-up (LOW, can be deferred to wave-gate):** add a normative line to BC-4.12.002 EC-001 or AC-010: "Note: `Some(Value::Null)` and `None` are semantically equivalent at the merge layer (BC-4.12.005); the resolver SHOULD return `None` for absent context. Round-trip determinism applies to semantically-distinct values."
- The trait `Resolver` at `resolver.rs:53-59` is declared but unused by the `#[resolver]` macro — the macro operates on a free `fn resolve_impl`, not on a trait impl. The trait is documentation-only for now. Worth noting; not blocking.

## Convergence assessment
- Within-story findings: 7 (3 HIGH, 3 MEDIUM, 1 LOW)
- Severity floor: HIGH
- Classification: **HIGH**
- Reasoning: F-001 (missing AC-004 trybuild) and F-002 (orphaned, malformed AC-007 trybuild) together mean two of the ten ACs lack their promised falsifiable witness — the test plan's compile-error gates do not exist or do not run. F-003 is a target-arch correctness bug in the macro-emitted code; it works on the intended target but is unguarded and silently miscompiles on the host where trybuild runs. These three findings each have specific file+line evidence, are within-story, and individually block convergence. F-004 contradicts INV2 in the AC-008 test itself. F-005 and F-006 are SemVer/public-API hygiene issues that should be resolved before any release containing this code. Recommend PROCEED_TO_FIX.
