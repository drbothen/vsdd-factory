---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-25T00:00:00
phase: 1a
inputs: [.factory/stories/S-6.01-create-adr-skill.md]
input-hash: "d81e07b"
traces_to: .factory/stories/S-6.01-create-adr-skill.md
origin: greenfield
extracted_from: ".factory/stories/S-6.01-create-adr-skill.md#AC-8"
subsystem: "SS-06"
capability: "CAP-017"
lifecycle_status: active
introduced: v1.0-brownfield-backfill
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-6.20.012
section: "6.20"
---

# BC-6.20.012: create-adr is atomic — any partial-state failure rolls back all side-effects

## Description

The create-adr skill treats the sequence (ID allocation → file write → supersession patch → ARCH-INDEX insertion → validation) as a single atomic unit. If any step fails, all side-effects already applied in that invocation are reverted: the new ADR file is deleted, the ARCH-INDEX row is removed, and the old ADR's `superseded_by` is restored. The one defined exception is validation failure (BC-6.20.011): the ADR file is intentionally left on disk for inspection, while ARCH-INDEX and supersession are still reverted. After a failure, idempotent re-invocation with the same arguments must succeed. Atomicity assumption: skill runs to completion. SIGKILL or system crash voids this contract — partial state may persist; user runs `validate-template-compliance.sh` to detect and `git status` + manual cleanup to recover.

## Preconditions

1. At least one step in the sequence has been partially applied (a file written, a patch applied, or a row inserted).
2. A subsequent step has failed (I/O error, validation failure, permission error, missing dependency, etc.).

## Postconditions

1. **New ADR file:** deleted if written (except on validation failure per BC-6.20.011 — file survives for inspection).
2. **ARCH-INDEX row:** reverted if inserted (applies to all failure modes including validation failure).
3. **Old ADR `superseded_by`:** restored to its pre-invocation value if patched (applies to all failure modes including validation failure).
4. Skill exits non-zero with a message that identifies (a) the step that failed and (b) each revert action taken.
5. Idempotency (bounded): After a full rollback (any failure path EXCEPT validation-failure per AC-7), re-invoking the skill with identical arguments produces the same result as a first-time invocation. After a validation-failure rollback (where the file persists by design — see BC-6.20.011 invariant 3), re-invocation requires either (a) deleting the leftover file manually before re-running, or (b) invoking with `--id` omitted to let auto-allocation pick the next free ID. Plain re-invocation with the original args after validation failure will trigger BC-6.20.002 duplicate detection and exit non-zero.

## Invariants

1. No partial state is ever left in the repository after a failed invocation (with the single documented exception of the ADR file on validation failure).
2. Revert messages are always specific: "Deleted decisions/ADR-NNN-slug.md", "Reverted ARCH-INDEX row for ADR-NNN", "Restored ADR-MMM superseded_by to null".
3. Rollback itself must not fail silently — if a revert operation fails, the skill reports which revert failed and which succeeded, and still exits non-zero.
4. The skill never exits 0 after a partial-state failure, even if rollback was fully successful.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | File write succeeds; ARCH-INDEX insertion fails | New ADR file deleted; exit non-zero naming both the insertion failure and the deletion revert |
| EC-002 | File write, supersession patch, and ARCH-INDEX insertion all succeed; validation fails | ARCH-INDEX row reverted; supersession patch reverted; ADR file left on disk; exit non-zero |
| EC-003 | File write succeeds; supersession patch fails | New ADR file deleted; ARCH-INDEX row not yet inserted (nothing to revert there); exit non-zero |
| EC-004 | Rollback of ARCH-INDEX row itself fails (e.g., file locked) | Skill reports "WARNING: ARCH-INDEX revert failed — manual cleanup required"; still exits non-zero |
| EC-005 | Two concurrent invocations try to write ADR-014 simultaneously | Second invocation detects file collision on write; rolls back; exits non-zero; user serialises. See also BC-6.20.001 precondition 5 for the TOCTOU race detail (both invocations may compute max+1 before either writes). |
| EC-006 | Re-invocation after EC-002 (file still on disk from previous failure) | BC-6.20.002 duplicate check triggers; user must delete the leftover file before re-running |
| EC-007 | Process killed mid-execution (SIGKILL, system crash, hardware failure) | Partial state may persist on disk: new ADR file present without ARCH-INDEX row, or ARCH-INDEX row inserted without supersession patch. Detection: run `validate-template-compliance.sh` post-restart; user runs `git status` and reverts as needed. The atomicity contract assumes graceful failure (exception or non-zero exit), not abrupt termination. |

## Canonical Test Vectors

| Failure Scenario | Expected Side-Effect State After | Exit Code | Category |
|-----------------|----------------------------------|-----------|----------|
| ARCH-INDEX insert fails after file write | New ADR file deleted; ARCH-INDEX unchanged | non-zero | error |
| Supersession patch fails after file write | New ADR file deleted; old ADR unchanged; ARCH-INDEX unchanged | non-zero | error |
| Validation fails after all writes | ARCH-INDEX row reverted; old ADR superseded_by reverted; ADR file remains on disk | non-zero | error |
| All steps succeed | All side-effects persisted | 0 | happy-path |
| Re-invocation after full rollback | Identical to first-time invocation | per normal flow | edge-case |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-058 | Atomicity: skill leaves either all-side-effects-applied or zero-side-effects-applied (validation failure: file survives, others reverted) | integration-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-017 |
| Capability Anchor Justification | Anchored to CAP-017 (Create and manage formal ADR records) per capabilities.md §CAP-017 — literal match for ADR scaffolding. |
| L2 Domain Invariants | none directly |
| Architecture Module | plugins/vsdd-factory/skills/create-adr/SKILL.md |
| Stories | S-6.01 |
| Source AC | S-6.01 §AC-8 (tests: `test_atomicity_write_fail_no_index_row`, `test_atomicity_index_insert_fail_file_deleted`, `test_atomicity_supersession_patch_fail_new_file_deleted`, `test_idempotent_reinvocation_after_failure`) |
| FR | FR-041 |

## Related BCs

- BC-6.20.007 — supersession patch; rollback restores old ADR to pre-invocation state
- BC-6.20.008 — ARCH-INDEX insertion; rollback removes inserted row
- BC-6.20.011 — validation gate; partial exception to full rollback (file survives)

## Architecture Anchors

- `plugins/vsdd-factory/skills/create-adr/SKILL.md` — skill entry point; atomicity section

## Story Anchor

S-6.01

## VP Anchors

- VP-058 — atomicity invariant
