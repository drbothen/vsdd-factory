# Step 3: Dependency Graph Extension

Define dependencies for each new story, linking to both new and existing stories.

## Inputs

- Draft story specs from Step 2
- Existing story graph from Step 1

## Actions

1. For each new story, define dependencies:
   - Dependencies on OTHER NEW stories (within this feature)
   - Dependencies on EXISTING stories (cross-feature links)
   - No story may depend on itself or create a cycle
2. Record dependencies in each story's frontmatter

## Outputs

- Extended dependency graph (existing + new stories)
- Cross-feature dependency links documented

## Completion Criteria

- Every new story has its dependencies explicitly defined
- No self-dependencies exist
- Cross-feature links are documented with rationale
