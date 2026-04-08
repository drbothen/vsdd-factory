# Step 4: Implementation (TDD Loop)

Implement the minimum code to make new tests pass, following TDD discipline.

## Inputs

- New story specs (in dependency order)
- Failing tests from Step 3
- Delta Analysis scope (which files can be modified)

## Actions

1. For each new story (in dependency order), spawn `implementer` agent to:
   - Work in an isolated worktree (if supported by the project)
   - Write the minimum code to make the new tests pass
   - Follow existing code conventions (naming, structure, error handling)
   - Do NOT modify existing code unless the Delta Analysis explicitly identified it as a MODIFIED file
2. TDD discipline: Red -> Green -> Refactor
3. The `tdd-enforcement.sh` hook prevents writing implementation code without corresponding tests

## Outputs

- New implementation files
- Modified existing files (only those in delta scope)

## Completion Criteria

- Implementation follows existing code conventions
- Only files within delta scope are modified
- TDD discipline maintained (tests first, then code)
