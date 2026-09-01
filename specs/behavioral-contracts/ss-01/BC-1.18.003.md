---
document_type: behavioral-contract
level: L3
version: "1.6"
status: draft
producer: product-owner
timestamp: 2026-08-31T00:00:00Z
phase: F2
inputs:
  - .factory/specs/architecture/decisions/ADR-047-indeterminate-outcome-model-durable-mutation-marker-next-advance-gate.md
  - .factory/specs/architecture/decisions/ADR-048-fail-closed-but-recoverable-gate-block-if-marker-crash-policy-marker-ttl-deadman-and-ungated-escape-invariant.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.18.001.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.18.002.md
  - .factory/feature-delta/validation-integrity-layer1/F1-delta-analysis.md
input-hash: "f722156"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-041"
lifecycle_status: draft
introduced: v1.0-feature-validation-integrity-layer1
modified: ["2026-08-31", "2026-08-31-v1.2-ttl-third-clear-path", "2026-08-31-v1.3-audited-clear-model-marker-cleared-event", "2026-08-31-v1.4-adr-048-d4-v1.2-emission-point-correction", "2026-09-01-v1.5-adr-048-d4-v1.3-emission-mechanism-precision-correction-plus-superseded-clear-path", "2026-09-01-v1.6-adr-048-d4-v1.4-reconciliation-premise-correction"]
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-1.18.003: Successful Re-Validation Clears the Unvalidated-Mutation Marker (Idempotent; Operator Escape Hatch Supported)

## Description

The `.factory/unvalidated-mutation.marker` lifecycle has four supported clear paths. Primary
path: when the same plugin that produced INDETERMINATE is subsequently dispatched PostToolUse on
the same artifact and produces `DispatchOutcome::Pass` (exit_code=0, `host_output_too_large_seen=false`),
the dispatcher deletes the marker file and emits `marker.cleared(REVALIDATED)`. Secondary path:
the operator manually deletes the marker via `rm .factory/unvalidated-mutation.marker` — a fully
supported and documented escape hatch requiring no special command or credential; a retroactive
`marker.cleared(OPERATOR_OVERRIDE)` event is reconciled via RAW_DELETE_DETECTED on the next gate
evaluation. Tertiary path (TTL deadman): the dispatcher-native `check_and_clear_expired_marker` pre-check
(`indeterminate_marker.rs`, called from `executor.rs`'s tier-execution loop BEFORE the Arm 1/Arm 2
WASM gate plugin runs) reads the marker on the normal execution path and, if it finds `expires_at`
elapsed (UTC), treats the marker as ABSENT, allows the dispatch, auto-deletes the marker file
(idempotent; swallow NotFound), and emits `marker.cleared(TTL_EXPIRED)` (TTL-loudness: previously
silent auto-delete is now audited per ADR-048 §Decision 4). **Emission-point correction (v1.4,
ADR-048 §D4 v1.2):** this determination, deletion, and emission are entirely dispatcher-native —
NOT performed by the WASM gate plugin's `evaluate_gate`, which by the time it runs sees only an
already-absent-or-non-expired marker and performs a pure presence check with no `expires_at`
awareness of its own. Quaternary path (SUPERSEDED — v1.5, ADR-048 §D4 v1.3): under BC-1.18.001
INV3 single-marker last-writer-wins, when a marker currently belonging to pair A
`(plugin_a, artifact_a)` is overwritten by a NEW INDETERMINATE event for a DIFFERENT pair B
`(plugin_b, artifact_b)` before A was ever cleared, `write_indeterminate_marker`'s caller reads
A's fields before the overwrite and emits `marker.cleared(clear_mode=SUPERSEDED, actor_type=system)`
for A — preventing a later raw-delete of B's marker from causing `reconcile_raw_delete` to
mis-attribute A's silent supersession to a human `OPERATOR_OVERRIDE` that never happened. All
four clear paths unblock the two gate arms simultaneously and emit a `marker.cleared` audited
event (BC-3.08.001 Event 9) with `clear_mode` one of
`REVALIDATED | TTL_EXPIRED | OPERATOR_OVERRIDE | SUPERSEDED`. **Emission-mechanism correction
(v1.5, ADR-048 §D4 v1.3):** all `marker.cleared` emissions (and `plugin.indeterminate`, Event 8)
go via `HostContext::emit_internal` — the same dual-sink helper (durable `InternalLog` write +
`ctx.events` queue) every sibling BC-3.08.001 dispatcher-native event already uses — never a raw
`InternalLog::write` call; `reconcile_raw_delete`'s bounded scan target is
`dispatcher-internal-{date}.jsonl` (via `InternalLog`), not a literal `events-{date}.jsonl` (see
PC3 and §Architecture Anchors below). **Reconciliation-premise correction (v1.6, ADR-048 §D4 v1.4
— S-25.01 adversary pass 6 F-P6-001 MEDIUM):** the RAW_DELETE_DETECTED scan that powers the
OPERATOR_OVERRIDE retroactive reconciliation now matches an unmatched `marker.written` (BC-3.08.001
Event 10 — emitted by `write_indeterminate_marker`'s caller ONLY after a confirmed successful
marker write, BC-1.18.001 PC4 v1.4) rather than an unmatched `plugin.indeterminate` (which fires
for EVERY INDETERMINATE outcome, PreToolUse or PostToolUse, whether or not a marker was ever
written). The pre-v1.6 premise — an unmatched fail-closed `plugin.indeterminate` proves a marker
was durably written and later raw-deleted — is FALSE whenever no marker was ever written at all: a
PreToolUse fail-closed INDETERMINATE (BC-1.18.001 INV4 — marker write is PostToolUse-only) or a
PostToolUse marker-write I/O failure (BC-1.18.001 EC-007) both leave an unmatched
`plugin.indeterminate` with no marker ever having existed, which the pre-v1.6 scan would misread as
a raw delete and fabricate `marker.cleared(OPERATOR_OVERRIDE)` for — a false NIST AU-3/AU-10
non-repudiation record. `marker.written` is a positive creation record, so an unmatched one is
proof-by-construction (not inference) that a marker existed and was later cleared by something
other than the three already-audited paths. See PC3 (below) for the full postcondition and EC-017
for the direct non-fabrication regression case. The clear operation is idempotent: if the marker is
absent at delete time, the operation is a no-op (no error). A failed re-validation (FAIL outcome)
does NOT clear the marker.

## Preconditions

1. `.factory/unvalidated-mutation.marker` exists (was written by a prior fail-closed PostToolUse
   INDETERMINATE event per BC-1.18.001 PC4).
2. For the primary clear path (PC1): The same plugin named in the marker's `plugin_name` field
   is dispatched again in a PostToolUse event on the same artifact (or a write to the same artifact
   path).
3. The operator has shell access to the session's working directory for the secondary escape-hatch
   path (PC3).

## Postconditions

1. **Successful re-validation deletes the marker (artifact-scoped) and emits `marker.cleared(REVALIDATED)`.** When the plugin named in
   the marker's `plugin_name` field is dispatched in a PostToolUse hook, produces
   `DispatchOutcome::Pass` (exit_code=0, `host_output_too_large_seen=false`), AND the current
   dispatch's artifact path matches the marker's `artifact_path` field, the dispatcher executes
   `delete_marker_if_pass` and deletes `.factory/unvalidated-mutation.marker`. "Matches" means
   exact string equality of the normalized absolute path (both paths resolved to absolute form
   before comparison; no trailing-slash normalization required beyond what the OS provides).
   **Empty-path fallback:** if the marker's `artifact_path` is empty (the failing plugin was a
   non-artifact-scoped validator — e.g., a wave-gate or path-staging validator with no single
   associated file), the artifact condition is vacuously satisfied and plugin_name equality alone
   suffices for clearance. After deletion, the subsequent `^Agent$` dispatch AND the next
   `git commit`/`git push` Bash dispatch are allowed by the gate (BC-1.18.002 PC4 path — marker
   absent → both arms pass). Clearance is scoped to the same plugin AND the same artifact: if a
   DIFFERENT plugin produces PASS, the marker is NOT cleared; if the same plugin produces PASS on
   a DIFFERENT non-empty artifact, the marker is also NOT cleared (artifact A's quarantine
   persists — A was never successfully re-validated).
   **Audited clear (ADR-048 §Decision 4):** immediately after `std::fs::remove_file(marker_path)`
   succeeds, the dispatcher MUST emit `marker.cleared` (BC-3.08.001 Event 9) with:
   `clear_mode = "REVALIDATED"`, `actor_type = "validator"`, `trace_id` = trace_id from the
   originating `plugin.indeterminate` event (read from marker TOML `trace_id` field),
   `plugin_name` from marker TOML, `artifact_path` from marker TOML, `reason = null`,
   `timestamp` = time of the clear event. Emitted via `emit_marker_cleared` in
   `crates/factory-dispatcher/src/indeterminate_marker.rs`, called from `delete_marker_if_pass`.

2. **Absent-marker clear is idempotent.** If `.factory/unvalidated-mutation.marker` is absent
   at delete time, `delete_marker_if_pass` is a no-op — it does NOT return an error and does
   NOT create a new file. The session remains operational. This covers the case where the marker
   was already cleared by a previous action (race-free semantics).

3. **Operator manual escape hatch with retroactive `marker.cleared(OPERATOR_OVERRIDE)` reconciliation.**
   `rm .factory/unvalidated-mutation.marker` is a fully supported operator action. No special
   command, no credential, no sentinel file, and no Agent dispatch is required. After the file
   is deleted, the next `^Agent$` dispatch and the next `git commit`/`git push` Bash dispatch
   proceed without block. This matches the break-glass posture from ADR-039 §Decision 3:
   authentication is by possession of shell access to the machine running the session.
   **Retroactive audited clear (ADR-048 §Decision 4):** since T3 is not dispatcher-mediated, a
   real-time `marker.cleared` event cannot be emitted at the moment of deletion. Reconciliation:
   the same dispatcher-native `check_and_clear_expired_marker` pre-check (`indeterminate_marker.rs`,
   `executor.rs` tier-execution loop, before the Arm 1/Arm 2 WASM gate plugin runs) finds the
   marker absent in its marker-absent branch, and the dispatcher (`reconcile_raw_delete`,
   `indeterminate_marker.rs`) performs a bounded, best-effort scan for an
   unmatched `marker.written` (BC-3.08.001 Event 10) for the same `(plugin_name, artifact_path)`
   with no corresponding `marker.cleared`. **Scan-target correction (v1.5, ADR-048 §D4 v1.3):** the
   scan target is `dispatcher-internal-{date}.jsonl` (via the durable `InternalLog`), not a literal
   `events-{date}.jsonl` — see §Architecture Anchors below for the full rationale.
   **Scan match-type correction (v1.6, ADR-048 §D4 v1.4 — S-25.01 adversary pass 6 F-P6-001
   MEDIUM):** the scan MUST match on an unmatched `marker.written` record, NOT an unmatched
   `plugin.indeterminate` record (with or without a `failure_policy == "fail-closed"` filter — that
   filter is now structurally redundant against `marker.written` and is REMOVED). `plugin.indeterminate`
   (Event 8) is emitted for EVERY INDETERMINATE outcome regardless of hook phase (BC-3.08.001 Event
   8 trigger), while `marker.written` (Event 10) is emitted ONLY after `write_indeterminate_marker`
   returns `Ok(())` (BC-1.18.001 PC4 v1.4). Matching on `plugin.indeterminate` was unsound: a
   PreToolUse fail-closed INDETERMINATE never attempts a marker write at all (BC-1.18.001 INV4 —
   marker write is PostToolUse-only), and a PostToolUse marker-write I/O failure (BC-1.18.001
   EC-007, swallowed best-effort) leaves the identical no-marker-ever-existed footprint — both
   produce an unmatched `plugin.indeterminate` with no marker ever written, which the pre-v1.6 scan
   would misread as a raw delete and fabricate `marker.cleared(OPERATOR_OVERRIDE)` for (a false NIST
   AU-3/AU-10 audit record). `marker.written`'s existence is proof-by-construction that a marker was
   actually, durably written, so an unmatched `marker.written` is a sound RAW_DELETE_DETECTED
   signal; see EC-017 below for the direct non-fabrication regression case.
   **Emission-point correction (v1.4, ADR-048 §D4 v1.2):** this
   reconciliation is entirely dispatcher-native — NOT performed by the WASM gate plugin, which
   cannot honor the `marker.cleared` wire contract's foreign `trace_id`/`plugin_name` requirement
   from inside the WASM sandbox (RESERVED_FIELDS wall on the `emit_event` host ABI). If found, the
   dispatcher (`reconcile_raw_delete`) emits a retroactive `marker.cleared` (BC-3.08.001
   Event 9) with: `clear_mode = "OPERATOR_OVERRIDE"`, `actor_type = "operator"`,
   `reason = "RAW_DELETE_DETECTED: marker absent without prior marker.cleared event; inferred operator out-of-band clear"`,
   `timestamp` = current evaluation time (deletion time is unobservable), `trace_id` = trace_id
   from the unmatched `marker.written` event (v1.6 — previously sourced from the unmatched
   `plugin.indeterminate` event; same field name, corrected source record), `plugin_name` /
   `artifact_path` = likewise read from the unmatched `marker.written` event's own fields.
   **Best-effort:** if the `dispatcher-internal-
   {date}.jsonl` log is unavailable (e.g., `ctx.internal_log` is `None`) or the unmatched record
   cannot be found, the annotation is omitted — no hard failure. An unreconciled gap is observable
   by tooling that monitors the event stream for `marker.written` events without subsequent
   `marker.cleared` (v1.6 — previously `plugin.indeterminate`).

4. **TTL-expiry clear path (marker-level deadman) with `marker.cleared(TTL_EXPIRED)` audit emission.**
   **Emission-point correction (v1.4, ADR-048 §D4 v1.2 — human-ratified):** TTL detection,
   auto-delete, and audited-clear emission are performed entirely by the dispatcher-native
   `check_and_clear_expired_marker` function (`crates/factory-dispatcher/src/indeterminate_marker.rs`),
   called from `executor.rs`'s tier-execution loop for every registry entry with
   `on_error == OnError::BlockIfMarker` BEFORE the Arm 1/Arm 2 WASM gate plugin's `evaluate_gate`
   is invoked on the normal (non-crash) path. This supersedes the v1.0–v1.3 attribution of TTL
   detection/deletion/emission to the gate plugin's `evaluate_gate`: the `emit_event` host ABI's
   RESERVED_FIELDS enrichment unconditionally overwrites any plugin-supplied `trace_id`/`plugin_name`
   with the CURRENT gate-plugin invocation's own dispatch identity, making it structurally
   impossible for `evaluate_gate` to emit `marker.cleared` carrying the MARKER's own
   `trace_id`/`plugin_name` (which the wire contract, BC-3.08.001 Event 9, requires). When
   `check_and_clear_expired_marker` reads `.factory/unvalidated-mutation.marker` and finds
   `expires_at` elapsed (`expires_at ≤ now (UTC)`), it treats the marker as ABSENT: deletes the
   marker file (idempotent; swallow `NotFound`) and returns `Some(fields)`, signaling the caller to
   treat the dispatch as Allow (`exit_code = 0`). The auto-delete prevents accumulation of dead
   marker artifacts. This is the THIRD clear path alongside PC1 (successful re-validation) and PC3
   (operator manual rm). By the time `evaluate_gate` subsequently runs, the marker is guaranteed
   already absent-or-non-expired — `evaluate_gate` performs NO `expires_at` parsing, deletion, or
   emission logic of its own; it is a pure marker-presence check.
   **TTL-loudness (ADR-048 §Decision 4):** immediately after the TTL auto-delete succeeds,
   `check_and_clear_expired_marker`'s caller MUST emit `marker.cleared` (BC-3.08.001 Event 9) with:
   `clear_mode = "TTL_EXPIRED"`, `actor_type = "deadman"`, `trace_id` from marker TOML `trace_id`
   field, `plugin_name` from marker TOML, `artifact_path` from marker TOML, `reason = null`,
   `timestamp` = time of the clear event. Emitted via `emit_marker_cleared` in
   `indeterminate_marker.rs`, called from the dispatcher-native pre-check — never from inside the
   WASM gate plugin. This replaces the prior SILENT TTL auto-delete. Note: the dispatcher's NATIVE
   crash-path TTL check (Decision 1 — crash + marker + expired → Allow, `block_if_marker_check`)
   does NOT emit `marker.cleared(TTL_EXPIRED)` — only the dispatcher-native normal-path pre-check
   (`check_and_clear_expired_marker`) emits it; these are two distinct dispatcher-native functions.
   The marker remains for the next normal-path pre-check to clear and emit the audited event.
   **`expires_at` stamping:** the `expires_at` field is written by `write_indeterminate_marker`
   at marker creation time as `timestamp + UNVALIDATED_MUTATION_MARKER_TTL_SECONDS` (86400s per
   ADR-048 §Decision 2; BC-1.18.001 PC4). Backward compatibility: markers written before
   ADR-048 implementation lack `expires_at`. `check_and_clear_expired_marker` MUST treat a missing
   `expires_at` field as non-expired (conservative; no silent auto-clear of old markers), returning
   `None` and leaving the marker in place — the subsequent `evaluate_gate` then sees a still-present
   marker and Blocks unconditionally, with no `expires_at` awareness required. Such markers remain
   in effect until explicitly cleared via rm (PC3) or until ADR-048 is implemented and new
   markers with `expires_at` replace them.

5. **SUPERSEDED clear path (cross-pair marker overwrite) with `marker.cleared(SUPERSEDED)` audit
   emission (v1.5, ADR-048 §D4 v1.3 — F-P3-002 LOW).** This is the FOURTH clear path, parallel to
   PC1 (successful re-validation), PC3 (operator manual rm), and PC4 (TTL expiry) — but distinct
   from all three in that it is triggered by a marker WRITE, not a marker deletion: under
   BC-1.18.001 Invariant 3 (single-marker, last-writer-wins), if pair A `(plugin_a, artifact_a)`
   currently holds the marker (INDETERMINATE, fail-closed, never cleared) and pair B
   `(plugin_b, artifact_b)` — a DIFFERENT pair — subsequently also goes INDETERMINATE fail-closed,
   B's `write_indeterminate_marker` call overwrites A's marker file. Before v1.5, this overwrite
   was silent: A's `plugin.indeterminate` record would later appear "unmatched" to
   `reconcile_raw_delete` if B's marker were ever raw-deleted out-of-band, causing a FALSE
   `marker.cleared(OPERATOR_OVERRIDE)` attribution for A (no human ever acted on A). **Fix:**
   `write_indeterminate_marker`'s caller (`executor.rs`, the marker-write callsite for
   INDETERMINATE outcomes) reads the EXISTING marker's fields (`trace_id`, `plugin_name`,
   `artifact_path`) BEFORE the temp+rename overwrite. If the existing marker's
   `(plugin_name, artifact_path)` DIFFERS from the new event's, the caller emits `marker.cleared`
   (BC-3.08.001 Event 9) via `ctx.emit_internal` — BEFORE the new marker write completes — with:
   `clear_mode = "SUPERSEDED"`, `actor_type = "system"`, `trace_id` = A's OWN trace_id (read before
   overwrite, NOT B's), `plugin_name` = A's own plugin_name, `artifact_path` = A's own
   artifact_path, `reason = "SUPERSEDED: marker overwritten by a new plugin.indeterminate event
   for a different (plugin_name, artifact_path) pair before being cleared; last-writer-wins
   (BC-1.18.001 INV3)"` (non-null — `SUPERSEDED` joins `OPERATOR_OVERRIDE` as a
   `reason`-mandatory `clear_mode`), `timestamp` = time of the overwrite. **Same-pair exception:**
   when the existing marker's `(plugin_name, artifact_path)` is the SAME as the new event's (the
   same validator re-INDETERMINATEs on the same artifact before being cleared), NO `SUPERSEDED`
   event is emitted — this is continuous quarantine of the same target, not a cross-pair
   supersession, and is already fully covered by the marker's own `trace_id` update at overwrite
   (see BC-1.18.001 Invariant 3 corollary). **Closes the reconciliation gap:** no change to
   `reconcile_raw_delete`'s matching logic is required — its scan already treats ANY
   `type == "marker.cleared"` record (regardless of `clear_mode`) as closing the
   `(plugin_name, artifact_path)` key, so a `SUPERSEDED` record for A closes A's key the moment
   B's write happens; a later raw-delete of B's marker no longer finds A unmatched.

## Invariants

1. **FAIL re-validation preserves the marker.** If the re-validating plugin produces FAIL
   (exit_code!=0), the marker is NOT deleted. The unvalidated state persists until either a
   successful re-validation (PC1) or manual clearance (PC3). This is the correct behavior: FAIL
   means the validator checked the write and found a blocking condition — the mutation should not
   proceed until the condition is resolved.

2. **Clear is scoped to the named plugin AND the original artifact.** Only the plugin whose name
   matches `plugin_name` in the marker, dispatched PostToolUse on an artifact whose normalized
   absolute path equals the marker's `artifact_path` (exact string equality), can trigger the
   automatic clear on PASS. A different plugin producing PASS does NOT clear the marker. The same
   plugin producing PASS on a different non-empty artifact does NOT clear the marker — artifact
   A's quarantine persists until A is successfully re-validated by the named plugin. **Empty-path
   exception:** if the marker's `artifact_path` is empty, the artifact condition is vacuously
   satisfied; plugin_name equality alone suffices for clearance. The named-plugin + artifact-scoped
   clear is deterministic and auditable.

3. **Both gate arms unblocked simultaneously.** A single marker deletion (by any clear path)
   unblocks both Arm 1 (Agent dispatch) and Arm 2 (git commit/push Bash) simultaneously. No
   per-arm unblock is needed.

4. **Idempotent delete must not error.** The `delete_marker_if_pass` function MUST use
   `fs::remove_file` and silently swallow `io::ErrorKind::NotFound`. Any other IO error
   (permissions, filesystem failure) MUST be propagated as an error to the caller (not silently
   swallowed) — the session must surface a real filesystem problem rather than masking it.

5. **TTL is marker-level expiry, not artifact-scoped.** Any `.factory/unvalidated-mutation.marker`
   expires 86400 seconds after its `expires_at` timestamp, REGARDLESS of which artifact triggered
   the INDETERMINATE event. TTL expiry (PC4) does not check `plugin_name` or `artifact_path`
   fields — it operates solely on the time dimension. This is orthogonal to the artifact-scoped
   re-validation clear (PC1/INV2), which requires (a) plugin_name match AND (b) artifact_path
   match. Consistency with single-marker policy (BC-1.18.001 INV3): the single marker carries one
   `expires_at` timestamp; if the marker is overwritten by a new INDETERMINATE event (last-writer-
   wins), the new `expires_at` resets the TTL clock to the most recent event. The MOST RECENT
   quarantine signal governs the effective TTL. This is the correct behavior: a new INDETERMINATE
   event on a second artifact renews the quarantine; it would be incorrect to TTL-expire the marker
   based on the first event's timestamp while the second event is still unresolved.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Re-validating plugin produces FAIL (exit_code=1) | Marker NOT deleted. Gate remains active. Unvalidated state persists. |
| EC-002 | Re-validating plugin fuel-exhausts again (INDETERMINATE) on re-run | Marker overwritten with new event (single-marker last-writer-wins per BC-1.18.001 Invariant 3). Gate remains active. |
| EC-003 | Different plugin produces PASS on same artifact | Marker NOT cleared (clear is scoped to named plugin per Invariant 2). |
| EC-004 | Marker absent at delete time (idempotent delete) | No-op. No error. Session continues. |
| EC-005 | Operator runs `rm .factory/unvalidated-mutation.marker` | Both gate arms immediately unblocked. Next Agent dispatch and git commit/push proceed. |
| EC-006 | Marker file has wrong permissions (delete fails) | `delete_marker_if_pass` propagates IO error (NOT silently swallowed). Session surfaces the failure. |
| EC-007 | CI environment has stale marker from prior run | `rm -f .factory/unvalidated-mutation.marker` in CI setup step clears it; test harness proceeds. Stale marker should be treated as a CI setup artifact, not a validation requirement. |
| EC-008 | Same plugin, DIFFERENT non-empty artifact, PostToolUse Pass | Marker NOT cleared. Artifact A's quarantine persists — plugin `p` validated artifact B successfully but artifact A was never re-validated. Gate remains active until plugin `p` produces PASS on artifact A (or operator manual deletion). |
| EC-009 | Same plugin, empty marker `artifact_path`, PostToolUse Pass on any artifact | Marker cleared (empty-path fallback: artifact condition vacuously satisfied; plugin_name equality alone suffices for clearance). Both gate arms unblocked. This covers non-artifact-scoped validators (e.g., wave-gate, path-staging validators) whose marker carries no artifact context. |
| EC-010 | PC1 path — marker cleared via re-validation PASS | `marker.cleared` event emitted with `clear_mode="REVALIDATED"`, `actor_type="validator"`, `trace_id` from originating `plugin.indeterminate`, `reason=null`. Follows immediately after `std::fs::remove_file` succeeds. |
| EC-011 | PC4 path — marker cleared via TTL expiry (dispatcher-native `check_and_clear_expired_marker` pre-check, before `evaluate_gate` runs; `expires_at` ≤ now UTC) | `marker.cleared` event emitted with `clear_mode="TTL_EXPIRED"`, `actor_type="deadman"`, `trace_id` from marker TOML, `reason=null`. TTL-loudness: previously silent auto-delete is now audited. Emission-point corrected v1.4 (ADR-048 §D4 v1.2) — dispatcher-native, not `evaluate_gate`. |
| EC-012 | PC3 path — marker cleared via operator out-of-band rm; `dispatcher-internal-{date}.jsonl` has an unmatched `marker.written` (v1.6 — previously described as unmatched `plugin.indeterminate`) | Dispatcher-native `check_and_clear_expired_marker` pre-check finds marker absent on next evaluation; dispatcher-native `reconcile_raw_delete` emits retroactive `marker.cleared` with `clear_mode="OPERATOR_OVERRIDE"`, `actor_type="operator"`, `reason="RAW_DELETE_DETECTED: marker absent without prior marker.cleared event; inferred operator out-of-band clear"`, `timestamp`=current eval time, `trace_id`/`plugin_name`/`artifact_path` from the unmatched `marker.written` event. Emission-point corrected v1.4 (ADR-048 §D4 v1.2) — dispatcher-native, not the WASM gate plugin. Scan match-type corrected v1.6 (ADR-048 §D4 v1.4) — matches unmatched `marker.written`, not unmatched `plugin.indeterminate`. |
| EC-013 | PC3 path — operator out-of-band rm; FileSink log unavailable | No `marker.cleared` emitted — best-effort; no hard failure. Gap is observable by monitoring for unmatched `marker.written` events (v1.6 — previously `plugin.indeterminate`) in the event stream. (`reconcile_raw_delete` is dispatcher-native per v1.4 correction; the best-effort/no-hard-failure behavior itself is unchanged.) |
| EC-014 | Crash-path TTL allow (dispatcher native check, Decision 1) — expires_at ≤ now UTC | No `marker.cleared(TTL_EXPIRED)` emitted — only the dispatcher-native normal-path pre-check (`check_and_clear_expired_marker`) emits it. Marker remains until the next normal-path pre-check clears it with the audited event. |
| EC-015 | PC5 path (v1.5) — marker for pair A `(plugin_a, artifact_a)` is overwritten by a NEW INDETERMINATE event for a DIFFERENT pair B `(plugin_b, artifact_b)` before A was ever cleared | `write_indeterminate_marker`'s caller emits `marker.cleared` for A with `clear_mode="SUPERSEDED"`, `actor_type="system"`, `trace_id`/`plugin_name`/`artifact_path` = A's OWN fields (not B's), `reason` non-null, via `ctx.emit_internal`, BEFORE B's marker write completes. Marker is then overwritten with B's fields (last-writer-wins, BC-1.18.001 INV3). |
| EC-016 | PC5 path (v1.5) — same plugin re-INDETERMINATEs on the SAME artifact before the marker is cleared (same-pair overwrite) | No `marker.cleared(SUPERSEDED)` emitted — continuous quarantine of the same target; already covered by the marker's own `trace_id` update at overwrite (contrast EC-015; see BC-1.18.001 EC-005). |
| EC-017 | PC3 negative control (v1.6, ADR-048 §D4 v1.4 — S-25.01 adversary pass 6 F-P6-001 MEDIUM) — a PreToolUse fail-closed INDETERMINATE (BC-1.18.001 INV4 — marker write is PostToolUse-only, so no marker write is ever attempted) OR a PostToolUse marker-write I/O failure (BC-1.18.001 EC-007) produces a `plugin.indeterminate` event with NO corresponding `marker.written` ever emitted | `reconcile_raw_delete`'s bounded scan of `dispatcher-internal-{date}.jsonl` finds no unmatched `marker.written` for that `(plugin_name, artifact_path)` pair; NO `marker.cleared(OPERATOR_OVERRIDE)` is fabricated — the marker was, correctly, never present because none was ever durably written. This is the direct regression test for F-P6-001: the pre-v1.6 scan (matching on unmatched `plugin.indeterminate`) could not distinguish this case from a genuine T3 raw delete and would incorrectly emit a fabricated OPERATOR_OVERRIDE audit record for it. |

## Canonical Test Vectors

| Scenario | Input State | Expected Outcome |
|----------|-------------|-----------------|
| Same plugin PASS on same artifact | Marker exists (plugin_name="regression-gate", artifact_path="/abs/A.md"), then "regression-gate" PostToolUse on "/abs/A.md" → PASS | Marker deleted; gate allows subsequent Agent and git-commit dispatches |
| Same plugin FAIL on same artifact | Marker exists (plugin_name="regression-gate", artifact_path="/abs/A.md"), then "regression-gate" PostToolUse on "/abs/A.md" → FAIL | Marker NOT deleted; gate still active |
| Different plugin PASS on same artifact | Marker exists (plugin_name="regression-gate", artifact_path="/abs/A.md"), then "convergence-tracker" PostToolUse on "/abs/A.md" → PASS | Marker NOT deleted (different plugin; EC-003) |
| Same plugin, DIFFERENT non-empty artifact, PASS | Marker exists (plugin_name="regression-gate", artifact_path="/abs/A.md"), then "regression-gate" PostToolUse on "/abs/B.md" → PASS | Marker NOT cleared; gate still active (A unvalidated; EC-008) |
| Same plugin, empty marker artifact_path, PASS | Marker exists (plugin_name="wave-gate-check", artifact_path=""), then "wave-gate-check" PostToolUse on any artifact → PASS | Marker cleared (empty-path fallback — plugin_name-only match; EC-009) |
| Idempotent delete — marker absent | No marker; `delete_marker_if_pass` called | No-op; no error |
| Manual escape hatch | Marker exists; operator runs `rm .factory/unvalidated-mutation.marker` | Marker gone; next Agent dispatch allowed |
| INDETERMINATE on re-run | Marker exists; same plugin fuel-exhausts again | Marker overwritten with new event; gate remains active |
| TTL-expiry clear — expired marker | Marker exists with `expires_at` in the PAST (UTC); dispatcher-native `check_and_clear_expired_marker` pre-check reads marker before `evaluate_gate` runs | Allow (exit_code=0); marker auto-deleted (idempotent; NotFound swallowed); both gate arms unblocked (PC4); no plugin_name or artifact_path check required; `marker.cleared(TTL_EXPIRED)` emitted (EC-011) |
| TTL-expiry — non-expired marker | Marker exists with `expires_at` in the FUTURE (UTC); `check_and_clear_expired_marker` returns None, `evaluate_gate` then reads marker | Block (exit_code=2); existing behavior unchanged; marker remains; no `marker.cleared` emitted |
| TTL-expiry — marker missing expires_at (legacy pre-ADR-048) | Marker exists without `expires_at` field; `check_and_clear_expired_marker` reads on normal path | Block (exit_code=2) — marker treated as non-expired (conservative backward-compat; PC4 note); marker NOT auto-deleted; no `marker.cleared` emitted |
| marker.cleared REVALIDATED emission | Marker exists; same plugin PASS on same artifact | After `delete_marker_if_pass` succeeds: `marker.cleared` emitted with `clear_mode="REVALIDATED"`, `actor_type="validator"`, `trace_id` from `plugin.indeterminate`, `reason=null` (EC-010) |
| marker.cleared TTL_EXPIRED emission | Marker expired; dispatcher-native `check_and_clear_expired_marker` normal-path pre-check eval | After TTL auto-delete: `marker.cleared` emitted with `clear_mode="TTL_EXPIRED"`, `actor_type="deadman"`, `reason=null` (EC-011; emission point corrected v1.4 — dispatcher-native, not `evaluate_gate`) |
| marker.cleared OPERATOR_OVERRIDE reconciliation | Operator rm'd marker out-of-band; next `check_and_clear_expired_marker` eval finds absent; dispatcher-native `reconcile_raw_delete` finds `dispatcher-internal-{date}.jsonl` has an unmatched `marker.written` (v1.6 — previously described as unmatched `plugin.indeterminate`) | `marker.cleared` emitted with `clear_mode="OPERATOR_OVERRIDE"`, `actor_type="operator"`, `reason="RAW_DELETE_DETECTED: marker absent without prior marker.cleared event; inferred operator out-of-band clear"` (EC-012; emission point corrected v1.4 — dispatcher-native, not the WASM gate plugin; scan match-type corrected v1.6 — ADR-048 §D4 v1.4, F-P6-001) |
| marker.cleared OPERATOR_OVERRIDE — no FileSink | Operator rm'd marker; FileSink unavailable | No `marker.cleared` emitted; no hard failure (best-effort; EC-013) |
| marker.cleared OPERATOR_OVERRIDE — non-fabrication negative control (v1.6, F-P6-001) | PreToolUse fail-closed INDETERMINATE (no marker write ever attempted, BC-1.18.001 INV4) OR PostToolUse marker-write I/O failure (BC-1.18.001 EC-007); `plugin.indeterminate` emitted but NO `marker.written` ever emitted | `reconcile_raw_delete` finds no unmatched `marker.written` for that `(plugin_name, artifact_path)` pair; NO `marker.cleared(OPERATOR_OVERRIDE)` emitted — no fabricated audit record (EC-017; direct F-P6-001 regression test) |
| marker.cleared SUPERSEDED emission — cross-pair overwrite | Marker exists for pair A (plugin_name="regression-gate", artifact_path="/abs/A.md"); pair B (plugin_name="convergence-tracker", artifact_path="/abs/B.md") — different — goes INDETERMINATE fail-closed | Before B's marker write completes: `marker.cleared` emitted for A with `clear_mode="SUPERSEDED"`, `actor_type="system"`, `trace_id`/`plugin_name`/`artifact_path` = A's own fields, `reason` non-null, via `ctx.emit_internal` (EC-015; BC-1.18.001 INV3 corollary) |
| marker.cleared SUPERSEDED — same-pair re-INDETERMINATE (no emission) | Marker exists for pair A (plugin_name="regression-gate", artifact_path="/abs/A.md"); SAME pair re-INDETERMINATEs before clearance | No `marker.cleared(SUPERSEDED)` emitted; marker overwritten with new trace_id only (EC-016) |

## Related BCs

- BC-1.18.001 — writes the marker; this BC defines how it is cleared (composes with)
- BC-1.18.002 — gate behavior while marker exists and when absent; this BC controls the transition from blocked to unblocked (composes with)
- BC-1.18.004 — fail-open advisory-only behavior; fail-open INDETERMINATE never writes a marker so this BC's clear is never needed for fail-open plugins (sibling)

## Architecture Anchors

- `crates/factory-dispatcher/src/indeterminate_marker.rs` — `delete_marker_if_pass(outcome, policy, dir, current_artifact_path: &str) -> Result<(), io::Error>`; idempotent `fs::remove_file` with `NotFound` swallowed; MUST compare the marker's `artifact_path` field against `current_artifact_path` (exact normalized absolute path equality) before deleting — if marker `artifact_path` is non-empty and does not match `current_artifact_path`, skip deletion (return Ok(())); if marker `artifact_path` is empty, skip the artifact check (vacuously satisfied); `should_write_marker(outcome, policy) -> bool` predicate used by executor to determine whether to write or skip marker. Also contains `emit_marker_cleared(clear_mode: ClearMode, marker: &MarkerContent, trace_id: &str) -> Result<()>` — called from `delete_marker_if_pass` (REVALIDATED), from `check_and_clear_expired_marker` (TTL_EXPIRED), from `reconcile_raw_delete` (OPERATOR_OVERRIDE), and — v1.5 — from `write_indeterminate_marker`'s caller (SUPERSEDED). `ClearMode` is an enum: `Revalidated | TtlExpired | OperatorOverride | Superseded` (v1.5 variant added). **v1.4 addition (ADR-048 §D4 v1.2 — human-ratified):** `check_and_clear_expired_marker(factory_root, now) -> io::Result<Option<MarkerFields>>` — dispatcher-native TTL pre-check: reads the marker via `read_all_marker_fields`; if present and `expires_at ≤ now`, deletes it (idempotent, swallow `NotFound`), emits `marker.cleared(TTL_EXPIRED)`, and returns `Some(fields)`; otherwise returns `None` without deleting. `reconcile_raw_delete(factory_root, ctx: &HostContext) -> io::Result<()>` — dispatcher-native OPERATOR_OVERRIDE reconciliation, invoked from the same pre-check's marker-absent branch: bounded, best-effort scan for an unmatched `plugin.indeterminate`; if found, emits `marker.cleared(OPERATOR_OVERRIDE)` via `emit_marker_cleared`.
  **v1.5 addition — emission-mechanism precision correction (ADR-048 §D4 v1.3 — S-25.01 LOCAL adversary pass 3 F-P3-001 MEDIUM):** `emit_indeterminate` (Event 8, `executor.rs`), `emit_marker_cleared`, `check_and_clear_expired_marker`, and `reconcile_raw_delete` (Event 9, this module) all changed their final emission statement from a raw `InternalLog::write(&ev)` call to `ctx.emit_internal(ev)` (`ctx: &HostContext`) — the SAME dual-sink helper (writes durable `InternalLog` when `ctx.internal_log` is `Some`, AND pushes onto `ctx.events`) that every OTHER dispatcher-native BC-3.08.001 event already uses (`emit_dispatcher_schema_mismatch`, `emit_registry_invalid_e_reg002`/`_e_reg003`, `emit_plugin_abandoned`, `emit_plugin_async_block_discarded`, `emit_plugin_completed_async`, `emit_plugin_timeout_async`, all in `host/emit_event.rs`). Function signatures widen from `(.., log: &InternalLog, session_id: &str)` to `(.., ctx: &HostContext)` — `session_id` is read from `ctx.session_id` internally; `base_ctx`/`base_host_ctx` are already in scope at every `executor.rs` call site, so no new plumbing is introduced. `emit_internal` does NOT re-enrich or overwrite the event's fields (unlike the WASM `emit_event` host ABI's RESERVED_FIELDS wall) — the marker-derived `trace_id`/`plugin_name` set via `.with_trace_id()`/`.with_plugin_name()` before the call are preserved exactly.
  **`reconcile_raw_delete`'s scan-target correction (v1.5, ADR-048 §D4 v1.3):** the bounded tail-scan (`RECONCILE_SCAN_BYTE_CAP`) MUST target `<InternalLog log_dir>/dispatcher-internal-{date}.jsonl` — obtained via `ctx.internal_log.as_ref().map(|l| l.log_dir())`, with `None` short-circuiting to a no-op exactly like `emit_internal`'s own guard — NOT a literal `events-{date}.jsonl` as v1.1–v1.4 prose (and this BC's own PC3 prior to v1.5) named. This is a DELIBERATE divergence from the literal filename, not an error: no default production dispatcher run durably writes a file literally named `events-{date}.jsonl` today — the `sinks::Router`/`FileSink` apparatus (`crates/factory-dispatcher/src/sinks/mod.rs`) that would produce one is not yet wired into `main.rs` (pending S-4.07), and `VSDD_SINK_FILE` is opt-in/diagnostic-only. `InternalLog` (`dispatcher-internal-{date}.jsonl`) is the ONLY log every production dispatch run writes unconditionally — independent of `observability-config.toml`. `HostContext::emit_internal` (the v1.5 fix above) still ALSO reaches `ctx.events`, so once S-4.07 wires the Router/FileSink into `main.rs`, or an operator sets `VSDD_SINK_FILE`, Events 8/9 appear at an `events-*.jsonl`-shaped path too, with no further code change.
  **`reconcile_raw_delete`'s scan match-type correction (v1.6, ADR-048 §D4 v1.4 — S-25.01 adversary pass 6 F-P6-001 MEDIUM):** the scan's match predicate changes from `type == "plugin.indeterminate" && failure_policy == "fail-closed"` to `type == "marker.written"` — `write_indeterminate_marker`'s caller now emits `marker.written` (via the new `emit_marker_written(ctx, &fields)` function in `indeterminate_marker.rs`, BC-1.18.001 PC4 v1.4) ONLY after the write returns `Ok(())`, so an unmatched `marker.written` record is proof-by-construction (not inference) that a marker existed and was cleared by something other than the three already-audited paths. The `failure_policy` filter on the old match arm is REMOVED as structurally redundant against `marker.written`, which is never emitted for a fail-open dispatch or a PreToolUse dispatch (no marker write occurs in either case, BC-1.18.001 INV4). `emit_marker_written`'s fields (`trace_id`, `plugin_name`, `artifact_path`, `cause`, `expires_at`) let `reconcile_raw_delete` reconstruct the full `MarkerFields` needed for the retroactive `marker.cleared(OPERATOR_OVERRIDE)` emission directly from the matched scan record — see BC-3.08.001 Event 10 wire format.
  **v1.5 addition — SUPERSEDED trigger (F-P3-002 LOW):** `write_indeterminate_marker`'s caller (`executor.rs`, the marker-write callsite for INDETERMINATE outcomes) reads the EXISTING marker's fields (if any) BEFORE the temp+rename overwrite; if the existing marker's `(plugin_name, artifact_path)` differs from the new event's, it calls `emit_marker_cleared(ctx, ..., ClearMode::Superseded, "system", Some(reason))` — symmetrical with the read-before-delete `delete_marker_if_pass`'s REVALIDATED path.
- `crates/factory-dispatcher/src/executor.rs` — calls `delete_marker_if_pass` after a PostToolUse PASS on a fail-closed plugin; the clear MUST be gated on (a) plugin_name match AND (b) artifact-path match (exact normalized absolute path equality, with empty-artifact_path fallback to name-only); BOTH callsites in the executor must apply these two conditions — missing the artifact check at either callsite causes premature quarantine discharge; same module as the INDETERMINATE classification in BC-1.18.001. **v1.4 addition:** the tier-execution loop calls `check_and_clear_expired_marker` for every `on_error == OnError::BlockIfMarker` registry entry BEFORE invoking the Arm 1/Arm 2 WASM gate plugin on the normal (non-crash) path. **v1.5 addition:** the marker-write callsite for INDETERMINATE fail-closed outcomes (feeding `write_indeterminate_marker`) reads the pre-overwrite marker fields and triggers the SUPERSEDED emission described above; `HostContext` (`base_ctx`) is already in scope at this callsite and at the `emit_indeterminate`/`emit_marker_cleared`/`check_and_clear_expired_marker`/`reconcile_raw_delete` call sites, so the `ctx: &HostContext` signature widening requires no new plumbing.
- `crates/hook-plugins/validate-unvalidated-mutation-marker/src/lib.rs` — **v1.4 correction (ADR-048 §D4 v1.2 — human-ratified):** `evaluate_gate` is a PURE PRESENCE CHECK, no TTL/emission logic. The v1.0–v1.3 attribution of a TTL-expiry clear branch and a RAW_DELETE_DETECTED reconciliation branch to this plugin crate is superseded — both were moved entirely to dispatcher-native code (`check_and_clear_expired_marker` and `reconcile_raw_delete` in `indeterminate_marker.rs`, see above) because the `emit_event` host ABI's RESERVED_FIELDS enrichment makes it structurally impossible for a WASM plugin to emit `marker.cleared` carrying a foreign `trace_id`/`plugin_name` (the marker's own identity, not the invoking plugin's). `evaluate_gate` now: reads marker presence only (no `expires_at` parsing); if the marker is present, returns Block (exit_code=2); if absent, returns Allow (exit_code=0). No `fs::remove_file`, no `emit_marker_cleared` call, and no FileSink scan occur inside this crate.

## Story Anchor

S-25.01 — Dispatcher INDETERMINATE Outcome Layer 1: Fail-Loud on Cannot-Complete — durable marker + next-advance gate

## VP Anchors

- VP-106 — Successful Re-Validation Deletes Marker (artifact-scoped: same plugin + same non-empty artifact OR empty-artifact_path fallback); TTL-expiry clear DECISION (dispatcher-native `check_and_clear_expired_marker` returns Some/None + auto-deletes); fail-open INDETERMINATE Writes No Marker (unit-test; covers PC1 artifact-scoped clear, PC2 idempotent delete, PC4 TTL-expiry clear decision, Invariant 1 FAIL-preserves, Invariant 2 artifact-scoped + empty-path exception, Invariant 4, Invariant 5 marker-level TTL, EC-008/EC-009)
- VP-108 — marker.cleared/marker.written Audited-Event Emission Correctness (unit-test; covers the five audited-emission postconditions — PC1 REVALIDATED, PC4 TTL_EXPIRED, PC3 OPERATOR_OVERRIDE, PC5 SUPERSEDED (v1.5) emissions, and PC4/EC-014 crash-path non-emission — all attributed dispatcher-native and emitted via `HostContext::emit_internal` per ADR-048 §D4 v1.3; **v1.4 retarget:** these rows moved here from VP-106 because VP-108 is the emission-correctness VP; VP-106 retains only the clear-decision (Some/None + file-state) properties; **v1.5 addition:** PC5 SUPERSEDED row added, and PC1–PC4 rows' emission mechanism corrected from a raw `InternalLog::write` to `ctx.emit_internal`, per ADR-048 §D4 v1.3 F-P3-001; **v1.6 addition (ADR-048 §D4 v1.4 — S-25.01 adversary pass 6 F-P6-001 MEDIUM):** this BC's PC3 OPERATOR_OVERRIDE row retargeted — VP-108's PC3 fixture now seeds an unmatched `marker.written` (BC-3.08.001 Event 10), not an unmatched `plugin.indeterminate`; VP-108 also gains PC6 (this BC's sibling BC-1.18.001 PC4 v1.4 `marker.written` write-path emission correctness) and PC7 (negative control proving `reconcile_raw_delete` does NOT fabricate OPERATOR_OVERRIDE when no `marker.written` was ever emitted — the direct regression test for F-P6-001, corresponding to EC-017 below))

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-106 | PASS re-validation (same plugin, same non-empty artifact) deletes `.factory/unvalidated-mutation.marker`; deletion is idempotent (no error when marker already absent) | unit-test |
| VP-106 | FAIL re-validation does NOT delete marker; marker persists until PASS or manual deletion | unit-test |
| VP-106 | `should_write_marker(Indeterminate, FailOpen) == false`; `should_write_marker(Indeterminate, FailClosed) == true` | unit-test |
| VP-106 | PASS re-validation (same plugin, DIFFERENT non-empty artifact) does NOT delete marker; marker persists — artifact A quarantine unresolved (EC-008) | unit-test |
| VP-106 | PASS re-validation (same plugin, empty marker `artifact_path`) DOES delete marker — empty-path fallback, plugin_name equality alone suffices (EC-009) | unit-test |
| VP-106 | TTL-expiry clear (PC4): dispatcher-native `check_and_clear_expired_marker` reads expired `expires_at` (≤ now UTC) on normal path → returns `Some(fields)` + auto-delete marker (idempotent; NotFound swallowed), signaling caller to Allow (exit_code=0); INV5 TTL is marker-level (no plugin_name/artifact_path check); non-expired marker → `None`, `evaluate_gate` then Blocks; legacy marker (no `expires_at`) treated as non-expired. **v1.4:** retargeted from `evaluate_gate` to `check_and_clear_expired_marker` per ADR-048 §D4 v1.2 | unit-test |
| VP-108 | REVALIDATED clear emits `marker.cleared(clear_mode=REVALIDATED, actor_type=validator, trace_id=originating-indeterminate-trace-id, reason=null)` after `delete_marker_if_pass` succeeds (PC1 audited clear; EC-010) | unit-test |
| VP-108 | TTL_EXPIRED clear emits `marker.cleared(clear_mode=TTL_EXPIRED, actor_type=deadman, reason=null)` after the dispatcher-native `check_and_clear_expired_marker` pre-check auto-deletes on normal-path eval (PC4 TTL-loudness; EC-011). **v1.4:** retargeted from VP-106/`evaluate_gate` to VP-108/`check_and_clear_expired_marker` per ADR-048 §D4 v1.2 | unit-test |
| VP-108 | OPERATOR_OVERRIDE reconciliation: dispatcher-native `reconcile_raw_delete` finds marker absent + unmatched `marker.written` (BC-3.08.001 Event 10) in `dispatcher-internal-{date}.jsonl` → emits `marker.cleared(clear_mode=OPERATOR_OVERRIDE, actor_type=operator, reason=RAW_DELETE_DETECTED:...)` best-effort; no hard failure if log unavailable (PC3 audited clear; EC-012/EC-013). **v1.4:** retargeted from VP-106/gate-plugin to VP-108/`reconcile_raw_delete` per ADR-048 §D4 v1.2. **v1.6:** scan match-type retargeted from unmatched `plugin.indeterminate` to unmatched `marker.written` per ADR-048 §D4 v1.4 (S-25.01 adversary pass 6 F-P6-001) | unit-test |
| VP-108 | Negative control (PC7, v1.6, ADR-048 §D4 v1.4 — F-P6-001 direct regression test): a PreToolUse fail-closed INDETERMINATE (no marker write ever attempted, BC-1.18.001 INV4) or a PostToolUse marker-write I/O failure (BC-1.18.001 EC-007) — neither of which ever emits `marker.written` — followed by `reconcile_raw_delete` does NOT fabricate `marker.cleared(OPERATOR_OVERRIDE)` for that `(plugin_name, artifact_path)` pair (EC-017) | unit-test |
| VP-108 | Crash-path TTL allow (dispatcher native check, `block_if_marker_check`) does NOT emit `marker.cleared(TTL_EXPIRED)` — only the dispatcher-native normal-path pre-check (`check_and_clear_expired_marker`) emits it (EC-014). **v1.4:** retargeted from VP-106 to VP-108 (emission-correctness scope) per ADR-048 §D4 v1.2 | unit-test |
| VP-108 | SUPERSEDED clear (PC5, v1.5): `write_indeterminate_marker`'s caller, given an existing marker for a DIFFERENT `(plugin_name, artifact_path)` pair, reads the existing marker's fields before overwrite and emits `marker.cleared(clear_mode=SUPERSEDED, actor_type=system, reason=non-null)` via `ctx.emit_internal` carrying the SUPERSEDED pair's own trace_id/plugin_name/artifact_path (not the new pair's), BEFORE the new marker write completes (EC-015); same-pair re-INDETERMINATE emits nothing (EC-016). ADR-048 §D4 v1.3 F-P3-002 | unit-test |
| VP-108 | Emission-mechanism precision (v1.5, ADR-048 §D4 v1.3 F-P3-001): ALL FOUR `clear_mode` emissions (REVALIDATED, TTL_EXPIRED, OPERATOR_OVERRIDE, SUPERSEDED) go via `ctx.emit_internal` (`ctx: &HostContext`), never a raw `InternalLog::write`; `reconcile_raw_delete`'s scan target is `dispatcher-internal-{date}.jsonl` (via `InternalLog`), not a literal `events-{date}.jsonl` | unit-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-041 |
| Capability Anchor Justification | CAP-041 ("Validation Integrity: INDETERMINATE Outcome, Durable Mutation Marker, and Next-Advance Gate") per capabilities.md §CAP-041 — this BC specifies the marker-clear lifecycle that completes the quarantine loop defined in CAP-041: "The marker is cleared by successful re-validation (same plugin, PASS outcome) or manual operator deletion (`rm .factory/unvalidated-mutation.marker`)." |
| L2 Domain Invariants | none (dispatcher runtime marker lifecycle invariant, not L2 domain spec) |
| Architecture Module | SS-01 (Hook Dispatcher Core — `delete_marker_if_pass` and `should_write_marker` in `indeterminate_marker.rs`; `executor.rs` clear call-site) |
| ADR | ADR-047 §Decision 5 (Marker Clear Protocol — Condition A successful re-validation; Condition B manual operator escape hatch; clear-on-PASS not clear-on-FAIL; idempotent delete; named-plugin AND artifact-scoped clear; empty-artifact_path falls back to name-only); ADR-047 §Decision 9 (both gate arms unblocked simultaneously by single marker deletion); ADR-048 §Decision 2 (TTL-expiry as third clear path — checked by the dispatcher-native `check_and_clear_expired_marker` pre-check on the normal path per §D4 v1.2; expired marker treated as absent → Allow + auto-delete; missing expires_at on legacy markers treated as non-expired; TTL is marker-level not artifact-scoped; UNVALIDATED_MUTATION_MARKER_TTL_SECONDS = 86_400); ADR-048 §Decision 4 (marker.cleared audited event — REVALIDATED emitted from delete_marker_if_pass; TTL_EXPIRED emitted from the dispatcher-native `check_and_clear_expired_marker` pre-check (TTL-loudness: previously silent); OPERATOR_OVERRIDE reconciled retroactively via the dispatcher-native `reconcile_raw_delete`'s RAW_DELETE_DETECTED scan when marker absent + unmatched plugin.indeterminate found; best-effort if FileSink unavailable; no signed digests/dual-control per cooperating-agent threat model; same FileSink/InternalLog emission path as plugin.indeterminate); **ADR-048 §Decision 4 v1.2 Emission-Point Correction** (S-25.01 LOCAL adversary pass 2 F-P2-002 HIGH + F-P2-003 MED; human-ratified — v1.4) — TTL_EXPIRED and OPERATOR_OVERRIDE `marker.cleared` emission moved entirely dispatcher-native (`check_and_clear_expired_marker` + `reconcile_raw_delete`, both new functions in `indeterminate_marker.rs`); the WASM gate plugin's `evaluate_gate` is simplified to a pure marker-presence check with no TTL parsing, deletion, or emission logic — the `emit_event` host ABI's RESERVED_FIELDS enrichment makes plugin-side emission of a foreign trace_id/plugin_name structurally impossible; **ADR-048 §Decision 4 v1.3 Emission-Mechanism Precision Correction + SUPERSEDED Clear Mode** (S-25.01 LOCAL adversary pass 3 F-P3-001 MEDIUM + F-P3-002 LOW; human-ratified — v1.5) — (a) ALL FOUR `clear_mode` emissions (REVALIDATED/TTL_EXPIRED/OPERATOR_OVERRIDE/SUPERSEDED) route through `HostContext::emit_internal` rather than a raw `InternalLog::write`, matching the pattern every other dispatcher-native BC-3.08.001 event already uses; (b) `reconcile_raw_delete`'s bounded scan target is corrected to `dispatcher-internal-{date}.jsonl` (via the durable `InternalLog`), not a literal `events-{date}.jsonl` — the observable FileSink/Router apparatus remains unwired pending S-4.07; (c) new PC5 `clear_mode = "SUPERSEDED"` / `actor_type = "system"`: `write_indeterminate_marker`'s caller reads the existing marker's fields before a cross-pair (last-writer-wins) overwrite and emits a retroactive-audit-preventing `marker.cleared(SUPERSEDED)` for the superseded pair, closing the false-`OPERATOR_OVERRIDE`-attribution gap in `reconcile_raw_delete`; **ADR-048 §Decision 4 v1.4 Reconciliation-Premise Correction** (S-25.01 adversary pass 6 F-P6-001 MEDIUM; architect adjudication — v1.6; HUMAN-RATIFIED 2026-09-01, POLICY 22, D-1141) — PC3's `reconcile_raw_delete` scan retargeted from unmatched `plugin.indeterminate` (`failure_policy=fail-closed`) to unmatched `marker.written` (BC-3.08.001 Event 10, new — BC-1.18.001 PC4 v1.4): the pre-v1.6 premise that an unmatched fail-closed `plugin.indeterminate` proves a marker was durably written is false whenever a PreToolUse fail-closed INDETERMINATE never attempts a write (BC-1.18.001 INV4) or a PostToolUse marker-write I/O failure (BC-1.18.001 EC-007) leaves the same no-marker-ever-existed footprint; both would fabricate `marker.cleared(OPERATOR_OVERRIDE)` under the pre-v1.6 scan; the `failure_policy` filter is removed as structurally redundant against `marker.written`, which is emitted iff a marker was actually written; new EC-017 is the direct F-P6-001 regression test |
| Stories | S-25.01 |
| Cycle | v1.0-feature-validation-integrity-layer1 (F2 — product-owner spec burst) |
| Feature | E-25 — Validation Integrity and Large-Artifact Resilience |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.6 | 2026-09-01 | product-owner | ADR-048 §Decision 4 v1.4 Reconciliation-Premise Correction (S-25.01 adversary pass 6 F-P6-001 MEDIUM; architect adjudication; HUMAN-RATIFIED 2026-09-01, POLICY 22, D-1141). `reconcile_raw_delete`'s RAW_DELETE_DETECTED scan (PC3, OPERATOR_OVERRIDE) is retargeted from matching an unmatched `plugin.indeterminate` (`failure_policy="fail-closed"`) to matching an unmatched `marker.written` (BC-3.08.001 Event 10, new sibling BC-1.18.001 PC4 v1.4). Root cause: `plugin.indeterminate` fires for EVERY INDETERMINATE outcome regardless of hook phase, while a marker write is conditionally gated (PostToolUse AND fail-closed AND the atomic write actually succeeding) — the pre-v1.6 premise conflated the two, so a PreToolUse fail-closed INDETERMINATE (BC-1.18.001 INV4 — never attempts a marker write) or a PostToolUse marker-write I/O failure (BC-1.18.001 EC-007) both leave an unmatched `plugin.indeterminate` with no marker ever written, causing `reconcile_raw_delete` to fabricate `marker.cleared(OPERATOR_OVERRIDE)` — a false NIST AU-3/AU-10 audit record attributing a human out-of-band action that never happened. `marker.written` is emitted ONLY after a confirmed successful write (BC-1.18.001 PC4 v1.4), making the reconciliation premise sound by construction. The `failure_policy=="fail-closed"` filter on the old match arm is REMOVED as structurally redundant. (1) Description gains the reconciliation-premise-correction paragraph. (2) PC3 rewritten: scan match-type correction clause added; `trace_id`/`plugin_name`/`artifact_path` sourcing corrected to the unmatched `marker.written` event; best-effort-gap monitoring sentence updated. (3) EC-012/EC-013 retargeted to `marker.written`. (4) New EC-017 added: PC3 negative control — the direct F-P6-001 regression test proving no fabrication occurs when no marker was ever written. (5) Canonical Test Vectors: OPERATOR_OVERRIDE row retargeted; new non-fabrication negative-control row added. (6) Architecture Anchors: `indeterminate_marker.rs` bullet gains the v1.6 scan-match-type-correction paragraph, naming the new `emit_marker_written` function this BC's scan now depends on. (7) VP Anchors + Verification Properties: VP-108 bullet renamed/expanded; PC3 row retargeted; new VP-108 PC7 negative-control row added. (8) Traceability ADR row: ADR-048 §Decision 4 v1.4 citation added. No wire-format/field-shape change to `marker.cleared` itself — this is a reconciliation-premise/input-source correction, not a new `clear_mode` value. `marker.written`'s own wire format is carried by the sibling BC-3.08.001 Event 10 catalog entry and BC-1.18.001 v1.4's PC4 addition. |
| 1.5 | 2026-09-01 | product-owner | ADR-048 §Decision 4 v1.3 Emission-Mechanism Precision Correction (F-P3-001 MEDIUM) + SUPERSEDED Clear Mode (F-P3-002 LOW) (S-25.01 LOCAL adversary pass 3; human-ratified 2026-09-01, POLICY 22, D-1140, per ADR-048 v1.3 Status). Two independent corrections. (A) Emission-mechanism: the frozen implementation emitted `plugin.indeterminate` (Event 8) and `marker.cleared` (Event 9, all three pre-v1.5 clear_modes) via a raw `InternalLog::write` call instead of the established `HostContext::emit_internal` dual-sink helper every sibling BC-3.08.001 dispatcher-native event uses — corrected. `reconcile_raw_delete`'s scan target corrected from the literal `events-{date}.jsonl` this BC previously implied to `dispatcher-internal-{date}.jsonl` (via `InternalLog`) — the only log every default production dispatcher run durably writes; the FileSink/Router apparatus remains unwired pending S-4.07. (B) New PC5 `SUPERSEDED` clear path (parallel to PC1/PC3/PC4): under BC-1.18.001 INV3 single-marker last-writer-wins, a cross-pair marker overwrite (marker for pair A overwritten by a NEW INDETERMINATE for a DIFFERENT pair B before A was cleared) now emits `marker.cleared(clear_mode=SUPERSEDED, actor_type=system, reason=non-null)` for A's own fields BEFORE B's write completes — closing a false-`OPERATOR_OVERRIDE`-attribution audit-integrity gap (NIST AU-3/AU-10) that `reconcile_raw_delete` would otherwise produce if B's marker were later raw-deleted. Same-pair re-INDETERMINATE emits nothing (no change to that case). (1) Description updated: three clear paths → four; emission-mechanism correction stated. (2) PC3 gains a scan-target-correction clause; "FileSink log unavailable" reworded to name `dispatcher-internal-{date}.jsonl`/`ctx.internal_log`. (3) New PC5 added (SUPERSEDED — full postcondition: trigger, fields, same-pair exception, reconciliation-gap closure). (4) EC-015 (cross-pair SUPERSEDED emission) and EC-016 (same-pair no-emission) added. (5) Canonical Test Vectors: two new SUPERSEDED rows added. (6) Architecture Anchors: `indeterminate_marker.rs` bullet gains the v1.5 emission-mechanism paragraph (signature widening to `ctx: &HostContext`), the scan-target-correction paragraph, and the SUPERSEDED-trigger paragraph; `executor.rs` bullet gains the v1.5 marker-write-callsite note. (7) VP Anchors + Verification Properties: VP-108 gains a PC5 SUPERSEDED row and an emission-mechanism-precision row. (8) Traceability ADR row: ADR-048 §Decision 4 v1.3 citation added. No wire-format/field-shape change to `marker.cleared`'s existing REVALIDATED/TTL_EXPIRED/OPERATOR_OVERRIDE fields — `SUPERSEDED` is a new VALUE of the existing `clear_mode` enum (BC-3.08.001 v1.32 carries the sibling wire-format enum + sink-destination-prose corrections). |
| 1.4 | 2026-08-31 | product-owner | ADR-048 §Decision 4 v1.2 Emission-Point Correction (S-25.01 LOCAL adversary pass 2 F-P2-002 HIGH + F-P2-003 MED; human-ratified). TTL_EXPIRED and OPERATOR_OVERRIDE `marker.cleared` emission re-attributed from the WASM gate plugin (`evaluate_gate`) to two new dispatcher-native functions in `indeterminate_marker.rs`: `check_and_clear_expired_marker` (TTL detection + auto-delete + emission, called from `executor.rs`'s tier-execution loop before the Arm 1/Arm 2 WASM gate plugin runs) and `reconcile_raw_delete` (OPERATOR_OVERRIDE reconciliation, same pre-check's marker-absent branch). Root cause: the `emit_event` host ABI's RESERVED_FIELDS enrichment unconditionally overwrites plugin-supplied `trace_id`/`plugin_name` with the invoking plugin's own dispatch identity, making it structurally impossible for `evaluate_gate` to emit `marker.cleared` carrying the marker's own (foreign) trace_id/plugin_name. `evaluate_gate` is simplified to a pure marker-presence check with no TTL parsing, deletion, or emission logic. (1) Description tertiary-path paragraph corrected. (2) PC3 and PC4 emission-point clauses rewritten. (3) EC-011/EC-012/EC-013/EC-014 wording corrected. (4) Canonical Test Vectors: TTL-expiry and marker.cleared TTL_EXPIRED/OPERATOR_OVERRIDE rows corrected. (5) Architecture Anchors: `indeterminate_marker.rs` bullet gains `check_and_clear_expired_marker` + `reconcile_raw_delete`; `validate-unvalidated-mutation-marker/src/lib.rs` bullet rewritten — TTL/RAW_DELETE_DETECTED logic REMOVED, replaced with pure-presence-check description. (6) Traceability ADR row: ADR-048 §Decision 4 v1.2 Emission-Point Correction citation added. (7) VP attribution: the four audited-emission postconditions (REVALIDATED, TTL_EXPIRED, OPERATOR_OVERRIDE, crash-path non-emission) retargeted from VP-106 to VP-108 in both §VP Anchors and §Verification Properties; VP-106 retains only the clear-decision (Some/None + file-state) properties, consistent with VP-108 v1.1 and VP-106 v1.5 (architect-side companion corrections in the same burst). No wire-format/field-shape change to `marker.cleared` itself (BC-3.08.001 Event 9 unaffected in shape; BC-3.08.001 v1.31 carries the sibling emission-locus correction). |
| 1.3 | 2026-08-31 | product-owner | ADR-048 §Decision 4 — audited clear model. (1) Description updated: all three clear paths now emit `marker.cleared` (BC-3.08.001 Event 9). (2) PC1 audited: `delete_marker_if_pass` now calls `emit_marker_cleared(REVALIDATED)` after successful delete; `trace_id` from originating `plugin.indeterminate`; `actor_type=validator`; `reason=null`. (3) PC3 audited: retroactive `marker.cleared(OPERATOR_OVERRIDE)` via RAW_DELETE_DETECTED reconciliation on next gate eval when marker absent + unmatched `plugin.indeterminate` found; best-effort (no hard failure if FileSink unavailable). (4) PC4 TTL-loudness: TTL auto-delete now emits `marker.cleared(TTL_EXPIRED)` with `actor_type=deadman`; previously silent auto-delete is now audited. Note: crash-path native TTL allow (dispatcher Decision 1) does NOT emit — only gate plugin normal-path auto-delete emits. (5) EC-010–014 added (three clear_mode emissions + FileSink-unavailable case + crash-path exclusion). (6) Canonical test vectors: three new rows for REVALIDATED/TTL_EXPIRED/OPERATOR_OVERRIDE emissions; prior TTL rows updated to note `marker.cleared(TTL_EXPIRED)` emission. (7) VP-106: four new property rows for audited emissions. (8) Architecture Anchors: `emit_marker_cleared` function added; `validate-unvalidated-mutation-marker` plugin crate anchored for TTL_EXPIRED + OPERATOR_OVERRIDE reconciliation. (9) Traceability ADR: ADR-048 §Decision 4 added. |
| 1.2 | 2026-08-31 | product-owner | ADR-048 §Decision 2 — adds TTL-expiry as a third clear path. (1) Description updated: two clear paths → three clear paths. (2) Added PC4: gate plugin reads expired `expires_at` on normal path → treat marker as ABSENT, Allow (exit_code=0), auto-delete (idempotent; swallow NotFound). (3) Added INV5: TTL is marker-level expiry (not artifact-scoped) — any marker expires 86400s after `expires_at` regardless of which artifact triggered INDETERMINATE; confirmed consistent with PC1 artifact-scoped clear and BC-1.18.001 INV3 single-marker. (4) Added canonical test vectors: expired-marker→Allow+auto-delete; non-expired-marker→Block (existing behavior confirmed). (5) Traceability ADR: ADR-048 §Decision 2 citation added. ADR-048 added to inputs. |
| 1.1 | 2026-08-31 | product-owner | S-25.01 adversary M-1 resolution. Promote artifact-scoped clear to authoritative predicate throughout BC. PC1 rewritten: clear requires (a) plugin_name match AND (b) artifact_path match (exact normalized absolute path equality), with explicit empty-artifact_path fallback (vacuously satisfied → name-only suffices for non-artifact-scoped validators). INV2 rewritten to match PC1. Architecture Anchors updated: `delete_marker_if_pass` signature gains `current_artifact_path` param; executor.rs note flags BOTH callsites must apply the two-condition gate. Added EC-008 (same plugin, different non-empty artifact → no clear) and EC-009 (empty marker artifact_path → clear via fallback). Added two canonical test vectors for EC-008/EC-009. VP-106 Anchors and Verification Properties table expanded with corresponding unit-test rows. ADR-047 §D5 citation updated to name artifact-scoped clear + empty-path fallback. Traceability ADR field updated to match. |
| 1.0 | 2026-08-30 | product-owner | Initial creation. F2 spec-evolution burst, validation-integrity-layer1. BC-1.18.003: marker-clear protocol (PASS-clears/FAIL-preserves/idempotent/operator-rm), named-plugin scoping invariant, both-arms-unblock invariant. VP-106 anchored. CAP-041 capability anchor. ADR-047 §D5 citations. |
