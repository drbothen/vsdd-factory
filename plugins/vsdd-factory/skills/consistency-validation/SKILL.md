---
name: consistency-validation
description: >
  Cross-document validation skill. Checks alignment between PRD, architecture,
  UX specs, stories, and implementation artifacts.
---

# Consistency Validation

## Index-First Validation (DF-021)

Before loading detail files, validate index files for structural completeness:
- Check `architecture/ARCH-INDEX.md` references all existing section files
- Check `behavioral-contracts/BC-INDEX.md` references all BC files
- Check `verification-properties/VP-INDEX.md` references all VP files
- Check `adversarial-reviews/ADV-P[N]-INDEX.md` references all finding files (if exists)
- Check `evaluations/EVAL-INDEX.md` references all per-scenario evaluations (if exists)
- Verify every detail file has `traces_to:` pointing at its index

## Validation Rules

Run these checks in order. Report pass/fail with specific violations.

### Rule 0: Spec Format Validation (DF-020)
Validate that specifications use the 4-level hierarchy:
- L1 Product Brief -> L2 Domain Spec -> L3 Behavioral Contracts (BC-S.SS.NNN)
  -> L4 Verification Properties (VP-NNN)
- If legacy FR-NNN format detected, flag for migration to BC-S.SS.NNN
- Validate L1->L4 chain completeness: every L1 capability traces through
  L2 -> L3 -> L4

### Rule 1: PRD -> Epic Coverage
Every PRD requirement (BC-S.SS.NNN format preferred, FR-XXX legacy accepted)
must map to at least one epic. If FR-NNN format detected, flag for migration
to BC-S.SS.NNN format (DF-020 4-level hierarchy).
Query: find requirements with no epic reference.

### Rule 2: Epic -> Story Coverage
Every epic must have at least one story. Every story must belong to an epic.
Query: find orphaned stories or empty epics.

### Rule 3: Story -> Architecture Mapping
Every story must reference at least one architecture component.
Query: find stories with empty architecture mapping.

### Rule 4: Story -> UX Screen Mapping
Every UX screen must be referenced by at least one story.
Query: find UX screens with no story reference.

### Rule 5: Acceptance Criteria Testability
Every acceptance criterion must have an executable test mapped.
Query: find ACs without test function names.

### Rule 6: Verification Property Coverage
Every story must have at least one verification property.
Query: find stories with empty VP section.

### Rule 7: Dependency Acyclicity
Story dependency graph must have no cycles.
Query: perform topological sort, report cycles.

### Rule 8: Data Model Consistency
Data models in stories must match architecture data model.
Query: compare entity names and field types.

### Rule 9: Performance Target Alignment
Story performance targets must not exceed architecture NFR targets.
Query: compare numerical targets.

### Rule 10: Purity Boundary Consistency
Module purity classifications in stories must match architecture purity map.
Query: compare per-module classifications.

### Rule 11: Semantic Drift Detection
Between phases, verify that implementation decisions have not drifted from the
spec's stated constraints. For each implemented module:
- Extract the spec's behavioral contract (preconditions, postconditions, invariants)
- Extract the implementation's actual behavior (from test assertions and code analysis)
- Compute semantic similarity between spec intent and implementation behavior
- Flag modules where similarity drops below 0.7 as "drifted"
- Semantic drift indicates the implementer either misunderstood the spec or made
  unstated design decisions that should be captured in the spec
Query: for each story, compare spec ACs against test assertions. Flag mismatches.

### Rule 12: Token Budget Validation
Verify that no story in the current phase exceeds the token budget threshold:
- Estimate story context size (spec + referenced files + test files)
- Flag stories exceeding 60% of the implementing agent's context window
- Recommend splitting stories that exceed the threshold
Query: sum token estimates per story, compare against agent context limits.

### Rule 13: Upstream Traceability Chain
Validate that the full traceability chain from vision to requirements is intact:

**Chain:** Vision Statement -> Success Criteria -> User Journeys -> FRs/NFRs

For each level:
- Every success criterion must trace to at least one user journey
- Every user journey must trace to at least one FR
- Every FR must trace to at least one story

**Detect:**
- **Orphan FRs:** Functional requirements that do not trace back to any
  success criterion or user journey — may be gold-plating
- **Broken chains:** Success criteria with no downstream FRs — unmet goals
- **Unsupported success criteria:** Success criteria with no corresponding
  user journey showing how the user achieves the outcome

Query: for each FR, trace backward through journeys to success criteria.
Report orphans and broken chains.

### Rule 14: Downstream Traceability Completeness
Validate that every FR has a complete downstream chain:

```
FR -> Epic -> Story -> AC -> Test -> Implementation
```

Any gap in the chain means the requirement is not fully covered. Report:
- FRs without epic mapping
- Epics without stories
- Stories without ACs
- ACs without test function names

### Rule 15: L1→L2 Chain Completeness
Validate that every L1 brief capability maps to at least one L2 CAP-NNN in the
domain spec. Report L1 sections with no L2 capability mapping.
Query: cross-reference product brief sections against `domain-spec/L2-INDEX.md` CAP entries.

### Rule 16: L2→L3 Chain Completeness
Validate that every L2 CAP-NNN maps to at least one L3 BC-S.SS.NNN behavioral contract.
Report capabilities with no BC coverage.
Query: cross-reference `domain-spec/L2-INDEX.md` CAP entries against `behavioral-contracts/` files.

### Rule 17: L3→L4 Chain Completeness
Validate that every L3 BC-S.SS.NNN that requires formal verification maps to at least
one L4 VP-NNN. Report BCs with missing VP coverage and no documented justification for
omission.
Query: cross-reference `behavioral-contracts/` files against `verification-properties/` files.

### Rule 18: L1→L4 Full Chain Integrity
Validate the complete L1→L2→L3→L4 chain end-to-end. For each L1 section, trace through
L2→L3→L4 and report any broken links. This is the master chain check that ensures the
4-level hierarchy is fully wired.
Query: for each L1 section, verify L2 CAP exists, L3 BC exists, and L4 VP exists (where applicable).

### Rule 19: BC-to-Story Mapping
Validate that every L3 BC-S.SS.NNN is covered by at least one story. Report BCs with
no story mapping.
Query: cross-reference `behavioral-contracts/` files against story files in `phase-2-stories/stories/`.

### Rule 20: AC-to-BC Traceability
Validate that every story AC-NNN traces to a BC precondition or postcondition. Report
ACs with no BC traceability.
Query: for each story, verify AC entries reference BC-S.SS.NNN identifiers.

### Rule 21: VP Registry Completeness
Validate that every VP-NNN in the L4 registry has a proof harness skeleton defined.
Report VPs with no harness.
Query: cross-reference `verification-properties/` files against proof harness definitions.

### Rule 22: Design System Token Compliance (DF-037 D1)
For UI/full-stack products, validate that implementation code uses design tokens:
- No hardcoded hex color values where token exists
- No hardcoded pixel spacing where token exists
- No hardcoded font sizes where token exists
- Custom CSS only where justified and documented
Query: scan CSS/styled-components for values that should use tokens.

### Rule 23: Component Contract Compliance (DF-037 D1/D4)
For UI/full-stack products, validate component implementations against contracts:
- All required props present
- All required states implemented
- All accessibility requirements satisfied
- All data-fetching components have 4 async states (loading/success/empty/error)
Query: cross-reference component files against .factory/design-system/components/contracts/.

### Rule 24: UI Traceability Completeness (DF-037 D3/D10)
For UI/full-stack products, validate the UI traceability matrix:
- Every UX spec screen has corresponding story, component, test, evidence
- No gaps in the matrix (all cells filled before convergence)
- Fidelity score: implemented elements / specified elements
Query: validate .factory/ui-traceability.yaml for completeness.

### Rule 25: BC Clause Reverse Coverage
Every numbered precondition, postcondition, and invariant clause in every active BC
must have at least one AC tracing to it. Gap Register entries with non-empty
justification (min 10 chars) count as covered.
Query: parse BC files in `behavioral-contracts/` for clause counts (preconditions,
postconditions, invariants). For each clause, grep story files for trace references
matching the specific BC ID + clause type + clause number. Report uncovered clauses
with severity: postconditions = Critical, preconditions/invariants = Major.

### Rule 26: Edge Case + Error Reverse Coverage
Every EC-NNN in BC edge case tables and every E-xxx-NNN in `prd-supplements/error-taxonomy.md`
must trace to a story AC or edge case.
Query: extract EC-NNN IDs from all BC files' Edge Cases tables. Extract E-xxx-NNN IDs
from `prd-supplements/error-taxonomy.md`. Grep story files for each ID. Report orphaned
IDs as Major (broken-severity errors) or Minor (cosmetic errors).

### Rule 27: NFR-to-Story Reverse Coverage
Every NFR-NNN must be referenced by at least one story.
Query: extract NFR IDs from `prd-supplements/nfr-catalog.md`. Grep story files and
`dependency-graph.md` for each NFR-NNN reference. Report uncovered NFRs with severity:
P0 = Critical, P1 = Major, P2 = Minor.

### Rule 28: Holdout-BC-AC Alignment
For each holdout scenario's `behavioral_contracts` field, verify those BC clauses have
AC coverage in stories. This is a cross-check, not a direct requirement -- it warns
about test gaps where holdouts test clauses that nobody implemented.
Query: parse holdout scenario files for `behavioral_contracts` references. For each
referenced BC clause, verify it has AC coverage (from Rule 25). Report uncovered clauses
as Major (must-pass scenarios) or Minor (should-pass scenarios).

### Rule 29: Story-Level UI State Completeness
For UI stories, verify component contract required states appear in the story.
Query: load component contract YAML from `.factory/design-system/components/contracts/`.
For each required state in the contract, verify it appears in the story's Design System
Components table with a corresponding AC. Report missing states as Major.

### Rule 30: PRD Scope & Differentiator Enforcement
Validate that no story AC implements Out of Scope features, and every differentiator
has BC backing:
- Parse PRD Section 1.5 (Out of Scope) for excluded feature descriptions
- For each story AC, check that it does not implement an excluded feature
- Parse PRD Section 1.3 (Key Differentiators) for KD-NNN IDs
- For each KD-NNN, verify it maps to at least one BC in PRD Section 6
- Report: Out of Scope violations = Major, unmapped differentiators = Major
Query: extract Section 1.5 items, grep story ACs for matching concepts. Extract
KD-NNN from Section 1.3, verify presence in Section 6.

### Rule 31: PRD RTM Completeness
Validate that every row in the PRD Section 7 Requirements Traceability Matrix has
a non-empty Module(s) column:
- Parse PRD Section 7 table rows
- For each row, verify Module(s) column is not empty, "--", or "[filled by architect]"
- Report empty Module(s) as Major (architect has not completed backfill)
Query: parse PRD Section 7 table, check Module(s) column for placeholder text.

### Rule 32: Frontmatter Cross-Reference Integrity
Validate story frontmatter cross-references are consistent:
- `cycle` field matches the active cycle in STATE.md
- `epic_id` references a valid epic defined in epics.md
- If story has a `prd_version` or `input-hash`, check it references a current version
Query: parse story frontmatter fields, cross-reference against STATE.md and epics.md.

### Rule 33: BC Lifecycle Field Coherence
Validate that BC lifecycle fields are internally consistent:
- `lifecycle_status: deprecated` requires `deprecated_by` non-null
- `lifecycle_status: retired` requires `retired` non-null
- `lifecycle_status: active` requires `deprecated`, `deprecated_by`, `retired`, `removed` all null
- `lifecycle_status: removed` requires `removed` non-null and `removal_reason` non-null
Query: parse BC frontmatter, validate field combinations.

### Rule 34: FM-NNN to Holdout Coverage
Validate that every failure mode has holdout scenario coverage:
- Parse L2 Domain Spec Section 8 (Failure Modes) for FM-NNN IDs
- For each FM-NNN, search holdout scenario files for a scenario that exercises the failure mode
- Report uncovered FM-NNNs as Major
Query: extract FM-NNN IDs from domain-spec/L2-INDEX.md Section 8, grep holdout scenario
files for each FM-NNN reference or matching failure mode description.

## Output Format

```markdown
## Consistency Report

**Score:** XX/100

| Rule | Status | Violations |
|------|--------|-----------|
| 1. PRD -> Epic | PASS/FAIL | [list] |
| 2. Epic -> Story | PASS/FAIL | [list] |
...

**Blocking violations:** [count]
**Non-blocking warnings:** [count]
```

### Rule 35: L2 Domain Spec Sharding Integrity
Verify `domain-spec/L2-INDEX.md` exists and references all section files.
Every section file must have `traces_to: L2-INDEX.md`. No orphaned files.
ID counts in index must match actual counts in section files.

### Rule 36: UX Spec Sharding Integrity (UI products only)
Verify `ux-spec/UX-INDEX.md` exists and references all screen and flow files.
Every screen file traces to UX-INDEX.md. Every flow file traces to UX-INDEX.md.
Every screen referenced in a flow's `screens` frontmatter must exist as a file.
