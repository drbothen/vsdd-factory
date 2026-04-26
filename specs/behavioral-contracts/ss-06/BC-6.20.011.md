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
extracted_from: ".factory/stories/S-6.01-create-adr-skill.md#AC-7"
subsystem: "SS-06"
capability: "CAP-001"
lifecycle_status: active
introduced: v1.0-brownfield-backfill
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-6.20.011
section: "6.20"
---

# BC-6.20.011: create-adr runs validate-template-compliance.sh as final gate, blocks on non-zero

## Description

After all file writes and ARCH-INDEX insertion are complete, the skill executes `plugins/vsdd-factory/bin/validate-template-compliance.sh` against the newly created ADR file. A zero exit code from the script causes the skill to print "Template compliance: PASS" and exit 0. A non-zero exit code causes the skill to print the script's output and a failure message, and then treat the invocation as failed: the ARCH-INDEX row is reverted and any supersession patch is reverted, but the ADR file itself is left on disk for inspection. The skill exits non-zero.

## Preconditions

1. All file writes (new ADR, brownfield annotation if applicable) are complete.
2. ARCH-INDEX row has been inserted.
3. Supersession patch has been applied (if `--supersedes` was supplied).
4. `plugins/vsdd-factory/bin/validate-template-compliance.sh` exists and is executable.

## Postconditions

1. **On script exit 0:**
   - Skill prints "Template compliance: PASS" to stdout.
   - Skill exits 0.
   - All side-effects (new ADR file, ARCH-INDEX row, supersession patch) persist.

2. **On script exit non-zero:**
   - Skill prints the script's combined stdout/stderr.
   - Skill prints: `"Template compliance: FAIL — ADR-NNN not registered. Fix the issues above and re-run."`
   - ARCH-INDEX row insertion is reverted.
   - Supersession patch (if applied) is reverted.
   - New ADR file is left on disk (not deleted — caller can inspect and fix).
   - Skill exits non-zero.

## Invariants

1. The validation script is always the final step — it runs after all other writes are complete.
2. On validation failure, ARCH-INDEX and supersession state are always reverted, but the ADR file always remains on disk.
3. This partial-rollback on validation failure is the one exception to BC-6.20.012's full-rollback rule: the ADR file survives validation failure specifically to enable inspection and re-run.
4. The script is invoked with exactly one argument: the path to the new ADR file.
5. If the script is missing or not executable, the skill exits non-zero without proceeding to pass/fail output (see EC-001).

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | `validate-template-compliance.sh` not found or not executable | Skill exits non-zero: "validate-template-compliance.sh not found at expected path — cannot complete AC-7"; ARCH-INDEX row and supersession patch are reverted; ADR file left on disk |
| EC-002 | Script exits 0 but prints warnings to stderr | Warnings printed to stdout; skill still exits 0 (pass) |
| EC-003 | Script exits non-zero with empty output | Failure message still printed; ARCH-INDEX and supersession reverted |
| EC-004 | Re-invocation after validation failure (idempotency) | Skill detects ADR-NNN file already exists (BC-6.20.002 duplicate check); must omit `--id` and let skill clean up or user deletes file manually before re-run |

## Canonical Test Vectors

| Scenario | Script Exit | Expected Skill Behavior | Category |
|----------|------------|------------------------|----------|
| Well-formed ADR, script exits 0 | 0 | Print "PASS"; exit 0; all writes persist | happy-path |
| ADR missing required section, script exits 1 | 1 | Print script output + "FAIL" message; revert ARCH-INDEX row; exit non-zero; file remains | error |
| Script binary missing | n/a | Exit non-zero with "not found" message; revert side-effects; file remains | error |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-058 | Atomicity: on validation failure, ARCH-INDEX and supersession are reverted (file survives) | manual + integration-test |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-001 |
| L2 Domain Invariants | none directly |
| Architecture Module | plugins/vsdd-factory/skills/create-adr/SKILL.md |
| Stories | S-6.01 |
| Source AC | S-6.01 §AC-7 (tests: `test_validation_pass_exits_zero`, `test_validation_fail_exits_nonzero_no_index_row`, `test_validation_fail_supersession_not_applied`) |
| FR | FR-041 |

## Related BCs

- BC-6.20.008 — ARCH-INDEX insertion precedes this validation gate
- BC-6.20.007 — supersession patch precedes this validation gate
- BC-6.20.012 — general atomicity contract; validation failure is a partial exception (file survives)

## Architecture Anchors

- `plugins/vsdd-factory/skills/create-adr/SKILL.md` — skill entry point
- `plugins/vsdd-factory/bin/validate-template-compliance.sh` — validation script invoked

## Story Anchor

S-6.01

## VP Anchors

- VP-058 — atomicity on validation failure
