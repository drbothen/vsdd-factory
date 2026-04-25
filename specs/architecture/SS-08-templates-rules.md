---
document_type: architecture-section
level: L3
section: "SS-08-templates-rules"
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

# SS-08: Templates and Rules

## [Section Content]

## Purpose

The Templates and Rules subsystem provides the output-shape skeletons and
cross-cutting policy files that govern the structure of every artifact the VSDD
pipeline produces. Templates ensure that skills produce consistently shaped
artifacts regardless of which agent or which project invokes them. Rules enforce
cross-cutting policies that apply across all agents, skills, and workflows.

The template catalog (108 files in the top-level `templates/` directory, plus
subdirectories) covers the full artifact inventory: behavioral contract templates
(`behavioral-contract-template.md`, `L4-verification-property-template.md`),
architecture templates (`architecture-index-template.md`,
`architecture-section-template.md`, `module-criticality-template.md`), PRD and
domain-spec templates, ADR templates, story templates, agent templates, wave-gate
and adversarial-review templates, and observability shell templates
(`verify-sha-currency.sh`). Each template defines the required frontmatter schema,
required H2 headings, and structural conventions that the `validate-template-compliance.sh`
hook (SS-07) checks on every Write/Edit.

Rules (9 files in `rules/`) encode policies such as: no AI attribution in commits,
factory branch guard rules, brownfield discipline rules, wave-gate sequencing rules.
They are referenced by agents and skills as authoritative policy sources. Rules are
prose markdown; they are not executable, but their enforcement is implemented in
the corresponding bash gate hooks in SS-07.

## Modules

| Module / File | Responsibility |
|---|---|
| `plugins/vsdd-factory/templates/behavioral-contract-template.md` | BC document shape: frontmatter, pre/post/invariants/events sections |
| `plugins/vsdd-factory/templates/L4-verification-property-template.md` | Verification property shape: proof harness skeleton, feasibility |
| `plugins/vsdd-factory/templates/architecture-index-template.md` | ARCH-INDEX shape: subsystem registry, document map, dependency graph |
| `plugins/vsdd-factory/templates/architecture-section-template.md` | Architecture section shape: required `## [Section Content]` heading and frontmatter |
| `plugins/vsdd-factory/templates/module-criticality-template.md` | Module criticality classification table shape |
| `plugins/vsdd-factory/templates/verification-coverage-matrix-template.md` | VP-to-module coverage matrix shape |
| `plugins/vsdd-factory/templates/verification-architecture-template.md` | Provable properties catalog and P0/P1 list shape |
| `plugins/vsdd-factory/templates/prd-template.md` | PRD L3 shape: BC tables, NFR catalog, subsystem tables |
| `plugins/vsdd-factory/templates/domain-spec-*.md` | Domain spec L2 shapes: capabilities, entities, invariants, events, risks |
| `plugins/vsdd-factory/templates/adr-template.md` | ADR shape: context, decision, rationale, consequences |
| `plugins/vsdd-factory/templates/story-template.md` | Story shape: acceptance criteria, BC links, implementation notes |
| `plugins/vsdd-factory/templates/pr-description-template.md` | PR description shape |
| `plugins/vsdd-factory/templates/recovered-architecture-template.md` | Brownfield recovery architecture shape |
| `plugins/vsdd-factory/templates/dtu-assessment-template.md` | DTU assessment shape |
| `plugins/vsdd-factory/templates/gene-transfusion-assessment-template.md` | Gene transfusion assessment shape |
| `plugins/vsdd-factory/templates/verify-sha-currency.sh` | Opt-in SHA-currency gate template (copy to hooks/ to activate; DRIFT-009) |
| `plugins/vsdd-factory/rules/*.md` (9 files) | Cross-cutting policy files: no-ai-attribution, factory-branch, brownfield-discipline, wave-gate-sequencing, etc. |

## Public Interface

Templates are consumed by skills and agents through prose references: a skill's
`SKILL.md` instructs the agent to "use `templates/<name>.md` as the output shape."
Templates are not programmatically rendered — agents read them and produce
conforming output by instruction.

The `validate-template-compliance.sh` hook (SS-07) provides the programmatic
enforcement layer. It does a case-insensitive grep for each required heading
defined in the template's own structure. Required headings are determined by the
template's own `## ` headings in the template body. The hook fires PostToolUse
on Write/Edit and emits a WARNING event if any required heading is absent from
the produced file.

Rules are referenced by agents in their persona markdown and by skill steps. The
no-ai-attribution rule governs `block-ai-attribution.sh` (SS-07). The
brownfield-discipline rule governs `brownfield-discipline.sh` (SS-07).

**Template naming conventions:**
- `<artifact>-template.md` — single artifact templates.
- `L<N>-<artifact>-template.md` — level-tagged templates (e.g., `L4-verification-property-template.md`).
- `architecture-*-template.md` — architecture family.
- `domain-spec-*` — domain spec family.

## Internal Structure

Templates are static markdown files. No templating engine. The content inside
templates uses `[PLACEHOLDER]` markers to indicate where the agent fills in
content. Required H2 headings define the structural contract enforced by the
validator hook.

Some templates include literal heading text that the validator checks for
(e.g., `## [Section Content]` in `architecture-section-template.md`). When a
heading contains brackets, the validator checks for the literal bracketed string.
Agents writing conforming files must include the literal heading in their output —
the brackets are part of the required string, not a meta-notation.

Rules files are numbered-section prose. They do not have machine-checkable
structure; their enforcement is through the corresponding gate hooks. Rule
updates require updating both the rule file and any corresponding bash hook that
implements the rule.

Template subdirectory structure (108 files total at top level; additional files
in subdirs such as `templates/hooks/`, `templates/skills/`, `templates/agents/`):
```
templates/
├── *.md                     # primary artifact templates
├── hooks/                   # hook-specific templates
├── skills/                  # skill-specific templates
└── agents/                  # agent-specific templates
```

## Dependencies

**Incoming (consumers of SS-08):**
- SS-06 (Skill Catalog) — skills reference global templates by name; skill-local
  `templates/` dirs override global templates for that skill's output.
- SS-07 (Hook Bash Layer) — `validate-template-compliance.sh` greps produced
  files against required headings derived from templates.
- SS-05 (Pipeline Orchestration) — agents reference rules files as policy
  authorities in their persona markdown.

**Outgoing (SS-08 depends on):**
- None. Templates and rules are static files. No dependencies on other subsystems.

## Cross-Cutting

- **Compliance enforcement:** `validate-template-compliance.sh` (SS-07) is the
  runtime enforcement mechanism. It fires PostToolUse on Write and Edit, checks
  for required headings, and emits WARNING events on violations. Violations do not
  block tool calls (exit 0); they are advisory.
- **Versioning:** Templates are versioned with the plugin release. Breaking
  template changes (removed required headings, renamed frontmatter fields) require
  a skill update to produce conforming output. No automated migration.
- **Template vs rule relationship:** Templates govern artifact shape. Rules govern
  agent behavior. They are distinct: a template defines what a BC file looks like;
  a rule defines that BCs must exist before implementation. The validator enforces
  templates; gate hooks enforce rules.
- **Observability:** Template compliance warnings are emitted as events via
  SS-07's `_emit` helper, landing in the dispatcher event stream.

## Behavioral Contracts

BC shard directory: `.factory/specs/behavioral-contracts/ss-08/`
(target prefix BC-8; ~130 BCs across 108 templates + 9 rules).

High-level BC groupings: architecture template family — required headings and
frontmatter (BC-8.001–BC-8.025), BC and VP template family (BC-8.026–BC-8.050),
spec templates — PRD, domain-spec, ADR, story (BC-8.051–BC-8.080), operational
templates — DTU, gene-transfusion, wave-gate (BC-8.081–BC-8.100), rules policy
contracts (BC-8.101–BC-8.115), template compliance validator contracts
(BC-8.116–BC-8.130).

## ADRs

No SS-08-specific ADRs. Template and rules design follows the VSDD methodology
conventions; no architectural decision records were created for the template
catalog specifically.

## Drift / Known Issues

- **Phase 0 count discrepancy:** Pass-8-final-synthesis.md §9 reports 105 top-level
  templates vs ARCH-INDEX `108 files + subdirs`. Difference is within the ±3
  delta noted in the Phase 0 convergence report. Not a correctness gap.
- **DRIFT-009 (P2 — low):** `templates/verify-sha-currency.sh` exists as an
  opt-in template but is not automatically installed into `hooks/`. Projects that
  have not copied it get no SHA-currency enforcement on adversarial review.
  Acceptable for 1.0 if documented clearly.
- **No machine-checkable rule structure:** Rules are prose only. Enforcement is
  entirely through bash gate hooks that implement each rule. A rule update that
  is not reflected in the corresponding hook creates silent drift.
