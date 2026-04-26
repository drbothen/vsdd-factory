---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-26T00:00:00
phase: 1a
inputs: [.factory/stories/S-7.03-tdd-discipline-hardening.md]
input-hash: "a3187d9"
traces_to: .factory/stories/S-7.03-tdd-discipline-hardening.md
origin: brownfield
extracted_from: ".factory/stories/S-7.03-tdd-discipline-hardening.md#AC-006"
subsystem: "SS-05"
capability: "CAP-016"
lifecycle_status: active
introduced: v1.0-brownfield-backfill
modified: []
deprecated: null
deprecated_by: null
replacement: null
retired: null
removed: null
removal_reason: null
bc_id: BC-8.29.002
section: "8.29"
---

# BC-8.29.002: each non-RED test must be documented in red-gate-log with rationale before threshold relaxation

## Description

Every test that passes (is GREEN) at Step 3 Red Gate — and is not already classified as GREEN-BY-DESIGN or WIRING-EXEMPT — must be individually documented in the Red Gate log with a rationale before the threshold relaxation pathway (BC-8.29.003) can be invoked. The log serves as the audit trail that distinguishes intentional exemptions from undetected anti-pattern violations. Its existence also provides the orchestrator with the information needed to make a sound remediation decision.

## Preconditions

1. Step 3 (Red Gate) has run and produced test results.
2. At least one new test is GREEN (passes against the stub) and is not already classified GREEN-BY-DESIGN or WIRING-EXEMPT.
3. RED_RATIO < 0.5 (gate would block without the exception path).

## Postconditions

1. A Red Gate log file is written to `.factory/logs/red-gate-log-<story-id>.md` before Step 4 is considered.
2. Each unexpectedly-GREEN test has an entry in the log with: test name, test function, result (GREEN), and rationale from one of: {PURE-DATA, FRAMEWORK-WIRING, STRUCTURAL-ASSERTION, PRE-EXISTING-BEHAVIOR, OTHER-JUSTIFIED}.
3. The total count of documented exemptions matches `TOTAL_NEW_TESTS - RED_TESTS - KNOWN_EXEMPT_TESTS`.
4. If any GREEN test cannot be justified under a recognized rationale, the log entry must read "UNJUSTIFIED — possible stub-as-implementation violation" and the orchestrator must treat this as a BLOCKING item.

## Invariants

1. The log must be produced before any exception-path invocation (BC-8.29.003). A retroactive log is invalid.
2. "UNJUSTIFIED" entries in the log are blocking and cannot be waived without human orchestrator sign-off.
3. Log format must be machine-parseable: each entry on a new line with structured fields (test_name, result, rationale_category, notes).
4. The log persists in `.factory/logs/` for at least the duration of the story's delivery cycle and is referenced in the PR description.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | RED_RATIO ≥ 0.5 — no GREEN tests requiring justification | Log is still written with `RED_RATIO: <value>` and `gate: PASS`. No exemption entries needed. |
| EC-002 | Test is GREEN because it tests a constant that happens to be correct in the stub (not stub-author's intent) | STRUCTURAL-ASSERTION rationale is appropriate if the test verifies compile-time-fixed behavior. Log with notes. |
| EC-003 | 10+ GREEN tests with no clear justification | Each must be logged as UNJUSTIFIED. Orchestrator must decide: roll back stub, accept with mutation-testing obligation, or explicitly waive. |

## Canonical Test Vectors

| Scenario | Log Entry | Valid? |
|----------|-----------|--------|
| `test_column_name_returns_jira_status` passes — GREEN-BY-DESIGN function | Entry: `{test: "test_column_name_returns_jira_status", result: GREEN, rationale: PURE-DATA, notes: "enum variant label"}` | Yes |
| `test_create_charge_returns_200` passes — handler implemented in stub | Entry: `{test: "test_create_charge_returns_200", result: GREEN, rationale: UNJUSTIFIED, notes: "handler business logic should be todo!()"}` | Blocking |
| All tests RED, RED_RATIO = 1.0 | Log: `{RED_RATIO: 1.0, gate: PASS, exemptions: []}` | Yes |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-063 | Log counts are consistent with RED_RATIO computation | integration (bats) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-016 |
| Capability Anchor Justification | CAP-016 ("Drive TDD delivery with red/green/refactor gate enforcement") per capabilities.md §CAP-016 — this BC provides the audit mechanism for CAP-016's gate enforcement, making the decision trail for threshold relaxation inspectable and reversible. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/workflows/phases/per-story-delivery.md (SS-05 territory), .factory/logs/ |
| Stories | S-7.03 |
| Source AC | S-7.03 §AC-006, AC-007 |
| FR | FR-043 |

## Related BCs

- BC-8.29.001 — parent (the gate that reads this log)
- BC-8.29.003 — sibling (exception path that depends on a complete log)
- BC-5.38.002 — depends on (GREEN-BY-DESIGN entries that pre-populate the log)
- BC-5.38.003 — depends on (WIRING-EXEMPT entries that pre-populate the log)

## Architecture Anchors

- `plugins/vsdd-factory/workflows/phases/per-story-delivery.md` — Step 3 Red Gate log production
- `.factory/logs/red-gate-log-<story-id>.md` — output artifact path

## Story Anchor

S-7.03

## VP Anchors

- VP-063 — RED_RATIO computation correctness

## Notes

**Subsystem Historical Artifact:** The BC-ID prefix `8.29` embeds the original subsystem assignment of SS-08. After adversarial pass-1, this BC was authoritatively re-anchored to **SS-05 (Pipeline Orchestration)** because its Architecture Module (`per-story-delivery.md`) lives in SS-05 territory. The `subsystem: SS-05` frontmatter is authoritative; the BC-ID prefix is a historical artifact. Per append_only_numbering policy, the ID is preserved rather than renumbered.
