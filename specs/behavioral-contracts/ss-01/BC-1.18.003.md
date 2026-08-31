---
document_type: behavioral-contract
level: L3
version: "1.1"
status: draft
producer: product-owner
timestamp: 2026-08-30T00:00:00Z
phase: F2
inputs:
  - .factory/specs/architecture/decisions/ADR-047-indeterminate-outcome-model-durable-mutation-marker-next-advance-gate.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.18.001.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.18.002.md
  - .factory/feature-delta/validation-integrity-layer1/F1-delta-analysis.md
input-hash: "27a3121"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-041"
lifecycle_status: draft
introduced: v1.0-feature-validation-integrity-layer1
modified: ["2026-08-31"]
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-1.18.003: Successful Re-Validation Clears the Unvalidated-Mutation Marker (Idempotent; Operator Escape Hatch Supported)

## Description

The `.factory/unvalidated-mutation.marker` lifecycle has two supported clear paths. Primary
path: when the same plugin that produced INDETERMINATE is subsequently dispatched PostToolUse on
the same artifact and produces `DispatchOutcome::Pass` (exit_code=0, `host_output_too_large_seen=false`),
the dispatcher deletes the marker file. Secondary path: the operator manually deletes the marker
via `rm .factory/unvalidated-mutation.marker` — a fully supported and documented escape hatch
requiring no special command or credential. Both clear paths unblock the two gate arms
simultaneously. The clear operation is idempotent: if the marker is absent at delete time, the
operation is a no-op (no error). A failed re-validation (FAIL outcome) does NOT clear the marker.

## Preconditions

1. `.factory/unvalidated-mutation.marker` exists (was written by a prior fail-closed PostToolUse
   INDETERMINATE event per BC-1.18.001 PC4).
2. For the primary clear path (PC1): The same plugin named in the marker's `plugin_name` field
   is dispatched again in a PostToolUse event on the same artifact (or a write to the same artifact
   path).
3. The operator has shell access to the session's working directory for the secondary escape-hatch
   path (PC3).

## Postconditions

1. **Successful re-validation deletes the marker (artifact-scoped).** When the plugin named in
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

2. **Absent-marker clear is idempotent.** If `.factory/unvalidated-mutation.marker` is absent
   at delete time, `delete_marker_if_pass` is a no-op — it does NOT return an error and does
   NOT create a new file. The session remains operational. This covers the case where the marker
   was already cleared by a previous action (race-free semantics).

3. **Operator manual escape hatch.** `rm .factory/unvalidated-mutation.marker` is a fully
   supported operator action. No special command, no credential, no sentinel file, and no
   Agent dispatch is required. After the file is deleted, the next `^Agent$` dispatch and the
   next `git commit`/`git push` Bash dispatch proceed without block. This matches the break-glass
   posture from ADR-039 §Decision 3: authentication is by possession of shell access to the
   machine running the session.

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

## Related BCs

- BC-1.18.001 — writes the marker; this BC defines how it is cleared (composes with)
- BC-1.18.002 — gate behavior while marker exists and when absent; this BC controls the transition from blocked to unblocked (composes with)
- BC-1.18.004 — fail-open advisory-only behavior; fail-open INDETERMINATE never writes a marker so this BC's clear is never needed for fail-open plugins (sibling)

## Architecture Anchors

- `crates/factory-dispatcher/src/indeterminate_marker.rs` — `delete_marker_if_pass(outcome, policy, dir, current_artifact_path: &str) -> Result<(), io::Error>`; idempotent `fs::remove_file` with `NotFound` swallowed; MUST compare the marker's `artifact_path` field against `current_artifact_path` (exact normalized absolute path equality) before deleting — if marker `artifact_path` is non-empty and does not match `current_artifact_path`, skip deletion (return Ok(())); if marker `artifact_path` is empty, skip the artifact check (vacuously satisfied); `should_write_marker(outcome, policy) -> bool` predicate used by executor to determine whether to write or skip marker
- `crates/factory-dispatcher/src/executor.rs` — calls `delete_marker_if_pass` after a PostToolUse PASS on a fail-closed plugin; the clear MUST be gated on (a) plugin_name match AND (b) artifact-path match (exact normalized absolute path equality, with empty-artifact_path fallback to name-only); BOTH callsites in the executor must apply these two conditions — missing the artifact check at either callsite causes premature quarantine discharge; same module as the INDETERMINATE classification in BC-1.18.001

## Story Anchor

S-25.01 — Dispatcher INDETERMINATE Outcome Layer 1: Fail-Loud on Cannot-Complete — durable marker + next-advance gate

## VP Anchors

- VP-106 — Successful Re-Validation Deletes Marker (artifact-scoped: same plugin + same non-empty artifact OR empty-artifact_path fallback); fail-open INDETERMINATE Writes No Marker (unit-test; covers PC1 artifact-scoped clear, PC2 idempotent delete, Invariant 1 FAIL-preserves, Invariant 2 artifact-scoped + empty-path exception, Invariant 4, EC-008/EC-009)

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-106 | PASS re-validation (same plugin, same non-empty artifact) deletes `.factory/unvalidated-mutation.marker`; deletion is idempotent (no error when marker already absent) | unit-test |
| VP-106 | FAIL re-validation does NOT delete marker; marker persists until PASS or manual deletion | unit-test |
| VP-106 | `should_write_marker(Indeterminate, FailOpen) == false`; `should_write_marker(Indeterminate, FailClosed) == true` | unit-test |
| VP-106 | PASS re-validation (same plugin, DIFFERENT non-empty artifact) does NOT delete marker; marker persists — artifact A quarantine unresolved (EC-008) | unit-test |
| VP-106 | PASS re-validation (same plugin, empty marker `artifact_path`) DOES delete marker — empty-path fallback, plugin_name equality alone suffices (EC-009) | unit-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-041 |
| Capability Anchor Justification | CAP-041 ("Validation Integrity: INDETERMINATE Outcome, Durable Mutation Marker, and Next-Advance Gate") per capabilities.md §CAP-041 — this BC specifies the marker-clear lifecycle that completes the quarantine loop defined in CAP-041: "The marker is cleared by successful re-validation (same plugin, PASS outcome) or manual operator deletion (`rm .factory/unvalidated-mutation.marker`)." |
| L2 Domain Invariants | none (dispatcher runtime marker lifecycle invariant, not L2 domain spec) |
| Architecture Module | SS-01 (Hook Dispatcher Core — `delete_marker_if_pass` and `should_write_marker` in `indeterminate_marker.rs`; `executor.rs` clear call-site) |
| ADR | ADR-047 §Decision 5 (Marker Clear Protocol — Condition A successful re-validation; Condition B manual operator escape hatch; clear-on-PASS not clear-on-FAIL; idempotent delete; named-plugin AND artifact-scoped clear; empty-artifact_path falls back to name-only); ADR-047 §Decision 9 (both gate arms unblocked simultaneously by single marker deletion) |
| Stories | S-25.01 |
| Cycle | v1.0-feature-validation-integrity-layer1 (F2 — product-owner spec burst) |
| Feature | E-25 — Validation Integrity and Large-Artifact Resilience |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.1 | 2026-08-31 | product-owner | S-25.01 adversary M-1 resolution. Promote artifact-scoped clear to authoritative predicate throughout BC. PC1 rewritten: clear requires (a) plugin_name match AND (b) artifact_path match (exact normalized absolute path equality), with explicit empty-artifact_path fallback (vacuously satisfied → name-only suffices for non-artifact-scoped validators). INV2 rewritten to match PC1. Architecture Anchors updated: `delete_marker_if_pass` signature gains `current_artifact_path` param; executor.rs note flags BOTH callsites must apply the two-condition gate. Added EC-008 (same plugin, different non-empty artifact → no clear) and EC-009 (empty marker artifact_path → clear via fallback). Added two canonical test vectors for EC-008/EC-009. VP-106 Anchors and Verification Properties table expanded with corresponding unit-test rows. ADR-047 §D5 citation updated to name artifact-scoped clear + empty-path fallback. Traceability ADR field updated to match. |
| 1.0 | 2026-08-30 | product-owner | Initial creation. F2 spec-evolution burst, validation-integrity-layer1. BC-1.18.003: marker-clear protocol (PASS-clears/FAIL-preserves/idempotent/operator-rm), named-plugin scoping invariant, both-arms-unblock invariant. VP-106 anchored. CAP-041 capability anchor. ADR-047 §D5 citations. |
