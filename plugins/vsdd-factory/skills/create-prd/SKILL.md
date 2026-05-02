---
name: create-prd
description: Create a PRD with behavioral contracts from the product brief and domain spec. Elaborates requirements into testable contracts with error taxonomy and edge cases. Writes to .factory/specs/prd.md and supplements.

allowed-tools: Read, Write, Edit, Bash, AskUserQuestion
---

## Hard Gate

Do NOT skip to architecture design or story decomposition. Every behavioral contract MUST be defined with testable preconditions and postconditions before proceeding.

# Create PRD

Transform the product brief (and optional domain spec) into a comprehensive PRD with behavioral contracts.

## Templates

Read and follow the output format in:
- `${CLAUDE_PLUGIN_ROOT}/templates/prd-template.md` — PRD structure
- `${CLAUDE_PLUGIN_ROOT}/templates/behavioral-contract-template.md` — BC-S.SS.NNN format
- `${CLAUDE_PLUGIN_ROOT}/templates/prd-supplement-error-taxonomy-template.md` — error taxonomy
- `${CLAUDE_PLUGIN_ROOT}/templates/prd-supplement-interface-definitions-template.md` — interface definitions
- `${CLAUDE_PLUGIN_ROOT}/templates/prd-supplement-nfr-catalog-template.md` — non-functional requirements
- `${CLAUDE_PLUGIN_ROOT}/templates/module-criticality-template.md` — module criticality classification

## Prerequisites

- `.factory/specs/product-brief.md` must exist. Read it first.
- `.factory/specs/domain-spec/L2-INDEX.md` is optional but recommended. If present, read the index and all section files.
- Check `.factory/specs/research/RESEARCH-INDEX.md` for all research. Read both domain and general research reports — domain research informs requirements, general research informs technical feasibility and library choices.

### Reference Repos (conditional)

If `.factory/reference-manifest.yaml` exists, this project derived requirements from ingested codebases. Before writing BCs:
- Read `.factory/semport/` synthesis files to understand what behaviors were extracted.
- When a BC traces to an ingested repo's behavior, reference the source: `Source: <project>/<file>:<function>` in the BC's Traceability section.
- Use `.reference/<project>/` to verify behavioral claims against actual code when the semport summary is ambiguous.

## Process

### 1. Identify Subsystems

From the brief and domain spec, identify the major subsystems. Each gets a subsystem number (01–99) for behavioral contract numbering.

Ask the user to validate the subsystem breakdown.

### 2. For Each Subsystem, Define Sections

Break each subsystem into logical sections (01–99). Each section groups related behaviors.

### 3. Write Behavioral Contracts

For each section, write individual behavioral contracts (BC-S.SS.NNN) following the format in `spec-format.md` rule.

Each BC must be:
- **Testable** — you can write a test that verifies it
- **Unambiguous** — only one interpretation
- **Complete** — preconditions, postconditions, and error cases defined

### 4. Define Error Taxonomy

Create `.factory/specs/prd-supplements/error-taxonomy.md`:
- Domain-specific error codes
- Error categories and severity
- Recovery strategies per error type
- User-facing vs internal error messages

### 5. Define Interface Contracts

Create `.factory/specs/prd-supplements/interface-definitions.md`:
- Public API surface (CLI commands, API endpoints, library exports)
- Input/output formats
- Type definitions

### 6. Classify Module Criticality

Create `.factory/specs/prd-supplements/module-criticality.md`:
- CRITICAL: Security, data integrity, financial
- HIGH: Core business logic
- MEDIUM: Supporting features
- LOW: Convenience, cosmetic

Criticality determines review depth, test coverage requirements, and holdout scenario density.

## Output

### Core PRD (`.factory/specs/prd.md`)

```markdown
# Product Requirements Document: <Product Name>

**Date:** <current date>
**Status:** draft
**Source:** product-brief.md, domain-spec/L2-INDEX.md

## Executive Summary
<Brief product description and value proposition>

## Subsystems

| ID | Name | Description | Criticality |
|----|------|-------------|-------------|
| 01 | ... | ... | CRITICAL/HIGH/MEDIUM/LOW |

## Requirements by Subsystem

### Subsystem 01: <Name>

#### Section 01: <Name>
- BC-1.01.001: <title> — see behavioral-contracts/ss-01/BC-1.01.001.md
- BC-1.01.002: <title>

## Cross-Cutting Concerns
<Requirements that span subsystems>

## Non-Functional Requirements
<Performance, security, accessibility, observability>

## Assumptions
<Explicit assumptions — things we believe true but haven't verified>

## Open Questions
<Things that need resolution>
```

### Individual BCs (`.factory/specs/behavioral-contracts/ss-NN/BC-S.SS.NNN.md`)

One file per contract, sharded into per-subsystem `ss-NN/` directories,
following `spec-format.md` format. Shard directory name is the bare
`SS-NN` identifier (lowercased) — descriptive subsystem names live in
ARCH-INDEX Subsystem Registry, not in the directory name.

### BC Index (`.factory/specs/behavioral-contracts/BC-INDEX.md`)

Table of all contracts with status tracking.

## Self-Review (before adversarial review)

Before routing to adversarial review, check your own work:

1. **Placeholder scan:** Any "TBD", "TODO", incomplete BCs, or vague preconditions/postconditions? Fix them now.
2. **Internal consistency:** Do BC IDs match the PRD index? Do subsystem numbers align across files? Does the error taxonomy cover all error cases referenced in BCs?
3. **Scope check:** Is each BC focused on a single behavior, or are any trying to cover multiple concerns?
4. **Ambiguity check:** Could any precondition or postcondition be interpreted two different ways? Pick one and make it explicit.

Fix issues inline. This is a cheap filter — catch obvious gaps before spending tokens on the adversary.

## After Writing

1. Commit all files to factory-artifacts.
2. Tell the user: "PRD created with <N> behavioral contracts across <N> subsystems. Next: `/create-architecture` to design the system, or `/adversarial-review` to stress-test the specs first."
