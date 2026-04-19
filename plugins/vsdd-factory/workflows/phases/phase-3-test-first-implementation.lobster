workflow:
  name: phase-3-test-first-implementation
  description: >
    Phase 3: Test-First Implementation. Per-story TDD delivery through specialist
    subagents (devops-engineer, test-writer, implementer, demo-recorder, pr-manager).
    Steps follow deliver-story skill's dispatch sequence with state-manager backups
    for crash recovery. This workflow runs once per story.
  version: "2.0.0"

  defaults:
    on_failure: escalate
    max_retries: 2
    timeout: "4h"

  steps:
    # === Step A: Create Worktree ===

    - name: create-worktree
      type: skill
      skill: "skills/deliver-story/steps/step-a-create-worktree.md"
      depends_on: []

    - name: backup-create-worktree
      type: agent
      agent: state-manager
      depends_on: [create-worktree]
      task: >
        Update STATE.md: phase: 3, step: create-worktree, status: complete,
        story: STORY-NNN.

    # === Step B: Generate Stubs ===

    - name: generate-stubs
      type: skill
      skill: "skills/deliver-story/steps/step-b-generate-stubs.md"
      depends_on: [backup-create-worktree]

    - name: backup-generate-stubs
      type: agent
      agent: state-manager
      depends_on: [generate-stubs]
      task: >
        Update STATE.md: phase: 3, step: generate-stubs, status: complete.

    # === Step C: Write Failing Tests + Red Gate ===

    - name: failing-tests
      type: skill
      skill: "skills/deliver-story/steps/step-c-failing-tests.md"
      depends_on: [backup-generate-stubs]

    - name: backup-failing-tests
      type: agent
      agent: state-manager
      depends_on: [failing-tests]
      task: >
        Commit red-gate-log.md to factory-artifacts.
        Update STATE.md: phase: 3, step: failing-tests, status: complete.

    # === Step D: TDD Implementation ===

    - name: implement
      type: skill
      skill: "skills/deliver-story/steps/step-d-implement.md"
      depends_on: [backup-failing-tests]
      timeout: "2h"

    - name: backup-implement
      type: agent
      agent: state-manager
      depends_on: [implement]
      task: >
        Update STATE.md: phase: 3, step: implement, status: complete.

    # === Step E: Record Demos ===

    - name: record-demos
      type: skill
      skill: "skills/deliver-story/steps/step-e-record-demos.md"
      depends_on: [backup-implement]

    - name: backup-record-demos
      type: agent
      agent: state-manager
      depends_on: [record-demos]
      task: >
        Update STATE.md: phase: 3, step: record-demos, status: complete.

    # === Step F: Push + PR Lifecycle ===

    - name: pr-lifecycle
      type: skill
      skill: "skills/deliver-story/steps/step-f-pr-lifecycle.md"
      depends_on: [backup-record-demos]
      timeout: "1h"

    - name: backup-pr-lifecycle
      type: agent
      agent: state-manager
      depends_on: [pr-lifecycle]
      task: >
        Update STATE.md: phase: 3, step: pr-lifecycle, status: complete.

    # === Step G: Cleanup + State Update ===

    - name: cleanup
      type: skill
      skill: "skills/deliver-story/steps/step-g-cleanup.md"
      depends_on: [backup-pr-lifecycle]

    - name: backup-cleanup
      type: agent
      agent: state-manager
      depends_on: [cleanup]
      task: >
        Update STATE.md: phase: 3, step: cleanup, status: complete.
        Update sprint-state.yaml: story status completed.

    # === Implementation Gate ===

    - name: implementation-gate
      type: gate
      depends_on: [backup-cleanup]
      gate:
        criteria:
          - "Red Gate passed -- all tests failed before implementation"
          - "All tests now pass"
          - "Implementation follows purity boundary map"
          - "No unwrap() in production code"
          - "Clippy/lint passes with zero warnings"
          - "Demo evidence covers all acceptance criteria"
          - "PR merged to develop"
          - "Worktree cleaned up"
        fail_action: block

    # === Input-Hash Drift Check ===

    - name: input-hash-drift-check
      type: skill
      skill: "skills/check-input-drift/SKILL.md"
      depends_on: [implementation-gate]
      description: >
        Verify spec artifacts have not drifted since implementation began.
