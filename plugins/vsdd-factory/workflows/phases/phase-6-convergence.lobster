workflow:
  name: phase-6-convergence
  description: >
    Phase 6 decomposed into reusable sub-workflow. Five-dimensional convergence
    assessment with demo recording and visual review integration.
  version: "1.0.0"

  defaults:
    on_failure: escalate
    max_retries: 2
    timeout: "2h"

  steps:
    - name: convergence-assessment
      type: skill
      skill: "skills/phase-6-convergence/SKILL.md"
      depends_on: []

    - name: input-hash-drift-check
      type: skill
      skill: "skills/check-input-drift/SKILL.md"
      depends_on: [convergence-assessment]
      description: >
        Final drift check before convergence gate. All artifacts must have
        current input-hashes — stale hashes indicate unresolved spec drift.

    - name: convergence-gate
      type: gate
      depends_on: [convergence-assessment, input-hash-drift-check]
      gate:
        criteria:
          - "Spec dimension: CONVERGED"
          - "Test dimension: CONVERGED"
          - "Implementation dimension: CONVERGED"
          - "Verification dimension: CONVERGED"
          - "Holdout dimension: CONVERGED (satisfaction >= 0.85)"
          - "Traceability matrix generated"
          - "Convergence report generated"
        fail_action: block

    - name: convergence-demo
      type: skill
      skill: "skills/demo-recording/SKILL.md"
      depends_on: [convergence-gate]
      condition: "config.demo_recording.enabled != false"
      timeout: "30m"

    - name: visual-review
      type: agent
      agent: visual-reviewer
      model_tier: review
      depends_on: [convergence-demo]
      condition: "config.demo_recording.enabled != false"
      task: >
        Analyze convergence demo recordings. Verify the "money shot" full
        user journey demo visually demonstrates all acceptance criteria.
        Write findings to .factory/demo-evidence/visual-review.md.

    - name: human-approval
      type: human-approval
      depends_on: [convergence-gate, visual-review]
      approval:
        prompt: >
          ZERO-SLOP ACHIEVED. All five convergence dimensions CONVERGED.
          Review convergence report, traceability matrix, and demo evidence.
        artifacts:
          - ".factory/cycles/**/convergence/convergence-report.md"
          - ".factory/cycles/**/convergence/traceability-matrix.md"
          - ".factory/demo-evidence/visual-review.md"
        timeout: "48h"
