workflow:
  name: per-story-delivery
  description: >
    Reusable per-story delivery sub-workflow. Invoked by greenfield.lobster,
    feature.lobster, maintenance.lobster, and multi-repo.lobster for each
    story that needs to be delivered. Encapsulates the complete flow:
    worktree -> stubs -> Red Gate -> implement -> micro-commits -> demo ->
    squash -> PR -> AI review -> security review -> converge -> merge -> cleanup.

    Cross-cutting infrastructure (DF-020-028) applies throughout. See
    docs/factory-paths.md Cross-Cutting Infrastructure section for shared baseline.
    Key additions from DF-029: refactored as reusable sub-workflow with typed
    inputs, UI/UX conditional steps (e2e-tester), security review conditional
    on criticality, information asymmetry walls for pr-reviewer, invoked by
    all code-producing paths.
    DF-037 additions: Storybook component test loop (D11/D18) with self-healing
    (max 10 iterations), per-story UI quality gate (D16) checking token compliance,
    a11y, component contract compliance, and async state coverage.
  version: "2.0.0"

  inputs:
    - story_id: "STORY-NNN"
    - worktree_path: ".worktrees/STORY-NNN"
    - feature_type: "ui | backend | full-stack | infrastructure"
    - module_criticality: "CRITICAL | HIGH | MEDIUM | LOW"
    - implementation_strategy: "tdd | gene-transfusion"

  defaults:
    on_failure: escalate
    max_retries: 2
    timeout: "1h"

  steps:
    # =========================================================================
    # Step 1: Create Worktree (if not already created by wave loop)
    # =========================================================================

    - name: create-worktree
      type: agent
      agent: devops-engineer
      depends_on: []
      condition: "worktree.not_exists == true"
      task: >
        Create git worktree for story:
        git worktree add [worktree_path] -b feature/[story_id] develop

    # =========================================================================
    # Step 2: Generate Stubs (Two-Step Red Gate)
    # =========================================================================

    - name: generate-stubs
      type: agent
      agent: test-writer
      depends_on: [create-worktree]
      task: >
        Create compilable stubs for [story_id] in [worktree_path].
        Build must pass (cargo check / tsc --noEmit / etc.).
        Do NOT write tests or implementation.

    # =========================================================================
    # Step 3: Red Gate (compile + fail)
    # =========================================================================

    - name: write-tests
      type: agent
      agent: test-writer
      depends_on: [generate-stubs]
      task: >
        Write failing tests for [story_id] in [worktree_path].
        Tests must compile but all fail (Red Gate).
        Do NOT implement.

    - name: red-gate
      type: gate
      depends_on: [write-tests]
      gate:
        criteria:
          - "Tests compile"
          - "All tests fail (Red Gate)"
        fail_action: block

    # =========================================================================
    # Step 4: Implement (TDD or Gene Transfusion)
    # =========================================================================

    - name: implement
      type: agent
      agent: implementer
      depends_on: [red-gate]
      task: >
        Implement [story_id] in [worktree_path] via TDD.
        Pick next failing test, write minimum code, repeat.
        All tests must pass when done. Micro-commits per test.
        If implementation_strategy == gene-transfusion:
          Use Semport translation workflow instead of writing from scratch.

    # =========================================================================
    # Step 5: E2E Tests (UI/full-stack only)
    # =========================================================================

    - name: e2e-tests
      type: agent
      agent: e2e-tester
      depends_on: [implement]
      condition: "feature_type in ['ui', 'full-stack']"
      task: >
        Run browser-based e2e tests for [story_id] in [worktree_path].
        Playwright/Cypress specs, visual snapshots.

    # =========================================================================
    # Step 5a: Storybook Component Tests (DF-037 D11/D18, UI only)
    # Self-healing loop: write stories -> run tests -> fix -> re-run (max 10)
    # =========================================================================

    - name: storybook-story-generation
      type: agent
      agent: test-writer
      depends_on: [implement]
      condition: "feature_type in ['ui', 'full-stack'] and storybook.available == true"
      task: >
        For each UI component created/modified in [story_id]:
        1. Call get-storybook-story-instructions (Storybook MCP) to learn project conventions
        2. Write Component.stories.tsx covering all variants x all states from component contract
        3. All required states from .factory/design-system/components/contracts/ must have stories

    - name: storybook-component-tests
      type: loop
      depends_on: [storybook-story-generation]
      condition: "feature_type in ['ui', 'full-stack'] and storybook.available == true"
      loop:
        max_iterations: 10
        exit_condition: "storybook_tests.all_pass"
        steps:
          - name: run-story-tests
            type: agent
            agent: test-writer
            task: >
              Call run-story-tests (Storybook MCP) with a11y: true.
              Report pass/fail + specific failure details + a11y violations.

          - name: fix-component
            type: agent
            agent: implementer
            depends_on: [run-story-tests]
            condition: "storybook_tests.has_failures"
            task: >
              Read failure details from run-story-tests.
              Fix component code in [worktree_path].
              If a11y violations: fix using component contract a11y requirements.

    # =========================================================================
    # Step 5b: Per-Story UI Quality Gate (DF-037 D16, lightweight)
    # Token compliance + a11y + component contract
    # =========================================================================

    - name: per-story-ui-quality-gate
      type: agent
      agent: consistency-validator
      depends_on: [implement, storybook-component-tests]
      condition: "feature_type in ['ui', 'full-stack']"
      task: >
        Per-story UI quality gate (lightweight):
        1. Token compliance: verify component uses design tokens, no hardcoded CSS
        2. A11y: axe-core zero violations on rendered page
        3. Component contract: required states implemented per contract
        4. Async states: loading/success/empty/error for data-fetching components
        Block merge if any check fails. Log failures to
        .factory/ui-quality/per-story-gate-[story_id].md.

    # =========================================================================
    # Step 6: Demo Recording
    # =========================================================================

    - name: demo-recording
      type: agent
      agent: demo-recorder
      depends_on: [implement]
      task: >
        Record per-AC demo evidence for [story_id] in [worktree_path]:
        1. Read story acceptance criteria
        2. For each AC: create VHS .tape (CLI) or Playwright spec (web)
        3. Execute to produce .gif + .webm
        4. Generate evidence report
        5. Commit: git add docs/demo-evidence/
      timeout: "30m"

    # =========================================================================
    # Step 7: Squash and Push
    # =========================================================================

    - name: squash-and-push
      type: agent
      agent: implementer
      depends_on: [demo-recording, e2e-tests]
      task: >
        Squash WIP commits into clean conventional commits.
        Push with --force-with-lease:
        git push --force-with-lease origin feature/[story_id]

    # =========================================================================
    # Step 8: Create PR (Mermaid diagrams, evidence)
    # =========================================================================

    - name: create-pr
      type: agent
      agent: pr-manager
      depends_on: [squash-and-push]
      task: >
        Create PR for [story_id] with structured description from
        templates/pr-description-template.md. Include:
        - Story traceability (BC -> AC -> Test -> Implementation)
        - Test evidence (pass count, coverage, mutation kill rate)
        - Demo evidence (embedded GIF thumbnails)
        - Mermaid diagrams (architecture, dependency, traceability)
        Delegate gh pr create to github-ops.

    # =========================================================================
    # Step 9: AI Review (Gemini, information asymmetry wall)
    # =========================================================================

    - name: ai-pr-review
      type: agent
      agent: pr-reviewer
      depends_on: [create-pr]
      task: >
        Review PR diff for [story_id]. Evaluate 8-item checklist:
        diff coherence, description accuracy, test coverage, demo evidence,
        commit quality, diff size, missing changes, dependency status.
        Post findings on GitHub via github-ops.
      context:
        exclude:
          # Information asymmetry wall: PR reviewer sees only the PR diff
          - ".factory/**"

    # =========================================================================
    # Step 10: Security Review (conditional on criticality)
    # =========================================================================

    - name: security-review
      type: agent
      agent: security-reviewer
      depends_on: [create-pr]
      condition: "module_criticality in ['CRITICAL', 'HIGH']"
      task: >
        Security review PR diff for [story_id]. CWE/OWASP analysis of
        changed lines. Post findings as inline review comments via github-ops.
        Max 3 security review cycles.
      context:
        exclude:
          # Information asymmetry wall: no implementer reasoning
          - ".factory/cycles/**/implementation/implementer-notes*"

    # =========================================================================
    # Step 11: PR Review Convergence (max 10 rounds)
    # =========================================================================

    - name: pr-review-convergence
      type: loop
      depends_on: [ai-pr-review]
      condition: "pr_reviewer.verdict == 'REQUEST_CHANGES'"
      loop:
        max_iterations: 10
        exit_condition: "pr_reviewer.verdict == 'APPROVE'"
        steps:
          - name: triage-findings
            type: agent
            agent: pr-manager
            task: >
              Triage pr-reviewer findings using pr-review-triage skill.
              Classify by severity. Route fixes to implementer/test-writer.

          - name: fix-findings
            type: agent
            agent: implementer
            depends_on: [triage-findings]
            task: >
              Fix review findings in [worktree_path].
              Commit fixes, push with --force-with-lease.

          - name: re-review
            type: agent
            agent: pr-reviewer
            depends_on: [fix-findings]
            task: >
              Re-review PR diff for [story_id]. Focus on whether previous
              blocking findings are resolved.
            context:
              exclude:
                - ".factory/**"

    # =========================================================================
    # Step 12: Brownfield Regression Gate (conditional)
    # =========================================================================

    - name: brownfield-full-regression
      type: agent
      agent: implementer
      depends_on: [pr-review-convergence]
      condition: "mode == 'brownfield'"
      task: >
        BROWNFIELD REGRESSION GATE: Run FULL existing test suite against
        worktree. If ANY existing test fails, HALT.

    - name: brownfield-codeowners-check
      type: agent
      agent: consistency-validator
      depends_on: [create-pr]
      condition: "mode == 'brownfield'"
      task: >
        Check if any files in the PR diff match CODEOWNERS patterns.
        If restricted files touched, add 'needs-review' label.

    # =========================================================================
    # Step 13: Wait for CI
    # =========================================================================

    - name: wait-for-ci
      type: loop
      depends_on: [pr-review-convergence, brownfield-full-regression]
      loop:
        max_iterations: 3
        exit_condition: "ci.status == 'all_passed'"
        steps:
          - name: poll-ci-status
            type: agent
            agent: pr-manager
            task: >
              Poll GitHub for CI status checks via github-ops.
              Wait up to 30 minutes.
            timeout: "35m"

          - name: fix-ci-failure
            type: agent
            agent: implementer
            depends_on: [poll-ci-status]
            condition: "ci.status == 'failed'"
            task: >
              Fix CI failure. Push to same branch. CI re-runs.

    # =========================================================================
    # Step 14: Dependency-Ordered Merge Check
    # =========================================================================

    - name: dependency-merge-check
      type: agent
      agent: pr-manager
      depends_on: [wait-for-ci]
      task: >
        Verify all dependency PRs for [story_id] are merged.
        If any dep PR NOT merged: WAIT.
        After all deps merged: rebase onto develop, re-verify tests.

    # =========================================================================
    # Step 15: Merge
    # =========================================================================

    - name: merge-pr
      type: agent
      agent: pr-manager
      depends_on: [dependency-merge-check]
      task: >
        Merge PR via github-ops: gh pr merge --squash --delete-branch.
        Read .factory/merge-config.yaml for autonomy level.
        Level 3: label for human review.
        Level 3.5: auto-merge low-risk, label medium/high.
        Level 4: auto-merge with squash.

    - name: delivery-human-approval
      type: human-approval
      depends_on: [merge-pr]
      condition: "merge_decision.requires_human == true"
      approval:
        prompt: "Code delivery complete. PR created with evidence. Review and approve."
        timeout: "24h"

    # =========================================================================
    # Step 16: Post-Merge Cleanup
    # =========================================================================

    - name: cleanup-worktree
      type: agent
      agent: devops-engineer
      depends_on: [merge-pr, delivery-human-approval]
      task: >
        Remove worktree: git worktree remove [worktree_path].
        Update .factory/STATE.md with merge status, PR number, timestamp.
