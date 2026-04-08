# Step 3: Architecture Delta (if needed)

Spawn `architect` agent to update architecture if structural changes are needed.

## Inputs

- Delta Analysis architecture assessment from Step 1
- Current architecture at `.factory/specs/architecture.md`

## Actions

If the Delta Analysis indicates structural architecture changes:

1. Spawn `architect` agent
2. Add new component definitions to the existing architecture
3. Update modified component interfaces
4. Verify the dependency graph remains acyclic after changes
5. Update the Purity Boundary Map if new modules are introduced
6. Write the architecture delta to `.factory/phase-f2-spec-evolution/architecture-delta.md`
7. Update the main architecture at `.factory/specs/architecture.md`

If no structural changes needed:
- Skip this step
- Note "Architecture unchanged" in the phase output

## Outputs

- `.factory/phase-f2-spec-evolution/architecture-delta.md` (if applicable)
- Updated `.factory/specs/architecture.md` (if applicable)

## Completion Criteria

- Architecture dependency graph remains acyclic
- New components have defined interfaces and module locations
- Purity boundary map is current
