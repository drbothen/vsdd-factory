# Step 1: Load Existing Story Graph

Read all existing story specs and build the current dependency graph.

## Inputs

- `.factory/stories/` directory

## Actions

1. Read all existing story specs from `.factory/stories/`
2. Build the current dependency graph (story IDs and their depends_on links)
3. Identify the highest existing story ID (to continue the sequence)
4. Note which stories are completed vs in-progress

## Outputs

- Story dependency graph (adjacency list)
- Highest story ID (sequence anchor)
- Story status map (completed / in-progress / planned)

## Completion Criteria

- All existing stories are indexed
- Dependency graph is complete and acyclic
- Next available story ID is known
