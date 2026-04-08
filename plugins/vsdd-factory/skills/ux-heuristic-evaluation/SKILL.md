---
name: ux-heuristic-evaluation
description: >
  Automated usability evaluation against Nielsen's 10 heuristics. Runs on
  UX specs (given UX spec document) and implemented UI (given running application
  after holdout evaluation). Produces scored report with specific findings and
  recommendations. Includes cognitive walkthrough for key user tasks.
agents:
  primary: ux-designer
  supporting: [accessibility-auditor, business-analyst]
inputs:
  - UX spec document or running application (after holdout evaluation)
  - Key user tasks from UX spec
outputs:
  - .factory/ui-quality/heuristic-evaluation.md
condition: "feature_type in ['ui', 'full-stack']"
---

# UX Heuristic Evaluation

## When It Runs

| Point | Input | Focus |
|-------|-------|-------|
| After UX spec produced | UX spec document | Spec-level heuristics, task flow |
| After holdout evaluation | Running application | Actual UI behavior, real interaction |
| F2 (feature spec) | UX spec delta | Delta heuristics, regression on existing |
| Wave gate | Implemented wave | Per-wave usability check |

## 10 Heuristic Checks

For each heuristic, evaluate the UI and score 0.0-1.0:

### H1: Visibility of System Status
- Loading indicators present for all async operations
- Progress bars for multi-step processes
- Feedback for every user action (button clicks, form submissions)
- Status of background processes visible

### H2: Match Between System and Real World
- Natural language (no technical jargon)
- Real-world conventions for icons and metaphors
- Information in natural, logical order
- Terminology matches user mental model

### H3: User Control and Freedom
- Undo/redo where applicable
- Cancel option for all multi-step processes
- Back navigation available
- Clear exit points from every state

### H4: Consistency and Standards
- Platform conventions followed
- Internal consistency (same action = same result)
- Design system tokens used consistently
- Component variants used correctly per contract

### H5: Error Prevention
- Confirmation dialogs for destructive actions
- Input validation before submission
- Smart defaults reduce error potential
- Disabled states prevent invalid actions

### H6: Recognition Over Recall
- Options visible (not hidden in menus)
- Context-appropriate hints and placeholders
- Recent items and history accessible
- Search available for large datasets

### H7: Flexibility and Efficiency
- Keyboard shortcuts for power users
- Batch operations for repeated tasks
- Customizable views/layouts
- Recent/frequent actions accessible

### H8: Aesthetic and Minimalist Design
- Appropriate visual density
- Adequate whitespace
- Clear information hierarchy
- Non-essential information hidden (progressive disclosure)

### H9: Help Users Recover from Errors
- Error messages in plain language
- Error messages suggest specific fix
- Errors are non-destructive (data preserved)
- Recovery path clearly indicated

### H10: Help and Documentation
- Contextual help available
- Documentation searchable
- Task-oriented help (not just reference)
- Onboarding for first-time users

## Cognitive Walkthrough

For each key user task identified in the UX spec:

1. **Define task:** goal and expected steps
2. **Walk through each step:**
   - Can the user find the right action?
   - Will they understand the action?
   - Will they know they did the right thing?
3. **Measure:**
   - Steps required (fewer = better)
   - Error potential per step (0.0-1.0)
   - Backtrack points (where user might go wrong)
4. **Score:** task completion confidence (0.0-1.0)

```yaml
tasks:
  - name: "Task description"
    expected_steps: N
    actual_steps: N
    error_potential: 0.N
    backtrack_points: N
    score: 0.N
    issues:
      - "Specific finding"
```

## Scoring

- Each heuristic scored 0.0-1.0
- Each task scored 0.0-1.0
- **Threshold: 0.7** -- scores below 0.7 flag for remediation
- Findings feed into adversarial review context

## Report Format

Output to `.factory/ui-quality/heuristic-evaluation.md`:

```markdown
## UX Heuristic Evaluation

### Heuristic Scores
| Heuristic | Score | Key Findings |
|-----------|-------|-------------|
| H1: Visibility of System Status | 0.N | ... |
| ... | ... | ... |

### Task Completion Analysis
| Task | Steps | Error Potential | Score | Issues |
|------|-------|----------------|-------|--------|
| ... | N | 0.N | 0.N | ... |

### Remediation Items
1. [Finding]: [Recommendation]

### Overall Score: N.NN / 1.0
```

## Quality Gate

- [ ] All 10 Nielsen heuristics evaluated and scored 0.0-1.0
- [ ] Cognitive walkthrough complete for all key user tasks
- [ ] Scored report produced in .factory/ui-quality/heuristic-evaluation.md
- [ ] All scores below 0.7 flagged with remediation items

## Failure Modes

- If no UX spec exists: report gap and skip evaluation
- If screen recordings unavailable: evaluate from spec only and note reduced confidence
- If key user tasks not defined in spec: derive tasks from screen definitions, flag for human review
