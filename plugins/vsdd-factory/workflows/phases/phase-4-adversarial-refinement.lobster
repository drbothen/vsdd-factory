workflow:
  name: phase-4-adversarial-refinement
  description: >
    Phase 4 decomposed into reusable sub-workflow. Multi-model adversarial
    review loop with GPT-5.4 primary and Gemini 3.1 Pro secondary pass.
  version: "1.0.0"

  defaults:
    on_failure: escalate
    max_retries: 2
    timeout: "3h"

  steps:
    - name: adversarial-review-loop
      type: loop
      depends_on: []
      loop:
        max_iterations: 10
        exit_condition: "adversary.verdict == 'CONVERGENCE_REACHED'"
        steps:
          - name: adversary-code-review
            type: agent
            agent: adversary
            model_tier: adversary
            task: >
              Review complete codebase with fresh context. Check spec fidelity,
              test quality, code quality, security surface, and spec gaps.
              Report CONVERGENCE_REACHED if findings are cosmetic only.
            context:
              include:
                - ".factory/specs/**"
                - ".factory/cycles/**/implementation/**"
                - "src/**"
                - "tests/**"

          - name: triage-and-fix
            type: skill
            skill: "skills/adversarial-review/SKILL.md"
            depends_on: [adversary-code-review]

    - name: gemini-secondary-review
      type: agent
      depends_on: [adversarial-review-loop]
      agent: code-reviewer
      model_tier: review
      task: >
        Secondary adversarial review using Gemini 3.1 Pro. Fresh context.
        Focus on patterns the GPT-5.4 adversary may have missed.
      condition: "config.enable_secondary_adversary == true"
      context:
        include:
          - ".factory/specs/**"
          - "src/**"
          - "tests/**"
