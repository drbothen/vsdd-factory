# Step 4: Cycle Detection

Validate the extended dependency graph is acyclic using topological sort.

## Inputs

- Extended dependency graph from Step 3

## Actions

1. Build adjacency list: existing stories + new stories
2. Run topological sort (Kahn's algorithm)
3. If sort completes: no cycles -- proceed
4. If sort fails: identify the cycle and report to human
5. If cycles found, restructure new stories to eliminate them:
   - Common fixes: merge two stories, split a story differently, remove a dependency

## Outputs

- Topological sort result (success or cycle identification)
- Restructured stories (if cycles were found and fixed)

## Completion Criteria

- Topological sort completes successfully on the full graph
- No cycles exist in the extended dependency graph
