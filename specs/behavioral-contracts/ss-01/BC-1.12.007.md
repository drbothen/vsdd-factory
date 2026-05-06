---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-05-06T00:00:00Z
phase: 1b
inputs:
  - .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md
  - .factory/stories/epics/E-10-single-stream-otel-event-emission.md
input-hash: "[pending-recompute]"
traces_to: ADR-015-single-stream-otel-schema.md
origin: greenfield
subsystem: "SS-01"
capability: "CAP-003"
lifecycle_status: active
introduced: v1.1.0
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# Behavioral Contract BC-1.12.007: factory-dispatcher::deprecation_lifecycle::wave1_call_graph_invariant — Router, SinkRegistry, DlqWriter, and sink-otel-grpc NOT called from any production code path after Wave 1; deprecated crates excluded from default-members; TD-015-a CI check deferred to Wave 5

## Description

ADR-015 D-15.1 defines two lifecycle states for deprecated items with distinct
verbs: **Deprecated (Wave 1)** means uncalled from production and excluded from
`default-members` in the workspace `Cargo.toml`, but NOT physically deleted.
**Retired (Wave 5)** means physically deleted from the repository.

This BC governs the **Wave 1 deprecation behavioral invariant**: after Wave 1
ships (S-10.09), the call graph of the production dispatcher binary MUST NOT
include any call to `Router::submit`, `SinkRegistry` dispatch,
`DlqWriter::write`, or any function exported by `sink-otel-grpc`. The deprecated
code exists on disk and compiles, but is not reachable from `main.rs` through
any live code path.

**SCOPE BOUNDARY (D-311 architect routing decision):** This BC covers the
Wave 1 call-graph behavioral invariant. Wave 5 crate deletion, type removal,
`publish = false` mutation, and SS-03 spec rewrite are SEPARATE work (S-10.09
SS-03-owned cleanup). The physical retirement contract will be authored as a
separate BC in the SS-03 cluster. BC-1.12.007 is a dispatcher hot-path
concern; it does NOT prescribe Wave 5 physical actions.

This is a future-implementation contract for S-10.09 (Wave 5 — but the
behavioral guarantee starts at Wave 1, the call-graph invariant that S-10.02
establishes). All Canonical Test Vectors describe the post-Wave-1 state.

## Preconditions

1. Wave 1 has shipped: `host::emit_event` in `main.rs` calls `FileSink::write`
   directly (per BC-1.12.001). The old multi-sink integration path is removed
   from the production hot path.
2. The following items are present on disk (pre-Wave-5 state) but are excluded
   from `Cargo.toml` `default-members`:
   - `crates/sink-otel-grpc/` crate
   - `Router` and `SinkRegistry` types within `crates/sink-core/`
   - `DlqWriter` type within `crates/sink-core/`
3. The workspace builds cleanly with `cargo build` (no compilation errors from
   the deprecated code being present on disk).

## Postconditions

1. **Call-graph invariant (post-Wave-1):** `Router::submit`, `SinkRegistry`
   dispatch methods, `DlqWriter::write`, and any public API of `sink-otel-grpc`
   are NOT called from any code path reachable from `main.rs` in the production
   dispatcher binary. This includes direct calls AND transitive calls through
   any helper, wrapper, or trait method.
2. The deprecated items (`sink-otel-grpc`, `Router`, `SinkRegistry`,
   `DlqWriter`) are excluded from `Cargo.toml` `default-members`. The
   workspace's default build (`cargo build`) does NOT compile these crates/types
   unless explicitly opted in (e.g., `cargo build -p sink-otel-grpc`).
3. The deprecated crates are marked `publish = false` in their respective
   `Cargo.toml` files. This prevents accidental publication to crates.io.
4. **Physical files remain on disk until Wave 5.** The Wave 1 deprecation does
   NOT delete any file. Git history is preserved. The two-phase lifecycle
   (deprecated = uncalled but present; retired = deleted) is intentional per
   ADR-015 to preserve `git bisect` / rollback options.
5. Events continue to route exclusively to `FileSink` writing to
   `events-*.jsonl` (per BC-1.12.001). No event reaches the deprecated code
   paths.

## Invariants

1. **The call-graph invariant is enforced by the Wave 1 implementation
   change**, NOT by a CI check. At Wave 1, `main.rs` is the sole wiring point;
   removing the `Router::submit` call from `main.rs` enforces the invariant
   mechanically. The invariant is testable via static analysis or integration
   test (assert no events in `sink-otel-grpc`'s output path).
2. **TD-015-a deferral (architect's D-311 note):** A CI check using
   `cargo metadata` to reject any PR that adds NEW intra-workspace dependencies
   on the deprecated crates is DEFERRED per ADR-015 D-15.1. This check is NOT
   a postcondition of Wave 1. It MUST be implemented before Wave 5 crate
   deletion to prevent a scenario where an undetected re-coupling causes workspace
   breakage when the deprecated crates are deleted. TD-015-a is tracked as an
   open technical debt item.
3. **`publish = false` is not sufficient re-coupling protection.** `publish =
   false` prevents crates.io publication but does NOT prevent other workspace
   crates from adding `sink-otel-grpc` as a dev-dependency or dependency in
   their own `Cargo.toml`. This gap is the reason TD-015-a exists. Between
   Wave 1 and Wave 5, the workspace MUST be manually audited (or TD-015-a
   implemented) to ensure no re-coupling occurs.
4. The deprecated types remain compilable but unused. The Rust compiler's
   unused-code warnings for these types MAY be suppressed with
   `#[allow(dead_code)]` in the interim period. This is acceptable; the code
   is intentionally kept for rollback safety, not because it is live.

## Related BCs

- BC-1.12.001 — Single-stream FileSink routing (this BC is the negative-space
  complement: BC-1.12.001 says what IS called; this BC says what is NOT called)
- BC-3.05.004 — `observability-config.toml` v2 schema (sibling: the config
  schema also removes multi-sink stanzas; SS-03-owned)

## Architecture Anchors

- `crates/factory-dispatcher/src/sinks/mod.rs` lines 11–15 — the open
  integration TODO that ADR-015 closes; `Router::submit` is NOT wired here;
  after Wave 1, this TODO is resolved by removing the comment and the dead code
- `Cargo.toml` (workspace root) — `default-members` field; deprecated crates
  excluded from default build
- `crates/sink-otel-grpc/Cargo.toml` — `publish = false` set at Wave 1
- ADR-015 D-15.1 — "Deprecated (Wave 1): Crates are excluded from
  `default-members` in the root `Cargo.toml` and marked `publish = false`.
  They are NOT called from any production code path. They remain on disk."
- ADR-015 D-15.1 — "Retired (Wave 5): Crates are physically deleted from the
  repository."
- ADR-015 D-15.1 — TD-015-a: `publish = false` on retired crates does not
  prevent intra-workspace re-coupling; CI check deferred

## Story Anchor

S-10.09 (Wave 5: Crate retirement + SS-03 spec rewrite) — the Wave 5 PHYSICAL
deletion is this story's deliverable. The BEHAVIORAL invariant (call-graph
exclusion) is established by S-10.02 (Wave 1 FileSink wiring), which is the
story that removes `Router::submit` from the production path. Both stories
reference this BC: S-10.02 establishes the invariant; S-10.09 completes it
via physical deletion.

## VP Anchors

(TBD — to be assigned after S-10.09 story authoring)

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | A developer adds `sink-otel-grpc` as a dev-dependency in a new crate between Wave 1 and Wave 5 | Per TD-015-a, this re-coupling is NOT currently detected by CI (the check is deferred). Manual workspace audit is the only current guard. This is a known risk window. |
| EC-002 | `cargo build` executed on the workspace post-Wave-1 | Builds without errors; `sink-otel-grpc` crate NOT compiled in default build (excluded from `default-members`); `Router`/`SinkRegistry`/`DlqWriter` code compiles within `sink-core` but is unused in the binary |
| EC-003 | Integration test that calls `emit_event` post-Wave-1 | Event appears in `events-*.jsonl`; zero bytes written to any `sink-otel-grpc` output target; `Router::submit` call count = 0 (verifiable via test mock or static analysis) |
| EC-004 | Wave 5 deletion: `sink-otel-grpc/` directory deleted from workspace | Workspace builds cleanly because no active crate has `sink-otel-grpc` as a dependency (TD-015-a gap must be closed before this to be safe) |
| EC-005 | Rollback between Wave 1 and Wave 5: code needs to be reverted | `git bisect` or `git revert` can restore the pre-Wave-1 state because the deprecated code remains on disk. This is the explicit reason the two-phase lifecycle exists per ADR-015. |
| EC-006 | **TD-015-a not implemented before Wave 5:** workspace crate has re-coupled to `sink-otel-grpc` | Wave 5 deletion breaks workspace compilation. This is the failure mode TD-015-a prevents. **This is a known risk per ADR-015 D-15.1.** BC-1.12.007 documents this risk explicitly as a Wave 5 prerequisite. |

## Canonical Test Vectors

| Input | Expected Output | Category |
|-------|----------------|----------|
| `cargo build` on workspace post-Wave-1 | Build succeeds; no `sink-otel-grpc` compilation output in default build | default-members-exclusion |
| Emit 10 events via `host::emit_event` post-Wave-1 | 10 events in `events-*.jsonl`; 0 events routed through `Router` or `sink-otel-grpc` | call-graph-exclusion |
| **Misimplementation distinguisher:** Wave 1 code still calls `Router::submit` | Test MUST assert no events appear in `sink-otel-grpc`'s output path AND no `Router::submit` call is made. A Wave 1 misimplementation that still calls `Router` routes events incorrectly. | misimplementation-witness-router-still-called |
| `cargo publish --dry-run -p sink-otel-grpc` | Fails with "publish = false in Cargo.toml" error | publish-false-guard |
| Static analysis (e.g., `cargo udeps` or custom lint) post-Wave-1 | No active workspace crate has `sink-otel-grpc`, `Router`, `SinkRegistry`, or `DlqWriter` as a live dependency | static-analysis-no-recoupling |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — Phase 1.6b) | No production code path calls `Router::submit` post-Wave-1 | integration test: emit events; assert zero calls to Router (via mock injection or call-graph analysis) |
| (TBD) | `sink-otel-grpc` excluded from default workspace build | build test: `cargo build` succeeds without compiling `sink-otel-grpc` |
| (TBD) | TD-015-a: no intra-workspace re-coupling between Wave 1 and Wave 5 | manual audit gate (deferred to pre-Wave-5 checklist per ADR-015 D-15.1 TD-015-a) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-003 ("Stream observability events to multiple configurable sinks") per capabilities.md §CAP-003 |
| Capability Anchor Justification | CAP-003 ("Stream observability events to multiple configurable sinks") per capabilities.md §CAP-003. This BC governs the routing invariant that ensures no event is silently lost to an uncalled deprecated sink path — preserving the stream's completeness. ADR-015 D-15.1 simplifies CAP-003's multi-sink model to single-stream FileSink; this BC is the negative-space enforcement guaranteeing the old multi-sink paths are not called from the production stream. |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/main.rs` (call-graph exclusion enforced by removing `Router::submit` wire); workspace `Cargo.toml` (`default-members` exclusion) |
| Stories | S-10.02 (Wave 1: establishes call-graph invariant by removing Router wire), S-10.09 (Wave 5: completes lifecycle via physical deletion) |
| Epic | E-10 (Single-stream OTel-aligned event emission) |
| ADR | ADR-015 D-15.1 (Deprecation and retirement semantics; two-phase lifecycle: deprecated = uncalled on disk; retired = deleted); ADR-015 D-15.1 TD-015-a (cargo-metadata CI check deferred) |
| Technical Debt | TD-015-a — CI check to reject new intra-workspace dependencies on deprecated crates; deferred; must be resolved before Wave 5 deletion |

### Purity Classification

| Property | Assessment |
|----------|-----------|
| I/O operations | NO — this BC is a BEHAVIORAL CONSTRAINT on the dispatcher call graph, not an active I/O operation. It is verified by absence: no calls to deprecated APIs occur. |
| Global state access | N/A |
| Deterministic | YES — the call graph is static after compilation |
| Thread safety | N/A |
| Overall classification | Behavioral invariant (call-graph constraint; verified by integration test and static analysis) |

### TD-VSDD-092 (BC-SOUL4-coverage) Verification

This BC's SOUL #4 consideration is unique: it governs ABSENCE of calls, not
active code paths.

- The Wave 1 implementation change at `main.rs` (removing `Router::submit` call)
  is the enforcement action. The `let _ =` concern does not apply to this BC
  directly, but the REMOVAL of the router call must be complete — no dormant
  `let _ = router.submit(event)` pattern may survive in the code (that would be
  a `let _ =` discard of a now-dead call, not a safety issue but a correctness
  signal that cleanup is incomplete).
- TD-015-a deferral: the CI guard against re-coupling is the long-term SOUL #4
  guard for this BC's invariant. Without TD-015-a, a future developer could
  silently re-couple to a deprecated crate with no build-time warning.
