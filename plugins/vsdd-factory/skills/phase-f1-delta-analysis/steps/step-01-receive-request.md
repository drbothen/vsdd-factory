# Step 1: Receive Feature Request

Read the feature request provided by the human. If no structured request exists, ask
the human to fill out `templates/feature-request-template.md` or provide equivalent
information (problem, proposed solution, scope, constraints, success criteria).

## Inputs

- Feature request document or verbal description from human

## Actions

1. Check if a structured feature request exists (using `templates/feature-request-template.md` format)
2. If not, prompt the human to provide: problem statement, proposed solution, scope (in/out), constraints, success criteria
3. Save or confirm the feature request location

## Outputs

- Confirmed feature request (structured or verbal, captured for subsequent steps)

## Completion Criteria

- Feature request is understood and captured
- Enough detail exists to proceed to artifact loading
