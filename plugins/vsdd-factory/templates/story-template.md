---
document_type: story
level: ops
story_id: "S-N.MM"
epic_id: "E-N"
version: "1.1"
status: draft
producer: story-writer
timestamp: YYYY-MM-DDTHH:MM:SS
phase: 2
inputs: [domain-spec/L2-INDEX.md, prd.md, architecture.md]
input-hash: "[md5]"
traces_to: prd.md
points: "[1-13]"
depends_on: []
blocks: []
behavioral_contracts: []
verification_properties: []
priority: "P0|P1|P2"
# Lifecycle fields (DF-030)
cycle: vX.Y.Z                  # cycle that created this story (e.g., v1.0.0-greenfield)
# Planning extensions (optional — v1.1)
wave: null                     # wave-schedule number for parallel execution
target_module: null             # target module/package/crate name
subsystems: []                 # which subsystems this story touches (from ARCH-INDEX)
estimated_days: null            # planning estimate (complements points)
# ASM/R traceability (optional)
assumption_validations: []     # ASM-NNN IDs this story validates
risk_mitigations: []           # R-NNN IDs this story mitigates
tdd_mode: strict  # strict | facade. strict = full TDD Iron Law enforced (todo!() + Red Gate ≥0.5 required); facade = scaffold+impl combined, mutation testing at wave gate
---

> **tdd_mode:** Absent or unrecognized values default to `strict` per BC-8.30.001 invariant 2 — no existing story is silently promoted to `facade` mode. Set `tdd_mode: facade` only for DTU API clones, mock servers, structural fakes, and config parsing wrappers where the scaffold IS the implementation.

> **Execute:** `/vsdd-factory:deliver-story S-N.MM`

# S-N.MM: [Title]

> **One-per-file:** Each story lives in its own file.
> Filename convention: `S-N.MM-[short-description].md` (e.g., `S-1.01-foundational-types.md`).
> Story IDs use `S-N.MM` format where `N` is the section/epic-grouping (single digit,
> matches the parent epic `E-N`) and `MM` is the zero-padded story number within that
> section (e.g., `S-3.05` = section 3, story 5).
> Story `N` (section/epic) and BC `S` (subsystem number in BC-S.SS.NNN) are
> intentionally different hierarchies — a story can implement BCs from multiple
> subsystems via the `subsystems: [SS-NN, ...]` frontmatter array.
> The story-writer produces individual files under `.factory/stories/`
> and a companion `STORY-INDEX.md` listing all stories with status and dependencies.
> Story numbering is append-only across cycles within a section -- no resets (DF-030).

## Narrative
- **As a** [actor]
- **I want to** [action]
- **So that** [outcome]

## Acceptance Criteria

Numbered behavioral assertions are the default format. Each criterion describes the
expected behavior concisely. BDD Given/When/Then is acceptable but not required.

**Every AC must trace to a specific behavioral contract clause** (precondition,
postcondition, or invariant). This ensures full coverage of BCs by stories.

### AC-001 (traces to BC-S.SS.NNN postcondition N)
- [Specific, testable behavioral assertion -- e.g., "API returns paginated
  results with next_page token when more than 20 results exist."]
- **Test:** `test_BC_S_SS_NNN_[descriptive_name]()`

### AC-002 (traces to BC-S.SS.NNN invariant N)
- [Specific, testable behavioral assertion.]
- **Test:** `test_BC_S_SS_NNN_[descriptive_name]()`

> **Note on Verification Properties:** VPs are NOT embedded in stories. They live
> in the L4 VP Registry (`vp-registry.md`). Stories reference them by ID only in
> the `verification_properties` frontmatter field. This prevents duplication and
> drift between the registry and story files.

## Architecture Mapping

| Component | Module | Pure/Effectful |
|-----------|--------|---------------|
| [component_id] | [module_path] | pure-core / effectful-shell |

## UX Screens (Required only for UI stories)

<!-- v1.1: Marked conditional. Omit for non-UI stories (CLI tools, APIs, libraries). -->

- [SCR-NNN] -- [screen name]

## Design System Components (DF-037, UI stories only)

> For UI stories: list all components used, their contracts, required states,
> and async states. The implementer is constrained by these contracts.

| Component | Contract | Variants | Required States | Async States |
|-----------|----------|----------|----------------|-------------|
| [name] | [contracts/name.yaml] | [list] | [from contract] | loading/success/empty/error |

### Storybook Stories Required
- [ ] Component.stories.tsx for each new/modified component
- [ ] All variants x all required states covered
- [ ] test-writer calls get-storybook-story-instructions before writing

## Edge Cases

| ID | Scenario | Expected Behavior |
|----|----------|-------------------|
| EC-001 | [boundary condition] | [expected behavior] |

## Purity Classification

| Module | Classification | Justification |
|--------|---------------|---------------|
| [module_path] | pure-core / effectful-shell | [why] |

## Token Budget Estimate (MANDATORY)

| Context Source | Estimated Tokens |
|---------------|-----------------|
| This story spec | [N] |
| Referenced code files | [N] |
| Test files | [N] |
| Tool outputs overhead | [N] |
| **Total** | **[N]** |
| Agent context window | [200K for Sonnet] |
| **Budget usage** | **[N]%** |

Target: <= 20-30% of agent context window. If over budget, split the story.

## Tasks (MANDATORY)

1. [ ] Write failing tests (test-writer)
2. [ ] Implement to pass tests (implementer)
3. [ ] Verify purity boundaries
4. [ ] Update STATE.md
5. [ ] Write edge case tests
6. [ ] Write property-based tests
7. [ ] Verify Red Gate (all tests fail)
8. [ ] Implement (minimum code per test)
9. [ ] Refactor
10. [ ] Write Kani proof harnesses (if VP requires)
11. [ ] Write fuzz targets (if VP requires)

## Previous Story Intelligence (MANDATORY)

| Story | Key Decisions | Patterns Established | Gotchas Discovered |
|-------|--------------|---------------------|-------------------|
| [S-N.MM] | [decisions made] | [patterns to follow] | [pitfalls to avoid] |

_Populated by the story-writer from completed stories in the same epic.
Each new story carries forward lessons from its predecessors to prevent
the implementer from repeating mistakes or reinventing established patterns._

## Architecture Compliance Rules (MANDATORY)

| Rule | Source | Enforcement |
|------|--------|-------------|
| [rule description] | [ADR-NNN or architecture.md section] | [how to verify] |

_Extracted from architecture.md and ADRs. The implementer checks these
before starting and after completing each test cycle._

## Library & Framework Requirements (MANDATORY)

| Tool | Version | Purpose |
|------|---------|---------|
| [library/framework] | [>= X.Y.Z] | [why this version is required] |

_From architecture.md technology stack. Prevents version drift across stories.
The story-writer populates this from the architecture doc AND runs web research
to verify versions are current._

## File Structure Requirements (MANDATORY)

| File | Action | Purpose |
|------|--------|---------|
| [path/to/file] | create / modify / delete / relocate | [what this file does] |

_Maps which files this story reads, creates, or modifies. Prevents the
implementer from creating files in unexpected locations or missing dependencies._
