---
document_type: architecture-section
level: L3
section: "SS-06-skill-catalog"
version: "1.0"
status: accepted
producer: architect
timestamp: 2026-04-25T00:00:00
phase: 1.2
inputs:
  - .factory/specs/architecture/ARCH-INDEX.md
  - .factory/phase-0-ingestion/pass-1-architecture.md
  - .factory/phase-0-ingestion/pass-8-final-synthesis.md
traces_to: ARCH-INDEX.md
---

# SS-06: Skill Catalog

## [Section Content]

## Purpose

The Skill Catalog subsystem is the largest single behavioral surface in Subsystem B,
containing 119 named procedures invokable as `/vsdd-factory:<skill-name>` slash
commands. Each skill is a self-contained unit of behavior with its own `SKILL.md`,
optional `steps/` subdirectory of step-level instructions, and optional local
`templates/` for skill-specific output shapes. Skills are the atomic units of
artifact production in the VSDD pipeline; sub-agents called by the orchestrator
execute skills to produce specs, stories, architecture, code, and reports.

Skills span the full VSDD lifecycle: brownfield ingestion (`brownfield-ingest`),
brief creation and validation (`create-brief`, `validate-brief`), domain spec
production (`create-domain-spec`), PRD authoring (`create-prd`), architecture
design (`create-architecture`), story decomposition (`decompose-stories`), TDD
delivery (`deliver-story`), adversarial review (`adversarial-review`),
formal verification (`formal-verify`), wave-gate bookkeeping (`wave-gate`), and
release management (`release`). The 119 skills collectively implement the complete
VSDD 0–7 phase SDLC.

The skill catalog is intentionally a standalone subsystem because each `SKILL.md`
is a discrete behavioral contract — its inputs, outputs, and acceptance criteria
are independently specifiable and verifiable. This isolation enables the BC count
for this subsystem (largest single BC surface in vsdd-factory; see ARCH-INDEX
Subsystem Registry table for current count) to scale with skill count rather than
being entangled with workflow or agent concerns.

## Modules

| Module / File | Responsibility |
|---|---|
| `plugins/vsdd-factory/skills/*/SKILL.md` (119 files) | Primary skill definition: frontmatter (`name`, `description`, optional `argument-hint`), procedure steps, acceptance criteria |
| `plugins/vsdd-factory/skills/*/steps/*.md` | Per-step detailed instructions for multi-step skills |
| `plugins/vsdd-factory/skills/*/templates/` | Skill-local output templates (overrides or supplements global templates in SS-08) |
| `plugins/vsdd-factory/commands/*.md` (110 files) | Slash-command binding files: `/vsdd-factory:<name>` → invokes the named skill |

**Selected skills by phase (not exhaustive):**

| Skill | Phase | Purpose |
|---|---|---|
| `brownfield-ingest` | Phase 0 | Multi-pass codebase analysis; produces pass-N-*.md files |
| `create-domain-spec` | Phase 1 | Produce L2 domain spec from brief + ingestion output |
| `create-prd` | Phase 1 | Produce L3 PRD from domain spec |
| `create-architecture` | Phase 1 | Produce L3 architecture index + section files |
| `decompose-stories` | Phase 2 | Decompose BCs into implementable stories |
| `deliver-story` | Phase 3 | TDD implementation of a story with green/red/refactor cycle |
| `adversarial-review` | Phase 5 | Adversarial spec + code review with SHA-currency gate |
| `formal-verify` | Phase 6 | Formal verification using Kani/proptest/fuzz |
| `wave-gate` | Cross-phase | Quality bookkeeping; records gate outcomes to STATE.md |
| `release` | Phase 7 | Semver bump + CHANGELOG + binary commit + marketplace publish |
| `activate` | Setup | Platform detection + hooks.json variant copy + binary verification |
| `factory-health` | Ops | Health check of factory state, STATE.md, wave gates |
| `check-state-health` | Ops | Validate STATE.md integrity and consistency |

## Public Interface

**Slash command surface:** Every skill is reachable as `/vsdd-factory:<skill-name>`.
110 of 119 skills have a corresponding `commands/<skill-name>.md` binding. The
binding file declares the slash command name and maps it to the skill invocation.

**Skill frontmatter schema:**
```yaml
name: deliver-story
description: "TDD implementation of a story: red → green → refactor."
argument-hint: "<story-id>"
```

**Skill contract (behavioral):** Each skill declares:
- What it reads (input files, STATE.md fields, CLI arguments).
- What it writes (output artifact paths, STATE.md mutations).
- Acceptance criteria (what constitutes a completed run).
- Composition: which other skills or templates it invokes.

Skills emit events via `bin/emit-event` (SS-10) to record progress to the
dispatcher event stream (second-class emission path; S-3.4 PARTIAL).

## Internal Structure

Skill directory convention (pass-5-conventions.md, §Plugin layer conventions):

```
skills/<skill-name>/
├── SKILL.md                # primary definition (always present)
├── steps/                  # optional; one .md per step
│   ├── 01-read-inputs.md
│   ├── 02-analyze.md
│   └── 03-write-output.md
└── templates/              # optional skill-local templates
    └── output-template.md
```

119/119 skills conform to the frontmatter convention. Skills compose each other
freely (e.g., `deliver-story` invokes `wave-gate`; `create-architecture` invokes
`create-prd` outputs as inputs). There is no circular dependency constraint
enforced by tooling — the skill author is responsible for acyclic composition.

Skills that produce `.factory/specs/` artifacts follow VSDD naming conventions:
`YYYY-MM-DD-slug` for dated docs, `L<N>-<artifact>` for level-tagged specs,
`BC-S.SS.NNN` for behavioral contracts. Slash commands follow the kebab-case
convention matching the skill directory name.

The 119 skills were analyzed in 3 alphabetical batches during Phase 0 ingestion
(pass-3-deep-skills-batch-1/2/3.md). All 119 are covered at 100% per the Phase 0
convergence report (pass-8-final-synthesis.md §9).

## Dependencies

**Incoming (consumers of SS-06):**
- SS-05 (Pipeline Orchestration) — orchestrator dispatches sub-agents who invoke
  skills; skills are called from workflow steps.
- User — invokes skills directly via `/vsdd-factory:<name>` slash commands.

**Outgoing (SS-06 depends on):**
- SS-08 (Templates and Rules) — skills render output using the global template
  catalog; `templates/` subdirs in skills override global templates.
- SS-10 (CLI Tools and Bin) — skills call `bin/emit-event`, `bin/wave-state`,
  `bin/factory-dashboard`, and other bin tools for state management and
  observability.

## Cross-Cutting

- **Observability:** Skills emit progress events via `bin/emit-event` shell tool.
  This is the bash-side second-class emission path (S-3.4 PARTIAL). Native WASM
  port will replace this with direct `vsdd::emit_event` host fn calls.
- **Template rendering:** Skill output shapes are governed by templates from SS-08.
  Skills must not hardcode output structure that belongs in a template — the
  template is the single source of truth for artifact shape.
- **Wave-gate integration:** Skills that complete a phase milestone call the
  `wave-gate` skill which writes gate outcome to STATE.md. SS-07 hook
  `validate-wave-gate-state.sh` validates STATE.md integrity on every write.
- **Argument handling:** Skills with `argument-hint` in frontmatter accept a
  single positional argument from the slash command. Complex multi-argument flows
  use STATE.md as the passing mechanism rather than CLI args.
- **Error handling:** Skills are prose-driven; error handling is expressed as
  conditional steps ("if X fails, do Y"). No programmatic error type system.

## Behavioral Contracts

BC shard directory: `.factory/specs/behavioral-contracts/ss-06/`
(target prefix BC-6; current BC count in ARCH-INDEX Subsystem Registry).

BC numbering uses two coexisting schemes within the BC-6 prefix:

1. **Flat BC-6.NNN namespace** for skill behaviors extracted in Phase 0
   ingestion (~120 skills covered): brownfield ingestion (BC-6.001–6.040),
   spec-crystallization — brief/domain-spec/PRD/architecture (BC-6.041–6.120),
   story lifecycle — decompose/deliver/review (BC-6.121–6.200),
   wave-gate and state-management (BC-6.201–6.250),
   release and activation (BC-6.251–6.300),
   observability and ops (BC-6.301–6.350),
   adversarial and formal-verification (BC-6.351–6.400),
   remaining skill contracts (BC-6.401–onward).
2. **Skill-scoped sub-namespace BC-6.NN.NNN** for new skills authored
   post-Phase 0 (e.g., `BC-6.20.NNN` = create-adr skill from S-6.01).
   Each new skill packs its BCs under a fresh `NN` section.

For the authoritative current BC count and per-section breakdown, consult
BC-INDEX.md and ARCH-INDEX Subsystem Registry — those are the sources of
truth; this prose is a high-level orientation only.

## ADRs

No SS-06-specific ADRs. Skill catalog design follows the conventions established
by the orchestration framework; see ADR-013 for adversarial review structure which
governs the `adversarial-review` skill's SHA-currency requirement.

## Drift / Known Issues

- **S-3.4 PARTIAL:** Skills emit events via `bin/emit-event` shell tool (SS-10),
  not directly through the `vsdd::emit_event` host fn. The two emission paths
  write to different internal structures. Full reconciliation pending S-3.4
  completion.
- **L-P0-003 (debt):** The 24+ `validate-*` hooks (SS-07) have no formal BCs.
  Phase 1 backfill must create one or more BC-7.NNN entries per validator script,
  treating the script body as authoritative source.
- **L-P2-002 (debt):** 110 slash commands are a flat directory with no navigable
  index. Catalog cross-reference to skills is by naming convention only.
