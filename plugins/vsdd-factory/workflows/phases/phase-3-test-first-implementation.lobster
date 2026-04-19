workflow:
  name: phase-3-test-first-implementation
  description: >
    Phase 3 decomposed into reusable sub-workflow. Wave-based parallel execution
    of TDD implementation across stories with no dependency edges between them.
  version: "1.0.0"

  defaults:
    on_failure: escalate
    max_retries: 2
    timeout: "8h"

  steps:
    - name: test-first-implementation
      type: skill
      skill: "skills/phase-3-test-first-implementation/SKILL.md"
      depends_on: []

    - name: input-hash-drift-check
      type: skill
      skill: "skills/check-input-drift/SKILL.md"
      depends_on: [test-first-implementation]
      description: >
        Verify spec artifacts have not drifted since implementation began.
        Stale input-hashes indicate specs changed without downstream updates.

    - name: implementation-gate
      type: gate
      depends_on: [test-first-implementation, input-hash-drift-check]
      gate:
        criteria:
          - "Red Gate passed -- all tests failed before implementation"
          - "All tests now pass"
          - "Implementation follows purity boundary map"
          - "No unwrap() in production code"
          - "Clippy/lint passes with zero warnings"
        fail_action: block
