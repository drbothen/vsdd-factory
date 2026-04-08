# Step 2: Generate Tests for New Stories

Write test cases for each new story before any implementation code.

## Inputs

- New story specs from Phase F3 (in dependency order)
- Existing test conventions and directory structure

## Actions

1. For each new story (in dependency order), spawn `test-writer` agent to:
   - Read the story's acceptance criteria
   - Write test cases that encode each acceptance criterion
   - Write boundary tests (empty, too-long, invalid, edge cases)
   - Place tests in the appropriate test directory following existing conventions
2. Tests are written to `.factory/phase-f4-implementation/tests/` AND to the project's actual test directory

## Outputs

- New test files in project test directory
- Copies in `.factory/phase-f4-implementation/tests/`

## Completion Criteria

- Every acceptance criterion has at least one corresponding test
- Boundary cases are covered (empty, invalid, edge cases)
- Tests follow existing naming and structural conventions
