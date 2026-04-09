# Step 3: Validate What Exists

> **Parent skill:** `artifact-detection` — see `../SKILL.md` for the full workflow.
> **Agent:** consistency-validator
> **This step:** run the per-artifact validation checklists and produce a per-artifact verdict.

## Inputs

- Inventory and classification from steps 1–2
- Validation checklists from SKILL.md § Step 3

## Outputs

- Per-artifact verdicts: `VALID` | `INCOMPLETE` | `MISSING`
- Per-artifact gap list (specific, not vague)

## Procedure

For each present artifact, run its checklist. Use explicit per-item checks; do not eyeball.

### Brief validation

For the brief file:
1. Open it and parse frontmatter. If unparseable, mark `INCOMPLETE` with gap "frontmatter unparseable".
2. Check that each required section has substantive content (more than a heading and one line):
   - [ ] What is this
   - [ ] Who is it for
   - [ ] Why does it matter
   - [ ] Differentiator
   - [ ] Scope (in v1) and out-of-scope
   - [ ] Success criteria (measurable)
3. Flag if the brief is just a title or one-liner.
4. Flag if scope says "build everything" or equivalent.

### PRD validation

For each PRD file:
1. [ ] Numbered functional requirements present (BC-S.SS.NNN or FR-NNN)
2. [ ] Numbered NFRs with numerical targets (not "fast" — "p95 < 200ms")
3. [ ] Measurable success criteria
4. [ ] Edge case catalog with boundary conditions
5. [ ] Each requirement is SMART (specific, measurable, achievable, relevant, time-bound)
6. [ ] Acceptance criteria use behavioral descriptions, not implementation hints
7. [ ] No vague language scan: grep for "should", "may", "could", "fast", "easy", "user-friendly" — flag each occurrence
8. [ ] **Bloat check:** word count vs reasonable budget (warn over 15k words)
9. [ ] **Token budget:** estimate tokens; warn if PRD exceeds 30% of a 200k context window

### Architecture validation

For each architecture artifact:
1. [ ] `ARCH-INDEX.md` exists with document map and cross-references (DF-021)
2. [ ] All expected section files present:
   - system-overview
   - module-decomposition
   - dependency-graph
   - api-surface
   - verification-architecture
   - purity-boundary-map
   - tooling-selection
   - verification-coverage-matrix
3. [ ] Each section file has `traces_to: ARCH-INDEX.md` in frontmatter
4. [ ] Component inventory with responsibilities (in module-decomposition.md)
5. [ ] Technology stack with explicit version constraints AND justification
6. [ ] Data model with relationships
7. [ ] Purity boundary map (pure core vs effectful shell)
8. [ ] Verification properties catalog for critical modules
9. [ ] Machine-readable architectural map (YAML/JSON block)
10. [ ] ADRs documented with context, rationale, consequences
11. [ ] Dependency graph is acyclic — walk the edges, flag any cycle

### Story validation

For each story file (from `.factory/stories/`):
1. [ ] Every PRD requirement covered by at least one story (cross-reference BC IDs)
2. [ ] Every story has acceptance criteria as numbered behavioral assertions
3. [ ] Dependency graph is acyclic
4. [ ] No story exceeds 13 story points
5. [ ] **Token budget:** each story fits within 20–30% of implementing agent's context
6. [ ] **Story content sweet spot:** 300–800 tokens

### Format-specific validation

- If FR-NNN format detected anywhere: mark the artifact `INCOMPLETE` with gap "legacy FR-NNN format — migration required before proceeding".
- If single-file architecture detected: mark `INCOMPLETE` with gap "legacy single-file architecture — sharding required (DF-021)".

## Procedure for recording results

Record per-artifact verdict in a structured form, ready for step 4:

```yaml
- artifact: brief
  path: .factory/planning/product-brief.md
  verdict: VALID | INCOMPLETE | MISSING
  gaps:
    - "specific gap 1"
    - "specific gap 2"
  migration_required: false
```

## Failure modes

- **Marking an artifact VALID without running every checklist item.** Each item is a yes/no — answer all of them.
- **Vague gap descriptions** ("PRD is incomplete"). Gaps must be specific: "NFR-003 has no numerical target", "section 'Success Criteria' is missing".
- **Skipping cross-referential checks** (PRD requirements vs story coverage). These catch the most damaging gaps.

## Quality gate

- [ ] Every present artifact has a verdict
- [ ] Every INCOMPLETE verdict lists specific gaps
- [ ] Migration flags propagated from step 1
- [ ] Verdict structure ready for step 4 to consume

## Hand-off to next step

Pass per-artifact verdicts and gaps to `step-04-gap-analysis.md`.
