# Commands Wrapper Generation Report

## Summary

- **Total commands created:** 47
- **Skipped (missing skill dir):** 0 — all target skills exist under `plugins/vsdd-factory/skills/`
- **Pre-existing command files left alone:** 0 — `commands/` directory did not exist prior to this run

## Location

`/Users/jmagady/Dev/vsdd-factory/plugins/vsdd-factory/commands/`

## Pattern

Each command is a thin wrapper (~6-10 lines) with frontmatter (`description`, optional `argument-hint`) and a body that delegates via `Use the \`<skill-name>\` skill via the Skill tool.` Commands taking arguments pass through `$ARGUMENTS`.

## Commands Created

Phase 0: brownfield-ingest, semport-analyze, disposition-pass
Phase 1: research, create-brief, guided-brief-creation, validate-brief, create-domain-spec, create-prd, create-architecture, adversarial-review
Phase 2: decompose-stories, create-story, wave-scheduling
Phase 3: deliver-story, pr-create, record-demo, worktree-manage, fix-pr-delivery, wave-gate, wave-status
Phase 4+: formal-verify, perf-check, dtu-validate, dtu-creation, convergence-check, release, holdout-eval
Cross-cutting: factory-health, factory-worktree-health, setup-env, track-debt, validate-consistency, spec-drift, state-update, next-step, mode-decision-guide, quick-dev-routing, multi-repo-health, maintenance-sweep, session-review
UI/Design: design-system-bootstrap, ui-quality-gate, ui-completeness-check, responsive-validation, ux-heuristic-evaluation, design-drift-detection
