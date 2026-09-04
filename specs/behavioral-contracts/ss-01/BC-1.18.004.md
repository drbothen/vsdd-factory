---
document_type: behavioral-contract
level: L3
version: "1.3"
last_amended: "2026-09-03 (v1.3) — POL-14 auto-promotion (state-manager; S-25.01 MERGED PR #807 squash `f3f9b3a1` into `develop` 2026-09-03; D-1159): `status`/`lifecycle_status` draft→active. No content/postcondition/wire-format change — mechanical POLICY-14 consequence of the anchoring story's merge. [Prior: 2026-09-03 (v1.2) — factual correction sibling-sweep of ADR-047 v1.4 (per determination-S2501-trigger-path.md, routed from pr-reviewer's fresh-eyes MAJOR finding on PR #807; human-directed; NOT a POLICY 22 design/security-model change — pure factual correction, no decision content altered): PC4's `validate-factory-path-staging` event/tool description corrected from the false 'PostToolUse hooks on Edit/Write/MultiEdit/Bash' to the registry ground truth 'PreToolUse, `^Bash$`-only'; EFFECTIVE-NOW label softened to ASSIGNED-NOW (was overclaiming live enforcement); new explicit 'Layer-1 effective fail-closed count at S-25.01 merge: ZERO' statement added (previously implied ONE via the EFFECTIVE-NOW framing) consistent with ADR-047 §Decision 8a v1.4; ZERO-enforcement gap anchored to recommended follow-up story S-25.04 (Epic E-25).]"
status: active
producer: product-owner
timestamp: 2026-08-30T00:00:00Z
phase: F2
inputs:
  - .factory/specs/architecture/decisions/ADR-047-indeterminate-outcome-model-durable-mutation-marker-next-advance-gate.md
  - .factory/specs/behavioral-contracts/ss-01/BC-1.18.001.md
  - .factory/feature-delta/validation-integrity-layer1/F1-delta-analysis.md
input-hash: "3edc772"
traces_to: .factory/specs/prd.md
origin: greenfield
extracted_from: null
subsystem: "SS-01"
capability: "CAP-041"
lifecycle_status: active
introduced: v1.0-feature-validation-integrity-layer1
modified:
  - "2026-08-30 (v1.1)"
  - "2026-09-03 (v1.2)"
  - "2026-09-03 (v1.3) — POL-14 auto-promotion draft->active, S-25.01 merged PR #807 f3f9b3a1"
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
---

# BC-1.18.004: `failure_policy = fail-open` INDETERMINATE Is Advisory-Only — No Marker Written, No Gate Triggered (Backward-Compatibility Anchor)

## Description

This contract is the explicit backward-compatibility anchor for Layer 1. All current ~76
production plugins have absent or `fail-open` `failure_policy`. When any such plugin cannot
complete (fuel exhaustion, epoch timeout, or OutputTooLarge), the INDETERMINATE outcome is
advisory-only: a `plugin.indeterminate` event is emitted to the event log, but no
`.factory/unvalidated-mutation.marker` is written and no next-advance gate is triggered. The
pipeline continues exactly as it did before Layer 1 for all fail-open plugins. The canonical
backward-compatibility guard test `test_BC_1_18_004_fail_open_default_preserves_advisory_behavior`
(VP-106) MUST NOT be deleted — it is the load-bearing regression guard for the ~76 existing
fail-open plugins.

## Preconditions

1. A plugin has `failure_policy` field absent (default) or `failure_policy = "fail-open"` in its
   `[[hook]]` registry entry. This is the current default for ALL ~76 production plugins.
2. The plugin produces an INDETERMINATE outcome (any cause: fuel, epoch, or output-too-large)
   on any hook type (PreToolUse or PostToolUse).
3. No explicit `failure_policy = "fail-closed"` assignment exists for this plugin.

## Postconditions

1. **Advisory `plugin.indeterminate` event emitted only.** The dispatcher emits a
   `plugin.indeterminate` event to the OTel-aligned event log (via FileSink, `events-*.jsonl`)
   with the same payload as for fail-closed (plugin_name, cause, trace_id, failure_policy="fail-open").
   This event is purely observational — it provides forensic data for operators to identify
   which fail-open plugins are fuel-exhausting, without blocking anything.

2. **No marker file written.** `.factory/unvalidated-mutation.marker` is NOT created or modified.
   The `should_write_marker(INDETERMINATE, FailOpen) == false` predicate enforces this.
   For the default (absent `failure_policy` field): `FailurePolicy::default() == FailOpen`
   (ADR-039 §Decision 1 + S-21.10 default implementation); the same `false` predicate applies.

3. **No next-advance gate triggered.** The `validate-unvalidated-mutation-marker` Arm 1 and
   `validate-unvalidated-mutation-marker-git` Arm 2 gate checks are not triggered (no marker
   exists). The subsequent `^Agent$` dispatch and `git commit`/`git push` Bash dispatches proceed
   normally.

4. **ALL current plugins default to this path.** In Layer 1, exactly THREE Cohort A validators
   receive `failure_policy = "fail-closed"` assignments (ADR-047 §Decision 8a — human-confirmed
   Cohort A). These are partitioned by activation status:
   - **`validate-factory-path-staging` — ASSIGNED-NOW (Cohort A-IMMEDIATE; corrected v1.2 —
     previously mislabeled EFFECTIVE-NOW, which overclaimed live enforcement):** No calibration
     dependency. `failure_policy = "fail-closed"` is set unconditionally from S-25.01 delivery.
     Registered **PreToolUse, `^Bash$`-only** (corrected v1.2 — previously misstated as
     PostToolUse on Edit/Write/MultiEdit/Bash; registry ground truth per hooks-registry.toml).
     Because the durable-marker write path (BC-1.18.001 invariant 4) fires ONLY on a PostToolUse
     dispatch, this PreToolUse-only registration structurally can never reach
     `write_indeterminate_marker`; combined with `on_error = "continue"` (which never blocks the
     current dispatch either), the assignment currently produces ZERO observable enforcement
     effect — identical to fail-open (ADR-047 §Decision 8a v1.4). This is a genuine coverage
     gap, not silently accepted: closure is tracked to follow-up story "Close
     validate-factory-path-staging zero-enforcement gap" (recommended ID S-25.04, next
     available slot under Epic E-25).
   - **`validate-pr-merge-prerequisites` and `validate-wave-gate-prerequisite` — SET-BUT-LATENT
     (Cohort A-DEFERRED):** `failure_policy = "fail-closed"` is set in the registry schema but
     calibration-gated per ADR-039 §Decision 3. These validators activate at S-21.24
     (calibration confirmation gate). Until activation, INDETERMINATE on these validators is
     advisory-only (same effect as fail-open) despite the fail-closed registry entry.
   No other validator receives a fail-closed assignment in S-25.01 unless explicitly confirmed
   by the human at the F3 spec gate. **Layer-1 effective fail-closed count at S-25.01 merge:
   ZERO** (corrected v1.2 — the ASSIGNED-NOW/EFFECTIVE-NOW framing above previously implied a
   count of ONE for `validate-factory-path-staging`; per the structural analysis above, all
   three Cohort A validators currently deliver zero enforcement effect distinct from fail-open,
   consistent with ADR-047 §Decision 8a v1.4). (Source: S-25.01 AC-016; ADR-047 §Decision 8a
   v1.4.)

5. **Backward-compat guard test preserved.** The implementation MUST include the test
   `test_BC_1_18_004_fail_open_default_preserves_advisory_behavior` (alternatively named
   `test_BC_1_18_004_fail_open_indeterminate_writes_no_marker` as in the VP-106 harness
   skeleton — either naming is acceptable IF a code comment cross-references the other name).
   This test MUST NOT be deleted per ADR-047 §Decision 7. It is the explicit regression guard
   for the ~76 existing fail-open plugins.

## Invariants

1. **Default is fail-open.** An absent `failure_policy` field is equivalent to `fail-open`. The
   `FailurePolicy::default()` implementation MUST return `FailurePolicy::FailOpen` (established
   by S-21.10 and ADR-039 §Decision 1). Any change to this default would be a breaking change
   requiring a new ADR.

2. **INDETERMINATE + fail-open never blocks.** No combination of INDETERMINATE cause (fuel,
   epoch, output-too-large) with `failure_policy = fail-open` can trigger a block via the
   INDETERMINATE mechanism. (Pre-existing `on_error = "block"` + crash semantics are orthogonal
   and remain unchanged — ADR-039 axes-independence invariant.)

3. **Event emission is the only observable effect for fail-open INDETERMINATE.** The advisory
   `plugin.indeterminate` event and the existing `plugin.timeout` event (still emitted for
   fuel/epoch timeouts) are the COMPLETE observable effects. No filesystem writes, no gate
   registration changes, no block decisions.

4. **Cohort A fail-closed assignments are orthogonal to this BC.** The three Cohort A fail-closed
   validators (once assigned) are governed by BC-1.18.001. This BC governs ALL other plugins.
   The two behaviors are mutually exclusive per plugin: a plugin is either fail-open (this BC) or
   fail-closed (BC-1.18.001 through BC-1.18.003), never both.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Fail-open plugin fuel-exhausts on PostToolUse | Advisory `plugin.indeterminate` event emitted. No marker. No gate block on next Agent dispatch. |
| EC-002 | Fail-open plugin fuel-exhausts on PreToolUse | Advisory `plugin.indeterminate` event emitted. No marker. No gate block. |
| EC-003 | Fail-open plugin returns OutputTooLarge + exit_code=0 (PostToolUse) | Advisory event emitted with cause=`output-too-large`. No marker (fail-open path). |
| EC-004 | `failure_policy` field absent (not even in registry TOML) | Treated as fail-open (default). Same advisory-only behavior. |
| EC-005 | All ~76 current production plugins fuel-exhaust in a single burst | No marker written for any of them. No gate activated. Advisory events emitted for each. Pipeline continues. |

## Canonical Test Vectors

| Scenario | Plugin failure_policy | Cause | Expected Marker | Expected Gate Block |
|----------|-----------------------|-------|-----------------|---------------------|
| Fail-open fuel exhaustion | `fail-open` | fuel | NOT written | No block |
| Fail-open epoch timeout | `fail-open` | epoch | NOT written | No block |
| Fail-open OutputTooLarge + exit_code=0 | `fail-open` | output-too-large | NOT written | No block |
| Absent failure_policy (default) | absent | fuel | NOT written | No block |
| Fail-closed fuel exhaustion (PC5 counterpart) | `fail-closed` | fuel | Written | Blocks next Agent and git-commit/push |

## Related BCs

- BC-1.18.001 — fail-closed INDETERMINATE (marker write); this BC is the complementary fail-open path (sibling)
- BC-1.18.002 — gate behavior; fail-open never triggers this gate (sibling)
- BC-1.18.003 — marker-clear; fail-open never writes a marker so clear is never needed for fail-open plugins (sibling)

## Architecture Anchors

- `crates/factory-dispatcher/src/executor.rs` — `should_write_marker(outcome, policy) -> bool`; `FailurePolicy::default() == FailOpen` check; INDETERMINATE + FailOpen → emit advisory event only, no marker write
- `crates/factory-dispatcher/src/executor.rs` — `test_BC_1_18_004_fail_open_default_preserves_advisory_behavior` (or `test_BC_1_18_004_fail_open_indeterminate_writes_no_marker`): backward-compat guard test; MUST NOT BE DELETED
- `crates/factory-dispatcher/src/indeterminate_marker.rs` — `should_write_marker` predicate function; `FailurePolicy::FailOpen` arm returns `false`

## Story Anchor

S-25.01 — Dispatcher INDETERMINATE Outcome Layer 1: Fail-Loud on Cannot-Complete — durable marker + next-advance gate

## VP Anchors

- VP-106 — Successful Re-Validation Deletes Marker; fail-open INDETERMINATE Writes No Marker (unit-test; covers PC2 no-marker and PC3 no-gate for fail-open; the `test_BC_1_18_004_fail_open_default_preserves_advisory_behavior` test is the load-bearing backward-compat guard)

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-106 | `should_write_marker(Indeterminate, FailOpen) == false` (no marker written); `should_write_marker(Indeterminate, FailClosed) == true` | unit-test |
| VP-106 | `FailurePolicy::default() == FailOpen`: absent-field default produces no-marker behavior | unit-test |
| VP-106 | `test_BC_1_18_004_fail_open_default_preserves_advisory_behavior` MUST PASS and MUST NOT be deleted (ADR-047 §Decision 7 canonical backward-compat guard) | unit-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-041 |
| Capability Anchor Justification | CAP-041 ("Validation Integrity: INDETERMINATE Outcome, Durable Mutation Marker, and Next-Advance Gate") per capabilities.md §CAP-041 — this BC specifies the backward-compatibility boundary of CAP-041's operation: "For plugins with `failure_policy = 'fail-open'` … INDETERMINATE is advisory-only — only the `plugin.indeterminate` event is emitted; no marker is written and no gate is triggered." CAP-041 explicitly defines the fail-open path; this BC is the load-bearing contract for that path. |
| L2 Domain Invariants | none (dispatcher runtime backward-compat invariant, not L2 domain spec) |
| Architecture Module | SS-01 (Hook Dispatcher Core — executor.rs `should_write_marker` predicate; `FailurePolicy::default()` implementation) |
| ADR | ADR-047 §Decision 2 (failure_policy reuse — fail-open default, no new field); ADR-047 §Decision 7 (Backward-Compatibility Contract — canonical test `test_BC_1_18_004_fail_open_default_preserves_advisory_behavior` MUST NOT be deleted); ADR-047 §Decision 8a (Cohort A = exactly three human-confirmed validators; all others remain fail-open in Layer 1); ADR-039 §Decision 1 (failure_policy semantics + FailOpen default; axes-independence invariant) |
| Stories | S-25.01 |
| Cycle | v1.0-feature-validation-integrity-layer1 (F2 — product-owner spec burst) |
| Feature | E-25 — Validation Integrity and Large-Artifact Resilience |

## Changelog

| Version | Date | Author | Change |
|---------|------|--------|--------|
| 1.3 | 2026-09-03 | state-manager | POL-14 auto-promotion (S-25.01 MERGED PR #807 squash `f3f9b3a1` into `develop` 2026-09-03; D-1159): `status`/`lifecycle_status` draft→active. No content/postcondition/wire-format change — mechanical POLICY-14 consequence of the anchoring story's merge. |
| 1.2 | 2026-09-03 | product-owner | Factual correction (sibling-sweep of ADR-047 v1.4; per determination-S2501-trigger-path.md, routed from pr-reviewer's fresh-eyes MAJOR finding on PR #807; human-directed; NOT a POLICY 22 change — pure factual correction, no design/security-model decision content altered). PC4's `validate-factory-path-staging` event/tool description corrected: was falsely stated as "PostToolUse hooks on Edit/Write/MultiEdit/Bash artifacts"; corrected to registry ground truth "PreToolUse, `^Bash$`-only" (per hooks-registry.toml). `EFFECTIVE-NOW` label softened to `ASSIGNED-NOW` (was overclaiming live enforcement — the fail-closed bit is safely SET, not effectively ENFORCED). New explicit statement added: "Layer-1 effective fail-closed count at S-25.01 merge: ZERO" (the prior EFFECTIVE-NOW framing implied a count of ONE for `validate-factory-path-staging`; corrected because its PreToolUse-only registration can never reach the PostToolUse-only durable-marker write path, and `on_error = "continue"` never blocks the current dispatch — net effect identical to fail-open, consistent with ADR-047 §Decision 8a v1.4). ZERO-enforcement gap anchored to recommended follow-up story "Close validate-factory-path-staging zero-enforcement gap" (ID S-25.04, Epic E-25) per CLAUDE.md Canonical Principle Rule 3, rather than left silently accepted. PC1-PC3, PC5, Invariants, Edge Cases, and Canonical Test Vectors unchanged. |
| 1.1 | 2026-08-30 | product-owner | Consistency-audit finding 9 (MINOR): PC4 reworded to state Cohort A-IMMEDIATE / Cohort A-DEFERRED partition explicitly per ADR-047 §Decision 8a v1.3 and S-25.01 AC-016. `validate-factory-path-staging` = EFFECTIVE-NOW (no calibration gating); `validate-pr-merge-prerequisites` + `validate-wave-gate-prerequisite` = SET-BUT-LATENT (ADR-039 §Decision 3, activate at S-21.24). PC5 (DO-NOT-DELETE guard-test clause) and fail-open backward-compat anchor unchanged. |
| 1.0 | 2026-08-30 | product-owner | Initial creation. F2 spec-evolution burst, validation-integrity-layer1. BC-1.18.004: fail-open advisory-only behavior, no-marker/no-gate, FailurePolicy::default()=FailOpen, Cohort A (3 validators only), canonical backward-compat guard test preservation obligation. VP-106 anchored. CAP-041 capability anchor. ADR-047 §D2/D7/D8a + ADR-039 §D1 citations. |
