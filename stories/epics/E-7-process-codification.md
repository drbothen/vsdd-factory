---
document_type: epic
epic_id: "E-7"
version: "1.2"
prd_capabilities: [CAP-001, CAP-016]
prd_frs: [FR-042, FR-043]
status: open
story_count: 3
producer: story-writer
timestamp: 2026-04-27T00:10:00Z
phase: 2
inputs:
  - .factory/STATE.md
  - .factory/stories/S-6.01-create-adr-skill.md
  - .factory/cycles/v1.0-brownfield-backfill/adversarial-reviews/s6.01-pass-1.md
  - .factory/stories/S-7.03-tdd-discipline-hardening.md
input-hash: "327e6f3"
traces_to: .factory/STATE.md#Lessons-Learned
---

# Epic E-7: Process Codification (Self-Improvement)

## Description

vsdd-factory improving vsdd-factory. E-7 captures process gaps that surfaced
during the S-6.01 adversarial sub-cycle (8 passes, 27 findings closed,
19→4→2→1→1→0→0→0 trajectory) and from evidence collected during Prism Wave 2
delivery. Several findings were not one-off content defects — they were recurring
failures in agent prompt discipline, defensive propagation sweeps, meta-process
rules, and TDD Iron Law enforcement. E-7 codifies those lessons into plugin source
(agent prompts, lint hooks, rule files, delivery workflow sections) so future
delivery cycles don't re-discover the same gaps.

This is a "self-referential dogfooding" epic: the factory's adversarial process
caught its own process deficiencies, and those deficiencies become E-7 stories.
The stories in this epic are themselves authored using the lessons they codify —
spec-first, BC-anchored, with proper subsystem labeling — to validate that the
lessons land correctly.

S-7.03 is the direct product of vsdd-factory's running instances revealing an
anti-pattern in themselves: Prism Wave 2 stub-architects pre-implemented business
logic (commits aa706543, 6d2d005e, 20b4a12a) instead of writing todo!() bodies,
because earlier merged DTU clones served as precedent templates. This story
codifies four structural defenses (anti-precedent guard, RED_RATIO density gate,
tdd_mode contract, mutation wave-gate) so the TDD Iron Law cannot be silently
bypassed in future waves.

Milestone: `v1.0.x` (process hardening, parallel with Tier E implementation).
Subsystems: SS-05 (Pipeline Orchestration), SS-06 (Skill Catalog),
SS-07 (Hook Bash Layer), SS-08 (Templates and Rules).

## Source / Origin

- **STATE.md "Lessons Learned (S-6.01 sub-cycle)":** 8 lessons tabulated; 5 marked
  OPEN or PARTIAL as of 2026-04-26. These 5 are the subject of E-7.
- **D-008 in STATE.md:** "Codify spec-first-then-TDD discipline + defensive-sweep
  pattern as plugin source rules — user caught 'no BCs/no E-6 epic' gap;
  F-027 (incomplete defensive sweep) caused 2 wasted passes; lessons should land
  in agent prompts and consistency-validator."
- **s6.01-pass-1.md:** Origin document with 19 findings. F-001 (no BCs), F-003
  (no VP anchor story), F-011 (capability anchor gap), F-023 (incomplete fix
  propagation), F-027 (state-manager defensive sweep missed) — all trace back to
  absent agent-level enforcement.

## Goals

- **Prevent gap recurrence at agent-prompt level:** Story-writer must not produce
  a story without BCs/VPs/epic/FR. Product-owner must cite source-of-truth
  verbatim in every BC capability anchor. Adversary must always check whether
  prior-pass fixes propagated to sibling files.
- **Add tooling enforcement where prompts alone can't hold:** A new Bash lint hook
  (`validate-count-propagation.sh`) catches corpus-wide count drift (ARCH-INDEX
  vs STATE.md vs SS-NN files) without relying on agent memory.
- **Codify the meta-discipline:** A new rule file (`lessons-codification.md`)
  formalizes "every novel adversary catch → codification follow-up before cycle
  closure" so future cycles self-enforce without human reminder.
- **Close PARTIAL lessons:** VP multi-BC convention (lesson #4) documents the
  `source_bc=primary, bcs[]=full list` pattern in a template-compliance hook check.
- **Prevent TDD Iron Law bypass via precedent cascade (S-7.03):** Embed a verbatim
  anti-precedent guard in the stub-architect dispatch prompt, add a quantitative
  RED_RATIO density gate (≥ 0.5 RED before Step 4 implementer dispatch), introduce
  a `tdd_mode: strict|facade` contract in the story template, and add mutation
  testing at the wave gate for facade-mode stories as the compensating quality
  control. Root cause: Prism Wave 2 stub-architects used pre-implemented DTU clones
  as templates (aa706543, 6d2d005e), causing 3 of 5 Step 4 dispatches to be no-ops.

## PRD Capabilities Covered

| Capability ID | Name | Priority |
|--------------|------|----------|
| CAP-001 | Run a self-orchestrating LLM-driven SDLC pipeline | P1 |
| CAP-016 | Drive TDD delivery with red/green/refactor gate enforcement | P1 |

**Anchor justifications:**
- **CAP-001** describes the pipeline itself. S-7.01 and S-7.02 improve the
  pipeline's agent prompts and validation hooks — direct changes to the SDLC
  pipeline's behavioral rules. Per `specs/domain-spec/capabilities.md` §CAP-001.
- **CAP-016** governs TDD gate enforcement. S-7.03 delivers the anti-precedent
  guard, RED_RATIO density gate, tdd_mode contract, and mutation wave-gate — all
  direct implementations of CAP-016's "failing test must exist before implementation"
  mandate. Per `specs/domain-spec/capabilities.md` §CAP-016.

## Behavioral Contracts Covered

28 BCs across 4 subsystems (SS-05, SS-06, SS-07, SS-08).

| BC ID | Title | Story |
|-------|-------|-------|
| BC-5.36.001 | story-writer agent rejects status=ready when behavioral_contracts is empty | S-7.01 |
| BC-5.36.002 | story-writer requires AC↔BC bidirectional traces before marking a story ready | S-7.01 |
| BC-5.36.003 | product-owner agent requires Capability Anchor Justification cell on every BC | S-7.01 |
| BC-5.36.004 | product-owner cites capabilities.md verbatim in every capability anchor justification | S-7.01 |
| BC-5.36.005 | adversary explicitly checks partial-fix-regression for every finding closed in a prior pass | S-7.01 |
| BC-5.36.006 | adversary checks fix propagation to bodies, sibling files, and prose — not just frontmatter | S-7.01 |
| BC-5.36.007 | all three agent prompts updated atomically in single delivery; no partial update | S-7.01 |
| BC-5.37.001 | state-manager runs corpus-wide grep before declaring count change complete | S-7.02 |
| BC-5.37.002 | state-manager logs sweep results before declaring count-change complete | S-7.02 |
| BC-7.05.001 | validate-count-propagation.sh detects count drift across index files and exits non-zero | S-7.02 |
| BC-7.05.002 | validate-count-propagation.sh runs in under 200ms and is deterministic | S-7.02 |
| BC-7.05.003 | validate-template-compliance.sh enforces VP multi-BC source_bc convention | S-7.02 |
| BC-7.05.004 | hooks-registry.toml registers validate-count-propagation.sh as PostToolUse on index file writes | S-7.02 |
| BC-8.28.001 | rules/lessons-codification.md requires codification follow-up for every novel process catch | S-7.02 |
| BC-8.28.002 | orchestrator cycle-closing checklist references lessons-codification.md rule | S-7.02 |
| BC-5.38.001 | stub-architect commit must contain todo!()/unimplemented!() bodies for all non-trivial function implementations | S-7.03 |
| BC-5.38.002 | pure data mappings in stub commits may be implemented inline and must be flagged GREEN-BY-DESIGN | S-7.03 |
| BC-5.38.003 | framework integration wiring may have minimal real code for cargo check; handler business logic must be todo!() | S-7.03 |
| BC-5.38.004 | stub-architect must not use pre-implemented sibling crates as stub templates | S-7.03 |
| BC-5.38.005 | stub-architect applies self-check before committing any non-todo!() function body | S-7.03 |
| BC-5.38.006 | deliver-story SKILL.md and per-story-delivery.md Step 2 must contain anti-precedent guard text verbatim | S-7.03 |
| BC-8.29.001 | RED_RATIO = RED_TESTS / TOTAL_NEW_TESTS must be ≥ 0.5 before Step 4 implementer dispatch (BLOCKING) | S-7.03 |
| BC-8.29.002 | each non-RED test must be documented in red-gate-log with rationale before threshold relaxation | S-7.03 |
| BC-8.29.003 | on RED_RATIO < 0.5 without GREEN-BY-DESIGN justification, orchestrator must choose remediation option A or B | S-7.03 |
| BC-8.30.001 | story template must include tdd_mode field with strict\|facade enum and strict default | S-7.03 |
| BC-8.30.002 | tdd_mode=facade modifies per-story-delivery semantics and mandates mutation testing at wave gate | S-7.03 |
| BC-6.21.001 | wave-gate skill must run cargo mutants for every story with tdd_mode=facade in the wave | S-7.03 |
| BC-6.21.002 | mutation kill rate floor is 80%; surviving mutants must be addressed via test, dead-code confirmation, or explicit waiver | S-7.03 |

## Acceptance Criteria (Epic-Level)

| ID | Criterion | Validation Method |
|----|-----------|-------------------|
| EAC-001 | story-writer agent will not produce a story marked ready with empty behavioral_contracts | story-writer prompt includes spec-first gate; test: attempt to mark story ready without BCs and observe rejection |
| EAC-002 | product-owner BC files contain verbatim source citation in every capability anchor | PO prompt includes anchor-justification rule; adversary policy 5 enforces in next sub-cycle |
| EAC-003 | Adversary pass always includes a "prior-pass fix propagation" check section | Adversary prompt policy rule added; next sub-cycle pass-1 output includes this section |
| EAC-004 | state-manager runs corpus-wide grep before finalizing any count-changing update | state-manager prompt includes defensive-sweep protocol; lint hook provides automated backstop |
| EAC-005 | validate-count-propagation.sh exits non-zero when ARCH-INDEX / STATE.md / SS-NN counts diverge | Hook integration test; `hooks-registry.toml` wired |
| EAC-006 | lessons-codification.md rule is readable by orchestrator and referenced in cycle-closure checklist | Rule file present at `plugins/vsdd-factory/rules/lessons-codification.md` |
| EAC-007 | stub-architect dispatch prompt contains verbatim ANTI-PRECEDENT GUARD text with all four Prism SHAs | tdd-discipline-gate.bats test (a) passes; adversary static-check grep on deliver-story SKILL.md |
| EAC-008 | RED_RATIO density gate present in per-story-delivery.md between Step 3 and Step 4; blocks at < 0.5 | tdd-discipline-gate.bats test (d) passes; validate-red-ratio.sh exits non-zero on low-ratio log |
| EAC-009 | story-template.md has tdd_mode field; absent field treated as strict by all pipeline steps | tdd-discipline-gate.bats test (e) passes |
| EAC-010 | wave-gate SKILL.md runs cargo mutants for facade stories and blocks on kill rate < 80% | tdd-discipline-gate.bats test (f) passes; VP-064 procedural verification |

## Stories

| Story ID | Title | Points | Depends On | Status |
|----------|-------|--------|------------|--------|
| S-7.01 | Agent prompt updates for spec/anchor/adversary discipline | 5 | — | ready |
| S-7.02 | State-manager defensive sweep + count-propagation hook + meta-rule | 8 | — | ready |
| S-7.03 | TDD Discipline Hardening — Stub-as-Implementation Anti-Pattern Prevention | 8 | — | ready |

## Dependencies (External)

| System | Capability Needed | Readiness | Used By |
|--------|------------------|-----------|---------|
| `plugins/vsdd-factory/agents/` | Agent prompt Markdown files editable | Available | S-7.01, S-7.02, S-7.03 |
| `plugins/vsdd-factory/hooks/` | New Bash hook can be added | Available | S-7.02, S-7.03 |
| `plugins/vsdd-factory/hooks-registry.toml` | New hook can be registered | Available | S-7.02, S-7.03 |
| `plugins/vsdd-factory/rules/` | New rule file can be added | Available | S-7.02 |
| `plugins/vsdd-factory/skills/` | Skill Markdown files editable | Available | S-7.03 |
| `plugins/vsdd-factory/workflows/phases/` | Delivery workflow editable | Available | S-7.03 |
| `plugins/vsdd-factory/templates/` | Story template editable | Available | S-7.03 |
| `plugins/vsdd-factory/tests/` | BATS test files can be added | Available | S-7.03 |
| `cargo-mutants` toolchain | cargo install cargo-mutants | Install required | S-7.03 |
| ARCH-INDEX.md | Subsystem Registry for hook subsystem assignment | Available | S-7.01, S-7.02, S-7.03 |
