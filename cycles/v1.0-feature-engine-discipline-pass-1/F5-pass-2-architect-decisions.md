---
document_type: architect-decision-log
cycle: v1.0-feature-engine-discipline-pass-1
phase: F5-pass-2
authored_by: architect
date: 2026-05-10
stories_covered: [S-12.03, S-12.05]
findings_covered: [F-P2-005, F-P2-002]
---

# F5 Pass-2 Architect Decisions

Two adversarial pass-2 findings require architectural disposition before fix bursts proceed.

---

## Decision 1: BC-4.12.005 INV1 vs `merge_resolver_outputs` signature (F-P2-005)

**Finding:** BC-4.12.005 INV1 specifies `merge(static_config: Value, resolver_outputs: Vec<ResolverOutput>) -> Value`. The
pass-1 implementation is `merge_resolver_outputs(static_config: Map<String, Value>, resolver_outputs: &[ResolverOutput], on_collision: impl Fn(&str, &Value, &Value)) -> Map<String, Value>`. The `on_collision` callback is invoked with file I/O (writes to `InternalLog`) in production `executor.rs`.

**Choice: Path B — restructure impl, preserve BC v1.1.**

**Reasoning:**

The `on_collision` callback exists to emit `resolver.merge_collision` telemetry. Its presence makes `merge_resolver_outputs` technically impure — the function's observable behavior depends on a side-effecting closure injected by the caller. This matters for VP-075, which requires proptest to demonstrate determinism. A proptest over a function accepting `impl Fn(...)` can pass an identity closure in tests while production passes a file-writing closure; the purity guarantee tested in proptest would not be the same property exercised in production. That is a verification gap.

Path A ("pure modulo callback") is not a coherent invariant: a function is either pure or it is not. Telemetry callbacks are a legitimate pattern, but they belong at the call site (executor), not inside the merge primitive. The `Map<String, Value>` type narrowing (vs the BC's `Value`) is unambiguously an improvement and does not require a BC amendment — the implementation is strictly more precise than the contract.

Path B cleanly solves both issues:
- `merge_resolver_outputs` becomes a genuine pure function: same inputs → same output, no callbacks.
- Collision detection returns data; the caller emits events. This is the existing pattern for `emit_not_found` and `emit_resolver_error` already at the same call site.
- VP-075 proptest is now verifying the exact same function exercised in production.
- One current call site (executor.rs:~541); S-12.06/S-12.07/S-12.08 will add more — each caller handles its own telemetry, which is correct because only the caller knows the trace context.

**Follow-up actions:**

Agent: **implementer** (S-12.03 fix burst)

1. Change `merge_resolver_outputs` signature to:
   ```rust
   pub fn merge_resolver_outputs(
       static_config: serde_json::Map<String, Value>,
       resolver_outputs: &[ResolverOutput],
   ) -> (serde_json::Map<String, Value>, Vec<CollisionInfo>)
   ```
   where `CollisionInfo` is:
   ```rust
   pub struct CollisionInfo {
       pub key: String,
       pub static_value: Value,
       pub resolver_value: Value,
   }
   ```

2. In `executor.rs`, after calling `merge_resolver_outputs`, iterate `Vec<CollisionInfo>` and emit `resolver.merge_collision` for each entry (same structure as the existing `on_collision` closure, now inlined at the call site).

3. BC-4.12.005 INV1 — NO change required. The `Map<String, Value>` type is a refinement of `Value`; the invariant's semantic content (pure function, same inputs → same output) is now accurately satisfied. Remove the doc-comment hedge "the caller decides whether to emit" from `resolver.rs:295`.

4. VP-075 proptest target: `merge_resolver_outputs` (no closure argument). Update proptest harness to call the no-callback signature.

Agent: **product-owner** — no BC edit needed.

---

## Decision 2: `Resolver` trait keep vs delete (F-P2-002)

**Finding:** S-12.05 implementation introduced a public `Resolver` trait in `hook-sdk/src/resolver.rs` re-exported via `pub use resolver::*`. The trait is unused by tests, unused by the `#[resolver]` macro, and S-12.05's CHANGELOG attributes it to BC-4.12.001 — which belongs to S-12.04 territory. The story frontmatter anchors only BC-4.12.002.

**Choice: Path B — delete the trait from S-12.05.**

**Reasoning:**

The `Resolver` trait introduces an SDK API surface commitment with no concrete implementor to validate against. Object-safety, `Send + Sync`, and `&self` vs `&mut self` choices all affect downstream users (S-12.07's `WaveContextResolver`). Making those choices now — before S-12.07 exists — is speculative design. If the trait shape is wrong, S-12.07 cannot implement it without either breaking the SDK or requesting a patch that re-opens S-12.05.

The `#[resolver]` macro's contract is `fn resolve_impl(input: ResolverInput) -> ResolverOutput`. That is the authoring primitive. A trait is a composition mechanism. S-12.07 will determine whether a trait is the right abstraction when it needs to share code between the `WaveContextResolver` and any future resolver. That is the correct point for the decision — with empirical constraints.

The CHANGELOG attribution to BC-4.12.001 confirms scope creep: BC-4.12.001 is S-12.04's contract (WASM lifecycle). Introducing BC-4.12.001 artifacts in S-12.05 breaks the one-story-one-BC-anchor discipline. Deleting the trait keeps S-12.05 cleanly anchored to BC-4.12.002.

**Follow-up actions:**

Agent: **implementer** (S-12.05 fix burst)

1. Delete `pub trait Resolver { ... }` (lines 44-50) from `crates/hook-sdk/src/resolver.rs`.
2. Ensure `pub use resolver::*` re-export does not inadvertently re-export other unintended items; audit and replace with explicit re-exports if needed.
3. Remove the CHANGELOG entry in `crates/hook-sdk/CHANGELOG.md` that attributes `Resolver` trait to BC-4.12.001.
4. Confirm `cargo test -p hook-sdk --features resolver-authoring` still passes after deletion (the trait has no tests, so deletion should be clean).

Agent: **story-writer** — no S-12.05 frontmatter edit needed. BC-4.12.002 is and remains the sole anchor. Do not add BC-4.12.001.

Agent: **product-owner** — no BC edit needed.

---

## Summary Table

| Decision | Finding | Choice | Owner Agent | Files Affected |
|----------|---------|--------|-------------|----------------|
| 1: INV1 callback removal | F-P2-005 | Path B: return `Vec<CollisionInfo>`, caller emits | implementer | `resolver.rs:301-322`, `executor.rs:529-541`, VP-075 proptest harness |
| 2: `Resolver` trait deletion | F-P2-002 | Path B: delete trait, stay anchored to BC-4.12.002 | implementer | `hook-sdk/src/resolver.rs:44-50`, `hook-sdk/CHANGELOG.md` |

Neither decision requires a BC version bump or story frontmatter amendment. Both are implementer fix-burst actions only.
