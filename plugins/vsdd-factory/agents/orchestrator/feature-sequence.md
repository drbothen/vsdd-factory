---
name: orchestrator-feature-sequence
description: Orchestrator workflow reference for the feature-mode delivery sequence on existing codebases. Loaded by the orchestrator agent during the corresponding phase. Not directly invokable.
disable-model-invocation: true
---

> **Global Operating Rules:** Read `../../docs/FACTORY.md` and `../../docs/VSDD.md` for factory-wide constraints.


# Feature Mode Delegation Sequence

Reference file for the orchestrator. Load when running feature mode (Path 3).

## Overview

Feature mode adds incremental capabilities to an existing VSDD-managed codebase.
All work is scoped to the delta — changed/new code only. Regression testing
protects existing functionality.

## Feature Cycle Initialization

Spawn state-manager:
1. Create cycle directory: cycles/vX.Y.Z-feature-NAME/
2. Initialize cycle-manifest.md from template
3. Update STATE.md: pipeline: FEATURE-CYCLE, active_cycle: vX.Y.Z-feature-NAME
4. Pause maintenance sweeps (avoid conflicts on develop branch)

## Environment Check

Spawn dx-engineer: "Verify development environment is current. Check tools,
LLM availability, MCP servers."

## Market Intelligence (skip for bug fixes)

Spawn business-analyst + research-agent: "Market intelligence for proposed
feature, scoped to existing product."
HUMAN reviews GO/CAUTION/STOP.

## Demo Baseline

Spawn demo-recorder: "Record current state of affected feature areas for
before/after comparison."

## Phase F1: Delta Analysis

Spawn architect: "Analyze impact boundary for the requested feature:
- Identify affected modules, BCs, VPs, stories
- Classify: feature / enhancement / bug-fix
- Determine scope: which specs need evolution
- Check for cross-repo impact (if multi-repo)
Write delta-analysis.md."

### Quick-Dev Routing

If delta analysis classifies as trivial (single module, no spec change,
no new BCs needed):
- Skip F2/F3, create single story directly
- Run F4 (single story delivery) -> regression -> F7 lite -> PATCH release
- See `per-story-delivery.md` for delivery cycle

### Bug-Fix Routing

If delta analysis classifies as bug fix:
- Create fix story with mandatory regression test
- Scoped holdout evaluation
- Compressed F5/F6/F7 -> PATCH release

## Phase F2: Spec Evolution

Spawn product-owner: "Evolve L3 PRD and behavioral contracts for the delta.
Add new BCs, modify existing BCs (version bump), deprecate removed BCs."

Spawn architect: "Update architecture for the delta. DTU re-assessment if
new external dependencies. Gene transfusion assessment if reference implementations."

For UI features:
- Spawn ux-designer: "Evolve UX spec for affected screens/flows"
- Spawn accessibility-auditor: "Review updated UX spec for a11y compliance"
- Run design system bootstrap if new components needed

Spec evolution review loop (max 10 passes):
a. Spawn adversary: "Review spec evolution delta"
b. If convergence reached (see VSDD.md Phase 6 Spec dimension) → exit loop
c. Spawn spec-reviewer: "Constructive review of evolved specs"
d. Route findings per VSDD.md Feedback Integration Loop
e. Go back to (a) with fresh adversary context

HUMAN APPROVAL.

## Phase F3: Incremental Stories

Spawn story-writer: "Create per-file stories for the delta. Wave schedule,
holdout scenarios for new behavior. AC->BC tracing."

Spawn consistency-validator: "Validate stories against evolved specs."

Story review loop (max 10 passes):
a. Spawn adversary: "Review story decomposition"
b. If convergence reached (see VSDD.md Phase 6 Spec dimension) → exit loop
c. Spawn spec-reviewer: "Constructive story review"
d. Route findings per VSDD.md Feedback Integration Loop
e. Go back to (a) with fresh adversary context

HUMAN APPROVAL.

## Phase F4: Delta Implementation

For each wave in the wave schedule:
- Run per-story delivery cycle from `per-story-delivery.md`
- Wave integration gate after each wave
- Full test suite on merged develop (regression protection)

## Build Verification

Spawn implementer: "Full release build + complete test suite on develop."

## Holdout Evaluation

Spawn holdout-evaluator: "Evaluate full product against holdout scenarios
(both new and existing). Regression check on existing scenarios."

## Phase F5: Scoped Adversarial Review

Delta adversarial review loop (max 10 passes per VSDD.md Phase 4):
a. Spawn adversary: "Delta-only adversarial review. Fresh context, information
   wall applies. Focus on new/changed code paths."
b. Spawn code-reviewer: "Constructive code review of delta."
c. Spawn security-reviewer: "Security review of delta, plus regression check
   on security-sensitive modules."
d. If convergence reached (see VSDD.md Phase 6) → exit loop
e. Route findings per VSDD.md Feedback Integration Loop
f. Fix PR delivery for each fix
g. Go back to (a) with fresh adversary context

## Phase F6: Targeted Hardening

Spawn formal-verifier: "VP proofs for new/changed verification properties only."
Spawn security-reviewer: "Final security scan of delta."
Spawn dtu-validator: "DTU adversarial testing" (if DTU clones affected)

For UI features:
- Spawn accessibility-auditor: "Accessibility recheck"
- Spawn demo-recorder: "Capture visual regression baseline"

## Phase F7: Delta Convergence

Delta convergence loop (max 10 cycles):
a. Spawn consistency-validator: "7-dimensional convergence check on delta"
b. If ALL dimensions converge → exit loop
c. Route failing dimensions per VSDD.md Feedback Integration Loop
d. Fix PR delivery for each fix
e. Go back to (a)
After 5 cycles stalled: escalate with cost-benefit analysis to human.

HUMAN APPROVAL.

## Release

MINOR bump for features, PATCH for enhancements/bug fixes.
Spawn devops-engineer: "Release (semver -> CHANGELOG -> tag -> publish)"

## Feature Cycle Handoff

See `steady-state.md` for cycle archival, maintenance resume, and backlog management.

## Session Review

Spawn session-review: "Review feature development quality and propose improvements."
HUMAN reviews proposals (72h).
