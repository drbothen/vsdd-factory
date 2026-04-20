workflow:
  name: phase-2-story-decomposition
  description: >
    Phase 2: Story Decomposition. Breaks PRD and architecture into epics,
    stories, dependency graph, wave schedule, and holdout scenarios. Steps
    follow decompose-stories skill's internal structure with state-manager
    backups for crash recovery.
  version: "1.0.0"

  defaults:
    on_failure: escalate
    max_retries: 2
    timeout: "2h"

  steps:
    # === Step A: Define Epics ===

    - name: define-epics
      type: skill
      skill: "skills/decompose-stories/steps/step-a-define-epics.md"
      depends_on: []

    - name: backup-define-epics
      type: agent
      agent: state-manager
      depends_on: [define-epics]
      task: >
        Commit epics.md to factory-artifacts.
        Update STATE.md: phase: 2, step: define-epics, status: complete.

    # === Step B: Create Stories ===

    - name: create-stories
      type: skill
      skill: "skills/decompose-stories/steps/step-b-create-stories.md"
      depends_on: [backup-define-epics]
      timeout: "1h"

    - name: backup-create-stories
      type: agent
      agent: state-manager
      depends_on: [create-stories]
      task: >
        Commit all STORY-NNN.md files to factory-artifacts.
        Update STATE.md: phase: 2, step: create-stories, status: complete.

    # === Step C: Dependency Graph ===

    - name: dependency-graph
      type: skill
      skill: "skills/decompose-stories/steps/step-c-dependency-graph.md"
      depends_on: [backup-create-stories]

    - name: backup-dependency-graph
      type: agent
      agent: state-manager
      depends_on: [dependency-graph]
      task: >
        Commit dependency-graph.md to factory-artifacts.
        Update STATE.md: phase: 2, step: dependency-graph, status: complete.

    # === Step D: Wave Schedule + Story Index + Sprint State ===

    - name: wave-schedule
      type: skill
      skill: "skills/decompose-stories/steps/step-d-wave-schedule.md"
      depends_on: [backup-dependency-graph]

    - name: backup-wave-schedule
      type: agent
      agent: state-manager
      depends_on: [wave-schedule]
      task: >
        Commit wave-schedule.md, STORY-INDEX.md, sprint-state.yaml to factory-artifacts.
        Update STATE.md: phase: 2, step: wave-schedule, status: complete.

    # === Step E: Holdout Scenarios ===

    - name: holdout-scenarios
      type: skill
      skill: "skills/decompose-stories/steps/step-e-holdout-scenarios.md"
      depends_on: [backup-wave-schedule]

    - name: backup-holdout-scenarios
      type: agent
      agent: state-manager
      depends_on: [holdout-scenarios]
      task: >
        Commit holdout scenario files to factory-artifacts.
        Update STATE.md: phase: 2, step: holdout-scenarios, status: complete.

    # === Decomposition Gate ===

    - name: decomposition-gate
      type: gate
      depends_on: [backup-holdout-scenarios]
      gate:
        criteria:
          - "Every BC in PRD traces to at least one story"
          - "No story contains TBD, TODO, or placeholder acceptance criteria"
          - "Dependency graph has no circular dependencies"
          - "Wave assignments respect dependency ordering"
          - "STORY-INDEX matches individual story files"
          - "At least one holdout scenario per wave"
        fail_action: block

    # === Adversarial Story Review (convergence loop) ===

    - name: adversarial-story-review
      type: loop
      depends_on: [decomposition-gate]
      loop:
        max_iterations: 10
        exit_condition: "adversary.verdict == 'CONVERGENCE_REACHED'"
        steps:
          - name: spawn-adversary-story-review
            type: agent
            agent: adversary
            model_tier: adversary
            task: >
              Review story breakdown with fresh context. Check for missing
              stories, oversized stories, insufficient verification properties,
              dependency ordering issues, BC coverage gaps, AC↔BC traceability,
              holdout scenario completeness.
              Write findings to .factory/stories/adversarial-reviews/ as
              per-finding files with ADV-P[N]-INDEX.md.
              Report CONVERGENCE_REACHED if findings are cosmetic only.
            context:
              include:
                - ".factory/stories/**"
                - ".factory/specs/behavioral-contracts/**"
                - ".factory/specs/verification-properties/**"
                - ".factory/specs/prd.md"
                - ".factory/specs/prd-supplements/**"
                - ".factory/specs/architecture/ARCH-INDEX.md"
              exclude:
                # ▓ WALL: no holdout scenarios (train/test separation)
                - ".factory/holdout-scenarios/**"
                # ▓ WALL: no prior adversarial history (fresh eyes)
                - ".factory/stories/adversarial-reviews/**"

          - name: fix-story-findings
            type: skill
            skill: "skills/decompose-stories/SKILL.md"
            depends_on: [spawn-adversary-story-review]

    # === Input-Hash Drift Check ===

    - name: input-hash-drift-check
      type: skill
      skill: "skills/check-input-drift/SKILL.md"
      depends_on: [adversarial-story-review]
      description: >
        Scan story and holdout artifacts for stale input-hashes.

    # === Human Approval ===

    - name: human-approval
      type: human-approval
      depends_on: [input-hash-drift-check]
      approval:
        prompt: >
          Phase 2 Story Decomposition is complete. Review the stories and wave schedule.
        artifacts:
          - ".factory/stories/epics.md"
          - ".factory/stories/STORY-INDEX.md"
          - ".factory/stories/dependency-graph.md"
          - ".factory/cycles/**/wave-schedule.md"
          - ".factory/stories/sprint-state.yaml"
          - ".factory/holdout-scenarios/HS-INDEX.md"
        timeout: "24h"
