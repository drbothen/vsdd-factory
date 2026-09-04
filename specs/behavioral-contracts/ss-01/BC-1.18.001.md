---
document_type: behavioral-contract
level: L3
version: "1.7"
status: active
producer: product-owner
timestamp: 2026-08-31T00:00:00Z
phase: F2
inputs:
  - .factory/specs/architecture/decisions/ADR-047-indeterminate-outcome-model-durable-mutation-marker-next-advance-gate.md
  - .factory/specs/architecture/decisions/ADR-048-fail-closed-but-recoverable-gate-block-if-marker-crash-policy-marker-ttl-deadman-and-ungated-escape-invariant.md
  - .factory/feature-delta/validation-integrity-layer1/F1-delta-analysis.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.17.001.md
  - .factory/cycles/v1.0-brownfield-backfill/S-25.04-f1-delta-analysis.md
  - .factory/cycles/v1.0-brownfield-backfill/S-25.04-f2-architecture-decisions.md
  - .factory/specs/behavioral-contracts/ss-04/BC-4.16.002.md
input-hash: "b6ad12a"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-041"
lifecycle_status: active
introduced: v1.0-feature-validation-integrity-layer1
modified: ["2026-08-31", "2026-08-31-v1.2-adr-048-d4-v1.2-ttl-locus-narration-correction", "2026-09-01-v1.3-adr-048-d4-v1.3-superseded-corollary", "2026-09-01-v1.4-adr-048-d4-v1.4-marker-written-event", "2026-09-01-v1.5-adr-048-d4-v1.5-superseded-emission-point-correction", "2026-09-03-v1.6-POL-14-auto-promotion-draft-to-active-S-25.01-merged-PR807-f3f9b3a1", "2026-09-04-v1.7-S-25.04-invariant-4-clarification-second-PostToolUse-instantiation"]
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-1.18.001: When a fail-closed Validator Cannot Complete, Dispatcher Classifies Outcome as INDETERMINATE, Emits `plugin.indeterminate` Event, and Writes Unvalidated-Mutation Marker

## Description

When a WASM hook plugin with `failure_policy = "fail-closed"` cannot complete — due to fuel
exhaustion (wasmtime `Trap::OutOfFuel`), epoch timeout (wasmtime `Trap::Interrupt`), or a host
function returning `OutputTooLarge (-3)` followed by the plugin returning `exit_code = 0` — the
dispatcher classifies the outcome as INDETERMINATE, a first-class named outcome distinct from PASS
and FAIL. For PostToolUse hooks, INDETERMINATE triggers: emission of a `plugin.indeterminate`
event to the OTel-aligned event log and atomic write of a durable `.factory/unvalidated-mutation.marker`
file. This mechanism converts the pre-Layer-1 silent false-PASS failure mode (CWE-754) into a
fail-LOUD quarantine-forward pattern (NIST SA-8(24) Fail Secure) without altering any existing
PASS or FAIL semantics.

## Preconditions

1. The dispatcher is running with a plugin that has `failure_policy = "fail-closed"` in its
   `[[hook]]` registry entry (`hooks-registry.toml` schema_version = 2; `FailurePolicy::FailClosed`
   in executor state).
2. One of the three cannot-complete conditions occurs:
   - (a) Fuel exhaustion: `PluginResult::Timeout { cause: TimeoutCause::Fuel }` observed — the
     wasmtime engine raises a Trap that downcasts to `Trap::OutOfFuel`.
   - (b) Epoch timeout: `PluginResult::Timeout { cause: TimeoutCause::Epoch }` observed — the
     wasmtime engine raises a Trap that downcasts to `Trap::Interrupt`.
   - (c) OutputTooLarge: any host function called from within the plugin returns `OutputTooLarge (-3)`
     (setting the per-invocation `StoreData::host_output_too_large_seen` flag to `true`), AND the
     plugin subsequently completes with `exit_code = 0`.
3. The dispatcher has write access to `.factory/` (or the configured factory root) for marker file
   creation.
4. The FileSink is operational for `plugin.indeterminate` event emission.

## Postconditions

1. **INDETERMINATE outcome classified.** The dispatcher's `classify_outcome` function returns
   `DispatchOutcome::Indeterminate` for the plugin invocation. Cause classification:
   - Fuel/epoch: via `Trap` variant downcast — `Trap::OutOfFuel` → cause=`fuel`;
     `Trap::Interrupt` → cause=`epoch`. `get_fuel()` is NOT authoritative and MUST NOT be used
     as the primary fuel-exhaustion signal (per ADR-047 §Decision 1 Implementation Note —
     remaining fuel counter is only supplementary when both fuel and epoch are enabled).
   - OutputTooLarge: post-invocation check — if `StoreData::host_output_too_large_seen == true`
     AND `exit_code == 0` AND `failure_policy == FailClosed` → cause=`output-too-large`. The
     `host_output_too_large_seen` flag MUST be reset to `false` immediately before each WASM
     module invocation (pre-invocation reset; ADR-047 §Decision 6 correctness requirement).
   - Any other Trap variant: NOT INDETERMINATE; route to existing `on_error` handling (wildcard
     arm required in Trap match — `Trap` is `#[non_exhaustive]`).

2. **INDETERMINATE is distinct from PASS and FAIL.** The trichotomy is:
   - PASS: `PluginResult::Ok { exit_code: 0 }` AND `host_output_too_large_seen == false`.
   - FAIL: `PluginResult::Ok { exit_code: non-zero }` OR crash/timeout with `on_error = "block"`.
   - INDETERMINATE: cannot-complete signal per PC1 above.
   INDETERMINATE does NOT alter PASS or FAIL paths. The ADR-039 axes-independence invariant is
   preserved: `on_error` and `failure_policy` govern orthogonal failure modes and are not unified.

3. **`plugin.indeterminate` event emitted.** The dispatcher emits a `plugin.indeterminate` event
   to the OTel-aligned event log (via FileSink, `events-*.jsonl`) with at minimum: `plugin_name`,
   `artifact_path` (from the PostToolUse payload), `cause` (one of: `fuel`, `epoch`,
   `output-too-large`), `trace_id` (dispatcher trace UUID), `failure_policy` (value:
   `"fail-closed"`). For fail-open plugins, the advisory event is also emitted — see
   BC-1.18.004 PC1. The wire format authority for `plugin.indeterminate` is BC-3.08.001
   (Event 8, as amended in this F2 burst).

4. **Durable marker written for fail-closed PostToolUse INDETERMINATE.** When `failure_policy ==
   FailClosed` AND the hook is PostToolUse (write has already occurred), the dispatcher atomically
   writes `.factory/unvalidated-mutation.marker` (write to a temp file in the same directory, then
   `rename`) with the following required TOML fields:
   ```toml
   timestamp = "<ISO-8601 UTC timestamp of the INDETERMINATE event>"
   plugin_name = "<name field from the [[hook]] entry in hooks-registry.toml>"
   artifact_path = "<absolute path of the artifact written in the triggering PostToolUse event>"
   cause = "<one of: fuel | epoch | output-too-large>"
   trace_id = "<dispatcher trace_id of the event that produced INDETERMINATE>"
   expires_at = "<ISO-8601 UTC timestamp = timestamp + UNVALIDATED_MUTATION_MARKER_TTL_SECONDS>"
   ```
   Where `UNVALIDATED_MUTATION_MARKER_TTL_SECONDS = 86_400` (24-hour deadman constant defined in
   `crates/factory-dispatcher/src/indeterminate_marker.rs`). `expires_at` is computed as
   `timestamp + Duration::seconds(86_400)` and stamped by `write_indeterminate_marker` at
   marker creation time. This field enables the dispatcher-native `check_and_clear_expired_marker`
   pre-check (`indeterminate_marker.rs`, called from `executor.rs`'s tier-execution loop before the
   Arm 1/Arm 2 WASM gate plugin runs) to auto-delete expired markers on the normal path
   (BC-1.18.003 PC4; ADR-048 §Decision 4 v1.2 — the gate plugin's `evaluate_gate` performs no
   `expires_at` parsing of its own) and enables the dispatcher's native `block_if_marker`
   crash-path check to honor TTL expiry (ADR-048 §Decision 2).
   Single-marker policy: if a marker already exists, it is overwritten (last-writer-wins). No
   per-plugin marker scheme in Layer 1 (ADR-047 §Decision 3 rationale).
   PreToolUse hooks do NOT write the marker (no write has occurred; the dispatch is blocked before
   any mutation).
   **`marker.written` audited creation event (v1.4, ADR-048 §Decision 4 v1.4 — S-25.01 adversary
   pass 6 F-P6-001 MEDIUM):** Immediately after `write_indeterminate_marker` returns `Ok(())` (the
   atomic temp-file-then-`rename` write succeeded), its caller (`executor.rs`, the same
   INDETERMINATE-outcome callsite that also performs the Invariant 3 SUPERSEDED check) emits
   `marker.written` (BC-3.08.001 Event 10) via `ctx.emit_internal` (`ctx: &HostContext`) — the same
   dual-sink primitive (durable `InternalLog` write to `dispatcher-internal-{date}.jsonl` + push
   onto `ctx.events`) every other dispatcher-native BC-3.08.001 event already uses — carrying the
   marker's OWN `trace_id`, `plugin_name`, `artifact_path`, `cause`, and `expires_at` fields (the
   same values just written to the TOML file above; `timestamp` is supplied by the event's own `ts`
   common field, not re-derived). Emission occurs at exactly one point in the marker lifecycle:
   strictly AFTER a confirmed successful write, NEVER before the write is attempted, and NEVER when
   `write_indeterminate_marker` returns `Err(_)` (EC-007 — atomic rename failure). `marker.written`
   is a positive creation record: it exists if and only if a marker was actually, durably written.
   This is what makes `reconcile_raw_delete`'s OPERATOR_OVERRIDE inference (BC-1.18.003 PC3) sound
   by construction rather than inferred from a proxy signal — see BC-1.18.003 PC3 and EC-017 for the
   full reconciliation-premise correction this event enables.
   **Write-success arm now emits BOTH write-tied events together (v1.5, ADR-048 §Decision 4 v1.5 —
   S-25.01 adversary pass 9 F-P9-001 MEDIUM):** the same `Ok(())` arm that triggers `marker.written`
   ALSO triggers `marker.cleared(clear_mode=SUPERSEDED, actor_type=system)` for a superseded
   cross-pair old marker, if the Invariant 3 SUPERSEDED check (above) found one — SUPERSEDED fires
   FIRST (for the OLD pair, using fields read before the overwrite), then `marker.written` fires
   SECOND (for the NEW pair). On `Err(_)`, NEITHER event is emitted — this closes the un-swept
   sibling gap the pre-v1.5 text left open: SUPERSEDED had previously been emitted unconditionally
   at the pre-overwrite READ (before the write was even attempted), so a subsequent write failure
   left a fabricated SUPERSEDED record on disk-inconsistent grounds (the OLD marker was still
   durably present and enforcing, yet a record already claimed it was overwritten — NIST AU-3/AU-10).
   See Invariant 3's SUPERSEDED-clear corollary (below) and BC-1.18.003 PC5/EC-018 for the full
   clear-path postcondition and the direct F-P9-001 regression test.

5. **Existing PASS/FAIL semantics UNCHANGED.** INDETERMINATE is a THIRD outcome class; it does not
   redefine PASS or FAIL. Existing `on_error = "block"` and `on_error = "continue"` semantics are
   preserved. Existing `PluginResult::Timeout` and `PluginResult::Crashed` event emission continues
   unchanged; `plugin.indeterminate` is ADDITIVE. All ~76 current production plugins with absent or
   `fail-open` `failure_policy` observe no behavior change beyond an additional advisory
   `plugin.indeterminate` event (see BC-1.18.004).

## Invariants

1. **Per-invocation Store flag reset.** `StoreData::host_output_too_large_seen` MUST be reset to
   `false` immediately before each WASM guest invocation — not only at Store creation. Failure to
   reset would cause invocation N's flag to misclassify invocation N+1 as INDETERMINATE even when
   N+1's host functions all returned successfully. The reset is a correctness requirement, not
   future-proofing.

2. **Trap non-exhaustive wildcard.** The match on wasmtime `Trap` variants MUST include a wildcard
   arm `_ => { /* not INDETERMINATE; route to on_error */ }`. `Trap` is `#[non_exhaustive]` (24+
   variants at authoring time); a future wasmtime upgrade MUST NOT silently bucket a new Trap variant
   as INDETERMINATE.

3. **Single-marker file.** Only one `.factory/unvalidated-mutation.marker` exists at a time.
   Simultaneous multi-plugin INDETERMINATE events use last-writer-wins. This is correct for Layer 1
   (Layer 3 bounded windows will reduce concurrent INDETERMINATE events to near-zero).
   **SUPERSEDED-clear corollary (v1.3, ADR-048 §Decision 4 v1.3 — F-P3-002 LOW; emission point
   corrected v1.5, ADR-048 §Decision 4 v1.5 — F-P9-001):** last-writer-wins
   overwrite is silent (no audited-clear event) ONLY when the overwritten and overwriting events
   share the SAME `(plugin_name, artifact_path)` pair — a same-pair re-INDETERMINATE is continuous
   quarantine of the same target, already fully covered by the marker's own `trace_id` update at
   overwrite; no `marker.cleared` is emitted for it. When the marker currently belongs to pair A
   `(plugin_a, artifact_a)` and a DIFFERENT pair B `(plugin_b, artifact_b)` subsequently goes
   INDETERMINATE fail-closed, `write_indeterminate_marker`'s caller MUST read A's existing marker
   fields (`trace_id`, `plugin_name`, `artifact_path`) BEFORE the temp+rename overwrite — this read
   is unavoidable, since the write itself overwrites the file the fields are read from — and emit
   `marker.cleared` (BC-3.08.001 Event 9) with `clear_mode = "SUPERSEDED"`, `actor_type = "system"`,
   A's OWN `trace_id`/`plugin_name`/`artifact_path` (never B's), and
   `reason = "SUPERSEDED: marker overwritten by a new plugin.indeterminate event for a different
   (plugin_name, artifact_path) pair before being cleared; last-writer-wins (BC-1.18.001 INV3)"`
   (non-null) — via `ctx.emit_internal`, ONLY AFTER B's `write_indeterminate_marker` call returns
   `Ok(())` (**corrected v1.5** — previously stated as BEFORE B's marker write completes), in the
   SAME write-success arm as `marker.written`'s emission for B (SUPERSEDED first, then
   `marker.written`). On `Err(_)`, NEITHER `marker.cleared(SUPERSEDED)` for A NOR `marker.written`
   for B is emitted. This closes a
   false-attribution audit-integrity gap (NIST AU-3/AU-10): without the AFTER-`Ok(())` gating,
   a write failure would leave A's marker still durably present and enforcing while a SUPERSEDED
   record had already been emitted falsely claiming it was overwritten — the identical fabrication
   class this corollary exists to prevent; without the corollary at all, `reconcile_raw_delete`
   (BC-1.18.003 PC3) would later find A's `plugin.indeterminate` unmatched and mis-attribute A's
   silent supersession to a human `OPERATOR_OVERRIDE` that never happened. The full clear-path
   postcondition (event contents, ordering, the same-pair non-emission case, and the write-failure
   non-fabrication case) is BC-1.18.003 PC5/EC-018; this BC states only the write-time triggering
   corollary.

4. **Marker is PostToolUse-only.** PreToolUse INDETERMINATE (e.g., if the gate plugin itself
   cannot complete) does NOT write the marker. The gate plugin (`validate-unvalidated-mutation-marker`)
   is registered `failure_policy = "fail-open"` to prevent unconditional self-lock (ADR-047
   §Decision 4 rationale).

   **Clarification (v1.7, S-25.04 F2 — product-owner):** "PostToolUse-only" is a property of the
   *event type* a `failure_policy = "fail-closed"` plugin is registered on, NOT an enumerated,
   closed set of specific plugins. Any fail-closed plugin registered `PostToolUse` reaches this
   marker-write path on a genuine cannot-complete outcome; any fail-closed plugin registered
   `PreToolUse` structurally cannot, regardless of which plugin it is. `validate-factory-path-staged`
   (BC-4.16.002, S-25.04) is the SECOND concrete PostToolUse fail-closed instantiation of this
   invariant — `validate-unvalidated-mutation-marker` (the Arm 1/Arm 2 gate plugin referenced
   above) is explicitly NOT such an instantiation: it is `failure_policy = "fail-open"` per
   ADR-047 §Decision 4, so it never reaches this path at all. This note prevents a future reader
   from conflating "the gate plugin" with "a marker-writing plugin" — they are disjoint by design.
   This is a clarification of the existing rule's scope; it introduces no postcondition,
   wire-format, or mechanism change.

5. **OutputTooLarge flag is dispatcher-internal.** `host_output_too_large_seen` lives in the
   dispatcher's `StoreData` struct. It is NOT part of the hook-sdk ABI (`crates/hook-sdk/`). Plugin
   WASM binaries do not read or write this flag directly. No HOST_ABI_VERSION bump required.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Fail-closed plugin fuel-exhausts on a PreToolUse hook | INDETERMINATE classified; `plugin.indeterminate` advisory event emitted; NO marker written (PreToolUse — no write has occurred; the dispatch is blocked by Arm 1 if a prior marker exists). |
| EC-002 | Fail-open plugin fuel-exhausts (any hook) | INDETERMINATE classified at outcome level; advisory `plugin.indeterminate` event emitted; NO marker written; NO gate triggered. See BC-1.18.004. |
| EC-003 | Plugin exits with `exit_code = 1` AND `host_output_too_large_seen = true` | Outcome is FAIL (plugin correctly propagated the error). NOT INDETERMINATE. AMD-003 semantics preserved. |
| EC-004 | Plugin exits `exit_code = 0` AND `host_output_too_large_seen = false` | Outcome is PASS (normal case). NOT INDETERMINATE. |
| EC-005 | Marker file already exists when a second fail-closed INDETERMINATE fires for the SAME `(plugin_name, artifact_path)` pair | Existing marker overwritten with new event's details (last-writer-wins, single-marker policy). Same-pair re-INDETERMINATE — NO `marker.cleared(SUPERSEDED)` emitted (INV3 corollary; contrast EC-009). |
| EC-006 | `Trap` variant is neither `OutOfFuel` nor `Interrupt` | NOT INDETERMINATE for the fuel/epoch axis. Routes to existing `on_error` handling. Wildcard arm required. |
| EC-007 | Atomic rename fails (disk full, permissions) | Marker write fails; `plugin.indeterminate` event still emitted. The failure is logged; dispatcher does not panic. **v1.4:** NO `marker.written` event is emitted for this failed write — `marker.written` is emitted only after a confirmed `Ok(())` return from `write_indeterminate_marker` (PC4 v1.4). This is load-bearing: a write failure producing the same positive creation record as a write success would make `reconcile_raw_delete`'s OPERATOR_OVERRIDE inference unsound again for the write-failure case (BC-1.18.003 PC3; see EC-017 there). **v1.5 addition (ADR-048 §D4 v1.5 — F-P9-001):** if this failed write was ALSO a cross-pair overwrite (an existing marker for a different pair was about to be superseded — EC-009), `marker.cleared(SUPERSEDED)` for the superseded pair is likewise NOT emitted — NEITHER write-tied event fires on failure, symmetric by construction (BC-1.18.003 PC5/EC-018). |
| EC-008 | OutputTooLarge fires but plugin exit_code = 1 (plugin blocked correctly) | FAIL outcome (not INDETERMINATE). Plugin correctly propagated the error. |
| EC-009 | Marker for pair A `(plugin_a, artifact_a)` exists; a DIFFERENT pair B `(plugin_b, artifact_b)` goes INDETERMINATE fail-closed and overwrites it, AND `write_indeterminate_marker(&pair_b, ...)` returns `Ok(())` (v1.3, ADR-048 §D4 v1.3 — F-P3-002; ordering corrected v1.5, ADR-048 §D4 v1.5 — F-P9-001) | Marker overwritten with B's fields (last-writer-wins, INV3). ONLY AFTER B's write returns `Ok(())` (**corrected v1.5** — previously described as before the overwrite completes), `write_indeterminate_marker`'s caller emits `marker.cleared(clear_mode="SUPERSEDED", actor_type="system", reason=non-null)` carrying A's OWN `trace_id`/`plugin_name`/`artifact_path` (not B's) via `ctx.emit_internal`, in the same write-success arm as `marker.written` for B (SUPERSEDED first, then `marker.written`) (BC-1.18.003 PC5). |
| EC-010 | Marker for pair A `(plugin_a, artifact_a)` exists; a DIFFERENT pair B `(plugin_b, artifact_b)` goes INDETERMINATE fail-closed and `write_indeterminate_marker(&pair_b, ...)` returns `Err(_)` — atomic rename fails during a cross-pair overwrite (v1.5, ADR-048 §D4 v1.5 — F-P9-001) | NO `marker.cleared(SUPERSEDED)` emitted for A and NO `marker.written` emitted for B. A's marker remains durably present on disk and still enforcing its quarantine — emitting SUPERSEDED here would be a fabricated audit record (NIST AU-3/AU-10) falsely claiming it was overwritten. Direct regression test for F-P9-001 (BC-1.18.003 EC-018). |

## Canonical Test Vectors

| Scenario | Input State | Expected Outcome | Expected Marker |
|----------|-------------|-----------------|-----------------|
| Fuel exhaustion, fail-closed, PostToolUse | `TimeoutCause::Fuel`, `failure_policy = FailClosed`, PostToolUse | INDETERMINATE; `plugin.indeterminate` emitted with `cause="fuel"` | Written with cause=`fuel` |
| Epoch timeout, fail-closed, PostToolUse | `TimeoutCause::Epoch`, `failure_policy = FailClosed`, PostToolUse | INDETERMINATE; `plugin.indeterminate` emitted with `cause="epoch"` | Written with cause=`epoch` |
| OutputTooLarge + exit_code=0, fail-closed, PostToolUse | `host_output_too_large_seen=true`, exit_code=0, `failure_policy = FailClosed`, PostToolUse | INDETERMINATE; `plugin.indeterminate` emitted with `cause="output-too-large"` | Written with cause=`output-too-large` |
| Normal PASS, fail-closed, PostToolUse | exit_code=0, `host_output_too_large_seen=false`, `failure_policy = FailClosed` | PASS | Not written |
| Normal FAIL, fail-closed, PostToolUse | exit_code=1, `host_output_too_large_seen=false`, `failure_policy = FailClosed` | FAIL | Not written |
| Fuel exhaustion, fail-open, PostToolUse | `TimeoutCause::Fuel`, `failure_policy = FailOpen` | advisory INDETERMINATE event only | Not written |
| OutputTooLarge + exit_code=1, fail-closed | `host_output_too_large_seen=true`, exit_code=1, `failure_policy = FailClosed` | FAIL (plugin correctly blocked) | Not written |
| Cross-pair overwrite (SUPERSEDED), fail-closed, PostToolUse, write succeeds | Marker exists for pair A `(plugin_a, artifact_a)`; DIFFERENT pair B `(plugin_b, artifact_b)` fires INDETERMINATE fail-closed; `write_indeterminate_marker(&pair_b, ...)` returns `Ok(())` | INDETERMINATE (for B); `marker.cleared(clear_mode="SUPERSEDED", actor_type="system", reason=non-null)` emitted for A's own fields via `ctx.emit_internal` ONLY AFTER B's write returns `Ok(())` (corrected v1.5 — previously before B's write completes; BC-1.18.003 PC5) | Overwritten with B's fields (last-writer-wins) |
| Cross-pair overwrite (SUPERSEDED), fail-closed, PostToolUse, write fails (v1.5 negative control) | Marker exists for pair A `(plugin_a, artifact_a)`; DIFFERENT pair B `(plugin_b, artifact_b)` fires INDETERMINATE fail-closed; `write_indeterminate_marker(&pair_b, ...)` returns `Err(_)` | INDETERMINATE (for B); `plugin.indeterminate` still emitted; NO `marker.cleared(SUPERSEDED)` for A and NO `marker.written` for B (BC-1.18.003 EC-018) | Marker A unchanged (write failed; A's fields remain) |

## Related BCs

- BC-1.18.002 — next-advance gate that fires when the marker exists (composes with this BC)
- BC-1.18.003 — marker-clear protocol (composes with this BC; successful re-validation deletes marker)
- BC-1.18.004 — fail-open advisory-only behavior (sibling; backward-compat anchor)
- BC-3.08.001 — SS-03 event catalog; Event 8 `plugin.indeterminate` wire-format authority (this BC is the triggering-condition/semantics authority; BC-3.08.001 is the field-shape authority; same pattern as Event 7/BC-1.03.019)
- BC-1.03.019 — `plugin.fuel_headroom_warning` event (sibling SS-01 BC; same observability pattern; Event 7 in BC-3.08.001)

## Architecture Anchors

- `crates/factory-dispatcher/src/executor.rs` — primary: `classify_outcome(PluginResult, FailurePolicy, bool) -> DispatchOutcome` pure function; INDETERMINATE classification logic; OutputTooLarge Store-flag post-invocation check
- `crates/factory-dispatcher/src/invoke.rs` — `StoreData::host_output_too_large_seen: bool` flag; pre-invocation reset; host function wrappers set flag when returning OutputTooLarge (-3)
- `crates/factory-dispatcher/src/indeterminate_marker.rs` (new module) — `write_indeterminate_marker`, `delete_marker_if_pass`, `should_write_marker`; atomic rename write; TOML field serialization. **v1.3 addition (ADR-048 §D4 v1.3 — F-P3-002):** `write_indeterminate_marker`'s caller (`executor.rs`, the marker-write callsite for INDETERMINATE outcomes) reads the EXISTING marker's fields (if any) BEFORE the temp+rename overwrite; if the existing marker's `(plugin_name, artifact_path)` differs from the new event's, it calls `emit_marker_cleared(ctx, ...)` (`ctx: &HostContext`) with `clear_mode = "SUPERSEDED"`, `actor_type = "system"` — symmetrical with the read-before-delete `delete_marker_if_pass`'s REVALIDATED path already performs. See BC-1.18.003 PC5 for the full clear-path postcondition. **v1.4 addition (ADR-048 §D4 v1.4 — F-P6-001):** the same callsite additionally calls a new `emit_marker_written(ctx, &fields)` function immediately after `write_indeterminate_marker` returns `Ok(())` (and after the SUPERSEDED check above, which reads the pre-overwrite state) — emitting `marker.written` (BC-3.08.001 Event 10) via `ctx.emit_internal`. `emit_marker_written` is called ONLY on the `Ok(())` arm; the `Err(_)` arm (EC-007) does not call it. **v1.5 correction (ADR-048 §D4 v1.5 — F-P9-001):** the v1.3 `emit_marker_cleared(ctx, ..., SUPERSEDED, system)` call described above is RELOCATED — it is no longer invoked at the pre-overwrite read; the read of A's fields remains at that point (unavoidable), but the call itself moves to INSIDE `write_indeterminate_marker`'s `Ok(())` arm, immediately BEFORE the v1.4 `emit_marker_written` call in that same arm (SUPERSEDED fires first, then `marker.written`). On `Err(_)`, NEITHER call fires. This is the un-swept sibling of the v1.4 correction: v1.4 gated `marker.written` on `Ok(())`; v1.5 applies the identical gating to `emit_marker_cleared(..., SUPERSEDED, ...)`.
- `crates/hook-sdk/src/host.rs` — NOT modified for ABI (flag is dispatcher-internal StoreData only); no HOST_ABI_VERSION bump
- `plugins/vsdd-factory/hooks-registry.toml` — `failure_policy = "fail-closed"` assignments for Cohort A; new `validate-unvalidated-mutation-marker` gate entries

## Story Anchor

S-25.01 — Dispatcher INDETERMINATE Outcome Layer 1: Fail-Loud on Cannot-Complete — durable marker + next-advance gate

## VP Anchors

- VP-102 — Fuel-Exhaustion and Epoch-Timeout Yield INDETERMINATE Outcome for fail-closed Plugin (unit-test; covers PC1 fuel/epoch axis)
- VP-103 — Host OutputTooLarge Then Plugin Ok(exit:0) Yields INDETERMINATE for fail-closed Plugin (unit-test; covers PC1 OutputTooLarge axis + PC1 per-invocation reset invariant)
- VP-104 — INDETERMINATE for fail-closed Plugin Writes Unvalidated-Mutation Marker with Required Fields (unit-test; covers PC4 + Invariant 3)
- VP-108 — marker.cleared/marker.written Audited-Event Emission Correctness (unit-test; PC5 covers the SUPERSEDED corollary triggered by this BC's Invariant 3 — cross-pair overwrite emits `marker.cleared(clear_mode=SUPERSEDED, actor_type=system)` for the superseded pair's own fields via `ctx.emit_internal`, ONLY AFTER `write_indeterminate_marker` returns `Ok(())` (**corrected v1.5, ADR-048 §D4 v1.5 — F-P9-001**); **v1.4 addition (ADR-048 §D4 v1.4 — F-P6-001):** VP-108 PC6 covers this BC's own PC4 v1.4 `marker.written` emission — `write_indeterminate_marker` returning `Ok(())` emits `marker.written` via `emit_marker_written`; returning `Err(_)` emits nothing; VP-108 PC7 is the negative-control regression test proving `reconcile_raw_delete` does not fabricate `marker.cleared(OPERATOR_OVERRIDE)` when no `marker.written` was ever emitted (BC-1.18.003 PC3/EC-017); **v1.5 addition (ADR-048 §D4 v1.5 — F-P9-001):** VP-108 PC8 is the negative-control regression test proving that a cross-pair overwrite where `write_indeterminate_marker` returns `Err(_)` emits NEITHER `marker.cleared(SUPERSEDED)` NOR `marker.written` (this BC's EC-010; BC-1.18.003 PC5/EC-018); VP-108 is the emission-correctness VP, not this BC's own VP-104, which covers only the marker WRITE's TOML-field contents)

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-102 | Fuel exhaustion on fail-closed plugin classifies as INDETERMINATE; epoch timeout on fail-closed plugin classifies as INDETERMINATE; fail-open timeout does NOT classify as blocking INDETERMINATE; PASS/FAIL paths unaffected | unit-test |
| VP-103 | host_output_too_large_seen=true + exit_code=0 + FailClosed → INDETERMINATE; same + FailOpen → not blocking INDETERMINATE; exit_code=1 + OTL → FAIL; no-OTL + exit_code=0 → PASS; per-invocation flag reset verified | unit-test |
| VP-104 | INDETERMINATE + fail-closed PostToolUse writes marker with all six required TOML fields (timestamp, plugin_name, artifact_path, cause, trace_id, expires_at); expires_at = timestamp + UNVALIDATED_MUTATION_MARKER_TTL_SECONDS (86400s), stamped by write_indeterminate_marker at creation; cause serializes correctly (fuel/epoch/output-too-large); atomic write (no temp file left); plugin_name and trace_id match input; fail-open does NOT write marker | unit-test |
| VP-108 | Cross-pair marker overwrite (Invariant 3 SUPERSEDED corollary; v1.3; ordering corrected v1.5): `write_indeterminate_marker`'s caller, given an existing marker for a DIFFERENT `(plugin_name, artifact_path)` pair, reads the existing marker's fields before overwrite and emits `marker.cleared(clear_mode=SUPERSEDED, actor_type=system, reason=non-null)` via `ctx.emit_internal` carrying the SUPERSEDED pair's own trace_id/plugin_name/artifact_path (not the new pair's), ONLY AFTER the new marker write returns `Ok(())` (**corrected v1.5, ADR-048 §D4 v1.5, F-P9-001** — previously described as before the new write completes); same-pair re-INDETERMINATE emits nothing | unit-test |
| VP-108 | (v1.4, PC6, ADR-048 §D4 v1.4 — F-P6-001): `write_indeterminate_marker` returning `Ok(())` → its caller emits `marker.written` via `emit_marker_written(ctx, &fields)` carrying the marker's own trace_id/plugin_name/artifact_path/cause/expires_at; `write_indeterminate_marker` returning `Err(_)` → zero `marker.written` events emitted | unit-test |
| VP-108 | (v1.4, PC7, ADR-048 §D4 v1.4 — F-P6-001 regression test): negative control — a fixture with an unmatched `plugin.indeterminate` but NO `marker.written` record (simulating a PreToolUse fail-closed INDETERMINATE or a PostToolUse write failure) → `reconcile_raw_delete` emits ZERO `marker.cleared(OPERATOR_OVERRIDE)` events for that pair (BC-1.18.003 PC3/EC-017) | unit-test |
| VP-108 | (v1.5, PC8, ADR-048 §D4 v1.5 — F-P9-001 regression test): negative control — existing marker for pair A, `write_indeterminate_marker(&pair_b, ...)` FORCED to return `Err(_)` during a cross-pair overwrite → ZERO `marker.cleared(SUPERSEDED)` events for A AND ZERO `marker.written` events for B are emitted (this BC's EC-010; BC-1.18.003 PC5/EC-018) | unit-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-041 |
| Capability Anchor Justification | CAP-041 ("Validation Integrity: INDETERMINATE Outcome, Durable Mutation Marker, and Next-Advance Gate") per capabilities.md §CAP-041 — this BC specifies the complete outcome classification trichotomy and the marker-write PostToolUse behavior that is the core of what CAP-041 defines: "The dispatcher classifies plugin non-completion (fuel exhaustion, epoch timeout, host OutputTooLarge) as a named INDETERMINATE outcome … causes … atomic write of a durable marker file." |
| L2 Domain Invariants | none (dispatcher runtime invariant, not L2 domain spec) |
| Architecture Module | SS-01 (Hook Dispatcher Core — executor.rs classification + invoke.rs Store flag + indeterminate_marker.rs new module) |
| ADR | ADR-047 §Decision 1 (outcome trichotomy); ADR-047 §Decision 2 (failure_policy reuse, no new field); ADR-047 §Decision 3 (durable marker path, TOML fields, atomic write, single-marker policy); ADR-047 §Decision 6 (OutputTooLarge Store-flag mechanism, per-invocation reset); ADR-047 §Decision 7 (backward-compatibility contract); ADR-039 §Decision 1 (failure_policy field semantics + axes-independence invariant); ADR-048 §Decision 2 (expires_at sixth required TOML field; UNVALIDATED_MUTATION_MARKER_TTL_SECONDS = 86_400 24h deadman constant; expires_at stamped by write_indeterminate_marker at creation; TTL checked by the dispatcher-native `check_and_clear_expired_marker` pre-check on the normal path + by dispatcher native `block_if_marker_check` on crash path — both dispatcher-native per ADR-048 §Decision 4 v1.2 Emission-Point Correction; the gate plugin's `evaluate_gate` has no `expires_at` awareness of its own); **ADR-048 §Decision 4 v1.3 Emission-Mechanism Precision Correction + SUPERSEDED Clear Mode** (S-25.01 LOCAL adversary pass 3 F-P3-002 LOW; human-ratified) — Invariant 3 SUPERSEDED corollary: cross-pair marker overwrite (last-writer-wins) MUST emit `marker.cleared(clear_mode=SUPERSEDED, actor_type=system, reason=non-null)` for the superseded pair's own fields via `ctx.emit_internal` ONLY AFTER the new marker write returns `Ok(())` (**corrected v1.5, ADR-048 §D4 v1.5, F-P9-001** — previously described as before the new write completes); same-pair re-INDETERMINATE emits nothing; closes the false-`OPERATOR_OVERRIDE`-attribution audit-integrity gap in `reconcile_raw_delete` (BC-1.18.003 PC3); **ADR-048 §Decision 4 v1.4 Reconciliation-Premise Correction** (S-25.01 adversary pass 6 F-P6-001 MEDIUM; architect adjudication) — PC4 gains the `marker.written` (BC-3.08.001 Event 10) audited creation event: emitted via `ctx.emit_internal` ONLY after `write_indeterminate_marker` returns `Ok(())`, never before, never on `Err(_)`; this positive creation record replaces the false pre-v1.4 premise that an unmatched fail-closed `plugin.indeterminate` proves a marker was durably written — closing the false-`OPERATOR_OVERRIDE`-attribution gap reachable via a PreToolUse fail-closed INDETERMINATE (INV4 — marker never attempted) or a PostToolUse marker-write I/O failure (EC-007); **ADR-048 §Decision 4 v1.5 Emission-Point Correction** (S-25.01 adversary pass 9 F-P9-001 MEDIUM; architect adjudication; HUMAN-RATIFIED 2026-09-01, POLICY 22, D-1142) — the SUPERSEDED corollary above is relocated from the unconditional pre-overwrite read to inside `write_indeterminate_marker`'s `Ok(())` arm, alongside the (unchanged) PC4 `marker.written` emission (SUPERSEDED fires first, then `marker.written`); on `Err(_)`, NEITHER event is emitted — closing the identical fabricated-audit-record class (NIST AU-3/AU-10) for SUPERSEDED that v1.4 closed for `marker.written`/OPERATOR_OVERRIDE reconciliation; the un-swept sibling gap the v1.4 amendment left open |
| Stories | S-25.01; S-25.04 (invariant 4 clarification — second PostToolUse fail-closed instantiation) |
| Cycle | v1.0-feature-validation-integrity-layer1 (F2 — product-owner spec burst); v1.0-brownfield-backfill (v1.7 clarification burst) |
| Feature | E-25 — Validation Integrity and Large-Artifact Resilience |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.7 | 2026-09-04 | product-owner | S-25.04 clarification amendment (F1/F2 delta analysis; human-ratified BROAD trigger scope for the new companion validator). Invariant 4 gains a forward-citing clarification paragraph: "PostToolUse-only" is a property of the event type a fail-closed plugin is registered on, not an enumerated closed set of plugins; `validate-factory-path-staged` (BC-4.16.002, new SS-04/CAP-034 sibling of BC-4.16.001) is cited as the SECOND concrete PostToolUse fail-closed instantiation of this rule; the Arm 1/Arm 2 gate plugin (`validate-unvalidated-mutation-marker`) is explicitly noted as NOT such an instantiation (it is `failure_policy = "fail-open"` per ADR-047 §Decision 4). No rewrite of the existing invariant-4 sentence; no postcondition, wire-format, or mechanism content change — clarification only. Traceability Stories/Cycle rows gain S-25.04/v1.0-brownfield-backfill citations. |
| 1.6 | 2026-09-03 | state-manager | POL-14 auto-promotion (S-25.01 MERGED PR #807 squash `f3f9b3a1` into `develop` 2026-09-03; D-1159): `status`/`lifecycle_status` draft→active. No content/postcondition/wire-format change — mechanical POLICY-14 consequence of the anchoring story's merge. |
| 1.5 | 2026-09-01 | product-owner | ADR-048 §Decision 4 v1.5 Emission-Point Correction (S-25.01 adversary pass 9 F-P9-001 MEDIUM; architect adjudication; HUMAN-RATIFIED 2026-09-01, POLICY 22, D-1142). Invariant 3's SUPERSEDED-clear corollary and PC4's `marker.written`-adjacent write-success-arm description are corrected so that BOTH write-tied audited events — `marker.cleared(clear_mode=SUPERSEDED, actor_type=system)` for a superseded cross-pair old marker, and `marker.written` for the new marker — are emitted together, ONLY inside `write_indeterminate_marker`'s `Ok(())` arm (SUPERSEDED first, then `marker.written`); on `Err(_)`, NEITHER is emitted. This closes F-P9-001: the pre-v1.5 text described SUPERSEDED as emitted unconditionally at the pre-overwrite field-read, BEFORE `write_indeterminate_marker` was even called, so a subsequent write failure (EC-007) left the OLD marker still durably present and enforcing while a SUPERSEDED record had already been emitted falsely claiming it was overwritten — a fabricated audit record (NIST AU-3/AU-10), the un-swept sibling of this BC's own v1.4 `marker.written` "emit only after `Ok(())`" fix. (1) PC4's `marker.written` paragraph gains a new "Write-success arm now emits BOTH write-tied events together" clause. (2) Invariant 3's SUPERSEDED-clear corollary rewritten: read-before-overwrite stated as unavoidable and unchanged; emission gated on `Ok(())`; new write-failure non-fabrication clause added. (3) EC-007 gains a clause: SUPERSEDED is likewise withheld when the failed write was also a cross-pair overwrite. (4) EC-009 corrected to AFTER-`Ok(())` ordering; new EC-010 added — the direct F-P9-001 regression test (cross-pair overwrite + write failure ⟹ neither event emitted). (5) Canonical Test Vectors: cross-pair-overwrite row corrected; new write-failure negative-control row added. (6) Architecture Anchors: `indeterminate_marker.rs` bullet gains the v1.5 correction note (SUPERSEDED call relocated into the `Ok(())` arm, immediately before `emit_marker_written`). (7) VP Anchors: VP-108 bullet's SUPERSEDED clause corrected to AFTER-`Ok(())` ordering; gains a new v1.5/PC8 citation. (8) Verification Properties: SUPERSEDED row corrected; new VP-108 PC8 row added. (9) Traceability ADR row: ADR-048 §Decision 4 v1.5 citation added; v1.3 SUPERSEDED-corollary citation's ordering language corrected in place. This BC's write-side postcondition is the counterpart to BC-1.18.003 v1.7's PC5 emission-point correction and BC-3.08.001 v1.34's Event 9/Event 10 wire-format-authority ordering correction — all three amendments jointly implement ADR-048 §D4 v1.5 / F-P9-001's fix, ground-truthed against the architect's ratified ADR-048 v1.5 and VP-108 v1.4. |
| 1.4 | 2026-09-01 | product-owner | ADR-048 §Decision 4 v1.4 Reconciliation-Premise Correction (S-25.01 adversary pass 6 F-P6-001 MEDIUM; architect adjudication; HUMAN-RATIFIED 2026-09-01, POLICY 22, D-1141). Adds `marker.written` (BC-3.08.001 Event 10) as a new audited creation event: emitted via `ctx.emit_internal` by `write_indeterminate_marker`'s caller (`executor.rs`) IMMEDIATELY after the atomic write returns `Ok(())`, carrying the marker's own trace_id/plugin_name/artifact_path/cause/expires_at — NEVER emitted before the write, NEVER on `Err(_)` (write failure). This closes F-P6-001: the pre-v1.4 `reconcile_raw_delete` OPERATOR_OVERRIDE inference ("an unmatched fail-closed `plugin.indeterminate` proves a marker was durably written and later raw-deleted") is false whenever a PreToolUse fail-closed INDETERMINATE never attempts a marker write (INV4) or a PostToolUse marker-write I/O failure (EC-007) leaves the same no-marker-ever-existed footprint — both fabricate `marker.cleared(OPERATOR_OVERRIDE)`, a false NIST AU-3/AU-10 audit record. (1) PC4 gains the `marker.written` emission paragraph (trigger, fields, ONLY-after-Ok(()) constraint). (2) EC-007 gains a clause: no `marker.written` on write failure. (3) Architecture Anchors: `indeterminate_marker.rs` bullet gains the v1.4 `emit_marker_written` note. (4) VP Anchors: VP-108 bullet renamed to cover both `marker.cleared`/`marker.written`; gains PC6 (write-path emission) + PC7 (negative control) citations. (5) Verification Properties: two new VP-108 rows (PC6, PC7). (6) Traceability ADR row: ADR-048 §Decision 4 v1.4 citation added. This BC's write-side postcondition is the counterpart to BC-1.18.003 v1.6's reconciliation-side retarget (scan now matches unmatched `marker.written`, not unmatched `plugin.indeterminate`) and BC-3.08.001's new Event 10 wire-format catalog entry. |
| 1.3 | 2026-09-01 | product-owner | ADR-048 §Decision 4 v1.3 Emission-Mechanism Precision Correction + SUPERSEDED Clear Mode (S-25.01 LOCAL adversary pass 3 F-P3-002 LOW; human-ratified 2026-09-01, POLICY 22, D-1140, per ADR-048 v1.3 Status). Invariant 3 gains a SUPERSEDED-clear corollary: a cross-pair marker overwrite (a new INDETERMINATE for a `(plugin_name, artifact_path)` pair DIFFERENT from the marker currently on disk) MUST emit `marker.cleared(clear_mode=SUPERSEDED, actor_type=system, reason=non-null)` for the superseded pair's own fields (trace_id/plugin_name/artifact_path — never the new pair's) via `ctx.emit_internal`, BEFORE the new marker write completes; a same-pair re-INDETERMINATE (continuous quarantine of the same target) does NOT emit SUPERSEDED — already covered by the marker's own `trace_id` update at overwrite. This closes the F-P3-002 false-`OPERATOR_OVERRIDE`-attribution audit-integrity gap: without it, `reconcile_raw_delete` (BC-1.18.003 PC3) would later find the superseded pair's `plugin.indeterminate` unmatched and mis-attribute its silent supersession to a human override that never happened. (1) Invariant 3 corollary added. (2) EC-005 clarified as the same-pair (no-emission) case; new EC-009 added for the cross-pair SUPERSEDED case. (3) Canonical Test Vectors: new cross-pair-overwrite row added. (4) Architecture Anchors: `indeterminate_marker.rs` bullet gains the v1.3 read-before-overwrite + `emit_marker_cleared(ctx, ..., SUPERSEDED, system)` note. (5) VP Anchors + Verification Properties: VP-108 added (this BC's Invariant 3 write-time trigger; the full clear-path postcondition — event contents, ordering, same-pair non-emission — is BC-1.18.003 PC5, which is the emission-correctness home per the established VP-104/VP-108 split). (6) Traceability ADR row: ADR-048 §Decision 4 v1.3 citation added. This is the write-time triggering corollary only; BC-1.18.003 v1.5 carries the full SUPERSEDED clear-path postcondition and BC-3.08.001 v1.32 carries the SUPERSEDED `clear_mode`/`system` `actor_type` wire-format enum additions. |
| 1.2 | 2026-08-31 | product-owner | Spot-check correction (ADR-048 §Decision 4 v1.2 Emission-Point Correction; human-ratified; sibling of BC-1.18.003 v1.4/BC-3.08.001 v1.31). PC4's `expires_at` field description narrated the pre-v1.2 locus ("This field enables the gate plugin to auto-delete expired markers on the normal path") — corrected to attribute the normal-path TTL auto-delete to the new dispatcher-native `check_and_clear_expired_marker` pre-check (`indeterminate_marker.rs`, called from `executor.rs`'s tier-execution loop before the WASM gate plugin runs); the gate plugin's `evaluate_gate` has no `expires_at` awareness of its own. Traceability ADR row's ADR-048 §Decision 2 citation ("TTL checked by gate plugin on normal path") corrected to name `check_and_clear_expired_marker` and cite ADR-048 §Decision 4 v1.2. No postcondition, wire-format, or field-shape change — this BC's own scope (marker WRITE) is otherwise unaffected; only stale TTL-locus narration was corrected. |
| 1.1 | 2026-08-31 | product-owner | ADR-048 §Decision 2 — adds `expires_at` as 6th required TOML marker field in PC4 (stamped by `write_indeterminate_marker` at creation; `UNVALIDATED_MUTATION_MARKER_TTL_SECONDS = 86_400` 24h deadman). Sweeps all "5 fields" → "6 fields": VP-104 Verification Properties row updated to reference all six fields including `expires_at`. Traceability ADR updated with ADR-048 §Decision 2 citation. ADR-048 added to inputs. |
| 1.0 | 2026-08-30 | product-owner | Initial creation. F2 spec-evolution burst, validation-integrity-layer1. BC-1.18.001: INDETERMINATE outcome classification (fuel/epoch/OTL), plugin.indeterminate event, durable marker (PostToolUse+fail-closed), backward-compat (PC5). VPs VP-102/VP-103/VP-104 anchored. CAP-041 capability anchor. ADR-047 §D1/D2/D3/D6/D7 citations. |
