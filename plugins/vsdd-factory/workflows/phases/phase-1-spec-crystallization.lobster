workflow:
  name: phase-1-spec-crystallization
  description: >
    Phase 1: Spec Crystallization. Transforms product brief into domain spec,
    PRD, behavioral contracts, architecture, verification properties, and UX spec.
    Steps use existing skills (create-brief, create-domain-spec, create-prd,
    create-architecture) with state-manager backups for crash recovery.
  version: "2.0.0"

  defaults:
    on_failure: escalate
    max_retries: 2
    timeout: "2h"

  steps:
    # === Step 1a: Product Brief ===

    - name: create-brief
      type: skill
      skill: "skills/create-brief/SKILL.md"
      depends_on: []

    - name: backup-create-brief
      type: agent
      agent: state-manager
      depends_on: [create-brief]
      task: >
        Commit product brief to factory-artifacts.
        Update STATE.md: phase: 1, step: create-brief, status: complete.

    # === Step 1b: Domain Spec (L2) ===

    - name: create-domain-spec
      type: skill
      skill: "skills/create-domain-spec/SKILL.md"
      depends_on: [backup-create-brief]

    - name: backup-create-domain-spec
      type: agent
      agent: state-manager
      depends_on: [create-domain-spec]
      task: >
        Commit L2 domain spec to factory-artifacts.
        Update STATE.md: phase: 1, step: create-domain-spec, status: complete.

    # === Step 1c: PRD + Behavioral Contracts ===

    - name: create-prd
      type: skill
      skill: "skills/create-prd/SKILL.md"
      depends_on: [backup-create-domain-spec]

    - name: backup-create-prd
      type: agent
      agent: state-manager
      depends_on: [create-prd]
      task: >
        Commit PRD and behavioral contracts to factory-artifacts.
        Update STATE.md: phase: 1, step: create-prd, status: complete.

    # === Step 1d: Architecture + VPs ===

    - name: create-architecture
      type: skill
      skill: "skills/create-architecture/SKILL.md"
      depends_on: [backup-create-prd]

    - name: backup-create-architecture
      type: agent
      agent: state-manager
      depends_on: [create-architecture]
      task: >
        Commit architecture and verification properties to factory-artifacts.
        Update STATE.md: phase: 1, step: create-architecture, status: complete.

    # === Step 1e: PRD Revision (architect feedback loop) ===

    - name: prd-revision
      type: skill
      skill: "skills/phase-1-prd-revision/SKILL.md"
      depends_on: [backup-create-architecture]
      condition: "exists('.factory/specs/architecture-feasibility-report.md')"

    - name: backup-prd-revision
      type: agent
      agent: state-manager
      depends_on: [prd-revision]
      task: >
        Commit PRD revisions to factory-artifacts.
        Update STATE.md: phase: 1, step: prd-revision, status: complete.

    # === Spec Gate ===

    - name: spec-gate
      type: gate
      depends_on: [backup-create-architecture, backup-prd-revision]
      gate:
        criteria:
          - "All requirements have unique IDs and numerical targets"
          - "Provable Properties Catalog covers all security boundaries"
          - "Purity Boundary Map is complete -- every module classified"
          - "Verification tooling selected and constraints documented"
          - "Module criticality classification written to .factory/module-criticality.md"
        fail_action: block

    # === Step 1f: Adversarial Spec Review (convergence loop) ===

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

    # === Input-Hash Drift Check ===

    - name: input-hash-drift-check
      type: skill
      skill: "skills/check-input-drift/SKILL.md"
      depends_on: [adversarial-spec-review]
      description: >
        Scan all .factory/specs/ artifacts for stale input-hashes.
        Any DRIFT results must be resolved before human approval.

    # === Human Approval ===

    - name: human-approval
      type: human-approval
      depends_on: [input-hash-drift-check]
      approval:
        prompt: >
          Phase 1 Spec Crystallization is complete. Review the spec package.
        artifacts:
          - ".factory/specs/prd.md"
          - ".factory/specs/architecture/"
          - ".factory/specs/verification-properties/"
          - ".factory/specs/behavioral-contracts/"
          - ".factory/specs/ux-spec.md"
          - ".factory/module-criticality.md"
        timeout: "24h"
