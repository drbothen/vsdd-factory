workflow:
  name: phase-1-spec-crystallization
  description: >
    Phase 1 decomposed into reusable sub-workflow. Transforms product brief into
    PRD, architecture, verification architecture, UX spec, and adversarial review.
    Can be invoked standalone or as part of greenfield/brownfield pipelines.
  version: "1.0.0"

  defaults:
    on_failure: escalate
    max_retries: 2
    timeout: "2h"

  steps:
    - name: spec-crystallization
      type: skill
      skill: "skills/phase-1-spec-crystallization/SKILL.md"
      depends_on: []

    - name: spec-gate
      type: gate
      depends_on: [spec-crystallization]
      gate:
        criteria:
          - "All requirements have unique IDs and numerical targets"
          - "Provable Properties Catalog covers all security boundaries"
          - "Purity Boundary Map is complete -- every module classified"
          - "Verification tooling selected and constraints documented"
          - "Module criticality classification written to .factory/module-criticality.md"
        fail_action: block

    - name: adversarial-spec-review
      type: loop
      depends_on: [spec-gate]
      loop:
        max_iterations: 10
        exit_condition: "adversary.verdict == 'CONVERGENCE_REACHED'"
        steps:
          - name: spawn-adversary
            type: agent
            agent: adversary
            model_tier: adversary
            task: >
              Review all Phase 1 spec artifacts with fresh context. Check for
              ambiguity, missing edge cases, contradictions, verification gaps.
              Write findings to .factory/specs/adversarial-spec-review.md.
              Report CONVERGENCE_REACHED if findings are cosmetic only.
            context:
              include:
                - ".factory/specs/**"
                - ".factory/module-criticality.md"
              exclude:
                - ".factory/holdout-scenarios/**"

          - name: fix-findings
            type: skill
            skill: "skills/phase-1d-adversarial-spec-review/SKILL.md"
            depends_on: [spawn-adversary]

    - name: human-approval
      type: human-approval
      depends_on: [adversarial-spec-review]
      approval:
        prompt: >
          Phase 1 Spec Crystallization is complete. Review the spec package.
        artifacts:
          - ".factory/specs/prd.md"
          - ".factory/specs/architecture.md"
          - ".factory/specs/verification-architecture.md"
          - ".factory/specs/ux-spec.md"
          - ".factory/module-criticality.md"
        timeout: "24h"
