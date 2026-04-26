---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-27T03:00:00Z
phase: 1a
inputs: [.factory/stories/S-7.03-tdd-discipline-hardening.md]
input-hash: "a3187d9"
traces_to: .factory/stories/S-7.03-tdd-discipline-hardening.md
origin: brownfield
extracted_from: ".factory/stories/S-7.03-tdd-discipline-hardening.md#AC-005"
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
bc_id: BC-5.38.005
section: "5.38"
---

# BC-5.38.005: stub-architect applies self-check before committing any non-todo!() function body

## Description

Before including any non-`todo!()` function body in a stub commit, the stub-architect must apply an explicit self-check question: "If I include this real implementation, will the test for this function pass trivially without any implementer work?" If the answer is yes, the implementation must be replaced with `todo!()` (unless it qualifies as GREEN-BY-DESIGN per BC-5.38.002 or WIRING-EXEMPT per BC-5.38.003). This self-check is the final cognitive gate before the structural anti-pattern takes root.

## Preconditions

1. The stub-architect is creating a stub commit for a `tdd_mode: strict` (or absent) story.
2. The stub-architect is about to include a function body that contains real logic (not `todo!()`).
3. The function does not meet GREEN-BY-DESIGN criteria (BC-5.38.002) or WIRING-EXEMPT criteria (BC-5.38.003).

## Postconditions

1. If the self-check answer is "yes, test would pass trivially" — the real impl is replaced with `todo!()`.
2. If the self-check answer is "no, or this is a legitimate exemption" — the impl proceeds with the exemption flag in the report.
3. The stub-architect's report or commit message body includes a self-check summary for any non-`todo!()` functions that passed the check (i.e., were legitimately kept).

## Invariants

1. The self-check question is: "If I include this real implementation, will the test for this function pass trivially without any implementer work?"
2. The self-check question must appear VERBATIM (not paraphrased). Paraphrased text is not acceptable, mirroring BC-5.38.006 EC-003.
3. "Trivially pass" means: the test would go GREEN at Step 3 Red Gate with this stub, with no implementer action required.
4. The self-check applies even when the implementation is simple — simplicity does not exempt a function from the check.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Stub-architect is uncertain whether a function qualifies — defaults to `todo!()` | CORRECT. When in doubt, stub with `todo!()`. The implementer will fill it. |
| EC-002 | Function body is a delegating call: `fn foo(&self) -> u64 { self.inner.foo() }` | Depends on `inner.foo()`. If that's also a stub, keeping the delegation is likely fine. Apply self-check to the delegation chain. |
| EC-003 | Stub-architect applies self-check and determines that ALL functions in scope are legitimately real | Full-exception path — all functions are exempted. Orchestrator must acknowledge per BC-8.29.003. |

## Canonical Test Vectors

| Input State | Expected Output | Category |
|-------------|----------------|----------|
| `fn validate_input(&self, s: &str) -> bool { s.len() > 0 }` — stub architect considers keeping it | Self-check: test `assert!(obj.validate_input("hello"))` would trivially pass. Replace with `todo!()`. | correct self-check application |
| `fn schema_version() -> u32 { 1 }` — stub architect considers keeping it | Self-check: constant return. Qualifies as GREEN-BY-DESIGN. Keep. Flag in report. | legitimate exemption |
| Stub-architect skips self-check and commits real impl | VIOLATION of this BC — self-check is non-optional for any non-`todo!()` body | negative |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (peer review) | Self-check rule present in stub-architect prompt constraints | adversarial check |
| BATS-static-check | test_self_check_question_in_stub_architect_prompt (S-7.03 AC-011 f) | bats grep |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-016 |
| Capability Anchor Justification | CAP-016 ("Drive TDD delivery with red/green/refactor gate enforcement") per capabilities.md §CAP-016 — this BC provides the agent-level cognitive protocol that operationalizes the "failing test must exist before implementation" principle at the exact moment of stub creation. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/agents/stub-architect.md, plugins/vsdd-factory/workflows/phases/per-story-delivery.md |
| Stories | S-7.03 |
| Source AC | S-7.03 §AC-005 |
| FR | FR-043 |

## Related BCs

- BC-5.38.001 — parent (todo!() obligation; this BC is the cognitive enforcement)
- BC-5.38.002 — sibling (GREEN-BY-DESIGN exemption; feeds self-check outcome)
- BC-5.38.003 — sibling (WIRING-EXEMPT exemption; feeds self-check outcome)
- BC-5.38.004 — sibling (anti-precedent guard; contextual constraint for self-check)

## Architecture Anchors

- `plugins/vsdd-factory/agents/stub-architect.md` — self-check rule in Constraints section

## Story Anchor

S-7.03

## VP Anchors

(peer review / adversarial coverage)
