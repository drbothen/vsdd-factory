---
name: artifact-detection
description: >
  Scans the project for existing planning artifacts (brief, PRD, architecture,
  UX spec, stories), validates their quality, identifies gaps, and routes to
  the correct pipeline entry point. This is the universal front-end for the
  VSDD pipeline — it replaces the assumption that the human always arrives
  with a finished product brief.
---

# Artifact Detection & Routing

## When This Skill Runs

This skill runs at the START of every pipeline invocation, before spec
crystallization begins. It replaces the former first step ("Receive Product Brief").

## Workflow

### Step 1: Scan for Existing Artifacts

Search for planning artifacts in the project:

| Artifact | Search Patterns | Required For |
|----------|----------------|-------------|
| Product Brief (L1) | `**/product-brief.md`, `**/brief.md`, `**/*brief*` | Spec entry point |
| Domain Spec (L2) | `.factory/specs/domain-spec/L2-INDEX.md` | L2 capability mapping |
| PRD | `**/prd.md`, `**/PRD.md`, `.factory/specs/prd.md` | Skip to architecture design |
| Behavioral Contracts (L3) | `.factory/specs/behavioral-contracts/BC-INDEX.md`, `.factory/specs/behavioral-contracts/BC-*.md` | L3 per-file contracts |
| Verification Properties (L4) | `.factory/specs/verification-properties/VP-INDEX.md`, `.factory/specs/verification-properties/VP-*.md` | L4 per-file properties |
| Architecture | `**/architecture/ARCH-INDEX.md`, `.factory/specs/architecture/ARCH-INDEX.md`, `.factory/specs/architecture/` | Story decomposition entry |
| Architecture Feasibility | `.factory/specs/architecture-feasibility-report.md` | Architect review |
| Verification Architecture | `.factory/specs/architecture/verification-architecture/ARCH-INDEX.md` | Story decomposition entry |
| Adversarial Reviews | `.factory/specs/adversarial-reviews/`, `.factory/cycles/**/adversarial-reviews/adversarial-reviews/` | Sharded review directories |
| Evaluations | `.factory/holdout-scenarios/evaluations/EVAL-INDEX.md` | Sharded evaluation directory |
| PRD Supplements | `.factory/specs/prd-supplements/` | Supplement directory |
| UX Spec | `**/ux-spec/UX-INDEX.md`, `**/ux-design.md`, `.factory/specs/ux-spec/UX-INDEX.md` | Story decomposition entry (optional) |
| Epics/Stories | `.factory/stories/epics.md`, `.factory/stories/stories/` | Implementation entry |
| Project Context | `**/project-context.md` | Brownfield context (DF-005) |

Also check for artifacts the human may have provided outside the `.factory/` structure:
- Documents in the project root or `docs/` directory
- Files the human explicitly mentions in their initial message

### Step 1b: Detect Format and Structure

Detect specification format (DF-020 4-level hierarchy):
- **Current format:** BC-S.SS.NNN (behavioral contracts with subsystem numbering)
- **Legacy format:** FR-NNN (flat functional requirement numbering)
- If FR-NNN detected, flag for migration before proceeding

Detect architecture structure (DF-021 sharding):
- **New format:** `ARCH-INDEX.md` + 7 section files in `architecture/` directory
- **Old format:** single `architecture/ARCH-INDEX.md` file
- Handle both formats gracefully; flag old format for migration

For partial specs using 4-level hierarchy: detect level correctly.
If spec uses old FR-NNN format: flag for migration before proceeding.

### Step 2: Classify Readiness Level

Based on what exists, classify the project:

| Level | Artifacts Found | Route To |
|-------|----------------|----------|
| **L0: Nothing** | No planning artifacts | Collaborative Discovery |
| **L1: Brief** | Product brief exists | Validate brief -> spec crystallization |
| **L2: PRD** | PRD exists (may include L2 domain spec) | Validate PRD -> remaining spec crystallization |
| **L3: PRD + Architecture** | PRD + architecture + L3 BCs exist | Validate both -> story decomposition |
| **L4: Full Spec** | PRD + architecture + L3 BCs + L4 VPs + stories exist | Validate all -> implementation |

### Step 3: Validate What Exists

For each artifact found, run the appropriate validation:

**Brief validation:**
- Contains all required sections (see Product Brief Definition)
- Not just a title or one-liner -- has substantive content
- Scope is defined (not just "build everything")

**PRD validation:**
- Has numbered functional requirements (FR-XXX)
- Has numbered non-functional requirements (NFR-XXX) with numerical targets
- Has measurable success criteria
- Has edge case catalog with boundary conditions
- Requirements are SMART
- Acceptance criteria use behavioral descriptions
- No vague language
- **Bloat check:** PRD is not over-detailed
- **Token budget estimate:** Total PRD fits within 30% of implementing agent's context window

**Architecture validation:**
- Has ARCH-INDEX.md with document map and cross-references (DF-021)
- Has all expected section files (system-overview, module-decomposition, dependency-graph,
  api-surface, verification-architecture, purity-boundary-map, tooling-selection,
  verification-coverage-matrix)
- Each section file has `traces_to: ARCH-INDEX.md` in frontmatter
- Has component inventory with responsibilities (in module-decomposition.md)
- Has technology stack with explicit version constraints and justification
- Has data model with relationships
- Has purity boundary map (pure core vs effectful shell, in purity-boundary-map.md)
- Has verification properties catalog for critical modules
- **Machine-readable architectural map** in structured format (YAML/JSON)
- **ADRs documented** with context, rationale, and consequences
- No circular dependencies in the component graph

**Story validation:**
- Every PRD requirement covered by at least one story
- Every story has acceptance criteria as numbered behavioral assertions
- Dependency graph is acyclic
- No story exceeds 13 story points
- **Token budget:** Each story fits within 20-30% of implementing agent's context window
- **Story content sweet spot:** 300-800 tokens

### Step 4: Gap Analysis

For each validation that fails, record the specific gap:

```markdown
## Gap Analysis Report

| Artifact | Status | Gaps |
|----------|--------|------|
| Brief | VALID / INCOMPLETE / MISSING | [specific gaps] |
| PRD | VALID / INCOMPLETE / MISSING | [specific gaps] |
| Architecture | VALID / INCOMPLETE / MISSING | [specific gaps] |
| Stories | VALID / INCOMPLETE / MISSING | [specific gaps] |
```

Write gap analysis to `.factory/planning/gap-analysis.md`.

### Step 5: Route Decision

Present the gap analysis to the human with routing options:

**If L0 (nothing exists):**
"No planning artifacts found. Would you like to:
1. Start with brainstorming (Collaborative Discovery -- recommended if exploring ideas)
2. Start with a product brief (Collaborative Discovery -- recommended if you know what you want)
3. Provide an existing document for me to analyze"

**If artifacts exist with gaps:**
"I found [artifacts]. Here are the gaps: [gap list].
Would you like to:
1. Fix the gaps interactively (I'll guide you through each one)
2. Proceed with what we have (gaps will be flagged but won't block)
3. Start fresh (discard existing artifacts and go through full discovery)"

**If all artifacts are valid:**
"All planning artifacts are complete and valid. Ready to proceed to [next phase]."

## Step-File Decomposition

**Directory:** `workflows/skills/artifact-detection/steps/`

| File | Step |
|------|------|
| `step-01-scan-artifacts.md` | Scan for Existing Artifacts |
| `step-02-classify-readiness.md` | Classify Readiness Level |
| `step-03-validate-artifacts.md` | Validate What Exists |
| `step-04-gap-analysis.md` | Gap Analysis |
| `step-05-route-decision.md` | Route Decision |

## Output Artifacts

- `.factory/planning/artifact-inventory.md` -- what was found
- `.factory/planning/gap-analysis.md` -- validation results and gaps
- `.factory/planning/routing-decision.md` -- which entry point was selected

## Failure Modes

- If `.factory/` does not exist: report L0 (nothing exists) and route to Collaborative Discovery
- If artifacts are found but corrupted (unparseable frontmatter, truncated content): flag specific corruption and recommend re-creation of affected artifacts
- If artifacts use legacy FR-NNN format: flag for migration before proceeding, do not silently skip

