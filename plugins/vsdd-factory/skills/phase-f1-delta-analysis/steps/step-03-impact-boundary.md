# Step 3: Impact Boundary Identification

Spawn `architect` agent to map the impact boundary of the feature request against the existing architecture.

## Inputs

- Feature request from Step 1
- Architecture and component list from Step 2

## Actions

1. Spawn `architect` agent with feature request and current architecture
2. Map which existing components are affected by the feature request
3. Identify new components that must be created
4. Determine if the architecture needs structural changes (new modules, new interfaces) or only internal changes (new logic within existing modules)
5. Classify each affected component:
   - **NEW**: Does not exist yet, must be created
   - **MODIFIED**: Exists, must change
   - **DEPENDENT**: Unchanged but depends on something that changed

## Outputs

- Component impact map with classifications (NEW / MODIFIED / DEPENDENT)
- Assessment: structural vs internal changes

## Completion Criteria

- Every affected component is classified
- New components are identified with proposed location in the architecture
- The scope of architectural change is clear (structural vs internal)
