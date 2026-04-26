---
document_type: behavioral-contract
level: L3
version: "1.0"
status: draft
producer: product-owner
timestamp: 2026-04-26T00:00:00
phase: 1a
inputs: [.factory/stories/S-7.03-tdd-discipline-hardening.md]
input-hash: ""
traces_to: .factory/stories/S-7.03-tdd-discipline-hardening.md
origin: brownfield
extracted_from: ".factory/stories/S-7.03-tdd-discipline-hardening.md#AC-001"
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
bc_id: BC-5.38.001
section: "5.38"
---

# BC-5.38.001: stub-architect commit must contain todo!()/unimplemented!() bodies for all non-trivial function implementations

## Description

The stub-architect agent, when creating stub crates or scaffold files, must use `todo!()` or `unimplemented!()` macro bodies for every function whose eventual implementation will exceed 3 lines of real logic. This prevents the Iron Law violation of pre-implementing business logic before failing tests exist. The anti-pattern was observed in Prism Wave 2 (commits aa706543, 6d2d005e, 20b4a12a) where full implementations in stub commits caused most Step 3 Red Gate tests to pass before any TDD cycle occurred.

## Preconditions

1. The stub-architect agent is performing Step 2 of per-story-delivery (stub commit creation).
2. A story has `tdd_mode: strict` (explicit) or `tdd_mode` is absent (defaults to strict).
3. A function being scaffolded has a non-trivial body — i.e., the eventual implementation will require more than 3 lines of real logic (e.g., business rule evaluation, data transformation, API call dispatch, error propagation chains).

## Postconditions

1. Every non-trivial function body in the stub commit contains `todo!()` or `unimplemented!()` — no real logic.
2. The stub commit passes `cargo check` (structural validity) but the test suite returns predominantly RED (≥50% of new tests fail) at Step 3.
3. The stub-architect's commit message or an accompanying report identifies any functions exempted from `todo!()` (e.g., pure data mapping) with explicit GREEN-BY-DESIGN justification.

## Invariants

1. The presence of `todo!()` macros in the stub commit is a necessary but not sufficient condition for the Iron Law. The Red Gate density check (BC-8.29.001) provides the sufficient enforcement.
2. A function body containing only `todo!()` is never treated as "implemented" by any downstream gate or metric.
3. This invariant applies regardless of whether sibling crates in the same workspace contain full implementations — those are anti-precedents, not templates.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Function has only 1–3 lines of real logic (e.g., `Ok(())` or simple field access) | Allowed in stub. Must be flagged GREEN-BY-DESIGN in stub report. |
| EC-002 | Framework wiring code (e.g., Tower `Service` impl `fn poll_ready`) requires minimal real code for `cargo check` | Allowed for structural minimum only. Business logic inside handlers MUST remain `todo!()`. |
| EC-003 | Stub-architect looks at a sibling crate (e.g., aa706543 Jira DTU) as a template | MUST NOT reproduce the anti-pattern. The anti-precedent guard text instructs stub-architect to treat sibling pre-implemented stubs as anti-examples. |
| EC-004 | story has `tdd_mode: facade` | This BC does not apply. See BC-8.30.001 for facade-mode semantics. |

## Canonical Test Vectors

| Input State | Expected Output | Category |
|-------------|----------------|----------|
| Stub commit for a service handler function with 15-line eventual impl | `todo!()` body in stub | happy-path (stub discipline satisfied) |
| Stub commit referencing sibling crate with full impl as template | Stub still uses `todo!()` bodies; anti-precedent guard applied | anti-pattern prevention |
| Stub commit for `enum Foo { Bar }; impl Foo { fn name(&self) -> &str { "bar" } }` | Full impl allowed; flagged GREEN-BY-DESIGN | exemption path |
| Stub commit for Tower `Service::poll_ready` returning `Poll::Ready(Ok(()))` | Minimal real code allowed for `cargo check`; handler body inside remains `todo!()` | framework wiring exemption |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| VP-064 | facade-mode mutation gate enforces correct wave-gate semantics (verifies complementary contract) | manual |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-016 |
| Capability Anchor Justification | CAP-016 ("Drive TDD delivery with red/green/refactor gate enforcement") per capabilities.md §CAP-016 — this BC directly governs the stub-architect's commitment to the red-gate phase of TDD delivery, which is the prerequisite condition that CAP-016's "failing test must exist before implementation" mandate depends on. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/agents/stub-architect.md, plugins/vsdd-factory/workflows/phases/per-story-delivery.md |
| Stories | S-7.03 |
| Source AC | S-7.03 §AC-001 |
| FR | FR-043 |

## Historical Evidence

**Anti-precedent commits (Prism Wave 2):**
- `aa706543` — Prism S-6.13 Jira DTU: full business logic in stub commit; Red Gate produced 0 RED tests
- `6d2d005e` — Prism S-6.12 PagerDuty DTU: full impl in stub; same violation
- `20b4a12a` — Prism S-2.04 audit-construction: full impl in stub; same violation

**Model-precedent commit:**
- `e86d03f2` — Prism S-2.06 datasource-trait: 5 genuine `todo!()` macros in algorithmic core; Red Gate produced real RED failures; TDD cycle was genuine

## Related BCs

- BC-5.38.002 — sibling (pure data mapping exemption criteria)
- BC-5.38.003 — sibling (framework wiring exemption criteria)
- BC-5.38.004 — sibling (anti-precedent guard text in dispatch prompt)
- BC-8.29.001 — depends on (Red Gate density enforcement — the gate that detects violations of this BC)

## Architecture Anchors

- `plugins/vsdd-factory/agents/stub-architect.md` — agent prompt file to be modified
- `plugins/vsdd-factory/workflows/phases/per-story-delivery.md` — Step 2 dispatch instructions

## Story Anchor

S-7.03

## VP Anchors

- VP-064 — facade-mode mutation gate (covers complementary facade behavior)
