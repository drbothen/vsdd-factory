---
name: phase-f2-spec-evolution
description: >
  Feature Mode Phase F2: Update specs incrementally -- PRD, architecture,
  and verification properties. Delta only, not full rewrite.
---

> **Delegation Reference:** This skill describes work the orchestrator delegates
> to specialist agents via the Agent tool. Each step names the target agent.
> The orchestrator does NOT execute these steps directly — it spawns the named
> agent for each step and reviews the output.

# Phase F2: Spec Evolution

## Prerequisites

- Phase F1 Delta Analysis complete and human-approved
- `.factory/phase-f1-delta-analysis/delta-analysis.md` exists

## Workflow

### Step 1: Load Delta Analysis

Read `.factory/phase-f1-delta-analysis/delta-analysis.md` to understand:
- Which requirements are new vs modified
- Which architecture components are affected
- Which verification properties need extension

### Step 2: PRD Delta

Spawn `product-owner` agent to:
- Append new behavioral contracts using BC-S.SS.NNN format (4-level hierarchy,
  DF-020). Continue numbering from existing BCs.
- Modify existing BCs that need updating (mark with UPDATED tag and
  record previous version inline)
- Append new non-functional requirements if applicable
- Append new edge cases to the Edge Case Catalog
- Do NOT rewrite or restructure existing unaffected requirements
- Write the PRD delta to `.factory/phase-f2-spec-evolution/prd-delta.md`
- Update the main PRD at `.factory/specs/prd.md` with the changes

### Step 2b: DTU Re-Assessment (if feature adds new external deps)

If the feature introduces new external service dependencies:

Spawn `architect` agent to:
- Assess each new dependency for DTU candidacy (fidelity levels L1-L4)
- Update dtu-assessment.md
- Spawn research-agent for API specs of new DTU candidates

### Step 2c: Gene Transfusion Assessment (if reference impl exists)

If the feature has a proven reference implementation in another language:

Spawn `architect` agent to:
- Evaluate FOR/AGAINST signals (complexity, test count, paradigm gap, license)
- Update gene-transfusion-assessment.md

### Step 2d: UX Design Delta (if UI feature)

Condition: `feature_type in ['ui', 'full-stack']`

Spawn `ux-designer` (T2) to:
- Update UX spec for the feature delta
- New screens / modified flows / updated components
- Wireframes or interaction descriptions
- Write `.factory/feature/ux-delta.md`

### Step 2e: Accessibility Review of UX Delta (if UI feature)

Condition: `feature_type in ['ui', 'full-stack']`

Spawn `accessibility-auditor` (T2) to:
- Review UX delta from Step 2d
- WCAG compliance check on new/modified UI elements
- Focus: keyboard navigation, screen reader, color contrast, touch targets
- Write `.factory/feature/accessibility-review-f2.md`

Information asymmetry wall: accessibility-auditor cannot see architecture or
implementation plans. Provide only the UX delta and existing UX spec.

### Step 3: Architecture Delta (if needed)

If the Delta Analysis indicates structural architecture changes:

Spawn `architect` agent to:
- Add new component definitions to the existing sharded architecture (DF-021)
- Update ARCH-INDEX.md and affected section files
- Update modified component interfaces
- Verify the dependency graph remains acyclic after changes
- Update the Purity Boundary Map if new modules are introduced
- Write the architecture delta to `.factory/phase-f2-spec-evolution/architecture-delta.md`
- Update the architecture section files at `.factory/specs/architecture/`

If no structural changes needed, skip this step and note "Architecture unchanged" in
the phase output.

### Step 4: Verification Property Extension

Spawn `formal-verifier` agent to:
- Define new verification properties for new requirements
  (continue VP-ID sequence)
- Update existing verification properties if requirements changed
- Determine which new Kani proofs, proptest strategies, or fuzz targets are needed
- Write to `.factory/phase-f2-spec-evolution/verification-delta.md`
- Update `.factory/specs/verification-architecture/ARCH-INDEX.md`

### Step 5: Spec Version Bump

Apply spec versioning rules (see `workflows/skills/spec-versioning/SKILL.md`):
- MAJOR: architectural rework, removed features, breaking changes
- MINOR: new features, new requirements (most common for Feature Mode)
- PATCH: wording fixes, edge case additions, clarifications

Update the spec version in the PRD frontmatter and write a changelog entry to
`.factory/spec-changelog.md` using `templates/spec-changelog-template.md`.

### Step 6: Adversarial Spec Review (Scoped)

Spawn `adversary` agent (adversary model, fresh context) to review ONLY:
- The PRD delta (new and modified requirements)
- The architecture delta (if any)
- The verification property extensions
- The UX spec delta (if UI feature -- `.factory/feature/ux-delta.md`)

The adversary does NOT review unchanged spec sections. Provide only the delta
documents and enough surrounding context for coherence.

Write review to `.factory/phase-f2-spec-evolution/adversarial-spec-delta-review.md`

### Step 7: Iterate on Findings

If the Adversary finds legitimate issues in the spec delta:
- Route findings to the responsible agent (product-owner, architect, or
  formal-verifier)
- Agent fixes the delta
- Re-run adversarial review on the updated delta (fresh context)
- Repeat until findings are cosmetic only

### Step 8: Human Approval Gate

Present the spec delta package to the human:
- PRD delta (new and modified requirements)
- Architecture delta (if any)
- Verification property extensions
- Adversarial review results
- Spec version bump and changelog entry

Phase F2 is COMPLETE only when the human explicitly approves the spec delta.

## Output Artifacts

- `.factory/phase-f2-spec-evolution/prd-delta.md`
- `.factory/phase-f2-spec-evolution/architecture-delta.md` (if applicable)
- `.factory/phase-f2-spec-evolution/verification-delta.md`
- `.factory/phase-f2-spec-evolution/adversarial-spec-delta-review.md`
- `.factory/feature/ux-delta.md` (if UI feature)
- `.factory/feature/accessibility-review-f2.md` (if UI feature)
- `.factory/spec-changelog.md` (appended)
- Updated: `.factory/specs/prd.md`
- Updated: `.factory/specs/architecture/ARCH-INDEX.md` (if applicable)
- Updated: `.factory/specs/verification-architecture/ARCH-INDEX.md`

## Quality Gate Criteria

- [ ] New BCs use BC-S.SS.NNN format (4-level hierarchy, DF-020)
- [ ] New requirements continue the existing ID sequence (no gaps, no collisions)
- [ ] Modified requirements retain previous version inline (UPDATED tag)
- [ ] Architecture dependency graph remains acyclic
- [ ] New verification properties have corresponding proof strategies
- [ ] Spec version bumped correctly per semver rules
- [ ] Changelog entry written
- [ ] Adversary findings on delta are cosmetic only
- [ ] DTU re-assessment complete (if new external deps flagged)
- [ ] Gene transfusion assessment complete (if reference impl exists)
- [ ] UX delta reviewed by ux-designer (if UI feature)
- [ ] Accessibility review of UX delta complete (if UI feature)
- [ ] Human has explicitly approved the spec delta

## Failure Modes

- If delta analysis is missing (`.factory/phase-f1-delta-analysis/delta-analysis.md` does not exist): stop and report to orchestrator — Phase F2 cannot proceed without F1 output
- If evolved specs conflict with existing behavioral contracts: flag the specific conflicting BCs with before/after comparison and escalate to product-owner
- If adversary finds CRITICAL issues in the delta after 3 iteration rounds: escalate to human with the unresolved findings
