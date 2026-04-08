# Step 2: Load Existing Artifacts

Read and index all existing pipeline artifacts to establish the current state of the project.

## Inputs

- Existing `.factory/` directory from previous pipeline run

## Actions

1. Read `.factory/specs/prd.md` -- current PRD with all requirement IDs
2. Read `.factory/specs/architecture.md` -- current architecture
3. Read `.factory/specs/verification-architecture.md` -- current verification properties
4. Read all story specs from `.factory/stories/`
5. Read `.factory/cycles/**/implementation/` -- existing test manifest
6. Read `.factory/cycles/**/convergence/` -- existing traceability chain
7. Build an index of: all requirement IDs, all story IDs, all verification property IDs, all component names

## Outputs

- Complete inventory of existing artifacts
- ID sequences identified (last BC-S.SS.NNN, last STORY-NNN, last VP-NNN)
- Component list with dependency graph

## Completion Criteria

- All artifact directories have been read
- ID sequences are known (to avoid collisions in later phases)
- Architecture component list is available for impact mapping
