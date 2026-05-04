---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: architect
timestamp: 2026-05-04T00:00:00Z
phase: 1.2-rev
inputs:
  - .factory/specs/architecture/decisions/ADR-015-single-stream-otel-schema.md
input-hash: "[pending-recompute]"
traces_to: ADR-015-single-stream-otel-schema.md
origin: spec-revision
subsystem: "SS-01"
capability: "CAP-TBD"
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

# Behavioral Contract BC-1.11.003: factory-dispatcher::host::emit_pair — atomic dual-emit host helper for Wave 2 migration window

## Description

During the Wave 2 migration window, plugins must emit both the old event name
AND the new reverse-DNS event name in the same logical operation (dual-emit).
In v1, this is done with two sequential `host::emit_event` calls. A
`FileSink` failure or dispatcher SIGKILL between those two calls produces an
orphaned half-pair: one emission present, the other absent.

This BC specifies the `vsdd_hook_sdk::host::emit_pair(old_event, new_event)`
host helper that wraps both emissions in a single host call, shifting the
orphan-prevention responsibility from plugin code to the host.

**Status:** This BC specifies the TARGET design for Wave 2+ full atomicity.
The v1 implementation ACCEPTS the orphan-half risk from sequential two-call
dual-emit (ADR-015 D-15.2.e). The `emit_pair` host helper is the Wave 2
deliverable that eliminates this risk. Plugins may migrate to `emit_pair`
one at a time; legacy two-call dual-emit continues to work via the orphan-
detection rule (consumers tolerate dangling cross-references) until all
plugins have migrated.

This BC resolves OQ-8 from ADR-015.

## Preconditions

1. Plugin is in the Wave 2 dual-emit transition window (emitting both old and
   new event names for a single logical event).
2. Plugin has the `emit_event` capability (no separate capability required for
   `emit_pair`; it is the same capability at a different granularity).
3. Plugin provides two well-formed event payloads: `old_event` (old-name
   emission) and `new_event` (new-name canonical emission).

## Postconditions

### Both writes succeed
1. Both `old_event` and `new_event` are written to `events-*.jsonl` as
   complete JSONL lines.
2. The host automatically assigns:
   - A shared `event.correlation_id` UUID to both events.
   - `old_event.event.deprecated_by` = `new_event.event.id`.
   - `new_event.event.replaces_deprecated_alias` = `old_event.event.id`.
3. The D-15.2.e identity contract is complete: bidirectional cross-reference
   with no plugin-side UUID coordination required.

### Either write fails (atomicity semantics)
4. If the `new_event` write fails after the `old_event` write succeeded, the
   host attempts to write a `vsdd.internal.emit_pair_partial_failure.v1`
   lifecycle event to the debug file, carrying the `old_event.event.id` of
   the orphaned half, and emits a `stderr` warning.
5. If the `old_event` write fails, the `new_event` write is NOT attempted.
   No orphan is created (the first write failed; the stream is consistent).
6. The `emit_pair` host call returns an error code to the plugin for any
   partial-write failure. The plugin MUST handle this as a non-fatal error
   (consistent with the non-blocking error contract from ADR-001).

### True atomicity scope
7. `emit_pair` does NOT guarantee that BOTH writes succeed or BOTH fail at
   the OS kernel level (that would require `fsync` + atomic rename, which is
   rejected per BC-1.11.002 design decision). The atomicity guarantee is:
   - The host coordinates the cross-reference UUID assignment (no plugin
     UUID mismatches possible).
   - The host detects the partial-write failure immediately and emits an
     observable signal (fallback event + stderr).
   - The orphaned-half condition is reduced from "silent" to "signaled."
   - Full crash atomicity (both-or-neither at OS level) is OUT OF SCOPE for
     v1. This is accepted and tracked as a known limitation.

## Invariants

1. `emit_pair` NEVER allows plugin code to set `event.correlation_id`,
   `event.deprecated_by`, or `event.replaces_deprecated_alias` — these fields
   are host-assigned inside `emit_pair`. Plugin-supplied values for these
   fields in the pair payloads are overridden.
2. The `event.correlation_id` is a fresh UUIDv4 generated per `emit_pair`
   call; it is the SAME UUID on both `old_event` and `new_event`.
3. `emit_pair` is NOT available as a standalone host import for non-Wave-2
   use. Its semantic is specifically "two events forming a deprecation pair."
   A plugin that uses `emit_pair` for non-deprecation-pair emissions
   produces malformed cross-references (the fields imply a migration pair
   that does not exist). Plugins MUST use `emit_event` for normal single
   emissions.

## Plugin-Side API Surface

```rust
// vsdd_hook_sdk::host module — Wave 2 addition
pub fn emit_pair(
    old_event_name: &str,
    old_fields: &[(&str, &str)],
    new_event_name: &str,
    new_fields: &[(&str, &str)],
) -> Result<EmitPairResult, HostError>;

pub struct EmitPairResult {
    pub old_event_id: Uuid,
    pub new_event_id: Uuid,
    pub correlation_id: Uuid,
}
```

The SDK wraps the underlying host ABI call. Both event payloads are
transmitted in a single host function invocation using the same
length-prefixed field encoding as `emit_event`, with a TLV framing that
distinguishes the two event halves.

## Legacy Two-Call Migration Path

Plugins that have NOT yet migrated to `emit_pair` continue to perform two
sequential `emit_event` calls. These work as before. The orphan-detection
rule from ADR-015 D-15.2.e applies: consumers tolerate dangling cross-
references (target `event.id` absent from the same trace scope) as non-paired
single-event records.

Migration to `emit_pair` is recommended but not mandatory during Wave 2.
All two-call shims MUST be removed at Wave 3 (shim removal) regardless of
whether `emit_pair` was adopted; Wave 3 shim removal eliminates dual-emit
entirely, making the atomicity question moot for post-Wave-3 operation.

## Related BCs

- BC-1.05.012 — emit_event enriches with host-owned fields (parent; emit_pair
  applies the same enrichment logic to both events)
- BC-1.11.001 — VSDD_TRACE_ID injection (sibling; trace context is inherited
  by both events in the pair via normal emit_event enrichment)
- BC-1.11.002 — FileSink partial-write recovery (sibling; emit_pair uses the
  same FileSink write path with the same failure semantics for each of the
  two writes)

## Architecture Anchors

- `crates/factory-dispatcher/src/host/emit_event.rs` — parent module;
  `emit_pair` is a new entry point in the same module
- `crates/hook-sdk/src/host.rs` — SDK-side `emit_pair` function
- ADR-015 D-15.2.e — orphan-half detection rule that `emit_pair` is designed
  to prevent
- ADR-015 OQ-8 — this BC resolves the open question

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `old_event` write succeeds; `new_event` write fails | `vsdd.internal.emit_pair_partial_failure.v1` emitted to debug file; stderr warning; old_event id logged as orphan |
| EC-002 | `old_event` write fails (e.g., disk full at first write) | `new_event` write NOT attempted; no orphan created; failure cascade per BC-1.11.002 |
| EC-003 | Plugin supplies `event.correlation_id` in the payload | Host overrides it with the emit_pair-generated UUID; `event.host_overrides` updated; `vsdd.internal.host_field_override.v1` emitted per D-15.3 |
| EC-004 | Plugin calls `emit_pair` after Wave 3 shim removal (post-migration) | `emit_pair` remains available but is a no-op shim that calls `emit_event` with only the `new_event` payload; the `old_event` is silently dropped; a stderr warning is emitted that the plugin is using `emit_pair` post-migration |
| EC-005 | Plugin uses `emit_pair` for a non-deprecation-pair (two unrelated events) | Functionally works but produces misleading cross-reference fields; this is plugin misuse; not prevented at the host level; documented as unsupported usage |

## Canonical Test Vectors

| Scenario | Expected events in events-*.jsonl |
|----------|------------------------------------|
| Both writes succeed | Two JSONL lines: old-name event with `event.deprecated_by` set; new-name event with `event.replaces_deprecated_alias` set; both share `event.correlation_id` |
| Old write succeeds, new write fails | One JSONL line (old-name event); one `vsdd.internal.emit_pair_partial_failure.v1` in debug file; stderr warning |
| Old write fails | Zero new JSONL lines; failure cascade per BC-1.11.002; no orphan |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (TBD — Phase 1.6b) | Both successful writes carry matching `event.correlation_id` | integration test |
| (TBD — Phase 1.6b) | `event.deprecated_by` on old-event equals `new_event.event.id` | unit test |
| (TBD — Phase 1.6b) | Partial failure (second write fails) produces observable signal | integration test with mocked FileSink |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-TBD |
| L2 Domain Invariants | TBD |
| Architecture Module | SS-01 — `crates/factory-dispatcher/src/host/emit_event.rs`; SS-02 — `crates/hook-sdk/src/host.rs` |
| Stories | Wave 2 story (TBD — dual-emit shim wave) |
| ADR | ADR-015 D-15.2.e (orphan-half detection); OQ-8 (this BC resolves it) |
| OQ Resolved | OQ-8 (atomic dual-emit host helper; v1 accepts orphan-half risk for sequential two-call shims; emit_pair is the Wave 2 mitigation) |

### Purity Classification

| Property | Assessment |
|----------|-----------|
| I/O operations | YES — two FileSink writes; optional debug file write on partial failure |
| Global state access | YES — shares HostContext and FileSink handle |
| Deterministic | YES given fixed HostContext and FileSink state |
| Thread safety | YES — FileSink is not shared across concurrent emit calls in single-thread dispatcher model |
| Overall classification | Effectful shell (two sequential file writes with coordinated UUID assignment) |
