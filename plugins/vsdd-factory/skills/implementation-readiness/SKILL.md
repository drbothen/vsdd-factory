---
name: implementation-readiness
description: >
  Validates that the complete spec package (PRD + architecture + stories) is
  internally consistent and ready for implementation. This is the gate
  between planning and building.
---

# Implementation Readiness Check

> **Step-file note:** This skill does NOT use step-file decomposition.
> Its 6 validation dimensions run as parallel checks, not sequential steps.

## When This Skill Runs

- In Spec Intake mode when L4 artifacts exist (full spec package)
- After story decomposition completes and is approved (standard pipeline gate)
- When the human asks "are we ready to implement?"

## Validation Dimensions

### 1. PRD Completeness
- All requirements have unique IDs (FR-XXX, NFR-XXX)
- All requirements have measurable acceptance criteria as behavioral assertions
- Acceptance criteria eliminate vague language
- Edge case catalog exists and covers boundary conditions
- Success metrics have numerical targets
- **Bloat check:** PRD doesn't include narrative padding in the requirements sections

### 2. Architecture Alignment
- Architecture directory exists with ARCH-INDEX.md (DF-021)
- All expected section files present (system-overview, module-decomposition, dependency-graph,
  api-surface, verification-architecture, purity-boundary-map, tooling-selection,
  verification-coverage-matrix)
- Each section file has `traces_to: ARCH-INDEX.md` in frontmatter
- Every architectural component maps to at least one PRD requirement
- Technology stack explicitly chosen with version constraints and justification
- Purity boundary map exists in `architecture/purity-boundary-map.md` -- every module classified as pure/effectful per FCIS pattern
- Verification properties catalog exists for critical modules
- No circular dependencies in the component graph
- **Machine-readable architectural map exists** -- component inventory, dependencies, and
  test coverage expressed in structured format (YAML/JSON section), not just prose
- **ADRs documented** -- key architecture decisions have context, rationale, alternatives
  considered, and consequences

### 3. Story Coverage
- Every PRD requirement is covered by at least one story
- Every story has acceptance criteria as **numbered behavioral assertions**
- Story dependency graph is acyclic (topological sort succeeds)
- No story exceeds 13 story points
- Every story references architecture components
- **Token budget validation:** Each story's total context estimated at 20-30% of
  implementing agent's context window
- **Story content density:** Each story is 300-800 tokens covering: acceptance criteria,
  architectural constraints, 1-2 code reference examples, and explicit non-goals

### 4. Cross-Document Consistency (Traceability Matrix)
- **Requirement traceability matrix** exists: every FR maps to architecture component(s)
  and story/stories
- Data models match across architecture and stories
- API contracts are consistent across all documents
- Naming conventions are consistent
- Performance targets align between stories and architecture NFRs
- **Typed schemas at agent boundaries:** Where agents exchange data, the interface is a
  defined schema, not free-form prose

### 5. Context Budget Validation
- **Total spec package token estimate:** Sum all artifacts that will be loaded into agent
  context during implementation. If total exceeds 60% of the implementing agent's context
  window, flag for compression using the Extended ToC pattern.
- **Information placement audit:** Critical constraints and requirements are frontloaded.
  Supporting details are at the end or in referenced sub-documents. Middle sections are
  flagged as "lost in the middle" risk.
- **Reference vs inline:** Flag any section over 500 words that could be referenced instead
  of inlined.

### 5b. PRD Implementation Leakage
Scan PRD functional and non-functional requirements for premature implementation decisions:

**FR/NFR leakage patterns:**
- FRs that name specific technologies
- NFRs that prescribe specific tools
- ACs that reference implementation details

**Heuristic:** If removing the technology name makes the requirement MORE clear about what
the system should do, it was leakage. If removing it makes the requirement ambiguous, it
was a legitimate constraint.

Report leakage with severity: Error (definitely leakage), Warning (possibly legitimate
constraint), Info (reference only).

### 5c. PRD Information Density
Apply the same information density check from brief validation to the PRD. Scan for
conversational filler, wordy phrases, redundant phrases, and hedge words. PRDs should
be maximally dense -- every token should carry signal.

Severity thresholds: Critical (>10), Warning (5-10), Pass (<5).

### 6. L3 Behavioral Contract Existence
- L3 behavioral contracts directory exists (`.factory/specs/behavioral-contracts/`)
- BC-INDEX.md exists and references all per-file BCs
- Every BC-S.SS.NNN has preconditions, postconditions, and invariants defined
- Every BC maps to at least one story (cross-reference with decomposed stories)
- No orphan BCs (BCs with no story coverage and no documented justification)

### 7. L4 Verification Property Existence
- L4 verification properties directory exists (`.factory/specs/verification-properties/`)
- VP-INDEX.md exists and references all per-file VPs
- Every VP-NNN has a proof strategy and harness location defined
- Every VP maps to at least one BC (cross-reference with L3 contracts)
- Architecture feasibility report confirms all VPs have viable proof strategies
- No VP-NNN marked as "infeasible" without documented justification and alternative

### 8. UX Alignment (if UX spec exists)
- Every UX screen maps to at least one story
- Accessibility requirements are documented
- All form inputs have validation rules

## Templates

Use `${CLAUDE_PLUGIN_ROOT}/templates/implementation-readiness-template.md` for the implementation readiness report format.

## Output

Write readiness report to `.factory/planning/readiness-report.md`:

| Dimension | Status | Findings |
|-----------|--------|----------|
| PRD Completeness | PASS / FAIL | [details] |
| Architecture Alignment | PASS / FAIL | [details] |
| Story Coverage | PASS / FAIL | [details] |
| Cross-Document Consistency | PASS / FAIL | [details] |
| Context Budget | PASS / WARNING / OVER | [token estimates per artifact] |
| L3 BC Existence | PASS / FAIL | [details] |
| L4 VP Existence | PASS / FAIL | [details] |
| UX Alignment | PASS / N/A | [details] |

**Overall: READY / CONCERNS / NOT_READY**

If CONCERNS: list items that should be addressed but don't block implementation.
If NOT_READY: list blocking issues that must be resolved.
If OVER (context budget): recommend specific artifacts to compress or reference.

## Quality Gate

- [ ] All cross-document consistency checks pass (traceability matrix complete)
- [ ] No blocking gaps in PRD, architecture, story coverage, or L3/L4 existence
- [ ] Context budget within 60% of implementing agent's context window
- [ ] Readiness report written to `.factory/planning/readiness-report.md` with overall verdict
