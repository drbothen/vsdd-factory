---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-26T00:00:00
phase: 1a
inputs: [.factory/stories/S-7.03-tdd-discipline-hardening.md]
input-hash: "a361f34"
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
bc_id: BC-8.29.001
section: "8.29"
---

# BC-8.29.001: RED_RATIO = RED_TESTS / TOTAL_NEW_TESTS must be ≥ 0.5 before Step 4 implementer dispatch (BLOCKING)

## Description

Between Step 3 (Red Gate — test-writer runs the test suite against the stub) and Step 4 (implementer dispatch) in per-story-delivery.md, a density check gate must pass. The gate computes RED_RATIO = RED_TESTS / TOTAL_NEW_TESTS (where TOTAL_NEW_TESTS excludes GREEN-BY-DESIGN and WIRING-EXEMPT tests per BC-5.38.002 and BC-5.38.003). If RED_RATIO < 0.5, the gate BLOCKS Step 4 dispatch. This gate was the missing control that allowed the Prism Wave 2 anti-pattern (aa706543, 6d2d005e, 20b4a12a) to proceed to implementation with no genuine TDD cycle.

## Preconditions

1. Step 3 (test-writer) has completed: test suite exists, tests have been run against the stub.
2. A test result report is available that classifies each new test as RED (fails against stub) or GREEN (passes against stub).
3. GREEN-BY-DESIGN and WIRING-EXEMPT classifications from the stub commit report (BC-5.38.002, BC-5.38.003) are available.
4. The story has `tdd_mode: strict` (explicit) or absent (defaults to strict).

## Postconditions

1. RED_RATIO is computed as: `RED_TESTS / (TOTAL_NEW_TESTS - EXEMPT_TESTS)` where EXEMPT_TESTS = count of GREEN-BY-DESIGN + WIRING-EXEMPT.
2. If RED_RATIO ≥ 0.5: Step 4 proceeds normally.
3. If RED_RATIO < 0.5 with no exception path documented: Step 4 is BLOCKED. The orchestrator receives a blocking report with the computed ratio, the counts, and the required remediation options (per BC-8.29.003).
4. The computed RED_RATIO value and test counts are logged to `.factory/logs/red-gate-log-<story-id>.md` (BC-8.29.002).

## Invariants

1. The RED_RATIO threshold is 0.5 (50%). No rounding: `RED_TESTS / TOTAL_EFFECTIVE_TESTS < 0.5` is blocking.
2. TOTAL_NEW_TESTS is scoped to tests introduced in this story's delivery, not the full suite.
3. A test that exercises a GREEN-BY-DESIGN or WIRING-EXEMPT function exclusively is excluded from TOTAL_NEW_TESTS for ratio computation purposes.
4. A test suite with TOTAL_EFFECTIVE_TESTS = 0 (all tests exempt) triggers the full-exception path (BC-8.29.003) — it does NOT vacuously satisfy the gate.
5. This gate is BLOCKING: no implementer dispatch without either (a) RED_RATIO ≥ 0.5, (b) full-exception acknowledgment per BC-8.29.003, or (c) `tdd_mode: facade` override per BC-8.30.002.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | RED_RATIO = 0.5 exactly (e.g., 3 RED of 6 effective) | Gate passes (≥ 0.5 is the threshold, inclusive). |
| EC-002 | RED_RATIO = 0.4999... | Gate blocks. No float rounding grace. Use integer arithmetic: `RED_TESTS * 2 >= TOTAL_EFFECTIVE_TESTS`. |
| EC-003 | Story has 0 new tests (test-writer wrote no tests) | RED_RATIO = 0 / 0 = undefined. BLOCKS with error: "No new tests written — cannot proceed to implementation without tests". |
| EC-004 | All new tests are RED (RED_RATIO = 1.0) | Gate passes trivially. |
| EC-005 | Test-writer produced tests but none cover stub functions (all test pre-existing code) | These tests do not count as TOTAL_NEW_TESTS for ratio purposes — they are not gating the stub quality. |

## Canonical Test Vectors

| RED_TESTS | EXEMPT_TESTS | TOTAL_NEW_TESTS | Effective Total | RED_RATIO | Gate Decision |
|-----------|-------------|-----------------|-----------------|-----------|---------------|
| 4 | 0 | 6 | 6 | 0.667 | PASS |
| 3 | 0 | 6 | 6 | 0.500 | PASS (boundary) |
| 2 | 0 | 6 | 6 | 0.333 | BLOCK |
| 4 | 2 | 6 | 4 | 1.000 | PASS |
| 1 | 0 | 6 | 6 | 0.167 | BLOCK |
| 0 | 6 | 6 | 0 | undefined | BLOCK (full-exception path) |
| 0 | 0 | 0 | 0 | undefined | BLOCK (no tests written) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-063 | RED_RATIO computation is correct, boundary-safe, and monotonic | integration (bats) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-016 |
| Capability Anchor Justification | CAP-016 ("Drive TDD delivery with red/green/refactor gate enforcement") per capabilities.md §CAP-016 — this BC is the primary quantitative enforcement of CAP-016's red-gate mandate: it specifies the exact threshold and blocking behavior that makes "a story cannot merge without a test that was red before implementation" machine-checkable. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/workflows/phases/per-story-delivery.md (SS-05 territory) |
| Stories | S-7.03 |
| Source AC | S-7.03 §AC-006, AC-007 |
| FR | FR-043 |

## Historical Evidence

**Root cause of Prism Wave 2 violation:**
The absence of this gate allowed 3 of 5 parallel stories (aa706543, 6d2d005e, 20b4a12a) to reach Step 4 implementer dispatch with RED_RATIO ≈ 0.0. Step 4 became a no-op. The TDD Iron Law was silently violated with no blocking signal.

## Related BCs

- BC-5.38.001 — depends on (stub discipline that enables a meaningful RED_RATIO)
- BC-8.29.002 — sibling (Red Gate log where ratio is recorded)
- BC-8.29.003 — sibling (exception path when RED_RATIO < 0.5 with justification)
- BC-8.30.002 — depends on (`tdd_mode: facade` overrides this gate for facade stories)

## Architecture Anchors

- `plugins/vsdd-factory/workflows/phases/per-story-delivery.md` — gate inserted between Step 3 and Step 4

## Story Anchor

S-7.03

## VP Anchors

- VP-063 — RED_RATIO computation correctness

## Notes

**Subsystem Historical Artifact:** The BC-ID prefix `8.29` embeds the original subsystem assignment of SS-08 (Templates and Rules). After adversarial pass-1, this BC was authoritatively re-anchored to **SS-05 (Pipeline Orchestration)** because its Architecture Module (`per-story-delivery.md`) lives in SS-05 territory, and the Red Gate density check governs workflow-phase behavior — not template/rules content. The `subsystem: SS-05` frontmatter is authoritative; the BC-ID prefix `8.` is a historical artifact of the original assignment and does not indicate current subsystem ownership. Per append_only_numbering policy, the ID is preserved rather than renumbered.
