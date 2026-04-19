workflow:
  name: phase-7-convergence
  description: >
    Phase 6: 7-Dimensional Convergence Assessment. Each dimension is checked
    independently with state-manager backups for crash recovery. Followed by
    demo recording, visual review, and human approval.
  version: "2.0.0"

  defaults:
    on_failure: escalate
    max_retries: 2
    timeout: "2h"

  steps:
    # === Dimension 1: Spec Convergence ===

    - name: spec-convergence
      type: skill
      skill: "skills/convergence-check/steps/step-a-spec-convergence.md"
      depends_on: []

    - name: backup-spec-convergence
      type: agent
      agent: state-manager
      depends_on: [spec-convergence]
      task: >
        Update STATE.md: phase: 7, step: spec-convergence, status: complete.

    # === Dimension 2: Test Convergence ===

    - name: test-convergence
      type: skill
      skill: "skills/convergence-check/steps/step-b-test-convergence.md"
      depends_on: [backup-spec-convergence]

    - name: backup-test-convergence
      type: agent
      agent: state-manager
      depends_on: [test-convergence]
      task: >
        Update STATE.md: phase: 7, step: test-convergence, status: complete.

    # === Dimension 3: Implementation Convergence ===

    - name: implementation-convergence
      type: skill
      skill: "skills/convergence-check/steps/step-c-implementation-convergence.md"
      depends_on: [backup-test-convergence]

    - name: backup-implementation-convergence
      type: agent
      agent: state-manager
      depends_on: [implementation-convergence]
      task: >
        Update STATE.md: phase: 7, step: implementation-convergence, status: complete.

    # === Dimension 4: Verification Convergence ===

    - name: verification-convergence
      type: skill
      skill: "skills/convergence-check/steps/step-d-verification-convergence.md"
      depends_on: [backup-implementation-convergence]

    - name: backup-verification-convergence
      type: agent
      agent: state-manager
      depends_on: [verification-convergence]
      task: >
        Update STATE.md: phase: 7, step: verification-convergence, status: complete.

    # === Dimension 5: Visual Convergence ===

    - name: visual-convergence
      type: skill
      skill: "skills/convergence-check/steps/step-e-visual-convergence.md"
      depends_on: [backup-verification-convergence]

    - name: backup-visual-convergence
      type: agent
      agent: state-manager
      depends_on: [visual-convergence]
      task: >
        Update STATE.md: phase: 7, step: visual-convergence, status: complete.

    # === Dimension 6: Performance Convergence ===

    - name: performance-convergence
      type: skill
      skill: "skills/convergence-check/steps/step-f-performance-convergence.md"
      depends_on: [backup-visual-convergence]

    - name: backup-performance-convergence
      type: agent
      agent: state-manager
      depends_on: [performance-convergence]
      task: >
        Update STATE.md: phase: 7, step: performance-convergence, status: complete.

    # === Dimension 7: Documentation Convergence ===

    - name: documentation-convergence
      type: skill
      skill: "skills/convergence-check/steps/step-g-documentation-convergence.md"
      depends_on: [backup-performance-convergence]

    - name: backup-documentation-convergence
      type: agent
      agent: state-manager
      depends_on: [documentation-convergence]
      task: >
        Commit convergence report to factory-artifacts.
        Update STATE.md: phase: 7, step: documentation-convergence, status: complete.

    # === Convergence Gate ===

    - name: convergence-gate
      type: gate
      depends_on: [backup-documentation-convergence]
      gate:
        criteria:
          - "Spec dimension: CONVERGED"
          - "Test dimension: CONVERGED"
          - "Implementation dimension: CONVERGED"
          - "Verification dimension: CONVERGED"
          - "Visual dimension: CONVERGED"
          - "Performance dimension: CONVERGED"
          - "Documentation dimension: CONVERGED"
          - "Traceability matrix generated"
          - "Convergence report generated"
        fail_action: block

    # === Input-Hash Drift Check ===

    - name: input-hash-drift-check
      type: skill
      skill: "skills/check-input-drift/SKILL.md"
      depends_on: [convergence-gate]
      description: >
        Final drift check before release. All artifacts must have current
        input-hashes.

    # === Demo + Visual Review ===

    - name: convergence-demo
      type: skill
      skill: "skills/demo-recording/SKILL.md"
      depends_on: [input-hash-drift-check]
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

    # === Human Approval ===

    - name: human-approval
      type: human-approval
      depends_on: [convergence-gate, visual-review]
      approval:
        prompt: >
          ZERO-SLOP ACHIEVED. All seven convergence dimensions CONVERGED.
          Review convergence report, traceability matrix, and demo evidence.
        artifacts:
          - ".factory/cycles/**/convergence/convergence-report.md"
          - ".factory/cycles/**/convergence/traceability-matrix.md"
          - ".factory/demo-evidence/visual-review.md"
        timeout: "48h"
