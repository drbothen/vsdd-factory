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
extracted_from: ".factory/stories/S-7.03-tdd-discipline-hardening.md#AC-002"
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
bc_id: BC-5.38.002
section: "5.38"
---

# BC-5.38.002: pure data mappings in stub commits may be implemented inline and must be flagged GREEN-BY-DESIGN

## Description

Not all stub functions require `todo!()` bodies. Pure data mapping functions — such as `enum Variant::column_name() -> &str` returning a literal string, unit struct constructors, or simple constant-returning accessors — may be fully implemented in the stub commit because they contain no real business logic and the corresponding tests will trivially pass. These functions must be explicitly identified in the stub-architect's report as GREEN-BY-DESIGN, preventing the Red Gate density check from treating their passing tests as violations.

## Preconditions

1. The stub-architect is creating a stub commit for a `tdd_mode: strict` (or absent) story.
2. A function qualifies as pure data mapping: it returns a compile-time-known literal, a simple field access, or a zero-logic constant with no branching, no I/O, no error paths, and no calls to non-trivial functions.
3. The stub-architect chooses to implement the function inline rather than using `todo!()`.

## Postconditions

1. The implemented pure-data function appears in the stub commit with its real body.
2. The stub-architect's report (or commit message body) lists each such function under a `GREEN-BY-DESIGN` section with a one-line justification (e.g., "enum variant label — no logic, literal return").
3. The Red Gate log (BC-8.29.002) consumes this list: tests exercising only GREEN-BY-DESIGN functions are excluded from the RED_RATIO denominator with documented rationale.

## Invariants

1. A function qualifies as GREEN-BY-DESIGN only if: (a) it has zero branching logic, (b) it has no I/O, (c) it has no calls to non-trivial helper functions, (d) its body is ≤3 lines.
2. The GREEN-BY-DESIGN designation is prospective (set at stub time) not retroactive (cannot be applied after Red Gate runs to excuse a failing threshold).
3. Enum discriminant methods, `Display` impls that format a literal string, and unit type constructors are canonical examples.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Stub-architect retroactively claims GREEN-BY-DESIGN after Red Gate runs below threshold | Not accepted. GREEN-BY-DESIGN must be in stub commit report, not added post-hoc. |
| EC-002 | Function has one tiny `if` branch (e.g., `if self.is_default() { "default" } else { "custom" }`) | Does NOT qualify. Any branching disqualifies pure-data status. Use `todo!()`. |
| EC-003 | All functions in the story are pure-data mappings (e.g., enum label crate) | Entire story may be GREEN-BY-DESIGN. RED_RATIO may be 0.0. Red Gate log must document this as a full-exception with orchestrator acknowledgment per BC-8.29.003. |

## Canonical Test Vectors

| Input State | Expected Output | Category |
|-------------|----------------|----------|
| `fn column_name(&self) -> &str { "jira_status" }` in stub | GREEN-BY-DESIGN in report; test passes; excluded from RED_RATIO | happy-path |
| `fn compute_risk_score(&self) -> f64 { 0.0 }` in stub | Does NOT qualify — business logic placeholder. Must be `todo!()`. | negative |
| All 8 functions in stub are GREEN-BY-DESIGN | RED_RATIO = 0.0; full-exception path triggered per BC-8.29.003 | edge-case (full-exception) |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-063 | RED_RATIO computation correctly excludes GREEN-BY-DESIGN tests from denominator | integration (bats) |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-016 |
| Capability Anchor Justification | CAP-016 ("Drive TDD delivery with red/green/refactor gate enforcement") per capabilities.md §CAP-016 — this BC defines the exemption criteria within the red-gate enforcement that CAP-016 mandates, ensuring the gate is precise rather than blunt. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/agents/stub-architect.md, plugins/vsdd-factory/workflows/phases/per-story-delivery.md |
| Stories | S-7.03 |
| Source AC | S-7.03 §AC-002 |
| FR | FR-043 |

## Related BCs

- BC-5.38.001 — sibling (primary todo!() obligation; this BC defines the exemption path)
- BC-5.38.003 — sibling (framework wiring exemption, separate category)
- BC-8.29.001 — depends on (Red Gate density threshold; GREEN-BY-DESIGN exclusion feeds into this gate)
- BC-8.29.002 — depends on (Red Gate log where GREEN-BY-DESIGN entries are recorded)

## Architecture Anchors

- `plugins/vsdd-factory/agents/stub-architect.md` — GREEN-BY-DESIGN reporting protocol
- `plugins/vsdd-factory/workflows/phases/per-story-delivery.md` — Step 3 Red Gate log consumption

## Story Anchor

S-7.03

## VP Anchors

- VP-063 — RED_RATIO computation correctness
