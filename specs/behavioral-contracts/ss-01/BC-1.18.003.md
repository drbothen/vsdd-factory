---
document_type: behavioral-contract
level: L3
version: "1.3"
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
input-hash: "815a46e"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-041"
lifecycle_status: draft
introduced: v1.0-feature-validation-integrity-layer1
modified: ["2026-08-31", "2026-08-31-v1.2-ttl-third-clear-path", "2026-08-31-v1.3-audited-clear-model-marker-cleared-event"]
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-1.18.003: Successful Re-Validation Clears the Unvalidated-Mutation Marker (Idempotent; Operator Escape Hatch Supported)

## Description

The `.factory/unvalidated-mutation.marker` lifecycle has three supported clear paths. Primary
path: when the same plugin that produced INDETERMINATE is subsequently dispatched PostToolUse on
the same artifact and produces `DispatchOutcome::Pass` (exit_code=0, `host_output_too_large_seen=false`),
the dispatcher deletes the marker file and emits `marker.cleared(REVALIDATED)`. Secondary path:
the operator manually deletes the marker via `rm .factory/unvalidated-mutation.marker` — a fully
supported and documented escape hatch requiring no special command or credential; a retroactive
`marker.cleared(OPERATOR_OVERRIDE)` event is reconciled via RAW_DELETE_DETECTED on the next gate
evaluation. Tertiary path (TTL deadman): when the gate plugin reads the marker on the normal
execution path and finds `expires_at` elapsed (UTC), it treats the marker as ABSENT, allows the
dispatch, auto-deletes the marker file (idempotent; swallow NotFound), and emits
`marker.cleared(TTL_EXPIRED)` (TTL-loudness: previously silent auto-delete is now audited per
ADR-048 §Decision 4). All three clear paths unblock the two gate arms simultaneously and emit a
`marker.cleared` audited event (BC-3.08.001 Event 9) with `clear_mode` ∈
{REVALIDATED|TTL_EXPIRED|OPERATOR_OVERRIDE}. The clear operation is idempotent: if the marker is
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
   when the gate plugin evaluates and finds the marker absent, and the FileSink log contains an
   unmatched `plugin.indeterminate` for the same `(plugin_name, artifact_path)` with no
   corresponding `marker.cleared`, the plugin emits a retroactive `marker.cleared` (BC-3.08.001
   Event 9) with: `clear_mode = "OPERATOR_OVERRIDE"`, `actor_type = "operator"`,
   `reason = "RAW_DELETE_DETECTED: marker absent without prior marker.cleared event; inferred operator out-of-band clear"`,
   `timestamp` = current evaluation time (deletion time is unobservable), `trace_id` = trace_id
   from the unmatched `plugin.indeterminate` event. **Best-effort:** if the FileSink log is
   unavailable or the unmatched record cannot be found, the annotation is omitted — no hard
   failure. An unreconciled gap is observable by tooling that monitors the event stream for
   `plugin.indeterminate` events without subsequent `marker.cleared`.

4. **TTL-expiry clear path (marker-level deadman) with `marker.cleared(TTL_EXPIRED)` audit emission.**
   When the gate plugin reads `.factory/unvalidated-mutation.marker` during normal (non-crash) gate
   evaluation and finds `expires_at` elapsed (`expires_at ≤ now (UTC)`), the gate treats the marker
   as ABSENT: returns `exit_code = 0` (Allow) and auto-deletes the marker file (idempotent; swallow
   `NotFound`). The auto-delete prevents accumulation of dead marker artifacts. This is the
   THIRD clear path alongside PC1 (successful re-validation) and PC3 (operator manual rm).
   **TTL-loudness (ADR-048 §Decision 4):** immediately after the TTL auto-delete, the gate plugin
   MUST emit `marker.cleared` (BC-3.08.001 Event 9) with: `clear_mode = "TTL_EXPIRED"`,
   `actor_type = "deadman"`, `trace_id` from marker TOML `trace_id` field, `plugin_name` from
   marker TOML, `artifact_path` from marker TOML, `reason = null`, `timestamp` = time of the
   clear event. Emitted from the plugin's TTL-check branch, immediately after auto-delete. This
   replaces the prior SILENT TTL auto-delete. Note: the dispatcher's NATIVE crash-path TTL check
   (Decision 1 — crash + marker + expired → Allow) does NOT emit `marker.cleared(TTL_EXPIRED)` —
   only the gate plugin's normal-path auto-delete emits it. The marker remains for the next normal
   plugin execution to clear and emit the audited event.
   **`expires_at` stamping:** the `expires_at` field is written by `write_indeterminate_marker`
   at marker creation time as `timestamp + UNVALIDATED_MUTATION_MARKER_TTL_SECONDS` (86400s per
   ADR-048 §Decision 2; BC-1.18.001 PC4). Backward compatibility: markers written before
   ADR-048 implementation lack `expires_at`. The gate plugin MUST treat a missing `expires_at`
   field as non-expired (conservative; no silent auto-clear of old markers). Such markers remain
   in effect until explicitly cleared via rm (PC3) or until ADR-048 is implemented and new
   markers with `expires_at` replace them.

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
| EC-011 | PC4 path — marker cleared via TTL expiry (normal plugin eval, `expires_at` ≤ now UTC) | `marker.cleared` event emitted with `clear_mode="TTL_EXPIRED"`, `actor_type="deadman"`, `trace_id` from marker TOML, `reason=null`. TTL-loudness: previously silent auto-delete is now audited. |
| EC-012 | PC3 path — marker cleared via operator out-of-band rm; FileSink log available with unmatched `plugin.indeterminate` | Gate plugin finds marker absent on next evaluation; emits retroactive `marker.cleared` with `clear_mode="OPERATOR_OVERRIDE"`, `actor_type="operator"`, `reason="RAW_DELETE_DETECTED: marker absent without prior marker.cleared event; inferred operator out-of-band clear"`, `timestamp`=current eval time, `trace_id` from unmatched `plugin.indeterminate`. |
| EC-013 | PC3 path — operator out-of-band rm; FileSink log unavailable | No `marker.cleared` emitted — best-effort; no hard failure. Gap is observable by monitoring for unmatched `plugin.indeterminate` events in the event stream. |
| EC-014 | Crash-path TTL allow (dispatcher native check, Decision 1) — expires_at ≤ now UTC | No `marker.cleared(TTL_EXPIRED)` emitted — only the gate plugin's normal-path auto-delete emits it. Marker remains until next normal plugin execution clears it with the audited event. |

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
| TTL-expiry clear — expired marker | Marker exists with `expires_at` in the PAST (UTC); gate plugin reads marker on normal path | Allow (exit_code=0); marker auto-deleted (idempotent; NotFound swallowed); both gate arms unblocked (PC4); no plugin_name or artifact_path check required; `marker.cleared(TTL_EXPIRED)` emitted (EC-011) |
| TTL-expiry — non-expired marker | Marker exists with `expires_at` in the FUTURE (UTC); gate plugin reads marker on normal path | Block (exit_code=2); existing behavior unchanged; marker remains; no `marker.cleared` emitted |
| TTL-expiry — marker missing expires_at (legacy pre-ADR-048) | Marker exists without `expires_at` field; gate plugin reads on normal path | Block (exit_code=2) — marker treated as non-expired (conservative backward-compat; PC4 note); marker NOT auto-deleted; no `marker.cleared` emitted |
| marker.cleared REVALIDATED emission | Marker exists; same plugin PASS on same artifact | After `delete_marker_if_pass` succeeds: `marker.cleared` emitted with `clear_mode="REVALIDATED"`, `actor_type="validator"`, `trace_id` from `plugin.indeterminate`, `reason=null` (EC-010) |
| marker.cleared TTL_EXPIRED emission | Marker expired; gate plugin normal-path eval | After TTL auto-delete: `marker.cleared` emitted with `clear_mode="TTL_EXPIRED"`, `actor_type="deadman"`, `reason=null` (EC-011) |
| marker.cleared OPERATOR_OVERRIDE reconciliation | Operator rm'd marker out-of-band; next gate eval finds absent; FileSink has unmatched `plugin.indeterminate` | `marker.cleared` emitted with `clear_mode="OPERATOR_OVERRIDE"`, `actor_type="operator"`, `reason="RAW_DELETE_DETECTED: marker absent without prior marker.cleared event; inferred operator out-of-band clear"` (EC-012) |
| marker.cleared OPERATOR_OVERRIDE — no FileSink | Operator rm'd marker; FileSink unavailable | No `marker.cleared` emitted; no hard failure (best-effort; EC-013) |

## Related BCs

- BC-1.18.001 — writes the marker; this BC defines how it is cleared (composes with)
- BC-1.18.002 — gate behavior while marker exists and when absent; this BC controls the transition from blocked to unblocked (composes with)
- BC-1.18.004 — fail-open advisory-only behavior; fail-open INDETERMINATE never writes a marker so this BC's clear is never needed for fail-open plugins (sibling)

## Architecture Anchors

- `crates/factory-dispatcher/src/indeterminate_marker.rs` — `delete_marker_if_pass(outcome, policy, dir, current_artifact_path: &str) -> Result<(), io::Error>`; idempotent `fs::remove_file` with `NotFound` swallowed; MUST compare the marker's `artifact_path` field against `current_artifact_path` (exact normalized absolute path equality) before deleting — if marker `artifact_path` is non-empty and does not match `current_artifact_path`, skip deletion (return Ok(())); if marker `artifact_path` is empty, skip the artifact check (vacuously satisfied); `should_write_marker(outcome, policy) -> bool` predicate used by executor to determine whether to write or skip marker. Also contains `emit_marker_cleared(clear_mode: ClearMode, marker: &MarkerContent, trace_id: &str) -> Result<()>` — called from `delete_marker_if_pass` (REVALIDATED) and from the gate plugin's TTL-check branch (TTL_EXPIRED). `ClearMode` is an enum: `Revalidated | TtlExpired | OperatorOverride`. OPERATOR_OVERRIDE reconciliation is invoked from the gate plugin's absent-marker evaluation path (RAW_DELETE_DETECTED).
- `crates/factory-dispatcher/src/executor.rs` — calls `delete_marker_if_pass` after a PostToolUse PASS on a fail-closed plugin; the clear MUST be gated on (a) plugin_name match AND (b) artifact-path match (exact normalized absolute path equality, with empty-artifact_path fallback to name-only); BOTH callsites in the executor must apply these two conditions — missing the artifact check at either callsite causes premature quarantine discharge; same module as the INDETERMINATE classification in BC-1.18.001
- `crates/hook-plugins/validate-unvalidated-mutation-marker/src/lib.rs` — TTL-expiry clear branch: after `fs::remove_file` succeeds on the TTL auto-delete, calls `emit_marker_cleared(TtlExpired, ...)` (implementer's choice: either emit via a shared crate function or inline — no shared-crate constraint). Absent-marker branch: RAW_DELETE_DETECTED reconciliation — read FileSink log for unmatched `plugin.indeterminate` matching `(plugin_name, artifact_path)`; emit `emit_marker_cleared(OperatorOverride, ...)` with `reason = "RAW_DELETE_DETECTED: ..."` if found; no hard failure if FileSink is unavailable.

## Story Anchor

S-25.01 — Dispatcher INDETERMINATE Outcome Layer 1: Fail-Loud on Cannot-Complete — durable marker + next-advance gate

## VP Anchors

- VP-106 — Successful Re-Validation Deletes Marker (artifact-scoped: same plugin + same non-empty artifact OR empty-artifact_path fallback); TTL-expiry auto-deletes marker; fail-open INDETERMINATE Writes No Marker (unit-test; covers PC1 artifact-scoped clear, PC2 idempotent delete, PC4 TTL-expiry clear, Invariant 1 FAIL-preserves, Invariant 2 artifact-scoped + empty-path exception, Invariant 4, Invariant 5 marker-level TTL, EC-008/EC-009)

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-106 | PASS re-validation (same plugin, same non-empty artifact) deletes `.factory/unvalidated-mutation.marker`; deletion is idempotent (no error when marker already absent) | unit-test |
| VP-106 | FAIL re-validation does NOT delete marker; marker persists until PASS or manual deletion | unit-test |
| VP-106 | `should_write_marker(Indeterminate, FailOpen) == false`; `should_write_marker(Indeterminate, FailClosed) == true` | unit-test |
| VP-106 | PASS re-validation (same plugin, DIFFERENT non-empty artifact) does NOT delete marker; marker persists — artifact A quarantine unresolved (EC-008) | unit-test |
| VP-106 | PASS re-validation (same plugin, empty marker `artifact_path`) DOES delete marker — empty-path fallback, plugin_name equality alone suffices (EC-009) | unit-test |
| VP-106 | TTL-expiry clear (PC4): gate reads expired `expires_at` (≤ now UTC) on normal path → Allow (exit_code=0) + auto-delete marker (idempotent; NotFound swallowed); INV5 TTL is marker-level (no plugin_name/artifact_path check); non-expired marker remains blocked; legacy marker (no `expires_at`) treated as non-expired | unit-test |
| VP-106 | REVALIDATED clear emits `marker.cleared(clear_mode=REVALIDATED, actor_type=validator, trace_id=originating-indeterminate-trace-id, reason=null)` after `delete_marker_if_pass` succeeds (PC1 audited clear; EC-010) | unit-test |
| VP-106 | TTL_EXPIRED clear emits `marker.cleared(clear_mode=TTL_EXPIRED, actor_type=deadman, reason=null)` after TTL auto-delete on normal-path eval (PC4 TTL-loudness; EC-011) | unit-test |
| VP-106 | OPERATOR_OVERRIDE reconciliation: gate finds marker absent + unmatched `plugin.indeterminate` in FileSink → emits `marker.cleared(clear_mode=OPERATOR_OVERRIDE, actor_type=operator, reason=RAW_DELETE_DETECTED:...)` best-effort; no hard failure if FileSink unavailable (PC3 audited clear; EC-012/EC-013) | unit-test |
| VP-106 | Crash-path TTL allow (dispatcher native check) does NOT emit `marker.cleared(TTL_EXPIRED)` — only gate plugin's normal-path auto-delete emits it (EC-014) | unit-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-041 |
| Capability Anchor Justification | CAP-041 ("Validation Integrity: INDETERMINATE Outcome, Durable Mutation Marker, and Next-Advance Gate") per capabilities.md §CAP-041 — this BC specifies the marker-clear lifecycle that completes the quarantine loop defined in CAP-041: "The marker is cleared by successful re-validation (same plugin, PASS outcome) or manual operator deletion (`rm .factory/unvalidated-mutation.marker`)." |
| L2 Domain Invariants | none (dispatcher runtime marker lifecycle invariant, not L2 domain spec) |
| Architecture Module | SS-01 (Hook Dispatcher Core — `delete_marker_if_pass` and `should_write_marker` in `indeterminate_marker.rs`; `executor.rs` clear call-site) |
| ADR | ADR-047 §Decision 5 (Marker Clear Protocol — Condition A successful re-validation; Condition B manual operator escape hatch; clear-on-PASS not clear-on-FAIL; idempotent delete; named-plugin AND artifact-scoped clear; empty-artifact_path falls back to name-only); ADR-047 §Decision 9 (both gate arms unblocked simultaneously by single marker deletion); ADR-048 §Decision 2 (TTL-expiry as third clear path — gate plugin checks expires_at on normal path; expired marker treated as absent → Allow + auto-delete; missing expires_at on legacy markers treated as non-expired; TTL is marker-level not artifact-scoped; UNVALIDATED_MUTATION_MARKER_TTL_SECONDS = 86_400); ADR-048 §Decision 4 (marker.cleared audited event — REVALIDATED emitted from delete_marker_if_pass; TTL_EXPIRED emitted from gate plugin normal-path TTL auto-delete branch (TTL-loudness: previously silent); OPERATOR_OVERRIDE reconciled retroactively via RAW_DELETE_DETECTED on next gate eval when marker absent + unmatched plugin.indeterminate found; best-effort if FileSink unavailable; no signed digests/dual-control per cooperating-agent threat model; same FileSink/InternalLog emission path as plugin.indeterminate) |
| Stories | S-25.01 |
| Cycle | v1.0-feature-validation-integrity-layer1 (F2 — product-owner spec burst) |
| Feature | E-25 — Validation Integrity and Large-Artifact Resilience |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.3 | 2026-08-31 | product-owner | ADR-048 §Decision 4 — audited clear model. (1) Description updated: all three clear paths now emit `marker.cleared` (BC-3.08.001 Event 9). (2) PC1 audited: `delete_marker_if_pass` now calls `emit_marker_cleared(REVALIDATED)` after successful delete; `trace_id` from originating `plugin.indeterminate`; `actor_type=validator`; `reason=null`. (3) PC3 audited: retroactive `marker.cleared(OPERATOR_OVERRIDE)` via RAW_DELETE_DETECTED reconciliation on next gate eval when marker absent + unmatched `plugin.indeterminate` found; best-effort (no hard failure if FileSink unavailable). (4) PC4 TTL-loudness: TTL auto-delete now emits `marker.cleared(TTL_EXPIRED)` with `actor_type=deadman`; previously silent auto-delete is now audited. Note: crash-path native TTL allow (dispatcher Decision 1) does NOT emit — only gate plugin normal-path auto-delete emits. (5) EC-010–014 added (three clear_mode emissions + FileSink-unavailable case + crash-path exclusion). (6) Canonical test vectors: three new rows for REVALIDATED/TTL_EXPIRED/OPERATOR_OVERRIDE emissions; prior TTL rows updated to note `marker.cleared(TTL_EXPIRED)` emission. (7) VP-106: four new property rows for audited emissions. (8) Architecture Anchors: `emit_marker_cleared` function added; `validate-unvalidated-mutation-marker` plugin crate anchored for TTL_EXPIRED + OPERATOR_OVERRIDE reconciliation. (9) Traceability ADR: ADR-048 §Decision 4 added. |
| 1.2 | 2026-08-31 | product-owner | ADR-048 §Decision 2 — adds TTL-expiry as a third clear path. (1) Description updated: two clear paths → three clear paths. (2) Added PC4: gate plugin reads expired `expires_at` on normal path → treat marker as ABSENT, Allow (exit_code=0), auto-delete (idempotent; swallow NotFound). (3) Added INV5: TTL is marker-level expiry (not artifact-scoped) — any marker expires 86400s after `expires_at` regardless of which artifact triggered INDETERMINATE; confirmed consistent with PC1 artifact-scoped clear and BC-1.18.001 INV3 single-marker. (4) Added canonical test vectors: expired-marker→Allow+auto-delete; non-expired-marker→Block (existing behavior confirmed). (5) Traceability ADR: ADR-048 §Decision 2 citation added. ADR-048 added to inputs. |
| 1.1 | 2026-08-31 | product-owner | S-25.01 adversary M-1 resolution. Promote artifact-scoped clear to authoritative predicate throughout BC. PC1 rewritten: clear requires (a) plugin_name match AND (b) artifact_path match (exact normalized absolute path equality), with explicit empty-artifact_path fallback (vacuously satisfied → name-only suffices for non-artifact-scoped validators). INV2 rewritten to match PC1. Architecture Anchors updated: `delete_marker_if_pass` signature gains `current_artifact_path` param; executor.rs note flags BOTH callsites must apply the two-condition gate. Added EC-008 (same plugin, different non-empty artifact → no clear) and EC-009 (empty marker artifact_path → clear via fallback). Added two canonical test vectors for EC-008/EC-009. VP-106 Anchors and Verification Properties table expanded with corresponding unit-test rows. ADR-047 §D5 citation updated to name artifact-scoped clear + empty-path fallback. Traceability ADR field updated to match. |
| 1.0 | 2026-08-30 | product-owner | Initial creation. F2 spec-evolution burst, validation-integrity-layer1. BC-1.18.003: marker-clear protocol (PASS-clears/FAIL-preserves/idempotent/operator-rm), named-plugin scoping invariant, both-arms-unblock invariant. VP-106 anchored. CAP-041 capability anchor. ADR-047 §D5 citations. |
