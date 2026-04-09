# Step 2: Classify Readiness Level

> **Parent skill:** `artifact-detection` — see `../SKILL.md` for the full workflow.
> **Agent:** orchestrator
> **This step:** map the inventory onto the L0–L4 readiness scale so step 5 knows where to route.

## Inputs

- Inventory from `step-01-scan-artifacts.md`

## Outputs

- A single readiness classification: `L0`, `L1`, `L2`, `L3`, or `L4`
- A short justification (which artifacts forced the classification)

## Procedure

1. Apply the readiness table from SKILL.md § Step 2 mechanically. Highest level wins, but every prerequisite must be present.

   | Level | Required artifacts | Route |
   |---|---|---|
   | **L0** | None | Collaborative Discovery |
   | **L1** | Product brief exists and is parseable | Validate brief → spec crystallization |
   | **L2** | L1 + PRD (may include L2 domain spec) | Validate PRD → remaining spec crystallization |
   | **L3** | L2 + architecture + L3 BCs | Validate both → story decomposition |
   | **L4** | L3 + L4 VPs + stories | Validate all → implementation |

2. **Classification rules:**
   - A file that exists but is unparseable does NOT count as present.
   - A legacy-format artifact (FR-NNN, single-file architecture) counts as present but must be flagged for migration.
   - Stories without a PRD → still classify as L0 or L1 based on what else exists. Stories alone do not cross the L4 threshold.
   - Architecture without BCs → classify as L2 (PRD layer), not L3.

3. **Tie-breaking** when prerequisites are partial:
   - If 80%+ of an L's prerequisites are met, classify as that L with `prerequisites_partial: true`. Step 4 will record the missing pieces.
   - If under 80%, classify as the level below.

4. **Record the classification** in working memory and append to the inventory file:

```markdown
## Readiness Classification

- **Level:** L<N>
- **Justification:** <which artifacts forced this level>
- **Prerequisites partial:** yes | no
- **Migration required:** yes | no
- **Recommended route:** <route from table>
```

## Decision points

- **L4 with stale stories** (older than the PRD they reference) → classify L4 but flag staleness for step 3 validation.
- **L0 with loose docs in root** → still L0, but pass loose docs to step 3 so they can be validated and possibly upgraded to L1.

## Failure modes

- **Counting unparseable files as present.** They do not count. Step 1 already flagged them.
- **Classifying L4 because stories exist, ignoring missing PRD.** Levels are cumulative. No skipping.
- **Promoting legacy formats silently.** Migration flag must propagate.

## Quality gate

- [ ] Exactly one L assigned
- [ ] Justification names the artifacts that forced the level
- [ ] Migration flag set if any legacy format detected
- [ ] Classification appended to the inventory file

## Hand-off to next step

Pass the classification and inventory to `step-03-validate-artifacts.md`.
