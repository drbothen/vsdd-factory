workflow:
  name: phase-3-5-holdout-evaluation
  description: >
    Phase 3.5 decomposed into reusable sub-workflow. Scenario rotation and
    holdout evaluation with information asymmetry wall enforcement.
  version: "1.0.0"

  defaults:
    on_failure: escalate
    max_retries: 2
    timeout: "2h"

  steps:
    - name: scenario-rotation
      type: agent
      agent: orchestrator
      depends_on: []
      task: >
        Perform scenario rotation for this pipeline run. Randomly select 80%
        of holdout scenarios. Write selected scenario IDs to
        .factory/holdout-evaluation/scenario-selection.json.

    - name: holdout-evaluation
      type: skill
      skill: "skills/holdout-eval/SKILL.md"
      depends_on: [scenario-rotation]

    - name: holdout-gate
      type: gate
      depends_on: [holdout-evaluation]
      gate:
        criteria:
          - "Holdout evaluator used different model family (GPT-5.4, not Claude)"
          - "Mean satisfaction score >= 0.85"
          - "No must-pass scenario below 0.6 satisfaction"
          - "Satisfaction standard deviation < 0.15"
          - "Scenario rotation applied (80% subset selected)"
        fail_action: block
