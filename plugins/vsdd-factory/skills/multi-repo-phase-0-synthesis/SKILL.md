---
name: multi-repo-phase-0-synthesis
description: >
  Synthesizes per-repo codebase ingestion outputs into a unified project-level
  context. Runs after all individual repo ingestions complete. Produces cross-repo
  dependency graph, unified architecture, convention reconciliation, unified
  security posture, cross-repo holdout scenarios, and unified project context.
agents:
  primary: codebase-analyzer
  supporting: [architect, consistency-validator, security-reviewer, product-owner, adversary, state-manager]
inputs:
  - Per-repo .factory/phase-0-ingestion/ artifacts (all repos)
  - project.yaml
outputs:
  - .factory-project/phase-0-synthesis/cross-repo-dependencies.md
  - .factory-project/phase-0-synthesis/unified-architecture.md
  - .factory-project/phase-0-synthesis/convention-reconciliation.md
  - .factory-project/phase-0-synthesis/unified-security-posture.md
  - .factory-project/cross-repo-holdout-scenarios/*.md
  - .factory-project/phase-0-synthesis/project-context.md
  - .factory-project/phase-0-synthesis/synthesis-validation-report.md
gate: Human reviews and approves unified project context
---

# Multi-Repo Phase 0 Synthesis

This skill synthesizes per-repo codebase ingestion outputs into a unified
project-level context document. It runs after all individual repo ingestions
complete successfully.

## Prerequisites

- All brownfield repos have completed DF-031 upgraded codebase ingestion (all 12 steps)
- Each repo has `.factory/phase-0-ingestion/project-context.md` (ingestion output)
- `project.yaml` is present and parsed

## Synthesis Steps

### Step 1: Cross-Repo Dependency Graph

**Agent:** codebase-analyzer (T1, via state-manager for writes)

Analyze per-repo artifacts to produce a cross-repo dependency graph:
- How repos relate at the code level (imports, API calls, shared types)
- Cross-repo API contract extraction (interfaces between repos)
- Data flow between repos (which data crosses repo boundaries)

**Output:** `.factory-project/phase-0-synthesis/cross-repo-dependencies.md`

### Step 2: Unified Architecture

**Agent:** architect (T2)

Produce a unified architecture view spanning all repos:
- Combined system architecture diagram
- Cross-repo module criticality (which cross-repo interfaces are critical)
- Integration points and failure modes
- Per-repo architecture summaries with cross-references

**Output:** `.factory-project/phase-0-synthesis/unified-architecture/ARCH-INDEX.md`

### Step 3: Convention Reconciliation

**Agent:** consistency-validator (T2)

Identify and flag conflicting patterns between repos:
- Naming convention differences (camelCase vs snake_case, etc.)
- Error handling pattern differences
- Logging format differences
- API versioning strategy differences
- Recommend resolution strategy for each conflict

**Output:** `.factory-project/phase-0-synthesis/convention-reconciliation.md`

### Step 4: Unified Security Posture

**Agent:** security-reviewer (T2)

Produce a unified security posture spanning all repos:
- Per-repo security audit summaries
- Cross-repo attack surface analysis
- Authentication/authorization flow across repos
- Data classification across repo boundaries
- Shared secret management assessment

**Output:** `.factory-project/phase-0-synthesis/unified-security-posture.md`

### Step 5: Cross-Repo Holdout Scenarios

**Agent:** product-owner (T2)

Create holdout scenarios that test integrated system behavior:
- Scenarios that span multiple repos (e.g., "user creates task via API,
  sees it in frontend")
- Cross-repo failure scenarios (e.g., "API server down, frontend graceful
  degradation")
- Contract violation scenarios

**Output:** `.factory-project/cross-repo-holdout-scenarios/*.md`

### Step 6: Unified Project Context

**Agent:** codebase-analyzer (T1, via state-manager for writes)

Produce the unified project context document combining all synthesis outputs:
- Project overview and repo relationships
- Unified architecture summary
- Convention reconciliation summary
- Security posture summary
- Cross-repo integration points
- Recommended approach for the requested feature/change

**Output:** `.factory-project/phase-0-synthesis/project-context.md`

### Step 7: Adversarial Review

**Agent:** adversary (adversary model)

Review the unified project context with information asymmetry wall:
- Cannot see raw codebase (per DF-025 wall)
- Reviews synthesized artifacts only
- Challenges assumptions about cross-repo interactions
- Flags missing integration concerns

### Step 8: Synthesis Validation

**Agent:** consistency-validator (T2)

Final validation of unified artifacts against greenfield schemas:
- All required sections present
- Cross-references between repos are accurate
- Convention conflicts all have resolution strategies
- Security posture covers all repos

**Output:** `.factory-project/phase-0-synthesis/synthesis-validation-report.md`

## Gate

Human reviews and approves:
- Unified architecture accuracy
- Cross-repo dependency completeness
- Convention conflict resolutions
- Security posture adequacy
- Per-repo ingestion accuracy

## Write Routing

All writes by codebase-analyzer (T1) are routed through state-manager,
consistent with DF-023 tier model. state-manager commits synthesis artifacts
to factory-project-artifacts branch.

## Quality Gate

- [ ] Unified project-context.md produced in `.factory-project/phase-0-synthesis/`
- [ ] Cross-repo dependency graph maps all inter-repo API calls and shared types
- [ ] All per-repo ingestion project-context.md files synthesized into unified view
- [ ] Convention conflicts identified with resolution strategy for each
- [ ] Synthesis validation report passes all checks

## Failure Modes

- If a per-repo ingestion is incomplete: halt synthesis and report which repo needs completion
- If cross-repo dependencies are circular: flag the cycle and escalate to architect
- If convention conflicts have no clear resolution: present both options to human for decision
