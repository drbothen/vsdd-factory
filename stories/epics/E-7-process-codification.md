---
document_type: epic
epic_id: "E-7"
version: "1.0"
prd_capabilities: [CAP-001]
prd_frs: [FR-042]
status: open
story_count: 2
producer: story-writer
timestamp: 2026-04-26T00:00:00
phase: 2
inputs:
  - .factory/STATE.md
  - .factory/stories/S-6.01-create-adr-skill.md
  - .factory/cycles/v1.0-brownfield-backfill/adversarial-reviews/s6.01-pass-1.md
input-hash: ""
traces_to: .factory/STATE.md#Lessons-Learned
---

# Epic E-7: Process Codification (Self-Improvement)

## Description

vsdd-factory improving vsdd-factory. E-7 captures process gaps that surfaced
during the S-6.01 adversarial sub-cycle (8 passes, 27 findings closed,
19→4→2→1→1→0→0→0 trajectory). Several findings were not one-off content
defects — they were recurring failures in agent prompt discipline, defensive
propagation sweeps, and meta-process rules. E-7 codifies those lessons into
plugin source (agent prompts, lint hooks, rule files) so future spec sub-cycles
don't re-discover the same gaps.

This is a "self-referential dogfooding" epic: the factory's adversarial process
caught its own process deficiencies, and those deficiencies become E-7 stories.
The stories in this epic are themselves authored using the lessons they codify —
spec-first, BC-anchored, with proper subsystem labeling — to validate that the
lessons land correctly.

Milestone: `v1.0.x` (process hardening, parallel with Tier E implementation).
Subsystems: SS-05 (Pipeline Orchestration), SS-07 (Hook Bash Layer),
SS-08 (Templates and Rules).

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

## PRD Capabilities Covered

| Capability ID | Name | Priority |
|--------------|------|----------|
| CAP-001 | Run a self-orchestrating LLM-driven SDLC pipeline | P1 |

**Anchor justification:** CAP-001 describes the pipeline itself. These stories
improve the pipeline's agent prompts and validation hooks — they are direct
changes to the SDLC pipeline's behavioral rules. No other CAP more precisely
describes "updating agent prompts and hooks that govern the pipeline." Per
`specs/domain-spec/capabilities.md` CAP-001.

## Behavioral Contracts Covered

15 BCs across 4 subsystems (SS-05, SS-07, SS-08).

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

## Acceptance Criteria (Epic-Level)

| ID | Criterion | Validation Method |
|----|-----------|-------------------|
| EAC-001 | story-writer agent will not produce a story marked ready with empty behavioral_contracts | story-writer prompt includes spec-first gate; test: attempt to mark story ready without BCs and observe rejection |
| EAC-002 | product-owner BC files contain verbatim source citation in every capability anchor | PO prompt includes anchor-justification rule; adversary policy 5 enforces in next sub-cycle |
| EAC-003 | Adversary pass always includes a "prior-pass fix propagation" check section | Adversary prompt policy rule added; next sub-cycle pass-1 output includes this section |
| EAC-004 | state-manager runs corpus-wide grep before finalizing any count-changing update | state-manager prompt includes defensive-sweep protocol; lint hook provides automated backstop |
| EAC-005 | validate-count-propagation.sh exits non-zero when ARCH-INDEX / STATE.md / SS-NN counts diverge | Hook integration test; `hooks-registry.toml` wired |
| EAC-006 | lessons-codification.md rule is readable by orchestrator and referenced in cycle-closure checklist | Rule file present at `plugins/vsdd-factory/rules/lessons-codification.md` |

## Stories

| Story ID | Title | Points | Depends On | Status |
|----------|-------|--------|------------|--------|
| S-7.01 | Agent prompt updates for spec/anchor/adversary discipline | 5 | — | ready |
| S-7.02 | State-manager defensive sweep + count-propagation hook + meta-rule | 8 | — | ready |

## Dependencies (External)

| System | Capability Needed | Readiness |
|--------|------------------|-----------|
| `plugins/vsdd-factory/agents/` | Agent prompt Markdown files editable | Available |
| `plugins/vsdd-factory/hooks/` | New Bash hook can be added | Available |
| `plugins/vsdd-factory/hooks-registry.toml` | New hook can be registered | Available |
| `plugins/vsdd-factory/rules/` | New rule file can be added | Available |
| ARCH-INDEX.md | Subsystem Registry for hook subsystem assignment | Available |
