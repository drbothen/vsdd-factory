# Step 4: Gap Analysis

> **Parent skill:** `artifact-detection` — see `../SKILL.md` for the full workflow.
> **Agent:** orchestrator
> **This step:** consolidate the validation results into a single gap analysis report.

## Inputs

- Per-artifact verdicts and gaps from `step-03-validate-artifacts.md`
- Inventory and classification from steps 1–2

## Outputs

- `.factory/planning/gap-analysis.md` — the consolidated report

## Procedure

1. Create `.factory/planning/` if it does not exist.

2. Write `gap-analysis.md` using exactly this structure:

```markdown
# Gap Analysis Report

**Generated:** <YYYY-MM-DD HH:MM>
**Readiness Level:** L<N>
**Migration required:** yes | no

## Summary Table

| Artifact | Status | Gap Count | Migration |
|----------|--------|-----------|-----------|
| Brief | VALID / INCOMPLETE / MISSING | <N> | — |
| Domain Spec (L2) | ... | <N> | — |
| PRD | ... | <N> | yes |
| Behavioral Contracts | ... | <N> | — |
| Architecture | ... | <N> | yes |
| Verification Properties | ... | <N> | — |
| UX Spec | ... | <N> | — |
| Stories | ... | <N> | — |

## Detailed Gaps

### Brief
**Status:** <verdict>
**Path:** <path or "not found">

- <specific gap 1>
- <specific gap 2>

### PRD
**Status:** <verdict>
**Path:** <path or "not found">

- <specific gap 1>
- <specific gap 2>

<...one section per artifact...>

## Migration Items

<List every legacy-format flag from step 1, with the migration action required.>

- `<path>`: legacy FR-NNN format → migrate to BC-S.SS.NNN before proceeding
- `<path>`: legacy single-file architecture → shard to ARCH-INDEX.md + section files (DF-021)

## Cross-Artifact Issues

<Issues that span multiple artifacts.>

- PRD requirement BC-1.02.005 has no story coverage
- Story STORY-007 references VP-014 which does not exist
- Architecture section "api-surface" is missing required `traces_to` frontmatter

## Blockers vs Warnings

**Blockers** (must fix before any pipeline progress):
- <blocker>

**Warnings** (can proceed with flags):
- <warning>
```

3. Categorize each gap as **blocker** or **warning**:
   - **Blocker:** legacy format requiring migration; missing required artifact for the chosen route; unparseable file the next phase depends on; circular dependency in architecture.
   - **Warning:** vague language in PRD; story exceeding token budget; missing optional section; bloat warnings.

4. Pass the report path and a summary count (blockers, warnings) to step 5.

## Failure modes

- **Hiding gaps in prose instead of bullets.** Gaps must be enumerated; step 5 routing logic counts them.
- **Mixing blockers and warnings.** They route differently. Keep them separate.
- **Forgetting cross-artifact issues.** These are the most valuable findings. Step 3 surfaces them — make sure they propagate here.

## Quality gate

- [ ] `gap-analysis.md` exists at `.factory/planning/gap-analysis.md`
- [ ] Summary table covers every artifact in the inventory
- [ ] Every gap from step 3 appears in detail or summary
- [ ] Migration items section populated (or explicitly "None")
- [ ] Blockers and warnings separated

## Hand-off to next step

Pass the gap analysis path, blocker count, and warning count to `step-05-route-decision.md`.
