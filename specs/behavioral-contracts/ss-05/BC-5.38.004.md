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
extracted_from: ".factory/stories/S-7.03-tdd-discipline-hardening.md#AC-004"
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
bc_id: BC-5.38.004
section: "5.38"
---

# BC-5.38.004: stub-architect must not use pre-implemented sibling crates as stub templates

## Description

The stub-architect agent must not use existing sibling crates in the same workspace as authoring templates when those sibling crates contain full implementations rather than `todo!()` stubs. In the Prism Wave 2 incident, stub-architects copied the pattern from previously merged DTU crates (aa706543, 6d2d005e) that had pre-implemented stubs, producing a precedent cascade that violated TDD Iron Law across 3 of 5 parallel stories. The dispatch prompt must contain an explicit anti-precedent guard that names this risk.

## Preconditions

1. The stub-architect agent is dispatched for Step 2 of per-story-delivery.
2. One or more sibling crates exist in the workspace from previous story deliveries.
3. The dispatch prompt or SKILL.md is loaded into the agent's context.

## Postconditions

1. The dispatch prompt for stub-architect (deliver-story SKILL.md and per-story-delivery.md Step 2 instructions) contains verbatim anti-precedent guard text (see Invariants §1 below).
2. The stub-architect, when observing sibling crates with full implementations, treats them as anti-patterns not templates.
3. The stub output conforms to BC-5.38.001 regardless of what sibling crates contain.

## Invariants

1. **Required verbatim guard text** (must appear in dispatch instructions):
   > "ANTI-PRECEDENT GUARD: Do not use sibling crates with pre-implemented stubs as templates for your stub work. If you observe that a sibling crate (e.g., a DTU clone or prior story's scaffold) contains full business logic rather than todo!() macros, treat it as a historical anti-pattern. Your stub must use todo!() for all non-trivial function bodies. Anti-precedent evidence: Prism commits aa706543, 6d2d005e, 20b4a12a. Model precedent: e86d03f2."
2. The guard text must appear before Step 2 scaffolding instructions in the prompt, not appended as a footnote.
3. The guard text must name the specific SHA commits as historical evidence — abstract warnings without evidence are insufficient.

## Edge Cases

| ID | Description | Expected Behavior |
|----|-------------|-------------------|
| EC-001 | Sibling crate is a `tdd_mode: facade` story that legitimately has full impl | Guard still applies: stub-architect must not reproduce the impl pattern in a `tdd_mode: strict` story context. |
| EC-002 | No sibling crates exist in workspace | Guard text still required in prompt. Its presence is an invariant regardless of workspace state. |
| EC-003 | Stub-architect cannot find any reference code and asks for clarification | Expected: stub-architect produces minimal `todo!()` skeletons from the story's spec and BC docs, not from inferred implementations. |

## Canonical Test Vectors

| Input State | Expected Output | Category |
|-------------|----------------|----------|
| deliver-story SKILL.md loaded, anti-precedent guard text present | Stub-architect uses `todo!()` bodies; guard citation appears in commit message | happy-path |
| Workspace has aa706543-pattern sibling crate; anti-precedent guard present in prompt | Stub-architect does NOT copy full impl; `todo!()` bodies used | anti-pattern prevention |
| Dispatch prompt missing anti-precedent guard text | VIOLATION of this BC — the prompt itself is non-compliant | static-check violation |

## Verification Properties

| VP-NNN | Property | Proof Method |
|--------|----------|-------------|
| (static-check) | Anti-precedent guard text present verbatim in deliver-story SKILL.md | peer review / adversarial check |

## Traceability

| Field | Value |
|-------|-------|
| L2 Capability | CAP-016 |
| Capability Anchor Justification | CAP-016 ("Drive TDD delivery with red/green/refactor gate enforcement") per capabilities.md §CAP-016 — this BC specifies the guard text that makes CAP-016's red-gate requirement enforceable when agent context includes anti-pattern precedents. |
| L2 Domain Invariants | none |
| Architecture Module | plugins/vsdd-factory/skills/deliver-story/SKILL.md, plugins/vsdd-factory/workflows/phases/per-story-delivery.md |
| Stories | S-7.03 |
| Source AC | S-7.03 §AC-004 |
| FR | FR-043 |

## Historical Evidence

**Anti-precedent commits (Prism Wave 2 — do NOT use as templates):**
- `aa706543` — Prism S-6.13 Jira DTU: full impl in stub; caused Red Gate bypass
- `6d2d005e` — Prism S-6.12 PagerDuty DTU: full impl in stub; same pattern
- `20b4a12a` — Prism S-2.04 audit-construction: full impl in stub; same pattern

**Model-precedent commit (correct pattern):**
- `e86d03f2` — Prism S-2.06 datasource-trait: 5 genuine `todo!()` macros; genuine TDD cycle

## Related BCs

- BC-5.38.001 — parent (todo!() obligation; this BC ensures the obligation is encoded in the prompt)
- BC-5.38.005 — sibling (self-check rule that complements this anti-precedent guard)

## Architecture Anchors

- `plugins/vsdd-factory/skills/deliver-story/SKILL.md` — must contain anti-precedent guard text
- `plugins/vsdd-factory/workflows/phases/per-story-delivery.md` — Step 2 dispatch; guard text location

## Story Anchor

S-7.03

## VP Anchors

(static check — no formal VP required; adversarial review covers this)
