# Step 5: Conflict Detection

Check for conflicts between new stories and in-progress work.

## Inputs

- New story specs from Step 2
- Existing story status map from Step 1
- Affected files list from Phase F1

## Actions

1. Check: Do any new stories modify the same files as in-progress stories?
2. Check: Do any new stories depend on stories that are not yet complete?
3. Check: Are there race conditions in the dependency graph?
4. Report conflicts to the human with recommended resolution

## Outputs

- Conflict report (if conflicts found)
- Recommended resolutions for each conflict

## Completion Criteria

- All potential conflicts are identified
- Each conflict has a recommended resolution
- If no conflicts: explicitly state "no conflicts detected"
